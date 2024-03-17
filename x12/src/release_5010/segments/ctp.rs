use serde::{Serialize, Deserialize};
use super::super::elements::CompositeUnitOfMeasure;
use crate::fixed::Fixed;
use crate::numeric::Numeric;
/**To specify pricing information

See docs at <https://www.stedi.com/edi/x12/segment/CTP>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CTP")]
pub struct Pricing {
    /**CTP-01 (687)
Code indicating class of trade*/
    pub class_of_trade_code: Option<Fixed<2>>,
    /**CTP-02 (236)
Code identifying pricing specification*/
    pub price_identifier_code: Option<Fixed<3>>,
    /**CTP-03 (212)
Price per unit of product, service, commodity, etc.*/
    pub unit_price: Option<String>,
    /**CTP-04 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**CTP-05 (C001)
To identify a composite unit of measure

(See Figures Appendix for examples of use)*/
    pub composite_unit_of_measure: Option<CompositeUnitOfMeasure>,
    /**CTP-06 (648)
Code indicating the type of price multiplier*/
    pub price_multiplier_qualifier: Option<Fixed<3>>,
    /**CTP-07 (649)
Value to be used as a multiplier to obtain a new value*/
    pub multiplier: Option<String>,
    /**CTP-08 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**CTP-09 (639)
Code identifying the type of unit price for an item*/
    pub basis_of_unit_price_code: Option<Fixed<2>>,
    /**CTP-10 (499)
Identifies rate restrictions or provisions*/
    pub condition_value: Option<String>,
    /**CTP-11 (289)
Quantity of units for a given price, e.g., 3 for $10.00*/
    pub multiple_price_quantity: Option<Numeric<0>>,
}
impl Pricing {}