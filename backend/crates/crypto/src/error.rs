//! The one error of the crypto port (phase 00, 7.22).

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CryptoError {
    /// OpenBao, or a key it holds, cannot be reached or refused the call. A caller answers 503
    /// `E_UNAVAILABLE` unless its own error section says otherwise.
    #[error("the key service is unavailable: {0}")]
    Unavailable(String),
    /// The value is not a sealed value, or its tag and AAD check failed: it was changed, moved to another row
    /// or sealed under another family.
    #[error("the sealed value was rejected")]
    Rejected,
    /// No key of that name, id or purpose is known to this process.
    #[error("unknown key: {0}")]
    UnknownKey(String),
}
