use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1367

See docs at <https://www.stedi.com/edi/x12/element/1367>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SubluxationLevelCode {
    ///C1 - Cervical 1
    Cervical1,
    ///C2 - Cervical 2
    Cervical2,
    ///C3 - Cervical 3
    Cervical3,
    ///C4 - Cervical 4
    Cervical4,
    ///C5 - Cervical 5
    Cervical5,
    ///C6 - Cervical 6
    Cervical6,
    ///C7 - Cervical 7
    Cervical7,
    ///CO - Coccyx
    Coccyx,
    ///IL - Ilium
    Ilium,
    ///L1 - Lumbar 1
    Lumbar1,
    ///L2 - Lumbar 2
    Lumbar2,
    ///L3 - Lumbar 3
    Lumbar3,
    ///L4 - Lumbar 4
    Lumbar4,
    ///L5 - Lumbar 5
    Lumbar5,
    ///OC - Occiput
    Occiput,
    ///SA - Sacrum
    Sacrum,
    ///T1 - Thoracic 1
    Thoracic1,
    ///T2 - Thoracic 2
    Thoracic2,
    ///T3 - Thoracic 3
    Thoracic3,
    ///T4 - Thoracic 4
    Thoracic4,
    ///T5 - Thoracic 5
    Thoracic5,
    ///T6 - Thoracic 6
    Thoracic6,
    ///T7 - Thoracic 7
    Thoracic7,
    ///T8 - Thoracic 8
    Thoracic8,
    ///T9 - Thoracic 9
    Thoracic9,
    ///T10 - Thoracic 10
    Thoracic10,
    ///T11 - Thoracic 11
    Thoracic11,
    ///T12 - Thoracic 12
    Thoracic12,
}
impl SubluxationLevelCode {
    pub fn code(&self) -> &str {
        {
            use SubluxationLevelCode::*;
            match self {
                Cervical1 => "C1",
                Cervical2 => "C2",
                Cervical3 => "C3",
                Cervical4 => "C4",
                Cervical5 => "C5",
                Cervical6 => "C6",
                Cervical7 => "C7",
                Coccyx => "CO",
                Ilium => "IL",
                Lumbar1 => "L1",
                Lumbar2 => "L2",
                Lumbar3 => "L3",
                Lumbar4 => "L4",
                Lumbar5 => "L5",
                Occiput => "OC",
                Sacrum => "SA",
                Thoracic1 => "T1",
                Thoracic2 => "T2",
                Thoracic3 => "T3",
                Thoracic4 => "T4",
                Thoracic5 => "T5",
                Thoracic6 => "T6",
                Thoracic7 => "T7",
                Thoracic8 => "T8",
                Thoracic9 => "T9",
                Thoracic10 => "T10",
                Thoracic11 => "T11",
                Thoracic12 => "T12",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<SubluxationLevelCode> {
        use SubluxationLevelCode::*;
        match code {
            b"C1" => Some(Cervical1),
            b"C2" => Some(Cervical2),
            b"C3" => Some(Cervical3),
            b"C4" => Some(Cervical4),
            b"C5" => Some(Cervical5),
            b"C6" => Some(Cervical6),
            b"C7" => Some(Cervical7),
            b"CO" => Some(Coccyx),
            b"IL" => Some(Ilium),
            b"L1" => Some(Lumbar1),
            b"L2" => Some(Lumbar2),
            b"L3" => Some(Lumbar3),
            b"L4" => Some(Lumbar4),
            b"L5" => Some(Lumbar5),
            b"OC" => Some(Occiput),
            b"SA" => Some(Sacrum),
            b"T1" => Some(Thoracic1),
            b"T2" => Some(Thoracic2),
            b"T3" => Some(Thoracic3),
            b"T4" => Some(Thoracic4),
            b"T5" => Some(Thoracic5),
            b"T6" => Some(Thoracic6),
            b"T7" => Some(Thoracic7),
            b"T8" => Some(Thoracic8),
            b"T9" => Some(Thoracic9),
            b"T10" => Some(Thoracic10),
            b"T11" => Some(Thoracic11),
            b"T12" => Some(Thoracic12),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use SubluxationLevelCode::*;
        match self {
            Cervical1 => "Cervical 1",
            Cervical2 => "Cervical 2",
            Cervical3 => "Cervical 3",
            Cervical4 => "Cervical 4",
            Cervical5 => "Cervical 5",
            Cervical6 => "Cervical 6",
            Cervical7 => "Cervical 7",
            Coccyx => "Coccyx",
            Ilium => "Ilium",
            Lumbar1 => "Lumbar 1",
            Lumbar2 => "Lumbar 2",
            Lumbar3 => "Lumbar 3",
            Lumbar4 => "Lumbar 4",
            Lumbar5 => "Lumbar 5",
            Occiput => "Occiput",
            Sacrum => "Sacrum",
            Thoracic1 => "Thoracic 1",
            Thoracic2 => "Thoracic 2",
            Thoracic3 => "Thoracic 3",
            Thoracic4 => "Thoracic 4",
            Thoracic5 => "Thoracic 5",
            Thoracic6 => "Thoracic 6",
            Thoracic7 => "Thoracic 7",
            Thoracic8 => "Thoracic 8",
            Thoracic9 => "Thoracic 9",
            Thoracic10 => "Thoracic 10",
            Thoracic11 => "Thoracic 11",
            Thoracic12 => "Thoracic 12",
        }
    }
    fn from_description(description: &str) -> Option<SubluxationLevelCode> {
        {
            use SubluxationLevelCode::*;
            match description {
                "Cervical 1" => Some(Cervical1),
                "Cervical 2" => Some(Cervical2),
                "Cervical 3" => Some(Cervical3),
                "Cervical 4" => Some(Cervical4),
                "Cervical 5" => Some(Cervical5),
                "Cervical 6" => Some(Cervical6),
                "Cervical 7" => Some(Cervical7),
                "Coccyx" => Some(Coccyx),
                "Ilium" => Some(Ilium),
                "Lumbar 1" => Some(Lumbar1),
                "Lumbar 2" => Some(Lumbar2),
                "Lumbar 3" => Some(Lumbar3),
                "Lumbar 4" => Some(Lumbar4),
                "Lumbar 5" => Some(Lumbar5),
                "Occiput" => Some(Occiput),
                "Sacrum" => Some(Sacrum),
                "Thoracic 1" => Some(Thoracic1),
                "Thoracic 2" => Some(Thoracic2),
                "Thoracic 3" => Some(Thoracic3),
                "Thoracic 4" => Some(Thoracic4),
                "Thoracic 5" => Some(Thoracic5),
                "Thoracic 6" => Some(Thoracic6),
                "Thoracic 7" => Some(Thoracic7),
                "Thoracic 8" => Some(Thoracic8),
                "Thoracic 9" => Some(Thoracic9),
                "Thoracic 10" => Some(Thoracic10),
                "Thoracic 11" => Some(Thoracic11),
                "Thoracic 12" => Some(Thoracic12),
                _ => None,
            }
        }
    }
}
impl Serialize for SubluxationLevelCode {
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
    type Value = SubluxationLevelCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Subluxation Level Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SubluxationLevelCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Subluxation Level Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SubluxationLevelCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Subluxation Level Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for SubluxationLevelCode {
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