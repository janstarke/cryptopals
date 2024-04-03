use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaddingError {
    #[error("unsupported block size")]
    UnsupportedBlockSize,
}

pub trait PaddingScheme {
    fn pad_for(&self, block_size: usize, data: &[u8]) -> Result<Vec<u8>, PaddingError>;
}

pub trait PadWith: Sized {
    fn pad_with(
        &mut self,
        block_size: usize,
        padding_scheme: impl PaddingScheme,
    ) -> Result<(), PaddingError>;

    fn padded_with(
        self,
        block_size: usize,
        padding_scheme: impl PaddingScheme,
    ) -> Result<Self, PaddingError>;
}

pub struct Pkcs7;

impl PaddingScheme for Pkcs7 {
    fn pad_for(&self, block_size: usize, data: &[u8]) -> Result<Vec<u8>, PaddingError> {
        if block_size > 0xff {
            return Err(PaddingError::UnsupportedBlockSize);
        }

        let last_block_size = data.len() % block_size;

        let pad_size = if last_block_size > 0 {
            block_size - last_block_size
        } else {
            0
        };
        debug_assert!(pad_size <= 0xff);

        let pad_value = u8::try_from(pad_size).unwrap();
        debug_assert_eq!((data.len() + pad_size) % block_size, 0);
        Ok(vec![pad_value; pad_size])
    }
}
