use std::str::FromStr;

use fixedstr::str256;
use fixedstr::tstr;
use libslug::slugcrypt::internals::signature::esphand_signature::EsphandKeypair;
use libslug::slugcrypt::internals::signature::shulginsigning::ShulginKeypair;
use libslug::slugcrypt::traits::IntoX59SecretKey;
use crate::muscarinehandler::handle::traits::MuscarineIdentity;
use libslug::prelude::SlugDigest;
use libslug::slugcrypt::internals::signature::shulginsigning;
use libslug::slugcrypt::internals::signature::esphand_signature;
use libslug::slugcrypt::internals::signature::utils::signing_csprng;
use crate::muscarinehandler::handle::traits::MuscarineIdentitySecret;
use crate::muscarinehandler::handle::identity::traits::MuscarineIdentityTrait;

#[derive(Clone,Copy,Hash,PartialEq,PartialOrd,Debug)]
pub struct MuscarineIdentityV1 {
    public_key: tstr<4096>, // TODO: Make 4000 bytes long
    cipher_suite: str256,
}

#[derive(Clone,Copy,Hash,PartialEq,PartialOrd,Debug)]
pub struct MuscarineIdentityV1Secret {
    private_key: tstr<4096>,
    cipher_suite: str256,
}

impl MuscarineIdentityTrait for MuscarineIdentityV1Secret {
    fn public_key(&self) -> tstr<4096> {
        let x: str256 = self.cipher_suite;
        let ciphersuite = CipherSuites::from_cipher_suite_id(x);

        //TODO: Work On Libslug20
        match ciphersuite {
            CipherSuites::ShulginSigning => {
                return self.private_key
            }
            CipherSuites::EsphandSigning => {
                return self.private_key
            }
            _ => panic!("No Valid Option")
        }
    }
    fn generate(cipher_suite: CipherSuites) -> Self {
        match cipher_suite {
            CipherSuites::ShulginSigning => {
                let x = ShulginKeypair::generate();
                let keypair = x.to_x59_format_full().unwrap();
                let mut keypair_output: tstr<4096> = tstr::new();
                keypair_output.push_str(&keypair);

                return Self {
                    private_key: keypair_output,
                    cipher_suite: cipher_suite.cipher_suite_id(),
                }
            }
            CipherSuites::EsphandSigning => {
                let x = EsphandKeypair::generate();
                let output = x.into_x59().unwrap();
                let mut keypair: tstr<4096> = tstr::new();
                keypair.push_str(&output);

                return Self {
                    private_key: keypair,
                    cipher_suite: cipher_suite.cipher_suite_id(),
                }
            }
            _ => {
                panic!("No valid option")
            }
        }
    }
}

#[derive(Clone,Copy,Hash,PartialEq,PartialOrd,Debug)]
pub struct MuscarineIdentityV1WithSecretKey {
    public_key: tstr<4096>,
    secret_key: tstr<4096>,
    cipher_suite: str256,
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash)]
pub enum AddressScheme {
    X59FMT,
    X59FMT_HEX,
    X59FMT_BASE58,
    X59FMT_BASE32,
}

pub enum CipherSuites {
    ShulginSigning, // SPHINCS+ (SHAKE256) & ED25519
    EsphandSigning, // FALCON1024 + ED25519
    AbsolveSigning, // ML-DSA3 + ED25519
}

type Key = tstr<4096>;

// Add Cipher Suite Checksum

impl CipherSuites {
    pub fn cipher_suite_id(&self) -> str256 {
        let x = self.clone();
        match x {
            CipherSuites::ShulginSigning => return str256::from_str("slug20_shulginsigning_sphincs_and_ed25519").unwrap(),
            CipherSuites::EsphandSigning => return str256::from_str("slug20_esphandsigning_falcon1024_and_ed25519").unwrap(),
            CipherSuites::AbsolveSigning => return str256::from_str("slug20_absolvesigning_ml_dsa_and_ed25519").unwrap(),
        }
    }
    pub fn from_cipher_suite_id(ciphersuite: str256) -> Self {
        match ciphersuite.as_str() {
            "slug20_shulginsigning_sphincs_and_ed25519" => return CipherSuites::ShulginSigning,
            "slug20_esphandsigning_falcon1024_and_ed25519" => return CipherSuites::EsphandSigning,
            "slug20_absolvesigning_ml_dsa_and_ed25519" => return CipherSuites::AbsolveSigning,
            _ => panic!("No CipherSuite Selected")
        }
    }
}


