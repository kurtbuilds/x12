use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
use super::super::elements::CompositeMedicalProcedure;
/**To specify the claim service detail for prescription drugs

See docs at <https://www.stedi.com/edi/x12-005010/segment/SV4>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SV4")]
pub struct DrugService {
    /**SV4-01 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: String,
    /**SV4-02 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub composite_medical_procedure: Option<CompositeMedicalProcedure>,
    /**SV4-03 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub sv4_03: Option<String>,
    /**SV4-04 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**SV4-05 (1329)
Code indicating whether or not the prescriber's instructions regarding generic substitution were followed*/
    pub dispense_as_written_code: Option<Fixed<1>>,
    /**SV4-06 (1338)
Code specifying the level of service rendered*/
    pub level_of_service_code: Option<String>,
    /**SV4-07 (1356)
Code indicating the origin of a prescription*/
    pub prescription_origin_code: Option<Fixed<1>>,
    /**SV4-08 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**SV4-09 (1073)
Code indicating a Yes or No condition or response*/
    pub sv4_09: Option<Fixed<1>>,
    /**SV4-10 (1073)
Code indicating a Yes or No condition or response*/
    pub sv4_10: Option<Fixed<1>>,
    /**SV4-11 (1370)
Code indicating the type of unit dose dispensing done*/
    pub unit_dose_code: Option<Fixed<1>>,
    /**SV4-12 (1319)
Code indicating the method by which the ingredient cost was calculated*/
    pub basis_of_cost_determination_code: Option<String>,
    /**SV4-13 (1320)
Code indicating the method by which the days supply was determined*/
    pub basis_of_days_supply_determination_code: Option<Fixed<1>>,
    /**SV4-14 (1330)
Code indicating the form in which the drug is dispensed*/
    pub dosage_form_code: Option<Fixed<2>>,
    /**SV4-15 (1327)
Code indicating whether or not co-payment requirements were met on a line by line basis*/
    pub copay_status_code: Option<Fixed<1>>,
    /**SV4-16 (1384)
Code identifying the location where patient is receiving medical treatment*/
    pub patient_location_code: Option<Fixed<1>>,
    /**SV4-17 (1337)
Code specifying the level of care provided by a nursing home facility*/
    pub level_of_care_code: Option<Fixed<1>>,
    /**SV4-18 (1357)
Code indicating the type of prior authorization or medical certification that has occurred*/
    pub prior_authorization_type_code: Option<Fixed<1>>,
}
impl DrugService {}