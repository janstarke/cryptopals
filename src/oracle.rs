use lazy_static::lazy_static;
use rand::random;

use crate::aes::AES;
use crate::padding::PadWith;
use crate::{Bytes, Key, Mode, Pkcs7, AES_BLOCKSIZE, IV};

lazy_static!{
    static ref ECB_KEY: Key = Key::random_128();
}

/// encryption oracle for challenge 11
pub fn encryption_oracle_c11(plaintext: &Bytes) -> anyhow::Result<(Bytes, bool)> {
    let input = random_bytes() + plaintext.clone() + random_bytes();
    let key = Key::random_128();
    let use_cbc = random();

    if use_cbc {
        let iv: IV = random();
        Ok((input.aes_cbc(Mode::Encrypt, &key, &iv)?, use_cbc))
    } else {
        Ok((input
            .padded_with(AES_BLOCKSIZE, Pkcs7)?
            .aes_ecb(Mode::Encrypt, &key)?, use_cbc))
    }
}

/// encryption oracle for challenge 12
pub fn encryption_oracle_c12(plaintext: &Bytes) -> anyhow::Result<Bytes> {
    let additional_content = Bytes::from_base64(
        "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
        aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
        dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
        YnkK")?;
    let input = plaintext.clone() + additional_content;

    input   
        .padded_with(AES_BLOCKSIZE, Pkcs7)?
        .aes_ecb(Mode::Encrypt, &ECB_KEY)
}

fn random_bytes() -> Bytes {
    let part1: [u8; 5] = random();
    let mut res = Vec::from(part1);
    for _ in 0..(random::<usize>() % 5) {
        res.push(random());
    }
    Bytes::from(res)
}
