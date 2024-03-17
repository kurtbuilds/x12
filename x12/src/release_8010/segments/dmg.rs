use serde::{Serialize, Deserialize};
use super::super::elements::CompositeRaceOrEthnicity;
use crate::fixed::Fixed;
/**To supply demographic information

See docs at <https://www.stedi.com/edi/x12/segment/DMG>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "DMG")]
pub struct Demographic {
    /**DMG-01 (1250)
Code indicating the date format, time format, or date and time format*/
    pub date_time_period_format_qualifier: Option<String>,
    /**DMG-02 (1251)
Expression of a date, a time, or range of dates, times or dates and times*/
    pub date_time_period: Option<String>,
    /**DMG-03 (1068)
Code indicating the sex of the individual*/
    pub gender_code: Option<Fixed<1>>,
    /**DMG-04 (1067)
Code specifying the marital status of a person*/
    pub marital_status_code: Option<Fixed<1>>,
    /**DMG-05 (C056)
To send general and detailed information on race or ethnicity*/
    pub composite_race_or_ethnicity: Vec<CompositeRaceOrEthnicity>,
    /**DMG-06 (1066)
Code indicating citizenship status*/
    pub citizenship_status_code: Option<String>,
    /**DMG-07 (26)
Code identifying the country*/
    pub country_code: Option<String>,
    /**DMG-08 (659)
Code indicating the basis of verification*/
    pub basis_of_verification_code: Option<String>,
    /**DMG-09 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**DMG-10 (1270)
Code identifying a specific industry code list*/
    pub code_list_qualifier_code: Option<String>,
    /**DMG-11 (1271)
Code indicating a code from a specific industry code list*/
    pub industry_code: Option<String>,
    /**DMG-12 (26)
Code identifying the country*/
    pub dmg12: Option<String>,
}
impl Demographic {}