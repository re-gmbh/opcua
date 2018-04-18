//! The server module defines types related to the server, its current running state
//! and end point information.

use std;
use std::sync::{Arc, Mutex, RwLock};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::str::FromStr;

use futures::{Future, Stream};
use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio_timer;

use opcua_types::service_types::ServerState as ServerStateType;
use opcua_core::config::Config;
use opcua_core::prelude::*;

use address_space::types::AddressSpace;
use comms::tcp_transport::*;
use config::ServerConfig;
use constants;
use metrics::ServerMetrics;
use state::ServerState;
use diagnostics::ServerDiagnostics;
use services::message_handler::MessageHandler;
use session::Session;
use util::PollingAction;

pub type Connections = Vec<Arc<RwLock<TcpTransport>>>;

/// The Server represents a running instance of OPC UA. There can be more than one server running
/// at a time providing they do not share the same thread or listen on the same ports.
pub struct Server {
    /// Certificate store for certs
    pub certificate_store: Arc<Mutex<CertificateStore>>,
    /// Server metrics - diagnostics and anything else that someone might be interested in that
    /// describes the current state of the server
    pub server_metrics: Arc<RwLock<ServerMetrics>>,
    /// The server state is everything that sessions share that can possibly change
    pub server_state: Arc<RwLock<ServerState>>,
    /// Address space
    pub address_space: Arc<RwLock<AddressSpace>>,
    /// List of open connections
    pub connections: Arc<RwLock<Connections>>,
}

impl Server {
    /// Create a new server instance
    pub fn new(config: ServerConfig) -> Server {
        if !config.is_valid() {
            panic!("Cannot create a server using an invalid configuration.");
        }

        // Set from config
        let application_name = config.application_name.clone();
        let application_uri = UAString::from(config.application_uri.as_ref());
        let product_uri = UAString::from(config.product_uri.as_ref());
        let namespaces = vec!["http://opcfoundation.org/UA/".to_string(), "urn:OPCUA-Rust-Internal".to_string(), config.application_uri.clone()];
        let start_time = DateTime::now();
        let servers = vec![config.application_uri.clone()];
        let base_endpoint = format!("opc.tcp://{}:{}", config.tcp_config.host, config.tcp_config.port);
        let max_subscriptions = config.max_subscriptions as usize;
        let diagnostics = Arc::new(RwLock::new(ServerDiagnostics::new()));
        // TODO max string, byte string and array lengths

        // Security, pki auto create cert
        let (certificate_store, server_certificate, server_pkey) = CertificateStore::new_with_keypair(&config.pki_dir, config.create_sample_keypair);
        if server_certificate.is_none() || server_pkey.is_none() {
            error!("Server is missing its application instance certificate and/or its private key. Encrypted endpoints will not function correctly.")
        }
        let config = Arc::new(RwLock::new(config.clone()));

        let server_state = ServerState {
            application_uri,
            product_uri,
            application_name: LocalizedText {
                locale: UAString::null(),
                text: UAString::from(application_name),
            },
            namespaces,
            servers,
            base_endpoint,
            state: ServerStateType::Running,
            start_time,
            config,
            server_certificate,
            server_pkey,
            last_subscription_id: 0,
            max_subscriptions,
            min_publishing_interval: constants::MIN_PUBLISHING_INTERVAL,
            max_keep_alive_count: constants::MAX_KEEP_ALIVE_COUNT,
            diagnostics,
            abort: false,
        };
        let server_state = Arc::new(RwLock::new(server_state));

        // Set some values in the address space from the server state
        let address_space = Arc::new(RwLock::new(AddressSpace::new()));

        {
            let mut address_space = trace_write_lock_unwrap!(address_space);
            address_space.set_server_state(server_state.clone());
        }

        // Server metrics
        let server_metrics = Arc::new(RwLock::new(ServerMetrics::new()));

        // Cert store
        let certificate_store = Arc::new(Mutex::new(certificate_store));

        let server = Server {
            server_state,
            server_metrics: server_metrics.clone(),
            address_space,
            certificate_store,
            connections: Arc::new(RwLock::new(Vec::new())),
        };

        let mut server_metrics = trace_write_lock_unwrap!(server_metrics);
        server_metrics.set_server_info(&server);

        server
    }

    // Log information about the endpoints on this server
    fn log_endpoint_info(&self) {
        let server_state = trace_read_lock_unwrap!(self.server_state);
        let config = trace_read_lock_unwrap!(server_state.config);
        info!("OPC UA Server: {}", server_state.application_name);
        info!("Base url: {}", server_state.base_endpoint);
        info!("Supported endpoints:");
        for (id, endpoint) in &config.endpoints {
            let users: Vec<String> = endpoint.user_token_ids.iter().map(|id| id.clone()).collect();
            let users = users.join(", ");
            info!("Endpoint \"{}\": {}", id, endpoint.path);
            info!("  Security Mode:    {}", endpoint.security_mode);
            info!("  Security Policy:  {}", endpoint.security_policy);
            info!("  Supported user tokens - {}", users);
        }
    }


