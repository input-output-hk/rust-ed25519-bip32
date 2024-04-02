use crate::{
    DerivationError,
    DerivationIndex,
    DerivationScheme,
    XPrv,
    XPRV_SIZE,
    XPub,
    XPUB_SIZE
};

/// This struct represents an XPubWrapper object.
///
/// # Fields
///
/// * `key` - A byte array of size `XPUB_SIZE` that holds the key data.
#[derive(Clone, Copy)]
pub struct XPubWrapper {
    pub key: [u8; XPUB_SIZE],
}

impl From<XPubWrapper> for XPub {
    /// Converts an `XPubWrapper` instance into an `XPub` instance
    ///
    /// # Arguments
    ///
    /// * `wrapper: XPubWrapper` - An instance of `XPubWrapper` that will be converted to `XPub`.
    ///
    /// # Returns
    ///
    /// * `Self` - This function will return a `XPub` instance.
    fn from(wrapper: XPubWrapper) -> Self {
        XPub::from_bytes(wrapper.key)
    }
}

impl From<XPub> for XPubWrapper {
    /// Converts an `XPub` value into an `XPubWrapper`, applying length checks.
    ///
    /// # Arguments
    ///
    /// * `value: XPub` - An instance of `XPub` that will be converted to `XPubWrapper`.
    ///
    /// # Returns
    ///
    /// * `Self` - This function will return an instance of `XPubWrapper` if the length of value is exactly equal to `XPUB_SIZE`.
    ///
    /// # Panics
    ///
    /// * This function will panic if `value` has a different length than `XPUB_SIZE`, with the error,
    ///   "Length must be XPUB_SIZE" where XPUB_SIZE is the expected size.
    ///
    /// # Note
    ///
    /// * The type `XPub` and `XPubWrapper` are not explicitly defined in your code provided. Please replace them with actual data types in your usage.
    fn from(value: XPub) -> Self {
        let clone = value.as_ref();
        if clone.len() == XPUB_SIZE {
            let key: [u8; XPUB_SIZE] = value.into();
            return XPubWrapper::new(key)
        } else {
            panic!("Length must be {}", XPUB_SIZE)
        }
    }
}

impl XPubWrapper {
    /// Creates a new `XPubWrapper` by providing an array of bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - An array of bytes representing the key.
    pub fn new(bytes: [u8; XPUB_SIZE]) -> Self {
        XPubWrapper {
            key: bytes
        }
    }

    /// Derives the object into a new instance based on the provided derivation scheme and index.
    ///
    /// # Arguments
    ///
    /// * `scheme` - The derivation scheme to use.
    /// * `index` - The derivation index to use.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DerivationError>` - The derived object on success, or an error on failure.
    pub fn derive(&self, scheme: DerivationScheme, index: DerivationIndex) -> Result<Self, DerivationError> {
        let x_pub: XPub = (*self).into();
        let result = x_pub.derive(scheme, index);
        return if result.is_ok() {
            Ok(result.unwrap().into())
        } else {
            Err(result.err().unwrap())
        }
    }
}

/// This struct represents an XPrvWrapper object.
///
/// # Fields
///
/// * `key` - A byte array of size XPRV_SIZE that holds the key data.
#[derive(Clone, Copy)]
pub struct XPrvWrapper {
    pub key: [u8; XPRV_SIZE],
}

impl From<XPrvWrapper> for XPrv {
    /// Converts an `XPrvWrapper` instance into an `XPrv` instance.
    ///
    /// # Arguments
    ///
    /// * `wrapper: XPrvWrapper` - An instance of `XPrvWrapper` that will be converted to an `XPrv`.
    ///
    /// # Returns
    ///
    /// * `Self` - This function returns an instance of `XPrv`.
    fn from(wrapper: XPrvWrapper) -> Self {
        XPrv::from_bytes(wrapper.key)
    }
}

impl From<XPrv> for XPrvWrapper {
    /// Converts an `XPrv` value into an `XPrvWrapper` by performing a length check.
    ///
    /// # Arguments
    ///
    /// * `value: XPrv` - An instance of `XPrv` which will be converted into `XPrvWrapper`.
    ///
    /// # Returns
    ///
    /// * `Self` - Returns an instance of `XPrvWrapper` if the length of the given value is exactly equal to `XPRV_SIZE`.
    ///
    /// # Panics
    ///
    /// * The function will panic if the length of `value` is not equal to `XPRV_SIZE`, with the message "Length must be XPRV_SIZE", where `XPRV_SIZE` is the expected size.
    fn from(value: XPrv) -> Self {
        let clone = value.as_ref();
        if clone.len() == XPRV_SIZE {
            let key: [u8; XPRV_SIZE] = value.into();
            return XPrvWrapper {
                key
            }
        } else {
            panic!("Length must be {}", XPRV_SIZE)
        }
    }
}

impl XPrvWrapper {
    /// Get the associated `XPubWrapper`
    pub fn public(&self) -> XPubWrapper {
        let x_prv: XPrv = (*self).into();
        return x_prv.public().into();
    }

    /// Derives a new private key from an existing one according to the specified derivation scheme and index.
    ///
    /// # Arguments
    ///
    /// * `scheme: DerivationScheme` - The derivation scheme to be used for generating the new private key.
    ///
    /// * `index: DerivationIndex` - The index to be used in the derivation of the new private key.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DerivationError>` - Returns a new instance of the private key if the derivation is successful.
    /// If the derivation fails, it returns a `DerivationError`.
    pub fn derive(&self, scheme: DerivationScheme, index: DerivationIndex) -> Result<Self, DerivationError> {
        let x_prv: XPrv = (*self).into();
        return Ok(x_prv.derive(scheme, index).into());
    }
}
