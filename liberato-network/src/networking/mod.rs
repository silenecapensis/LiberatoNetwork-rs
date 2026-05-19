//! # Networking Internals Using Libp2p
//! 
//! ## MuscarineV1 Modules
//! 
//! - [ ] General
//!     - [ ] Muscarine-Bootstrap (In-Progress)
//!     - [ ] Muscarine-Address-Translator
//!     - [ ] Muscarine-Data-Structures
//!     - [ ] Muscarine-Traits
//!     - [ ] Muscarine-Data-Formats
//! - [X] Cryptography
//!     - [X] Oint-Wallet (In-Progress)
//!     - [X] SlugEncode (Constant-Time Encodings)
//!     - [X] libslug :TEMP
//! - [ ] Smart Contract / Programs    
//!     - [ ] Muscarine-WASM
//! 
//! ## MuscalineV1 Related Projects
//! 
//! - [ ] PROJECT SUMIC
//! - [X] X59 FMT
//! 
//! ## MuscarineV1 Features
//! 
//! ### Cryptography
//! 
//! - [X] Cryptography
//!     - [X] Signing
//!         - [X] ShulginSigning (ED25519 and SPHINCS+)
//!         - [ ] Falcon
//!             - [ ] Falcon512
//!             - [ ] Falcon1024
//!         - [ ] ML-DSA (Dilithium65)
//!         - [ ] ED25519
//!         - [ ] RSA
//!     - [X] Symmetric Encryption
//!         - [X] AES256-GCM
//!         - [X] XCHACHA20-POLY1305
//!         - [ ] Morus
//!     - [X] Public Key Encryption
//!         - [X] ECIES-ED25519-SHA3
//!         - [ ] RSA
//!         - [ ] Kyber768
//!     - [X] Hash Functions
//!         - [ ] SHA256, SHA384, SHA512
//!         - [ ] SHA3
//!         - [X] BLAKE2
//!             - [X] BLAKE2s
//!             - [X] BLAKE2b
//!         - [X] BLAKE3
//! 
//! ### Protocols
//! 
//! - [X] MuscarineV1 Protocols
//!     - [ ] (0x00) BLANK
//!     - [ ] (0x01) Assignment-By-Signature
//!     - [ ] (0x02) Bootstrap Protocol
//!     - [ ] (0x03) Interconnected Design Protocol (ICD)
//!         - [ ] Modular, easy to use
//!     - [ ] (0x04) Gateway Protocol
//!         - [ ] Static Gateway Protocol
//! 
//! ## Features
//! 
//! - [X] Connection (Transport)
//!     - [X] Use of Websockets
//!     - [X] Use of Noise Protocol
//!     - [X] Use of Yamux Multiplexing
//! - [X] Commands
//!     - [ ] Implemented
//!         - [ ] lsp
//!         - [ ] ...
//! - [X] Behaviour
//!     - [ ] Peer Discovery
//!         - [X] KAD
//!         - [ ] MDNS
//! - [X] Transports Implemented For Swarms
//!     - [X] TCP + Noise + Yamux
//!     - [X] WSS + Noise + Yamux
//!     - [X] QUIC (UDP)
//! - [X] Keypairs for Peers
//!     - [X] ED25519
//!     - [X] ECDSA
//!     - [X] Secp256k1
//!     - [ ] RSA

/// Internals
pub mod internals;
/// Custom Protocols
pub mod protocols;