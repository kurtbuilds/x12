use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1373

See docs at <https://www.stedi.com/edi/x12/element/1373>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementMethodOrDeviceCode {
    ///BM - Bellows Meter
    BellowsMeter,
    ///BO - Based on 120 Days List to Contract
    BasedOn120DaysListToContract,
    ///DM - Displacement Meter
    DisplacementMeter,
    ///FT - Flange Tap
    FlangeTap,
    ///HN - Hubometer
    Hubometer,
    ///MA - Measured Over Six Months
    MeasuredOverSixMonths,
    ///MM - Mercury Meter
    MercuryMeter,
    ///OM - Orifice Meter
    OrificeMeter,
    ///OU - Odometer
    Odometer,
    ///PT - Pipe Tap
    PipeTap,
    ///TM - Turbine Meter
    TurbineMeter,
    ///VA - Valued at 90 Day Marketing Time Method
    ValuedAt90DayMarketingTimeMethod,
    ///VB - Valued at 120 Day Marketing Time Method
    ValuedAt120DayMarketingTimeMethod,
    ///VC - Valued at 180 Day Marketing Time Method
    ValuedAt180DayMarketingTimeMethod,
}
impl MeasurementMethodOrDeviceCode {
    pub fn code(&self) -> &str {
        {
            use MeasurementMethodOrDeviceCode::*;
            match self {
                BellowsMeter => "BM",
                BasedOn120DaysListToContract => "BO",
                DisplacementMeter => "DM",
                FlangeTap => "FT",
                Hubometer => "HN",
                MeasuredOverSixMonths => "MA",
                MercuryMeter => "MM",
                OrificeMeter => "OM",
                Odometer => "OU",
                PipeTap => "PT",
                TurbineMeter => "TM",
                ValuedAt90DayMarketingTimeMethod => "VA",
                ValuedAt120DayMarketingTimeMethod => "VB",
                ValuedAt180DayMarketingTimeMethod => "VC",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<MeasurementMethodOrDeviceCode> {
        use MeasurementMethodOrDeviceCode::*;
        match code {
            b"BM" => Some(BellowsMeter),
            b"BO" => Some(BasedOn120DaysListToContract),
            b"DM" => Some(DisplacementMeter),
            b"FT" => Some(FlangeTap),
            b"HN" => Some(Hubometer),
            b"MA" => Some(MeasuredOverSixMonths),
            b"MM" => Some(MercuryMeter),
            b"OM" => Some(OrificeMeter),
            b"OU" => Some(Odometer),
            b"PT" => Some(PipeTap),
            b"TM" => Some(TurbineMeter),
            b"VA" => Some(ValuedAt90DayMarketingTimeMethod),
            b"VB" => Some(ValuedAt120DayMarketingTimeMethod),
            b"VC" => Some(ValuedAt180DayMarketingTimeMethod),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use MeasurementMethodOrDeviceCode::*;
        match self {
            BellowsMeter => "Bellows Meter",
            BasedOn120DaysListToContract => "Based on 120 Days List to Contract",
            DisplacementMeter => "Displacement Meter",
            FlangeTap => "Flange Tap",
            Hubometer => "Hubometer",
            MeasuredOverSixMonths => "Measured Over Six Months",
            MercuryMeter => "Mercury Meter",
            OrificeMeter => "Orifice Meter",
            Odometer => "Odometer",
            PipeTap => "Pipe Tap",
            TurbineMeter => "Turbine Meter",
            ValuedAt90DayMarketingTimeMethod => "Valued at 90 Day Marketing Time Method",
            ValuedAt120DayMarketingTimeMethod => {
                "Valued at 120 Day Marketing Time Method"
            }
            ValuedAt180DayMarketingTimeMethod => {
                "Valued at 180 Day Marketing Time Method"
            }
        }
    }
    fn from_description(description: &str) -> Option<MeasurementMethodOrDeviceCode> {
        {
            use MeasurementMethodOrDeviceCode::*;
            match description {
                "Bellows Meter" => Some(BellowsMeter),
                "Based on 120 Days List to Contract" => {
                    Some(BasedOn120DaysListToContract)
                }
                "Displacement Meter" => Some(DisplacementMeter),
                "Flange Tap" => Some(FlangeTap),
                "Hubometer" => Some(Hubometer),
                "Measured Over Six Months" => Some(MeasuredOverSixMonths),
                "Mercury Meter" => Some(MercuryMeter),
                "Orifice Meter" => Some(OrificeMeter),
                "Odometer" => Some(Odometer),
                "Pipe Tap" => Some(PipeTap),
                "Turbine Meter" => Some(TurbineMeter),
                "Valued at 90 Day Marketing Time Method" => {
                    Some(ValuedAt90DayMarketingTimeMethod)
                }
                "Valued at 120 Day Marketing Time Method" => {
                    Some(ValuedAt120DayMarketingTimeMethod)
                }
                "Valued at 180 Day Marketing Time Method" => {
                    Some(ValuedAt180DayMarketingTimeMethod)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for MeasurementMethodOrDeviceCode {
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
    type Value = MeasurementMethodOrDeviceCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Measurement Method or Device Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementMethodOrDeviceCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Measurement Method or Device Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementMethodOrDeviceCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Measurement Method or Device Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for MeasurementMethodOrDeviceCode {
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