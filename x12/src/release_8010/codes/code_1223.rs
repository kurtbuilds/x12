use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1223

See docs at <https://www.stedi.com/edi/x12/element/1223>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderOrganizationCode {
    ///001 - Single Practice
    SinglePractice,
    ///002 - Partnership
    Partnership,
    ///003 - Professional Association (PA)
    Code003,
    ///004 - Clinic
    Clinic,
    ///005 - Single Entity Facility or Hospital
    SingleEntityFacilityOrHospital,
    ///006 - Distinct Part Facility or Hospital
    DistinctPartFacilityOrHospital,
    ///007 - Individual
    Individual,
    ///008 - Corporation
    Corporation,
}
impl ProviderOrganizationCode {
    pub fn code(&self) -> &str {
        {
            use ProviderOrganizationCode::*;
            match self {
                SinglePractice => "001",
                Partnership => "002",
                Code003 => "003",
                Clinic => "004",
                SingleEntityFacilityOrHospital => "005",
                DistinctPartFacilityOrHospital => "006",
                Individual => "007",
                Corporation => "008",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ProviderOrganizationCode> {
        use ProviderOrganizationCode::*;
        match code {
            b"001" => Some(SinglePractice),
            b"002" => Some(Partnership),
            b"003" => Some(Code003),
            b"004" => Some(Clinic),
            b"005" => Some(SingleEntityFacilityOrHospital),
            b"006" => Some(DistinctPartFacilityOrHospital),
            b"007" => Some(Individual),
            b"008" => Some(Corporation),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ProviderOrganizationCode::*;
        match self {
            SinglePractice => "Single Practice",
            Partnership => "Partnership",
            Code003 => "Professional Association (PA)",
            Clinic => "Clinic",
            SingleEntityFacilityOrHospital => "Single Entity Facility or Hospital",
            DistinctPartFacilityOrHospital => "Distinct Part Facility or Hospital",
            Individual => "Individual",
            Corporation => "Corporation",
        }
    }
    fn from_description(description: &str) -> Option<ProviderOrganizationCode> {
        {
            use ProviderOrganizationCode::*;
            match description {
                "Single Practice" => Some(SinglePractice),
                "Partnership" => Some(Partnership),
                "Professional Association (PA)" => Some(Code003),
                "Clinic" => Some(Clinic),
                "Single Entity Facility or Hospital" => {
                    Some(SingleEntityFacilityOrHospital)
                }
                "Distinct Part Facility or Hospital" => {
                    Some(DistinctPartFacilityOrHospital)
                }
                "Individual" => Some(Individual),
                "Corporation" => Some(Corporation),
                _ => None,
            }
        }
    }
}
impl Serialize for ProviderOrganizationCode {
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
    type Value = ProviderOrganizationCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Provider Organization Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderOrganizationCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Provider Organization Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderOrganizationCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Provider Organization Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ProviderOrganizationCode {
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