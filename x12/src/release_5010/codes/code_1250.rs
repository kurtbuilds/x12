use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1250

See docs at <https://www.stedi.com/edi/x12/element/1250>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateTimePeriodFormatQualifier {
    ///CC - First Two Digits of Year Expressed in Format CCYY
    FirstTwoDigitsOfYearExpressedInFormatCcyy,
    ///CD - Month and Year Expressed in Format MMMYYYY
    MonthAndYearExpressedInFormatMmmyyyy,
    ///CM - Date in Format CCYYMM
    DateInFormatCcyymm,
    ///CQ - Date in Format CCYYQ
    DateInFormatCcyyq,
    ///CY - Year Expressed in Format CCYY
    YearExpressedInFormatCcyy,
    ///D6 - Date Expressed in Format YYMMDD
    DateExpressedInFormatYymmdd,
    ///D8 - Date Expressed in Format CCYYMMDD
    DateExpressedInFormatCcyymmdd,
    ///DA - Range of Dates within a Single Month Expressed in Format DD-DD
    RangeOfDatesWithinASingleMonthExpressedInFormatDdDd,
    ///DB - Date Expressed in Format MMDDCCYY
    DateExpressedInFormatMmddccyy,
    ///DD - Day of Month in Numeric Format
    DayOfMonthInNumericFormat,
    ///DDT - Range of Dates and Time, Expressed in CCYYMMDD-CCYYMMDDHHMM
    CodeDDT,
    ///DT - Date and Time Expressed in Format CCYYMMDDHHMM
    DateAndTimeExpressedInFormatCcyymmddhhmm,
    ///DTD - Range of Dates and Time, Expressed in CCYYMMDDHHMM-CCYYMMDD
    CodeDTD,
    ///DTS - Range of Date and Time Expressed in Format CCYYMMDDHHMMSS-CCYYMMDDHHMMSS
    RangeOfDateAndTimeExpressedInFormatCcyymmddhhmmssCcyymmddhhmmss,
    ///EH - Last Digit of Year and Julian Date Expressed in Format YDDD
    LastDigitOfYearAndJulianDateExpressedInFormatYddd,
    ///KA - Date Expressed in Format YYMMMDD
    DateExpressedInFormatYymmmdd,
    ///MCY - MMCCYY
    Mmccyy,
    ///MD - Month of Year and Day of Month Expressed in Format MMDD
    MonthOfYearAndDayOfMonthExpressedInFormatMmdd,
    ///MM - Month of Year in Numeric Format
    MonthOfYearInNumericFormat,
    ///RD - Range of Dates Expressed in Format MMDDCCYY-MMDDCCYY
    RangeOfDatesExpressedInFormatMmddccyyMmddccyy,
    ///RD2 - Range of Years Expressed in Format YY-YY
    RangeOfYearsExpressedInFormatYyYy,
    ///RD4 - Range of Years Expressed in Format CCYY-CCYY
    RangeOfYearsExpressedInFormatCcyyCcyy,
    ///RD5 - Range of Years and Months Expressed in Format CCYYMM-CCYYMM
    RangeOfYearsAndMonthsExpressedInFormatCcyymmCcyymm,
    ///RD6 - Range of Dates Expressed in Format YYMMDD-YYMMDD
    RangeOfDatesExpressedInFormatYymmddYymmdd,
    ///RD8 - Range of Dates Expressed in Format CCYYMMDD-CCYYMMDD
    RangeOfDatesExpressedInFormatCcyymmddCcyymmdd,
    ///RDM - Range of Dates Expressed in Format YYMMDD-MMDD
    RangeOfDatesExpressedInFormatYymmddMmdd,
    ///RDT - Range of Date and Time, Expressed in Format CCYYMMDDHHMM-CCYYMMDDHHMM
    CodeRDT,
    ///RMD - Range of Months and Days Expressed in Format MMDD-MMDD
    RangeOfMonthsAndDaysExpressedInFormatMmddMmdd,
    ///RMY - Range of Years and Months Expressed in Format YYMM-YYMM
    RangeOfYearsAndMonthsExpressedInFormatYymmYymm,
    ///RTM - Range of Time Expressed in Format HHMM-HHMM
    RangeOfTimeExpressedInFormatHhmmHhmm,
    ///RTS - Date and Time Expressed in Format CCYYMMDDHHMMSS
    DateAndTimeExpressedInFormatCcyymmddhhmmss,
    ///TC - Julian Date Expressed in Format DDD
    JulianDateExpressedInFormatDdd,
    ///TM - Time Expressed in Format HHMM
    TimeExpressedInFormatHhmm,
    ///TQ - Date Expressed in Format MMYY
    DateExpressedInFormatMmyy,
    ///TR - Date and Time Expressed in Format DDMMYYHHMM
    DateAndTimeExpressedInFormatDdmmyyhhmm,
    ///TS - Time Expressed in Format HHMMSS
    TimeExpressedInFormatHhmmss,
    ///TT - Date Expressed in Format MMDDYY
    DateExpressedInFormatMmddyy,
    ///TU - Date Expressed in Format YYDDD
    DateExpressedInFormatYyddd,
    ///UN - Unstructured
    Unstructured,
    ///YM - Year and Month Expressed in Format YYMM
    YearAndMonthExpressedInFormatYymm,
    ///YMM - Range of Year and Months, Expressed in CCYYMMM-MMM Format
    CodeYMM,
    ///YY - Last Two Digits of Year Expressed in Format CCYY
    LastTwoDigitsOfYearExpressedInFormatCcyy,
}
impl DateTimePeriodFormatQualifier {
    pub fn code(&self) -> &str {
        {
            use DateTimePeriodFormatQualifier::*;
            match self {
                FirstTwoDigitsOfYearExpressedInFormatCcyy => "CC",
                MonthAndYearExpressedInFormatMmmyyyy => "CD",
                DateInFormatCcyymm => "CM",
                DateInFormatCcyyq => "CQ",
                YearExpressedInFormatCcyy => "CY",
                DateExpressedInFormatYymmdd => "D6",
                DateExpressedInFormatCcyymmdd => "D8",
                RangeOfDatesWithinASingleMonthExpressedInFormatDdDd => "DA",
                DateExpressedInFormatMmddccyy => "DB",
                DayOfMonthInNumericFormat => "DD",
                CodeDDT => "DDT",
                DateAndTimeExpressedInFormatCcyymmddhhmm => "DT",
                CodeDTD => "DTD",
                RangeOfDateAndTimeExpressedInFormatCcyymmddhhmmssCcyymmddhhmmss => "DTS",
                LastDigitOfYearAndJulianDateExpressedInFormatYddd => "EH",
                DateExpressedInFormatYymmmdd => "KA",
                Mmccyy => "MCY",
                MonthOfYearAndDayOfMonthExpressedInFormatMmdd => "MD",
                MonthOfYearInNumericFormat => "MM",
                RangeOfDatesExpressedInFormatMmddccyyMmddccyy => "RD",
                RangeOfYearsExpressedInFormatYyYy => "RD2",
                RangeOfYearsExpressedInFormatCcyyCcyy => "RD4",
                RangeOfYearsAndMonthsExpressedInFormatCcyymmCcyymm => "RD5",
                RangeOfDatesExpressedInFormatYymmddYymmdd => "RD6",
                RangeOfDatesExpressedInFormatCcyymmddCcyymmdd => "RD8",
                RangeOfDatesExpressedInFormatYymmddMmdd => "RDM",
                CodeRDT => "RDT",
                RangeOfMonthsAndDaysExpressedInFormatMmddMmdd => "RMD",
                RangeOfYearsAndMonthsExpressedInFormatYymmYymm => "RMY",
                RangeOfTimeExpressedInFormatHhmmHhmm => "RTM",
                DateAndTimeExpressedInFormatCcyymmddhhmmss => "RTS",
                JulianDateExpressedInFormatDdd => "TC",
                TimeExpressedInFormatHhmm => "TM",
                DateExpressedInFormatMmyy => "TQ",
                DateAndTimeExpressedInFormatDdmmyyhhmm => "TR",
                TimeExpressedInFormatHhmmss => "TS",
                DateExpressedInFormatMmddyy => "TT",
                DateExpressedInFormatYyddd => "TU",
                Unstructured => "UN",
                YearAndMonthExpressedInFormatYymm => "YM",
                CodeYMM => "YMM",
                LastTwoDigitsOfYearExpressedInFormatCcyy => "YY",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<DateTimePeriodFormatQualifier> {
        use DateTimePeriodFormatQualifier::*;
        match code {
            b"CC" => Some(FirstTwoDigitsOfYearExpressedInFormatCcyy),
            b"CD" => Some(MonthAndYearExpressedInFormatMmmyyyy),
            b"CM" => Some(DateInFormatCcyymm),
            b"CQ" => Some(DateInFormatCcyyq),
            b"CY" => Some(YearExpressedInFormatCcyy),
            b"D6" => Some(DateExpressedInFormatYymmdd),
            b"D8" => Some(DateExpressedInFormatCcyymmdd),
            b"DA" => Some(RangeOfDatesWithinASingleMonthExpressedInFormatDdDd),
            b"DB" => Some(DateExpressedInFormatMmddccyy),
            b"DD" => Some(DayOfMonthInNumericFormat),
            b"DDT" => Some(CodeDDT),
            b"DT" => Some(DateAndTimeExpressedInFormatCcyymmddhhmm),
            b"DTD" => Some(CodeDTD),
            b"DTS" => {
                Some(RangeOfDateAndTimeExpressedInFormatCcyymmddhhmmssCcyymmddhhmmss)
            }
            b"EH" => Some(LastDigitOfYearAndJulianDateExpressedInFormatYddd),
            b"KA" => Some(DateExpressedInFormatYymmmdd),
            b"MCY" => Some(Mmccyy),
            b"MD" => Some(MonthOfYearAndDayOfMonthExpressedInFormatMmdd),
            b"MM" => Some(MonthOfYearInNumericFormat),
            b"RD" => Some(RangeOfDatesExpressedInFormatMmddccyyMmddccyy),
            b"RD2" => Some(RangeOfYearsExpressedInFormatYyYy),
            b"RD4" => Some(RangeOfYearsExpressedInFormatCcyyCcyy),
            b"RD5" => Some(RangeOfYearsAndMonthsExpressedInFormatCcyymmCcyymm),
            b"RD6" => Some(RangeOfDatesExpressedInFormatYymmddYymmdd),
            b"RD8" => Some(RangeOfDatesExpressedInFormatCcyymmddCcyymmdd),
            b"RDM" => Some(RangeOfDatesExpressedInFormatYymmddMmdd),
            b"RDT" => Some(CodeRDT),
            b"RMD" => Some(RangeOfMonthsAndDaysExpressedInFormatMmddMmdd),
            b"RMY" => Some(RangeOfYearsAndMonthsExpressedInFormatYymmYymm),
            b"RTM" => Some(RangeOfTimeExpressedInFormatHhmmHhmm),
            b"RTS" => Some(DateAndTimeExpressedInFormatCcyymmddhhmmss),
            b"TC" => Some(JulianDateExpressedInFormatDdd),
            b"TM" => Some(TimeExpressedInFormatHhmm),
            b"TQ" => Some(DateExpressedInFormatMmyy),
            b"TR" => Some(DateAndTimeExpressedInFormatDdmmyyhhmm),
            b"TS" => Some(TimeExpressedInFormatHhmmss),
            b"TT" => Some(DateExpressedInFormatMmddyy),
            b"TU" => Some(DateExpressedInFormatYyddd),
            b"UN" => Some(Unstructured),
            b"YM" => Some(YearAndMonthExpressedInFormatYymm),
            b"YMM" => Some(CodeYMM),
            b"YY" => Some(LastTwoDigitsOfYearExpressedInFormatCcyy),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use DateTimePeriodFormatQualifier::*;
        match self {
            FirstTwoDigitsOfYearExpressedInFormatCcyy => {
                "First Two Digits of Year Expressed in Format CCYY"
            }
            MonthAndYearExpressedInFormatMmmyyyy => {
                "Month and Year Expressed in Format MMMYYYY"
            }
            DateInFormatCcyymm => "Date in Format CCYYMM",
            DateInFormatCcyyq => "Date in Format CCYYQ",
            YearExpressedInFormatCcyy => "Year Expressed in Format CCYY",
            DateExpressedInFormatYymmdd => "Date Expressed in Format YYMMDD",
            DateExpressedInFormatCcyymmdd => "Date Expressed in Format CCYYMMDD",
            RangeOfDatesWithinASingleMonthExpressedInFormatDdDd => {
                "Range of Dates within a Single Month Expressed in Format DD-DD"
            }
            DateExpressedInFormatMmddccyy => "Date Expressed in Format MMDDCCYY",
            DayOfMonthInNumericFormat => "Day of Month in Numeric Format",
            CodeDDT => "Range of Dates and Time, Expressed in CCYYMMDD-CCYYMMDDHHMM",
            DateAndTimeExpressedInFormatCcyymmddhhmm => {
                "Date and Time Expressed in Format CCYYMMDDHHMM"
            }
            CodeDTD => "Range of Dates and Time, Expressed in CCYYMMDDHHMM-CCYYMMDD",
            RangeOfDateAndTimeExpressedInFormatCcyymmddhhmmssCcyymmddhhmmss => {
                "Range of Date and Time Expressed in Format CCYYMMDDHHMMSS-CCYYMMDDHHMMSS"
            }
            LastDigitOfYearAndJulianDateExpressedInFormatYddd => {
                "Last Digit of Year and Julian Date Expressed in Format YDDD"
            }
            DateExpressedInFormatYymmmdd => "Date Expressed in Format YYMMMDD",
            Mmccyy => "MMCCYY",
            MonthOfYearAndDayOfMonthExpressedInFormatMmdd => {
                "Month of Year and Day of Month Expressed in Format MMDD"
            }
            MonthOfYearInNumericFormat => "Month of Year in Numeric Format",
            RangeOfDatesExpressedInFormatMmddccyyMmddccyy => {
                "Range of Dates Expressed in Format MMDDCCYY-MMDDCCYY"
            }
            RangeOfYearsExpressedInFormatYyYy => {
                "Range of Years Expressed in Format YY-YY"
            }
            RangeOfYearsExpressedInFormatCcyyCcyy => {
                "Range of Years Expressed in Format CCYY-CCYY"
            }
            RangeOfYearsAndMonthsExpressedInFormatCcyymmCcyymm => {
                "Range of Years and Months Expressed in Format CCYYMM-CCYYMM"
            }
            RangeOfDatesExpressedInFormatYymmddYymmdd => {
                "Range of Dates Expressed in Format YYMMDD-YYMMDD"
            }
            RangeOfDatesExpressedInFormatCcyymmddCcyymmdd => {
                "Range of Dates Expressed in Format CCYYMMDD-CCYYMMDD"
            }
            RangeOfDatesExpressedInFormatYymmddMmdd => {
                "Range of Dates Expressed in Format YYMMDD-MMDD"
            }
            CodeRDT => {
                "Range of Date and Time, Expressed in Format CCYYMMDDHHMM-CCYYMMDDHHMM"
            }
            RangeOfMonthsAndDaysExpressedInFormatMmddMmdd => {
                "Range of Months and Days Expressed in Format MMDD-MMDD"
            }
            RangeOfYearsAndMonthsExpressedInFormatYymmYymm => {
                "Range of Years and Months Expressed in Format YYMM-YYMM"
            }
            RangeOfTimeExpressedInFormatHhmmHhmm => {
                "Range of Time Expressed in Format HHMM-HHMM"
            }
            DateAndTimeExpressedInFormatCcyymmddhhmmss => {
                "Date and Time Expressed in Format CCYYMMDDHHMMSS"
            }
            JulianDateExpressedInFormatDdd => "Julian Date Expressed in Format DDD",
            TimeExpressedInFormatHhmm => "Time Expressed in Format HHMM",
            DateExpressedInFormatMmyy => "Date Expressed in Format MMYY",
            DateAndTimeExpressedInFormatDdmmyyhhmm => {
                "Date and Time Expressed in Format DDMMYYHHMM"
            }
            TimeExpressedInFormatHhmmss => "Time Expressed in Format HHMMSS",
            DateExpressedInFormatMmddyy => "Date Expressed in Format MMDDYY",
            DateExpressedInFormatYyddd => "Date Expressed in Format YYDDD",
            Unstructured => "Unstructured",
            YearAndMonthExpressedInFormatYymm => {
                "Year and Month Expressed in Format YYMM"
            }
            CodeYMM => "Range of Year and Months, Expressed in CCYYMMM-MMM Format",
            LastTwoDigitsOfYearExpressedInFormatCcyy => {
                "Last Two Digits of Year Expressed in Format CCYY"
            }
        }
    }
    fn from_description(description: &str) -> Option<DateTimePeriodFormatQualifier> {
        {
            use DateTimePeriodFormatQualifier::*;
            match description {
                "First Two Digits of Year Expressed in Format CCYY" => {
                    Some(FirstTwoDigitsOfYearExpressedInFormatCcyy)
                }
                "Month and Year Expressed in Format MMMYYYY" => {
                    Some(MonthAndYearExpressedInFormatMmmyyyy)
                }
                "Date in Format CCYYMM" => Some(DateInFormatCcyymm),
                "Date in Format CCYYQ" => Some(DateInFormatCcyyq),
                "Year Expressed in Format CCYY" => Some(YearExpressedInFormatCcyy),
                "Date Expressed in Format YYMMDD" => Some(DateExpressedInFormatYymmdd),
                "Date Expressed in Format CCYYMMDD" => {
                    Some(DateExpressedInFormatCcyymmdd)
                }
                "Range of Dates within a Single Month Expressed in Format DD-DD" => {
                    Some(RangeOfDatesWithinASingleMonthExpressedInFormatDdDd)
                }
                "Date Expressed in Format MMDDCCYY" => {
                    Some(DateExpressedInFormatMmddccyy)
                }
                "Day of Month in Numeric Format" => Some(DayOfMonthInNumericFormat),
                "Range of Dates and Time, Expressed in CCYYMMDD-CCYYMMDDHHMM" => {
                    Some(CodeDDT)
                }
                "Date and Time Expressed in Format CCYYMMDDHHMM" => {
                    Some(DateAndTimeExpressedInFormatCcyymmddhhmm)
                }
                "Range of Dates and Time, Expressed in CCYYMMDDHHMM-CCYYMMDD" => {
                    Some(CodeDTD)
                }
                "Range of Date and Time Expressed in Format CCYYMMDDHHMMSS-CCYYMMDDHHMMSS" => {
                    Some(RangeOfDateAndTimeExpressedInFormatCcyymmddhhmmssCcyymmddhhmmss)
                }
                "Last Digit of Year and Julian Date Expressed in Format YDDD" => {
                    Some(LastDigitOfYearAndJulianDateExpressedInFormatYddd)
                }
                "Date Expressed in Format YYMMMDD" => Some(DateExpressedInFormatYymmmdd),
                "MMCCYY" => Some(Mmccyy),
                "Month of Year and Day of Month Expressed in Format MMDD" => {
                    Some(MonthOfYearAndDayOfMonthExpressedInFormatMmdd)
                }
                "Month of Year in Numeric Format" => Some(MonthOfYearInNumericFormat),
                "Range of Dates Expressed in Format MMDDCCYY-MMDDCCYY" => {
                    Some(RangeOfDatesExpressedInFormatMmddccyyMmddccyy)
                }
                "Range of Years Expressed in Format YY-YY" => {
                    Some(RangeOfYearsExpressedInFormatYyYy)
                }
                "Range of Years Expressed in Format CCYY-CCYY" => {
                    Some(RangeOfYearsExpressedInFormatCcyyCcyy)
                }
                "Range of Years and Months Expressed in Format CCYYMM-CCYYMM" => {
                    Some(RangeOfYearsAndMonthsExpressedInFormatCcyymmCcyymm)
                }
                "Range of Dates Expressed in Format YYMMDD-YYMMDD" => {
                    Some(RangeOfDatesExpressedInFormatYymmddYymmdd)
                }
                "Range of Dates Expressed in Format CCYYMMDD-CCYYMMDD" => {
                    Some(RangeOfDatesExpressedInFormatCcyymmddCcyymmdd)
                }
                "Range of Dates Expressed in Format YYMMDD-MMDD" => {
                    Some(RangeOfDatesExpressedInFormatYymmddMmdd)
                }
                "Range of Date and Time, Expressed in Format CCYYMMDDHHMM-CCYYMMDDHHMM" => {
                    Some(CodeRDT)
                }
                "Range of Months and Days Expressed in Format MMDD-MMDD" => {
                    Some(RangeOfMonthsAndDaysExpressedInFormatMmddMmdd)
                }
                "Range of Years and Months Expressed in Format YYMM-YYMM" => {
                    Some(RangeOfYearsAndMonthsExpressedInFormatYymmYymm)
                }
                "Range of Time Expressed in Format HHMM-HHMM" => {
                    Some(RangeOfTimeExpressedInFormatHhmmHhmm)
                }
                "Date and Time Expressed in Format CCYYMMDDHHMMSS" => {
                    Some(DateAndTimeExpressedInFormatCcyymmddhhmmss)
                }
                "Julian Date Expressed in Format DDD" => {
                    Some(JulianDateExpressedInFormatDdd)
                }
                "Time Expressed in Format HHMM" => Some(TimeExpressedInFormatHhmm),
                "Date Expressed in Format MMYY" => Some(DateExpressedInFormatMmyy),
                "Date and Time Expressed in Format DDMMYYHHMM" => {
                    Some(DateAndTimeExpressedInFormatDdmmyyhhmm)
                }
                "Time Expressed in Format HHMMSS" => Some(TimeExpressedInFormatHhmmss),
                "Date Expressed in Format MMDDYY" => Some(DateExpressedInFormatMmddyy),
                "Date Expressed in Format YYDDD" => Some(DateExpressedInFormatYyddd),
                "Unstructured" => Some(Unstructured),
                "Year and Month Expressed in Format YYMM" => {
                    Some(YearAndMonthExpressedInFormatYymm)
                }
                "Range of Year and Months, Expressed in CCYYMMM-MMM Format" => {
                    Some(CodeYMM)
                }
                "Last Two Digits of Year Expressed in Format CCYY" => {
                    Some(LastTwoDigitsOfYearExpressedInFormatCcyy)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for DateTimePeriodFormatQualifier {
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
    type Value = DateTimePeriodFormatQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Date Time Period Format Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DateTimePeriodFormatQualifier::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Date Time Period Format Qualifier: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DateTimePeriodFormatQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Date Time Period Format Qualifier: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for DateTimePeriodFormatQualifier {
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