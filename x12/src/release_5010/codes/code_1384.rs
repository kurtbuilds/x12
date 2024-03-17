use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1384

See docs at <https://www.stedi.com/edi/x12-005010/element/1384>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatientLocationCode {
    ///A - Acute Care Facility
    AcuteCareFacility,
    ///B - Boarding Home
    BoardingHome,
    ///C - Hospice
    Hospice,
    ///D - Intermediate Care Facility
    IntermediateCareFacility,
    ///E - Long-term or Extended Care Facility
    LongTermOrExtendedCareFacility,
    ///F - Not Specified
    NotSpecified,
    ///G - Nursing Home
    NursingHome,
    ///H - Sub-acute Care Facility
    SubAcuteCareFacility,
    ///L - Other Location
    OtherLocation,
    ///M - Rehabilitation Facility
    RehabilitationFacility,
    ///O - Outpatient Facility
    OutpatientFacility,
    ///P - Private Home
    PrivateHome,
    ///R - Residential Treatment Facility
    ResidentialTreatmentFacility,
    ///S - Skilled Nursing Home
    SkilledNursingHome,
    ///T - Rest Home
    RestHome,
}
impl PatientLocationCode {
    pub fn code(&self) -> &str {
        {
            use PatientLocationCode::*;
            match self {
                AcuteCareFacility => "A",
                BoardingHome => "B",
                Hospice => "C",
                IntermediateCareFacility => "D",
                LongTermOrExtendedCareFacility => "E",
                NotSpecified => "F",
                NursingHome => "G",
                SubAcuteCareFacility => "H",
                OtherLocation => "L",
                RehabilitationFacility => "M",
                OutpatientFacility => "O",
                PrivateHome => "P",
                ResidentialTreatmentFacility => "R",
                SkilledNursingHome => "S",
                RestHome => "T",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PatientLocationCode> {
        use PatientLocationCode::*;
        match code {
            b"A" => Some(AcuteCareFacility),
            b"B" => Some(BoardingHome),
            b"C" => Some(Hospice),
            b"D" => Some(IntermediateCareFacility),
            b"E" => Some(LongTermOrExtendedCareFacility),
            b"F" => Some(NotSpecified),
            b"G" => Some(NursingHome),
            b"H" => Some(SubAcuteCareFacility),
            b"L" => Some(OtherLocation),
            b"M" => Some(RehabilitationFacility),
            b"O" => Some(OutpatientFacility),
            b"P" => Some(PrivateHome),
            b"R" => Some(ResidentialTreatmentFacility),
            b"S" => Some(SkilledNursingHome),
            b"T" => Some(RestHome),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PatientLocationCode::*;
        match self {
            AcuteCareFacility => "Acute Care Facility",
            BoardingHome => "Boarding Home",
            Hospice => "Hospice",
            IntermediateCareFacility => "Intermediate Care Facility",
            LongTermOrExtendedCareFacility => "Long-term or Extended Care Facility",
            NotSpecified => "Not Specified",
            NursingHome => "Nursing Home",
            SubAcuteCareFacility => "Sub-acute Care Facility",
            OtherLocation => "Other Location",
            RehabilitationFacility => "Rehabilitation Facility",
            OutpatientFacility => "Outpatient Facility",
            PrivateHome => "Private Home",
            ResidentialTreatmentFacility => "Residential Treatment Facility",
            SkilledNursingHome => "Skilled Nursing Home",
            RestHome => "Rest Home",
        }
    }
    fn from_description(description: &str) -> Option<PatientLocationCode> {
        {
            use PatientLocationCode::*;
            match description {
                "Acute Care Facility" => Some(AcuteCareFacility),
                "Boarding Home" => Some(BoardingHome),
                "Hospice" => Some(Hospice),
                "Intermediate Care Facility" => Some(IntermediateCareFacility),
                "Long-term or Extended Care Facility" => {
                    Some(LongTermOrExtendedCareFacility)
                }
                "Not Specified" => Some(NotSpecified),
                "Nursing Home" => Some(NursingHome),
                "Sub-acute Care Facility" => Some(SubAcuteCareFacility),
                "Other Location" => Some(OtherLocation),
                "Rehabilitation Facility" => Some(RehabilitationFacility),
                "Outpatient Facility" => Some(OutpatientFacility),
                "Private Home" => Some(PrivateHome),
                "Residential Treatment Facility" => Some(ResidentialTreatmentFacility),
                "Skilled Nursing Home" => Some(SkilledNursingHome),
                "Rest Home" => Some(RestHome),
                _ => None,
            }
        }
    }
}
impl Serialize for PatientLocationCode {
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
    type Value = PatientLocationCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Patient Location Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PatientLocationCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Patient Location Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PatientLocationCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Patient Location Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PatientLocationCode {
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