use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1363

See docs at <https://www.stedi.com/edi/x12/element/1363>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReleaseOfInformationCode {
    ///A - Appropriate Release of Information on File at Health Care Service Provider or at Utilization Review Organization
    AppropriateReleaseOfInformationOnFileAtHealthCareServiceProviderOrAtUtilizationReviewOrganization,
    ///I - Informed Consent to Release Medical Information for Conditions or Diagnoses Regulated by Federal Statutes
    InformedConsentToReleaseMedicalInformationForConditionsOrDiagnosesRegulatedByFederalStatutes,
    ///M - The Provider has Limited or Restricted Ability to Release Data Related to a Claim
    TheProviderHasLimitedOrRestrictedAbilityToReleaseDataRelatedToAClaim,
    ///N - No, Provider is Not Allowed to Release Data
    CodeN,
    ///O - On file at Payor or at Plan Sponsor
    OnFileAtPayorOrAtPlanSponsor,
    ///Y - Yes, Provider has a Signed Statement Permitting Release of Medical Billing Data Related to a Claim
    CodeY,
}
impl ReleaseOfInformationCode {
    pub fn code(&self) -> &str {
        {
            use ReleaseOfInformationCode::*;
            match self {
                AppropriateReleaseOfInformationOnFileAtHealthCareServiceProviderOrAtUtilizationReviewOrganization => {
                    "A"
                }
                InformedConsentToReleaseMedicalInformationForConditionsOrDiagnosesRegulatedByFederalStatutes => {
                    "I"
                }
                TheProviderHasLimitedOrRestrictedAbilityToReleaseDataRelatedToAClaim => {
                    "M"
                }
                CodeN => "N",
                OnFileAtPayorOrAtPlanSponsor => "O",
                CodeY => "Y",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ReleaseOfInformationCode> {
        use ReleaseOfInformationCode::*;
        match code {
            b"A" => {
                Some(
                    AppropriateReleaseOfInformationOnFileAtHealthCareServiceProviderOrAtUtilizationReviewOrganization,
                )
            }
            b"I" => {
                Some(
                    InformedConsentToReleaseMedicalInformationForConditionsOrDiagnosesRegulatedByFederalStatutes,
                )
            }
            b"M" => {
                Some(
                    TheProviderHasLimitedOrRestrictedAbilityToReleaseDataRelatedToAClaim,
                )
            }
            b"N" => Some(CodeN),
            b"O" => Some(OnFileAtPayorOrAtPlanSponsor),
            b"Y" => Some(CodeY),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ReleaseOfInformationCode::*;
        match self {
            AppropriateReleaseOfInformationOnFileAtHealthCareServiceProviderOrAtUtilizationReviewOrganization => {
                "Appropriate Release of Information on File at Health Care Service Provider or at Utilization Review Organization"
            }
            InformedConsentToReleaseMedicalInformationForConditionsOrDiagnosesRegulatedByFederalStatutes => {
                "Informed Consent to Release Medical Information for Conditions or Diagnoses Regulated by Federal Statutes"
            }
            TheProviderHasLimitedOrRestrictedAbilityToReleaseDataRelatedToAClaim => {
                "The Provider has Limited or Restricted Ability to Release Data Related to a Claim"
            }
            CodeN => "No, Provider is Not Allowed to Release Data",
            OnFileAtPayorOrAtPlanSponsor => "On file at Payor or at Plan Sponsor",
            CodeY => {
                "Yes, Provider has a Signed Statement Permitting Release of Medical Billing Data Related to a Claim"
            }
        }
    }
    fn from_description(description: &str) -> Option<ReleaseOfInformationCode> {
        {
            use ReleaseOfInformationCode::*;
            match description {
                "Appropriate Release of Information on File at Health Care Service Provider or at Utilization Review Organization" => {
                    Some(
                        AppropriateReleaseOfInformationOnFileAtHealthCareServiceProviderOrAtUtilizationReviewOrganization,
                    )
                }
                "Informed Consent to Release Medical Information for Conditions or Diagnoses Regulated by Federal Statutes" => {
                    Some(
                        InformedConsentToReleaseMedicalInformationForConditionsOrDiagnosesRegulatedByFederalStatutes,
                    )
                }
                "The Provider has Limited or Restricted Ability to Release Data Related to a Claim" => {
                    Some(
                        TheProviderHasLimitedOrRestrictedAbilityToReleaseDataRelatedToAClaim,
                    )
                }
                "No, Provider is Not Allowed to Release Data" => Some(CodeN),
                "On file at Payor or at Plan Sponsor" => {
                    Some(OnFileAtPayorOrAtPlanSponsor)
                }
                "Yes, Provider has a Signed Statement Permitting Release of Medical Billing Data Related to a Claim" => {
                    Some(CodeY)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for ReleaseOfInformationCode {
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
    type Value = ReleaseOfInformationCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Release of Information Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReleaseOfInformationCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Release of Information Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ReleaseOfInformationCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Release of Information Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ReleaseOfInformationCode {
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