use serde::{Serialize, Deserialize};
use super::super::codes::ToothSurfaceCode;
/**To identify one or more tooth surface codes

See docs at <https://www.stedi.com/edi/x12-005010/element/C005>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C005")]
pub struct ToothSurface {
    /**C005-01 (1369)
Code identifying the area of the tooth that was treated*/
    pub tooth_surface: ToothSurfaceCode,
    /**C005-02 (1369)
Code identifying the area of the tooth that was treated*/
    pub c005_02: Option<ToothSurfaceCode>,
    /**C005-03 (1369)
Code identifying the area of the tooth that was treated*/
    pub c005_03: Option<ToothSurfaceCode>,
    /**C005-04 (1369)
Code identifying the area of the tooth that was treated*/
    pub c005_04: Option<ToothSurfaceCode>,
    /**C005-05 (1369)
Code identifying the area of the tooth that was treated*/
    pub c005_05: Option<ToothSurfaceCode>,
}
impl ToothSurface {}