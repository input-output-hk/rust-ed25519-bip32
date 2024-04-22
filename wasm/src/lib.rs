use ed25519_bip32::{ 
  DerivationScheme,
  XPrv,
  XPRV_SIZE,
  CHAIN_CODE_SIZE,
  EXTENDED_SECRET_KEY_SIZE
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Uint8Array;

/// This struct represents an XPrvWrapper object.
///
/// # Fields
///
/// * `key` - A byte array of size XPRV_SIZE that holds the key data.
#[wasm_bindgen]
pub struct XPrvWrapper {
    prv: XPrv
}

/**
 * Difference between the raw key (64 bytes) and the extended raw key with 96 bytes.
 * 
 * The 64 bytes raw key will contain the raw key with nothing else.
 * 
 * The extended key contains the raw data of the key + the derivation stuff , which is basically the chainCode.
 * If you export a key without the chainCode, when imported this derivationPath will be lost and unusable.
 * 
 * The chainCode in bip32 if i'm not wrong is the first 32 bytes of the key.
 */

 #[wasm_bindgen]
 impl XPrvWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(prv: Uint8Array) -> Self {
      let mut bytes = [0u8; XPRV_SIZE];
      prv.copy_to(&mut bytes);
      let x_prv = XPrv::from_slice_verified(&bytes).unwrap();
      XPrvWrapper { prv: x_prv }
    }

    pub fn public(&self) -> Uint8Array {
      let pub_inst =  self.prv.public();
      return self.bytes_to_buffer(pub_inst.public_key_bytes(), 32);
    }

    pub fn derive(&self, index: u32) -> Self {
      let derived = self.prv.derive(DerivationScheme::V2, index);
      XPrvWrapper { prv: derived }
    }

    pub fn extended_secret_key(&self) -> Uint8Array {
      return self.get_extended_secret_key(&self.prv);
    }

    pub fn chain_code(&self) -> Uint8Array {
      return self.get_chain_code(&self.prv);
    }

    fn get_extended_secret_key(&self, x_prv: &XPrv) -> Uint8Array {
      return self.bytes_to_buffer(x_prv.extended_secret_key_bytes(), EXTENDED_SECRET_KEY_SIZE);
    }

    fn get_chain_code(&self, x_prv: &XPrv) -> Uint8Array {
      return self.bytes_to_buffer(x_prv.chain_code(), CHAIN_CODE_SIZE);
    }

    fn bytes_to_buffer(&self, bytes: &[u8], length: usize) -> Uint8Array {
      let buffer = Uint8Array::new_with_length(length as u32);
      buffer.copy_from(bytes);
      return buffer
    }

    pub fn from_nonextended_noforce(
      js_bytes: Uint8Array,
      js_chain_code: Uint8Array,
    ) -> Self {
      let mut bytes = [0u8; 32];
      let mut chain_code = [0u8; CHAIN_CODE_SIZE];
      js_bytes.copy_to(&mut bytes);
      js_chain_code.copy_to(&mut chain_code);

      let x_prv = XPrv::from_nonextended_noforce(&bytes, &chain_code).unwrap();
      XPrvWrapper { prv: x_prv }
    }

    pub fn from_extended_and_chaincode(
      js_bytes: Uint8Array,
      js_chain_code: Uint8Array,
    ) -> Self {
      let mut bytes = [0u8; EXTENDED_SECRET_KEY_SIZE];
      let mut chain_code = [0u8; CHAIN_CODE_SIZE];
      js_bytes.copy_to(&mut bytes);
      js_chain_code.copy_to(&mut chain_code);

      let x_prv = XPrv::from_extended_and_chaincode(&bytes, &chain_code);
      XPrvWrapper{ prv: x_prv }
    }
}
