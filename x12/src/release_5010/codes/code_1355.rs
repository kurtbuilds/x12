use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1355

See docs at <https://www.stedi.com/edi/x12-005010/element/1355>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrescriptionDenialOverrideCode {
    ///00 - Not specified
    NotSpecified,
    ///01 - No override
    NoOverride,
    ///02 - Other override
    OtherOverride,
    ///03 - Vacation Supply
    VacationSupply,
    ///04 - Lost Prescription
    LostPrescription,
    ///05 - Therapy Change
    TherapyChange,
    ///06 - Starter Dose
    StarterDose,
    ///07 - Medically Necessary
    MedicallyNecessary,
}
impl PrescriptionDenialOverrideCode {
    pub fn code(&self) -> &str {
        {
            use PrescriptionDenialOverrideCode::*;
            match self {
                NotSpecified => "00",
                NoOverride => "01",
                OtherOverride => "02",
                VacationSupply => "03",
                LostPrescription => "04",
                TherapyChange => "05",
                StarterDose => "06",
                MedicallyNecessary => "07",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PrescriptionDenialOverrideCode> {
        use PrescriptionDenialOverrideCode::*;
        match code {
            b"00" => Some(NotSpecified),
            b"01" => Some(NoOverride),
            b"02" => Some(OtherOverride),
            b"03" => Some(VacationSupply),
            b"04" => Some(LostPrescription),
            b"05" => Some(TherapyChange),
            b"06" => Some(StarterDose),
            b"07" => Some(MedicallyNecessary),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PrescriptionDenialOverrideCode::*;
        match self {
            NotSpecified => "Not specified",
            NoOverride => "No override",
            OtherOverride => "Other override",
            VacationSupply => "Vacation Supply",
            LostPrescription => "Lost Prescription",
            TherapyChange => "Therapy Change",
            StarterDose => "Starter Dose",
            MedicallyNecessary => "Medically Necessary",
        }
    }
    fn from_description(description: &str) -> Option<PrescriptionDenialOverrideCode> {
        {
            use PrescriptionDenialOverrideCode::*;
            match description {
                "Not specified" => Some(NotSpecified),
                "No override" => Some(NoOverride),
                "Other override" => Some(OtherOverride),
                "Vacation Supply" => Some(VacationSupply),
                "Lost Prescription" => Some(LostPrescription),
                "Therapy Change" => Some(TherapyChange),
                "Starter Dose" => Some(StarterDose),
                "Medically Necessary" => Some(MedicallyNecessary),
                _ => None,
            }
        }
    }
}
impl Serialize for PrescriptionDenialOverrideCode {
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
    type Value = PrescriptionDenialOverrideCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Prescription Denial Override Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PrescriptionDenialOverrideCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Prescription Denial Override Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PrescriptionDenialOverrideCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Prescription Denial Override Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PrescriptionDenialOverrideCode {
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