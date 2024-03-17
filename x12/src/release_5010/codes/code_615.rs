use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**615

See docs at <https://www.stedi.com/edi/x12/element/615>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimePeriodQualifier {
    ///1 - Chargeable Periods
    ChargeablePeriods,
    ///2 - Periods Held
    PeriodsHeld,
    ///3 - Free Periods
    FreePeriods,
    ///4 - Saturdays, Sundays and Holidays
    Code4,
    ///5 - Other Allowance Periods
    OtherAllowancePeriods,
    ///6 - Hour
    Hour,
    ///7 - Day
    Day,
    ///8 - Not Applicable
    NotApplicable,
    ///9 - Initial Visit
    InitialVisit,
    ///10 - Six Hours
    SixHours,
    ///11 - 12 Hours
    Code11,
    ///12 - 18 Hours
    Code12,
    ///13 - 24 Hours
    Code13,
    ///14 - Debit Days
    DebitDays,
    ///15 - Credit Days
    CreditDays,
    ///16 - Excess Days
    ExcessDays,
    ///17 - Hazardous Days
    HazardousDays,
    ///18 - Holidays
    Holidays,
    ///19 - Saturdays and Sundays
    SaturdaysAndSundays,
    ///20 - Sundays and Holidays
    SundaysAndHolidays,
    ///21 - Years
    Years,
    ///22 - Service Year
    ServiceYear,
    ///23 - Calendar Year
    CalendarYear,
    ///24 - Year to Date
    YearToDate,
    ///25 - Contract
    Contract,
    ///26 - Episode
    Episode,
    ///27 - Visit
    Visit,
    ///28 - Outlier
    Outlier,
    ///29 - Remaining
    Remaining,
    ///30 - Exceeded
    Exceeded,
    ///31 - Not Exceeded
    NotExceeded,
    ///32 - Lifetime
    Lifetime,
    ///33 - Lifetime Remaining
    LifetimeRemaining,
    ///34 - Month
    Month,
    ///35 - Week
    Week,
    ///36 - Admission
    Admission,
    ///37 - Three Months
    ThreeMonths,
    ///Z - Mutually Defined
    MutuallyDefined,
}
impl TimePeriodQualifier {
    pub fn code(&self) -> &str {
        {
            use TimePeriodQualifier::*;
            match self {
                ChargeablePeriods => "1",
                PeriodsHeld => "2",
                FreePeriods => "3",
                Code4 => "4",
                OtherAllowancePeriods => "5",
                Hour => "6",
                Day => "7",
                NotApplicable => "8",
                InitialVisit => "9",
                SixHours => "10",
                Code11 => "11",
                Code12 => "12",
                Code13 => "13",
                DebitDays => "14",
                CreditDays => "15",
                ExcessDays => "16",
                HazardousDays => "17",
                Holidays => "18",
                SaturdaysAndSundays => "19",
                SundaysAndHolidays => "20",
                Years => "21",
                ServiceYear => "22",
                CalendarYear => "23",
                YearToDate => "24",
                Contract => "25",
                Episode => "26",
                Visit => "27",
                Outlier => "28",
                Remaining => "29",
                Exceeded => "30",
                NotExceeded => "31",
                Lifetime => "32",
                LifetimeRemaining => "33",
                Month => "34",
                Week => "35",
                Admission => "36",
                ThreeMonths => "37",
                MutuallyDefined => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<TimePeriodQualifier> {
        use TimePeriodQualifier::*;
        match code {
            b"1" => Some(ChargeablePeriods),
            b"2" => Some(PeriodsHeld),
            b"3" => Some(FreePeriods),
            b"4" => Some(Code4),
            b"5" => Some(OtherAllowancePeriods),
            b"6" => Some(Hour),
            b"7" => Some(Day),
            b"8" => Some(NotApplicable),
            b"9" => Some(InitialVisit),
            b"10" => Some(SixHours),
            b"11" => Some(Code11),
            b"12" => Some(Code12),
            b"13" => Some(Code13),
            b"14" => Some(DebitDays),
            b"15" => Some(CreditDays),
            b"16" => Some(ExcessDays),
            b"17" => Some(HazardousDays),
            b"18" => Some(Holidays),
            b"19" => Some(SaturdaysAndSundays),
            b"20" => Some(SundaysAndHolidays),
            b"21" => Some(Years),
            b"22" => Some(ServiceYear),
            b"23" => Some(CalendarYear),
            b"24" => Some(YearToDate),
            b"25" => Some(Contract),
            b"26" => Some(Episode),
            b"27" => Some(Visit),
            b"28" => Some(Outlier),
            b"29" => Some(Remaining),
            b"30" => Some(Exceeded),
            b"31" => Some(NotExceeded),
            b"32" => Some(Lifetime),
            b"33" => Some(LifetimeRemaining),
            b"34" => Some(Month),
            b"35" => Some(Week),
            b"36" => Some(Admission),
            b"37" => Some(ThreeMonths),
            b"Z" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use TimePeriodQualifier::*;
        match self {
            ChargeablePeriods => "Chargeable Periods",
            PeriodsHeld => "Periods Held",
            FreePeriods => "Free Periods",
            Code4 => "Saturdays, Sundays and Holidays",
            OtherAllowancePeriods => "Other Allowance Periods",
            Hour => "Hour",
            Day => "Day",
            NotApplicable => "Not Applicable",
            InitialVisit => "Initial Visit",
            SixHours => "Six Hours",
            Code11 => "12 Hours",
            Code12 => "18 Hours",
            Code13 => "24 Hours",
            DebitDays => "Debit Days",
            CreditDays => "Credit Days",
            ExcessDays => "Excess Days",
            HazardousDays => "Hazardous Days",
            Holidays => "Holidays",
            SaturdaysAndSundays => "Saturdays and Sundays",
            SundaysAndHolidays => "Sundays and Holidays",
            Years => "Years",
            ServiceYear => "Service Year",
            CalendarYear => "Calendar Year",
            YearToDate => "Year to Date",
            Contract => "Contract",
            Episode => "Episode",
            Visit => "Visit",
            Outlier => "Outlier",
            Remaining => "Remaining",
            Exceeded => "Exceeded",
            NotExceeded => "Not Exceeded",
            Lifetime => "Lifetime",
            LifetimeRemaining => "Lifetime Remaining",
            Month => "Month",
            Week => "Week",
            Admission => "Admission",
            ThreeMonths => "Three Months",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<TimePeriodQualifier> {
        {
            use TimePeriodQualifier::*;
            match description {
                "Chargeable Periods" => Some(ChargeablePeriods),
                "Periods Held" => Some(PeriodsHeld),
                "Free Periods" => Some(FreePeriods),
                "Saturdays, Sundays and Holidays" => Some(Code4),
                "Other Allowance Periods" => Some(OtherAllowancePeriods),
                "Hour" => Some(Hour),
                "Day" => Some(Day),
                "Not Applicable" => Some(NotApplicable),
                "Initial Visit" => Some(InitialVisit),
                "Six Hours" => Some(SixHours),
                "12 Hours" => Some(Code11),
                "18 Hours" => Some(Code12),
                "24 Hours" => Some(Code13),
                "Debit Days" => Some(DebitDays),
                "Credit Days" => Some(CreditDays),
                "Excess Days" => Some(ExcessDays),
                "Hazardous Days" => Some(HazardousDays),
                "Holidays" => Some(Holidays),
                "Saturdays and Sundays" => Some(SaturdaysAndSundays),
                "Sundays and Holidays" => Some(SundaysAndHolidays),
                "Years" => Some(Years),
                "Service Year" => Some(ServiceYear),
                "Calendar Year" => Some(CalendarYear),
                "Year to Date" => Some(YearToDate),
                "Contract" => Some(Contract),
                "Episode" => Some(Episode),
                "Visit" => Some(Visit),
                "Outlier" => Some(Outlier),
                "Remaining" => Some(Remaining),
                "Exceeded" => Some(Exceeded),
                "Not Exceeded" => Some(NotExceeded),
                "Lifetime" => Some(Lifetime),
                "Lifetime Remaining" => Some(LifetimeRemaining),
                "Month" => Some(Month),
                "Week" => Some(Week),
                "Admission" => Some(Admission),
                "Three Months" => Some(ThreeMonths),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for TimePeriodQualifier {
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
    type Value = TimePeriodQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Time Period Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TimePeriodQualifier::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Time Period Qualifier: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TimePeriodQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Time Period Qualifier: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for TimePeriodQualifier {
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