//! # Bootstrapping Protocol
//! 
//! Bootstrapping is essential to joining the network. There are multiple methods of bootstrapping. These are known as a `BootstrapProtocol`.
//! 
//! The Bootstrapping Struct contains the peers needed to join the network and is checksumed to look for any differences in peers.
//! 
//! ## Seeding and Relaying
//! 
//! - [ ] `MuscarnicSeeders`: Seeders provide data that is essential to the network.
//! 
//! - [ ] `MuscarineRelays`: These relays provide relaying information from one to another.
//! 
//! - [ ] `MuscarineValidators`: Validators provide proof for certain information

use async_std::path;
use fixedstr::str128;
use libp2p::PeerId;
use crate::errors::NetworkingStackErrors;
use log::{debug,error};
use blake2_rfc::blake2s::Blake2s;
use slugencode::SlugEncodingUsage;
use fixedstr::str32;
use fixedstr::str64;
use fixedstr::str256;
use std::str::FromStr;
use serde::{Serialize,Deserialize};
use serde_big_array::BigArray;


/// # Bootstrap Addresses
/// 
/// The Bootstrap Addresses contains the `checksum` and `addresses`.
/// 
/// These addresses are used to connect to the network and bootstrap the process.
/// 
/// This struct is modular and can be used to bootstrap for many purposes.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BootstrapAddressesModular {
    checksum: str32,
    
    #[serde(with = "BigArray")]
    addresses: [str128;256],

    num_of_addresses: usize,
    
    bootstrap_usage: str64,
}

/// # Bootstrap Signature
/// 
/// ## Algorithms
/// 
/// - [X] ShulginSigning
/// - [ ] ED25519
/// - [ ] ED448
/// - [ ] ECDSA
/// - [ ] RSA
#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BootstrapSignature {
    checksum: str32,
    signature: str256,
    signature_alternative: Option<String>,
    alg: str32,
}

/// # Bootstrap Public Key
/// 
/// ## Algorithms
/// 
/// - [X] ShulginSigning (ED25519 + SPHINCS+) encoded in Hexadecimal
/// - [ ] ED25519
/// - [ ] ED448
/// - [ ] ECDSA
/// - [ ] RSA
#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BootstrapPublicKey {
    pkh: str256,
    alg: str32,
}

/*
impl BootstrapAddressesModular {
    pub fn new(bootstrap_protocol: BootstrapProtocol) -> Self {
        match bootstrap_protocol {
            BootstrapProtocol::InitFromDomain(v) => {

            }
            BootstrapProtocol::InitFromFile(v) => {

            }
            BootstrapProtocol::InitFromPeer(v) => {

            }
            BootstrapProtocol::InitFromPeers(v) => {

            }
            BootstrapProtocol::InitializeRequest(v) => {

            }
        }
    }
    /// Creates a New Empty Bootstrap
    pub fn new_with_no_values() -> Self {
        return Self {
            checksum: str32::new(),
            addresses: [str128::new();256],
            num_of_addresses: 0usize,
            
            bootstrap_usage: str64::new(),
        }
    }
    /// # Add
    /// 
    /// Adds a Peer ID in the format of Base58. It is serialized as a str128.
    pub fn add(&mut self, peer_id: PeerId) {
        debug!("[Shrine::Networking::KAD][Bootstrap] Adding Peer ID to Bootstrap Addresses");
        //self.addresses = (str128::from_str(&peer_id.to_base58()).unwrap())
    }
    pub fn remove(&mut self, peer_id: PeerId) -> Result<(),NetworkingStackErrors> {
        debug!("[Shrine::Networking::KAD][Bootstrap] Removing Peer ID");
        
        if self.addresses.contains(&peer_id) {
            debug!("[Shrine::Networking::KAD][Bootstrap] Peer ID is in bootstrap");
            let value: Result<usize, usize> = self.addresses.binary_search(&peer_id);
            debug!("[Shrine::Networking::KAD][Bootstrap] Found Peer ID");
            let x = self.addresses.swap_remove(value.unwrap());
            debug!("[Shrine::Networking::KAD][Bootstrap] Swapped Peer ID with Last Element");
            Ok(())
        }
        else {
            error!("[Error: 0x00] [Shrine::Networking::KAD] Removing Peer ID Failed");
            return Err(NetworkingStackErrors::Networking(0x00, String::from("KAD-BOOTSTRAP"), String::from("Failed To Remove Peer ID"), String::from("Check whether Peer ID was in Struct"), peer_id.to_string()))
        }
    }
    pub fn get_addresses(&self) -> Vec<PeerId> {
        return self.addresses.clone()
    }
    pub fn get_addresses_str(&self) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        
        for x in &self.addresses {
            output.push(x.to_base58())
        }
        return output
    }
    /// # Integrity Check
    /// 
    /// 8-byte BLAKE2s hash of the Peer IDs encoded in base58 to check for changes.
    pub fn integrity_check(&mut self) {
        let mut checksum_output = Blake2s::new(8);

        for i in &self.addresses {
            let x = i.to_bytes();
            checksum_output.update(&x);
        }

        let output = checksum_output.finalize();
        let encoder = SlugEncodingUsage::new(slugencode::SlugEncodings::Base58);
        let output_to_checksum = encoder.encode(output.as_bytes()).unwrap();
        self.checksum = str32::from_str(&output_to_checksum).unwrap()
    }
}
*/ 


/// # Bootstrap Protocol
/// 
/// Lists the methods of Bootstrapping Nodes For Muscarine-Network :TEMP
/// 
/// ## Protocols Implemented
/// 
/// - [ ] InitializeRequest
/// - [ ] InitFromPeer (contains Peer ID)
/// - [ ] InitFromPeers (contains Peer IDs)
/// - [ ] InitFromDomain (str256 domain)
/// - [ ] InitFromFile (str256 path)
pub enum BootstrapProtocol {
    InitializeRequest(u32),
    
    // From Peers
    InitFromPeer(PeerId),
    InitFromPeers(Vec<PeerId>),

    // From Domain
    InitFromDomain(str256),

    // From File
    InitFromFile(str256),

    // Search
    //SearchForPeers(str256),
}