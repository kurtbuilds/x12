use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1403

See docs at <https://www.stedi.com/edi/x12/element/1403>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImplantTypeCode {
    ///1 - Pulse Generator
    PulseGenerator,
    ///2 - Atrial Lead
    AtrialLead,
    ///3 - Ventricular Lead
    VentricularLead,
}
impl ImplantTypeCode {
    pub fn code(&self) -> &str {
        {
            use ImplantTypeCode::*;
            match self {
                PulseGenerator => "1",
                AtrialLead => "2",
                VentricularLead => "3",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ImplantTypeCode> {
        use ImplantTypeCode::*;
        match code {
            b"1" => Some(PulseGenerator),
            b"2" => Some(AtrialLead),
            b"3" => Some(VentricularLead),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ImplantTypeCode::*;
        match self {
            PulseGenerator => "Pulse Generator",
            AtrialLead => "Atrial Lead",
            VentricularLead => "Ventricular Lead",
        }
    }
    fn from_description(description: &str) -> Option<ImplantTypeCode> {
        {
            use ImplantTypeCode::*;
            match description {
                "Pulse Generator" => Some(PulseGenerator),
                "Atrial Lead" => Some(AtrialLead),
                "Ventricular Lead" => Some(VentricularLead),
                _ => None,
            }
        }
    }
}
impl Serialize for ImplantTypeCode {
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
    type Value = ImplantTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Implant Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImplantTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Implant Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImplantTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Implant Type Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ImplantTypeCode {
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