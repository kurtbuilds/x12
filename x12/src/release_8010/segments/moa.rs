use serde::{Serialize, Deserialize};
/**To provide claim level data related to the adjudication of outpatient claims

See docs at <https://www.stedi.com/edi/x12/segment/MOA>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "MOA")]
pub struct OutpatientAdjudication {
    /**MOA-01 (954)
Percentage expressed as a decimal (e.g., 0.0 through 1.0 represents 0% through 100%)*/
    pub percentage_as_decimal: Option<String>,
    /**MOA-02 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**MOA-03 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**MOA-04 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub moa04: Option<String>,
    /**MOA-05 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub moa05: Option<String>,
    /**MOA-06 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub moa06: Option<String>,
    /**MOA-07 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub moa07: Option<String>,
    /**MOA-08 (782)
Monetary amount*/
    pub moa08: Option<String>,
    /**MOA-09 (782)
Monetary amount*/
    pub moa09: Option<String>,
}
impl OutpatientAdjudication {}