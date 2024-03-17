use serde::{Serialize, Deserialize};
use super::super::elements::CompositeMedicalProcedure;
use crate::numeric::Numeric;
/**To convey service line adjudication information for coordination of benefits between the initial payers of a health care claim and all subsequent payers

See docs at <https://www.stedi.com/edi/x12/segment/SVD>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SVD")]
pub struct ServiceLineAdjudication {
    /**SVD-01 (67)
Code identifying a party or other code*/
    pub identification_code: String,
    /**SVD-02 (782)
Monetary amount*/
    pub monetary_amount: String,
    /**SVD-03 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub composite_medical_procedure: Option<CompositeMedicalProcedure>,
    /**SVD-04 (234)
Identifying number for a product or service*/
    pub product_service_id: Option<String>,
    /**SVD-05 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**SVD-06 (554)
Number assigned for differentiation within a transaction set*/
    pub assigned_number: Option<Numeric<0>>,
}
impl ServiceLineAdjudication {}