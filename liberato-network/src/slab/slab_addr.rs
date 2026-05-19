//! # Slab
//! 
//! A SLAB is similar to cookies used in browsers but instead of being stored on the client-side, they are stored on servers and verified.
//! 
//! ## Features
//! 
//! - [X] Slab
//!     - [X] 

use fixedstr::{str32, str64, str256};

/// # SlabAddress
/// 
/// A SlabAddress is where the data is located.
pub struct SlabAddress {
    addr: str256,
    secret: Option<str64>, // secret
    derived_from: SlabAddressDerivedFrom,
}

pub enum SlabAddressDerivedFrom {
    Default,
    BLAKE3,
}