use std::iter::FromIterator;

use serde_derive::{Deserialize, Serialize};

use crate::types::*;

/// A list of `Raw`, which helps you store much bigger data than a `Raw`
#[derive(Clone, Serialize, Deserialize)]
pub struct Row {
    pub(super) inner: Vec<Raw>,
    _buffer: Vec<u8>,
}

impl Row {
    pub fn new(slice: &[u8]) -> Self {
        let inner: Vec<Raw> = slice.iter().copied().collect::<Vec<u8>>().chunks(32).fold(
            Vec::<Raw>::new(),
            |mut vec, chunk| {
                vec.push(chunk.into());
                vec
            },
        );
        Self {
            inner,
            ..Default::default()
        }
    }

    pub fn make_buffer(&mut self) {
        self._buffer = self.inner.iter().fold(Vec::<u8>::new(), |mut vec, raw| {
            vec.extend_from_slice(raw.as_ref());
            vec
        });
    }

    pub fn clean_buffer(&mut self) {
        self._buffer = Vec::new();
    }

    pub fn to_utf8_string(&self) -> Result<String, std::string::FromUtf8Error> {
        let buf = self.inner.iter().fold(Vec::<u8>::new(), |mut vec, raw| {
            vec.extend_from_slice(raw.as_ref());
            vec
        });
        String::from_utf8(buf)
    }

    pub fn into_raw_vec(self) -> Vec<Raw> {
        self.inner
    }

    pub fn into_u8_vec(mut self) -> Vec<u8> {
        self.make_buffer();
        self._buffer
    }

    /// wipe the header with header size in bytes
    pub fn wipe_header(&mut self, header_size: usize) {
        assert!(header_size <= 32);
        self.inner[0].wipe_header(header_size);
    }
}

impl Default for Row {
    fn default() -> Self {
        Row {
            inner: Vec::new(),
            _buffer: Vec::new(),
        }
    }
}

impl ExactSizeIterator for Row {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl From<Vec<Raw>> for Row {
    fn from(v: Vec<Raw>) -> Self {
        Self {
            inner: v,
            ..Default::default()
        }
    }
}

impl From<&[Raw]> for Row {
    fn from(v: &[Raw]) -> Self {
        Self {
            inner: v.into(),
            ..Default::default()
        }
    }
}

impl Iterator for Row {
    type Item = u8;

    #[allow(clippy::never_loop)]
    fn next(&mut self) -> Option<Self::Item> {
        for raw in self.inner.iter() {
            for b in raw.bytes.iter() {
                return Some(*b);
            }
        }
        None
    }
}

impl Iterator for &Row {
    type Item = u8;

    #[allow(clippy::never_loop)]
    fn next(&mut self) -> Option<Self::Item> {
        for raw in self.inner.iter() {
            for b in raw.bytes.iter() {
                return Some(*b);
            }
        }
        None
    }
}

impl FromIterator<u8> for Row {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = u8>,
    {
        let row: Vec<Raw> = iter.into_iter().collect::<Vec<u8>>().chunks(32).fold(
            Vec::<Raw>::new(),
            |mut vec, chunk| {
                vec.push(chunk.into());
                vec
            },
        );
        row.into()
    }
}

impl From<&[u8]> for Row {
    fn from(slice: &[u8]) -> Self {
        Row::new(slice)
    }
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        Self::from(s.as_bytes())
    }
}

impl From<String> for Row {
    fn from(s: String) -> Self {
        Self::from(s.as_bytes())
    }
}

impl From<&String> for Row {
    fn from(s: &String) -> Self {
        Self::from(s.as_bytes())
    }
}

impl From<&Raw> for Row {
    fn from(v: &Raw) -> Self {
        Self {
            inner: vec![*v],
            ..Default::default()
        }
    }
}

impl From<Raw> for Row {
    fn from(v: Raw) -> Self {
        Self {
            inner: vec![v],
            ..Default::default()
        }
    }
}

impl From<Vec<u8>> for Row {
    fn from(v: Vec<u8>) -> Self {
        let inner: Vec<Raw> = v.into_iter().collect::<Vec<u8>>().chunks(32).fold(
            Vec::<Raw>::new(),
            |mut vec, chunk| {
                vec.push(chunk.into());
                vec
            },
        );
        Self {
            inner,
            ..Default::default()
        }
    }
}

impl From<Box<[u8]>> for Row {
    fn from(v: Box<[u8]>) -> Self {
        Row::new(&v)
    }
}

impl std::borrow::Borrow<[u8]> for Row {
    fn borrow(&self) -> &[u8] {
        if !self.inner.is_empty() && self._buffer.is_empty() {
            panic!("make buffer before brrow")
        }
        self._buffer.as_ref()
    }
}

impl std::borrow::Borrow<[u8]> for &Row {
    fn borrow(&self) -> &[u8] {
        if !self.inner.is_empty() && self._buffer.is_empty() {
            panic!("make buffer before brrow")
        }
        self._buffer.as_ref()
    }
}
