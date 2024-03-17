use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1032

See docs at <https://www.stedi.com/edi/x12/element/1032>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClaimFilingIndicatorCode {
    ///01 - Property Conveyance
    PropertyConveyance,
    ///02 - Mortgage Assignment
    MortgageAssignment,
    ///03 - Automatic Mortgage Assignment
    AutomaticMortgageAssignment,
    ///04 - Mortgage Coinsurance
    MortgageCoinsurance,
    ///05 - Supplemental Claim
    SupplementalClaim,
    ///06 - Property Nonconveyance (Claim without Conveyance of Title)
    Code06,
    ///07 - Property Preforeclosure Sale
    PropertyPreforeclosureSale,
    ///08 - Initial Claim
    InitialClaim,
    ///09 - Self-pay
    SelfPay,
    ///10 - Central Certification
    CentralCertification,
    ///11 - Other Non-Federal Programs
    OtherNonFederalPrograms,
    ///12 - Preferred Provider Organization (PPO)
    Code12,
    ///13 - Point of Service (POS)
    Code13,
    ///14 - Exclusive Provider Organization (EPO)
    Code14,
    ///15 - Indemnity Insurance
    IndemnityInsurance,
    ///16 - Health Maintenance Organization (HMO) Medicare Risk
    Code16,
    ///17 - Dental Maintenance Organization
    DentalMaintenanceOrganization,
    ///18 - Deed-in-Lieu Property Sold
    DeedInLieuPropertySold,
    ///19 - Deed-in-Lieu Property Not Sold
    DeedInLieuPropertyNotSold,
    ///20 - Foreclosure Complete Property Sold
    ForeclosureCompletePropertySold,
    ///21 - Foreclosure Complete Property Not Sold
    ForeclosureCompletePropertyNotSold,
    ///22 - Liability Insurance
    LiabilityInsurance,
    ///31 - Special Forbearance
    SpecialForbearance,
    ///32 - Loan Modifications
    LoanModifications,
    ///33 - Partial Claim
    PartialClaim,
    ///34 - Managed Dental
    ManagedDental,
    ///AM - Automobile Medical
    AutomobileMedical,
    ///BL - Blue Cross/Blue Shield
    BlueCrossBlueShield,
    ///CH - TRICARE
    Tricare,
    ///CI - Commercial Insurance Co.
    CommercialInsuranceCo,
    ///CN - Contractual
    Contractual,
    ///DS - Disability
    Disability,
    ///FI - Federal Employees Program
    FederalEmployeesProgram,
    ///HM - Health Maintenance Organization
    HealthMaintenanceOrganization,
    ///LI - Liability
    Liability,
    ///LM - Liability Medical
    LiabilityMedical,
    ///MA - Medicare Part A
    MedicarePartA,
    ///MB - Medicare Part B
    MedicarePartB,
    ///MC - Medicaid
    Medicaid,
    ///MD - Medicare Part D
    MedicarePartD,
    ///ME - Medicare Advantage Plan
    MedicareAdvantagePlan,
    ///MH - Managed Care Non-HMO
    ManagedCareNonHmo,
    ///OF - Other Federal Program
    OtherFederalProgram,
    ///SA - Self-administered Group
    SelfAdministeredGroup,
    ///TV - Title V
    TitleV,
    ///UK - Unknown
    Unknown,
    ///VA - Veterans Affairs Plan
    VeteransAffairsPlan,
    ///WB - Workers' Compensation First Report of Injury
    WorkersCompensationFirstReportOfInjury,
    ///WC - Workers' Compensation Health Claim
    WorkersCompensationHealthClaim,
    ///WD - Workers' Compensation Subsequent Report of Injury
    WorkersCompensationSubsequentReportOfInjury,
    ///WE - Workers' Compensation Combined First and Subsequent Report
    WorkersCompensationCombinedFirstAndSubsequentReport,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl ClaimFilingIndicatorCode {
    pub fn code(&self) -> &str {
        {
            use ClaimFilingIndicatorCode::*;
            match self {
                PropertyConveyance => "01",
                MortgageAssignment => "02",
                AutomaticMortgageAssignment => "03",
                MortgageCoinsurance => "04",
                SupplementalClaim => "05",
                Code06 => "06",
                PropertyPreforeclosureSale => "07",
                InitialClaim => "08",
                SelfPay => "09",
                CentralCertification => "10",
                OtherNonFederalPrograms => "11",
                Code12 => "12",
                Code13 => "13",
                Code14 => "14",
                IndemnityInsurance => "15",
                Code16 => "16",
                DentalMaintenanceOrganization => "17",
                DeedInLieuPropertySold => "18",
                DeedInLieuPropertyNotSold => "19",
                ForeclosureCompletePropertySold => "20",
                ForeclosureCompletePropertyNotSold => "21",
                LiabilityInsurance => "22",
                SpecialForbearance => "31",
                LoanModifications => "32",
                PartialClaim => "33",
                ManagedDental => "34",
                AutomobileMedical => "AM",
                BlueCrossBlueShield => "BL",
                Tricare => "CH",
                CommercialInsuranceCo => "CI",
                Contractual => "CN",
                Disability => "DS",
                FederalEmployeesProgram => "FI",
                HealthMaintenanceOrganization => "HM",
                Liability => "LI",
                LiabilityMedical => "LM",
                MedicarePartA => "MA",
                MedicarePartB => "MB",
                Medicaid => "MC",
                MedicarePartD => "MD",
                MedicareAdvantagePlan => "ME",
                ManagedCareNonHmo => "MH",
                OtherFederalProgram => "OF",
                SelfAdministeredGroup => "SA",
                TitleV => "TV",
                Unknown => "UK",
                VeteransAffairsPlan => "VA",
                WorkersCompensationFirstReportOfInjury => "WB",
                WorkersCompensationHealthClaim => "WC",
                WorkersCompensationSubsequentReportOfInjury => "WD",
                WorkersCompensationCombinedFirstAndSubsequentReport => "WE",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ClaimFilingIndicatorCode> {
        use ClaimFilingIndicatorCode::*;
        match code {
            b"01" => Some(PropertyConveyance),
            b"02" => Some(MortgageAssignment),
            b"03" => Some(AutomaticMortgageAssignment),
            b"04" => Some(MortgageCoinsurance),
            b"05" => Some(SupplementalClaim),
            b"06" => Some(Code06),
            b"07" => Some(PropertyPreforeclosureSale),
            b"08" => Some(InitialClaim),
            b"09" => Some(SelfPay),
            b"10" => Some(CentralCertification),
            b"11" => Some(OtherNonFederalPrograms),
            b"12" => Some(Code12),
            b"13" => Some(Code13),
            b"14" => Some(Code14),
            b"15" => Some(IndemnityInsurance),
            b"16" => Some(Code16),
            b"17" => Some(DentalMaintenanceOrganization),
            b"18" => Some(DeedInLieuPropertySold),
            b"19" => Some(DeedInLieuPropertyNotSold),
            b"20" => Some(ForeclosureCompletePropertySold),
            b"21" => Some(ForeclosureCompletePropertyNotSold),
            b"22" => Some(LiabilityInsurance),
            b"31" => Some(SpecialForbearance),
            b"32" => Some(LoanModifications),
            b"33" => Some(PartialClaim),
            b"34" => Some(ManagedDental),
            b"AM" => Some(AutomobileMedical),
            b"BL" => Some(BlueCrossBlueShield),
            b"CH" => Some(Tricare),
            b"CI" => Some(CommercialInsuranceCo),
            b"CN" => Some(Contractual),
            b"DS" => Some(Disability),
            b"FI" => Some(FederalEmployeesProgram),
            b"HM" => Some(HealthMaintenanceOrganization),
            b"LI" => Some(Liability),
            b"LM" => Some(LiabilityMedical),
            b"MA" => Some(MedicarePartA),
            b"MB" => Some(MedicarePartB),
            b"MC" => Some(Medicaid),
            b"MD" => Some(MedicarePartD),
            b"ME" => Some(MedicareAdvantagePlan),
            b"MH" => Some(ManagedCareNonHmo),
            b"OF" => Some(OtherFederalProgram),
            b"SA" => Some(SelfAdministeredGroup),
            b"TV" => Some(TitleV),
            b"UK" => Some(Unknown),
            b"VA" => Some(VeteransAffairsPlan),
            b"WB" => Some(WorkersCompensationFirstReportOfInjury),
            b"WC" => Some(WorkersCompensationHealthClaim),
            b"WD" => Some(WorkersCompensationSubsequentReportOfInjury),
            b"WE" => Some(WorkersCompensationCombinedFirstAndSubsequentReport),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ClaimFilingIndicatorCode::*;
        match self {
            PropertyConveyance => "Property Conveyance",
            MortgageAssignment => "Mortgage Assignment",
            AutomaticMortgageAssignment => "Automatic Mortgage Assignment",
            MortgageCoinsurance => "Mortgage Coinsurance",
            SupplementalClaim => "Supplemental Claim",
            Code06 => "Property Nonconveyance (Claim without Conveyance of Title)",
            PropertyPreforeclosureSale => "Property Preforeclosure Sale",
            InitialClaim => "Initial Claim",
            SelfPay => "Self-pay",
            CentralCertification => "Central Certification",
            OtherNonFederalPrograms => "Other Non-Federal Programs",
            Code12 => "Preferred Provider Organization (PPO)",
            Code13 => "Point of Service (POS)",
            Code14 => "Exclusive Provider Organization (EPO)",
            IndemnityInsurance => "Indemnity Insurance",
            Code16 => "Health Maintenance Organization (HMO) Medicare Risk",
            DentalMaintenanceOrganization => "Dental Maintenance Organization",
            DeedInLieuPropertySold => "Deed-in-Lieu Property Sold",
            DeedInLieuPropertyNotSold => "Deed-in-Lieu Property Not Sold",
            ForeclosureCompletePropertySold => "Foreclosure Complete Property Sold",
            ForeclosureCompletePropertyNotSold => {
                "Foreclosure Complete Property Not Sold"
            }
            LiabilityInsurance => "Liability Insurance",
            SpecialForbearance => "Special Forbearance",
            LoanModifications => "Loan Modifications",
            PartialClaim => "Partial Claim",
            ManagedDental => "Managed Dental",
            AutomobileMedical => "Automobile Medical",
            BlueCrossBlueShield => "Blue Cross/Blue Shield",
            Tricare => "TRICARE",
            CommercialInsuranceCo => "Commercial Insurance Co.",
            Contractual => "Contractual",
            Disability => "Disability",
            FederalEmployeesProgram => "Federal Employees Program",
            HealthMaintenanceOrganization => "Health Maintenance Organization",
            Liability => "Liability",
            LiabilityMedical => "Liability Medical",
            MedicarePartA => "Medicare Part A",
            MedicarePartB => "Medicare Part B",
            Medicaid => "Medicaid",
            MedicarePartD => "Medicare Part D",
            MedicareAdvantagePlan => "Medicare Advantage Plan",
            ManagedCareNonHmo => "Managed Care Non-HMO",
            OtherFederalProgram => "Other Federal Program",
            SelfAdministeredGroup => "Self-administered Group",
            TitleV => "Title V",
            Unknown => "Unknown",
            VeteransAffairsPlan => "Veterans Affairs Plan",
            WorkersCompensationFirstReportOfInjury => {
                "Workers' Compensation First Report of Injury"
            }
            WorkersCompensationHealthClaim => "Workers' Compensation Health Claim",
            WorkersCompensationSubsequentReportOfInjury => {
                "Workers' Compensation Subsequent Report of Injury"
            }
            WorkersCompensationCombinedFirstAndSubsequentReport => {
                "Workers' Compensation Combined First and Subsequent Report"
            }
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ClaimFilingIndicatorCode> {
        {
            use ClaimFilingIndicatorCode::*;
            match description {
                "Property Conveyance" => Some(PropertyConveyance),
                "Mortgage Assignment" => Some(MortgageAssignment),
                "Automatic Mortgage Assignment" => Some(AutomaticMortgageAssignment),
                "Mortgage Coinsurance" => Some(MortgageCoinsurance),
                "Supplemental Claim" => Some(SupplementalClaim),
                "Property Nonconveyance (Claim without Conveyance of Title)" => {
                    Some(Code06)
                }
                "Property Preforeclosure Sale" => Some(PropertyPreforeclosureSale),
                "Initial Claim" => Some(InitialClaim),
                "Self-pay" => Some(SelfPay),
                "Central Certification" => Some(CentralCertification),
                "Other Non-Federal Programs" => Some(OtherNonFederalPrograms),
                "Preferred Provider Organization (PPO)" => Some(Code12),
                "Point of Service (POS)" => Some(Code13),
                "Exclusive Provider Organization (EPO)" => Some(Code14),
                "Indemnity Insurance" => Some(IndemnityInsurance),
                "Health Maintenance Organization (HMO) Medicare Risk" => Some(Code16),
                "Dental Maintenance Organization" => Some(DentalMaintenanceOrganization),
                "Deed-in-Lieu Property Sold" => Some(DeedInLieuPropertySold),
                "Deed-in-Lieu Property Not Sold" => Some(DeedInLieuPropertyNotSold),
                "Foreclosure Complete Property Sold" => {
                    Some(ForeclosureCompletePropertySold)
                }
                "Foreclosure Complete Property Not Sold" => {
                    Some(ForeclosureCompletePropertyNotSold)
                }
                "Liability Insurance" => Some(LiabilityInsurance),
                "Special Forbearance" => Some(SpecialForbearance),
                "Loan Modifications" => Some(LoanModifications),
                "Partial Claim" => Some(PartialClaim),
                "Managed Dental" => Some(ManagedDental),
                "Automobile Medical" => Some(AutomobileMedical),
                "Blue Cross/Blue Shield" => Some(BlueCrossBlueShield),
                "TRICARE" => Some(Tricare),
                "Commercial Insurance Co." => Some(CommercialInsuranceCo),
                "Contractual" => Some(Contractual),
                "Disability" => Some(Disability),
                "Federal Employees Program" => Some(FederalEmployeesProgram),
                "Health Maintenance Organization" => Some(HealthMaintenanceOrganization),
                "Liability" => Some(Liability),
                "Liability Medical" => Some(LiabilityMedical),
                "Medicare Part A" => Some(MedicarePartA),
                "Medicare Part B" => Some(MedicarePartB),
                "Medicaid" => Some(Medicaid),
                "Medicare Part D" => Some(MedicarePartD),
                "Medicare Advantage Plan" => Some(MedicareAdvantagePlan),
                "Managed Care Non-HMO" => Some(ManagedCareNonHmo),
                "Other Federal Program" => Some(OtherFederalProgram),
                "Self-administered Group" => Some(SelfAdministeredGroup),
                "Title V" => Some(TitleV),
                "Unknown" => Some(Unknown),
                "Veterans Affairs Plan" => Some(VeteransAffairsPlan),
                "Workers' Compensation First Report of Injury" => {
                    Some(WorkersCompensationFirstReportOfInjury)
                }
                "Workers' Compensation Health Claim" => {
                    Some(WorkersCompensationHealthClaim)
                }
                "Workers' Compensation Subsequent Report of Injury" => {
                    Some(WorkersCompensationSubsequentReportOfInjury)
                }
                "Workers' Compensation Combined First and Subsequent Report" => {
                    Some(WorkersCompensationCombinedFirstAndSubsequentReport)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ClaimFilingIndicatorCode {
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
    type Value = ClaimFilingIndicatorCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Claim Filing Indicator Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimFilingIndicatorCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Claim Filing Indicator Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ClaimFilingIndicatorCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Claim Filing Indicator Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ClaimFilingIndicatorCode {
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