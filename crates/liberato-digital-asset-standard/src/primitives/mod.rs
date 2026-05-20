//! # Liberato Digital Asset Standard: Primitives
//! 
//! This module defines the primitive types and structures that form the foundation of the Liberato Digital Asset Standard (LDAS). These primitives are essential for representing digital assets, their properties, and interactions within the Liberato ecosystem.
//! 
//! The primitives defined in this module include:
//! 
//! - `DigitalAsset`: A structure representing a digital asset, including its source data and corresponding digest.
//! - `DigitalAssetSourceContentID`: A structure representing the source content ID of a digital
//! 


use std::str::FromStr;

use fixedstr::str192;
use fixedstr::{str64, str128, str256};
use serde::{Serialize, Deserialize};
use slugencode::SlugDecoder;
use crate::traits::LiberatoDigitalAssetStandard;
use bincode;
use crate::primitives::pieces::DigitalAssetPiece;
use blake2_rfc::blake2b::Blake2b;
use slugencode::SlugEncodings;
use slugencode::SlugEncodingUsage;

/// # DigitalAsset
/// 
/// A structure representing a digital asset. It includes the source data and its corresponding digest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
pub struct DigitalAsset {
    pub source_data: Option<DigitalAssetData>,
    pub source_locations: Option<Vec<DigitalAssetSourceContentID>>,
    
    pub source_digest: str256,
    pub source_cid: str256,
}

/// # DigitalAssetSourceContentID
/// 
/// A structure representing the source content ID of a digital asset, which includes its content identifier (CID), common name (CN), and other identifying information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Hash, Copy)]
pub struct DigitalAssetSourceContentID {
    /// The content identifier (CID) of the source data, which is a unique identifier that can be used to retrieve the data from a decentralized storage network or other content-addressable storage systems.
    pub source_cid: str256,
    /// The Common Name (CN) associated with the source data, which can be used for human-readable identification and referencing of the digital asset.
    pub source_cn: str192,
    /// An array of optional tags associated with the source data, which can be used for categorization, filtering, and searching of digital assets based on specific attributes or keywords.
    pub source_tag: [Option<str64>;10],
    /// The type of the source data, which can indicate the format or nature of the data (e.g., "image/png", "text/plain", etc.) and can be used for processing and handling the data appropriately.
    pub source_type: str64,
}

impl DigitalAssetSourceContentID {
    /// # New
    /// 
    /// Creates a new `DigitalAssetSourceContentID` instance with the specified content identifier (CID), common name (CN), tags, and type.
    pub fn new(source_cid: str256, source_cn: str192, source_tag: [Option<str64>;10], source_type: str64) -> Self {
        Self {
            source_cid,
            source_cn,
            source_tag,
            source_type,
        }
    }
    /// Creates a new `DigitalAssetSourceContentID` instance from string inputs. This function takes string representations of the content identifier (CID), common name (CN), tags, and type, and converts them into the appropriate fixed-size string types used in the structure.
    pub fn new_from_str<T: AsRef<str>>(source_cid: T, source_cn: T, source_tag: [Option<T>;10], source_type: T) -> Result<Self, &'static str> {
        let source_cid = str256::from_str(source_cid.as_ref())?;
        let source_cn = str192::from_str(source_cn.as_ref())?;
        let source_tag = source_tag.map(|tag| tag.map(|t| str64::from_str(t.as_ref())?));
        let source_type = str64::from_str(source_type.as_ref())?;
        Ok(Self {
            source_cid,
            source_cn,
            source_tag,
            source_type,
        })
    }
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

/// Digital Asset Piece Size: {256kb, 16kb, 4kb}
pub mod pieces;