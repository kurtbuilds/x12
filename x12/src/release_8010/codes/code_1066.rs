use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1066

See docs at <https://www.stedi.com/edi/x12/element/1066>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CitizenshipStatusCode {
    ///1 - U.S. Citizen
    USCitizen,
    ///2 - Non-Resident Alien
    NonResidentAlien,
    ///3 - Resident Alien
    ResidentAlien,
    ///4 - Illegal Alien
    IllegalAlien,
    ///5 - Alien
    Alien,
    ///6 - U.S. Citizen - Non-Resident
    USCitizenNonResident,
    ///7 - U.S. Citizen - Resident
    USCitizenResident,
    ///8 - Citizen
    Citizen,
    ///9 - Non-citizen with Student Authorization
    NonCitizenWithStudentAuthorization,
    ///A - Non-permanent Resident Alien
    NonPermanentResidentAlien,
    ///B - Permanent Visa
    PermanentVisa,
    ///C - Temporary Visa
    TemporaryVisa,
    ///D - Work Permit
    WorkPermit,
    ///E - Nordic Citizen
    NordicCitizen,
    ///F - Non-Nordic Citizen
    NonNordicCitizen,
    ///G - Naturalized Citizen
    NaturalizedCitizen,
    ///H - Eligible Non-citizen
    EligibleNonCitizen,
    ///I - Ineligible Non-citizen
    IneligibleNonCitizen,
}
impl CitizenshipStatusCode {
    pub fn code(&self) -> &str {
        {
            use CitizenshipStatusCode::*;
            match self {
                USCitizen => "1",
                NonResidentAlien => "2",
                ResidentAlien => "3",
                IllegalAlien => "4",
                Alien => "5",
                USCitizenNonResident => "6",
                USCitizenResident => "7",
                Citizen => "8",
                NonCitizenWithStudentAuthorization => "9",
                NonPermanentResidentAlien => "A",
                PermanentVisa => "B",
                TemporaryVisa => "C",
                WorkPermit => "D",
                NordicCitizen => "E",
                NonNordicCitizen => "F",
                NaturalizedCitizen => "G",
                EligibleNonCitizen => "H",
                IneligibleNonCitizen => "I",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CitizenshipStatusCode> {
        use CitizenshipStatusCode::*;
        match code {
            b"1" => Some(USCitizen),
            b"2" => Some(NonResidentAlien),
            b"3" => Some(ResidentAlien),
            b"4" => Some(IllegalAlien),
            b"5" => Some(Alien),
            b"6" => Some(USCitizenNonResident),
            b"7" => Some(USCitizenResident),
            b"8" => Some(Citizen),
            b"9" => Some(NonCitizenWithStudentAuthorization),
            b"A" => Some(NonPermanentResidentAlien),
            b"B" => Some(PermanentVisa),
            b"C" => Some(TemporaryVisa),
            b"D" => Some(WorkPermit),
            b"E" => Some(NordicCitizen),
            b"F" => Some(NonNordicCitizen),
            b"G" => Some(NaturalizedCitizen),
            b"H" => Some(EligibleNonCitizen),
            b"I" => Some(IneligibleNonCitizen),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CitizenshipStatusCode::*;
        match self {
            USCitizen => "U.S. Citizen",
            NonResidentAlien => "Non-Resident Alien",
            ResidentAlien => "Resident Alien",
            IllegalAlien => "Illegal Alien",
            Alien => "Alien",
            USCitizenNonResident => "U.S. Citizen - Non-Resident",
            USCitizenResident => "U.S. Citizen - Resident",
            Citizen => "Citizen",
            NonCitizenWithStudentAuthorization => {
                "Non-citizen with Student Authorization"
            }
            NonPermanentResidentAlien => "Non-permanent Resident Alien",
            PermanentVisa => "Permanent Visa",
            TemporaryVisa => "Temporary Visa",
            WorkPermit => "Work Permit",
            NordicCitizen => "Nordic Citizen",
            NonNordicCitizen => "Non-Nordic Citizen",
            NaturalizedCitizen => "Naturalized Citizen",
            EligibleNonCitizen => "Eligible Non-citizen",
            IneligibleNonCitizen => "Ineligible Non-citizen",
        }
    }
    fn from_description(description: &str) -> Option<CitizenshipStatusCode> {
        {
            use CitizenshipStatusCode::*;
            match description {
                "U.S. Citizen" => Some(USCitizen),
                "Non-Resident Alien" => Some(NonResidentAlien),
                "Resident Alien" => Some(ResidentAlien),
                "Illegal Alien" => Some(IllegalAlien),
                "Alien" => Some(Alien),
                "U.S. Citizen - Non-Resident" => Some(USCitizenNonResident),
                "U.S. Citizen - Resident" => Some(USCitizenResident),
                "Citizen" => Some(Citizen),
                "Non-citizen with Student Authorization" => {
                    Some(NonCitizenWithStudentAuthorization)
                }
                "Non-permanent Resident Alien" => Some(NonPermanentResidentAlien),
                "Permanent Visa" => Some(PermanentVisa),
                "Temporary Visa" => Some(TemporaryVisa),
                "Work Permit" => Some(WorkPermit),
                "Nordic Citizen" => Some(NordicCitizen),
                "Non-Nordic Citizen" => Some(NonNordicCitizen),
                "Naturalized Citizen" => Some(NaturalizedCitizen),
                "Eligible Non-citizen" => Some(EligibleNonCitizen),
                "Ineligible Non-citizen" => Some(IneligibleNonCitizen),
                _ => None,
            }
        }
    }
}
impl Serialize for CitizenshipStatusCode {
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
    type Value = CitizenshipStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Citizenship Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CitizenshipStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Citizenship Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CitizenshipStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Citizenship Status Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for CitizenshipStatusCode {
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