use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1221

See docs at <https://www.stedi.com/edi/x12/element/1221>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderCode {
    ///AD - Admitting
    Admitting,
    ///AS - Assistant Surgeon
    AssistantSurgeon,
    ///AT - Attending
    Attending,
    ///BI - Billing
    Billing,
    ///BS - Billing Service
    BillingService,
    ///CO - Consulting
    Consulting,
    ///CV - Covering
    Covering,
    ///H - Hospital
    Hospital,
    ///HH - Home Health Care
    HomeHealthCare,
    ///LA - Laboratory
    Laboratory,
    ///ON - On Staff
    OnStaff,
    ///OP - Operating
    Operating,
    ///OR - Ordering
    Ordering,
    ///OT - Other Physician
    OtherPhysician,
    ///P1 - Pharmacist
    Pharmacist,
    ///P2 - Pharmacy
    Pharmacy,
    ///PC - Primary Care Physician
    PrimaryCarePhysician,
    ///PE - Performing
    Performing,
    ///PT - Pay-To
    PayTo,
    ///PU - Purchasing
    Purchasing,
    ///R - Rural Health Clinic
    RuralHealthClinic,
    ///RF - Referring
    Referring,
    ///RP - Reporting Provider
    ReportingProvider,
    ///SB - Submitting
    Submitting,
    ///SK - Skilled Nursing Facility
    SkilledNursingFacility,
    ///SU - Supervising
    Supervising,
}
impl ProviderCode {
    pub fn code(&self) -> &str {
        {
            use ProviderCode::*;
            match self {
                Admitting => "AD",
                AssistantSurgeon => "AS",
                Attending => "AT",
                Billing => "BI",
                BillingService => "BS",
                Consulting => "CO",
                Covering => "CV",
                Hospital => "H",
                HomeHealthCare => "HH",
                Laboratory => "LA",
                OnStaff => "ON",
                Operating => "OP",
                Ordering => "OR",
                OtherPhysician => "OT",
                Pharmacist => "P1",
                Pharmacy => "P2",
                PrimaryCarePhysician => "PC",
                Performing => "PE",
                PayTo => "PT",
                Purchasing => "PU",
                RuralHealthClinic => "R",
                Referring => "RF",
                ReportingProvider => "RP",
                Submitting => "SB",
                SkilledNursingFacility => "SK",
                Supervising => "SU",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ProviderCode> {
        use ProviderCode::*;
        match code {
            b"AD" => Some(Admitting),
            b"AS" => Some(AssistantSurgeon),
            b"AT" => Some(Attending),
            b"BI" => Some(Billing),
            b"BS" => Some(BillingService),
            b"CO" => Some(Consulting),
            b"CV" => Some(Covering),
            b"H" => Some(Hospital),
            b"HH" => Some(HomeHealthCare),
            b"LA" => Some(Laboratory),
            b"ON" => Some(OnStaff),
            b"OP" => Some(Operating),
            b"OR" => Some(Ordering),
            b"OT" => Some(OtherPhysician),
            b"P1" => Some(Pharmacist),
            b"P2" => Some(Pharmacy),
            b"PC" => Some(PrimaryCarePhysician),
            b"PE" => Some(Performing),
            b"PT" => Some(PayTo),
            b"PU" => Some(Purchasing),
            b"R" => Some(RuralHealthClinic),
            b"RF" => Some(Referring),
            b"RP" => Some(ReportingProvider),
            b"SB" => Some(Submitting),
            b"SK" => Some(SkilledNursingFacility),
            b"SU" => Some(Supervising),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ProviderCode::*;
        match self {
            Admitting => "Admitting",
            AssistantSurgeon => "Assistant Surgeon",
            Attending => "Attending",
            Billing => "Billing",
            BillingService => "Billing Service",
            Consulting => "Consulting",
            Covering => "Covering",
            Hospital => "Hospital",
            HomeHealthCare => "Home Health Care",
            Laboratory => "Laboratory",
            OnStaff => "On Staff",
            Operating => "Operating",
            Ordering => "Ordering",
            OtherPhysician => "Other Physician",
            Pharmacist => "Pharmacist",
            Pharmacy => "Pharmacy",
            PrimaryCarePhysician => "Primary Care Physician",
            Performing => "Performing",
            PayTo => "Pay-To",
            Purchasing => "Purchasing",
            RuralHealthClinic => "Rural Health Clinic",
            Referring => "Referring",
            ReportingProvider => "Reporting Provider",
            Submitting => "Submitting",
            SkilledNursingFacility => "Skilled Nursing Facility",
            Supervising => "Supervising",
        }
    }
    fn from_description(description: &str) -> Option<ProviderCode> {
        {
            use ProviderCode::*;
            match description {
                "Admitting" => Some(Admitting),
                "Assistant Surgeon" => Some(AssistantSurgeon),
                "Attending" => Some(Attending),
                "Billing" => Some(Billing),
                "Billing Service" => Some(BillingService),
                "Consulting" => Some(Consulting),
                "Covering" => Some(Covering),
                "Hospital" => Some(Hospital),
                "Home Health Care" => Some(HomeHealthCare),
                "Laboratory" => Some(Laboratory),
                "On Staff" => Some(OnStaff),
                "Operating" => Some(Operating),
                "Ordering" => Some(Ordering),
                "Other Physician" => Some(OtherPhysician),
                "Pharmacist" => Some(Pharmacist),
                "Pharmacy" => Some(Pharmacy),
                "Primary Care Physician" => Some(PrimaryCarePhysician),
                "Performing" => Some(Performing),
                "Pay-To" => Some(PayTo),
                "Purchasing" => Some(Purchasing),
                "Rural Health Clinic" => Some(RuralHealthClinic),
                "Referring" => Some(Referring),
                "Reporting Provider" => Some(ReportingProvider),
                "Submitting" => Some(Submitting),
                "Skilled Nursing Facility" => Some(SkilledNursingFacility),
                "Supervising" => Some(Supervising),
                _ => None,
            }
        }
    }
}
impl Serialize for ProviderCode {
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
    type Value = ProviderCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Provider Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Provider Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Provider Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ProviderCode {
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