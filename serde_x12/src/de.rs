use std::fmt::{Display, Formatter};

use serde::de::{DeserializeSeed, EnumAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserializer;

use crate::{Level, X12Formatter};
use crate::formatter::{detect_format, split_string};
use crate::debug;

pub fn from_str<'de, T>(input: &'de str) -> Result<T, X12DeserializerError> where T: serde::Deserialize<'de> {
    from_str_returning_format(input).map(|(v, _)| v)
}

pub fn from_str_returning_format<'de, T>(input: &'de str) -> Result<(T, X12Formatter), X12DeserializerError> where T: serde::Deserialize<'de> {
    let formatter = detect_format(input).unwrap_or_default();
    from_str_using_format(input, formatter).map(|v| (v, formatter))
}

pub fn from_str_using_format<'de, T>(input: &'de str, formatter: X12Formatter) -> Result<T, X12DeserializerError> where T: serde::Deserialize<'de> {
    let mut deserializer = X12Deserializer::new(input, formatter, Level::Segment);
    let value = T::deserialize(&mut deserializer)?;
    if let Some(buf) = deserializer.buffer {
        Err(X12DeserializerError::TrailingCharacters {
            buffer: buf.to_string(),
        })
    } else {
        Ok(value)
    }
}



#[derive(Debug)]
pub enum X12DeserializerError {
    TrailingCharacters { buffer: String },
    EarlyEndOfDocument,
    InvalidType {
        expected: &'static str,
        got: String,
    },
    UnexpectedSegment {
        expected: &'static str,
        got: String,
    },
    GenericError(String),
}

impl X12DeserializerError {
    pub fn forgivable(&self) -> bool {
        match self {
            X12DeserializerError::TrailingCharacters { .. } => false,
            X12DeserializerError::EarlyEndOfDocument => true,
            X12DeserializerError::InvalidType { .. } => false,
            X12DeserializerError::UnexpectedSegment { .. } => true,
            X12DeserializerError::GenericError(_) => false,
        }
    }
}

impl std::error::Error for X12DeserializerError {}

impl Display for X12DeserializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl serde::de::Error for X12DeserializerError {
    fn custom<T: Display>(msg: T) -> Self {
        let s = msg.to_string();
        if s.starts_with("invalid length 0, expected a string of length") {
            return X12DeserializerError::EarlyEndOfDocument;
        }
        X12DeserializerError::GenericError(msg.to_string())
    }
}

// initially, for subelement
pub struct X12Deserializer<'de> {
    pub(crate) buffer: Option<&'de str>,
    formatter: X12Formatter,
    delimiter: u8,
    level: Level,
}

