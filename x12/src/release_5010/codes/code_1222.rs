use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1222

See docs at <https://www.stedi.com/edi/x12-005010/element/1222>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderSpecialtyCode {
    ///DEN - Denturist
    Denturist,
    ///DGP - Dental General Practice
    DentalGeneralPractice,
    ///END - Endodontics
    Endodontics,
    ///IHG - Independent Hygienist
    IndependentHygienist,
    ///OPY - Oral Pathology
    OralPathology,
    ///ORT - Orthodontics
    Orthodontics,
    ///OSY - Oral Surgery
    OralSurgery,
    ///PDT - Periodontics
    Periodontics,
    ///PED - Pediatric Dentistry
    PediatricDentistry,
    ///PHD - Public Health Dentistry
    PublicHealthDentistry,
    ///PST - Prosthodontics
    Prosthodontics,
}
impl ProviderSpecialtyCode {
    pub fn code(&self) -> &str {
        {
            use ProviderSpecialtyCode::*;
            match self {
                Denturist => "DEN",
                DentalGeneralPractice => "DGP",
                Endodontics => "END",
                IndependentHygienist => "IHG",
                OralPathology => "OPY",
                Orthodontics => "ORT",
                OralSurgery => "OSY",
                Periodontics => "PDT",
                PediatricDentistry => "PED",
                PublicHealthDentistry => "PHD",
                Prosthodontics => "PST",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ProviderSpecialtyCode> {
        use ProviderSpecialtyCode::*;
        match code {
            b"DEN" => Some(Denturist),
            b"DGP" => Some(DentalGeneralPractice),
            b"END" => Some(Endodontics),
            b"IHG" => Some(IndependentHygienist),
            b"OPY" => Some(OralPathology),
            b"ORT" => Some(Orthodontics),
            b"OSY" => Some(OralSurgery),
            b"PDT" => Some(Periodontics),
            b"PED" => Some(PediatricDentistry),
            b"PHD" => Some(PublicHealthDentistry),
            b"PST" => Some(Prosthodontics),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ProviderSpecialtyCode::*;
        match self {
            Denturist => "Denturist",
            DentalGeneralPractice => "Dental General Practice",
            Endodontics => "Endodontics",
            IndependentHygienist => "Independent Hygienist",
            OralPathology => "Oral Pathology",
            Orthodontics => "Orthodontics",
            OralSurgery => "Oral Surgery",
            Periodontics => "Periodontics",
            PediatricDentistry => "Pediatric Dentistry",
            PublicHealthDentistry => "Public Health Dentistry",
            Prosthodontics => "Prosthodontics",
        }
    }
    fn from_description(description: &str) -> Option<ProviderSpecialtyCode> {
        {
            use ProviderSpecialtyCode::*;
            match description {
                "Denturist" => Some(Denturist),
                "Dental General Practice" => Some(DentalGeneralPractice),
                "Endodontics" => Some(Endodontics),
                "Independent Hygienist" => Some(IndependentHygienist),
                "Oral Pathology" => Some(OralPathology),
                "Orthodontics" => Some(Orthodontics),
                "Oral Surgery" => Some(OralSurgery),
                "Periodontics" => Some(Periodontics),
                "Pediatric Dentistry" => Some(PediatricDentistry),
                "Public Health Dentistry" => Some(PublicHealthDentistry),
                "Prosthodontics" => Some(Prosthodontics),
                _ => None,
            }
        }
    }
}
impl Serialize for ProviderSpecialtyCode {
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
    type Value = ProviderSpecialtyCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Provider Specialty Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderSpecialtyCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Provider Specialty Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderSpecialtyCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Provider Specialty Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ProviderSpecialtyCode {
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