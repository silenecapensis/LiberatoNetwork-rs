//! Universal ICD Protocol
//! 
//! A protocol for exchanging files using ICD (Interconnected Design) standards.
//! 
//! ## Features
//! 
//! - [ ] Command-Namespace
//!     - [ ] source: u64
//!     - [ ] source_id: str
//!     - [ ] source_location
//!         - [ ] _type
//!         - [ ] id
//!         - [ ] unique namespace
//! 
//! - [ ] Registering a Service
//!     - [ ] Requires PoW
//!     - [ ] Requires Monetary
//!     - [ ] Requires Immutable Source
//! 
//! - [X] Register a Service as a Source ID and provide git for it. Uses a u64 in /ol-core/icd/sourceid/<source_id>
//! - [X] Request files by their file ID (string)

use async_trait::async_trait;
use futures::{io, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use libp2p::{request_response, StreamProtocol};

/// Source and Bridge definitions
pub mod source;
pub mod reg;

/// Implemented Attributes for Universal ICD Protocol & Labels
pub mod attributes;

/// Core Implementations for Universal ICD Protocol
pub mod core;


use libp2p::request_response::{
    ProtocolSupport, Codec as RRCodec, Config,
};

use libp2p::request_response::cbor::codec::Codec as CborCodec;
use librustysigs::UserCertificate;
use serde::{Serialize,Deserialize};

#[derive(Default, Clone)]
pub struct Codec;

pub struct RegisterServiceRequest {
    // User Certificate
    pub certificate: UserCertificate,
    // Encryption Key
    pub encryption_key: Option<()>,

    pub service_common_name: String,
    pub service_secret: String,


}

/// The request message for the file exchange protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    /// Registered Source ID for the request
    pub source_id: u64,
    pub file_id: String,
}

/// The response message for the file exchange protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// The contents of the file that is being sent.
    pub file_body: Vec<u8>,
}












//======ICD Protocol Implementation======//
pub struct UniversalIcdProtocolHeader {
    pub protocol_name: String,
    pub protocol_version: String,
    pub protocol_implementation: String,
    pub protocol_support: ProtocolSupport,
}

pub struct UniversalIcdNodeProtocol {
    pub header: UniversalIcdProtocolHeader,    
}

pub struct UniversalIcdCertificate {
    pub icd_certificate: UserCertificate,
    pub icd_encryption_key: Option<()>,

    pub icd_service_common_name: String,
    
}

pub struct TrustNet {
    pub trustnet_id: u64,
    pub trustnet_name: String,
    pub trustnet_description: String,
}

pub struct TrustNetMembership {
    pub trustnet_id: u64,
    pub member_id: u64,
    pub member_certificate: UserCertificate,
    pub member_role: String,

}

pub struct TrustedRoles {
    pub universal_role_id: u64, // Unique Role ID registered on the TrustNet/Blockchain
    
    pub role_name: String,
    pub role_description: String,
    pub role_permissions: Vec<String>,
    pub role_trust_level: u8, // 0-100
    
}

impl TrustNet {
    pub fn new(id: u64, name: String, description: String) -> Self {
        Self {
            trustnet_id: id,
            trustnet_name: name,
            trustnet_description: description,
        }
        
    }

}












/*


#[async_trait]
impl request_response::Codec for Codec {
    type Protocol = StreamProtocol;
    type Request = Request;
    type Response = Response;

    async fn read_request<T>(&mut self, _: &StreamProtocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, 1_000_000).await?;

        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        Ok(Request {
            source_id: 0u64,
            file_id: String::from_utf8(vec).unwrap(),
        })
    }

    async fn read_response<T>(
        &mut self,
        _: &StreamProtocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, 500_000_000).await?; // update transfer maximum

        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        Ok(Response { file_body: vec })
    }

    async fn write_request<T>(
        &mut self,
        _: &StreamProtocol,
        io: &mut T,
        Request { source_id: 0u64, file_id: String::from("test") }: Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, file_id).await?;

        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &StreamProtocol,
        io: &mut T,
        Response { file_body }: Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, file_body).await?;

        Ok(())
    }
}

/// Writes a message to the given socket with a length prefix appended to it. Also flushes the socket.
///
/// > **Note**: Prepends a variable-length prefix indicate the length of the message. This is
/// >           compatible with what [`read_length_prefixed`] expects.
pub async fn write_length_prefixed(
    socket: &mut (impl AsyncWrite + Unpin),
    data: impl AsRef<[u8]>,
) -> Result<(), io::Error> {
    write_varint(socket, data.as_ref().len()).await?;
    socket.write_all(data.as_ref()).await?;
    socket.flush().await?;

    Ok(())
}

/// Writes a variable-length integer to the `socket`.
///
/// > **Note**: Does **NOT** flush the socket.
pub async fn write_varint(
    socket: &mut (impl AsyncWrite + Unpin),
    len: usize,
) -> Result<(), io::Error> {
    let mut len_data = unsigned_varint::encode::usize_buffer();
    let encoded_len = unsigned_varint::encode::usize(len, &mut len_data).len();
    socket.write_all(&len_data[..encoded_len]).await?;

    Ok(())
}

/// Reads a variable-length integer from the `socket`.
///
/// As a special exception, if the `socket` is empty and EOFs right at the beginning, then we
/// return `Ok(0)`.
///
/// > **Note**: This function reads bytes one by one from the `socket`. It is therefore encouraged
/// >           to use some sort of buffering mechanism.
async fn read_varint(socket: &mut (impl AsyncRead + Unpin)) -> Result<usize, io::Error> {
    let mut buffer = unsigned_varint::encode::usize_buffer();
    let mut buffer_len = 0;

    loop {
        match socket.read(&mut buffer[buffer_len..buffer_len + 1]).await? {
            0 => {
                // Reaching EOF before finishing to read the length is an error, unless the EOF is
                // at the very beginning of the substream, in which case we assume that the data is
                // empty.
                if buffer_len == 0 {
                    return Ok(0);
                } else {
                    return Err(io::ErrorKind::UnexpectedEof.into());
                }
            }
            n => debug_assert_eq!(n, 1),
        }

        buffer_len += 1;

        match unsigned_varint::decode::usize(&buffer[..buffer_len]) {
            Ok((len, _)) => return Ok(len),
            Err(unsigned_varint::decode::Error::Overflow) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "overflow in variable-length integer",
                ));
            }
            // TODO: why do we have a `__Nonexhaustive` variant in the error? I don't know how to process it
            // Err(unsigned_varint::decode::Error::Insufficient) => {}
            Err(_) => {}
        }
    }
}

/// Reads a length-prefixed message from the given socket.
///
/// The `max_size` parameter is the maximum size in bytes of the message that we accept. This is
/// necessary in order to avoid DoS attacks where the remote sends us a message of several
/// gigabytes.
///
/// > **Note**: Assumes that a variable-length prefix indicates the length of the message. This is
/// >           compatible with what [`write_length_prefixed`] does.
async fn read_length_prefixed(
    socket: &mut (impl AsyncRead + Unpin),
    max_size: usize,
) -> io::Result<Vec<u8>> {
    let len = read_varint(socket).await?;
    if len > max_size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Received data size ({len} bytes) exceeds maximum ({max_size} bytes)"),
        ));
    }
    let mut buf = vec![0; len];
    socket.read_exact(&mut buf).await?;

    Ok(buf)
}
     */