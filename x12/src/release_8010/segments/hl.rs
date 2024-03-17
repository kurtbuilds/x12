use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To identify dependencies among and the content of hierarchically related groups of data segments

See docs at <https://www.stedi.com/edi/x12/segment/HL>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "HL")]
pub struct HierarchicalLevel {
    /**HL-01 (628)
A unique number assigned by the sender to identify a particular data segment in a hierarchical structure*/
    pub hierarchical_id_number: String,
    /**HL-02 (734)
Identification number of the next higher hierarchical data segment that the data segment being described is subordinate to*/
    pub hierarchical_parent_id_number: Option<String>,
    /**HL-03 (735)
Code specifying the characteristic of a level in a hierarchical structure*/
    pub hierarchical_level_code: String,
    /**HL-04 (736)
Code indicating if there are hierarchical child data segments subordinate to the level being described*/
    pub hierarchical_child_code: Option<Fixed<1>>,
}
impl HierarchicalLevel {}