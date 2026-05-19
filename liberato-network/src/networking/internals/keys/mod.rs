//! # networking/internals/keys
//! 
//! Simple Wrapper Around `libp2p::identity::Keypair`
//! 
//! TODO: Add RSA4096
//! 
//! ## Types of Keys:
//! - [X] ED25519
//! - [X] ECDSA
//! - [X] SECP256k1
//! - [ ] RSA
//! 
//! ## Exporting
//! 
//! - [X] Protobuf
//!     - [X] To
//!     - [X] From

use libp2p::identity::*;

/// # ShrineKeys
/// 
/// Keypairs for usage.
#[derive(Clone)]
pub struct P2PKeys {
    pub key: Keypair
}

impl P2PKeys {
    pub fn from(protobuf: &[u8]) -> Result<Keypair,DecodingError> {
        Keypair::from_protobuf_encoding(protobuf)
    }
    pub fn from_into(protobuf: &[u8]) -> Result<Self,DecodingError> {
        let x = Keypair::from_protobuf_encoding(protobuf)?;

        return Ok(Self {
            key: x,
        })
    }
    pub fn generate_ed25519() -> P2PKeys {
        return Self {
            key: Keypair::generate_ed25519()
        }
    }
    pub fn generate_ecdsa() -> P2PKeys {
        return Self {
            key: Keypair::generate_ecdsa()
        }
    }
    pub fn generate_secp256k1() -> P2PKeys {
        return Self {
            key: Keypair::generate_secp256k1()
        }
    }
    pub fn into_protobuf(&self) -> Result<Vec<u8>,DecodingError>  {
        self.key.to_protobuf_encoding()
    }
}