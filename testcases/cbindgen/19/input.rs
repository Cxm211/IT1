
use sodiumoxide::crypto::box_::{PublicKey, SecretKey};

#[no_mangle]
pub extern fn test(pk: &PublicKey, sk: &SecretKey) {}