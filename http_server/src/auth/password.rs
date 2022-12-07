use argonautica::{Hasher, Verifier};

pub fn hash_password(password: &str, hash_secret: String) -> String {
    let mut hasher = Hasher::default();

    hasher
        .with_password(password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap()
}

pub fn verify_password(password: &str, password_hash: &str, hash_secret: String) -> bool {
    let mut verifier = Verifier::default();

    verifier
        .with_hash(password_hash)
        .with_password(password)
        .with_secret_key(hash_secret)
        .verify()
        .unwrap()
}
