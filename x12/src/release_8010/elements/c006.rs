use serde::{Serialize, Deserialize};
/**To identify one or more areas of the oral cavity

See docs at <https://www.stedi.com/edi/x12/element/C006>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C006")]
pub struct OralCavityDesignation {
    /**C006-01 (1361)
Code Identifying the area of the oral cavity in which service is rendered*/
    pub oral_cavity_designation: String,
    /**C006-02 (1361)
Code Identifying the area of the oral cavity in which service is rendered*/
    pub c006_02: Option<String>,
    /**C006-03 (1361)
Code Identifying the area of the oral cavity in which service is rendered*/
    pub c006_03: Option<String>,
    /**C006-04 (1361)
Code Identifying the area of the oral cavity in which service is rendered*/
    pub c006_04: Option<String>,
    /**C006-05 (1361)
Code Identifying the area of the oral cavity in which service is rendered*/
    pub c006_05: Option<String>,
}
impl OralCavityDesignation {}