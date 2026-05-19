//! # Liberato Digital Asset Standard: Primitives
//! 
//! This module defines the primitive types and structures that form the foundation of the Liberato Digital Asset Standard (LDAS). These primitives are essential for representing digital assets, their properties, and interactions within the Liberato ecosystem.


use std::str::FromStr;

use fixedstr::{str64, str128, str256};
use serde::{Serialize, Deserialize};
use slugencode::SlugDecoder;
use crate::traits::LiberatoDigitalAssetStandard;
use bincode;
use crate::primitives::pieces::DigitalAssetPiece;
use blake2_rfc::blake2b::Blake2b;
use slugencode::SlugEncodings;
use slugencode::SlugEncodingUsage;

pub struct DigitalAsset {

}

/// # DigitalAssetData
/// 
/// A structure representing the data associated with a digital asset. This can include metadata, content, or any other relevant information that defines the asset's characteristics and properties.
/// 
/// ## Fields
/// 
/// - `data`: A byte vector containing the actual data of the digital asset. This can be any form of data, such as text, images, videos, or binary files.
/// - `content_type`: A string indicating the MIME type of the data, providing information about
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
pub struct DigitalAssetData {
    pub data: Vec<u8>,
    pub content_type: str64,
}

pub struct DigitalAssetPieces {
    pub pieces: Vec<DigitalAssetPiece>,
}

impl LiberatoDigitalAssetStandard for DigitalAssetData {
    fn get_digest(&self) -> str256 {
        let mut hasher = Blake2b::new(48);
        let mut slug_encoder: SlugEncodingUsage = SlugEncodingUsage::new(SlugEncodings::Hex);
        hasher.update(&self.data);
        let output = slug_encoder.encode(hasher.finalize().as_bytes()).unwrap();
        str256::from_str(&output).unwrap()
    }
    fn get_digest_with_extension<T: AsRef<str>>(&self) -> str256 {
        let mut hasher: Blake2b = Blake2b::new(48);
        let mut slug_encoder: SlugEncodingUsage = SlugEncodingUsage::new(SlugEncodings::Hex);
        hasher.update(&self.data);
        hasher.update(&self.content_type.to_lowercase().as_bytes());
        let output: String = slug_encoder.encode(hasher.finalize().as_bytes()).unwrap();
        str256::from_str(&output).unwrap()
    }
}

pub mod pieces;