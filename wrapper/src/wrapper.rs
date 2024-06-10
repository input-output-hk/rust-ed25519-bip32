use std::{collections::HashMap, convert::TryInto};
use ed25519_bip32::{DerivationScheme, XPrv};

fn xprv_to_hashmap(xprv: XPrv) -> HashMap<String, Vec<u8>> {
  let sk_encoded = xprv.extended_secret_key().to_vec();
  let cc_encoded = xprv.chain_code().to_vec();
  
  return HashMap::from([
    ("secret_key".to_string(), sk_encoded),
    ("chain_code".to_string(), cc_encoded),
  ]);
}

pub fn from_nonextended(
  sk: Vec<u8>,
  chain_code: Vec<u8>,
) -> HashMap<String, Vec<u8>> {
  let sk_bytes: [u8; 32] = sk.as_slice().try_into().unwrap();
  let cc_bytes: [u8; 32] = chain_code.as_slice().try_into().unwrap();
  let xprv = XPrv::from_nonextended_force(&sk_bytes, &cc_bytes).unwrap();

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
