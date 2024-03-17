use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1254

See docs at <https://www.stedi.com/edi/x12/element/1254>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImmunizationStatusCode {
    ///1 - First Inoculation
    FirstInoculation,
    ///2 - Second Inoculation
    SecondInoculation,
    ///3 - Third Inoculation
    ThirdInoculation,
    ///4 - Fourth Inoculation
    FourthInoculation,
    ///5 - Fifth Inoculation
    FifthInoculation,
    ///6 - Sixth Inoculation
    SixthInoculation,
    ///7 - Seventh Inoculation
    SeventhInoculation,
    ///8 - Eighth Inoculation
    EighthInoculation,
    ///9 - Ninth Inoculation
    NinthInoculation,
    ///10 - Medical Exemption
    MedicalExemption,
    ///11 - Personal Exemption
    PersonalExemption,
    ///12 - Religious Exemption
    ReligiousExemption,
    ///13 - Had the Disease
    HadTheDisease,
    ///14 - Has Not Had the Disease
    HasNotHadTheDisease,
}
impl ImmunizationStatusCode {
    pub fn code(&self) -> &str {
        {
            use ImmunizationStatusCode::*;
            match self {
                FirstInoculation => "1",
                SecondInoculation => "2",
                ThirdInoculation => "3",
                FourthInoculation => "4",
                FifthInoculation => "5",
                SixthInoculation => "6",
                SeventhInoculation => "7",
                EighthInoculation => "8",
                NinthInoculation => "9",
                MedicalExemption => "10",
                PersonalExemption => "11",
                ReligiousExemption => "12",
                HadTheDisease => "13",
                HasNotHadTheDisease => "14",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ImmunizationStatusCode> {
        use ImmunizationStatusCode::*;
        match code {
            b"1" => Some(FirstInoculation),
            b"2" => Some(SecondInoculation),
            b"3" => Some(ThirdInoculation),
            b"4" => Some(FourthInoculation),
            b"5" => Some(FifthInoculation),
            b"6" => Some(SixthInoculation),
            b"7" => Some(SeventhInoculation),
            b"8" => Some(EighthInoculation),
            b"9" => Some(NinthInoculation),
            b"10" => Some(MedicalExemption),
            b"11" => Some(PersonalExemption),
            b"12" => Some(ReligiousExemption),
            b"13" => Some(HadTheDisease),
            b"14" => Some(HasNotHadTheDisease),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ImmunizationStatusCode::*;
        match self {
            FirstInoculation => "First Inoculation",
            SecondInoculation => "Second Inoculation",
            ThirdInoculation => "Third Inoculation",
            FourthInoculation => "Fourth Inoculation",
            FifthInoculation => "Fifth Inoculation",
            SixthInoculation => "Sixth Inoculation",
            SeventhInoculation => "Seventh Inoculation",
            EighthInoculation => "Eighth Inoculation",
            NinthInoculation => "Ninth Inoculation",
            MedicalExemption => "Medical Exemption",
            PersonalExemption => "Personal Exemption",
            ReligiousExemption => "Religious Exemption",
            HadTheDisease => "Had the Disease",
            HasNotHadTheDisease => "Has Not Had the Disease",
        }
    }
    fn from_description(description: &str) -> Option<ImmunizationStatusCode> {
        {
            use ImmunizationStatusCode::*;
            match description {
                "First Inoculation" => Some(FirstInoculation),
                "Second Inoculation" => Some(SecondInoculation),
                "Third Inoculation" => Some(ThirdInoculation),
                "Fourth Inoculation" => Some(FourthInoculation),
                "Fifth Inoculation" => Some(FifthInoculation),
                "Sixth Inoculation" => Some(SixthInoculation),
                "Seventh Inoculation" => Some(SeventhInoculation),
                "Eighth Inoculation" => Some(EighthInoculation),
                "Ninth Inoculation" => Some(NinthInoculation),
                "Medical Exemption" => Some(MedicalExemption),
                "Personal Exemption" => Some(PersonalExemption),
                "Religious Exemption" => Some(ReligiousExemption),
                "Had the Disease" => Some(HadTheDisease),
                "Has Not Had the Disease" => Some(HasNotHadTheDisease),
                _ => None,
            }
        }
    }
}
impl Serialize for ImmunizationStatusCode {
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
    type Value = ImmunizationStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Immunization Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImmunizationStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Immunization Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImmunizationStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Immunization Status Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ImmunizationStatusCode {
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