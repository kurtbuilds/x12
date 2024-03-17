use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1322

See docs at <https://www.stedi.com/edi/x12/element/1322>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CertificationTypeCode {
    ///1 - Appeal - Immediate
    AppealImmediate,
    ///2 - Appeal - Standard
    AppealStandard,
    ///3 - Cancel
    Cancel,
    ///4 - Extension
    Extension,
    ///5 - Notification
    Notification,
    ///6 - Verification
    Verification,
    ///7 - Add-on
    AddOn,
    ///8 - Replacement
    Replacement,
    ///A - Certification of Current Cost and Pricing Data
    CertificationOfCurrentCostAndPricingData,
    ///B - Certification of Overhead
    CertificationOfOverhead,
    ///C - Certification Not Required
    CertificationNotRequired,
    ///D - Final
    Final,
    ///E - Recertification
    Recertification,
    ///I - Initial
    Initial,
    ///N - Reconsideration
    Reconsideration,
    ///R - Renewal
    Renewal,
    ///S - Revised
    Revised,
}
impl CertificationTypeCode {
    pub fn code(&self) -> &str {
        {
            use CertificationTypeCode::*;
            match self {
                AppealImmediate => "1",
                AppealStandard => "2",
                Cancel => "3",
                Extension => "4",
                Notification => "5",
                Verification => "6",
                AddOn => "7",
                Replacement => "8",
                CertificationOfCurrentCostAndPricingData => "A",
                CertificationOfOverhead => "B",
                CertificationNotRequired => "C",
                Final => "D",
                Recertification => "E",
                Initial => "I",
                Reconsideration => "N",
                Renewal => "R",
                Revised => "S",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CertificationTypeCode> {
        use CertificationTypeCode::*;
        match code {
            b"1" => Some(AppealImmediate),
            b"2" => Some(AppealStandard),
            b"3" => Some(Cancel),
            b"4" => Some(Extension),
            b"5" => Some(Notification),
            b"6" => Some(Verification),
            b"7" => Some(AddOn),
            b"8" => Some(Replacement),
            b"A" => Some(CertificationOfCurrentCostAndPricingData),
            b"B" => Some(CertificationOfOverhead),
            b"C" => Some(CertificationNotRequired),
            b"D" => Some(Final),
            b"E" => Some(Recertification),
            b"I" => Some(Initial),
            b"N" => Some(Reconsideration),
            b"R" => Some(Renewal),
            b"S" => Some(Revised),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CertificationTypeCode::*;
        match self {
            AppealImmediate => "Appeal - Immediate",
            AppealStandard => "Appeal - Standard",
            Cancel => "Cancel",
            Extension => "Extension",
            Notification => "Notification",
            Verification => "Verification",
            AddOn => "Add-on",
            Replacement => "Replacement",
            CertificationOfCurrentCostAndPricingData => {
                "Certification of Current Cost and Pricing Data"
            }
            CertificationOfOverhead => "Certification of Overhead",
            CertificationNotRequired => "Certification Not Required",
            Final => "Final",
            Recertification => "Recertification",
            Initial => "Initial",
            Reconsideration => "Reconsideration",
            Renewal => "Renewal",
            Revised => "Revised",
        }
    }
    fn from_description(description: &str) -> Option<CertificationTypeCode> {
        {
            use CertificationTypeCode::*;
            match description {
                "Appeal - Immediate" => Some(AppealImmediate),
                "Appeal - Standard" => Some(AppealStandard),
                "Cancel" => Some(Cancel),
                "Extension" => Some(Extension),
                "Notification" => Some(Notification),
                "Verification" => Some(Verification),
                "Add-on" => Some(AddOn),
                "Replacement" => Some(Replacement),
                "Certification of Current Cost and Pricing Data" => {
                    Some(CertificationOfCurrentCostAndPricingData)
                }
                "Certification of Overhead" => Some(CertificationOfOverhead),
                "Certification Not Required" => Some(CertificationNotRequired),
                "Final" => Some(Final),
                "Recertification" => Some(Recertification),
                "Initial" => Some(Initial),
                "Reconsideration" => Some(Reconsideration),
                "Renewal" => Some(Renewal),
                "Revised" => Some(Revised),
                _ => None,
            }
        }
    }
}
impl Serialize for CertificationTypeCode {
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
    type Value = CertificationTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Certification Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CertificationTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Certification Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CertificationTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Certification Type Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for CertificationTypeCode {
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