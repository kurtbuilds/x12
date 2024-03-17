use std::fmt;

use serde::{Deserialize, Serialize};
use serde::{ser, de};
use serde::de::{Error};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Response {
    Yes,
    No,
}

impl Response {
    pub fn code(&self) -> &str {
        match self {
            Response::Yes => "Y",
            Response::No => "N",
        }
    }
    pub fn value(&self) -> bool {
        match self {
            Response::Yes => true,
            Response::No => false,
        }
    }
}

impl Serialize for Response  {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ser::Serializer {
        if serializer.is_human_readable() {
            serializer.serialize_bool(self.value())
        } else {
            serializer.serialize_str(self.code())
        }
    }
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Response;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a fixed length string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        if v == "Y" {
            Ok(Response::Yes)
        } else if v == "N" {
            Ok(Response::No)
        } else {
            Err(E::custom("invalid response"))
        }
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: Error {
        if v {
            Ok(Response::Yes)
        } else {
            Ok(Response::No)
        }
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: de::Deserializer<'de> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_bool(Visitor)
        } else {
            deserializer.deserialize_str(Visitor)
        }
    }
}