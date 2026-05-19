//! # Basics of Service Nodes
//! 
//! - [ ] Node Information
//!     - [X] ShulginSigning (SPHINCS+ and ED25519 with hedged signature)
//!     - [ ] ED25519 (with hedged signature)
//!     - [ ] WOTS Signing
//!     - [ ] * Idea like the one i made for decentralized databases using ZKPs
//! 
//! TODO: Add an Easy String type Enum that allows for different types based on specified inputs as a library like for fingerprint it should be Blake2b(28) or something

use std::collections::HashMap;

use librustysigs::RustySignature;
use librustysigs::RustySignaturesUsage;
use serde::{Serialize,Deserialize};
use zeroize::{Zeroize,ZeroizeOnDrop};


use crate::core::crx::UserCertificate;
use crate::core::crx::UserCertificatePriv;


#[derive(Clone,Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
/// # NodeInformation
/// 
/// ID and `UserCertificate`
pub struct NodeInformation {
    id: u64,
    certificate: UserCertificate,
}

#[derive(Clone,Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
/// # NodeInformation (Node's Certificate and Info)
pub struct NodePrivateInfo {
    id: u64,
    local_id: u64,
    certificate: UserCertificatePriv,
}

/// # ShrineDomain
/// 
/// Domain Space
pub struct ShrineDomain {
    tld: String,
    name: String,
    
    subdomains: Option<Vec<String>>,

    pivot: String,
}

pub struct ServiceAssignedDatabase {

}

impl NodePrivateInfo {
    pub fn generate(local_id: u64, id: u64) -> Self {
        let cert = UserCertificatePriv::generate();

        return Self {
            id: id,
            local_id: local_id,
            certificate: cert,
        }
    }
    pub fn pretty_print(&self) -> String {
        let x = self.certificate.publiccert();
        return x.export().unwrap()
    }
    /// # ED25519
    /// 
    /// Hexadecimal
    pub fn ed25519_public_key(&self) -> String {
        self.certificate.cert.clkey.to_hex_string()
    }
    /// # SPHINCS+
    /// 
    /// Base58
    pub fn sphincs_public_key(&self) -> String {
        self.certificate.cert.pqkey.to_base58_string()
    }
    /// # Public Keys
    pub fn public_keys(&self) -> (String,String) {
        let clkey = self.certificate.cert.clkey.to_hex_string();
        let pqkey = self.certificate.cert.pqkey.to_base58_string();

        return (clkey,pqkey)
    }
    pub fn public_key(&self) -> String {
        let mut pk_str = String::new();
        let pk = self.public_keys();

        pk_str.push_str(&pk.0);
        pk_str.push_str(":");
        pk_str.push_str(&pk.1);

        return pk_str
    }
}

#[test]
fn run() {
    let node_slab = NodePrivateInfo::generate(0, 0);
    let pk = node_slab.public_key();
    println!("Public Key: {}",pk);
    println!("{}",node_slab.pretty_print())

}