use ed25519_bip32::{DerivationError, DerivationScheme, XPrv, XPub};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Uint8Array;
use wasm_bindgen::JsValue;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct WasmDerivationError(DerivationError);

impl From<DerivationError> for WasmDerivationError {
    fn from(err: DerivationError) -> Self {
        Self(err)
    }
}

// Convert your custom error to JsValue
impl From<WasmDerivationError> for JsValue {
    fn from(error: WasmDerivationError) -> Self {
        JsValue::from_str(&format!("Derivation error: {:?}", error.0))
    }
}

fn bytes_to_buffer(bytes: &[u8], length: usize) -> Uint8Array {
  let buffer = Uint8Array::new_with_length(length as u32);
  buffer.copy_from(bytes);
  return buffer;
}

fn xprv_to_vec(xprv: XPrv) -> Vec<Uint8Array> {
  let sk_encoded = bytes_to_buffer(&xprv.extended_secret_key(), 64);
  let cc_encoded = bytes_to_buffer(xprv.chain_code(), 32);

  return vec![sk_encoded, cc_encoded];
}

fn xpub_to_vec(xpub: XPub) -> Vec<Uint8Array> {
  let pk_encoded = bytes_to_buffer(&xpub.public_key(), 32);
  let cc_encoded = bytes_to_buffer(xpub.chain_code(), 32);

  return vec![pk_encoded, cc_encoded];
}

/**
 * coerce given nonextended key and chain_code to valid ed25519 values or panic
 */
#[wasm_bindgen]
pub fn from_nonextended(
  key: Uint8Array,
  chain_code: Uint8Array,
) -> Vec<Uint8Array> {
  let mut sk_bytes = [0u8; 32];
  let mut cc_bytes = [0u8; 32];
  key.copy_to(&mut sk_bytes);
  chain_code.copy_to(&mut cc_bytes);

  let xprv = XPrv::from_nonextended_force(&sk_bytes, &cc_bytes);
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
  let mut sk_bytes = [0u8; 64];
  let mut cc_bytes = [0u8; 32];
  sk.copy_to(&mut sk_bytes);
  chain_code.copy_to(&mut cc_bytes);

  let xprv = XPrv::from_extended_and_chaincode(&sk_bytes, &cc_bytes);
  let derived = xprv.derive(DerivationScheme::V2, index);

  return xprv_to_vec(derived);
}

/**
 * given public_key, chain_code and index, derive the child key values
 */
 #[wasm_bindgen]
 pub fn derive_bytes_pub(
   pk: Uint8Array,
   chain_code: Uint8Array,
   index: u32
 ) -> Result<Vec<Uint8Array>, WasmDerivationError> {
   let mut pk_bytes = [0u8; 32];
   let mut cc_bytes = [0u8; 32];
   pk.copy_to(&mut pk_bytes);
   chain_code.copy_to(&mut cc_bytes);
 
   let xpub = XPub::from_pk_and_chaincode(&pk_bytes, &cc_bytes);
   let derived = xpub.derive(DerivationScheme::V2, index)?;
 
   return Ok(xpub_to_vec(derived));
 }
 