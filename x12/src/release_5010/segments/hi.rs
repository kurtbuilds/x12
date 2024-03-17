use serde::{Serialize, Deserialize};
use super::super::elements::HealthCareCode;
/**To supply information related to the delivery of health care

See docs at <https://www.stedi.com/edi/x12/segment/HI>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "HI")]
pub struct HealthCareInformationCodes {
    /**HI-01 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub health_care_code: HealthCareCode,
    /**HI-02 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi02: Option<HealthCareCode>,
    /**HI-03 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi03: Option<HealthCareCode>,
    /**HI-04 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi04: Option<HealthCareCode>,
    /**HI-05 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi05: Option<HealthCareCode>,
    /**HI-06 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi06: Option<HealthCareCode>,
    /**HI-07 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi07: Option<HealthCareCode>,
    /**HI-08 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi08: Option<HealthCareCode>,
    /**HI-09 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi09: Option<HealthCareCode>,
    /**HI-10 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi10: Option<HealthCareCode>,
    /**HI-11 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi11: Option<HealthCareCode>,
    /**HI-12 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub hi12: Option<HealthCareCode>,
}
impl HealthCareInformationCodes {}