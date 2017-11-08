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
use generated::ContentFilterResult;

#[derive(Debug, Clone, PartialEq)]
pub struct EventFilterResult {
    pub select_clause_results: Option<Vec<StatusCode>>,
    pub select_clause_diagnostic_infos: Option<Vec<DiagnosticInfo>>,
    pub where_clause_result: ContentFilterResult,
}

impl BinaryEncoder<EventFilterResult> for EventFilterResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.select_clause_results);
        size += byte_len_array(&self.select_clause_diagnostic_infos);
        size += self.where_clause_result.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.select_clause_results)?;
        size += write_array(stream, &self.select_clause_diagnostic_infos)?;
        size += self.where_clause_result.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let select_clause_results: Option<Vec<StatusCode>> = read_array(stream)?;
        let select_clause_diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        let where_clause_result = ContentFilterResult::decode(stream)?;
        Ok(EventFilterResult {
            select_clause_results,
            select_clause_diagnostic_infos,
            where_clause_result,
        })
    }
}