    /// Runs the server. Note server is supplied protected by a lock allowing access to the server
    /// to be shared.
    pub fn run(server: Arc<RwLock<Server>>) {
        // Debug endpoints
        {
            let server = trace_read_lock_unwrap!(server);
            server.log_endpoint_info();
        }

        // Get the 
        let (sock_addr, discovery_server_url) = {
            let server = trace_read_lock_unwrap!(server);
            let server_state = trace_read_lock_unwrap!(server.server_state);
            let config = trace_read_lock_unwrap!(server_state.config);
            let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(&config.tcp_config.host).unwrap()), config.tcp_config.port);
            (sock_addr, config.discovery_server_url.clone())
        };

        // TODO this needs to be a repeating tokio task
        let discovery_server_timer = {
            let mut server = trace_write_lock_unwrap!(server);
            server.start_discovery_server_registration_timer(discovery_server_url)
        };

        info!("Waiting for Connection");
        // This is the main tokio task
        tokio::run({
            let server = server.clone();

            // TODO spawn a discovery registration timer here, remove the old code
            let listener = TcpListener::bind(&sock_addr).unwrap();
            listener.incoming().for_each(move |socket| {
                // Clear out dead sessions
                info!("Handling new connection {:?}", socket);
                let mut server = trace_write_lock_unwrap!(server);
                if server.is_abort() {
                    info!("Server is aborting");
                } else {
                    server.remove_dead_connections();
                    server.handle_connection(socket);
                }
                Ok(())
            }).map_err(|err| {
                error!("Accept error = {:?}", err);
            })
        });

        drop(discovery_server_timer);
    }

    // Terminates the running server
    pub fn abort(&mut self) {
        let mut server_state = trace_write_lock_unwrap!(self.server_state);
        server_state.abort = true;

        // TODO - if the server is running we want to open a socket to it to stimulate it to
        // close down
    }

    fn is_abort(&mut self) -> bool {
        let server_state = trace_read_lock_unwrap!(self.server_state);
        server_state.abort
    }

    fn remove_dead_connections(&mut self) {
        // Go through all connections, removing those that have terminated
        let mut connections = trace_write_lock_unwrap!(self.connections);
        connections.retain(|connection| {
            // Try to obtain the lock on the transport and the session and check if session is terminated
            // if it is, then we'll use its termination status to sweep it out.
            let mut lock = connection.try_read();
            if let Ok(ref mut connection) = lock {
                !connection.terminated()
            } else {
                true
            }
        });
    }

    /// Start a timer that triggers every 5 minutes and causes the server to register itself with a discovery server
    fn start_discovery_server_registration_timer(&self, discovery_server_url: Option<String>) {
        if let Some(discovery_server_url) = discovery_server_url {
            let server_state = self.server_state.clone();
            let interval_timer = tokio_timer::Timer::default()
                .interval(std::time::Duration::from_secs(5 * 60))
                .for_each(move |_| {
                    let server_state = trace_read_lock_unwrap!(server_state);
                    let config = trace_read_lock_unwrap!(server_state.config);
                    // TODO - open a secure channel to discovery server, and register the endpoints of this server
                    // with the discovery server
                    trace!("Discovery server registration stub is triggering for {}", config.base_endpoint_url());
                    Ok(())
                });
            tokio::spawn(interval_timer.map_err(|_| ()));
        }
    }

    /// Creates a polling action that happens continuously on an interval.
    ///
    /// The returned PollingAction will ensure the function is called for as long as it is
    /// in scope. Once the action is dropped, the function will no longer be called.
    pub fn create_polling_action<F>(&mut self, interval_ms: u32, action: F) -> PollingAction
        where F: 'static + FnMut() + Send {
        let mut action = action;
        PollingAction::new(interval_ms, move || {
            // Call the provided closure with the address space
            action();
        })
    }

    pub fn new_transport(&self) -> TcpTransport {
        let session = {
            Arc::new(RwLock::new(Session::new(self)))
        };
        let address_space = self.address_space.clone();
        let message_handler = MessageHandler::new(self.certificate_store.clone(), self.server_state.clone(), session.clone(), address_space.clone());
        TcpTransport::new(self.server_state.clone(), session, address_space, message_handler)
    }

    /// Handles the incoming request
    fn handle_connection(&mut self, socket: TcpStream) {
        trace!("Connection thread spawning");

        // Spawn a thread for the connection
        let connection = Arc::new(RwLock::new(self.new_transport()));
        {
            let mut connections = trace_write_lock_unwrap!(self.connections);
            connections.push(connection.clone());
        }

        // Run adds a session task to the tokio session
        TcpTransport::run(connection, socket);
    }
}
