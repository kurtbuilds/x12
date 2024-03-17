use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde::{ser, de};
use serde::de::Error;

#[derive(Clone, Copy, PartialEq)]
pub struct Numeric<const POSITION: usize>(u64);

impl<const N: usize> Numeric<N> {
    pub fn from_raw(v: u64) -> Self {
        Self(v)
    }
}

impl<const N: usize> fmt::Debug for Numeric<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.0 as f64;
        let v = v / 10_f64.powi(N as i32);
        write!(f, "{:.*}", N, v)
    }
}

impl<const N: usize> fmt::Display for Numeric<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.0 as f64;
        let v = v / 10_f64.powi(N as i32);
        write!(f, "{:.*}", N, v)
    }
}

impl<const N: usize> From<f64> for Numeric<N> {
    fn from(v: f64) -> Self {
        let v = v * 10_f64.powi(N as i32);
        Numeric(v as u64)
    }
}

impl<const N: usize> From<u32> for Numeric<N> {
    fn from(v: u32) -> Self {
        let v = v as u64 * 10_u64.pow(N as u32);
        Numeric(v as u64)
    }
}

impl<const N: usize> From<u64> for Numeric<N> {
    fn from(v: u64) -> Self {
        let v = v * 10_u64.pow(N as u32);
        Numeric(v)
    }
}

impl<const N: usize> Serialize for Numeric<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ser::Serializer {
        if serializer.is_human_readable() {
            let value = self.0.to_string();
            serializer.serialize_str(&value)
        } else {
            serializer.serialize_str(&self.0.to_string())
        }
    }
}

struct FixedVisitor<const N: usize>;

impl<'de, const N: usize> de::Visitor<'de> for FixedVisitor<N> {
    type Value = Numeric<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a fixed length string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        u64::from_str(v).map(Numeric).map_err(E::custom)
    }
}

impl<'de, const N: usize> Deserialize<'de> for Numeric<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: de::Deserializer<'de> {
        deserializer.deserialize_str(FixedVisitor::<N>)
    }
}