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
pub struct HistoryUpdateResult {
    pub status_code: StatusCode,
    pub operation_results: Option<Vec<StatusCode>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for HistoryUpdateResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryUpdateResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryUpdateResult> for HistoryUpdateResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += byte_len_array(&self.operation_results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += write_array(stream, &self.operation_results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let operation_results: Option<Vec<StatusCode>> = read_array(stream)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        Ok(HistoryUpdateResult {
            status_code,
            operation_results,
            diagnostic_infos,
        })
    }
}
