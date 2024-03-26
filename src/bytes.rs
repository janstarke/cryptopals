use std::{
    borrow::Cow,
    cmp::max,
    ops::{BitXor, Index},
};

use encoding_rs::Encoding;

#[derive(Clone)]
pub struct Bytes(Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        let value = Vec::from(value);
        Self(value)
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

impl Bytes {
    pub fn from_hex(value: &str) -> anyhow::Result<Self> {
        Ok(Self::from(hex::decode(value)?))
    }

    pub fn from_base64(value: &str) -> anyhow::Result<Self> {
        use base64::prelude::*;
        Ok(Self::from(BASE64_STANDARD.decode(value)?))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self[..])
    }

    pub fn to_base64(&self) -> String {
        use base64::prelude::*;
        BASE64_STANDARD.encode(&self[..])
    }

    pub fn to_string<'a, 's>(&'s self, encoding: &'static Encoding) -> (Cow<'a, str>, bool)
    where
        's: 'a,
    {
        encoding.decode_without_bom_handling(&self[..])
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
}
