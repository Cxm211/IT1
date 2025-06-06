#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gnutls_cipher_algorithm {
    GNUTLS_CIPHER_UNKNOWN = 0,
    GNUTLS_CIPHER_NULL = 1,
    // etc…
}
pub use self::gnutls_cipher_algorithm as gnutls_cipher_algorithm_t;
// … 2000 lines later …
pub use self::gnutls_cipher_algorithm_t as gnutls_cipher_algorithm;