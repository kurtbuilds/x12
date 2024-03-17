use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1356

See docs at <https://www.stedi.com/edi/x12/element/1356>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrescriptionOriginCode {
    ///0 - Not Specified
    NotSpecified,
    ///1 - Written Prescription
    WrittenPrescription,
    ///2 - Telephone Prescription
    TelephonePrescription,
    ///3 - Telephone Emergency Prescription
    TelephoneEmergencyPrescription,
}
impl PrescriptionOriginCode {
    pub fn code(&self) -> &str {
        {
            use PrescriptionOriginCode::*;
            match self {
                NotSpecified => "0",
                WrittenPrescription => "1",
                TelephonePrescription => "2",
                TelephoneEmergencyPrescription => "3",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PrescriptionOriginCode> {
        use PrescriptionOriginCode::*;
        match code {
            b"0" => Some(NotSpecified),
            b"1" => Some(WrittenPrescription),
            b"2" => Some(TelephonePrescription),
            b"3" => Some(TelephoneEmergencyPrescription),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PrescriptionOriginCode::*;
        match self {
            NotSpecified => "Not Specified",
            WrittenPrescription => "Written Prescription",
            TelephonePrescription => "Telephone Prescription",
            TelephoneEmergencyPrescription => "Telephone Emergency Prescription",
        }
    }
    fn from_description(description: &str) -> Option<PrescriptionOriginCode> {
        {
            use PrescriptionOriginCode::*;
            match description {
                "Not Specified" => Some(NotSpecified),
                "Written Prescription" => Some(WrittenPrescription),
                "Telephone Prescription" => Some(TelephonePrescription),
                "Telephone Emergency Prescription" => {
                    Some(TelephoneEmergencyPrescription)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for PrescriptionOriginCode {
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
    type Value = PrescriptionOriginCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Prescription Origin Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PrescriptionOriginCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Prescription Origin Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PrescriptionOriginCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Prescription Origin Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PrescriptionOriginCode {
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