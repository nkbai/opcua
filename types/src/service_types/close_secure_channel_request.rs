// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

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

#[derive(Debug, Clone, PartialEq)]
pub struct CloseSecureChannelRequest {
    pub request_header: RequestHeader,
}

impl MessageInfo for CloseSecureChannelRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CloseSecureChannelRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CloseSecureChannelRequest> for CloseSecureChannelRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        Ok(CloseSecureChannelRequest {
            request_header,
        })
    }
}
