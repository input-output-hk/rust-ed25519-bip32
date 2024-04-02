fn main() {
    uniffi::generate_scaffolding("./uniffi/ed25519_bip32.udl").unwrap();
}
