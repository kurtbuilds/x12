use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify the currency (dollars, pounds, francs, etc.) used in a transaction

See docs at <https://www.stedi.com/edi/x12-005010/segment/CUR>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CUR")]
pub struct Currency {
    /**CUR-01 (98)
Code identifying an organizational entity, a physical location, property or an individual*/
    pub entity_identifier_code: String,
    /**CUR-02 (100)
Code (Standard ISO) for country in whose currency the charges are specified*/
    pub currency_code: Fixed<3>,
    /**CUR-03 (280)
Value to be used as a multiplier conversion factor to convert monetary value from one currency to another*/
    pub exchange_rate: Option<String>,
    /**CUR-04 (98)
Code identifying an organizational entity, a physical location, property or an individual*/
    pub cur04: Option<String>,
    /**CUR-05 (100)
Code (Standard ISO) for country in whose currency the charges are specified*/
    pub cur05: Option<Fixed<3>>,
    /**CUR-06 (669)
Code identifying the market upon which the currency exchange rate is based*/
    pub currency_market_exchange_code: Option<Fixed<3>>,
    /**CUR-07 (374)
Code specifying type of date or time, or both date and time*/
    pub date_time_qualifier: Option<Fixed<3>>,
    /**CUR-08 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub date: Option<Fixed<8>>,
    /**CUR-09 (337)
Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)*/
    pub time: Option<String>,
    /**CUR-10 (374)
Code specifying type of date or time, or both date and time*/
    pub cur10: Option<Fixed<3>>,
    /**CUR-11 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cur11: Option<Fixed<8>>,
    /**CUR-12 (337)
Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)*/
    pub cur12: Option<String>,
    /**CUR-13 (374)
Code specifying type of date or time, or both date and time*/
    pub cur13: Option<Fixed<3>>,
    /**CUR-14 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cur14: Option<Fixed<8>>,
    /**CUR-15 (337)
Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)*/
    pub cur15: Option<String>,
    /**CUR-16 (374)
Code specifying type of date or time, or both date and time*/
    pub cur16: Option<Fixed<3>>,
    /**CUR-17 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cur17: Option<Fixed<8>>,
    /**CUR-18 (337)
Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)*/
    pub cur18: Option<String>,
    /**CUR-19 (374)
Code specifying type of date or time, or both date and time*/
    pub cur19: Option<Fixed<3>>,
    /**CUR-20 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cur20: Option<Fixed<8>>,
    /**CUR-21 (337)
Time expressed in 24-hour clock time as follows: HHMM, or HHMMSS, or HHMMSSD, or HHMMSSDD, where H = hours (00-23), M = minutes (00-59), S = integer seconds (00-59) and DD = decimal seconds; decimal seconds are expressed as follows: D = tenths (0-9) and DD = hundredths (00-99)*/
    pub cur21: Option<String>,
}
impl Currency {}