fn main() {
    uniffi::generate_scaffolding("./ed25519_bip32.udl").unwrap();
}
