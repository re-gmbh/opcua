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
pub struct SubscriptionAcknowledgement {
    pub subscription_id: UInt32,
    pub sequence_number: UInt32,
}

impl MessageInfo for SubscriptionAcknowledgement {
    fn object_id(&self) -> ObjectId {
        ObjectId::SubscriptionAcknowledgement_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SubscriptionAcknowledgement> for SubscriptionAcknowledgement {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.subscription_id.byte_len();
        size += self.sequence_number.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.subscription_id.encode(stream)?;
        size += self.sequence_number.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let subscription_id = UInt32::decode(stream)?;
        let sequence_number = UInt32::decode(stream)?;
        Ok(SubscriptionAcknowledgement {
            subscription_id,
            sequence_number,
        })
    }
}
