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
    string::UAString,
    byte_string::ByteString,
    service_types::ApplicationDescription,
};

/// Creates a new session with the server.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSessionRequest {
    pub request_header: RequestHeader,
    pub client_description: ApplicationDescription,
    pub server_uri: UAString,
    /*
    The network address that the Client used to access the Session Endpoint.
    The HostName portion of the URL should be one of the HostNames for the
    application that are specified in the Server’s ApplicationInstanceCertificate (see
    Cause 7.2). The Server shall raise an AuditUrlMismatchEventType event if the URL
    does not match the Server’s HostNames. AuditUrlMismatchEventType event type is
    defined in Part 5.
    The Server uses this information for diagnostics and to determine the set of
    EndpointDescriptions to return in the response.
    */
    pub endpoint_url: UAString,
    /*
    Human readable string that identifies the Session. The Server makes this name and
    the sessionId visible in its AddressSpace for diagnostic purposes. The Client should
    provide a name that is unique for the instance of the Client.
    If this parameter is not specified the Server shall assign a value
    */
    pub session_name: UAString,
    /*
    A random number that should never be used in any other request. This number shall
have a minimum length of 32 bytes. Profiles may increase the required length. The
Server shall use this value to prove possession of its application instance Certificate
in the response.
    */
    pub client_nonce: ByteString,
    pub client_certificate: ByteString,
    /*
    Actual maximum number of milliseconds that a Session shall remain open without
    activity. The Server should attempt to honour the Client request for this parameter,
    but may negotiate this value up or down to meet its own constraints.
    */
    pub requested_session_timeout: f64,
    pub max_response_message_size: u32,
}

impl MessageInfo for CreateSessionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateSessionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateSessionRequest> for CreateSessionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.client_description.byte_len();
        size += self.server_uri.byte_len();
        size += self.endpoint_url.byte_len();
        size += self.session_name.byte_len();
        size += self.client_nonce.byte_len();
        size += self.client_certificate.byte_len();
        size += self.requested_session_timeout.byte_len();
        size += self.max_response_message_size.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.client_description.encode(stream)?;
        size += self.server_uri.encode(stream)?;
        size += self.endpoint_url.encode(stream)?;
        size += self.session_name.encode(stream)?;
        size += self.client_nonce.encode(stream)?;
        size += self.client_certificate.encode(stream)?;
        size += self.requested_session_timeout.encode(stream)?;
        size += self.max_response_message_size.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let client_description = ApplicationDescription::decode(stream, decoding_limits)?;
        let server_uri = UAString::decode(stream, decoding_limits)?;
        let endpoint_url = UAString::decode(stream, decoding_limits)?;
        let session_name = UAString::decode(stream, decoding_limits)?;
        let client_nonce = ByteString::decode(stream, decoding_limits)?;
        let client_certificate = ByteString::decode(stream, decoding_limits)?;
        let requested_session_timeout = f64::decode(stream, decoding_limits)?;
        let max_response_message_size = u32::decode(stream, decoding_limits)?;
        Ok(CreateSessionRequest {
            request_header,
            client_description,
            server_uri,
            endpoint_url,
            session_name,
            client_nonce,
            client_certificate,
            requested_session_timeout,
            max_response_message_size,
        })
    }
}
