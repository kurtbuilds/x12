use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1065

See docs at <https://www.stedi.com/edi/x12/element/1065>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityTypeQualifier {
    ///1 - Person
    Person,
    ///2 - Non-Person Entity
    NonPersonEntity,
    ///3 - Unknown
    Unknown,
    ///4 - Corporation
    Corporation,
    ///5 - Trust
    Trust,
    ///6 - Organization
    Organization,
    ///7 - Limited Liability Corporation
    LimitedLiabilityCorporation,
    ///8 - Partnership
    Partnership,
    ///9 - S Corporation
    SCorporation,
    ///A - Federally Chartered Financial Institution
    FederallyCharteredFinancialInstitution,
    ///B - State Chartered Financial Institution
    StateCharteredFinancialInstitution,
    ///C - Custodial
    Custodial,
    ///D - Non-Profit Organization
    NonProfitOrganization,
    ///E - Sole Proprietorship
    SoleProprietorship,
    ///G - Government
    Government,
    ///H - Non-Profit Government Agency
    NonProfitGovernmentAgency,
    ///L - Limited Partnership
    LimitedPartnership,
}
impl EntityTypeQualifier {
    pub fn code(&self) -> &str {
        {
            use EntityTypeQualifier::*;
            match self {
                Person => "1",
                NonPersonEntity => "2",
                Unknown => "3",
                Corporation => "4",
                Trust => "5",
                Organization => "6",
                LimitedLiabilityCorporation => "7",
                Partnership => "8",
                SCorporation => "9",
                FederallyCharteredFinancialInstitution => "A",
                StateCharteredFinancialInstitution => "B",
                Custodial => "C",
                NonProfitOrganization => "D",
                SoleProprietorship => "E",
                Government => "G",
                NonProfitGovernmentAgency => "H",
                LimitedPartnership => "L",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<EntityTypeQualifier> {
        use EntityTypeQualifier::*;
        match code {
            b"1" => Some(Person),
            b"2" => Some(NonPersonEntity),
            b"3" => Some(Unknown),
            b"4" => Some(Corporation),
            b"5" => Some(Trust),
            b"6" => Some(Organization),
            b"7" => Some(LimitedLiabilityCorporation),
            b"8" => Some(Partnership),
            b"9" => Some(SCorporation),
            b"A" => Some(FederallyCharteredFinancialInstitution),
            b"B" => Some(StateCharteredFinancialInstitution),
            b"C" => Some(Custodial),
            b"D" => Some(NonProfitOrganization),
            b"E" => Some(SoleProprietorship),
            b"G" => Some(Government),
            b"H" => Some(NonProfitGovernmentAgency),
            b"L" => Some(LimitedPartnership),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use EntityTypeQualifier::*;
        match self {
            Person => "Person",
            NonPersonEntity => "Non-Person Entity",
            Unknown => "Unknown",
            Corporation => "Corporation",
            Trust => "Trust",
            Organization => "Organization",
            LimitedLiabilityCorporation => "Limited Liability Corporation",
            Partnership => "Partnership",
            SCorporation => "S Corporation",
            FederallyCharteredFinancialInstitution => {
                "Federally Chartered Financial Institution"
            }
            StateCharteredFinancialInstitution => "State Chartered Financial Institution",
            Custodial => "Custodial",
            NonProfitOrganization => "Non-Profit Organization",
            SoleProprietorship => "Sole Proprietorship",
            Government => "Government",
            NonProfitGovernmentAgency => "Non-Profit Government Agency",
            LimitedPartnership => "Limited Partnership",
        }
    }
    fn from_description(description: &str) -> Option<EntityTypeQualifier> {
        {
            use EntityTypeQualifier::*;
            match description {
                "Person" => Some(Person),
                "Non-Person Entity" => Some(NonPersonEntity),
                "Unknown" => Some(Unknown),
                "Corporation" => Some(Corporation),
                "Trust" => Some(Trust),
                "Organization" => Some(Organization),
                "Limited Liability Corporation" => Some(LimitedLiabilityCorporation),
                "Partnership" => Some(Partnership),
                "S Corporation" => Some(SCorporation),
                "Federally Chartered Financial Institution" => {
                    Some(FederallyCharteredFinancialInstitution)
                }
                "State Chartered Financial Institution" => {
                    Some(StateCharteredFinancialInstitution)
                }
                "Custodial" => Some(Custodial),
                "Non-Profit Organization" => Some(NonProfitOrganization),
                "Sole Proprietorship" => Some(SoleProprietorship),
                "Government" => Some(Government),
                "Non-Profit Government Agency" => Some(NonProfitGovernmentAgency),
                "Limited Partnership" => Some(LimitedPartnership),
                _ => None,
            }
        }
    }
}
impl Serialize for EntityTypeQualifier {
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
    type Value = EntityTypeQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Entity Type Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EntityTypeQualifier::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Entity Type Qualifier: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EntityTypeQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Entity Type Qualifier: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for EntityTypeQualifier {
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