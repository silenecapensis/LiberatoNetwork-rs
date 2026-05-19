use crate::muscarinehandler::handle::identity::identityv1::CipherSuites;
use fixedstr::tstr;

/// # MuscarineIdentityTrait
/// 
/// Used in `MuscarineIdentityV1`
pub trait MuscarineIdentityTrait: Sized {
    /// # Public Key
    /// 
    /// Gets Public Key From Struct
    fn public_key(&self) -> tstr<4096>;
    /// # Generation
    /// 
    /// Generates a keypair
    fn generate(cipher_suite: CipherSuites) -> Self;
}