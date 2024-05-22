use super::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_derive_bytes() {
  let sk = [248, 162, 146, 49, 238, 56, 214, 197, 191, 113, 93, 91, 172, 33, 199, 80, 87, 122, 163, 121, 139, 34, 215, 157, 101, 191, 151, 214, 250, 222, 161, 90, 220, 209, 238, 26, 189, 247, 139, 212, 190, 100, 115, 26, 18, 222, 185, 77, 54, 113, 120, 65, 18, 235, 111, 54, 75, 135, 24, 81, 253, 28, 154, 36];
  let cc = [115, 132, 219, 154, 214,  0, 59, 189, 8, 179, 177, 221, 192, 208, 122, 89, 114, 147, 255, 133, 233, 97, 191, 37, 43, 51, 18, 98, 237, 223, 173, 13];
  let index = 2147483649;

  let result = derive_bytes(bytes_to_buffer(&sk, 64), bytes_to_buffer(&cc, 32), index);

  let xprv = XPrv::from_extended_and_chaincode(&sk, &cc);
  let derived = xprv.derive(DerivationScheme::V2, index);
  let xprv_sk = derived.extended_secret_key();
  let xprv_cc = derived.chain_code();

  let mut sk_bytes = [0u8; 64];
  let mut cc_bytes = [0u8; 32];
  result[0].copy_to(&mut sk_bytes);
  result[1].copy_to(&mut cc_bytes);

  // derive_bytes() same as xprv.derive()
  assert_eq!(sk_bytes, xprv_sk);
  assert_eq!(&cc_bytes, xprv_cc);
  
  let expected_sk = [64, 87, 235, 108, 171, 144, 0, 227, 182, 254, 126, 85, 99, 65, 218, 28, 162, 245, 221, 224, 182, 137, 167, 181, 140, 185, 63, 25, 2, 223, 161, 90, 90, 16, 115, 47, 243, 72, 5, 28, 110, 8, 101, 198, 41, 49, 212, 167, 63, 168, 5, 11, 143, 245, 67, 180, 63, 192, 0, 10, 126, 44, 87, 0];
  let expected_cc = [154, 23, 15, 104, 156, 139, 155, 53, 2, 238, 132, 111, 69, 122, 179, 221, 27, 1, 124, 251, 44, 214, 136, 101, 199, 242, 77, 186, 188, 188, 34, 86];
  assert_eq!(sk_bytes, expected_sk);
  assert_eq!(cc_bytes, expected_cc);
}
