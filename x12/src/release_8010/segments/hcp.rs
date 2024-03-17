use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify pricing or repricing information about a health care claim or line item

See docs at <https://www.stedi.com/edi/x12/segment/HCP>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "HCP")]
pub struct HealthCarePricing {
    /**HCP-01 (1473)
Code specifying pricing methodology at which the claim or line item has been priced or repriced*/
    pub pricing_methodology_code: Option<Fixed<2>>,
    /**HCP-02 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**HCP-03 (782)
Monetary amount*/
    pub hcp03: Option<String>,
    /**HCP-04 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**HCP-05 (118)
Rate expressed in the standard monetary denomination for the currency specified*/
    pub rate: Option<String>,
    /**HCP-06 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub hcp06: Option<String>,
    /**HCP-07 (782)
Monetary amount*/
    pub hcp07: Option<String>,
    /**HCP-08 (234)
Identifying number for a product or service*/
    pub product_service_id: Option<String>,
    /**HCP-09 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub product_service_id_qualifier: Option<Fixed<2>>,
    /**HCP-10 (234)
Identifying number for a product or service*/
    pub hcp10: Option<String>,
    /**HCP-11 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**HCP-12 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**HCP-13 (901)
Code identifying reason for rejection as assigned by issuer*/
    pub reject_reason_code: Option<Fixed<2>>,
    /**HCP-14 (1526)
Code specifying policy compliance*/
    pub policy_compliance_code: Option<String>,
    /**HCP-15 (1527)
Code specifying the exception reason for consideration of out-of-network health care services*/
    pub exception_code: Option<String>,
}
impl HealthCarePricing {}