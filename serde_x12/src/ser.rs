use std::fmt::{Display, Formatter};

use serde::{ser, Serialize};

use colored::Colorize;

use crate::de::{is_container_element, is_segment};
use crate::formatter::{Level, VisualSeparator};
use crate::X12Formatter;

pub fn to_string<T>(value: &T) -> Result<String, X12SerializerError>
    where
        T: ser::Serialize,
{
    let mut inner = X12Serializer::default();
    let ser = SerState {
        delimiter: inner.formatter.segment_delimiter,
        ser: &mut inner,
        level: Level::Segment,
    };
    value.serialize(ser)?;
    Ok(inner.to_string())
}

#[derive(Debug)]
pub enum X12SerializerError {
    GenericError(String),
}

impl std::error::Error for X12SerializerError {}

impl Display for X12SerializerError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ser::Error for X12SerializerError {
    fn custom<T: Display>(msg: T) -> Self {
        X12SerializerError::GenericError(msg.to_string())
    }
}

pub struct X12Serializer {
    output: Vec<u8>,
    formatter: X12Formatter,
}

pub struct SerState<'ser> {
    pub(crate) ser: &'ser mut X12Serializer,
    pub(crate) delimiter: u8,
    pub(crate) level: Level,
}

pub struct SerStruct<'ser> {
    ser: &'ser mut X12Serializer,
    delimiter: u8,
    level: Level,
    skip: bool,
    name: &'static str,
}

impl<'ser> SerStruct<'ser> {
    pub fn new(ser: &'ser mut X12Serializer, level: Level) -> SerStruct<'ser> {
        Self {
            delimiter: ser.formatter.delimiter(level),
            ser,
            level,
            skip: false,
            name: "",
        }
    }
}

impl<'ser> SerState<'ser> {
    pub fn new(ser: &'ser mut X12Serializer, level: Level) -> SerState<'ser> {
        Self {
            delimiter: ser.formatter.delimiter(level),
            ser,
            level,
        }
    }
}

impl Default for X12Serializer {
    fn default() -> Self {
        let formatter = X12Formatter::default();
        Self {
            output: Vec::new(),
            formatter,
        }
    }
}

impl X12Serializer {
    pub fn to_string(self) -> String {
        String::from_utf8(self.output).unwrap()
    }

