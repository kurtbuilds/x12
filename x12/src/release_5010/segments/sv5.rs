use serde::{Serialize, Deserialize};
use super::super::elements::CompositeMedicalProcedure;
use crate::fixed::Fixed;
/**To specify the claim service detail for durable medical equipment

See docs at <https://www.stedi.com/edi/x12-005010/segment/SV5>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SV5")]
pub struct DurableMedicalEquipmentService {
    /**SV5-01 (C003)
To identify a medical procedure by its standardized codes and applicable modifiers*/
    pub composite_medical_procedure: CompositeMedicalProcedure,
    /**SV5-02 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Fixed<2>,
    /**SV5-03 (380)
Numeric value of quantity*/
    pub quantity: String,
    /**SV5-04 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**SV5-05 (782)
Monetary amount*/
    pub sv5_05: Option<String>,
    /**SV5-06 (594)
Code indicating frequency or type of activities or actions being reported*/
    pub frequency_code: Option<Fixed<1>>,
    /**SV5-07 (923)
Code indicating physician's prognosis for the patient*/
    pub prognosis_code: Option<Fixed<1>>,
}
impl DurableMedicalEquipmentService {}