use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**935

See docs at <https://www.stedi.com/edi/x12/element/935>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementSignificanceCode {
    ///01 - Where Air = 1
    Code01,
    ///02 - Where Butyl Acetate = 1
    Code02,
    ///03 - Approximately
    Approximately,
    ///04 - Equal to
    EqualTo,
    ///05 - Greater than or equal to
    GreaterThanOrEqualTo,
    ///06 - Greater than
    GreaterThan,
    ///07 - Less than
    LessThan,
    ///08 - Less than or equal to
    LessThanOrEqualTo,
    ///09 - Where H2O = 1 or Water = 1
    Code09,
    ///10 - Not equal to
    NotEqualTo,
    ///11 - Corrected to 60 Degrees Fahrenheit
    CorrectedTo60DegreesFahrenheit,
    ///12 - Where Toluene = 1
    Code12,
    ///13 - Vapor in Air
    VaporInAir,
    ///14 - Vapor in Other Than Air
    VaporInOtherThanAir,
    ///15 - Standard Temperature and Pressure
    StandardTemperatureAndPressure,
    ///16 - Conditions Other Than Standard Temperature and Pressure
    ConditionsOtherThanStandardTemperatureAndPressure,
    ///17 - In Ethyl Alcohol
    InEthylAlcohol,
    ///18 - In Ethyl Ether
    InEthylEther,
    ///19 - In Water
    InWater,
    ///20 - At 1 Atmosphere Pressure
    At1AtmospherePressure,
    ///21 - Where Ether = 1
    Code21,
    ///22 - Actual
    Actual,
    ///23 - Predicted
    Predicted,
    ///24 - Air-dried Basis
    AirDriedBasis,
    ///25 - As-received Basis
    AsReceivedBasis,
    ///26 - Dry Basis
    DryBasis,
    ///27 - Equilibrium Basis
    EquilibriumBasis,
    ///28 - Moisture and Ash-Free Basis
    MoistureAndAshFreeBasis,
    ///29 - Oxidizing Atmosphere
    OxidizingAtmosphere,
    ///30 - Reducing Atmosphere
    ReducingAtmosphere,
    ///31 - Calculated
    Calculated,
    ///32 - Scaled Weight
    ScaledWeight,
    ///34 - Ratchet
    Ratchet,
    ///35 - Saturated Vapor
    SaturatedVapor,
    ///36 - Unconditional
    Unconditional,
    ///37 - Short-term
    ShortTerm,
    ///38 - Time-weighted
    TimeWeighted,
    ///39 - Corrected
    Corrected,
    ///40 - Uncorrected
    Uncorrected,
    ///41 - Off Peak
    OffPeak,
    ///42 - On Peak
    OnPeak,
    ///43 - Intermediate
    Intermediate,
    ///44 - Average
    Average,
    ///45 - Per Gallon
    PerGallon,
    ///46 - Estimated
    Estimated,
    ///47 - Minimum
    Minimum,
    ///49 - Mist
    Mist,
    ///50 - Predominant
    Predominant,
    ///51 - Total
    Total,
    ///52 - Cost
    Cost,
    ///53 - Tenant
    Tenant,
    ///54 - Owner
    Owner,
    ///55 - For Sale
    ForSale,
    ///56 - Real Estate Owned or Corporate Owned
    RealEstateOwnedOrCorporateOwned,
    ///57 - Boarded or Blocked Up
    BoardedOrBlockedUp,
    ///58 - Planned
    Planned,
    ///59 - Completed
    Completed,
    ///60 - Sold
    Sold,
    ///61 - Rented
    Rented,
    ///62 - Current
    Current,
    ///63 - Current List
    CurrentList,
    ///64 - Effective
    Effective,
    ///65 - List When Sold
    ListWhenSold,
    ///66 - Sales
    Sales,
    ///67 - Final List
    FinalList,
    ///68 - As Is
    AsIs,
    ///69 - As Repaired or Improved
    AsRepairedOrImproved,
    ///70 - Instantaneous
    Instantaneous,
    ///71 - Low
    Low,
    ///72 - Low to Good
    LowToGood,
    ///73 - Low to High
    LowToHigh,
    ///74 - Low to Medium
    LowToMedium,
    ///75 - Low to Moderate
    LowToModerate,
    ///76 - Medium
    Medium,
    ///77 - Medium to Good
    MediumToGood,
    ///78 - Medium to High
    MediumToHigh,
    ///79 - Moderate
    Moderate,
    ///80 - Moderate to Good
    ModerateToGood,
    ///81 - Moderate to High
    ModerateToHigh,
    ///82 - Moderate to Medium
    ModerateToMedium,
    ///83 - Good
    Good,
    ///84 - Good to High
    GoodToHigh,
    ///85 - High
    High,
    ///86 - Budgeted
    Budgeted,
    ///87 - Forecast
    Forecast,
    ///88 - Adjusted
    Adjusted,
    ///89 - Allocated
    Allocated,
    ///90 - Increasing
    Increasing,
    ///91 - Stable
    Stable,
    ///92 - Declining
    Declining,
    ///93 - Previous
    Previous,
    ///94 - Potential
    Potential,
    ///95 - Modeled
    Modeled,
    ///96 - Measured
    Measured,
    ///97 - Maximum
    Maximum,
    ///98 - Regulated
    Regulated,
    ///99 - Spring
    Spring,
    ///AA - Summer On-peak
    SummerOnPeak,
    ///AB - Summer Mid-peak
    SummerMidPeak,
    ///AC - Summer Off-peak
    SummerOffPeak,
    ///AD - Summer Super On-peak
    SummerSuperOnPeak,
    ///AE - Summer Super Off-peak
    SummerSuperOffPeak,
    ///AF - Winter On-peak
    WinterOnPeak,
    ///AG - Winter Mid-peak
    WinterMidPeak,
    ///AH - Winter Off-peak
    WinterOffPeak,
    ///AI - Winter Super On-peak
    WinterSuperOnPeak,
    ///AJ - Winter Super Off-peak
    WinterSuperOffPeak,
    ///AK - Summer Day
    SummerDay,
    ///AL - Summer Night
    SummerNight,
    ///AM - Winter Day
    WinterDay,
    ///AN - Winter Night
    WinterNight,
    ///AO - Summer
    Summer,
    ///AP - Winter
    Winter,
    ///AQ - Day
    Day,
    ///AR - Night
    Night,
    ///AS - Peak-2
    Peak2,
    ///AT - Peak-3
    Peak3,
    ///AU - Peak-4
    Peak4,
    ///AV - Shoulder
    Shoulder,
    ///AW - Non Time Related Demand
    NonTimeRelatedDemand,
    ///AX - Fall
    Fall,
    ///AY - Summer On Peak-2
    SummerOnPeak2,
    ///AZ - Winter On Peak-2
    WinterOnPeak2,
    ///BA - Probable Contamination
    ProbableContamination,
    ///BB - Not Confirmed
    NotConfirmed,
    ///BC - Tentative Identification
    TentativeIdentification,
    ///BD - Failed
    Failed,
    ///BE - Summer Mid Peak-2
    SummerMidPeak2,
    ///BF - Winter Mid Peak-2
    WinterMidPeak2,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl MeasurementSignificanceCode {
    pub fn code(&self) -> &str {
        {
            use MeasurementSignificanceCode::*;
            match self {
                Code01 => "01",
                Code02 => "02",
                Approximately => "03",
                EqualTo => "04",
                GreaterThanOrEqualTo => "05",
                GreaterThan => "06",
                LessThan => "07",
                LessThanOrEqualTo => "08",
                Code09 => "09",
                NotEqualTo => "10",
                CorrectedTo60DegreesFahrenheit => "11",
                Code12 => "12",
                VaporInAir => "13",
                VaporInOtherThanAir => "14",
                StandardTemperatureAndPressure => "15",
                ConditionsOtherThanStandardTemperatureAndPressure => "16",
                InEthylAlcohol => "17",
                InEthylEther => "18",
                InWater => "19",
                At1AtmospherePressure => "20",
                Code21 => "21",
                Actual => "22",
                Predicted => "23",
                AirDriedBasis => "24",
                AsReceivedBasis => "25",
                DryBasis => "26",
                EquilibriumBasis => "27",
                MoistureAndAshFreeBasis => "28",
                OxidizingAtmosphere => "29",
                ReducingAtmosphere => "30",
                Calculated => "31",
                ScaledWeight => "32",
                Ratchet => "34",
                SaturatedVapor => "35",
                Unconditional => "36",
                ShortTerm => "37",
                TimeWeighted => "38",
                Corrected => "39",
                Uncorrected => "40",
                OffPeak => "41",
                OnPeak => "42",
                Intermediate => "43",
                Average => "44",
                PerGallon => "45",
                Estimated => "46",
                Minimum => "47",
                Mist => "49",
                Predominant => "50",
                Total => "51",
                Cost => "52",
                Tenant => "53",
                Owner => "54",
                ForSale => "55",
                RealEstateOwnedOrCorporateOwned => "56",
                BoardedOrBlockedUp => "57",
                Planned => "58",
                Completed => "59",
                Sold => "60",
                Rented => "61",
                Current => "62",
                CurrentList => "63",
                Effective => "64",
                ListWhenSold => "65",
                Sales => "66",
                FinalList => "67",
                AsIs => "68",
                AsRepairedOrImproved => "69",
                Instantaneous => "70",
                Low => "71",
                LowToGood => "72",
                LowToHigh => "73",
                LowToMedium => "74",
                LowToModerate => "75",
                Medium => "76",
                MediumToGood => "77",
                MediumToHigh => "78",
                Moderate => "79",
                ModerateToGood => "80",
                ModerateToHigh => "81",
                ModerateToMedium => "82",
                Good => "83",
                GoodToHigh => "84",
                High => "85",
                Budgeted => "86",
                Forecast => "87",
                Adjusted => "88",
                Allocated => "89",
                Increasing => "90",
                Stable => "91",
                Declining => "92",
                Previous => "93",
                Potential => "94",
                Modeled => "95",
                Measured => "96",
                Maximum => "97",
                Regulated => "98",
                Spring => "99",
                SummerOnPeak => "AA",
                SummerMidPeak => "AB",
                SummerOffPeak => "AC",
                SummerSuperOnPeak => "AD",
                SummerSuperOffPeak => "AE",
                WinterOnPeak => "AF",
                WinterMidPeak => "AG",
                WinterOffPeak => "AH",
                WinterSuperOnPeak => "AI",
                WinterSuperOffPeak => "AJ",
                SummerDay => "AK",
                SummerNight => "AL",
                WinterDay => "AM",
                WinterNight => "AN",
                Summer => "AO",
                Winter => "AP",
                Day => "AQ",
                Night => "AR",
                Peak2 => "AS",
                Peak3 => "AT",
                Peak4 => "AU",
                Shoulder => "AV",
                NonTimeRelatedDemand => "AW",
                Fall => "AX",
                SummerOnPeak2 => "AY",
                WinterOnPeak2 => "AZ",
                ProbableContamination => "BA",
                NotConfirmed => "BB",
                TentativeIdentification => "BC",
                Failed => "BD",
                SummerMidPeak2 => "BE",
                WinterMidPeak2 => "BF",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<MeasurementSignificanceCode> {
        use MeasurementSignificanceCode::*;
        match code {
            b"01" => Some(Code01),
            b"02" => Some(Code02),
            b"03" => Some(Approximately),
            b"04" => Some(EqualTo),
            b"05" => Some(GreaterThanOrEqualTo),
            b"06" => Some(GreaterThan),
            b"07" => Some(LessThan),
            b"08" => Some(LessThanOrEqualTo),
            b"09" => Some(Code09),
            b"10" => Some(NotEqualTo),
            b"11" => Some(CorrectedTo60DegreesFahrenheit),
            b"12" => Some(Code12),
            b"13" => Some(VaporInAir),
            b"14" => Some(VaporInOtherThanAir),
            b"15" => Some(StandardTemperatureAndPressure),
            b"16" => Some(ConditionsOtherThanStandardTemperatureAndPressure),
            b"17" => Some(InEthylAlcohol),
            b"18" => Some(InEthylEther),
            b"19" => Some(InWater),
            b"20" => Some(At1AtmospherePressure),
            b"21" => Some(Code21),
            b"22" => Some(Actual),
            b"23" => Some(Predicted),
            b"24" => Some(AirDriedBasis),
            b"25" => Some(AsReceivedBasis),
            b"26" => Some(DryBasis),
            b"27" => Some(EquilibriumBasis),
            b"28" => Some(MoistureAndAshFreeBasis),
            b"29" => Some(OxidizingAtmosphere),
            b"30" => Some(ReducingAtmosphere),
            b"31" => Some(Calculated),
            b"32" => Some(ScaledWeight),
            b"34" => Some(Ratchet),
            b"35" => Some(SaturatedVapor),
            b"36" => Some(Unconditional),
            b"37" => Some(ShortTerm),
            b"38" => Some(TimeWeighted),
            b"39" => Some(Corrected),
            b"40" => Some(Uncorrected),
            b"41" => Some(OffPeak),
            b"42" => Some(OnPeak),
            b"43" => Some(Intermediate),
            b"44" => Some(Average),
            b"45" => Some(PerGallon),
            b"46" => Some(Estimated),
            b"47" => Some(Minimum),
            b"49" => Some(Mist),
            b"50" => Some(Predominant),
            b"51" => Some(Total),
            b"52" => Some(Cost),
            b"53" => Some(Tenant),
            b"54" => Some(Owner),
            b"55" => Some(ForSale),
            b"56" => Some(RealEstateOwnedOrCorporateOwned),
            b"57" => Some(BoardedOrBlockedUp),
            b"58" => Some(Planned),
            b"59" => Some(Completed),
            b"60" => Some(Sold),
            b"61" => Some(Rented),
            b"62" => Some(Current),
            b"63" => Some(CurrentList),
            b"64" => Some(Effective),
            b"65" => Some(ListWhenSold),
            b"66" => Some(Sales),
            b"67" => Some(FinalList),
            b"68" => Some(AsIs),
            b"69" => Some(AsRepairedOrImproved),
            b"70" => Some(Instantaneous),
            b"71" => Some(Low),
            b"72" => Some(LowToGood),
            b"73" => Some(LowToHigh),
            b"74" => Some(LowToMedium),
            b"75" => Some(LowToModerate),
            b"76" => Some(Medium),
            b"77" => Some(MediumToGood),
            b"78" => Some(MediumToHigh),
            b"79" => Some(Moderate),
            b"80" => Some(ModerateToGood),
            b"81" => Some(ModerateToHigh),
            b"82" => Some(ModerateToMedium),
            b"83" => Some(Good),
            b"84" => Some(GoodToHigh),
            b"85" => Some(High),
            b"86" => Some(Budgeted),
            b"87" => Some(Forecast),
            b"88" => Some(Adjusted),
            b"89" => Some(Allocated),
            b"90" => Some(Increasing),
            b"91" => Some(Stable),
            b"92" => Some(Declining),
            b"93" => Some(Previous),
            b"94" => Some(Potential),
            b"95" => Some(Modeled),
            b"96" => Some(Measured),
            b"97" => Some(Maximum),
            b"98" => Some(Regulated),
            b"99" => Some(Spring),
            b"AA" => Some(SummerOnPeak),
            b"AB" => Some(SummerMidPeak),
            b"AC" => Some(SummerOffPeak),
            b"AD" => Some(SummerSuperOnPeak),
            b"AE" => Some(SummerSuperOffPeak),
            b"AF" => Some(WinterOnPeak),
            b"AG" => Some(WinterMidPeak),
            b"AH" => Some(WinterOffPeak),
            b"AI" => Some(WinterSuperOnPeak),
            b"AJ" => Some(WinterSuperOffPeak),
            b"AK" => Some(SummerDay),
            b"AL" => Some(SummerNight),
            b"AM" => Some(WinterDay),
            b"AN" => Some(WinterNight),
            b"AO" => Some(Summer),
            b"AP" => Some(Winter),
            b"AQ" => Some(Day),
            b"AR" => Some(Night),
            b"AS" => Some(Peak2),
            b"AT" => Some(Peak3),
            b"AU" => Some(Peak4),
            b"AV" => Some(Shoulder),
            b"AW" => Some(NonTimeRelatedDemand),
            b"AX" => Some(Fall),
            b"AY" => Some(SummerOnPeak2),
            b"AZ" => Some(WinterOnPeak2),
            b"BA" => Some(ProbableContamination),
            b"BB" => Some(NotConfirmed),
            b"BC" => Some(TentativeIdentification),
            b"BD" => Some(Failed),
            b"BE" => Some(SummerMidPeak2),
            b"BF" => Some(WinterMidPeak2),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use MeasurementSignificanceCode::*;
        match self {
            Code01 => "Where Air = 1",
            Code02 => "Where Butyl Acetate = 1",
            Approximately => "Approximately",
            EqualTo => "Equal to",
            GreaterThanOrEqualTo => "Greater than or equal to",
            GreaterThan => "Greater than",
            LessThan => "Less than",
            LessThanOrEqualTo => "Less than or equal to",
            Code09 => "Where H2O = 1 or Water = 1",
            NotEqualTo => "Not equal to",
            CorrectedTo60DegreesFahrenheit => "Corrected to 60 Degrees Fahrenheit",
            Code12 => "Where Toluene = 1",
            VaporInAir => "Vapor in Air",
            VaporInOtherThanAir => "Vapor in Other Than Air",
            StandardTemperatureAndPressure => "Standard Temperature and Pressure",
            ConditionsOtherThanStandardTemperatureAndPressure => {
                "Conditions Other Than Standard Temperature and Pressure"
            }
            InEthylAlcohol => "In Ethyl Alcohol",
            InEthylEther => "In Ethyl Ether",
            InWater => "In Water",
            At1AtmospherePressure => "At 1 Atmosphere Pressure",
            Code21 => "Where Ether = 1",
            Actual => "Actual",
            Predicted => "Predicted",
            AirDriedBasis => "Air-dried Basis",
            AsReceivedBasis => "As-received Basis",
            DryBasis => "Dry Basis",
            EquilibriumBasis => "Equilibrium Basis",
            MoistureAndAshFreeBasis => "Moisture and Ash-Free Basis",
            OxidizingAtmosphere => "Oxidizing Atmosphere",
            ReducingAtmosphere => "Reducing Atmosphere",
            Calculated => "Calculated",
            ScaledWeight => "Scaled Weight",
            Ratchet => "Ratchet",
            SaturatedVapor => "Saturated Vapor",
            Unconditional => "Unconditional",
            ShortTerm => "Short-term",
            TimeWeighted => "Time-weighted",
            Corrected => "Corrected",
            Uncorrected => "Uncorrected",
            OffPeak => "Off Peak",
            OnPeak => "On Peak",
            Intermediate => "Intermediate",
            Average => "Average",
            PerGallon => "Per Gallon",
            Estimated => "Estimated",
            Minimum => "Minimum",
            Mist => "Mist",
            Predominant => "Predominant",
            Total => "Total",
            Cost => "Cost",
            Tenant => "Tenant",
            Owner => "Owner",
            ForSale => "For Sale",
            RealEstateOwnedOrCorporateOwned => "Real Estate Owned or Corporate Owned",
            BoardedOrBlockedUp => "Boarded or Blocked Up",
            Planned => "Planned",
            Completed => "Completed",
            Sold => "Sold",
            Rented => "Rented",
            Current => "Current",
            CurrentList => "Current List",
            Effective => "Effective",
            ListWhenSold => "List When Sold",
            Sales => "Sales",
            FinalList => "Final List",
            AsIs => "As Is",
            AsRepairedOrImproved => "As Repaired or Improved",
            Instantaneous => "Instantaneous",
            Low => "Low",
            LowToGood => "Low to Good",
            LowToHigh => "Low to High",
            LowToMedium => "Low to Medium",
            LowToModerate => "Low to Moderate",
            Medium => "Medium",
            MediumToGood => "Medium to Good",
            MediumToHigh => "Medium to High",
            Moderate => "Moderate",
            ModerateToGood => "Moderate to Good",
            ModerateToHigh => "Moderate to High",
            ModerateToMedium => "Moderate to Medium",
            Good => "Good",
            GoodToHigh => "Good to High",
            High => "High",
            Budgeted => "Budgeted",
            Forecast => "Forecast",
            Adjusted => "Adjusted",
            Allocated => "Allocated",
            Increasing => "Increasing",
            Stable => "Stable",
            Declining => "Declining",
            Previous => "Previous",
            Potential => "Potential",
            Modeled => "Modeled",
            Measured => "Measured",
            Maximum => "Maximum",
            Regulated => "Regulated",
            Spring => "Spring",
            SummerOnPeak => "Summer On-peak",
            SummerMidPeak => "Summer Mid-peak",
            SummerOffPeak => "Summer Off-peak",
            SummerSuperOnPeak => "Summer Super On-peak",
            SummerSuperOffPeak => "Summer Super Off-peak",
            WinterOnPeak => "Winter On-peak",
            WinterMidPeak => "Winter Mid-peak",
            WinterOffPeak => "Winter Off-peak",
            WinterSuperOnPeak => "Winter Super On-peak",
            WinterSuperOffPeak => "Winter Super Off-peak",
            SummerDay => "Summer Day",
            SummerNight => "Summer Night",
            WinterDay => "Winter Day",
            WinterNight => "Winter Night",
            Summer => "Summer",
            Winter => "Winter",
            Day => "Day",
            Night => "Night",
            Peak2 => "Peak-2",
            Peak3 => "Peak-3",
            Peak4 => "Peak-4",
            Shoulder => "Shoulder",
            NonTimeRelatedDemand => "Non Time Related Demand",
            Fall => "Fall",
            SummerOnPeak2 => "Summer On Peak-2",
            WinterOnPeak2 => "Winter On Peak-2",
            ProbableContamination => "Probable Contamination",
            NotConfirmed => "Not Confirmed",
            TentativeIdentification => "Tentative Identification",
            Failed => "Failed",
            SummerMidPeak2 => "Summer Mid Peak-2",
            WinterMidPeak2 => "Winter Mid Peak-2",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<MeasurementSignificanceCode> {
        {
            use MeasurementSignificanceCode::*;
            match description {
                "Where Air = 1" => Some(Code01),
                "Where Butyl Acetate = 1" => Some(Code02),
                "Approximately" => Some(Approximately),
                "Equal to" => Some(EqualTo),
                "Greater than or equal to" => Some(GreaterThanOrEqualTo),
                "Greater than" => Some(GreaterThan),
                "Less than" => Some(LessThan),
                "Less than or equal to" => Some(LessThanOrEqualTo),
                "Where H2O = 1 or Water = 1" => Some(Code09),
                "Not equal to" => Some(NotEqualTo),
                "Corrected to 60 Degrees Fahrenheit" => {
                    Some(CorrectedTo60DegreesFahrenheit)
                }
                "Where Toluene = 1" => Some(Code12),
                "Vapor in Air" => Some(VaporInAir),
                "Vapor in Other Than Air" => Some(VaporInOtherThanAir),
                "Standard Temperature and Pressure" => {
                    Some(StandardTemperatureAndPressure)
                }
                "Conditions Other Than Standard Temperature and Pressure" => {
                    Some(ConditionsOtherThanStandardTemperatureAndPressure)
                }
                "In Ethyl Alcohol" => Some(InEthylAlcohol),
                "In Ethyl Ether" => Some(InEthylEther),
                "In Water" => Some(InWater),
                "At 1 Atmosphere Pressure" => Some(At1AtmospherePressure),
                "Where Ether = 1" => Some(Code21),
                "Actual" => Some(Actual),
                "Predicted" => Some(Predicted),
                "Air-dried Basis" => Some(AirDriedBasis),
                "As-received Basis" => Some(AsReceivedBasis),
                "Dry Basis" => Some(DryBasis),
                "Equilibrium Basis" => Some(EquilibriumBasis),
                "Moisture and Ash-Free Basis" => Some(MoistureAndAshFreeBasis),
                "Oxidizing Atmosphere" => Some(OxidizingAtmosphere),
                "Reducing Atmosphere" => Some(ReducingAtmosphere),
                "Calculated" => Some(Calculated),
                "Scaled Weight" => Some(ScaledWeight),
                "Ratchet" => Some(Ratchet),
                "Saturated Vapor" => Some(SaturatedVapor),
                "Unconditional" => Some(Unconditional),
                "Short-term" => Some(ShortTerm),
                "Time-weighted" => Some(TimeWeighted),
                "Corrected" => Some(Corrected),
                "Uncorrected" => Some(Uncorrected),
                "Off Peak" => Some(OffPeak),
                "On Peak" => Some(OnPeak),
                "Intermediate" => Some(Intermediate),
                "Average" => Some(Average),
                "Per Gallon" => Some(PerGallon),
                "Estimated" => Some(Estimated),
                "Minimum" => Some(Minimum),
                "Mist" => Some(Mist),
                "Predominant" => Some(Predominant),
                "Total" => Some(Total),
                "Cost" => Some(Cost),
                "Tenant" => Some(Tenant),
                "Owner" => Some(Owner),
                "For Sale" => Some(ForSale),
                "Real Estate Owned or Corporate Owned" => {
                    Some(RealEstateOwnedOrCorporateOwned)
                }
                "Boarded or Blocked Up" => Some(BoardedOrBlockedUp),
                "Planned" => Some(Planned),
                "Completed" => Some(Completed),
                "Sold" => Some(Sold),
                "Rented" => Some(Rented),
                "Current" => Some(Current),
                "Current List" => Some(CurrentList),
                "Effective" => Some(Effective),
                "List When Sold" => Some(ListWhenSold),
                "Sales" => Some(Sales),
                "Final List" => Some(FinalList),
                "As Is" => Some(AsIs),
                "As Repaired or Improved" => Some(AsRepairedOrImproved),
                "Instantaneous" => Some(Instantaneous),
                "Low" => Some(Low),
                "Low to Good" => Some(LowToGood),
                "Low to High" => Some(LowToHigh),
                "Low to Medium" => Some(LowToMedium),
                "Low to Moderate" => Some(LowToModerate),
                "Medium" => Some(Medium),
                "Medium to Good" => Some(MediumToGood),
                "Medium to High" => Some(MediumToHigh),
                "Moderate" => Some(Moderate),
                "Moderate to Good" => Some(ModerateToGood),
                "Moderate to High" => Some(ModerateToHigh),
                "Moderate to Medium" => Some(ModerateToMedium),
                "Good" => Some(Good),
                "Good to High" => Some(GoodToHigh),
                "High" => Some(High),
                "Budgeted" => Some(Budgeted),
                "Forecast" => Some(Forecast),
                "Adjusted" => Some(Adjusted),
                "Allocated" => Some(Allocated),
                "Increasing" => Some(Increasing),
                "Stable" => Some(Stable),
                "Declining" => Some(Declining),
                "Previous" => Some(Previous),
                "Potential" => Some(Potential),
                "Modeled" => Some(Modeled),
                "Measured" => Some(Measured),
                "Maximum" => Some(Maximum),
                "Regulated" => Some(Regulated),
                "Spring" => Some(Spring),
                "Summer On-peak" => Some(SummerOnPeak),
                "Summer Mid-peak" => Some(SummerMidPeak),
                "Summer Off-peak" => Some(SummerOffPeak),
                "Summer Super On-peak" => Some(SummerSuperOnPeak),
                "Summer Super Off-peak" => Some(SummerSuperOffPeak),
                "Winter On-peak" => Some(WinterOnPeak),
                "Winter Mid-peak" => Some(WinterMidPeak),
                "Winter Off-peak" => Some(WinterOffPeak),
                "Winter Super On-peak" => Some(WinterSuperOnPeak),
                "Winter Super Off-peak" => Some(WinterSuperOffPeak),
                "Summer Day" => Some(SummerDay),
                "Summer Night" => Some(SummerNight),
                "Winter Day" => Some(WinterDay),
                "Winter Night" => Some(WinterNight),
                "Summer" => Some(Summer),
                "Winter" => Some(Winter),
                "Day" => Some(Day),
                "Night" => Some(Night),
                "Peak-2" => Some(Peak2),
                "Peak-3" => Some(Peak3),
                "Peak-4" => Some(Peak4),
                "Shoulder" => Some(Shoulder),
                "Non Time Related Demand" => Some(NonTimeRelatedDemand),
                "Fall" => Some(Fall),
                "Summer On Peak-2" => Some(SummerOnPeak2),
                "Winter On Peak-2" => Some(WinterOnPeak2),
                "Probable Contamination" => Some(ProbableContamination),
                "Not Confirmed" => Some(NotConfirmed),
                "Tentative Identification" => Some(TentativeIdentification),
                "Failed" => Some(Failed),
                "Summer Mid Peak-2" => Some(SummerMidPeak2),
                "Winter Mid Peak-2" => Some(WinterMidPeak2),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for MeasurementSignificanceCode {
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
    type Value = MeasurementSignificanceCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Measurement Significance Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementSignificanceCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Measurement Significance Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementSignificanceCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Measurement Significance Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for MeasurementSignificanceCode {
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