impl MuscarineIdentity for MuscarineIdentityV1 {
    /// # Address Scheme
    /// 
    /// `X59FMT/Hex`: BLAKE2B(48, tstr<4096>.as_bytes()) | Format: Hex
    fn address(&self, address_scheme: AddressScheme) -> str256 {
        let address = self.public_key;
        if address_scheme == AddressScheme::X59FMT || address_scheme == AddressScheme::X59FMT_HEX {
            let output = SlugDigest::blake2b(48, &address.as_bytes());
            let output = output.digest();
            assert_eq!(output.as_bytes().len(),96);
            return str256::from_str(output).unwrap()
        }
        else {
            return str256::new()
        }
    }
}

impl MuscarineIdentitySecret for MuscarineIdentityV1WithSecretKey {
    fn generate(identity: CipherSuites) -> Result<Self,str256> {
        match identity {
            CipherSuites::EsphandSigning => {
                let x = EsphandKeypair::generate();
                let key = x.into_x59().unwrap();
                let (ed25519, falcon1024) = key.split_once("/").unwrap();
                let (ed25519pk, ed25519sk) = ed25519.split_once(":").unwrap();
                let (falcon1024pk, falcon1024sk) = falcon1024.split_once(":").unwrap();
                let mut output_pk = String::new();
                output_pk.push_str(ed25519pk);
                output_pk.push_str(":");
                output_pk.push_str(falcon1024pk);

                let mut output_sk = String::new();
                output_sk.push_str(ed25519sk);
                output_sk.push_str(":");
                output_sk.push_str(falcon1024sk);

                let mut pk_output_mutable: tstr<4096> = tstr::new();
                let mut sk_output_mutable: tstr<4096> = tstr::new();
                pk_output_mutable.push(&output_pk);
                sk_output_mutable.push(&output_sk);

                //let output_pk_2: tstr<4096> = tstr::<4096>::from_str(&output_pk).unwrap();
                //let output_sk_2: tstr<4096> = tstr::<4096>::from_str(&output_sk).unwrap();


                return Ok(MuscarineIdentityV1WithSecretKey { 
                    public_key: pk_output_mutable, 
                    secret_key: sk_output_mutable,
                    cipher_suite: identity.cipher_suite_id()
                })

            }
            CipherSuites::ShulginSigning => {
                let x = ShulginKeypair::generate();
                let output = x.to_x59_format_full().unwrap();
                let output_2 = output.split_once("/");
                let (pk, sk) = output_2.unwrap();

                let mut pk_mutable: tstr<4096> = tstr::new();
                pk_mutable.push(pk);

                let mut sk_mutable: tstr<4096> = tstr::new();
                sk_mutable.push(sk);

                //let sk_2: tstr<4096> = tstr::from_str(sk).unwrap();
                //let pk_2: tstr<4096> = tstr::from_str(pk).unwrap();
                //let output_key_sk: tstr<4096> = Key::from_str(sk).unwrap();
                //let output_key: tstr<4096> = Key::from_str(pk).unwrap();

                return Ok(MuscarineIdentityV1WithSecretKey {
                    public_key: pk_mutable,
                    secret_key: sk_mutable,
                    cipher_suite: identity.cipher_suite_id(),
                })
            }
            _ => panic!("No Valid Algorithm")
        }
    }
}

#[test]
fn create_identity() {
    let esphandsignature = MuscarineIdentityV1WithSecretKey::generate(CipherSuites::EsphandSigning).unwrap();
    let shulginsigning = MuscarineIdentityV1WithSecretKey::generate(CipherSuites::ShulginSigning).unwrap();

    println!("{:?}",esphandsignature);
    println!("{:?}",shulginsigning);
}