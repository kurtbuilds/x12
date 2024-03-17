use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde::{ser, de};
use serde::de::Error;

#[derive(Clone, Copy, PartialEq)]
pub struct Fixed<const N: usize>([u8; N]);

impl<const N: usize> fmt::Debug for Fixed<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = std::str::from_utf8(&self.0).unwrap();
        write!(f, "{:?}", value)
    }
}

impl std::ops::Deref for Fixed<1> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<const N: usize> Fixed<N> {
    pub fn new(s: &[u8]) -> Self {
        if s.len() != N {
            panic!("wrong byte slice length");
        }
        let mut bytes = [b' '; N];
        bytes[..s.len()].copy_from_slice(s);
        Fixed(bytes)
    }

    pub fn as_arr(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> PartialEq<&[u8]> for Fixed<N> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == **other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Fixed<N> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> fmt::Display for Fixed<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = std::str::from_utf8(&self.0).unwrap();
        write!(f, "{}", value.trim())
    }
}

#[derive(Debug)]
pub struct InvalidLength;

impl<const N: usize> FromStr for Fixed<N> {
    type Err = InvalidLength;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > N {
            return Err(InvalidLength);
        }
        let mut bytes = [b' '; N];
        bytes[..s.len()].copy_from_slice(s.as_bytes());
        Ok(Fixed(bytes))
    }
}

impl<const N: usize> Serialize for Fixed<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ser::Serializer {
        if serializer.is_human_readable() {
            let value = std::str::from_utf8(&self.0).unwrap();
            let value = value.trim();
            serializer.serialize_str(value)
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

struct FixedVisitor<const N: usize> {
    exact: bool,
}

impl<'de, const N: usize> de::Visitor<'de> for FixedVisitor<N> {
    type Value = Fixed<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a fixed length string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        if v.len() > N || (self.exact && v.len() < N) {
            return Err(E::invalid_length(v.len(), &format!("a string of length {}", N).as_str()));
        }
        let mut bytes = [b' '; N];
        bytes[..v.len()].copy_from_slice(v.as_bytes());
        Ok(Fixed(bytes))
    }
}

impl<'de, const N: usize> Deserialize<'de> for Fixed<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: de::Deserializer<'de> {
        let exact = !deserializer.is_human_readable();
        deserializer.deserialize_str(FixedVisitor::<N> { exact })
    }
}