use serde::{Serialize, Deserialize};
use super::super::elements::CompositeMedicalProcedure;
use crate::numeric::Numeric;
use crate::fixed::Fixed;
/**To specify the claim service detail for anesthesia

See docs at <https://www.stedi.com/edi/x12/segment/SV6>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SV6")]
pub struct AnesthesiaService {
    /**SV6-01 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub composite_medical_procedure: CompositeMedicalProcedure,
    /**SV6-02 (1332)
Code identifying the type of facility referenced*/
    pub facility_code_qualifier: Option<String>,
    /**SV6-03 (1331)
Code identifying where services were, or may be, performed; the National Uniform Billing Committee (NUBC) Facility Type Code for Institutional Services or the Place of Service Codes for Professional or Dental Services.*/
    pub facility_code_value: Option<String>,
    /**SV6-04 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**SV6-05 (1328)
A pointer to the diagnosis code in the order of importance to this service*/
    pub diagnosis_code_pointer: Vec<Numeric<0>>,
    /**SV6-06 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**SV6-07 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
}
impl AnesthesiaService {}