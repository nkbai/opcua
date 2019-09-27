// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    request_header::RequestHeader,
};

/// Closes a session with the server.
#[derive(Debug, Clone, PartialEq)]
pub struct CloseSessionRequest {
    pub request_header: RequestHeader,
    pub delete_subscriptions: bool,
}

impl MessageInfo for CloseSessionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CloseSessionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CloseSessionRequest> for CloseSessionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.delete_subscriptions.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.delete_subscriptions.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let delete_subscriptions = bool::decode(stream, decoding_limits)?;
        Ok(CloseSessionRequest {
            request_header,
            delete_subscriptions,
        })
    }
}
