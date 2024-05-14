use ed25519_bip32::{ 
  DerivationScheme,
  XPrv,
  CHAIN_CODE_SIZE,
  EXTENDED_SECRET_KEY_SIZE
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Uint8Array;

fn bytes_to_buffer(bytes: &[u8], length: usize) -> Uint8Array {
  let buffer = Uint8Array::new_with_length(length as u32);
  buffer.copy_from(bytes);
  return buffer
}

fn xprv_to_vec(xprv: XPrv) -> Vec<Uint8Array> {
  let sk_encoded = bytes_to_buffer(&xprv.extended_secret_key(), EXTENDED_SECRET_KEY_SIZE);
  let cc_encoded = bytes_to_buffer(xprv.chain_code(), CHAIN_CODE_SIZE);
  
  return vec![sk_encoded, cc_encoded];
}

/**
 * coerce given nonextended key and chain_code to valid ed25519 values or panic
 */
#[wasm_bindgen]
pub fn from_nonextended_noforce(
  key: Uint8Array,
  chain_code: Uint8Array,
) -> Vec<Uint8Array> {
  let mut sk_bytes = [0u8; 32];
  let mut cc_bytes = [0u8; CHAIN_CODE_SIZE];
  key.copy_to(&mut sk_bytes);
  chain_code.copy_to(&mut cc_bytes);

  let xprv = XPrv::from_nonextended_noforce(&sk_bytes, &cc_bytes).unwrap();
  return xprv_to_vec(xprv);
}

/**
 * given extended secret_key, chain_code and index, derive the child key values
 */
#[wasm_bindgen]
pub fn derive_bytes(
  sk: Uint8Array,
  chain_code: Uint8Array,
  index: u32
) -> Vec<Uint8Array> {
  let mut sk_bytes = [0u8; EXTENDED_SECRET_KEY_SIZE];
  let mut cc_bytes = [0u8; CHAIN_CODE_SIZE];
  sk.copy_to(&mut sk_bytes);
  chain_code.copy_to(&mut cc_bytes);

  let xprv = XPrv::from_extended_and_chaincode(&sk_bytes, &cc_bytes);
  let derived = xprv.derive(DerivationScheme::V2, index);
  
  return xprv_to_vec(derived);
}
