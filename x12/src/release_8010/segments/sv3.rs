use serde::{Serialize, Deserialize};
use super::super::elements::CompositeMedicalProcedure;
use crate::numeric::Numeric;
use super::super::elements::OralCavityDesignation;
use crate::fixed::Fixed;
/**To specify the service line item detail for dental work

See docs at <https://www.stedi.com/edi/x12/segment/SV3>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SV3")]
pub struct DentalService {
    /**SV3-01 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub composite_medical_procedure: CompositeMedicalProcedure,
    /**SV3-02 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**SV3-03 (1331)
Code identifying where services were, or may be, performed; the National Uniform Billing Committee (NUBC) Facility Type Code for Institutional Services or the Place of Service Codes for Professional or Dental Services.*/
    pub facility_code_value: Option<String>,
    /**SV3-04 (C006)
To identify one or more areas of the oral cavity*/
    pub oral_cavity_designation: Option<OralCavityDesignation>,
    /**SV3-05 (1358)
Code specifying the placement status for the dental work*/
    pub prosthesis: Option<Fixed<1>>,
    /**SV3-06 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**SV3-07 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**SV3-08 (1327)
Code indicating whether or not co-payment requirements were met on a line by line basis*/
    pub copay_status_code: Option<Fixed<1>>,
    /**SV3-09 (1360)
Code indicating the type of agreement under which the provider is submitting this claim*/
    pub provider_agreement_code: Option<Fixed<1>>,
    /**SV3-10 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**SV3-11 (1328)
A pointer to the diagnosis code in the order of importance to this service*/
    pub diagnosis_code_pointer: Vec<Numeric<0>>,
}
impl DentalService {}