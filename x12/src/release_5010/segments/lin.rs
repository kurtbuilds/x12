use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify basic item identification data

See docs at <https://www.stedi.com/edi/x12-005010/segment/LIN>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "LIN")]
pub struct ItemIdentification {
    /**LIN-01 (350)
Alphanumeric characters assigned for differentiation within a transaction set*/
    pub assigned_identification: Option<String>,
    /**LIN-02 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub product_service_id_qualifier: Fixed<2>,
    /**LIN-03 (234)
Identifying number for a product or service*/
    pub product_service_id: String,
    /**LIN-04 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin04: Option<Fixed<2>>,
    /**LIN-05 (234)
Identifying number for a product or service*/
    pub lin05: Option<String>,
    /**LIN-06 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin06: Option<Fixed<2>>,
    /**LIN-07 (234)
Identifying number for a product or service*/
    pub lin07: Option<String>,
    /**LIN-08 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin08: Option<Fixed<2>>,
    /**LIN-09 (234)
Identifying number for a product or service*/
    pub lin09: Option<String>,
    /**LIN-10 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin10: Option<Fixed<2>>,
    /**LIN-11 (234)
Identifying number for a product or service*/
    pub lin11: Option<String>,
    /**LIN-12 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin12: Option<Fixed<2>>,
    /**LIN-13 (234)
Identifying number for a product or service*/
    pub lin13: Option<String>,
    /**LIN-14 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin14: Option<Fixed<2>>,
    /**LIN-15 (234)
Identifying number for a product or service*/
    pub lin15: Option<String>,
    /**LIN-16 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin16: Option<Fixed<2>>,
    /**LIN-17 (234)
Identifying number for a product or service*/
    pub lin17: Option<String>,
    /**LIN-18 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin18: Option<Fixed<2>>,
    /**LIN-19 (234)
Identifying number for a product or service*/
    pub lin19: Option<String>,
    /**LIN-20 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin20: Option<Fixed<2>>,
    /**LIN-21 (234)
Identifying number for a product or service*/
    pub lin21: Option<String>,
    /**LIN-22 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin22: Option<Fixed<2>>,
    /**LIN-23 (234)
Identifying number for a product or service*/
    pub lin23: Option<String>,
    /**LIN-24 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin24: Option<Fixed<2>>,
    /**LIN-25 (234)
Identifying number for a product or service*/
    pub lin25: Option<String>,
    /**LIN-26 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin26: Option<Fixed<2>>,
    /**LIN-27 (234)
Identifying number for a product or service*/
    pub lin27: Option<String>,
    /**LIN-28 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin28: Option<Fixed<2>>,
    /**LIN-29 (234)
Identifying number for a product or service*/
    pub lin29: Option<String>,
    /**LIN-30 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub lin30: Option<Fixed<2>>,
    /**LIN-31 (234)
Identifying number for a product or service*/
    pub lin31: Option<String>,
}
impl ItemIdentification {}