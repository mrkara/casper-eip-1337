use ed25519_dalek::{
  Signer,
};

use types::{
  SecretKey,
  crypto:: {
    PublicKey
  },
};

use hex;

use std::env;
use std::fs;

mod der;

pub fn sign(secret_key: SecretKey, message_bytes: [u8;32]) -> String {
  match secret_key {
    SecretKey::Ed25519(secret_key) => {
        let pub_key: ed25519_dalek::PublicKey = (&secret_key).into();
        let pair = ed25519_dalek::Keypair{
            secret: secret_key,
            public: pub_key,
        };

        let signature: ed25519_dalek::Signature = pair.sign(&message_bytes);
        hex::encode(signature.to_bytes())
    },
    _ => panic!("secret key should be a Ed25519"),
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  assert_eq!(args.len(), 3);
  println!("{:?}", args);

  let message_key_str = &args[2];
  let mut message_bytes =  [0u8;32];
  hex::decode_to_slice(message_key_str, &mut message_bytes as &mut [u8]).unwrap();

  let secret_key_file = &args[1];
  let secret_key_str = fs::read_to_string(secret_key_file).expect("Secret key file could not be read.");
  let secret_key = der::from_pem(secret_key_str.clone().as_bytes()).unwrap();

  let public_key: PublicKey = (&secret_key).into();

  println!("Public Key {:?}", public_key.to_account_hash().to_formatted_string());
  println!("Signature {:?}", sign(secret_key, message_bytes));
}