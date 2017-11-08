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

/// The token that identifies a set of keys for an active secure channel.
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelSecurityToken {
    pub channel_id: UInt32,
    pub token_id: UInt32,
    pub created_at: DateTime,
    pub revised_lifetime: UInt32,
}

impl MessageInfo for ChannelSecurityToken {
    fn object_id(&self) -> ObjectId {
        ObjectId::ChannelSecurityToken_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ChannelSecurityToken> for ChannelSecurityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.channel_id.byte_len();
        size += self.token_id.byte_len();
        size += self.created_at.byte_len();
        size += self.revised_lifetime.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.channel_id.encode(stream)?;
        size += self.token_id.encode(stream)?;
        size += self.created_at.encode(stream)?;
        size += self.revised_lifetime.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let channel_id = UInt32::decode(stream)?;
        let token_id = UInt32::decode(stream)?;
        let created_at = DateTime::decode(stream)?;
        let revised_lifetime = UInt32::decode(stream)?;
        Ok(ChannelSecurityToken {
            channel_id,
            token_id,
            created_at,
            revised_lifetime,
        })
    }
}
