use std::fmt::Debug;
use std::str::FromStr;
use serde::{ser, Serialize};
use crate::ser::{SerState, X12Serializer, X12SerializerError};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Level {
    Segment,
    Element,
    Subelement,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Segment => write!(f, "Segment"),
            Level::Element => write!(f, "Element"),
            Level::Subelement => write!(f, "Subelement"),
        }
    }
}

impl Level {
    pub fn lower(&self) -> Self {
        match self {
            Level::Segment => Level::Element,
            _ => Level::Subelement,
        }
    }
}

pub fn split_string(s: &str, d: char) -> (&str, Option<&str>) {
    let Some(idx) = s.find(d) else {
        return (s, None);
    };
    let (tok, rest) = s.split_at(idx);
    let rest = &rest[1..];
    (tok, Some(rest))
}

#[derive(Copy, Clone)]
pub enum VisualSeparator {
    None,
    /// \n
    Newline,
    /// \r\n
    CarriageNewline,
}

impl VisualSeparator {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            VisualSeparator::None => b"",
            VisualSeparator::Newline => b"\n",
            VisualSeparator::CarriageNewline => b"\r\n",
        }
    }
}

impl Debug for VisualSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualSeparator::None => write!(f, r#""""#),
            VisualSeparator::Newline => write!(f, r#""\n""#),
            VisualSeparator::CarriageNewline => write!(f, r#""\r\n""#),
        }
    }
}

#[derive(Copy, Clone)]
pub struct X12Formatter {
    pub element_delimiter: u8,
    pub sub_element_delimiter: u8,
    pub segment_delimiter: u8,
    pub visual_separator: VisualSeparator,
}

impl X12Formatter {
    pub fn delimiter(&self, level: Level) -> u8 {
        match level {
            Level::Segment => self.segment_delimiter,
            Level::Element => self.element_delimiter,
            Level::Subelement => self.sub_element_delimiter,
        }
    }

    pub fn segment_terminator(&self) -> Vec<u8> {
        match self.visual_separator {
            VisualSeparator::None => Vec::from(&[self.segment_delimiter]),
            VisualSeparator::Newline => Vec::from(&[self.segment_delimiter, b'\n']),
            VisualSeparator::CarriageNewline => Vec::from(&[self.segment_delimiter, b'\r', b'\n']),
        }
    }

    pub fn to_string<T>(&self, value: &T) -> Result<String, X12SerializerError>
        where
            T: ser::Serialize,
    {
        let mut inner = X12Serializer::new(*self);
        let ser = SerState {
            delimiter: self.segment_delimiter,
            ser: &mut inner,
            level: Level::Segment,
        };
        value.serialize(ser)?;
        Ok(inner.to_string())
    }
}

impl Debug for X12Formatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("X12Formatter")
            .field("element_delimiter", &(self.element_delimiter as char))
            .field("sub_element_delimiter", &(self.sub_element_delimiter as char))
            .field("segment_delimiter", &(self.segment_delimiter as char))
            .field("visual_separator", &self.visual_separator)
            .finish()
    }
}

impl Default for X12Formatter {
    fn default() -> Self {
        X12Formatter {
            element_delimiter: b'*',
            sub_element_delimiter: b':',
            segment_delimiter: b'~',
            visual_separator: VisualSeparator::Newline,
        }
    }
}

fn ensure_isa(s: &str) -> Option<()> {
    if s.len() < 106 {
        return None
    }
    if &s[0..3] != "ISA" {
        return None
    }
    Some(())
}

pub fn detect_format(s: &str) -> Option<X12Formatter> {
    ensure_isa(s)?;
    let s = s.as_bytes();
    let element_delimiter = s[103];
    let sub_element_delimiter = s[104];
    let segment_delimiter = s[105];
    let visual_separator = match &s[106..108] {
        b"\r\n" => VisualSeparator::CarriageNewline,
        b"\nG" => VisualSeparator::Newline,
        b"GS" => VisualSeparator::None,
        _ => return None,
    };
    Some(X12Formatter {
        element_delimiter,
        sub_element_delimiter,
        segment_delimiter,
        visual_separator,
    })
}

/// Codes from https://www.stedi.com/edi/x12/element/I11
/// Return is ALWAYS 5 characters long
///
/// 00501 appears to be used for a lot of healthcare (837)
/// 00801 is the most recent version, as far as i can tell
pub fn detect_version(s: &str) -> Option<&str> {
    ensure_isa(s)?;
    Some(&s[84..89])
}