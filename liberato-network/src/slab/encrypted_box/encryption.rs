//! # Encrypted Box
//! 
//! One-Time Use Boxes that are Encrypted
//! 
//! ## Algorithms
//! 
//! - XCHACHA20-POLY1305
//! - AES256-GCM

use fixedstr::str256;
use libslug::slugcrypt::internals::encrypt::aes256::{AESCipherText,DecryptAES256,EncryptAES256,EncryptionKey,EncryptionNonce};
use libslug::slugcrypt::internals::encrypt::chacha20::{XChaCha20Encrypt,EncryptionCipherText};
use libslug::slugcrypt::internals::encrypt::chacha20::EncryptionKey as ChaCha20EncryptionKey;
use libslug::slugcrypt::internals::encrypt::chacha20::EncryptionNonce as ChaCha20EncryptionNonce;
use serde::{Deserialize, Serialize};


pub struct EncryptionStorageUnitAES256GCM {
    ciphertext: AESCipherText,
    nonce: EncryptionNonce,
}

pub struct EncryptionStorageUnitAddress {
    addr: str256,
}


#[derive(Debug,Clone,Copy,Serialize,Deserialize,PartialEq,Hash)]
pub enum Algorithms {
    XCHACHA20POLY1305,
    AES256GCM,
}