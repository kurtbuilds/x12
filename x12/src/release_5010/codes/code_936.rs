use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**936

See docs at <https://www.stedi.com/edi/x12/element/936>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementAttributeCode {
    ///01 - Clear
    Clear,
    ///02 - Hazy
    Hazy,
    ///03 - Excess
    Excess,
    ///04 - Some
    Some_,
    ///05 - Undetectable
    Undetectable,
    ///06 - Trace
    Trace,
    ///07 - Yes
    Yes,
    ///08 - Closed
    Closed,
    ///09 - Pass
    Pass,
    ///10 - Present
    Present,
    ///11 - Gel
    Gel,
    ///12 - OK
    Ok,
    ///13 - Slight
    Slight,
    ///14 - No Good
    NoGood,
    ///15 - Marginal
    Marginal,
    ///16 - Nil
    Nil,
    ///17 - Oil Free
    OilFree,
    ///18 - Open
    Open,
    ///19 - Free
    Free,
    ///20 - No
    No,
    ///21 - Checked
    Checked,
    ///22 - Fail
    Fail,
    ///23 - Absent
    Absent,
    ///24 - Good
    Good,
    ///25 - Fair
    Fair,
    ///26 - Poor
    Poor,
    ///27 - Excellent
    Excellent,
    ///28 - Bright
    Bright,
    ///29 - To Be Determined
    ToBeDetermined,
    ///30 - High
    High,
    ///31 - Negative
    Negative,
    ///32 - Partial
    Partial,
    ///33 - Variable
    Variable,
    ///40 - Balance
    Balance,
    ///41 - Complete
    Complete,
    ///42 - Low
    Low,
    ///44 - Not Applicable
    NotApplicable,
    ///45 - Not Determined
    NotDetermined,
    ///46 - Negligible
    Negligible,
    ///48 - Moderate
    Moderate,
    ///49 - Appreciable
    Appreciable,
    ///50 - Not Available
    NotAvailable,
    ///51 - Conforming
    Conforming,
    ///52 - Non-conforming
    NonConforming,
    ///53 - Probable Contamination
    ProbableContamination,
    ///54 - Tentative Identification
    TentativeIdentification,
    ///56 - Detected; Not Quantified
    Code56,
    ///BA - Backer
    Backer,
    ///FL - Full
    Full,
    ///NA - Not Analyzed
    NotAnalyzed,
    ///ND - Not Detected
    NotDetected,
    ///NS - Not Sampled
    NotSampled,
    ///PR - Present and Not Counted
    PresentAndNotCounted,
    ///Q1 - First Quality
    FirstQuality,
    ///Q2 - Second Quality
    SecondQuality,
    ///TA - Too Numerous to Count
    TooNumerousToCount,
    ///TB - New
    New,
    ///WS - Washcoat
    Washcoat,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl MeasurementAttributeCode {
    pub fn code(&self) -> &str {
        {
            use MeasurementAttributeCode::*;
            match self {
                Clear => "01",
                Hazy => "02",
                Excess => "03",
                Some_ => "04",
                Undetectable => "05",
                Trace => "06",
                Yes => "07",
                Closed => "08",
                Pass => "09",
                Present => "10",
                Gel => "11",
                Ok => "12",
                Slight => "13",
                NoGood => "14",
                Marginal => "15",
                Nil => "16",
                OilFree => "17",
                Open => "18",
                Free => "19",
                No => "20",
                Checked => "21",
                Fail => "22",
                Absent => "23",
                Good => "24",
                Fair => "25",
                Poor => "26",
                Excellent => "27",
                Bright => "28",
                ToBeDetermined => "29",
                High => "30",
                Negative => "31",
                Partial => "32",
                Variable => "33",
                Balance => "40",
                Complete => "41",
                Low => "42",
                NotApplicable => "44",
                NotDetermined => "45",
                Negligible => "46",
                Moderate => "48",
                Appreciable => "49",
                NotAvailable => "50",
                Conforming => "51",
                NonConforming => "52",
                ProbableContamination => "53",
                TentativeIdentification => "54",
                Code56 => "56",
                Backer => "BA",
                Full => "FL",
                NotAnalyzed => "NA",
                NotDetected => "ND",
                NotSampled => "NS",
                PresentAndNotCounted => "PR",
                FirstQuality => "Q1",
                SecondQuality => "Q2",
                TooNumerousToCount => "TA",
                New => "TB",
                Washcoat => "WS",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<MeasurementAttributeCode> {
        use MeasurementAttributeCode::*;
        match code {
            b"01" => Some(Clear),
            b"02" => Some(Hazy),
            b"03" => Some(Excess),
            b"04" => Some(Some_),
            b"05" => Some(Undetectable),
            b"06" => Some(Trace),
            b"07" => Some(Yes),
            b"08" => Some(Closed),
            b"09" => Some(Pass),
            b"10" => Some(Present),
            b"11" => Some(Gel),
            b"12" => Some(Ok),
            b"13" => Some(Slight),
            b"14" => Some(NoGood),
            b"15" => Some(Marginal),
            b"16" => Some(Nil),
            b"17" => Some(OilFree),
            b"18" => Some(Open),
            b"19" => Some(Free),
            b"20" => Some(No),
            b"21" => Some(Checked),
            b"22" => Some(Fail),
            b"23" => Some(Absent),
            b"24" => Some(Good),
            b"25" => Some(Fair),
            b"26" => Some(Poor),
            b"27" => Some(Excellent),
            b"28" => Some(Bright),
            b"29" => Some(ToBeDetermined),
            b"30" => Some(High),
            b"31" => Some(Negative),
            b"32" => Some(Partial),
            b"33" => Some(Variable),
            b"40" => Some(Balance),
            b"41" => Some(Complete),
            b"42" => Some(Low),
            b"44" => Some(NotApplicable),
            b"45" => Some(NotDetermined),
            b"46" => Some(Negligible),
            b"48" => Some(Moderate),
            b"49" => Some(Appreciable),
            b"50" => Some(NotAvailable),
            b"51" => Some(Conforming),
            b"52" => Some(NonConforming),
            b"53" => Some(ProbableContamination),
            b"54" => Some(TentativeIdentification),
            b"56" => Some(Code56),
            b"BA" => Some(Backer),
            b"FL" => Some(Full),
            b"NA" => Some(NotAnalyzed),
            b"ND" => Some(NotDetected),
            b"NS" => Some(NotSampled),
            b"PR" => Some(PresentAndNotCounted),
            b"Q1" => Some(FirstQuality),
            b"Q2" => Some(SecondQuality),
            b"TA" => Some(TooNumerousToCount),
            b"TB" => Some(New),
            b"WS" => Some(Washcoat),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use MeasurementAttributeCode::*;
        match self {
            Clear => "Clear",
            Hazy => "Hazy",
            Excess => "Excess",
            Some_ => "Some",
            Undetectable => "Undetectable",
            Trace => "Trace",
            Yes => "Yes",
            Closed => "Closed",
            Pass => "Pass",
            Present => "Present",
            Gel => "Gel",
            Ok => "OK",
            Slight => "Slight",
            NoGood => "No Good",
            Marginal => "Marginal",
            Nil => "Nil",
            OilFree => "Oil Free",
            Open => "Open",
            Free => "Free",
            No => "No",
            Checked => "Checked",
            Fail => "Fail",
            Absent => "Absent",
            Good => "Good",
            Fair => "Fair",
            Poor => "Poor",
            Excellent => "Excellent",
            Bright => "Bright",
            ToBeDetermined => "To Be Determined",
            High => "High",
            Negative => "Negative",
            Partial => "Partial",
            Variable => "Variable",
            Balance => "Balance",
            Complete => "Complete",
            Low => "Low",
            NotApplicable => "Not Applicable",
            NotDetermined => "Not Determined",
            Negligible => "Negligible",
            Moderate => "Moderate",
            Appreciable => "Appreciable",
            NotAvailable => "Not Available",
            Conforming => "Conforming",
            NonConforming => "Non-conforming",
            ProbableContamination => "Probable Contamination",
            TentativeIdentification => "Tentative Identification",
            Code56 => "Detected; Not Quantified",
            Backer => "Backer",
            Full => "Full",
            NotAnalyzed => "Not Analyzed",
            NotDetected => "Not Detected",
            NotSampled => "Not Sampled",
            PresentAndNotCounted => "Present and Not Counted",
            FirstQuality => "First Quality",
            SecondQuality => "Second Quality",
            TooNumerousToCount => "Too Numerous to Count",
            New => "New",
            Washcoat => "Washcoat",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<MeasurementAttributeCode> {
        {
            use MeasurementAttributeCode::*;
            match description {
                "Clear" => Some(Clear),
                "Hazy" => Some(Hazy),
                "Excess" => Some(Excess),
                "Some" => Some(Some_),
                "Undetectable" => Some(Undetectable),
                "Trace" => Some(Trace),
                "Yes" => Some(Yes),
                "Closed" => Some(Closed),
                "Pass" => Some(Pass),
                "Present" => Some(Present),
                "Gel" => Some(Gel),
                "OK" => Some(Ok),
                "Slight" => Some(Slight),
                "No Good" => Some(NoGood),
                "Marginal" => Some(Marginal),
                "Nil" => Some(Nil),
                "Oil Free" => Some(OilFree),
                "Open" => Some(Open),
                "Free" => Some(Free),
                "No" => Some(No),
                "Checked" => Some(Checked),
                "Fail" => Some(Fail),
                "Absent" => Some(Absent),
                "Good" => Some(Good),
                "Fair" => Some(Fair),
                "Poor" => Some(Poor),
                "Excellent" => Some(Excellent),
                "Bright" => Some(Bright),
                "To Be Determined" => Some(ToBeDetermined),
                "High" => Some(High),
                "Negative" => Some(Negative),
                "Partial" => Some(Partial),
                "Variable" => Some(Variable),
                "Balance" => Some(Balance),
                "Complete" => Some(Complete),
                "Low" => Some(Low),
                "Not Applicable" => Some(NotApplicable),
                "Not Determined" => Some(NotDetermined),
                "Negligible" => Some(Negligible),
                "Moderate" => Some(Moderate),
                "Appreciable" => Some(Appreciable),
                "Not Available" => Some(NotAvailable),
                "Conforming" => Some(Conforming),
                "Non-conforming" => Some(NonConforming),
                "Probable Contamination" => Some(ProbableContamination),
                "Tentative Identification" => Some(TentativeIdentification),
                "Detected; Not Quantified" => Some(Code56),
                "Backer" => Some(Backer),
                "Full" => Some(Full),
                "Not Analyzed" => Some(NotAnalyzed),
                "Not Detected" => Some(NotDetected),
                "Not Sampled" => Some(NotSampled),
                "Present and Not Counted" => Some(PresentAndNotCounted),
                "First Quality" => Some(FirstQuality),
                "Second Quality" => Some(SecondQuality),
                "Too Numerous to Count" => Some(TooNumerousToCount),
                "New" => Some(New),
                "Washcoat" => Some(Washcoat),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for MeasurementAttributeCode {
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
    type Value = MeasurementAttributeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Measurement Attribute Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementAttributeCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Measurement Attribute Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementAttributeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Measurement Attribute Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for MeasurementAttributeCode {
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