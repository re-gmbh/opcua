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

/// Registers one or more nodes for repeated use within a session.
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterNodesResponse {
    pub response_header: ResponseHeader,
    pub registered_node_ids: Option<Vec<NodeId>>,
}

impl MessageInfo for RegisterNodesResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::RegisterNodesResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RegisterNodesResponse> for RegisterNodesResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.registered_node_ids);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.registered_node_ids)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let registered_node_ids: Option<Vec<NodeId>> = read_array(stream)?;
        Ok(RegisterNodesResponse {
            response_header,
            registered_node_ids,
        })
    }
}
