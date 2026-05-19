//! # Errors
//! 
//! 0x00|0: Unassigned Error
//! 0x01|1: KAD: Failed To Remove Peer From Peer ID Table Due To Peer Not Being Found In Record. 
//! 
//! 
//! 
//! 

use thiserror::Error;


/// # MuscarineNetworkError
/// 
/// This is the main error handling enum that also includes many of other errors implemented into it.
#[derive(Error,Debug)]
pub enum MuscarineNetworkError {
    #[error("The Networking Stack Threw An Error")]
    NetworkingError,
    #[error("The Serialization of Data has returned an Error")]
    SerializationError,
    #[error("An Unknown Error Has Occured")]
    UnknownError,
}


#[derive(Error,Debug)]
pub enum NetworkingStackErrors {
    #[error("The Networking Error {0} has occured, this is in reference to the problem: {1} | Reason: {2}, Solution: {3}, Debug Info: {4}")]
    Networking(u16,String,String,String,String),

    #[error("Unknown Error, Information: {0}")]
    UnknownError(String),
}