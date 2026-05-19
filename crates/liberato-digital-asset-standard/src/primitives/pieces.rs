use serde::{Serialize, Deserialize};
use bincode::{Encode, Decode};
use serde_big_array::BigArray;

/// # Digital Asset Piece
/// 
/// A 256kb piece of a digital asset, used for breaking down larger assets into manageable parts for storage and transfer. Each piece contains a fixed-size byte array that holds the actual data of the asset piece, allowing for efficient handling and processing of digital assets within the Liberato ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
pub struct DigitalAssetPiece {
    /// 256kb
    #[serde(with = "BigArray")]
    pub piece_data: [u8; 262_144],
}