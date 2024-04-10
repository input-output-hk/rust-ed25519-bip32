 use ed25519_bip32::{ DerivationScheme, XPrv, XPRV_SIZE};
 use wasm_bindgen::prelude::*;
 use wasm_bindgen_futures::js_sys::{ArrayBuffer, Uint8Array};



/// This struct represents an XPrvWrapper object.
///
/// # Fields
///
/// * `key` - A byte array of size XPRV_SIZE that holds the key data.
#[wasm_bindgen]
pub struct XPrvWrapper {
    prv: XPrv
}


// // Convert &[u8; 64] to Uint8Array
// pub fn bytes_to_uint8array(bytes: &[u8; 64]) -> Uint8Array {
//     let buffer = Uint8Array::new_with_length(64);
//     buffer.set(&JsValue::from(bytes.as_ref())).unwrap();
//     buffer
// }

// // Convert Uint8Array to &[u8; 64]
// pub fn uint8array_to_bytes(array: Uint8Array) -> Result<[u8; 64], JsValue> {
//     let js_slice = array.to_vec();
//     if js_slice.len() != 64 {
//         return Err(JsValue::from_str("Expected Uint8Array of length 64"));
//     }

//     let mut bytes = [0; 64];
//     bytes.copy_from_slice(&js_slice);
//     Ok(bytes)
// }


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
      // Copy bytes from the Uint8Array into the array
        let mut bytes = [0u8; XPRV_SIZE];
        prv.copy_to(&mut bytes);
        let x_prv: XPrv = XPrv::from_slice_verified(&bytes).unwrap();
        XPrvWrapper { prv:x_prv }
    }

    pub fn public(&self) -> Uint8Array {
        let pub_inst =  self.prv.public();
        let pub_bytes = pub_inst.public_key_bytes();
        let buffer = Uint8Array::new_with_length(32);
        buffer.copy_from(pub_bytes);
        return buffer;
    }

    pub fn derive(&self, index: u32) -> Uint8Array {
        let derivation_schema = DerivationScheme::V2;
        let derived = self.prv.derive(derivation_schema, index);
        let derived_bytes_js =  derived.extended_secret_key_bytes();
        let buffer = Uint8Array::new_with_length(64);
        buffer.copy_from(derived_bytes_js);
        return buffer
    }

}
