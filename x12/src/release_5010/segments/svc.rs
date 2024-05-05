use serde::{Serialize, Deserialize};
use super::super::elements::CompositeMedicalProcedure;
/**To supply payment and control information to a provider for a particular service

See docs at <https://www.stedi.com/edi/hipaa/segment/SVC>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SVC")]
pub struct Service {
    /**SVC-01 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub composite_medical_procedure: CompositeMedicalProcedure,
    /**SVC-02 (782)
Monetary amount*/
    pub monetary_amount: String,
    /**SVC-03 (782)
Monetary amount*/
    pub svc03: Option<String>,
    /**SVC-04 (234)
Identifying number for a product or service*/
    pub product_service_id: Option<String>,
    /**SVC-05 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**SVC-06 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub svc06: Option<CompositeMedicalProcedure>,
    /**SVC-07 (380)
Numeric value of quantity*/
    pub svc07: Option<String>,
}
impl Service {}