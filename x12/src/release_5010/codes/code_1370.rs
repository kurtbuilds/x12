use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1370

See docs at <https://www.stedi.com/edi/x12/element/1370>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitDoseCode {
    ///0 - Not Specified
    NotSpecified,
    ///1 - Not Unit Dose
    NotUnitDose,
    ///2 - Manufacturer Unit Dose
    ManufacturerUnitDose,
    ///3 - Pharmacy Unit Dose
    PharmacyUnitDose,
    ///4 - Treated Group
    TreatedGroup,
    ///5 - Untreated Control Group
    UntreatedControlGroup,
    ///6 - Vehicle Control Group
    VehicleControlGroup,
    ///7 - Positive Control Group
    PositiveControlGroup,
    ///8 - Optional Control Group
    OptionalControlGroup,
    ///Z - Mutually Defined
    MutuallyDefined,
}
impl UnitDoseCode {
    pub fn code(&self) -> &str {
        {
            use UnitDoseCode::*;
            match self {
                NotSpecified => "0",
                NotUnitDose => "1",
                ManufacturerUnitDose => "2",
                PharmacyUnitDose => "3",
                TreatedGroup => "4",
                UntreatedControlGroup => "5",
                VehicleControlGroup => "6",
                PositiveControlGroup => "7",
                OptionalControlGroup => "8",
                MutuallyDefined => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<UnitDoseCode> {
        use UnitDoseCode::*;
        match code {
            b"0" => Some(NotSpecified),
            b"1" => Some(NotUnitDose),
            b"2" => Some(ManufacturerUnitDose),
            b"3" => Some(PharmacyUnitDose),
            b"4" => Some(TreatedGroup),
            b"5" => Some(UntreatedControlGroup),
            b"6" => Some(VehicleControlGroup),
            b"7" => Some(PositiveControlGroup),
            b"8" => Some(OptionalControlGroup),
            b"Z" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use UnitDoseCode::*;
        match self {
            NotSpecified => "Not Specified",
            NotUnitDose => "Not Unit Dose",
            ManufacturerUnitDose => "Manufacturer Unit Dose",
            PharmacyUnitDose => "Pharmacy Unit Dose",
            TreatedGroup => "Treated Group",
            UntreatedControlGroup => "Untreated Control Group",
            VehicleControlGroup => "Vehicle Control Group",
            PositiveControlGroup => "Positive Control Group",
            OptionalControlGroup => "Optional Control Group",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<UnitDoseCode> {
        {
            use UnitDoseCode::*;
            match description {
                "Not Specified" => Some(NotSpecified),
                "Not Unit Dose" => Some(NotUnitDose),
                "Manufacturer Unit Dose" => Some(ManufacturerUnitDose),
                "Pharmacy Unit Dose" => Some(PharmacyUnitDose),
                "Treated Group" => Some(TreatedGroup),
                "Untreated Control Group" => Some(UntreatedControlGroup),
                "Vehicle Control Group" => Some(VehicleControlGroup),
                "Positive Control Group" => Some(PositiveControlGroup),
                "Optional Control Group" => Some(OptionalControlGroup),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for UnitDoseCode {
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
    type Value = UnitDoseCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Unit Dose Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnitDoseCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Unit Dose Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnitDoseCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Unit Dose Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for UnitDoseCode {
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