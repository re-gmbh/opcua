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
pub struct SetPublishingModeRequest {
    pub request_header: RequestHeader,
    pub publishing_enabled: Boolean,
    pub subscription_ids: Option<Vec<UInt32>>,
}

impl MessageInfo for SetPublishingModeRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::SetPublishingModeRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SetPublishingModeRequest> for SetPublishingModeRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.publishing_enabled.byte_len();
        size += byte_len_array(&self.subscription_ids);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.publishing_enabled.encode(stream)?;
        size += write_array(stream, &self.subscription_ids)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let publishing_enabled = Boolean::decode(stream)?;
        let subscription_ids: Option<Vec<UInt32>> = read_array(stream)?;
        Ok(SetPublishingModeRequest {
            request_header,
            publishing_enabled,
            subscription_ids,
        })
    }
}
