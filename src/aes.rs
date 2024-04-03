use openssl::symm::{Cipher, Crypter};
use thiserror::Error;

use crate::{Bytes, PadWith, Pkcs7, Unpad};

#[derive(Debug, Error)]
pub enum AESError {
    #[error("invalid key size")]
    InvalidKeySize,

    #[error("invalid IV size")]
    InvalidIVSize,

    #[error("invalid ciphertext length")]
    InvalidCyphertextLength,

    #[error("received unpadded data of length {0}; but data length must be a multiple of the block size")]
    UnpaddedData(usize),
}

#[derive(Copy, Clone)]
pub enum Mode {
    Decrypt,
    Encrypt,
}

impl From<Mode> for openssl::symm::Mode {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Decrypt => openssl::symm::Mode::Decrypt,
            Mode::Encrypt => openssl::symm::Mode::Encrypt,
        }
    }
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
        if self.len() % AES_BLOCKSIZE != 0 {
            return Err(AESError::UnpaddedData(self.len()).into());
        }

        let mut crypter = Crypter::new(key.ecb_cipher(), mode.into(), key.bytes(), None)?;
        crypter.pad(false);
        let mut output = vec![0; self.len() + AES_BLOCKSIZE];

        let mut count = 0;
        count += crypter.update(&self[..], &mut output[count..])?;
        count += crypter.finalize(&mut output[count..])?;

        Ok(Bytes::from(&output[0..count]))
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

                result = Bytes::from(result).unpadded(Pkcs7)?.into_inner();

            }
            Mode::Encrypt => {
                for chunk in self[..].chunks_exact(AES_BLOCKSIZE).map(Bytes::from) {
                    let encrypted = (chunk ^ previous_block).aes_ecb(mode, key)?;
                    result.extend(&encrypted[..]);
                    previous_block = encrypted;
                }
                let remainder = Bytes::from(self[..].chunks_exact(AES_BLOCKSIZE).remainder());
                if !remainder.is_empty() {
                    let encrypted = (remainder.padded_with(AES_BLOCKSIZE, Pkcs7)? ^ previous_block)
                        .aes_ecb(mode, key)?;
                    result.extend(&encrypted[..]);
                }
            }
        }

        Ok(result.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bytes, Mode, AES, AES_BLOCKSIZE};

    #[test]
    fn test_ecb_without_padding() {
        let test_data = Bytes::from_ascii("Lorem ipsum dolo");
        let key = Bytes::from_ascii("YELLOW SUBMARINE");
        assert_eq!(key.len(), 16);
        let key = key.try_into().unwrap();

        let encrypted = test_data.aes_ecb(Mode::Encrypt, &key).unwrap();
        let decrypted = encrypted.aes_ecb(Mode::Decrypt, &key).unwrap();

        assert_eq!(test_data, decrypted);
    }

    #[test]
    fn test_cbc() {
        let test_data = Bytes::from_ascii("Lorem ipsum dolor sit amet duis. Qui nulla minim enim est dolor \
        adipisicing eiusmod qui veniam fugiat exercitation dolor. Esse magna qui do exercitation eu dolor consequat \
        elit officia est ex in mollit eiusmod dolore anim proident excepteur culpa.");
        let key = Bytes::from_ascii("YELLOW SUBMARINE");
        assert_eq!(key.len(), 16);
        let key = key.try_into().unwrap();

        let iv = vec![0; AES_BLOCKSIZE].try_into().unwrap();
        let encrypted = test_data.aes_cbc(Mode::Encrypt, &key, &iv).unwrap();
        assert!(encrypted.len() > test_data.len());
        assert_eq!(encrypted.len() % AES_BLOCKSIZE, 0);

        let decrypted = encrypted.aes_cbc(Mode::Decrypt, &key, &iv).unwrap();

        assert_eq!(test_data, decrypted);
    }
}
