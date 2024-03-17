use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1345

See docs at <https://www.stedi.com/edi/x12-005010/element/1345>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NursingHomeResidentialStatusCode {
    ///1 - Transferred to Intermediate Care Facility - Mentally Retarded (ICF-MR)
    Code1,
    ///2 - Newly Admitted
    NewlyAdmitted,
    ///3 - Newly Eligible
    NewlyEligible,
    ///4 - No Longer Eligible
    NoLongerEligible,
    ///5 - Still a Resident
    StillAResident,
    ///6 - Temporary Absence - Hospital
    TemporaryAbsenceHospital,
    ///7 - Temporary Absence - Other
    TemporaryAbsenceOther,
    ///8 - Transferred to Intermediate Care Facility - Level II (ICF II)
    Code8,
    ///9 - Other
    Other,
}
impl NursingHomeResidentialStatusCode {
    pub fn code(&self) -> &str {
        {
            use NursingHomeResidentialStatusCode::*;
            match self {
                Code1 => "1",
                NewlyAdmitted => "2",
                NewlyEligible => "3",
                NoLongerEligible => "4",
                StillAResident => "5",
                TemporaryAbsenceHospital => "6",
                TemporaryAbsenceOther => "7",
                Code8 => "8",
                Other => "9",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<NursingHomeResidentialStatusCode> {
        use NursingHomeResidentialStatusCode::*;
        match code {
            b"1" => Some(Code1),
            b"2" => Some(NewlyAdmitted),
            b"3" => Some(NewlyEligible),
            b"4" => Some(NoLongerEligible),
            b"5" => Some(StillAResident),
            b"6" => Some(TemporaryAbsenceHospital),
            b"7" => Some(TemporaryAbsenceOther),
            b"8" => Some(Code8),
            b"9" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use NursingHomeResidentialStatusCode::*;
        match self {
            Code1 => {
                "Transferred to Intermediate Care Facility - Mentally Retarded (ICF-MR)"
            }
            NewlyAdmitted => "Newly Admitted",
            NewlyEligible => "Newly Eligible",
            NoLongerEligible => "No Longer Eligible",
            StillAResident => "Still a Resident",
            TemporaryAbsenceHospital => "Temporary Absence - Hospital",
            TemporaryAbsenceOther => "Temporary Absence - Other",
            Code8 => "Transferred to Intermediate Care Facility - Level II (ICF II)",
            Other => "Other",
        }
    }
    fn from_description(description: &str) -> Option<NursingHomeResidentialStatusCode> {
        {
            use NursingHomeResidentialStatusCode::*;
            match description {
                "Transferred to Intermediate Care Facility - Mentally Retarded (ICF-MR)" => {
                    Some(Code1)
                }
                "Newly Admitted" => Some(NewlyAdmitted),
                "Newly Eligible" => Some(NewlyEligible),
                "No Longer Eligible" => Some(NoLongerEligible),
                "Still a Resident" => Some(StillAResident),
                "Temporary Absence - Hospital" => Some(TemporaryAbsenceHospital),
                "Temporary Absence - Other" => Some(TemporaryAbsenceOther),
                "Transferred to Intermediate Care Facility - Level II (ICF II)" => {
                    Some(Code8)
                }
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for NursingHomeResidentialStatusCode {
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
    type Value = NursingHomeResidentialStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Nursing Home Residential Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NursingHomeResidentialStatusCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Nursing Home Residential Status Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NursingHomeResidentialStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Nursing Home Residential Status Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for NursingHomeResidentialStatusCode {
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