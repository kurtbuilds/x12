use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1317

See docs at <https://www.stedi.com/edi/x12-005010/element/1317>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AmbulanceTransportReasonCode {
    ///A - Patient was transported to nearest facility for care of symptoms, complaints, or both
    CodeA,
    ///B - Patient was transported for the benefit of a preferred physician
    PatientWasTransportedForTheBenefitOfAPreferredPhysician,
    ///C - Patient was transported for the nearness of family members
    PatientWasTransportedForTheNearnessOfFamilyMembers,
    ///D - Patient was transported for the care of a specialist or for availability of specialized equipment
    PatientWasTransportedForTheCareOfASpecialistOrForAvailabilityOfSpecializedEquipment,
    ///E - Patient Transferred to Rehabilitation Facility
    PatientTransferredToRehabilitationFacility,
    ///F - Patient Transferred to Residential Facility
    PatientTransferredToResidentialFacility,
}
impl AmbulanceTransportReasonCode {
    pub fn code(&self) -> &str {
        {
            use AmbulanceTransportReasonCode::*;
            match self {
                CodeA => "A",
                PatientWasTransportedForTheBenefitOfAPreferredPhysician => "B",
                PatientWasTransportedForTheNearnessOfFamilyMembers => "C",
                PatientWasTransportedForTheCareOfASpecialistOrForAvailabilityOfSpecializedEquipment => {
                    "D"
                }
                PatientTransferredToRehabilitationFacility => "E",
                PatientTransferredToResidentialFacility => "F",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<AmbulanceTransportReasonCode> {
        use AmbulanceTransportReasonCode::*;
        match code {
            b"A" => Some(CodeA),
            b"B" => Some(PatientWasTransportedForTheBenefitOfAPreferredPhysician),
            b"C" => Some(PatientWasTransportedForTheNearnessOfFamilyMembers),
            b"D" => {
                Some(
                    PatientWasTransportedForTheCareOfASpecialistOrForAvailabilityOfSpecializedEquipment,
                )
            }
            b"E" => Some(PatientTransferredToRehabilitationFacility),
            b"F" => Some(PatientTransferredToResidentialFacility),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use AmbulanceTransportReasonCode::*;
        match self {
            CodeA => {
                "Patient was transported to nearest facility for care of symptoms, complaints, or both"
            }
            PatientWasTransportedForTheBenefitOfAPreferredPhysician => {
                "Patient was transported for the benefit of a preferred physician"
            }
            PatientWasTransportedForTheNearnessOfFamilyMembers => {
                "Patient was transported for the nearness of family members"
            }
            PatientWasTransportedForTheCareOfASpecialistOrForAvailabilityOfSpecializedEquipment => {
                "Patient was transported for the care of a specialist or for availability of specialized equipment"
            }
            PatientTransferredToRehabilitationFacility => {
                "Patient Transferred to Rehabilitation Facility"
            }
            PatientTransferredToResidentialFacility => {
                "Patient Transferred to Residential Facility"
            }
        }
    }
    fn from_description(description: &str) -> Option<AmbulanceTransportReasonCode> {
        {
            use AmbulanceTransportReasonCode::*;
            match description {
                "Patient was transported to nearest facility for care of symptoms, complaints, or both" => {
                    Some(CodeA)
                }
                "Patient was transported for the benefit of a preferred physician" => {
                    Some(PatientWasTransportedForTheBenefitOfAPreferredPhysician)
                }
                "Patient was transported for the nearness of family members" => {
                    Some(PatientWasTransportedForTheNearnessOfFamilyMembers)
                }
                "Patient was transported for the care of a specialist or for availability of specialized equipment" => {
                    Some(
                        PatientWasTransportedForTheCareOfASpecialistOrForAvailabilityOfSpecializedEquipment,
                    )
                }
                "Patient Transferred to Rehabilitation Facility" => {
                    Some(PatientTransferredToRehabilitationFacility)
                }
                "Patient Transferred to Residential Facility" => {
                    Some(PatientTransferredToResidentialFacility)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for AmbulanceTransportReasonCode {
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
    type Value = AmbulanceTransportReasonCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Ambulance Transport Reason Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AmbulanceTransportReasonCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Ambulance Transport Reason Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AmbulanceTransportReasonCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Ambulance Transport Reason Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for AmbulanceTransportReasonCode {
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