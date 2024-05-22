import * as pkg from "./pkg/ed25519_bip32_wasm";
import wasm from "./pkg/ed25519_bip32_wasm_bg.wasm";

pkg.initSync(wasm());

export default pkg;
