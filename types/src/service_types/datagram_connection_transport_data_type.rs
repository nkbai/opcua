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
    extension_object::ExtensionObject,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DatagramConnectionTransportDataType {
    pub discovery_address: ExtensionObject,
}

impl BinaryEncoder<DatagramConnectionTransportDataType> for DatagramConnectionTransportDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.discovery_address.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.discovery_address.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let discovery_address = ExtensionObject::decode(stream, decoding_limits)?;
        Ok(DatagramConnectionTransportDataType {
            discovery_address,
        })
    }
}
