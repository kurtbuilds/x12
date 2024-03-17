use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1366

See docs at <https://www.stedi.com/edi/x12/element/1366>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpecialProgramCode {
    ///01 - Early & Periodic Screening, Diagnosis, and Treatment (EPSDT) or Child Health Assessment Program (CHAP)
    Code01,
    ///02 - Physically Handicapped Children's Program
    PhysicallyHandicappedChildrensProgram,
    ///03 - Special Federal Funding
    SpecialFederalFunding,
    ///04 - Family Planning
    FamilyPlanning,
    ///05 - Disability
    Disability,
    ///06 - Pneumococcal Pneumonia Vaccine (PPV) or Medicare 100% Payment
    Code06,
    ///07 - Induced Abortion - Danger to Life
    InducedAbortionDangerToLife,
    ///08 - Induced Abortion - Rape or Incest
    InducedAbortionRapeOrIncest,
    ///09 - Second Opinion or Surgery
    SecondOpinionOrSurgery,
    ///10 - Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) External Partnership Program
    Code10,
}
impl SpecialProgramCode {
    pub fn code(&self) -> &str {
        {
            use SpecialProgramCode::*;
            match self {
                Code01 => "01",
                PhysicallyHandicappedChildrensProgram => "02",
                SpecialFederalFunding => "03",
                FamilyPlanning => "04",
                Disability => "05",
                Code06 => "06",
                InducedAbortionDangerToLife => "07",
                InducedAbortionRapeOrIncest => "08",
                SecondOpinionOrSurgery => "09",
                Code10 => "10",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<SpecialProgramCode> {
        use SpecialProgramCode::*;
        match code {
            b"01" => Some(Code01),
            b"02" => Some(PhysicallyHandicappedChildrensProgram),
            b"03" => Some(SpecialFederalFunding),
            b"04" => Some(FamilyPlanning),
            b"05" => Some(Disability),
            b"06" => Some(Code06),
            b"07" => Some(InducedAbortionDangerToLife),
            b"08" => Some(InducedAbortionRapeOrIncest),
            b"09" => Some(SecondOpinionOrSurgery),
            b"10" => Some(Code10),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use SpecialProgramCode::*;
        match self {
            Code01 => {
                "Early & Periodic Screening, Diagnosis, and Treatment (EPSDT) or Child Health Assessment Program (CHAP)"
            }
            PhysicallyHandicappedChildrensProgram => {
                "Physically Handicapped Children's Program"
            }
            SpecialFederalFunding => "Special Federal Funding",
            FamilyPlanning => "Family Planning",
            Disability => "Disability",
            Code06 => "Pneumococcal Pneumonia Vaccine (PPV) or Medicare 100% Payment",
            InducedAbortionDangerToLife => "Induced Abortion - Danger to Life",
            InducedAbortionRapeOrIncest => "Induced Abortion - Rape or Incest",
            SecondOpinionOrSurgery => "Second Opinion or Surgery",
            Code10 => {
                "Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) External Partnership Program"
            }
        }
    }
    fn from_description(description: &str) -> Option<SpecialProgramCode> {
        {
            use SpecialProgramCode::*;
            match description {
                "Early & Periodic Screening, Diagnosis, and Treatment (EPSDT) or Child Health Assessment Program (CHAP)" => {
                    Some(Code01)
                }
                "Physically Handicapped Children's Program" => {
                    Some(PhysicallyHandicappedChildrensProgram)
                }
                "Special Federal Funding" => Some(SpecialFederalFunding),
                "Family Planning" => Some(FamilyPlanning),
                "Disability" => Some(Disability),
                "Pneumococcal Pneumonia Vaccine (PPV) or Medicare 100% Payment" => {
                    Some(Code06)
                }
                "Induced Abortion - Danger to Life" => Some(InducedAbortionDangerToLife),
                "Induced Abortion - Rape or Incest" => Some(InducedAbortionRapeOrIncest),
                "Second Opinion or Surgery" => Some(SecondOpinionOrSurgery),
                "Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) External Partnership Program" => {
                    Some(Code10)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for SpecialProgramCode {
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
    type Value = SpecialProgramCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Special Program Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SpecialProgramCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Special Program Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SpecialProgramCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Special Program Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for SpecialProgramCode {
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