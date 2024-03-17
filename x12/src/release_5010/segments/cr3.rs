use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information regarding a physician's certification for durable medical equipment

See docs at <https://www.stedi.com/edi/x12-005010/segment/CR3>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CR3")]
pub struct DurableMedicalEquipmentCertification {
    /**CR3-01 (1322)
Code indicating the type of certification*/
    pub certification_type_code: Option<Fixed<1>>,
    /**CR3-02 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**CR3-03 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**CR3-04 (1335)
Code indicating the condition that demonstrates insulin dependence*/
    pub insulin_dependent_code: Option<Fixed<1>>,
    /**CR3-05 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
}
impl DurableMedicalEquipmentCertification {}