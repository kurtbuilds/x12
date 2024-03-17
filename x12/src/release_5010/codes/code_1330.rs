use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1330

See docs at <https://www.stedi.com/edi/x12-005010/element/1330>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DosageFormCode {
    ///01 - Combination Forms
    CombinationForms,
    ///02 - Test Kits
    TestKits,
    ///03 - Contraceptive Devices
    ContraceptiveDevices,
    ///04 - Devices
    Devices,
    ///05 - Patch or Disc
    PatchOrDisc,
    ///06 - Chewable (candy) Bar
    Code06,
    ///07 - Dosepak
    Dosepak,
    ///10 - Tablet
    Tablet,
    ///11 - Enteric Coated Tablet
    EntericCoatedTablet,
    ///12 - Sustained Release Tablet
    SustainedReleaseTablet,
    ///13 - Buccal or Sublingual Tablet
    BuccalOrSublingualTablet,
    ///14 - Chewable Tablet
    ChewableTablet,
    ///15 - Soluble Tablet
    SolubleTablet,
    ///16 - Tablet Unspecified
    TabletUnspecified,
    ///20 - Capsule
    Capsule,
    ///21 - Sustained Release Capsule
    SustainedReleaseCapsule,
    ///22 - Capsule Unspecified
    CapsuleUnspecified,
    ///23 - Tablet 21 Day Supply
    Tablet21DaySupply,
    ///24 - Tablet 28 Day Supply
    Tablet28DaySupply,
    ///25 - Enteric Coated Capsule
    EntericCoatedCapsule,
    ///30 - Lozenge or Troche
    LozengeOrTroche,
    ///31 - Internal Powder
    InternalPowder,
    ///32 - Chewing Gum
    ChewingGum,
    ///33 - Granules
    Granules,
    ///34 - Swabs
    Swabs,
    ///40 - Injection
    Injection,
    ///41 - Sustained Release Injection
    SustainedReleaseInjection,
    ///42 - Injectable Unspecified
    InjectableUnspecified,
    ///43 - Injectable Lyophilized Powder
    InjectableLyophilizedPowder,
    ///50 - Ophthalmic
    Ophthalmic,
    ///51 - Ophthalmic Liquid
    OphthalmicLiquid,
    ///52 - Ophthalmic or Otic
    OphthalmicOrOtic,
    ///53 - Ophthalmic Liquid (Compliance Cap)
    Code53,
    ///60 - Elixir
    Elixir,
    ///61 - Suspension
    Suspension,
    ///62 - Syrup
    Syrup,
    ///63 - Solution
    Solution,
    ///64 - Emulsion
    Emulsion,
    ///65 - Drops
    Drops,
    ///66 - Pediatric Liquid
    PediatricLiquid,
    ///67 - Liquid
    Liquid,
    ///68 - Oral, Liquid and Sustained Release
    Code68,
    ///69 - Rectal Cream or Ointment
    RectalCreamOrOintment,
    ///70 - Rectal Suppository
    RectalSuppository,
    ///71 - Vaginal Suppository
    VaginalSuppository,
    ///72 - Vaginal Tablet
    VaginalTablet,
    ///73 - Vaginal Cream
    VaginalCream,
    ///74 - Vaginal Foam
    VaginalFoam,
    ///75 - Urethral Suppository
    UrethralSuppository,
    ///76 - Enema
    Enema,
    ///77 - Douche
    Douche,
    ///78 - Vaginal Ointment
    VaginalOintment,
    ///79 - Contraceptive Sponge
    ContraceptiveSponge,
    ///80 - External Ointment
    ExternalOintment,
    ///81 - External Cream
    ExternalCream,
    ///82 - Dental Product
    DentalProduct,
    ///83 - Aerosol Powder
    AerosolPowder,
    ///84 - Aerosol Spray
    AerosolSpray,
    ///85 - External Liquid
    ExternalLiquid,
    ///86 - External Powder
    ExternalPowder,
    ///87 - Dental Mouth Rinse
    DentalMouthRinse,
    ///88 - Inhalant (Refill Canister Only)
    Code88,
    ///90 - Irrigant
    Irrigant,
    ///91 - Gargle
    Gargle,
    ///92 - Throat Spray and Swabs
    ThroatSprayAndSwabs,
    ///93 - Nasal
    Nasal,
    ///94 - Inhalant
    Inhalant,
    ///95 - Otic
    Otic,
    ///96 - Soap
    Soap,
    ///97 - Stick
    Stick,
    ///98 - Dressing or Bandage
    DressingOrBandage,
    ///99 - Miscellaneous Unspecified
    MiscellaneousUnspecified,
}
impl DosageFormCode {
    pub fn code(&self) -> &str {
        {
            use DosageFormCode::*;
            match self {
                CombinationForms => "01",
                TestKits => "02",
                ContraceptiveDevices => "03",
                Devices => "04",
                PatchOrDisc => "05",
                Code06 => "06",
                Dosepak => "07",
                Tablet => "10",
                EntericCoatedTablet => "11",
                SustainedReleaseTablet => "12",
                BuccalOrSublingualTablet => "13",
                ChewableTablet => "14",
                SolubleTablet => "15",
                TabletUnspecified => "16",
                Capsule => "20",
                SustainedReleaseCapsule => "21",
                CapsuleUnspecified => "22",
                Tablet21DaySupply => "23",
                Tablet28DaySupply => "24",
                EntericCoatedCapsule => "25",
                LozengeOrTroche => "30",
                InternalPowder => "31",
                ChewingGum => "32",
                Granules => "33",
                Swabs => "34",
                Injection => "40",
                SustainedReleaseInjection => "41",
                InjectableUnspecified => "42",
                InjectableLyophilizedPowder => "43",
                Ophthalmic => "50",
                OphthalmicLiquid => "51",
                OphthalmicOrOtic => "52",
                Code53 => "53",
                Elixir => "60",
                Suspension => "61",
                Syrup => "62",
                Solution => "63",
                Emulsion => "64",
                Drops => "65",
                PediatricLiquid => "66",
                Liquid => "67",
                Code68 => "68",
                RectalCreamOrOintment => "69",
                RectalSuppository => "70",
                VaginalSuppository => "71",
                VaginalTablet => "72",
                VaginalCream => "73",
                VaginalFoam => "74",
                UrethralSuppository => "75",
                Enema => "76",
                Douche => "77",
                VaginalOintment => "78",
                ContraceptiveSponge => "79",
                ExternalOintment => "80",
                ExternalCream => "81",
                DentalProduct => "82",
                AerosolPowder => "83",
                AerosolSpray => "84",
                ExternalLiquid => "85",
                ExternalPowder => "86",
                DentalMouthRinse => "87",
                Code88 => "88",
                Irrigant => "90",
                Gargle => "91",
                ThroatSprayAndSwabs => "92",
                Nasal => "93",
                Inhalant => "94",
                Otic => "95",
                Soap => "96",
                Stick => "97",
                DressingOrBandage => "98",
                MiscellaneousUnspecified => "99",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<DosageFormCode> {
        use DosageFormCode::*;
        match code {
            b"01" => Some(CombinationForms),
            b"02" => Some(TestKits),
            b"03" => Some(ContraceptiveDevices),
            b"04" => Some(Devices),
            b"05" => Some(PatchOrDisc),
            b"06" => Some(Code06),
            b"07" => Some(Dosepak),
            b"10" => Some(Tablet),
            b"11" => Some(EntericCoatedTablet),
            b"12" => Some(SustainedReleaseTablet),
            b"13" => Some(BuccalOrSublingualTablet),
            b"14" => Some(ChewableTablet),
            b"15" => Some(SolubleTablet),
            b"16" => Some(TabletUnspecified),
            b"20" => Some(Capsule),
            b"21" => Some(SustainedReleaseCapsule),
            b"22" => Some(CapsuleUnspecified),
            b"23" => Some(Tablet21DaySupply),
            b"24" => Some(Tablet28DaySupply),
            b"25" => Some(EntericCoatedCapsule),
            b"30" => Some(LozengeOrTroche),
            b"31" => Some(InternalPowder),
            b"32" => Some(ChewingGum),
            b"33" => Some(Granules),
            b"34" => Some(Swabs),
            b"40" => Some(Injection),
            b"41" => Some(SustainedReleaseInjection),
            b"42" => Some(InjectableUnspecified),
            b"43" => Some(InjectableLyophilizedPowder),
            b"50" => Some(Ophthalmic),
            b"51" => Some(OphthalmicLiquid),
            b"52" => Some(OphthalmicOrOtic),
            b"53" => Some(Code53),
            b"60" => Some(Elixir),
            b"61" => Some(Suspension),
            b"62" => Some(Syrup),
            b"63" => Some(Solution),
            b"64" => Some(Emulsion),
            b"65" => Some(Drops),
            b"66" => Some(PediatricLiquid),
            b"67" => Some(Liquid),
            b"68" => Some(Code68),
            b"69" => Some(RectalCreamOrOintment),
            b"70" => Some(RectalSuppository),
            b"71" => Some(VaginalSuppository),
            b"72" => Some(VaginalTablet),
            b"73" => Some(VaginalCream),
            b"74" => Some(VaginalFoam),
            b"75" => Some(UrethralSuppository),
            b"76" => Some(Enema),
            b"77" => Some(Douche),
            b"78" => Some(VaginalOintment),
            b"79" => Some(ContraceptiveSponge),
            b"80" => Some(ExternalOintment),
            b"81" => Some(ExternalCream),
            b"82" => Some(DentalProduct),
            b"83" => Some(AerosolPowder),
            b"84" => Some(AerosolSpray),
            b"85" => Some(ExternalLiquid),
            b"86" => Some(ExternalPowder),
            b"87" => Some(DentalMouthRinse),
            b"88" => Some(Code88),
            b"90" => Some(Irrigant),
            b"91" => Some(Gargle),
            b"92" => Some(ThroatSprayAndSwabs),
            b"93" => Some(Nasal),
            b"94" => Some(Inhalant),
            b"95" => Some(Otic),
            b"96" => Some(Soap),
            b"97" => Some(Stick),
            b"98" => Some(DressingOrBandage),
            b"99" => Some(MiscellaneousUnspecified),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use DosageFormCode::*;
        match self {
            CombinationForms => "Combination Forms",
            TestKits => "Test Kits",
            ContraceptiveDevices => "Contraceptive Devices",
            Devices => "Devices",
            PatchOrDisc => "Patch or Disc",
            Code06 => "Chewable (candy) Bar",
            Dosepak => "Dosepak",
            Tablet => "Tablet",
            EntericCoatedTablet => "Enteric Coated Tablet",
            SustainedReleaseTablet => "Sustained Release Tablet",
            BuccalOrSublingualTablet => "Buccal or Sublingual Tablet",
            ChewableTablet => "Chewable Tablet",
            SolubleTablet => "Soluble Tablet",
            TabletUnspecified => "Tablet Unspecified",
            Capsule => "Capsule",
            SustainedReleaseCapsule => "Sustained Release Capsule",
            CapsuleUnspecified => "Capsule Unspecified",
            Tablet21DaySupply => "Tablet 21 Day Supply",
            Tablet28DaySupply => "Tablet 28 Day Supply",
            EntericCoatedCapsule => "Enteric Coated Capsule",
            LozengeOrTroche => "Lozenge or Troche",
            InternalPowder => "Internal Powder",
            ChewingGum => "Chewing Gum",
            Granules => "Granules",
            Swabs => "Swabs",
            Injection => "Injection",
            SustainedReleaseInjection => "Sustained Release Injection",
            InjectableUnspecified => "Injectable Unspecified",
            InjectableLyophilizedPowder => "Injectable Lyophilized Powder",
            Ophthalmic => "Ophthalmic",
            OphthalmicLiquid => "Ophthalmic Liquid",
            OphthalmicOrOtic => "Ophthalmic or Otic",
            Code53 => "Ophthalmic Liquid (Compliance Cap)",
            Elixir => "Elixir",
            Suspension => "Suspension",
            Syrup => "Syrup",
            Solution => "Solution",
            Emulsion => "Emulsion",
            Drops => "Drops",
            PediatricLiquid => "Pediatric Liquid",
            Liquid => "Liquid",
            Code68 => "Oral, Liquid and Sustained Release",
            RectalCreamOrOintment => "Rectal Cream or Ointment",
            RectalSuppository => "Rectal Suppository",
            VaginalSuppository => "Vaginal Suppository",
            VaginalTablet => "Vaginal Tablet",
            VaginalCream => "Vaginal Cream",
            VaginalFoam => "Vaginal Foam",
            UrethralSuppository => "Urethral Suppository",
            Enema => "Enema",
            Douche => "Douche",
            VaginalOintment => "Vaginal Ointment",
            ContraceptiveSponge => "Contraceptive Sponge",
            ExternalOintment => "External Ointment",
            ExternalCream => "External Cream",
            DentalProduct => "Dental Product",
            AerosolPowder => "Aerosol Powder",
            AerosolSpray => "Aerosol Spray",
            ExternalLiquid => "External Liquid",
            ExternalPowder => "External Powder",
            DentalMouthRinse => "Dental Mouth Rinse",
            Code88 => "Inhalant (Refill Canister Only)",
            Irrigant => "Irrigant",
            Gargle => "Gargle",
            ThroatSprayAndSwabs => "Throat Spray and Swabs",
            Nasal => "Nasal",
            Inhalant => "Inhalant",
            Otic => "Otic",
            Soap => "Soap",
            Stick => "Stick",
            DressingOrBandage => "Dressing or Bandage",
            MiscellaneousUnspecified => "Miscellaneous Unspecified",
        }
    }
    fn from_description(description: &str) -> Option<DosageFormCode> {
        {
            use DosageFormCode::*;
            match description {
                "Combination Forms" => Some(CombinationForms),
                "Test Kits" => Some(TestKits),
                "Contraceptive Devices" => Some(ContraceptiveDevices),
                "Devices" => Some(Devices),
                "Patch or Disc" => Some(PatchOrDisc),
                "Chewable (candy) Bar" => Some(Code06),
                "Dosepak" => Some(Dosepak),
                "Tablet" => Some(Tablet),
                "Enteric Coated Tablet" => Some(EntericCoatedTablet),
                "Sustained Release Tablet" => Some(SustainedReleaseTablet),
                "Buccal or Sublingual Tablet" => Some(BuccalOrSublingualTablet),
                "Chewable Tablet" => Some(ChewableTablet),
                "Soluble Tablet" => Some(SolubleTablet),
                "Tablet Unspecified" => Some(TabletUnspecified),
                "Capsule" => Some(Capsule),
                "Sustained Release Capsule" => Some(SustainedReleaseCapsule),
                "Capsule Unspecified" => Some(CapsuleUnspecified),
                "Tablet 21 Day Supply" => Some(Tablet21DaySupply),
                "Tablet 28 Day Supply" => Some(Tablet28DaySupply),
                "Enteric Coated Capsule" => Some(EntericCoatedCapsule),
                "Lozenge or Troche" => Some(LozengeOrTroche),
                "Internal Powder" => Some(InternalPowder),
                "Chewing Gum" => Some(ChewingGum),
                "Granules" => Some(Granules),
                "Swabs" => Some(Swabs),
                "Injection" => Some(Injection),
                "Sustained Release Injection" => Some(SustainedReleaseInjection),
                "Injectable Unspecified" => Some(InjectableUnspecified),
                "Injectable Lyophilized Powder" => Some(InjectableLyophilizedPowder),
                "Ophthalmic" => Some(Ophthalmic),
                "Ophthalmic Liquid" => Some(OphthalmicLiquid),
                "Ophthalmic or Otic" => Some(OphthalmicOrOtic),
                "Ophthalmic Liquid (Compliance Cap)" => Some(Code53),
                "Elixir" => Some(Elixir),
                "Suspension" => Some(Suspension),
                "Syrup" => Some(Syrup),
                "Solution" => Some(Solution),
                "Emulsion" => Some(Emulsion),
                "Drops" => Some(Drops),
                "Pediatric Liquid" => Some(PediatricLiquid),
                "Liquid" => Some(Liquid),
                "Oral, Liquid and Sustained Release" => Some(Code68),
                "Rectal Cream or Ointment" => Some(RectalCreamOrOintment),
                "Rectal Suppository" => Some(RectalSuppository),
                "Vaginal Suppository" => Some(VaginalSuppository),
                "Vaginal Tablet" => Some(VaginalTablet),
                "Vaginal Cream" => Some(VaginalCream),
                "Vaginal Foam" => Some(VaginalFoam),
                "Urethral Suppository" => Some(UrethralSuppository),
                "Enema" => Some(Enema),
                "Douche" => Some(Douche),
                "Vaginal Ointment" => Some(VaginalOintment),
                "Contraceptive Sponge" => Some(ContraceptiveSponge),
                "External Ointment" => Some(ExternalOintment),
                "External Cream" => Some(ExternalCream),
                "Dental Product" => Some(DentalProduct),
                "Aerosol Powder" => Some(AerosolPowder),
                "Aerosol Spray" => Some(AerosolSpray),
                "External Liquid" => Some(ExternalLiquid),
                "External Powder" => Some(ExternalPowder),
                "Dental Mouth Rinse" => Some(DentalMouthRinse),
                "Inhalant (Refill Canister Only)" => Some(Code88),
                "Irrigant" => Some(Irrigant),
                "Gargle" => Some(Gargle),
                "Throat Spray and Swabs" => Some(ThroatSprayAndSwabs),
                "Nasal" => Some(Nasal),
                "Inhalant" => Some(Inhalant),
                "Otic" => Some(Otic),
                "Soap" => Some(Soap),
                "Stick" => Some(Stick),
                "Dressing or Bandage" => Some(DressingOrBandage),
                "Miscellaneous Unspecified" => Some(MiscellaneousUnspecified),
                _ => None,
            }
        }
    }
}
impl Serialize for DosageFormCode {
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
    type Value = DosageFormCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Dosage Form Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DosageFormCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Dosage Form Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DosageFormCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Dosage Form Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for DosageFormCode {
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