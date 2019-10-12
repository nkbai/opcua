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

/// Delete one or more nodes from the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteNodesResponse {
    pub response_header: ResponseHeader,
    pub results: Option<Vec<StatusCode>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for DeleteNodesResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::DeleteNodesResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DeleteNodesResponse> for DeleteNodesResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_limits)?;
        let results: Option<Vec<StatusCode>> = read_array(stream, decoding_limits)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream, decoding_limits)?;
        Ok(DeleteNodesResponse {
            response_header,
            results,
            diagnostic_infos,
        })
    }
}
