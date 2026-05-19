//! # Authority Node
//! 
//! 

use librustysigs::UserCertificate;

pub struct AuthorityNodeSlab {
    pub certificate: UserCertificate,
    pub constraints: Vec<AuthorityNodeSlabConstraints>,





    pub authority_certificate: String,
}

pub enum AuthorityNodeSlabConstraints {
    Revocation,
}