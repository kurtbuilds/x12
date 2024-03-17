use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**594

See docs at <https://www.stedi.com/edi/x12/element/594>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrequencyCode {
    ///0 - Annualized; 12-month equivalent
    Code0,
    ///1 - Weekly
    Weekly,
    ///2 - Biweekly
    Biweekly,
    ///3 - Semimonthly
    Semimonthly,
    ///4 - Monthly
    Monthly,
    ///5 - Other
    Other,
    ///6 - Daily
    Daily,
    ///7 - Annual
    Annual,
    ///8 - Two Calendar Months
    TwoCalendarMonths,
    ///9 - Lump-Sum Separation Allowance
    LumpSumSeparationAllowance,
    ///A - Quarter-to-Date
    QuarterToDate,
    ///B - Year-to-Date
    YearToDate,
    ///C - Single
    Single,
    ///D - Policy Period
    PolicyPeriod,
    ///E - Claim Period
    ClaimPeriod,
    ///F - Unit Report Identifier
    UnitReport,
    ///G - Month-to-Date
    MonthToDate,
    ///H - Hourly
    Hourly,
    ///J - Current Period
    CurrentPeriod,
    ///Q - Quarterly
    Quarterly,
    ///S - Semiannual
    Semiannual,
    ///U - Unknown
    Unknown,
    ///Z - Mutually Defined
    MutuallyDefined,
}
impl FrequencyCode {
    pub fn code(&self) -> &str {
        {
            use FrequencyCode::*;
            match self {
                Code0 => "0",
                Weekly => "1",
                Biweekly => "2",
                Semimonthly => "3",
                Monthly => "4",
                Other => "5",
                Daily => "6",
                Annual => "7",
                TwoCalendarMonths => "8",
                LumpSumSeparationAllowance => "9",
                QuarterToDate => "A",
                YearToDate => "B",
                Single => "C",
                PolicyPeriod => "D",
                ClaimPeriod => "E",
                UnitReport => "F",
                MonthToDate => "G",
                Hourly => "H",
                CurrentPeriod => "J",
                Quarterly => "Q",
                Semiannual => "S",
                Unknown => "U",
                MutuallyDefined => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<FrequencyCode> {
        use FrequencyCode::*;
        match code {
            b"0" => Some(Code0),
            b"1" => Some(Weekly),
            b"2" => Some(Biweekly),
            b"3" => Some(Semimonthly),
            b"4" => Some(Monthly),
            b"5" => Some(Other),
            b"6" => Some(Daily),
            b"7" => Some(Annual),
            b"8" => Some(TwoCalendarMonths),
            b"9" => Some(LumpSumSeparationAllowance),
            b"A" => Some(QuarterToDate),
            b"B" => Some(YearToDate),
            b"C" => Some(Single),
            b"D" => Some(PolicyPeriod),
            b"E" => Some(ClaimPeriod),
            b"F" => Some(UnitReport),
            b"G" => Some(MonthToDate),
            b"H" => Some(Hourly),
            b"J" => Some(CurrentPeriod),
            b"Q" => Some(Quarterly),
            b"S" => Some(Semiannual),
            b"U" => Some(Unknown),
            b"Z" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use FrequencyCode::*;
        match self {
            Code0 => "Annualized; 12-month equivalent",
            Weekly => "Weekly",
            Biweekly => "Biweekly",
            Semimonthly => "Semimonthly",
            Monthly => "Monthly",
            Other => "Other",
            Daily => "Daily",
            Annual => "Annual",
            TwoCalendarMonths => "Two Calendar Months",
            LumpSumSeparationAllowance => "Lump-Sum Separation Allowance",
            QuarterToDate => "Quarter-to-Date",
            YearToDate => "Year-to-Date",
            Single => "Single",
            PolicyPeriod => "Policy Period",
            ClaimPeriod => "Claim Period",
            UnitReport => "Unit Report Identifier",
            MonthToDate => "Month-to-Date",
            Hourly => "Hourly",
            CurrentPeriod => "Current Period",
            Quarterly => "Quarterly",
            Semiannual => "Semiannual",
            Unknown => "Unknown",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<FrequencyCode> {
        {
            use FrequencyCode::*;
            match description {
                "Annualized; 12-month equivalent" => Some(Code0),
                "Weekly" => Some(Weekly),
                "Biweekly" => Some(Biweekly),
                "Semimonthly" => Some(Semimonthly),
                "Monthly" => Some(Monthly),
                "Other" => Some(Other),
                "Daily" => Some(Daily),
                "Annual" => Some(Annual),
                "Two Calendar Months" => Some(TwoCalendarMonths),
                "Lump-Sum Separation Allowance" => Some(LumpSumSeparationAllowance),
                "Quarter-to-Date" => Some(QuarterToDate),
                "Year-to-Date" => Some(YearToDate),
                "Single" => Some(Single),
                "Policy Period" => Some(PolicyPeriod),
                "Claim Period" => Some(ClaimPeriod),
                "Unit Report Identifier" => Some(UnitReport),
                "Month-to-Date" => Some(MonthToDate),
                "Hourly" => Some(Hourly),
                "Current Period" => Some(CurrentPeriod),
                "Quarterly" => Some(Quarterly),
                "Semiannual" => Some(Semiannual),
                "Unknown" => Some(Unknown),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for FrequencyCode {
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
    type Value = FrequencyCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Frequency Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        FrequencyCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Frequency Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        FrequencyCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Frequency Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for FrequencyCode {
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