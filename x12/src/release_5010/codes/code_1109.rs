use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1109

See docs at <https://www.stedi.com/edi/x12/element/1109>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RaceOrEthnicityCode {
    ///7 - Not Provided
    NotProvided,
    ///8 - Not Applicable
    NotApplicable,
    ///A - Asian or Pacific Islander
    AsianOrPacificIslander,
    ///B - Black
    Black,
    ///C - Caucasian
    Caucasian,
    ///D - Subcontinent Asian American
    SubcontinentAsianAmerican,
    ///E - Other Race or Ethnicity
    OtherRaceOrEthnicity,
    ///F - Asian Pacific American
    AsianPacificAmerican,
    ///G - Native American
    NativeAmerican,
    ///H - Hispanic
    Hispanic,
    ///I - American Indian or Alaskan Native
    AmericanIndianOrAlaskanNative,
    ///J - Native Hawaiian
    NativeHawaiian,
    ///N - Black (Non-Hispanic)
    CodeN,
    ///O - White (Non-Hispanic)
    CodeO,
    ///P - Pacific Islander
    PacificIslander,
    ///Q - Black or African American (Office of Management and Budget 1997)
    CodeQ,
    ///R - Hispanic or Latino (Office of Management and Budget 1997)
    CodeR,
    ///S - White (Office of Management and Budget 1997)
    CodeS,
    ///T - American Indian or Alaska Native (Office of Management and Budget 1997)
    CodeT,
    ///U - Asian (Office of Management and Budget 1997)
    CodeU,
    ///V - Native Hawaiian or Other Pacific Islander (Office of Management and Budget 1997)
    CodeV,
    ///W - Not Hispanic or Latino (Office of Management and Budget 1997)
    CodeW,
    ///Z - Mutually Defined
    MutuallyDefined,
}
impl RaceOrEthnicityCode {
    pub fn code(&self) -> &str {
        {
            use RaceOrEthnicityCode::*;
            match self {
                NotProvided => "7",
                NotApplicable => "8",
                AsianOrPacificIslander => "A",
                Black => "B",
                Caucasian => "C",
                SubcontinentAsianAmerican => "D",
                OtherRaceOrEthnicity => "E",
                AsianPacificAmerican => "F",
                NativeAmerican => "G",
                Hispanic => "H",
                AmericanIndianOrAlaskanNative => "I",
                NativeHawaiian => "J",
                CodeN => "N",
                CodeO => "O",
                PacificIslander => "P",
                CodeQ => "Q",
                CodeR => "R",
                CodeS => "S",
                CodeT => "T",
                CodeU => "U",
                CodeV => "V",
                CodeW => "W",
                MutuallyDefined => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<RaceOrEthnicityCode> {
        use RaceOrEthnicityCode::*;
        match code {
            b"7" => Some(NotProvided),
            b"8" => Some(NotApplicable),
            b"A" => Some(AsianOrPacificIslander),
            b"B" => Some(Black),
            b"C" => Some(Caucasian),
            b"D" => Some(SubcontinentAsianAmerican),
            b"E" => Some(OtherRaceOrEthnicity),
            b"F" => Some(AsianPacificAmerican),
            b"G" => Some(NativeAmerican),
            b"H" => Some(Hispanic),
            b"I" => Some(AmericanIndianOrAlaskanNative),
            b"J" => Some(NativeHawaiian),
            b"N" => Some(CodeN),
            b"O" => Some(CodeO),
            b"P" => Some(PacificIslander),
            b"Q" => Some(CodeQ),
            b"R" => Some(CodeR),
            b"S" => Some(CodeS),
            b"T" => Some(CodeT),
            b"U" => Some(CodeU),
            b"V" => Some(CodeV),
            b"W" => Some(CodeW),
            b"Z" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use RaceOrEthnicityCode::*;
        match self {
            NotProvided => "Not Provided",
            NotApplicable => "Not Applicable",
            AsianOrPacificIslander => "Asian or Pacific Islander",
            Black => "Black",
            Caucasian => "Caucasian",
            SubcontinentAsianAmerican => "Subcontinent Asian American",
            OtherRaceOrEthnicity => "Other Race or Ethnicity",
            AsianPacificAmerican => "Asian Pacific American",
            NativeAmerican => "Native American",
            Hispanic => "Hispanic",
            AmericanIndianOrAlaskanNative => "American Indian or Alaskan Native",
            NativeHawaiian => "Native Hawaiian",
            CodeN => "Black (Non-Hispanic)",
            CodeO => "White (Non-Hispanic)",
            PacificIslander => "Pacific Islander",
            CodeQ => "Black or African American (Office of Management and Budget 1997)",
            CodeR => "Hispanic or Latino (Office of Management and Budget 1997)",
            CodeS => "White (Office of Management and Budget 1997)",
            CodeT => {
                "American Indian or Alaska Native (Office of Management and Budget 1997)"
            }
            CodeU => "Asian (Office of Management and Budget 1997)",
            CodeV => {
                "Native Hawaiian or Other Pacific Islander (Office of Management and Budget 1997)"
            }
            CodeW => "Not Hispanic or Latino (Office of Management and Budget 1997)",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<RaceOrEthnicityCode> {
        {
            use RaceOrEthnicityCode::*;
            match description {
                "Not Provided" => Some(NotProvided),
                "Not Applicable" => Some(NotApplicable),
                "Asian or Pacific Islander" => Some(AsianOrPacificIslander),
                "Black" => Some(Black),
                "Caucasian" => Some(Caucasian),
                "Subcontinent Asian American" => Some(SubcontinentAsianAmerican),
                "Other Race or Ethnicity" => Some(OtherRaceOrEthnicity),
                "Asian Pacific American" => Some(AsianPacificAmerican),
                "Native American" => Some(NativeAmerican),
                "Hispanic" => Some(Hispanic),
                "American Indian or Alaskan Native" => {
                    Some(AmericanIndianOrAlaskanNative)
                }
                "Native Hawaiian" => Some(NativeHawaiian),
                "Black (Non-Hispanic)" => Some(CodeN),
                "White (Non-Hispanic)" => Some(CodeO),
                "Pacific Islander" => Some(PacificIslander),
                "Black or African American (Office of Management and Budget 1997)" => {
                    Some(CodeQ)
                }
                "Hispanic or Latino (Office of Management and Budget 1997)" => {
                    Some(CodeR)
                }
                "White (Office of Management and Budget 1997)" => Some(CodeS),
                "American Indian or Alaska Native (Office of Management and Budget 1997)" => {
                    Some(CodeT)
                }
                "Asian (Office of Management and Budget 1997)" => Some(CodeU),
                "Native Hawaiian or Other Pacific Islander (Office of Management and Budget 1997)" => {
                    Some(CodeV)
                }
                "Not Hispanic or Latino (Office of Management and Budget 1997)" => {
                    Some(CodeW)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for RaceOrEthnicityCode {
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
    type Value = RaceOrEthnicityCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Race or Ethnicity Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RaceOrEthnicityCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Race or Ethnicity Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RaceOrEthnicityCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Race or Ethnicity Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for RaceOrEthnicityCode {
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