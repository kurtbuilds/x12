use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1338

See docs at <https://www.stedi.com/edi/x12-005010/element/1338>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LevelOfServiceCode {
    ///00 - Not specified
    NotSpecified,
    ///01 - Patient Consultation
    PatientConsultation,
    ///02 - Home delivery
    HomeDelivery,
    ///03 - Emergency
    Emergency,
    ///04 - 24 Hour
    Code04,
    ///05 - Patient Consultation Regarding Generic Product Selection
    PatientConsultationRegardingGenericProductSelection,
    ///06 - As Needed
    AsNeeded,
    ///09 - Other
    Other,
    ///10 - Initial Office Visit
    InitialOfficeVisit,
    ///11 - Follow-up Office Visit
    FollowUpOfficeVisit,
    ///E - Elective
    Elective,
    ///F1 - Full Treatment - Phase One
    FullTreatmentPhaseOne,
    ///F2 - Full Treatment - Phase Two
    FullTreatmentPhaseTwo,
    ///I - Initial
    Initial,
    ///L - Limited Treatment
    LimitedTreatment,
    ///NBC - Newborn Care
    NewbornCare,
    ///R - Routine
    Routine,
    ///U - Urgent
    Urgent,
}
impl LevelOfServiceCode {
    pub fn code(&self) -> &str {
        {
            use LevelOfServiceCode::*;
            match self {
                NotSpecified => "00",
                PatientConsultation => "01",
                HomeDelivery => "02",
                Emergency => "03",
                Code04 => "04",
                PatientConsultationRegardingGenericProductSelection => "05",
                AsNeeded => "06",
                Other => "09",
                InitialOfficeVisit => "10",
                FollowUpOfficeVisit => "11",
                Elective => "E",
                FullTreatmentPhaseOne => "F1",
                FullTreatmentPhaseTwo => "F2",
                Initial => "I",
                LimitedTreatment => "L",
                NewbornCare => "NBC",
                Routine => "R",
                Urgent => "U",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<LevelOfServiceCode> {
        use LevelOfServiceCode::*;
        match code {
            b"00" => Some(NotSpecified),
            b"01" => Some(PatientConsultation),
            b"02" => Some(HomeDelivery),
            b"03" => Some(Emergency),
            b"04" => Some(Code04),
            b"05" => Some(PatientConsultationRegardingGenericProductSelection),
            b"06" => Some(AsNeeded),
            b"09" => Some(Other),
            b"10" => Some(InitialOfficeVisit),
            b"11" => Some(FollowUpOfficeVisit),
            b"E" => Some(Elective),
            b"F1" => Some(FullTreatmentPhaseOne),
            b"F2" => Some(FullTreatmentPhaseTwo),
            b"I" => Some(Initial),
            b"L" => Some(LimitedTreatment),
            b"NBC" => Some(NewbornCare),
            b"R" => Some(Routine),
            b"U" => Some(Urgent),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use LevelOfServiceCode::*;
        match self {
            NotSpecified => "Not specified",
            PatientConsultation => "Patient Consultation",
            HomeDelivery => "Home delivery",
            Emergency => "Emergency",
            Code04 => "24 Hour",
            PatientConsultationRegardingGenericProductSelection => {
                "Patient Consultation Regarding Generic Product Selection"
            }
            AsNeeded => "As Needed",
            Other => "Other",
            InitialOfficeVisit => "Initial Office Visit",
            FollowUpOfficeVisit => "Follow-up Office Visit",
            Elective => "Elective",
            FullTreatmentPhaseOne => "Full Treatment - Phase One",
            FullTreatmentPhaseTwo => "Full Treatment - Phase Two",
            Initial => "Initial",
            LimitedTreatment => "Limited Treatment",
            NewbornCare => "Newborn Care",
            Routine => "Routine",
            Urgent => "Urgent",
        }
    }
    fn from_description(description: &str) -> Option<LevelOfServiceCode> {
        {
            use LevelOfServiceCode::*;
            match description {
                "Not specified" => Some(NotSpecified),
                "Patient Consultation" => Some(PatientConsultation),
                "Home delivery" => Some(HomeDelivery),
                "Emergency" => Some(Emergency),
                "24 Hour" => Some(Code04),
                "Patient Consultation Regarding Generic Product Selection" => {
                    Some(PatientConsultationRegardingGenericProductSelection)
                }
                "As Needed" => Some(AsNeeded),
                "Other" => Some(Other),
                "Initial Office Visit" => Some(InitialOfficeVisit),
                "Follow-up Office Visit" => Some(FollowUpOfficeVisit),
                "Elective" => Some(Elective),
                "Full Treatment - Phase One" => Some(FullTreatmentPhaseOne),
                "Full Treatment - Phase Two" => Some(FullTreatmentPhaseTwo),
                "Initial" => Some(Initial),
                "Limited Treatment" => Some(LimitedTreatment),
                "Newborn Care" => Some(NewbornCare),
                "Routine" => Some(Routine),
                "Urgent" => Some(Urgent),
                _ => None,
            }
        }
    }
}
impl Serialize for LevelOfServiceCode {
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
    type Value = LevelOfServiceCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Level of Service Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LevelOfServiceCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Level of Service Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LevelOfServiceCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Level of Service Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for LevelOfServiceCode {
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