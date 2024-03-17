use serde::{Serialize, Deserialize};
use super::super::elements::CompositeMedicalProcedure;
use crate::fixed::Fixed;
use super::super::elements::CompositeDiagnosisCodePointer;
/**To specify the service line item detail for a health care professional

See docs at <https://www.stedi.com/edi/x12/segment/SV1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SV1")]
pub struct ProfessionalService {
    /**SV1-01 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub composite_medical_procedure: CompositeMedicalProcedure,
    /**SV1-02 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**SV1-03 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**SV1-04 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**SV1-05 (1331)
Code identifying where services were, or may be, performed; the first and second positions of the Uniform Bill Type Code for Institutional Services or the Place of Service Codes for Professional or Dental Services.*/
    pub facility_code_value: Option<String>,
    /**SV1-06 (1365)
Code identifying the classification of service*/
    pub service_type_code: Option<String>,
    /**SV1-07 (C004)
To identify one or more diagnosis code pointers*/
    pub composite_diagnosis_code_pointer: Option<CompositeDiagnosisCodePointer>,
    /**SV1-08 (782)
Monetary amount*/
    pub sv1_08: Option<String>,
    /**SV1-09 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**SV1-10 (1340)
Code indicating proper adjudication and payment determination in cases involving multiple surgical procedures during the same surgical session*/
    pub multiple_procedure_code: Option<String>,
    /**SV1-11 (1073)
Code indicating a Yes or No condition or response*/
    pub sv1_11: Option<Fixed<1>>,
    /**SV1-12 (1073)
Code indicating a Yes or No condition or response*/
    pub sv1_12: Option<Fixed<1>>,
    /**SV1-13 (1364)
Code identifying extenuating circumstances or justifications which might assist any review of the medical necessity for this service*/
    pub review_code: Option<String>,
    /**SV1-14 (1341)
Value assigned by national or local organizations for various healthcare data elements*/
    pub national_or_local_assigned_review_value: Option<String>,
    /**SV1-15 (1327)
Code indicating whether or not co-payment requirements were met on a line by line basis*/
    pub copay_status_code: Option<Fixed<1>>,
    /**SV1-16 (1334)
Code identifying the Health Care Professional Shortage Area Code (HPSA)*/
    pub health_care_professional_shortage_area_code: Option<Fixed<1>>,
    /**SV1-17 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**SV1-18 (116)
Code defining international postal zone code excluding punctuation and blanks (zip code for United States)*/
    pub postal_code: Option<String>,
    /**SV1-19 (782)
Monetary amount*/
    pub sv1_19: Option<String>,
    /**SV1-20 (1337)
Code specifying the level of care provided by a nursing home facility*/
    pub level_of_care_code: Option<Fixed<1>>,
    /**SV1-21 (1360)
Code indicating the type of agreement under which the provider is submitting this claim*/
    pub provider_agreement_code: Option<Fixed<1>>,
}
impl ProfessionalService {}