
use anyhow::Result;
use cryptopals::{Bytes, Key, Mode, PadWith, Pkcs7, AES, AES_BLOCKSIZE, IV};
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
            input.aes_cbc(Mode::Encrypt, &key, &iv)?
        } else {
            input
                .padded_with(AES_BLOCKSIZE, Pkcs7)?
                .aes_ecb(Mode::Encrypt, &key)?
        };

        if is_using_cbc(&encrypted) == use_cbc {
            println!("correct decision");
        } else {
            println!("wrong decision");
        }
    }

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
