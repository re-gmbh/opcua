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
use generated::ReadValueId;
use generated::MonitoringParameters;

#[derive(Debug, Clone, PartialEq)]
pub struct MonitoredItemCreateRequest {
    pub item_to_monitor: ReadValueId,
    pub monitoring_mode: MonitoringMode,
    pub requested_parameters: MonitoringParameters,
}

impl MessageInfo for MonitoredItemCreateRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoredItemCreateRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoredItemCreateRequest> for MonitoredItemCreateRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.item_to_monitor.byte_len();
        size += self.monitoring_mode.byte_len();
        size += self.requested_parameters.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.item_to_monitor.encode(stream)?;
        size += self.monitoring_mode.encode(stream)?;
        size += self.requested_parameters.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let item_to_monitor = ReadValueId::decode(stream)?;
        let monitoring_mode = MonitoringMode::decode(stream)?;
        let requested_parameters = MonitoringParameters::decode(stream)?;
        Ok(MonitoredItemCreateRequest {
            item_to_monitor,
            monitoring_mode,
            requested_parameters,
        })
    }
}
