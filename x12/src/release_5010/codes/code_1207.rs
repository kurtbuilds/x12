use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1207

See docs at <https://www.stedi.com/edi/x12/element/1207>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoverageLevelCode {
    ///CHD - Children Only
    ChildrenOnly,
    ///DEP - Dependents Only
    DependentsOnly,
    ///E1D - Employee and One Dependent
    EmployeeAndOneDependent,
    ///E2D - Employee and Two Dependents
    EmployeeAndTwoDependents,
    ///E3D - Employee and Three Dependents
    EmployeeAndThreeDependents,
    ///E5D - Employee and One or More Dependents
    EmployeeAndOneOrMoreDependents,
    ///E6D - Employee and Two or More Dependents
    EmployeeAndTwoOrMoreDependents,
    ///E7D - Employee and Three or More Dependents
    EmployeeAndThreeOrMoreDependents,
    ///E8D - Employee and Four or More Dependents
    EmployeeAndFourOrMoreDependents,
    ///E9D - Employee and Five or More Dependents
    EmployeeAndFiveOrMoreDependents,
    ///ECH - Employee and Children
    EmployeeAndChildren,
    ///EMP - Employee Only
    EmployeeOnly,
    ///ESP - Employee and Spouse
    EmployeeAndSpouse,
    ///FAM - Family
    Family,
    ///IND - Individual
    Individual,
    ///S1C - Subscriber and 1 child
    SubscriberAnd1Child,
    ///S5C - Subscriber and 1 or more children
    SubscriberAnd1OrMoreChildren,
    ///S6C - Subscriber and 2 or more children
    SubscriberAnd2OrMoreChildren,
    ///SPC - Spouse and Children
    SpouseAndChildren,
    ///SPO - Spouse Only
    SpouseOnly,
    ///SS1 - Subscriber, spouse, and 1 child
    CodeSS1,
    ///SS5 - Subscriber, spouse, and 1 or more children
    CodeSS5,
    ///SS6 - Subscriber, spouse, and 2 or more children
    CodeSS6,
    ///SSP - Subscriber and spouse
    SubscriberAndSpouse,
    ///TWO - Two Party
    TwoParty,
}
impl CoverageLevelCode {
    pub fn code(&self) -> &str {
        {
            use CoverageLevelCode::*;
            match self {
                ChildrenOnly => "CHD",
                DependentsOnly => "DEP",
                EmployeeAndOneDependent => "E1D",
                EmployeeAndTwoDependents => "E2D",
                EmployeeAndThreeDependents => "E3D",
                EmployeeAndOneOrMoreDependents => "E5D",
                EmployeeAndTwoOrMoreDependents => "E6D",
                EmployeeAndThreeOrMoreDependents => "E7D",
                EmployeeAndFourOrMoreDependents => "E8D",
                EmployeeAndFiveOrMoreDependents => "E9D",
                EmployeeAndChildren => "ECH",
                EmployeeOnly => "EMP",
                EmployeeAndSpouse => "ESP",
                Family => "FAM",
                Individual => "IND",
                SubscriberAnd1Child => "S1C",
                SubscriberAnd1OrMoreChildren => "S5C",
                SubscriberAnd2OrMoreChildren => "S6C",
                SpouseAndChildren => "SPC",
                SpouseOnly => "SPO",
                CodeSS1 => "SS1",
                CodeSS5 => "SS5",
                CodeSS6 => "SS6",
                SubscriberAndSpouse => "SSP",
                TwoParty => "TWO",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CoverageLevelCode> {
        use CoverageLevelCode::*;
        match code {
            b"CHD" => Some(ChildrenOnly),
            b"DEP" => Some(DependentsOnly),
            b"E1D" => Some(EmployeeAndOneDependent),
            b"E2D" => Some(EmployeeAndTwoDependents),
            b"E3D" => Some(EmployeeAndThreeDependents),
            b"E5D" => Some(EmployeeAndOneOrMoreDependents),
            b"E6D" => Some(EmployeeAndTwoOrMoreDependents),
            b"E7D" => Some(EmployeeAndThreeOrMoreDependents),
            b"E8D" => Some(EmployeeAndFourOrMoreDependents),
            b"E9D" => Some(EmployeeAndFiveOrMoreDependents),
            b"ECH" => Some(EmployeeAndChildren),
            b"EMP" => Some(EmployeeOnly),
            b"ESP" => Some(EmployeeAndSpouse),
            b"FAM" => Some(Family),
            b"IND" => Some(Individual),
            b"S1C" => Some(SubscriberAnd1Child),
            b"S5C" => Some(SubscriberAnd1OrMoreChildren),
            b"S6C" => Some(SubscriberAnd2OrMoreChildren),
            b"SPC" => Some(SpouseAndChildren),
            b"SPO" => Some(SpouseOnly),
            b"SS1" => Some(CodeSS1),
            b"SS5" => Some(CodeSS5),
            b"SS6" => Some(CodeSS6),
            b"SSP" => Some(SubscriberAndSpouse),
            b"TWO" => Some(TwoParty),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CoverageLevelCode::*;
        match self {
            ChildrenOnly => "Children Only",
            DependentsOnly => "Dependents Only",
            EmployeeAndOneDependent => "Employee and One Dependent",
            EmployeeAndTwoDependents => "Employee and Two Dependents",
            EmployeeAndThreeDependents => "Employee and Three Dependents",
            EmployeeAndOneOrMoreDependents => "Employee and One or More Dependents",
            EmployeeAndTwoOrMoreDependents => "Employee and Two or More Dependents",
            EmployeeAndThreeOrMoreDependents => "Employee and Three or More Dependents",
            EmployeeAndFourOrMoreDependents => "Employee and Four or More Dependents",
            EmployeeAndFiveOrMoreDependents => "Employee and Five or More Dependents",
            EmployeeAndChildren => "Employee and Children",
            EmployeeOnly => "Employee Only",
            EmployeeAndSpouse => "Employee and Spouse",
            Family => "Family",
            Individual => "Individual",
            SubscriberAnd1Child => "Subscriber and 1 child",
            SubscriberAnd1OrMoreChildren => "Subscriber and 1 or more children",
            SubscriberAnd2OrMoreChildren => "Subscriber and 2 or more children",
            SpouseAndChildren => "Spouse and Children",
            SpouseOnly => "Spouse Only",
            CodeSS1 => "Subscriber, spouse, and 1 child",
            CodeSS5 => "Subscriber, spouse, and 1 or more children",
            CodeSS6 => "Subscriber, spouse, and 2 or more children",
            SubscriberAndSpouse => "Subscriber and spouse",
            TwoParty => "Two Party",
        }
    }
    fn from_description(description: &str) -> Option<CoverageLevelCode> {
        {
            use CoverageLevelCode::*;
            match description {
                "Children Only" => Some(ChildrenOnly),
                "Dependents Only" => Some(DependentsOnly),
                "Employee and One Dependent" => Some(EmployeeAndOneDependent),
                "Employee and Two Dependents" => Some(EmployeeAndTwoDependents),
                "Employee and Three Dependents" => Some(EmployeeAndThreeDependents),
                "Employee and One or More Dependents" => {
                    Some(EmployeeAndOneOrMoreDependents)
                }
                "Employee and Two or More Dependents" => {
                    Some(EmployeeAndTwoOrMoreDependents)
                }
                "Employee and Three or More Dependents" => {
                    Some(EmployeeAndThreeOrMoreDependents)
                }
                "Employee and Four or More Dependents" => {
                    Some(EmployeeAndFourOrMoreDependents)
                }
                "Employee and Five or More Dependents" => {
                    Some(EmployeeAndFiveOrMoreDependents)
                }
                "Employee and Children" => Some(EmployeeAndChildren),
                "Employee Only" => Some(EmployeeOnly),
                "Employee and Spouse" => Some(EmployeeAndSpouse),
                "Family" => Some(Family),
                "Individual" => Some(Individual),
                "Subscriber and 1 child" => Some(SubscriberAnd1Child),
                "Subscriber and 1 or more children" => Some(SubscriberAnd1OrMoreChildren),
                "Subscriber and 2 or more children" => Some(SubscriberAnd2OrMoreChildren),
                "Spouse and Children" => Some(SpouseAndChildren),
                "Spouse Only" => Some(SpouseOnly),
                "Subscriber, spouse, and 1 child" => Some(CodeSS1),
                "Subscriber, spouse, and 1 or more children" => Some(CodeSS5),
                "Subscriber, spouse, and 2 or more children" => Some(CodeSS6),
                "Subscriber and spouse" => Some(SubscriberAndSpouse),
                "Two Party" => Some(TwoParty),
                _ => None,
            }
        }
    }
}
impl Serialize for CoverageLevelCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = if serializer.is_human_readable() {
            self.description()
        } else {
            self.code()
        };
        serializer.serialize_str(value)
    }
}
struct Visitor;
impl<'de> de::Visitor<'de> for Visitor {
    type Value = CoverageLevelCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Coverage Level Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CoverageLevelCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Coverage Level Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CoverageLevelCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Coverage Level Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for CoverageLevelCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(Visitor)
        } else {
            deserializer.deserialize_bytes(Visitor)
        }
    }
}