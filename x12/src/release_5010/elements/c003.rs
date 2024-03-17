use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To identify a medical procedure by its standardized codes and applicable modifiers

See docs at <https://www.stedi.com/edi/x12-005010/element/C003>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C003")]
pub struct CompositeMedicalProcedure {
    /**C003-01 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub composite_medical_procedure: Fixed<2>,
    /**C003-02 (234)
Identifying number for a product or service*/
    pub c003_02: String,
    /**C003-03 (1339)
This identifies special circumstances related to the performance of the service, as defined by trading partners*/
    pub c003_03: Option<Fixed<2>>,
    /**C003-04 (1339)
This identifies special circumstances related to the performance of the service, as defined by trading partners*/
    pub c003_04: Option<Fixed<2>>,
    /**C003-05 (1339)
This identifies special circumstances related to the performance of the service, as defined by trading partners*/
    pub c003_05: Option<Fixed<2>>,
    /**C003-06 (1339)
This identifies special circumstances related to the performance of the service, as defined by trading partners*/
    pub c003_06: Option<Fixed<2>>,
    /**C003-07 (352)
A free-form description to clarify the related data elements and their content*/
    pub c003_07: Option<String>,
    /**C003-08 (234)
Identifying number for a product or service*/
    pub c003_08: Option<String>,
}
impl CompositeMedicalProcedure {}