    pub fn new(formatter: X12Formatter) -> Self {
        Self {
            output: Vec::new(),
            formatter,
        }
    }
}


impl<'ser> ser::Serializer for SerState<'ser> {
    type Ok = ();
    type Error = X12SerializerError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = SerStruct<'ser>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.output.extend_from_slice(v.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ser.output.extend_from_slice(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        // paired with logic after serialize in SerStruct
        // if self.level != Level::Segment {
        //     self.ser.output.pop();
        // }
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(mut self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        todo!()
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        // eprintln!("serialize_seq: {:?}", len);
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(mut self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        // eprintln!("{}: {} {}", "serialize struct:".yellow(), name, len);
        let mut s = SerStruct::new(self.ser, Level::Segment);
        s.name = name;
        if is_container_element(name) {
            s.level = Level::Subelement;
            s.delimiter = s.ser.formatter.delimiter(s.level);
            s.skip = true;
        } else if is_segment(name) {
            s.level = Level::Element;
            s.delimiter = s.ser.formatter.delimiter(s.level);
        }
        Ok(s)
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'ser> ser::SerializeMap for SerState<'ser> {
    type Ok = ();
    type Error = X12SerializerError;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
    {
        todo!()
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'ser> ser::SerializeStruct for SerStruct<'ser> {
    type Ok = ();
    type Error = X12SerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
    {
        // eprintln!("SerializeStruct({})::serialize_field: {}", self.name, key);
        if self.skip {
            self.skip = false;
            return Ok(());
        }
        let size = self.ser.output.len();
        let r = value.serialize(SerState::new(self.ser, self.level));
        if self.level != Level::Segment {
            self.ser.output.push(self.delimiter);
        }
        r
    }

    fn end(self) -> Result<(), Self::Error> {
        let count = self.ser.output.iter().rev().take_while(|&&c| c == self.delimiter).count();
        if count > 0 {
            self.ser.output.truncate(self.ser.output.len() - count + 1);
        }
        if self.level != Level::Segment {
            self.ser.output.pop();
        }
        if self.level == Level::Element {
            let f = self.ser.formatter;
            let o = &self.ser.output;
            let ends_segment = o.ends_with(f.visual_separator.as_bytes());
            if !ends_segment {
                self.ser.output.push(self.ser.formatter.segment_delimiter);
                self.ser.output.extend_from_slice(self.ser.formatter.visual_separator.as_bytes());
            }
        }
        // eprintln!("finished {} - {}.\n{}\n{}", self.name, self.level, "colored output:".green(), String::from_utf8(self.ser.output.clone()).unwrap());
        Ok(())
    }
}

impl<'ser> ser::SerializeStructVariant for SerState<'ser> {
    type Ok = ();
    type Error = X12SerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'ser> ser::SerializeTupleVariant for SerState<'ser> {
    type Ok = ();
    type Error = X12SerializerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'ser> ser::SerializeTupleStruct for SerState<'ser> {
    type Ok = ();
    type Error = X12SerializerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'ser> ser::SerializeSeq for SerState<'ser> {
    type Ok = ();
    type Error = X12SerializerError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
    {
        // eprintln!("{}", self.level);
        let l = self.level.lower();
        let r = value.serialize(SerState::new(self.ser, l));
        // pairs with logic in serialize_none
        if self.level != Level::Segment {
            self.ser.output.push(self.delimiter);
        }
        r
    }

    fn end(self) -> Result<(), Self::Error> {
        if self.level != Level::Segment {
            self.ser.output.pop();
        }
        Ok(())
    }
}

impl<'ser> ser::SerializeTuple for SerState<'ser> {
    type Ok = ();
    type Error = X12SerializerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<(), Self::Error> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::Element;

    use super::*;

    #[test]
    fn test_serialize_element_vec() {
        let value = vec![
            Element::Value("a".to_string()),
            Element::Value("b".to_string()),
            Element::Value("c".to_string()),
        ];
        let mut inner = X12Serializer::default();
        let ser = SerState::new(&mut inner, Level::Element);
        // let mut ser = X12Serializer::new(X12Formatter::default(), Level::Element);
        value.serialize(ser).unwrap();
        assert_eq!(inner.to_string(), "a*b*c");
    }

    #[test]
    fn test_serialize_element_vec_with_container() {
        let value = vec![
            Element::Value("01".to_string()),
            Element::Container(vec!["02a".to_string(), "02b".to_string(), "02c".to_string()]),
            Element::Value("03".to_string()),
        ];
        let mut inner = X12Serializer::default();
        let ser = SerState::new(&mut inner, Level::Element);
        value.serialize(ser).unwrap();
        assert_eq!(inner.to_string(), "01*02a:02b:02c*03");
    }

    #[test]
    fn test_struct_segment() {
        #[derive(Serialize, PartialEq, Debug)]
        struct Segment {
            field1: String,
            field2: String,
            field3: String,
        }
        let value = Segment {
            field1: "01".to_string(),
            field2: "02".to_string(),
            field3: "03".to_string(),
        };
        let mut inner = X12Serializer::default();
        let ser = SerState::new(&mut inner, Level::Element);
        value.serialize(ser).unwrap();
        assert_eq!(inner.to_string(), "01*02*03~\n");
    }

    #[test]
    fn test_single_field_struct_segment_with_container_element() {
        #[derive(Serialize, PartialEq, Debug)]
        struct Segment {
            field2: ContainerElement,
        }
        #[derive(Serialize, PartialEq, Debug)]
        #[serde(tag = "code", rename = "C123")]
        struct ContainerElement {
            field1: String,
            field2: String,
            field3: String,
        }
        let value = Segment {
            field2: ContainerElement {
                field1: "02a".to_string(),
                field2: "02b".to_string(),
                field3: "02c".to_string(),
            },
        };
        let mut inner = X12Serializer::default();
        let ser = SerState::new(&mut inner, Level::Element);
        value.serialize(ser).unwrap();
        assert_eq!(inner.to_string(), "02a:02b:02c~\n");
    }

    #[test]
    fn test_struct_segment_with_container() {
        #[derive(Serialize, PartialEq, Debug)]
        struct Segment {
            field1: String,
            field2: ContainerElement,
            field3: String,
        }
        #[derive(Serialize, PartialEq, Debug)]
        #[serde(tag = "code", rename = "C123")]
        struct ContainerElement {
            field1: String,
            field2: String,
            field3: String,
        }
        let value = Segment {
            field1: "01".to_string(),
            field2: ContainerElement {
                field1: "02a".to_string(),
                field2: "02b".to_string(),
                field3: "02c".to_string(),
            },
            field3: "03".to_string(),
        };
        let mut inner = X12Serializer::default();
        let ser = SerState::new(&mut inner, Level::Element);
        value.serialize(ser).unwrap();
        assert_eq!(inner.to_string(), "01*02a:02b:02c*03~\n");
    }
}