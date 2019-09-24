// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    response_header::ResponseHeader,
    status_codes::StatusCode,
    diagnostic_info::DiagnosticInfo,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SetTriggeringResponse {
    pub response_header: ResponseHeader,
    pub add_results: Option<Vec<StatusCode>>,
    pub add_diagnostic_infos: Option<Vec<DiagnosticInfo>>,
    pub remove_results: Option<Vec<StatusCode>>,
    pub remove_diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for SetTriggeringResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::SetTriggeringResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SetTriggeringResponse> for SetTriggeringResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.add_results);
        size += byte_len_array(&self.add_diagnostic_infos);
        size += byte_len_array(&self.remove_results);
        size += byte_len_array(&self.remove_diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.add_results)?;
        size += write_array(stream, &self.add_diagnostic_infos)?;
        size += write_array(stream, &self.remove_results)?;
        size += write_array(stream, &self.remove_diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_limits)?;
        let add_results: Option<Vec<StatusCode>> = read_array(stream, decoding_limits)?;
        let add_diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream, decoding_limits)?;
        let remove_results: Option<Vec<StatusCode>> = read_array(stream, decoding_limits)?;
        let remove_diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream, decoding_limits)?;
        Ok(SetTriggeringResponse {
            response_header,
            add_results,
            add_diagnostic_infos,
            remove_results,
            remove_diagnostic_infos,
        })
    }
}
