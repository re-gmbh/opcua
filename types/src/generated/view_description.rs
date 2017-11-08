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

/// The view to browse.
#[derive(Debug, Clone, PartialEq)]
pub struct ViewDescription {
    pub view_id: NodeId,
    pub timestamp: DateTime,
    pub view_version: UInt32,
}

impl MessageInfo for ViewDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::ViewDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ViewDescription> for ViewDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.view_id.byte_len();
        size += self.timestamp.byte_len();
        size += self.view_version.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.view_id.encode(stream)?;
        size += self.timestamp.encode(stream)?;
        size += self.view_version.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let view_id = NodeId::decode(stream)?;
        let timestamp = DateTime::decode(stream)?;
        let view_version = UInt32::decode(stream)?;
        Ok(ViewDescription {
            view_id,
            timestamp,
            view_version,
        })
    }
}
