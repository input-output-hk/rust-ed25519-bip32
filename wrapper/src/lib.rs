
mod wrapper;

pub use ed25519_bip32::{DerivationError, DerivationIndex, DerivationScheme};
pub use ed25519_bip32::{PrivateKeyError, PublicKeyError, XPrv, XPub, XPRV_SIZE, XPUB_SIZE};
pub use ed25519_bip32::{Signature, SignatureError, SIGNATURE_SIZE};
pub use wrapper::*;

uniffi::include_scaffolding!("ed25519_bip32");