impl<'de> X12Deserializer<'de> {
    pub fn new(buffer: &'de str, formatter: X12Formatter, level: Level) -> Self {
        Self {
            buffer: Some(buffer),
            delimiter: formatter.delimiter(level),
            formatter,
            level,
        }
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut X12Deserializer<'de> {
    type Error = X12DeserializerError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        // eprintln!("BufferDeserializer({})::deserialize_any", self.level);
        let Some(b) = self.buffer else {
            return Err(X12DeserializerError::EarlyEndOfDocument);
        };
        // eprintln!("buffer: {}", b);
        let (el, rest) = split_string(b, self.delimiter as char);
        self.buffer = rest;
        if el.contains(self.formatter.sub_element_delimiter as char) {
            visitor.visit_seq(&mut X12Deserializer::new(el, self.formatter, Level::Subelement))
        } else {
            // eprintln!("visiting str");
            visitor.visit_str(el)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let Some(el) = self.buffer else {
            return Err(X12DeserializerError::EarlyEndOfDocument);
        };
        let (element, rest) = split_string(el, self.delimiter as char);
        self.buffer = rest;
        visitor.visit_str(element)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let Some(el) = self.buffer else {
            return Err(X12DeserializerError::EarlyEndOfDocument);
        };
        let (element, rest) = split_string(el, self.delimiter as char);
        self.buffer = rest;
        visitor.visit_str(element)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let Some(el) = self.buffer else {
            return Err(X12DeserializerError::EarlyEndOfDocument);
        };
        let (element, rest) = split_string(el, self.delimiter as char);
        self.buffer = rest;
        visitor.visit_bytes(element.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let next = self.buffer
            .and_then(|b| b.split(self.delimiter as char).next());
        debug!("BufferDeserializer::deserialize_option: {:?}", next);
        if self.buffer.is_none() {
            return visitor.visit_none();
        }
        if let Some(buf) = self.buffer.as_mut() {
            if buf.as_bytes()[0] == self.delimiter {
                *buf = &buf[1..];
                return visitor.visit_none()
            }
        }
        // this only works because all the visitors ive seen are ZSTs
        // if they did have state, this would be a disaster
        let insanely_unsafe_copy_of_visitor = unsafe {
            std::ptr::read(&visitor as *const V)
        };
        let mut s = visitor.visit_some(self);
        if let Err(s) = &s {
            if s.forgivable() {
                return insanely_unsafe_copy_of_visitor.visit_none()
            }
        }
        s
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let next = self.buffer
            .and_then(|b| b.split(self.delimiter as char).next());
        debug!("BufferDeserializer::deserialize_seq: {:?}", next);
        visitor.visit_seq(self)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_struct<V>(mut self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let next = self.buffer
            .and_then(|b| b.split(self.delimiter as char).next());
        debug!("BufferDeserializer::deserialize_struct: {}, encountered: {:?}", name, next);
        let passthrough = match self.level {
            Level::Segment => !is_segment(name),
            Level::Element => !is_container_element(name),
            Level::Subelement => true,
        };
        if passthrough {
            return visitor.visit_seq(SizedDeserializer {
                de: &mut self,
                remaining: fields.len(),
                total: fields.len(),
                name,
            });
        }
        let Some(buf) = self.buffer else {
            return Err(X12DeserializerError::EarlyEndOfDocument);
        };
        let (mut sub_buf, mut rest) = split_string(buf, self.delimiter as char);
        if self.level == Level::Segment {
            if let Some(r) = rest.as_mut() {
                let sep = self.formatter.visual_separator.as_bytes();
                if r.as_bytes().starts_with(sep) {
                    *r = &r[sep.len()..];
                }
                if r.is_empty() {
                    rest = None;
                }
            }

            if name != "segment" {
                let (segment_name, segment_rest) = split_string(sub_buf, self.formatter.element_delimiter as char);
                if segment_name != name {
                    return Err(X12DeserializerError::UnexpectedSegment {
                        expected: name,
                        got: segment_name.to_string(),
                    });
                }
                let Some(segment_rest) = segment_rest else {
                    return Err(X12DeserializerError::EarlyEndOfDocument);
                };
                sub_buf = segment_rest;
            }
        }
        self.buffer = rest;
        visitor.visit_seq(SizedDeserializer {
            de: &mut X12Deserializer::new(sub_buf, self.formatter, self.level.lower()),
            remaining: fields.len(),
            total: fields.len(),
            name,
        })
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn is_human_readable(&self) -> bool { false }
}

impl<'de> SeqAccess<'de> for X12Deserializer<'de> {
    type Error = X12DeserializerError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: DeserializeSeed<'de> {
        // eprintln!("BufferDeserializer({})::next_element_seed: {:?}", self.level, self.buffer);
        if self.buffer.is_none() {
            return Ok(None);
        }
        match seed.deserialize(&mut *self).map(Some) {
            Ok(value) => Ok(value),
            Err(X12DeserializerError::EarlyEndOfDocument) => Ok(None),
            Err(X12DeserializerError::UnexpectedSegment { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

struct SizedDeserializer<'a, 'de> {
    de: &'a mut X12Deserializer<'de>,
    remaining: usize,
    total: usize,
    name: &'static str,
}

impl<'a, 'de> SeqAccess<'de> for SizedDeserializer<'a, 'de> {
    type Error = X12DeserializerError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: DeserializeSeed<'de> {
        // eprintln!("SizedBufferDeserializer({})::next_element_seed: {}", self.name, self.total - self.remaining);
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

pub fn is_segment(s: &str) -> bool {
    if s == "Segment" {
        return true;
    }
    !s.chars().any(|c| c.is_lowercase())
}

pub fn is_container_element(s: &str) -> bool {
    if s.starts_with('C') && s.chars().skip(1).all(|c| c.is_digit(10)) {
        true
    } else if s.contains('-') {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::Element;

    use super::*;

    #[test]
    fn test_is_container_element() {
        assert!(is_container_element("C123"));
        assert!(!is_container_element("ISA"));
        assert!(!is_container_element("Document"));
    }

    #[test]
    fn test_generic_segment() {
        let input = "01*02*03";
        let mut de = X12Deserializer::new(input, X12Formatter::default(), Level::Element);
        let value = Vec::<String>::deserialize(&mut de).unwrap();
        assert_eq!(value, vec!["01".to_string(), "02".to_string(), "03".to_string()]);
    }

    #[test]
    fn test_generic_segment_with_container_element() {
        let input = "01*02a:02b:02c*03";
        let mut de = X12Deserializer::new(input, X12Formatter::default(), Level::Element);
        let value = Vec::<Element>::deserialize(&mut de).unwrap();
        assert_eq!(value, vec![
            Element::Value("01".to_string()),
            Element::Container(vec!["02a".to_string(), "02b".to_string(), "02c".to_string()]),
            Element::Value("03".to_string()),
        ]);
    }

    #[test]
    fn test_struct_segment() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Segment {
            field1: String,
            field2: String,
            field3: String,
        }
        let input = "01*02*03";
        let mut de = X12Deserializer::new(input, X12Formatter::default(), Level::Element);
        let value = Segment::deserialize(&mut de).unwrap();
        assert_eq!(value, Segment {
            field1: "01".to_string(),
            field2: "02".to_string(),
            field3: "03".to_string(),
        })
    }
    //
    #[test]
    fn test_single_field_struct_segment_with_container_element() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Segment {
            field2: ContainerElement,
        }
        #[derive(Deserialize, PartialEq, Debug)]
        #[serde(tag = "code", rename = "C123")]
        struct ContainerElement {
            field1: String,
            field2: String,
            field3: String,
        }
        let input = "02a:02b:02c";
        let mut de = X12Deserializer::new(input, X12Formatter::default(), Level::Element);
        let value = Segment::deserialize(&mut de).unwrap();
        assert_eq!(value, Segment {
            field2: ContainerElement {
                field1: "02a".to_string(),
                field2: "02b".to_string(),
                field3: "02c".to_string(),
            },
        })
    }

    #[test]
    fn test_struct_segment_with_container_element() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Segment {
            field1: String,
            field2: ContainerElement,
            field3: String,
        }
        #[derive(Deserialize, PartialEq, Debug)]
        #[serde(tag = "code", rename = "C123")]
        struct ContainerElement {
            field1: String,
            field2: String,
            field3: String,
        }
        let input = "01*02a:02b:02c*03";
        let mut de = X12Deserializer::new(input, X12Formatter::default(), Level::Element);
        let value = Segment::deserialize(&mut de).unwrap();
        assert_eq!(value, Segment {
            field1: "01".to_string(),
            field2: ContainerElement {
                field1: "02a".to_string(),
                field2: "02b".to_string(),
                field3: "02c".to_string(),
            },
            field3: "03".to_string(),
        })
    }

    #[test]
    fn test_deserialize_str_becomes_none() {
        let input = "*one";
        #[derive(Deserialize, PartialEq, Debug)]
        struct Segment {
            field1: Option<String>,
            field2: String,
        }
        let mut de = X12Deserializer::new(input, X12Formatter::default(), Level::Element);
        let value = Segment::deserialize(&mut de).unwrap();
        assert_eq!(value, Segment {
            field1: None,
            field2: "one".to_string(),
        })
    }
}