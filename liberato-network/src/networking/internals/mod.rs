/// 
/// 
/// ## Progress
/// 
/// ### Swarms
/// 
/// - [X] SWARMS
///     - [X] Core Swarm (MuscarineV1)
/// 
/// ### Transport
/// 
/// - [X] Transports
///     - [X] TCP + Noise + Yamux
///     - [X] WSS + Noise + Yamux
///     - [X] QUIC
///     - [ ] ..
/// 
/// ### Keys
/// 
/// - [X] Keypairs For P2P
///     - [X] ED25519
///     - [X] Secp256k1
///     - [X] ECDSA
///     - [ ] RSA

/// Transport Methods: How To Send Bytes
pub mod transport;

/// The Behavior of the Network
pub mod behavior;

/// The Keyring
pub mod keys;

/// Implemented Commands
pub mod commands;

/// Implements different Swarms
pub mod swarm;

pub mod other;