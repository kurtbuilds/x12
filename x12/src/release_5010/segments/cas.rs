use serde::{Serialize, Deserialize};
/**To supply adjustment reason codes and amounts as needed for an entire claim or for a particular service within the claim being paid

See docs at <https://www.stedi.com/edi/x12/segment/CAS>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CAS")]
pub struct ClaimsAdjustment {
    /**CAS-01 (1033)
Code identifying the general category of payment adjustment*/
    pub claim_adjustment_group_code: String,
    /**CAS-02 (1034)
Code identifying the detailed reason the adjustment was made*/
    pub claim_adjustment_reason_code: String,
    /**CAS-03 (782)
Monetary amount*/
    pub monetary_amount: String,
    /**CAS-04 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**CAS-05 (1034)
Code identifying the detailed reason the adjustment was made*/
    pub cas05: Option<String>,
    /**CAS-06 (782)
Monetary amount*/
    pub cas06: Option<String>,
    /**CAS-07 (380)
Numeric value of quantity*/
    pub cas07: Option<String>,
    /**CAS-08 (1034)
Code identifying the detailed reason the adjustment was made*/
    pub cas08: Option<String>,
    /**CAS-09 (782)
Monetary amount*/
    pub cas09: Option<String>,
    /**CAS-10 (380)
Numeric value of quantity*/
    pub cas10: Option<String>,
    /**CAS-11 (1034)
Code identifying the detailed reason the adjustment was made*/
    pub cas11: Option<String>,
    /**CAS-12 (782)
Monetary amount*/
    pub cas12: Option<String>,
    /**CAS-13 (380)
Numeric value of quantity*/
    pub cas13: Option<String>,
    /**CAS-14 (1034)
Code identifying the detailed reason the adjustment was made*/
    pub cas14: Option<String>,
    /**CAS-15 (782)
Monetary amount*/
    pub cas15: Option<String>,
    /**CAS-16 (380)
Numeric value of quantity*/
    pub cas16: Option<String>,
    /**CAS-17 (1034)
Code identifying the detailed reason the adjustment was made*/
    pub cas17: Option<String>,
    /**CAS-18 (782)
Monetary amount*/
    pub cas18: Option<String>,
    /**CAS-19 (380)
Numeric value of quantity*/
    pub cas19: Option<String>,
}
impl ClaimsAdjustment {}