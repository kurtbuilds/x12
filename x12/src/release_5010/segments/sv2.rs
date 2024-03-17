use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
use super::super::elements::CompositeMedicalProcedure;
/**To specify the service line item detail for a health care institution

See docs at <https://www.stedi.com/edi/x12/segment/SV2>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SV2")]
pub struct InstitutionalService {
    /**SV2-01 (234)
Identifying number for a product or service*/
    pub product_service_id: Option<String>,
    /**SV2-02 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub composite_medical_procedure: Option<CompositeMedicalProcedure>,
    /**SV2-03 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**SV2-04 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**SV2-05 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**SV2-06 (1371)
The rate per unit of associate revenue for hospital accommodation*/
    pub unit_rate: Option<String>,
    /**SV2-07 (782)
Monetary amount*/
    pub sv2_07: Option<String>,
    /**SV2-08 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**SV2-09 (1345)
Code specifying the status of a nursing home resident at the time of service*/
    pub nursing_home_residential_status_code: Option<Fixed<1>>,
    /**SV2-10 (1337)
Code specifying the level of care provided by a nursing home facility*/
    pub level_of_care_code: Option<Fixed<1>>,
}
impl InstitutionalService {}