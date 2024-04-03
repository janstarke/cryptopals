use std::fs::File;

use anyhow::Result;
use cryptopals::{Bytes, Key, Mode, PadWith, Pkcs7, AES, AES_BLOCKSIZE, IV};
use encoding_rs::WINDOWS_1252;
use rand::random;

fn main() -> Result<()> {
    let data = Bytes::from_ascii("Lorem ipsum dolor sit amet duis. Qui nulla minim enim est dolor \
    adipisicing eiusmod qui veniam fugiat exercitation dolor. Esse magna qui do exercitation eu dolor consequat \
    elit officia est ex in mollit eiusmod dolore anim proident excepteur culpa.");

    for _ in 0..10 {
        let input = random_bytes() + data.clone() + random_bytes();
        let key = Key::random_128();
        let use_cbc = random();

        let encrypted = if use_cbc {
            let iv: IV = random();
            input.aes_cbc(Mode::Decrypt, &key, &iv)?
        } else {
            input
                .padded_with(AES_BLOCKSIZE, Pkcs7)?
                .aes_ecb(Mode::Decrypt, &key)?
        };

        if is_using_cbc(&encrypted) == use_cbc {
            println!("correct decision");
        } else {
            println!("wrong decision");
        }
    }

    let data = Bytes::from_base64_stream(File::open("data/10.txt")?)?;
    let key = Bytes::from_ascii("YELLOW SUBMARINE").try_into()?;
    let iv = Bytes::from(vec![0; AES_BLOCKSIZE]).try_into()?;
    let decrypted = data.aes_cbc(Mode::Decrypt, &key, &iv)?;
    println!("{}", decrypted.to_string(WINDOWS_1252).0);
    Ok(())
}

fn is_using_cbc(_encrypted: &Bytes) -> bool {
    true
}

fn random_bytes() -> Bytes {
    let part1: [u8; 5] = random();
    let mut res = Vec::from(part1);
    for _ in 0..(random::<usize>() % 5) {
        res.push(random());
    }
    Bytes::from(res)
}
