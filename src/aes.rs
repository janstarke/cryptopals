use openssl::symm::{decrypt, encrypt, Cipher};
use thiserror::Error;

use crate::{Bytes, PadWith, Pkcs7};

#[derive(Debug, Error)]
pub enum AESError {
    #[error("invalid key size")]
    InvalidKeySize,

    #[error("invalid IV size")]
    InvalidIVSize,

    #[error("invalid ciphertext length")]
    InvalidCyphertextLength,
}

#[derive(Copy, Clone)]
pub enum Mode {
    Decrypt,
    Encrypt,
}

#[derive(Copy, Clone)]
pub enum Key {
    AES128([u8; 16]),
    AES192([u8; 24]),
    AES256([u8; 32]),
}

impl TryFrom<Bytes> for Key {
    type Error = AESError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let bytes: Vec<_> = value.into();
        match bytes.len() {
            16 => Ok(Self::AES128(bytes.try_into().unwrap())),
            24 => Ok(Self::AES192(bytes.try_into().unwrap())),
            32 => Ok(Self::AES256(bytes.try_into().unwrap())),
            _ => Err(AESError::InvalidKeySize),
        }
    }
}

impl Key {
    pub fn ecb_cipher(&self) -> Cipher {
        match self {
            Key::AES128(_) => Cipher::aes_128_ecb(),
            Key::AES192(_) => Cipher::aes_192_ecb(),
            Key::AES256(_) => Cipher::aes_256_ecb(),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            Key::AES128(key_bytes) => key_bytes,
            Key::AES192(key_bytes) => key_bytes,
            Key::AES256(key_bytes) => key_bytes,
        }
    }
}

pub const AES_BLOCKSIZE: usize = 16;
type IV = [u8; AES_BLOCKSIZE];

impl TryFrom<Bytes> for IV {
    type Error = AESError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let bytes: Vec<_> = value.into();
        if bytes.len() == 16 {
            Ok(bytes.try_into().unwrap())
        } else {
            Err(AESError::InvalidIVSize)
        }
    }
}

pub trait AES: Sized {
    fn aes_ecb(&self, mode: Mode, key: &Key) -> anyhow::Result<Self>;
    fn aes_cbc(&self, mode: Mode, key: &Key, iv: &IV) -> anyhow::Result<Self>;
}

impl AES for Bytes {
    fn aes_ecb(&self, mode: Mode, key: &Key) -> anyhow::Result<Self> {
        let result = match mode {
            Mode::Decrypt => decrypt(key.ecb_cipher(), key.bytes(), None, &self[..])?,
            Mode::Encrypt => encrypt(key.ecb_cipher(), key.bytes(), None, &self[..])?,
        };
        Ok(result.into())
    }

    fn aes_cbc(&self, mode: Mode, key: &Key, iv: &IV) -> anyhow::Result<Self> {
        let mut result = Vec::new();
        let mut previous_block = Bytes::from(&iv[..]);

        match mode {
            Mode::Decrypt => {
                if self.len() % AES_BLOCKSIZE != 0 {
                    return Err(AESError::InvalidCyphertextLength.into());
                }

                for chunk in self[..].chunks_exact(AES_BLOCKSIZE).map(Bytes::from) {
                    let decrypted = chunk.aes_ecb(mode, key)?;
                    let xored = &decrypted ^ &previous_block;
                    result.extend(&xored[..]);
                    previous_block = chunk;
                }
            }
            Mode::Encrypt => {
                for chunk in self[..].chunks_exact(AES_BLOCKSIZE).map(Bytes::from) {
                    let encrypted = (chunk ^ previous_block).aes_ecb(mode, key)?;
                    result.extend(&encrypted[..]);
                    previous_block = encrypted;
                }
                let mut remainder = Bytes::from(self[..].chunks_exact(AES_BLOCKSIZE).remainder());
                if !remainder.is_empty() {
                    remainder.pad_with(AES_BLOCKSIZE, Pkcs7)?;
                    let encrypted = (remainder ^ previous_block).aes_ecb(mode, key)?;
                    result.extend(&encrypted[..]);
                }
            }
        }

        Ok(result.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::padding::PadWith;
    use crate::{Bytes, Mode, Pkcs7, AES, AES_BLOCKSIZE};

    #[test]
    fn test_cbc() {
        let test_data = Bytes::from_ascii("Lorem ipsum dolor sit amet duis. Qui nulla minim enim est dolor \
        adipisicing eiusmod qui veniam fugiat exercitation dolor. Esse magna qui do exercitation eu dolor consequat \
        elit officia est ex in mollit eiusmod dolore anim proident excepteur culpa.");
        let key = Bytes::from_ascii("YELLOW SUBMARINE")
            .padded_with(16, Pkcs7)
            .unwrap();
        assert_eq!(key.len(), 16);
        let key = key.try_into().unwrap();

        let iv = vec![0;AES_BLOCKSIZE].try_into().unwrap();
        let encrypted = test_data.aes_cbc(Mode::Encrypt, &key, &iv).unwrap();
        let decrypted = encrypted.aes_cbc(Mode::Decrypt, &key, &iv).unwrap();

        assert_eq!(test_data, decrypted);
    }
}
