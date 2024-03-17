use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1336

See docs at <https://www.stedi.com/edi/x12/element/1336>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InsuranceTypeCode {
    ///12 - Medicare Secondary Working Aged Beneficiary or Spouse with Employer Group Health Plan
    MedicareSecondaryWorkingAgedBeneficiaryOrSpouseWithEmployerGroupHealthPlan,
    ///13 - Medicare Secondary End-Stage Renal Disease Beneficiary in the Mandated Coordination Period with an Employer's Group Health Plan
    MedicareSecondaryEndStageRenalDiseaseBeneficiaryInTheMandatedCoordinationPeriodWithAnEmployersGroupHealthPlan,
    ///14 - Medicare Secondary, No-fault Insurance including Auto is Primary
    Code14,
    ///15 - Medicare Secondary Worker's Compensation
    MedicareSecondaryWorkersCompensation,
    ///16 - Medicare Secondary Public Health Service (PHS)or Other Federal Agency
    Code16,
    ///41 - Medicare Secondary Black Lung
    MedicareSecondaryBlackLung,
    ///42 - Medicare Secondary Veteran's Administration
    MedicareSecondaryVeteransAdministration,
    ///43 - Medicare Secondary Disabled Beneficiary Under Age 65 with Large Group Health Plan (LGHP)
    Code43,
    ///47 - Medicare Secondary, Other Liability Insurance is Primary
    Code47,
    ///AP - Auto Insurance Policy
    AutoInsurancePolicy,
    ///C1 - Commercial
    Commercial,
    ///CO - Consolidated Omnibus Budget Reconciliation Act (COBRA)
    CodeCO,
    ///CP - Medicare Conditionally Primary
    MedicareConditionallyPrimary,
    ///D - Disability
    Disability,
    ///DB - Disability Benefits
    DisabilityBenefits,
    ///E - Medicare - Point of Service (POS)
    CodeE,
    ///EP - Exclusive Provider Organization
    ExclusiveProviderOrganization,
    ///FF - Family or Friends
    FamilyOrFriends,
    ///GP - Group Policy
    GroupPolicy,
    ///HM - Health Maintenance Organization (HMO)
    CodeHM,
    ///HN - Health Maintenance Organization (HMO) - Medicare Risk
    CodeHN,
    ///HS - Special Low Income Medicare Beneficiary
    SpecialLowIncomeMedicareBeneficiary,
    ///IN - Indemnity
    Indemnity,
    ///IP - Individual Policy
    IndividualPolicy,
    ///LC - Long Term Care
    LongTermCare,
    ///LD - Long Term Policy
    LongTermPolicy,
    ///LI - Life Insurance
    LifeInsurance,
    ///LT - Litigation
    Litigation,
    ///MA - Medicare Part A
    MedicarePartA,
    ///MB - Medicare Part B
    MedicarePartB,
    ///MC - Medicaid
    Medicaid,
    ///MH - Medigap Part A
    MedigapPartA,
    ///MI - Medigap Part B
    MedigapPartB,
    ///MP - Medicare Primary
    MedicarePrimary,
    ///OT - Other
    Other,
    ///PE - Property Insurance - Personal
    PropertyInsurancePersonal,
    ///PL - Personal
    Personal,
    ///PP - Personal Payment (Cash - No Insurance)
    CodePP,
    ///PR - Preferred Provider Organization (PPO)
    CodePR,
    ///PS - Point of Service (POS)
    CodePS,
    ///QM - Qualified Medicare Beneficiary
    QualifiedMedicareBeneficiary,
    ///RP - Property Insurance - Real
    PropertyInsuranceReal,
    ///SP - Supplemental Policy
    SupplementalPolicy,
    ///TF - Tax Equity Fiscal Responsibility Act (TEFRA)
    CodeTF,
    ///U - Multiple Options Health Plan
    MultipleOptionsHealthPlan,
    ///WC - Workers Compensation
    WorkersCompensation,
    ///WU - Wrap Up Policy
    WrapUpPolicy,
}
impl InsuranceTypeCode {
    pub fn code(&self) -> &str {
        {
            use InsuranceTypeCode::*;
            match self {
                MedicareSecondaryWorkingAgedBeneficiaryOrSpouseWithEmployerGroupHealthPlan => {
                    "12"
                }
                MedicareSecondaryEndStageRenalDiseaseBeneficiaryInTheMandatedCoordinationPeriodWithAnEmployersGroupHealthPlan => {
                    "13"
                }
                Code14 => "14",
                MedicareSecondaryWorkersCompensation => "15",
                Code16 => "16",
                MedicareSecondaryBlackLung => "41",
                MedicareSecondaryVeteransAdministration => "42",
                Code43 => "43",
                Code47 => "47",
                AutoInsurancePolicy => "AP",
                Commercial => "C1",
                CodeCO => "CO",
                MedicareConditionallyPrimary => "CP",
                Disability => "D",
                DisabilityBenefits => "DB",
                CodeE => "E",
                ExclusiveProviderOrganization => "EP",
                FamilyOrFriends => "FF",
                GroupPolicy => "GP",
                CodeHM => "HM",
                CodeHN => "HN",
                SpecialLowIncomeMedicareBeneficiary => "HS",
                Indemnity => "IN",
                IndividualPolicy => "IP",
                LongTermCare => "LC",
                LongTermPolicy => "LD",
                LifeInsurance => "LI",
                Litigation => "LT",
                MedicarePartA => "MA",
                MedicarePartB => "MB",
                Medicaid => "MC",
                MedigapPartA => "MH",
                MedigapPartB => "MI",
                MedicarePrimary => "MP",
                Other => "OT",
                PropertyInsurancePersonal => "PE",
                Personal => "PL",
                CodePP => "PP",
                CodePR => "PR",
                CodePS => "PS",
                QualifiedMedicareBeneficiary => "QM",
                PropertyInsuranceReal => "RP",
                SupplementalPolicy => "SP",
                CodeTF => "TF",
                MultipleOptionsHealthPlan => "U",
                WorkersCompensation => "WC",
                WrapUpPolicy => "WU",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<InsuranceTypeCode> {
        use InsuranceTypeCode::*;
        match code {
            b"12" => {
                Some(
                    MedicareSecondaryWorkingAgedBeneficiaryOrSpouseWithEmployerGroupHealthPlan,
                )
            }
            b"13" => {
                Some(
                    MedicareSecondaryEndStageRenalDiseaseBeneficiaryInTheMandatedCoordinationPeriodWithAnEmployersGroupHealthPlan,
                )
            }
            b"14" => Some(Code14),
            b"15" => Some(MedicareSecondaryWorkersCompensation),
            b"16" => Some(Code16),
            b"41" => Some(MedicareSecondaryBlackLung),
            b"42" => Some(MedicareSecondaryVeteransAdministration),
            b"43" => Some(Code43),
            b"47" => Some(Code47),
            b"AP" => Some(AutoInsurancePolicy),
            b"C1" => Some(Commercial),
            b"CO" => Some(CodeCO),
            b"CP" => Some(MedicareConditionallyPrimary),
            b"D" => Some(Disability),
            b"DB" => Some(DisabilityBenefits),
            b"E" => Some(CodeE),
            b"EP" => Some(ExclusiveProviderOrganization),
            b"FF" => Some(FamilyOrFriends),
            b"GP" => Some(GroupPolicy),
            b"HM" => Some(CodeHM),
            b"HN" => Some(CodeHN),
            b"HS" => Some(SpecialLowIncomeMedicareBeneficiary),
            b"IN" => Some(Indemnity),
            b"IP" => Some(IndividualPolicy),
            b"LC" => Some(LongTermCare),
            b"LD" => Some(LongTermPolicy),
            b"LI" => Some(LifeInsurance),
            b"LT" => Some(Litigation),
            b"MA" => Some(MedicarePartA),
            b"MB" => Some(MedicarePartB),
            b"MC" => Some(Medicaid),
            b"MH" => Some(MedigapPartA),
            b"MI" => Some(MedigapPartB),
            b"MP" => Some(MedicarePrimary),
            b"OT" => Some(Other),
            b"PE" => Some(PropertyInsurancePersonal),
            b"PL" => Some(Personal),
            b"PP" => Some(CodePP),
            b"PR" => Some(CodePR),
            b"PS" => Some(CodePS),
            b"QM" => Some(QualifiedMedicareBeneficiary),
            b"RP" => Some(PropertyInsuranceReal),
            b"SP" => Some(SupplementalPolicy),
            b"TF" => Some(CodeTF),
            b"U" => Some(MultipleOptionsHealthPlan),
            b"WC" => Some(WorkersCompensation),
            b"WU" => Some(WrapUpPolicy),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use InsuranceTypeCode::*;
        match self {
            MedicareSecondaryWorkingAgedBeneficiaryOrSpouseWithEmployerGroupHealthPlan => {
                "Medicare Secondary Working Aged Beneficiary or Spouse with Employer Group Health Plan"
            }
            MedicareSecondaryEndStageRenalDiseaseBeneficiaryInTheMandatedCoordinationPeriodWithAnEmployersGroupHealthPlan => {
                "Medicare Secondary End-Stage Renal Disease Beneficiary in the Mandated Coordination Period with an Employer's Group Health Plan"
            }
            Code14 => "Medicare Secondary, No-fault Insurance including Auto is Primary",
            MedicareSecondaryWorkersCompensation => {
                "Medicare Secondary Worker's Compensation"
            }
            Code16 => {
                "Medicare Secondary Public Health Service (PHS)or Other Federal Agency"
            }
            MedicareSecondaryBlackLung => "Medicare Secondary Black Lung",
            MedicareSecondaryVeteransAdministration => {
                "Medicare Secondary Veteran's Administration"
            }
            Code43 => {
                "Medicare Secondary Disabled Beneficiary Under Age 65 with Large Group Health Plan (LGHP)"
            }
            Code47 => "Medicare Secondary, Other Liability Insurance is Primary",
            AutoInsurancePolicy => "Auto Insurance Policy",
            Commercial => "Commercial",
            CodeCO => "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
            MedicareConditionallyPrimary => "Medicare Conditionally Primary",
            Disability => "Disability",
            DisabilityBenefits => "Disability Benefits",
            CodeE => "Medicare - Point of Service (POS)",
            ExclusiveProviderOrganization => "Exclusive Provider Organization",
            FamilyOrFriends => "Family or Friends",
            GroupPolicy => "Group Policy",
            CodeHM => "Health Maintenance Organization (HMO)",
            CodeHN => "Health Maintenance Organization (HMO) - Medicare Risk",
            SpecialLowIncomeMedicareBeneficiary => {
                "Special Low Income Medicare Beneficiary"
            }
            Indemnity => "Indemnity",
            IndividualPolicy => "Individual Policy",
            LongTermCare => "Long Term Care",
            LongTermPolicy => "Long Term Policy",
            LifeInsurance => "Life Insurance",
            Litigation => "Litigation",
            MedicarePartA => "Medicare Part A",
            MedicarePartB => "Medicare Part B",
            Medicaid => "Medicaid",
            MedigapPartA => "Medigap Part A",
            MedigapPartB => "Medigap Part B",
            MedicarePrimary => "Medicare Primary",
            Other => "Other",
            PropertyInsurancePersonal => "Property Insurance - Personal",
            Personal => "Personal",
            CodePP => "Personal Payment (Cash - No Insurance)",
            CodePR => "Preferred Provider Organization (PPO)",
            CodePS => "Point of Service (POS)",
            QualifiedMedicareBeneficiary => "Qualified Medicare Beneficiary",
            PropertyInsuranceReal => "Property Insurance - Real",
            SupplementalPolicy => "Supplemental Policy",
            CodeTF => "Tax Equity Fiscal Responsibility Act (TEFRA)",
            MultipleOptionsHealthPlan => "Multiple Options Health Plan",
            WorkersCompensation => "Workers Compensation",
            WrapUpPolicy => "Wrap Up Policy",
        }
    }
    fn from_description(description: &str) -> Option<InsuranceTypeCode> {
        {
            use InsuranceTypeCode::*;
            match description {
                "Medicare Secondary Working Aged Beneficiary or Spouse with Employer Group Health Plan" => {
                    Some(
                        MedicareSecondaryWorkingAgedBeneficiaryOrSpouseWithEmployerGroupHealthPlan,
                    )
                }
                "Medicare Secondary End-Stage Renal Disease Beneficiary in the Mandated Coordination Period with an Employer's Group Health Plan" => {
                    Some(
                        MedicareSecondaryEndStageRenalDiseaseBeneficiaryInTheMandatedCoordinationPeriodWithAnEmployersGroupHealthPlan,
                    )
                }
                "Medicare Secondary, No-fault Insurance including Auto is Primary" => {
                    Some(Code14)
                }
                "Medicare Secondary Worker's Compensation" => {
                    Some(MedicareSecondaryWorkersCompensation)
                }
                "Medicare Secondary Public Health Service (PHS)or Other Federal Agency" => {
                    Some(Code16)
                }
                "Medicare Secondary Black Lung" => Some(MedicareSecondaryBlackLung),
                "Medicare Secondary Veteran's Administration" => {
                    Some(MedicareSecondaryVeteransAdministration)
                }
                "Medicare Secondary Disabled Beneficiary Under Age 65 with Large Group Health Plan (LGHP)" => {
                    Some(Code43)
                }
                "Medicare Secondary, Other Liability Insurance is Primary" => {
                    Some(Code47)
                }
                "Auto Insurance Policy" => Some(AutoInsurancePolicy),
                "Commercial" => Some(Commercial),
                "Consolidated Omnibus Budget Reconciliation Act (COBRA)" => Some(CodeCO),
                "Medicare Conditionally Primary" => Some(MedicareConditionallyPrimary),
                "Disability" => Some(Disability),
                "Disability Benefits" => Some(DisabilityBenefits),
                "Medicare - Point of Service (POS)" => Some(CodeE),
                "Exclusive Provider Organization" => Some(ExclusiveProviderOrganization),
                "Family or Friends" => Some(FamilyOrFriends),
                "Group Policy" => Some(GroupPolicy),
                "Health Maintenance Organization (HMO)" => Some(CodeHM),
                "Health Maintenance Organization (HMO) - Medicare Risk" => Some(CodeHN),
                "Special Low Income Medicare Beneficiary" => {
                    Some(SpecialLowIncomeMedicareBeneficiary)
                }
                "Indemnity" => Some(Indemnity),
                "Individual Policy" => Some(IndividualPolicy),
                "Long Term Care" => Some(LongTermCare),
                "Long Term Policy" => Some(LongTermPolicy),
                "Life Insurance" => Some(LifeInsurance),
                "Litigation" => Some(Litigation),
                "Medicare Part A" => Some(MedicarePartA),
                "Medicare Part B" => Some(MedicarePartB),
                "Medicaid" => Some(Medicaid),
                "Medigap Part A" => Some(MedigapPartA),
                "Medigap Part B" => Some(MedigapPartB),
                "Medicare Primary" => Some(MedicarePrimary),
                "Other" => Some(Other),
                "Property Insurance - Personal" => Some(PropertyInsurancePersonal),
                "Personal" => Some(Personal),
                "Personal Payment (Cash - No Insurance)" => Some(CodePP),
                "Preferred Provider Organization (PPO)" => Some(CodePR),
                "Point of Service (POS)" => Some(CodePS),
                "Qualified Medicare Beneficiary" => Some(QualifiedMedicareBeneficiary),
                "Property Insurance - Real" => Some(PropertyInsuranceReal),
                "Supplemental Policy" => Some(SupplementalPolicy),
                "Tax Equity Fiscal Responsibility Act (TEFRA)" => Some(CodeTF),
                "Multiple Options Health Plan" => Some(MultipleOptionsHealthPlan),
                "Workers Compensation" => Some(WorkersCompensation),
                "Wrap Up Policy" => Some(WrapUpPolicy),
                _ => None,
            }
        }
    }
}
impl Serialize for InsuranceTypeCode {
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
    type Value = InsuranceTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Insurance Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        InsuranceTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Insurance Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        InsuranceTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Insurance Type Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for InsuranceTypeCode {
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