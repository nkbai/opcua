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
    string::UAString,
    variant::Variant,
    extension_object::ExtensionObject,
    service_types::KeyValuePair,
    service_types::WriterGroupDataType,
    service_types::ReaderGroupDataType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PubSubConnectionDataType {
    pub name: UAString,
    pub enabled: bool,
    pub publisher_id: Variant,
    pub transport_profile_uri: UAString,
    pub address: ExtensionObject,
    pub connection_properties: Option<Vec<KeyValuePair>>,
    pub transport_settings: ExtensionObject,
    pub writer_groups: Option<Vec<WriterGroupDataType>>,
    pub reader_groups: Option<Vec<ReaderGroupDataType>>,
}

impl MessageInfo for PubSubConnectionDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::PubSubConnectionDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<PubSubConnectionDataType> for PubSubConnectionDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.name.byte_len();
        size += self.enabled.byte_len();
        size += self.publisher_id.byte_len();
        size += self.transport_profile_uri.byte_len();
        size += self.address.byte_len();
        size += byte_len_array(&self.connection_properties);
        size += self.transport_settings.byte_len();
        size += byte_len_array(&self.writer_groups);
        size += byte_len_array(&self.reader_groups);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.name.encode(stream)?;
        size += self.enabled.encode(stream)?;
        size += self.publisher_id.encode(stream)?;
        size += self.transport_profile_uri.encode(stream)?;
        size += self.address.encode(stream)?;
        size += write_array(stream, &self.connection_properties)?;
        size += self.transport_settings.encode(stream)?;
        size += write_array(stream, &self.writer_groups)?;
        size += write_array(stream, &self.reader_groups)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let name = UAString::decode(stream, decoding_limits)?;
        let enabled = bool::decode(stream, decoding_limits)?;
        let publisher_id = Variant::decode(stream, decoding_limits)?;
        let transport_profile_uri = UAString::decode(stream, decoding_limits)?;
        let address = ExtensionObject::decode(stream, decoding_limits)?;
        let connection_properties: Option<Vec<KeyValuePair>> = read_array(stream, decoding_limits)?;
        let transport_settings = ExtensionObject::decode(stream, decoding_limits)?;
        let writer_groups: Option<Vec<WriterGroupDataType>> = read_array(stream, decoding_limits)?;
        let reader_groups: Option<Vec<ReaderGroupDataType>> = read_array(stream, decoding_limits)?;
        Ok(PubSubConnectionDataType {
            name,
            enabled,
            publisher_id,
            transport_profile_uri,
            address,
            connection_properties,
            transport_settings,
            writer_groups,
            reader_groups,
        })
    }
}
