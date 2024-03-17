use serde::{Serialize, Deserialize};
/**To provide claim level data related to the adjudication of inpatient claims

See docs at <https://www.stedi.com/edi/x12/segment/MIA>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "MIA")]
pub struct InpatientAdjudication {
    /**MIA-01 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**MIA-02 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**MIA-03 (380)
Numeric value of quantity*/
    pub mia03: Option<String>,
    /**MIA-04 (782)
Monetary amount*/
    pub mia04: Option<String>,
    /**MIA-05 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**MIA-06 (782)
Monetary amount*/
    pub mia06: Option<String>,
    /**MIA-07 (782)
Monetary amount*/
    pub mia07: Option<String>,
    /**MIA-08 (782)
Monetary amount*/
    pub mia08: Option<String>,
    /**MIA-09 (782)
Monetary amount*/
    pub mia09: Option<String>,
    /**MIA-10 (782)
Monetary amount*/
    pub mia10: Option<String>,
    /**MIA-11 (782)
Monetary amount*/
    pub mia11: Option<String>,
    /**MIA-12 (782)
Monetary amount*/
    pub mia12: Option<String>,
    /**MIA-13 (782)
Monetary amount*/
    pub mia13: Option<String>,
    /**MIA-14 (782)
Monetary amount*/
    pub mia14: Option<String>,
    /**MIA-15 (380)
Numeric value of quantity*/
    pub mia15: Option<String>,
    /**MIA-16 (782)
Monetary amount*/
    pub mia16: Option<String>,
    /**MIA-17 (782)
Monetary amount*/
    pub mia17: Option<String>,
    /**MIA-18 (782)
Monetary amount*/
    pub mia18: Option<String>,
    /**MIA-19 (782)
Monetary amount*/
    pub mia19: Option<String>,
    /**MIA-20 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub mia20: Option<String>,
    /**MIA-21 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub mia21: Option<String>,
    /**MIA-22 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub mia22: Option<String>,
    /**MIA-23 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub mia23: Option<String>,
    /**MIA-24 (782)
Monetary amount*/
    pub mia24: Option<String>,
}
impl InpatientAdjudication {}