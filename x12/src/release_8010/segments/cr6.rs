use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information related to the certification of a home health care patient

See docs at <https://www.stedi.com/edi/x12/segment/CR6>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CR6")]
pub struct HomeHealthCareCertification {
    /**CR6-01 (923)
Code indicating physician's prognosis for the patient*/
    pub prognosis_code: Fixed<1>,
    /**CR6-02 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub date: Fixed<8>,
    /**CR6-03 (1250)
Code indicating the date format, time format, or date and time format*/
    pub date_time_period_format_qualifier: Option<String>,
    /**CR6-04 (1251)
Expression of a date, a time, or range of dates, times or dates and times*/
    pub date_time_period: Option<String>,
    /**CR6-05 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_05: Option<Fixed<8>>,
    /**CR6-06 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**CR6-07 (1073)
Code indicating a Yes or No condition or response*/
    pub cr6_07: Option<Fixed<1>>,
    /**CR6-08 (1322)
Code indicating the type of certification*/
    pub certification_type_code: Option<Fixed<1>>,
    /**CR6-09 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_09: Option<Fixed<8>>,
    /**CR6-10 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub product_service_id_qualifier: Option<Fixed<2>>,
    /**CR6-11 (1137)
Code value for describing a medical condition or procedure*/
    pub medical_code_value: Option<String>,
    /**CR6-12 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_12: Option<Fixed<8>>,
    /**CR6-13 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_13: Option<Fixed<8>>,
    /**CR6-14 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_14: Option<Fixed<8>>,
    /**CR6-15 (1250)
Code indicating the date format, time format, or date and time format*/
    pub cr6_15: Option<String>,
    /**CR6-16 (1251)
Expression of a date, a time, or range of dates, times or dates and times*/
    pub cr6_16: Option<String>,
    /**CR6-17 (1384)
Code identifying the location where patient is receiving medical treatment*/
    pub patient_location_code: Option<Fixed<1>>,
    /**CR6-18 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_18: Option<Fixed<8>>,
    /**CR6-19 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_19: Option<Fixed<8>>,
    /**CR6-20 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_20: Option<Fixed<8>>,
    /**CR6-21 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr6_21: Option<Fixed<8>>,
}
impl HomeHealthCareCertification {}