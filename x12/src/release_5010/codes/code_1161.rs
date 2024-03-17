use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1161

See docs at <https://www.stedi.com/edi/x12-005010/element/1161>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProductOptionCode {
    ///1 - Pretax
    Pretax,
    ///2 - Post-tax
    PostTax,
    ///3 - Qualified
    Qualified,
    ///4 - Non-qualified
    NonQualified,
    ///5 - 401K
    Code5,
    ///6 - Individual Retirement Account
    IndividualRetirementAccount,
    ///7 - Keogh
    Keogh,
    ///8 - Simplified Employee Pension
    SimplifiedEmployeePension,
    ///9 - Single Premium
    SinglePremium,
    ///10 - Flexible Premium
    FlexiblePremium,
    ///11 - Variable Premium
    VariablePremium,
    ///12 - Fixed Premium
    FixedPremium,
    ///13 - Registered under the Income Tax Act of Canada
    RegisteredUnderTheIncomeTaxActOfCanada,
    ///14 - Non-Registered under the Income Tax Act of Canada
    NonRegisteredUnderTheIncomeTaxActOfCanada,
    ///15 - Registered Spousal Case
    RegisteredSpousalCase,
    ///28 - Exclusive
    Exclusive,
    ///29 - Shopped
    Shopped,
    ///30 - Lead Reinsurer
    LeadReinsurer,
    ///31 - Facultative Excess
    FacultativeExcess,
    ///A - First to Die
    FirstToDie,
    ///B - Last to Die
    LastToDie,
    ///BA - Bank Account
    BankAccount,
    ///C - Child Rider
    ChildRider,
    ///D - Discontinue One-Bill Submission
    DiscontinueOneBillSubmission,
    ///GA - Government Allocation
    GovernmentAllocation,
    ///N - Benefit Continuation
    BenefitContinuation,
    ///NC - One-Bill Submission Not Chosen
    OneBillSubmissionNotChosen,
    ///O - One-Bill Submission
    OneBillSubmission,
    ///PD - Payroll Deduction
    PayrollDeduction,
    ///S - Salary Continuation
    SalaryContinuation,
}
impl ProductOptionCode {
    pub fn code(&self) -> &str {
        {
            use ProductOptionCode::*;
            match self {
                Pretax => "1",
                PostTax => "2",
                Qualified => "3",
                NonQualified => "4",
                Code5 => "5",
                IndividualRetirementAccount => "6",
                Keogh => "7",
                SimplifiedEmployeePension => "8",
                SinglePremium => "9",
                FlexiblePremium => "10",
                VariablePremium => "11",
                FixedPremium => "12",
                RegisteredUnderTheIncomeTaxActOfCanada => "13",
                NonRegisteredUnderTheIncomeTaxActOfCanada => "14",
                RegisteredSpousalCase => "15",
                Exclusive => "28",
                Shopped => "29",
                LeadReinsurer => "30",
                FacultativeExcess => "31",
                FirstToDie => "A",
                LastToDie => "B",
                BankAccount => "BA",
                ChildRider => "C",
                DiscontinueOneBillSubmission => "D",
                GovernmentAllocation => "GA",
                BenefitContinuation => "N",
                OneBillSubmissionNotChosen => "NC",
                OneBillSubmission => "O",
                PayrollDeduction => "PD",
                SalaryContinuation => "S",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ProductOptionCode> {
        use ProductOptionCode::*;
        match code {
            b"1" => Some(Pretax),
            b"2" => Some(PostTax),
            b"3" => Some(Qualified),
            b"4" => Some(NonQualified),
            b"5" => Some(Code5),
            b"6" => Some(IndividualRetirementAccount),
            b"7" => Some(Keogh),
            b"8" => Some(SimplifiedEmployeePension),
            b"9" => Some(SinglePremium),
            b"10" => Some(FlexiblePremium),
            b"11" => Some(VariablePremium),
            b"12" => Some(FixedPremium),
            b"13" => Some(RegisteredUnderTheIncomeTaxActOfCanada),
            b"14" => Some(NonRegisteredUnderTheIncomeTaxActOfCanada),
            b"15" => Some(RegisteredSpousalCase),
            b"28" => Some(Exclusive),
            b"29" => Some(Shopped),
            b"30" => Some(LeadReinsurer),
            b"31" => Some(FacultativeExcess),
            b"A" => Some(FirstToDie),
            b"B" => Some(LastToDie),
            b"BA" => Some(BankAccount),
            b"C" => Some(ChildRider),
            b"D" => Some(DiscontinueOneBillSubmission),
            b"GA" => Some(GovernmentAllocation),
            b"N" => Some(BenefitContinuation),
            b"NC" => Some(OneBillSubmissionNotChosen),
            b"O" => Some(OneBillSubmission),
            b"PD" => Some(PayrollDeduction),
            b"S" => Some(SalaryContinuation),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ProductOptionCode::*;
        match self {
            Pretax => "Pretax",
            PostTax => "Post-tax",
            Qualified => "Qualified",
            NonQualified => "Non-qualified",
            Code5 => "401K",
            IndividualRetirementAccount => "Individual Retirement Account",
            Keogh => "Keogh",
            SimplifiedEmployeePension => "Simplified Employee Pension",
            SinglePremium => "Single Premium",
            FlexiblePremium => "Flexible Premium",
            VariablePremium => "Variable Premium",
            FixedPremium => "Fixed Premium",
            RegisteredUnderTheIncomeTaxActOfCanada => {
                "Registered under the Income Tax Act of Canada"
            }
            NonRegisteredUnderTheIncomeTaxActOfCanada => {
                "Non-Registered under the Income Tax Act of Canada"
            }
            RegisteredSpousalCase => "Registered Spousal Case",
            Exclusive => "Exclusive",
            Shopped => "Shopped",
            LeadReinsurer => "Lead Reinsurer",
            FacultativeExcess => "Facultative Excess",
            FirstToDie => "First to Die",
            LastToDie => "Last to Die",
            BankAccount => "Bank Account",
            ChildRider => "Child Rider",
            DiscontinueOneBillSubmission => "Discontinue One-Bill Submission",
            GovernmentAllocation => "Government Allocation",
            BenefitContinuation => "Benefit Continuation",
            OneBillSubmissionNotChosen => "One-Bill Submission Not Chosen",
            OneBillSubmission => "One-Bill Submission",
            PayrollDeduction => "Payroll Deduction",
            SalaryContinuation => "Salary Continuation",
        }
    }
    fn from_description(description: &str) -> Option<ProductOptionCode> {
        {
            use ProductOptionCode::*;
            match description {
                "Pretax" => Some(Pretax),
                "Post-tax" => Some(PostTax),
                "Qualified" => Some(Qualified),
                "Non-qualified" => Some(NonQualified),
                "401K" => Some(Code5),
                "Individual Retirement Account" => Some(IndividualRetirementAccount),
                "Keogh" => Some(Keogh),
                "Simplified Employee Pension" => Some(SimplifiedEmployeePension),
                "Single Premium" => Some(SinglePremium),
                "Flexible Premium" => Some(FlexiblePremium),
                "Variable Premium" => Some(VariablePremium),
                "Fixed Premium" => Some(FixedPremium),
                "Registered under the Income Tax Act of Canada" => {
                    Some(RegisteredUnderTheIncomeTaxActOfCanada)
                }
                "Non-Registered under the Income Tax Act of Canada" => {
                    Some(NonRegisteredUnderTheIncomeTaxActOfCanada)
                }
                "Registered Spousal Case" => Some(RegisteredSpousalCase),
                "Exclusive" => Some(Exclusive),
                "Shopped" => Some(Shopped),
                "Lead Reinsurer" => Some(LeadReinsurer),
                "Facultative Excess" => Some(FacultativeExcess),
                "First to Die" => Some(FirstToDie),
                "Last to Die" => Some(LastToDie),
                "Bank Account" => Some(BankAccount),
                "Child Rider" => Some(ChildRider),
                "Discontinue One-Bill Submission" => Some(DiscontinueOneBillSubmission),
                "Government Allocation" => Some(GovernmentAllocation),
                "Benefit Continuation" => Some(BenefitContinuation),
                "One-Bill Submission Not Chosen" => Some(OneBillSubmissionNotChosen),
                "One-Bill Submission" => Some(OneBillSubmission),
                "Payroll Deduction" => Some(PayrollDeduction),
                "Salary Continuation" => Some(SalaryContinuation),
                _ => None,
            }
        }
    }
}
impl Serialize for ProductOptionCode {
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
    type Value = ProductOptionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Product Option Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProductOptionCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Product Option Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProductOptionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Product Option Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ProductOptionCode {
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