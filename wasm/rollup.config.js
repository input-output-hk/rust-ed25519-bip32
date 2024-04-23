const { wasm } = require('@rollup/plugin-wasm');

module.exports = {
  input: './index.js',
  output: {
    file: 'build/ed25519_bip32_wasm.js',
    format: 'cjs'
  },
  plugins: [wasm({
     targetEnv: "auto-inline",
     sync: ["pkg/ed25519_bip32_wasm_bg.wasm"],
  })]
};
