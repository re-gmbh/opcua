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
use generated::RelativePath;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryDataDescription {
    pub relative_path: RelativePath,
    pub attribute_id: UInt32,
    pub index_range: UAString,
}

impl MessageInfo for QueryDataDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryDataDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryDataDescription> for QueryDataDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.relative_path.byte_len();
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.relative_path.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let relative_path = RelativePath::decode(stream)?;
        let attribute_id = UInt32::decode(stream)?;
        let index_range = UAString::decode(stream)?;
        Ok(QueryDataDescription {
            relative_path,
            attribute_id,
            index_range,
        })
    }
}
