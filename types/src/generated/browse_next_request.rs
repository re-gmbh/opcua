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

/// Continues one or more browse operations.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowseNextRequest {
    pub request_header: RequestHeader,
    pub release_continuation_points: Boolean,
    pub continuation_points: Option<Vec<ByteString>>,
}

impl MessageInfo for BrowseNextRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowseNextRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowseNextRequest> for BrowseNextRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.release_continuation_points.byte_len();
        size += byte_len_array(&self.continuation_points);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.release_continuation_points.encode(stream)?;
        size += write_array(stream, &self.continuation_points)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let release_continuation_points = Boolean::decode(stream)?;
        let continuation_points: Option<Vec<ByteString>> = read_array(stream)?;
        Ok(BrowseNextRequest {
            request_header,
            release_continuation_points,
            continuation_points,
        })
    }
}
