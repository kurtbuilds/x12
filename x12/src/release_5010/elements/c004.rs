use serde::{Serialize, Deserialize};
/**To identify one or more diagnosis code pointers

See docs at <https://www.stedi.com/edi/x12-005010/element/C004>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C004")]
pub struct CompositeDiagnosisCodePointer {
    /**C004-01 (1328)
A pointer to the diagnosis code in the order of importance to this service*/
    pub composite_diagnosis_code_pointer: String,
    /**C004-02 (1328)
A pointer to the diagnosis code in the order of importance to this service*/
    pub c004_02: Option<String>,
    /**C004-03 (1328)
A pointer to the diagnosis code in the order of importance to this service*/
    pub c004_03: Option<String>,
    /**C004-04 (1328)
A pointer to the diagnosis code in the order of importance to this service*/
    pub c004_04: Option<String>,
}
impl CompositeDiagnosisCodePointer {}