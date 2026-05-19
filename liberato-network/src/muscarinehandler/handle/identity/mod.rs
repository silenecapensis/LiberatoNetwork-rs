//! # MuscarineHandler/Handle/Identity
//! 
//! This module contains the identity information using x59-fmt.

use std::str::FromStr;

use fixedstr::str256;
use librustysigs::registry::ShulginSigning;
use libslug::{errors::SlugErrors, slugcrypt::internals::signature::shulginsigning::{ShulginKeypair, ShulginSignature}};
use serde::{Serialize,Deserialize};

pub mod traits;

// TODO: Make keypair longer to 4000 bytes
#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Serialize,Deserialize)]
pub struct MuscarineIdentity {
    common_name: str256,
    description: str256,
    email: str256,
    keypair: str256,
}

impl MuscarineIdentity {
    /// # TODO:
    /// 
    /// Work on Error-Checking
    pub fn generate<T: AsRef<str>>(cn: T, description: T, email: T) -> Self  {
        let keypair = ShulginKeypair::generate();
        let pk = keypair.to_x59_pk_format().unwrap();
        let pk_str = str256::from_str(&pk).unwrap();

        let cn = str256::from_str(cn.as_ref()).unwrap();
        let desc = str256::from_str(description.as_ref()).unwrap();
        let email = str256::from_str(email.as_ref()).unwrap();

        Self {
            common_name: cn,
            description: desc,
            email: email,
            keypair: pk_str,
        }
    }
    pub fn into_shulginsigning(&self) -> ShulginKeypair {
        let pk: ShulginKeypair = ShulginKeypair::from_x59_pk_format(self.keypair.to_str()).expect("Invalid Format For Shulgin Signing Public Key");

        return pk
    }
    pub fn verify_signature<T: AsRef<str>>(&self, msg: T, sig_as_x59: T) -> Result<bool, SlugErrors> {
        let pk = self.into_shulginsigning();
        let signature = ShulginSignature::from_x59_format(sig_as_x59.as_ref())?;
        let is_valid = pk.verify(msg.as_ref(),&signature)?;
        return Ok(is_valid)
    }
}

#[test]
fn example_identity() {
    let identity = MuscarineIdentity::generate("LiberatoUser", "This is an example of a user identity", "example@gmail.com");
    let output = serde_yaml::to_string(&identity).unwrap();

    println!("{}",output)
}

/// IdentityV1
pub mod identityv1;