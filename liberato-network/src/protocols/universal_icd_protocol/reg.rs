//! # Universal ICD Registration Protocol
//! 
//! A protocol for registering services and exchanging certificates using ICD (Interconnected Design) standards.
//! 
//! ## Features
//! 
//! - [X] Register a Service with its Git repository and labels.
//! - [X] Exchange User Certificates for authentication and verification.
//! 
//! ## TODO:
//! 
//! - OINTWALLET
use librustysigs::{RustySignature, UserCertificate};



pub struct ServiceICDRegistrationRequest {
    pub git_repo: String,
    pub labels: Vec<String>,
    pub certificate: UserCertificate,
}















pub struct RegisterCertificate {
    // TODO: UserCertificate as PK
    pub certificate: UserCertificate,
}
pub struct ResponseCertificate {
    pub certificate: UserCertificate,
    pub signed_by: Vec<(UserCertificate,RustySignature)>,
}