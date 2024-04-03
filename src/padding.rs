use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaddingError {
    #[error("unsupported block size")]
    UnsupportedBlockSize,

    #[error("illegal padding result detected")]
    IllegalPadding,
}

pub trait PaddingScheme {
    fn pad_for(&self, block_size: usize, data: &[u8]) -> Result<Vec<u8>, PaddingError>;
    fn padding_length(&self, data: &[u8]) -> Result<usize, PaddingError>;
}

pub trait PadWith: Sized {
    fn pad_with(
        &mut self,
        block_size: usize,
        padding_scheme: impl PaddingScheme,
    ) -> Result<(), PaddingError>;

    fn padded_with(
        mut self,
        block_size: usize,
        padding_scheme: impl PaddingScheme,
    ) -> Result<Self, PaddingError> {
        self.pad_with(block_size, padding_scheme)?;
        Ok(self)
    }
}

pub trait Unpad: Sized {
    fn unpad_with(&mut self, padding_scheme: impl PaddingScheme) -> Result<(), PaddingError>;
    fn unpadded(mut self, padding_scheme: impl PaddingScheme) -> Result<Self, PaddingError> {
        self.unpad_with(padding_scheme)?;
        Ok(self)
    }
}

pub struct Pkcs7;

impl PaddingScheme for Pkcs7 {
    /* <https://datatracker.ietf.org/doc/html/rfc5652#section-6.3>
    Some content-encryption algorithms assume the input length is a
    multiple of k octets, where k is greater than one.  For such
    algorithms, the input shall be padded at the trailing end with
    k-(lth mod k) octets all having value k-(lth mod k), where lth is
    the length of the input.  In other words, the input is padded at
    the trailing end with one of the following strings:

                      01 -- if lth mod k = k-1
                   02 02 -- if lth mod k = k-2
                       .
                       .
                       .
             k k ... k k -- if lth mod k = 0

    The padding can be removed unambiguously since all input is padded,
    including input values that are already a multiple of the block size,
    and no padding string is a suffix of another.  This padding method is
    well defined if and only if k is less than 256.
      */
    fn pad_for(&self, block_size: usize, data: &[u8]) -> Result<Vec<u8>, PaddingError> {
        if block_size > 0xff {
            return Err(PaddingError::UnsupportedBlockSize);
        }

        let pad_size = block_size - (data.len() % block_size);

        assert!(pad_size > 0);
        debug_assert!(pad_size <= 0xff);

        let pad_value = u8::try_from(pad_size).unwrap();
        debug_assert_eq!((data.len() + pad_size) % block_size, 0);
        Ok(vec![pad_value; pad_size])
    }

    fn padding_length(&self, data: &[u8]) -> Result<usize, PaddingError> {
        if let Some(padding_byte) = data.last() {
            let padding_count = usize::from(*padding_byte);
            let last_data_index = data.len() - padding_count;

            if (last_data_index..(last_data_index + padding_count))
                .map(|idx| data[idx])
                .any(|val| val != *padding_byte)
            {
                Err(PaddingError::IllegalPadding)
            } else {
                Ok(padding_count)
            }
        } else {
            Ok(0)
        }
    }
}
