//! # Liberato Digital Asset Standard Traits
//! 
//! ## Implementation of Traits for LDAS
//! 
//! - [ ] LiberatoDigitalAssetStandard: A trait that defines the core functionalities and behaviors expected from any digital asset within the Liberato ecosystem. This includes methods for retrieving the asset's digest, metadata, and other essential properties.
//! - [ ] LiberatoDigitalAssetStandardPieces: A trait that specifies the requirements for handling

use fixedstr::str256;

pub trait LiberatoDigitalAssetStandard {
    /// # Get Digest
    /// 
    /// Returns the digest of the asset
    fn get_digest(&self) -> str256;
    /// # Get Digest with Extension
    /// 
    /// Returns the digest of the asset with an extension
    fn get_digest_with_extension<T: AsRef<str>>(&self) -> str256;
}

pub trait LiberatoDigitalAssetStandardPieces {
    /// # Get Pieces
    /// 
    /// Returns the pieces of the asset
    fn get_piece(&self) -> Vec<u8>;
}