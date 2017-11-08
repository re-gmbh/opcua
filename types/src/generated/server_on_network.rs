// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use attribute::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use generated::node_ids::ObjectId;
#[allow(unused_imports)]
use generated::status_codes::StatusCode;

#[derive(Debug, Clone, PartialEq)]
pub struct ServerOnNetwork {
    pub record_id: UInt32,
    pub server_name: UAString,
    pub discovery_url: UAString,
    pub server_capabilities: Option<Vec<UAString>>,
}

impl MessageInfo for ServerOnNetwork {
    fn object_id(&self) -> ObjectId {
        ObjectId::ServerOnNetwork_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ServerOnNetwork> for ServerOnNetwork {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.record_id.byte_len();
        size += self.server_name.byte_len();
        size += self.discovery_url.byte_len();
        size += byte_len_array(&self.server_capabilities);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.record_id.encode(stream)?;
        size += self.server_name.encode(stream)?;
        size += self.discovery_url.encode(stream)?;
        size += write_array(stream, &self.server_capabilities)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let record_id = UInt32::decode(stream)?;
        let server_name = UAString::decode(stream)?;
        let discovery_url = UAString::decode(stream)?;
        let server_capabilities: Option<Vec<UAString>> = read_array(stream)?;
        Ok(ServerOnNetwork {
            record_id,
            server_name,
            discovery_url,
            server_capabilities,
        })
    }
}
