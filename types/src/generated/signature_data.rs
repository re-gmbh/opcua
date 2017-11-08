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

/// A digital signature.
#[derive(Debug, Clone, PartialEq)]
pub struct SignatureData {
    pub algorithm: UAString,
    pub signature: ByteString,
}

impl MessageInfo for SignatureData {
    fn object_id(&self) -> ObjectId {
        ObjectId::SignatureData_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SignatureData> for SignatureData {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.algorithm.byte_len();
        size += self.signature.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.algorithm.encode(stream)?;
        size += self.signature.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let algorithm = UAString::decode(stream)?;
        let signature = ByteString::decode(stream)?;
        Ok(SignatureData {
            algorithm,
            signature,
        })
    }
}
