use std::{
    borrow::Cow,
    cmp::max,
    fmt::{Debug, Display},
    io::{BufReader, Read},
    iter,
    ops::{Add, BitXor, Index, Sub},
};

use base64ct::Base64;
use encoding_rs::{Encoding, WINDOWS_1252};

use crate::{PadWith, PaddingError, PaddingScheme, Unpad};

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Bytes(Vec<u8>);

impl Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.to_string(WINDOWS_1252).0, f)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(val: Bytes) -> Self {
        val.0
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        let value = Vec::from(value);
        Self(value)
    }
}

impl From<&u8> for Bytes {
    fn from(value: &u8) -> Self {
        Self(vec![*value])
    }
}

impl From<u8> for Bytes {
    fn from(value: u8) -> Self {
        Self(vec![value])
    }
}

impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl<Idx> Index<Idx> for Bytes
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        self.0.index(index)
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl BitXor for Bytes {
    type Output = Bytes;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::xor(&self, &rhs)
    }
}

impl BitXor for &Bytes {
    type Output = Bytes;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bytes::xor(self, rhs)
    }
}

impl Sub for &Bytes {
    type Output = u32;

    /// calculate the hamming distance
    ///
    /// ```rust
    /// use cryptopals::Bytes;
    /// let b1 = Bytes::from_ascii("this is a test");
    /// let b2 = Bytes::from_ascii("wokka wokka!!!");
    /// assert_eq!(&b1 - &b2, 37);
    ///
    /// let b1 = Bytes::from_ascii("this is a test");
    /// let b2 = Bytes::from_ascii("");
    /// assert_eq!(&b1 - &b2, b1.count_ones());
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        // extend the shorter iterator with zeroes
        let (short, long) = if self.len() < rhs.len() {
            (
                self.0.iter().chain(iter::once(&0x00u8).cycle()),
                rhs.0.iter(),
            )
        } else {
            (
                rhs.0.iter().chain(iter::once(&0x00u8).cycle()),
                self.0.iter(),
            )
        };

        #[allow(clippy::suspicious_arithmetic_impl)]
        short
            .zip(long)
            .map(|(lhs, rhs)| (lhs ^ rhs).count_ones())
            .sum()
    }
}

impl Sub for Bytes {
    type Output = u32;

    /// calculate the hamming distance
    ///
    /// ```rust
    /// use cryptopals::Bytes;
    /// let b1 = Bytes::from_ascii("this is a test");
    /// let b2 = Bytes::from_ascii("wokka wokka!!!");
    /// assert_eq!(b1 - b2, 37);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        (&self) - (&rhs)
    }
}

impl Add for Bytes {
    type Output = Bytes;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0.extend(rhs.0);
        self
    }
}

impl Add for &Bytes {
    type Output = Bytes;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self.0.clone();
        res.extend(&rhs.0[..]);
        Self::Output::from(res)
    }
}

impl PadWith for Bytes {
    fn pad_with(
        &mut self,
        block_size: usize,
        padding_scheme: impl PaddingScheme,
    ) -> Result<(), PaddingError> {
        self.0.extend(padding_scheme.pad_for(block_size, &self.0[..])?);
        Ok(())
    }
}

impl Unpad for Bytes {
    fn unpad_with(&mut self, padding_scheme: impl PaddingScheme) -> Result<(), PaddingError> {
        let padding_length = padding_scheme.padding_length(&self.0[..])?;
        self.0.truncate(self.len() - padding_length);
        Ok(())
    }
}

impl Bytes {
    pub fn from_hex(value: &str) -> anyhow::Result<Self> {
        Ok(Self::from(hex::decode(value)?))
    }

    pub fn from_base64(value: &str) -> anyhow::Result<Self> {
        use base64ct::Encoding;
        Ok(Self::from(Base64::decode_vec(value).unwrap()))
    }

    pub fn from_base64_stream<R>(reader: R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        use base64ct::Encoding;
        let mut reader = BufReader::new(reader);
        let mut buf = String::new();
        let _ = reader.read_to_string(&mut buf)?;
        let decoded = Base64::decode_vec(&buf.replace('\n', "")).unwrap();
        Ok(Self::from(decoded))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self[..])
    }

    pub fn to_base64(&self) -> String {
        use base64ct::Encoding;
        Base64::encode_string(&self[..])
    }

    pub fn from_string(value: &str, encoding: &'static Encoding) -> Self {
        Self::from(&encoding.encode(value).0[..])
    }

    pub fn from_ascii(value: &str) -> Self {
        Self::from_string(value, encoding_rs::WINDOWS_1252)
    }

    pub fn to_string<'a, 's>(&'s self, encoding: &'static Encoding) -> (Cow<'a, str>, bool)
    where
        's: 'a,
    {
        encoding.decode_without_bom_handling(&self[..])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn count_ones(&self) -> u32 {
        self.0.iter().map(|b| b.count_ones()).sum()
    }

    pub fn xor(lhs: &Self, rhs: &Self) -> Self {
        let mut ctr_left = 0;
        let mut ctr_right = 0;
        let bytes_count = max(lhs.0.len(), rhs.0.len());
        let mut result = Vec::with_capacity(bytes_count);

        for _ in 0..bytes_count {
            result.push(lhs[ctr_left] ^ rhs[ctr_right]);
            ctr_left = (ctr_left + 1) % lhs.0.len();
            ctr_right = (ctr_right + 1) % rhs.0.len();
        }

        Self::from(result)
    }

    pub fn chunkify(&self, size: usize) -> Vec<Vec<u8>> {
        self.0.chunks_exact(size).map(Vec::from).collect()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    pub fn append(&mut self, b: u8) {
        self.0.push(b)
    }
}
