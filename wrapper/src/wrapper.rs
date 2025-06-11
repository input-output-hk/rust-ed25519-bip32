use std::{collections::HashMap, convert::TryInto};
use ed25519_bip32::{DerivationError, DerivationScheme, XPrv, XPub};

fn xprv_to_hashmap(xprv: XPrv) -> HashMap<String, Vec<u8>> {
  let sk_encoded = xprv.extended_secret_key().to_vec();
  let cc_encoded = xprv.chain_code().to_vec();
  
  return HashMap::from([
    ("secret_key".to_string(), sk_encoded),
    ("chain_code".to_string(), cc_encoded),
  ]);
}

fn xpub_to_hashmap(xpub: XPub) -> HashMap<String, Vec<u8>> {
  let pk_encoded = xpub.public_key().to_vec();
  let cc_encoded = xpub.chain_code().to_vec();
  
  return HashMap::from([
    ("public_key".to_string(), pk_encoded),
    ("chain_code".to_string(), cc_encoded),
  ]);
}

pub fn from_nonextended(
  sk: Vec<u8>,
  chain_code: Vec<u8>,
) -> HashMap<String, Vec<u8>> {
  let sk_bytes: [u8; 32] = sk.as_slice().try_into().unwrap();
  let cc_bytes: [u8; 32] = chain_code.as_slice().try_into().unwrap();
  let xprv = XPrv::from_nonextended_force(&sk_bytes, &cc_bytes);

  return xprv_to_hashmap(xprv);
}

pub fn derive_bytes(
  sk: Vec<u8>,
  chain_code: Vec<u8>,
  index: u32
) -> HashMap<String, Vec<u8>> {
  let sk_bytes: [u8; 64] = sk.as_slice().try_into().unwrap();
  let cc_bytes: [u8; 32] = chain_code.as_slice().try_into().unwrap();
  let xprv = XPrv::from_extended_and_chaincode(&sk_bytes, &cc_bytes);
  let derived = xprv.derive(DerivationScheme::V2, index);
  
  return xprv_to_hashmap(derived);
}

pub fn derive_bytes_pub(
  pk: Vec<u8>,
  chain_code: Vec<u8>,
  index: u32
) -> Result<HashMap<String, Vec<u8>>, DerivationError> {
  let pk_bytes: [u8; 32] = pk.as_slice().try_into().expect("Invalid public key length");
  let cc_bytes: [u8; 32] = chain_code.as_slice().try_into().expect("Invalid chain code length");

  let xpub = XPub::from_pk_and_chaincode(&pk_bytes, &cc_bytes);
  let derived = xpub.derive(DerivationScheme::V2, index)?;

  Ok(xpub_to_hashmap(derived))
}
