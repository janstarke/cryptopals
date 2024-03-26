use std::{
    cmp::max,
    ops::{BitXor, Index},
};

pub struct Bytes(Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
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
        let mut ctr_left = 0;
        let mut ctr_right = 0;
        let bytes_count = max(self.0.len(), rhs.0.len());
        let mut result = Vec::with_capacity(bytes_count);

        for _ in 0..bytes_count {
            result.push(self[ctr_left] ^ rhs[ctr_right]);
            ctr_left = (ctr_left + 1) % self.0.len();
            ctr_right = (ctr_right + 1) % rhs.0.len();
        }

        Self::from(result)
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
}
