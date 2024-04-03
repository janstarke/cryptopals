use openssl::symm::{decrypt, Cipher};

use crate::Bytes;

pub trait AES: Sized {
    fn decrypt_with_aes_128_ecb(&self, key: &Self) -> anyhow::Result<Self>;
}

impl AES for Bytes {
    fn decrypt_with_aes_128_ecb(&self, key: &Self) -> anyhow::Result<Self> {
        Ok(Bytes::from(decrypt(
            Cipher::aes_128_ecb(),
            &key[..],
            None,
            &self[..],
        )?))
    }
}
