use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**736

See docs at <https://www.stedi.com/edi/x12/element/736>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HierarchicalChildCode {
    ///0 - No Subordinate HL Segment in this Hierarchical Structure.
    NoSubordinateHlSegmentInThisHierarchicalStructure,
    ///1 - Additional Subordinate HL Data Segment in this Hierarchical Structure.
    AdditionalSubordinateHlDataSegmentInThisHierarchicalStructure,
}
impl HierarchicalChildCode {
    pub fn code(&self) -> &str {
        {
            use HierarchicalChildCode::*;
            match self {
                NoSubordinateHlSegmentInThisHierarchicalStructure => "0",
                AdditionalSubordinateHlDataSegmentInThisHierarchicalStructure => "1",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<HierarchicalChildCode> {
        use HierarchicalChildCode::*;
        match code {
            b"0" => Some(NoSubordinateHlSegmentInThisHierarchicalStructure),
            b"1" => Some(AdditionalSubordinateHlDataSegmentInThisHierarchicalStructure),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use HierarchicalChildCode::*;
        match self {
            NoSubordinateHlSegmentInThisHierarchicalStructure => {
                "No Subordinate HL Segment in this Hierarchical Structure."
            }
            AdditionalSubordinateHlDataSegmentInThisHierarchicalStructure => {
                "Additional Subordinate HL Data Segment in this Hierarchical Structure."
            }
        }
    }
    fn from_description(description: &str) -> Option<HierarchicalChildCode> {
        {
            use HierarchicalChildCode::*;
            match description {
                "No Subordinate HL Segment in this Hierarchical Structure." => {
                    Some(NoSubordinateHlSegmentInThisHierarchicalStructure)
                }
                "Additional Subordinate HL Data Segment in this Hierarchical Structure." => {
                    Some(AdditionalSubordinateHlDataSegmentInThisHierarchicalStructure)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for HierarchicalChildCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = if serializer.is_human_readable() {
            self.description()
        } else {
            self.code()
        };
        serializer.serialize_str(value)
    }
}
struct Visitor;
impl<'de> de::Visitor<'de> for Visitor {
    type Value = HierarchicalChildCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Hierarchical Child Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        HierarchicalChildCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Hierarchical Child Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        HierarchicalChildCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Hierarchical Child Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for HierarchicalChildCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(Visitor)
        } else {
            deserializer.deserialize_bytes(Visitor)
        }
    }
}