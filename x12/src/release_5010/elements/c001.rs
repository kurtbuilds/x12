use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To identify a composite unit of measure

(See Figures Appendix for examples of use)

See docs at <https://www.stedi.com/edi/x12-005010/element/C001>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C001")]
pub struct CompositeUnitOfMeasure {
    /**C001-01 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub composite_unit_of_measure: Fixed<2>,
    /**C001-02 (1018)
Power to which a unit is raised*/
    pub c001_02: Option<String>,
    /**C001-03 (649)
Value to be used as a multiplier to obtain a new value*/
    pub c001_03: Option<String>,
    /**C001-04 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub c001_04: Option<Fixed<2>>,
    /**C001-05 (1018)
Power to which a unit is raised*/
    pub c001_05: Option<String>,
    /**C001-06 (649)
Value to be used as a multiplier to obtain a new value*/
    pub c001_06: Option<String>,
    /**C001-07 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub c001_07: Option<Fixed<2>>,
    /**C001-08 (1018)
Power to which a unit is raised*/
    pub c001_08: Option<String>,
    /**C001-09 (649)
Value to be used as a multiplier to obtain a new value*/
    pub c001_09: Option<String>,
    /**C001-10 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub c001_10: Option<Fixed<2>>,
    /**C001-11 (1018)
Power to which a unit is raised*/
    pub c001_11: Option<String>,
    /**C001-12 (649)
Value to be used as a multiplier to obtain a new value*/
    pub c001_12: Option<String>,
    /**C001-13 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub c001_13: Option<Fixed<2>>,
    /**C001-14 (1018)
Power to which a unit is raised*/
    pub c001_14: Option<String>,
    /**C001-15 (649)
Value to be used as a multiplier to obtain a new value*/
    pub c001_15: Option<String>,
}
impl CompositeUnitOfMeasure {}