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
    byte_string::ByteString,
};

/// Continues one or more browse operations.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowseNextRequest {
    pub request_header: RequestHeader,
    pub release_continuation_points: bool,
    pub continuation_points: Option<Vec<ByteString>>,
}

impl MessageInfo for BrowseNextRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowseNextRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowseNextRequest> for BrowseNextRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.release_continuation_points.byte_len();
        size += byte_len_array(&self.continuation_points);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.release_continuation_points.encode(stream)?;
        size += write_array(stream, &self.continuation_points)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let release_continuation_points = bool::decode(stream, decoding_limits)?;
        let continuation_points: Option<Vec<ByteString>> = read_array(stream, decoding_limits)?;
        Ok(BrowseNextRequest {
            request_header,
            release_continuation_points,
            continuation_points,
        })
    }
}
