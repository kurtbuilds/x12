use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1382

See docs at <https://www.stedi.com/edi/x12-005010/element/1382>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OxygenDeliverySystemCode {
    ///A - Nasal Cannula
    NasalCannula,
    ///B - Oxygen Conserving Device
    OxygenConservingDevice,
    ///C - Oxygen Conserving Device with Oxygen Pulse System
    OxygenConservingDeviceWithOxygenPulseSystem,
    ///D - Oxygen Conserving Device with Reservoir System
    OxygenConservingDeviceWithReservoirSystem,
    ///E - Transtracheal Catheter
    TranstrachealCatheter,
}
impl OxygenDeliverySystemCode {
    pub fn code(&self) -> &str {
        {
            use OxygenDeliverySystemCode::*;
            match self {
                NasalCannula => "A",
                OxygenConservingDevice => "B",
                OxygenConservingDeviceWithOxygenPulseSystem => "C",
                OxygenConservingDeviceWithReservoirSystem => "D",
                TranstrachealCatheter => "E",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<OxygenDeliverySystemCode> {
        use OxygenDeliverySystemCode::*;
        match code {
            b"A" => Some(NasalCannula),
            b"B" => Some(OxygenConservingDevice),
            b"C" => Some(OxygenConservingDeviceWithOxygenPulseSystem),
            b"D" => Some(OxygenConservingDeviceWithReservoirSystem),
            b"E" => Some(TranstrachealCatheter),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use OxygenDeliverySystemCode::*;
        match self {
            NasalCannula => "Nasal Cannula",
            OxygenConservingDevice => "Oxygen Conserving Device",
            OxygenConservingDeviceWithOxygenPulseSystem => {
                "Oxygen Conserving Device with Oxygen Pulse System"
            }
            OxygenConservingDeviceWithReservoirSystem => {
                "Oxygen Conserving Device with Reservoir System"
            }
            TranstrachealCatheter => "Transtracheal Catheter",
        }
    }
    fn from_description(description: &str) -> Option<OxygenDeliverySystemCode> {
        {
            use OxygenDeliverySystemCode::*;
            match description {
                "Nasal Cannula" => Some(NasalCannula),
                "Oxygen Conserving Device" => Some(OxygenConservingDevice),
                "Oxygen Conserving Device with Oxygen Pulse System" => {
                    Some(OxygenConservingDeviceWithOxygenPulseSystem)
                }
                "Oxygen Conserving Device with Reservoir System" => {
                    Some(OxygenConservingDeviceWithReservoirSystem)
                }
                "Transtracheal Catheter" => Some(TranstrachealCatheter),
                _ => None,
            }
        }
    }
}
impl Serialize for OxygenDeliverySystemCode {
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
    type Value = OxygenDeliverySystemCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Oxygen Delivery System Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        OxygenDeliverySystemCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Oxygen Delivery System Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        OxygenDeliverySystemCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Oxygen Delivery System Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for OxygenDeliverySystemCode {
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