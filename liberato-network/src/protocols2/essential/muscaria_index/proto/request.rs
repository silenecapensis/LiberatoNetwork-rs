//! # MuscariaIndexRequest
//! 
//! The MuscariaIndexRequest is a way of gathering data from indexes of data for Muscaria/Muscarine Network.
//! 
//! 

use fixedstr::str256;
use serde::{Deserialize, Serialize};

/// # MuscarineIndexRequest
/// 
/// ## Features
/// 
/// - [ ] Get MuscariaIndex By Hash Digest
/// - [ ] Get MuscariaIndex By Topic
/// - [ ] Get MuscariaIndex By Tags
/// 
/// ### Authentication
/// 
/// - [ ] Get MuscariaIndex after Authentication
pub struct MuscariaIndexRequest {
    req: str256,

    auth: Option<str256>,
    session_token: Option<str256>, // session-token derived from chain
}

#[derive(Clone,Copy,PartialEq,PartialOrd,Serialize,Deserialize,Debug)]
pub struct MuscariaIndexSecretKey {
    secret_key: str256,
}