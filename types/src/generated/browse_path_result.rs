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
use generated::BrowsePathTarget;

/// The result of a translate opearation.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowsePathResult {
    pub status_code: StatusCode,
    pub targets: Option<Vec<BrowsePathTarget>>,
}

impl MessageInfo for BrowsePathResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowsePathResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowsePathResult> for BrowsePathResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += byte_len_array(&self.targets);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += write_array(stream, &self.targets)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let targets: Option<Vec<BrowsePathTarget>> = read_array(stream)?;
        Ok(BrowsePathResult {
            status_code,
            targets,
        })
    }
}
