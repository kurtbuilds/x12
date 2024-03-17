use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**752

See docs at <https://www.stedi.com/edi/x12/element/752>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurfaceLayerPositionCode {
    ///1S - Side One
    SideOne,
    ///2S - Side Two
    SideTwo,
    ///A1 - Single
    Single,
    ///A2 - Left Front
    LeftFront,
    ///A3 - Right Front
    RightFront,
    ///A4 - Left Rear
    LeftRear,
    ///A5 - Right Rear
    RightRear,
    ///A6 - Inside Left Rear
    InsideLeftRear,
    ///A7 - Outside Left Rear
    OutsideLeftRear,
    ///A8 - Inside Right Rear
    InsideRightRear,
    ///A9 - Outside Right Rear
    OutsideRightRear,
    ///AL - All
    All,
    ///AO - Single End Overhang
    SingleEndOverhang,
    ///AS - Siding
    Siding,
    ///B1 - Bolster
    Bolster,
    ///BC - Back of Cab
    BackOfCab,
    ///BI - Bilateral
    Bilateral,
    ///BK - Rear
    Rear,
    ///BL - Block
    Block,
    ///BR - Brick
    Brick,
    ///BS - Both Sides
    BothSides,
    ///BT - Bottom
    Bottom,
    ///CH - Casing Head Flange
    CasingHeadFlange,
    ///CT - Center
    Center,
    ///DO - Double End Overhang
    DoubleEndOverhang,
    ///DT - Downstream Tap
    DownstreamTap,
    ///DU - Dual Fuel Tank Position
    DualFuelTankPosition,
    ///EX - Exterior
    Exterior,
    ///FR - Front
    Front,
    ///FS - Front Spare
    FrontSpare,
    ///GF - Rig Floor
    RigFloor,
    ///IN - Inside
    Inside,
    ///IT - Interior
    Interior,
    ///KB - Kelly Bushing
    KellyBushing,
    ///LC - Less Critical Surface
    LessCriticalSurface,
    ///LO - Lower
    Lower,
    ///LT - Left
    Left,
    ///M1 - Multiple
    Multiple,
    ///MC - More Critical Surface
    MoreCriticalSurface,
    ///MD - Middle
    Middle,
    ///NS - Not Specified
    NotSpecified,
    ///NT - Next Relative Position
    NextRelativePosition,
    ///OA - Overall
    Overall,
    ///OS - One Side
    OneSide,
    ///OT - Outside
    Outside,
    ///R0 - Relative Position 10
    RelativePosition10,
    ///R1 - Relative Position 1
    RelativePosition1,
    ///R2 - Relative Position 2
    RelativePosition2,
    ///R3 - Relative Position 3
    RelativePosition3,
    ///R4 - Relative Position 4
    RelativePosition4,
    ///R5 - Relative Position 5
    RelativePosition5,
    ///R6 - Relative Position 6
    RelativePosition6,
    ///R7 - Relative Position 7
    RelativePosition7,
    ///R8 - Relative Position 8
    RelativePosition8,
    ///R9 - Relative Position 9
    RelativePosition9,
    ///RA - Relative Position 11
    RelativePosition11,
    ///RB - Relative Position 12
    RelativePosition12,
    ///RC - Relative Position 13
    RelativePosition13,
    ///RD - Relative Position 14
    RelativePosition14,
    ///RE - Relative Position 15
    RelativePosition15,
    ///RF - Relative Position 16
    RelativePosition16,
    ///RG - Relative Position 17
    RelativePosition17,
    ///RH - Relative Position 18
    RelativePosition18,
    ///RI - Relative Position 19
    RelativePosition19,
    ///RJ - Relative Position 20
    RelativePosition20,
    ///RK - Relative Position 21
    RelativePosition21,
    ///RL - Relative Position 22
    RelativePosition22,
    ///RM - Relative Position 23
    RelativePosition23,
    ///RN - Relative Position 24
    RelativePosition24,
    ///RO - Relative Position 25
    RelativePosition25,
    ///RP - Relative Position 26
    RelativePosition26,
    ///RQ - Relative Position 27
    RelativePosition27,
    ///RR - Relative Position 28
    RelativePosition28,
    ///RS - Relative Position 29
    RelativePosition29,
    ///RT - Relative Position 30
    RelativePosition30,
    ///RU - Relative Position 31
    RelativePosition31,
    ///RV - Relative Position 32
    RelativePosition32,
    ///RW - Relative Position 33
    RelativePosition33,
    ///RX - Relative Position 34
    RelativePosition34,
    ///RY - Relative Position 35
    RelativePosition35,
    ///RZ - Relative Position 36
    RelativePosition36,
    ///S1 - Relative Position 37
    RelativePosition37,
    ///S2 - Relative Position 38
    RelativePosition38,
    ///S3 - Relative Position 39
    RelativePosition39,
    ///S4 - Relative Position 40
    RelativePosition40,
    ///S5 - Relative Position 41
    RelativePosition41,
    ///S6 - Relative Position 42
    RelativePosition42,
    ///S7 - Relative Position 43
    RelativePosition43,
    ///S8 - Relative Position 44
    RelativePosition44,
    ///S9 - Relative Position 45
    RelativePosition45,
    ///SA - Relative Position 46
    RelativePosition46,
    ///SB - Right
    Right,
    ///SC - Relative Position 47
    RelativePosition47,
    ///SD - Relative Position 48
    RelativePosition48,
    ///SE - Relative Position 49
    RelativePosition49,
    ///SF - Relative Position 50
    RelativePosition50,
    ///SN - Stone
    Stone,
    ///SP - Spare Tire Position
    SpareTirePosition,
    ///SS - Rear Spare
    RearSpare,
    ///ST - Stucco
    Stucco,
    ///SU - Sub-sea
    SubSea,
    ///TB - Tank Bottom
    TankBottom,
    ///TP - Top
    Top,
    ///TS - Two Sides
    TwoSides,
    ///UC - Under Cab
    UnderCab,
    ///UN - Unilateral
    Unilateral,
    ///UP - Upper
    Upper,
    ///UT - Upstream Tap
    UpstreamTap,
    ///WF - Wood
    Wood,
}
impl SurfaceLayerPositionCode {
    pub fn code(&self) -> &str {
        {
            use SurfaceLayerPositionCode::*;
            match self {
                SideOne => "1S",
                SideTwo => "2S",
                Single => "A1",
                LeftFront => "A2",
                RightFront => "A3",
                LeftRear => "A4",
                RightRear => "A5",
                InsideLeftRear => "A6",
                OutsideLeftRear => "A7",
                InsideRightRear => "A8",
                OutsideRightRear => "A9",
                All => "AL",
                SingleEndOverhang => "AO",
                Siding => "AS",
                Bolster => "B1",
                BackOfCab => "BC",
                Bilateral => "BI",
                Rear => "BK",
                Block => "BL",
                Brick => "BR",
                BothSides => "BS",
                Bottom => "BT",
                CasingHeadFlange => "CH",
                Center => "CT",
                DoubleEndOverhang => "DO",
                DownstreamTap => "DT",
                DualFuelTankPosition => "DU",
                Exterior => "EX",
                Front => "FR",
                FrontSpare => "FS",
                RigFloor => "GF",
                Inside => "IN",
                Interior => "IT",
                KellyBushing => "KB",
                LessCriticalSurface => "LC",
                Lower => "LO",
                Left => "LT",
                Multiple => "M1",
                MoreCriticalSurface => "MC",
                Middle => "MD",
                NotSpecified => "NS",
                NextRelativePosition => "NT",
                Overall => "OA",
                OneSide => "OS",
                Outside => "OT",
                RelativePosition10 => "R0",
                RelativePosition1 => "R1",
                RelativePosition2 => "R2",
                RelativePosition3 => "R3",
                RelativePosition4 => "R4",
                RelativePosition5 => "R5",
                RelativePosition6 => "R6",
                RelativePosition7 => "R7",
                RelativePosition8 => "R8",
                RelativePosition9 => "R9",
                RelativePosition11 => "RA",
                RelativePosition12 => "RB",
                RelativePosition13 => "RC",
                RelativePosition14 => "RD",
                RelativePosition15 => "RE",
                RelativePosition16 => "RF",
                RelativePosition17 => "RG",
                RelativePosition18 => "RH",
                RelativePosition19 => "RI",
                RelativePosition20 => "RJ",
                RelativePosition21 => "RK",
                RelativePosition22 => "RL",
                RelativePosition23 => "RM",
                RelativePosition24 => "RN",
                RelativePosition25 => "RO",
                RelativePosition26 => "RP",
                RelativePosition27 => "RQ",
                RelativePosition28 => "RR",
                RelativePosition29 => "RS",
                RelativePosition30 => "RT",
                RelativePosition31 => "RU",
                RelativePosition32 => "RV",
                RelativePosition33 => "RW",
                RelativePosition34 => "RX",
                RelativePosition35 => "RY",
                RelativePosition36 => "RZ",
                RelativePosition37 => "S1",
                RelativePosition38 => "S2",
                RelativePosition39 => "S3",
                RelativePosition40 => "S4",
                RelativePosition41 => "S5",
                RelativePosition42 => "S6",
                RelativePosition43 => "S7",
                RelativePosition44 => "S8",
                RelativePosition45 => "S9",
                RelativePosition46 => "SA",
                Right => "SB",
                RelativePosition47 => "SC",
                RelativePosition48 => "SD",
                RelativePosition49 => "SE",
                RelativePosition50 => "SF",
                Stone => "SN",
                SpareTirePosition => "SP",
                RearSpare => "SS",
                Stucco => "ST",
                SubSea => "SU",
                TankBottom => "TB",
                Top => "TP",
                TwoSides => "TS",
                UnderCab => "UC",
                Unilateral => "UN",
                Upper => "UP",
                UpstreamTap => "UT",
                Wood => "WF",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<SurfaceLayerPositionCode> {
        use SurfaceLayerPositionCode::*;
        match code {
            b"1S" => Some(SideOne),
            b"2S" => Some(SideTwo),
            b"A1" => Some(Single),
            b"A2" => Some(LeftFront),
            b"A3" => Some(RightFront),
            b"A4" => Some(LeftRear),
            b"A5" => Some(RightRear),
            b"A6" => Some(InsideLeftRear),
            b"A7" => Some(OutsideLeftRear),
            b"A8" => Some(InsideRightRear),
            b"A9" => Some(OutsideRightRear),
            b"AL" => Some(All),
            b"AO" => Some(SingleEndOverhang),
            b"AS" => Some(Siding),
            b"B1" => Some(Bolster),
            b"BC" => Some(BackOfCab),
            b"BI" => Some(Bilateral),
            b"BK" => Some(Rear),
            b"BL" => Some(Block),
            b"BR" => Some(Brick),
            b"BS" => Some(BothSides),
            b"BT" => Some(Bottom),
            b"CH" => Some(CasingHeadFlange),
            b"CT" => Some(Center),
            b"DO" => Some(DoubleEndOverhang),
            b"DT" => Some(DownstreamTap),
            b"DU" => Some(DualFuelTankPosition),
            b"EX" => Some(Exterior),
            b"FR" => Some(Front),
            b"FS" => Some(FrontSpare),
            b"GF" => Some(RigFloor),
            b"IN" => Some(Inside),
            b"IT" => Some(Interior),
            b"KB" => Some(KellyBushing),
            b"LC" => Some(LessCriticalSurface),
            b"LO" => Some(Lower),
            b"LT" => Some(Left),
            b"M1" => Some(Multiple),
            b"MC" => Some(MoreCriticalSurface),
            b"MD" => Some(Middle),
            b"NS" => Some(NotSpecified),
            b"NT" => Some(NextRelativePosition),
            b"OA" => Some(Overall),
            b"OS" => Some(OneSide),
            b"OT" => Some(Outside),
            b"R0" => Some(RelativePosition10),
            b"R1" => Some(RelativePosition1),
            b"R2" => Some(RelativePosition2),
            b"R3" => Some(RelativePosition3),
            b"R4" => Some(RelativePosition4),
            b"R5" => Some(RelativePosition5),
            b"R6" => Some(RelativePosition6),
            b"R7" => Some(RelativePosition7),
            b"R8" => Some(RelativePosition8),
            b"R9" => Some(RelativePosition9),
            b"RA" => Some(RelativePosition11),
            b"RB" => Some(RelativePosition12),
            b"RC" => Some(RelativePosition13),
            b"RD" => Some(RelativePosition14),
            b"RE" => Some(RelativePosition15),
            b"RF" => Some(RelativePosition16),
            b"RG" => Some(RelativePosition17),
            b"RH" => Some(RelativePosition18),
            b"RI" => Some(RelativePosition19),
            b"RJ" => Some(RelativePosition20),
            b"RK" => Some(RelativePosition21),
            b"RL" => Some(RelativePosition22),
            b"RM" => Some(RelativePosition23),
            b"RN" => Some(RelativePosition24),
            b"RO" => Some(RelativePosition25),
            b"RP" => Some(RelativePosition26),
            b"RQ" => Some(RelativePosition27),
            b"RR" => Some(RelativePosition28),
            b"RS" => Some(RelativePosition29),
            b"RT" => Some(RelativePosition30),
            b"RU" => Some(RelativePosition31),
            b"RV" => Some(RelativePosition32),
            b"RW" => Some(RelativePosition33),
            b"RX" => Some(RelativePosition34),
            b"RY" => Some(RelativePosition35),
            b"RZ" => Some(RelativePosition36),
            b"S1" => Some(RelativePosition37),
            b"S2" => Some(RelativePosition38),
            b"S3" => Some(RelativePosition39),
            b"S4" => Some(RelativePosition40),
            b"S5" => Some(RelativePosition41),
            b"S6" => Some(RelativePosition42),
            b"S7" => Some(RelativePosition43),
            b"S8" => Some(RelativePosition44),
            b"S9" => Some(RelativePosition45),
            b"SA" => Some(RelativePosition46),
            b"SB" => Some(Right),
            b"SC" => Some(RelativePosition47),
            b"SD" => Some(RelativePosition48),
            b"SE" => Some(RelativePosition49),
            b"SF" => Some(RelativePosition50),
            b"SN" => Some(Stone),
            b"SP" => Some(SpareTirePosition),
            b"SS" => Some(RearSpare),
            b"ST" => Some(Stucco),
            b"SU" => Some(SubSea),
            b"TB" => Some(TankBottom),
            b"TP" => Some(Top),
            b"TS" => Some(TwoSides),
            b"UC" => Some(UnderCab),
            b"UN" => Some(Unilateral),
            b"UP" => Some(Upper),
            b"UT" => Some(UpstreamTap),
            b"WF" => Some(Wood),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use SurfaceLayerPositionCode::*;
        match self {
            SideOne => "Side One",
            SideTwo => "Side Two",
            Single => "Single",
            LeftFront => "Left Front",
            RightFront => "Right Front",
            LeftRear => "Left Rear",
            RightRear => "Right Rear",
            InsideLeftRear => "Inside Left Rear",
            OutsideLeftRear => "Outside Left Rear",
            InsideRightRear => "Inside Right Rear",
            OutsideRightRear => "Outside Right Rear",
            All => "All",
            SingleEndOverhang => "Single End Overhang",
            Siding => "Siding",
            Bolster => "Bolster",
            BackOfCab => "Back of Cab",
            Bilateral => "Bilateral",
            Rear => "Rear",
            Block => "Block",
            Brick => "Brick",
            BothSides => "Both Sides",
            Bottom => "Bottom",
            CasingHeadFlange => "Casing Head Flange",
            Center => "Center",
            DoubleEndOverhang => "Double End Overhang",
            DownstreamTap => "Downstream Tap",
            DualFuelTankPosition => "Dual Fuel Tank Position",
            Exterior => "Exterior",
            Front => "Front",
            FrontSpare => "Front Spare",
            RigFloor => "Rig Floor",
            Inside => "Inside",
            Interior => "Interior",
            KellyBushing => "Kelly Bushing",
            LessCriticalSurface => "Less Critical Surface",
            Lower => "Lower",
            Left => "Left",
            Multiple => "Multiple",
            MoreCriticalSurface => "More Critical Surface",
            Middle => "Middle",
            NotSpecified => "Not Specified",
            NextRelativePosition => "Next Relative Position",
            Overall => "Overall",
            OneSide => "One Side",
            Outside => "Outside",
            RelativePosition10 => "Relative Position 10",
            RelativePosition1 => "Relative Position 1",
            RelativePosition2 => "Relative Position 2",
            RelativePosition3 => "Relative Position 3",
            RelativePosition4 => "Relative Position 4",
            RelativePosition5 => "Relative Position 5",
            RelativePosition6 => "Relative Position 6",
            RelativePosition7 => "Relative Position 7",
            RelativePosition8 => "Relative Position 8",
            RelativePosition9 => "Relative Position 9",
            RelativePosition11 => "Relative Position 11",
            RelativePosition12 => "Relative Position 12",
            RelativePosition13 => "Relative Position 13",
            RelativePosition14 => "Relative Position 14",
            RelativePosition15 => "Relative Position 15",
            RelativePosition16 => "Relative Position 16",
            RelativePosition17 => "Relative Position 17",
            RelativePosition18 => "Relative Position 18",
            RelativePosition19 => "Relative Position 19",
            RelativePosition20 => "Relative Position 20",
            RelativePosition21 => "Relative Position 21",
            RelativePosition22 => "Relative Position 22",
            RelativePosition23 => "Relative Position 23",
            RelativePosition24 => "Relative Position 24",
            RelativePosition25 => "Relative Position 25",
            RelativePosition26 => "Relative Position 26",
            RelativePosition27 => "Relative Position 27",
            RelativePosition28 => "Relative Position 28",
            RelativePosition29 => "Relative Position 29",
            RelativePosition30 => "Relative Position 30",
            RelativePosition31 => "Relative Position 31",
            RelativePosition32 => "Relative Position 32",
            RelativePosition33 => "Relative Position 33",
            RelativePosition34 => "Relative Position 34",
            RelativePosition35 => "Relative Position 35",
            RelativePosition36 => "Relative Position 36",
            RelativePosition37 => "Relative Position 37",
            RelativePosition38 => "Relative Position 38",
            RelativePosition39 => "Relative Position 39",
            RelativePosition40 => "Relative Position 40",
            RelativePosition41 => "Relative Position 41",
            RelativePosition42 => "Relative Position 42",
            RelativePosition43 => "Relative Position 43",
            RelativePosition44 => "Relative Position 44",
            RelativePosition45 => "Relative Position 45",
            RelativePosition46 => "Relative Position 46",
            Right => "Right",
            RelativePosition47 => "Relative Position 47",
            RelativePosition48 => "Relative Position 48",
            RelativePosition49 => "Relative Position 49",
            RelativePosition50 => "Relative Position 50",
            Stone => "Stone",
            SpareTirePosition => "Spare Tire Position",
            RearSpare => "Rear Spare",
            Stucco => "Stucco",
            SubSea => "Sub-sea",
            TankBottom => "Tank Bottom",
            Top => "Top",
            TwoSides => "Two Sides",
            UnderCab => "Under Cab",
            Unilateral => "Unilateral",
            Upper => "Upper",
            UpstreamTap => "Upstream Tap",
            Wood => "Wood",
        }
    }
    fn from_description(description: &str) -> Option<SurfaceLayerPositionCode> {
        {
            use SurfaceLayerPositionCode::*;
            match description {
                "Side One" => Some(SideOne),
                "Side Two" => Some(SideTwo),
                "Single" => Some(Single),
                "Left Front" => Some(LeftFront),
                "Right Front" => Some(RightFront),
                "Left Rear" => Some(LeftRear),
                "Right Rear" => Some(RightRear),
                "Inside Left Rear" => Some(InsideLeftRear),
                "Outside Left Rear" => Some(OutsideLeftRear),
                "Inside Right Rear" => Some(InsideRightRear),
                "Outside Right Rear" => Some(OutsideRightRear),
                "All" => Some(All),
                "Single End Overhang" => Some(SingleEndOverhang),
                "Siding" => Some(Siding),
                "Bolster" => Some(Bolster),
                "Back of Cab" => Some(BackOfCab),
                "Bilateral" => Some(Bilateral),
                "Rear" => Some(Rear),
                "Block" => Some(Block),
                "Brick" => Some(Brick),
                "Both Sides" => Some(BothSides),
                "Bottom" => Some(Bottom),
                "Casing Head Flange" => Some(CasingHeadFlange),
                "Center" => Some(Center),
                "Double End Overhang" => Some(DoubleEndOverhang),
                "Downstream Tap" => Some(DownstreamTap),
                "Dual Fuel Tank Position" => Some(DualFuelTankPosition),
                "Exterior" => Some(Exterior),
                "Front" => Some(Front),
                "Front Spare" => Some(FrontSpare),
                "Rig Floor" => Some(RigFloor),
                "Inside" => Some(Inside),
                "Interior" => Some(Interior),
                "Kelly Bushing" => Some(KellyBushing),
                "Less Critical Surface" => Some(LessCriticalSurface),
                "Lower" => Some(Lower),
                "Left" => Some(Left),
                "Multiple" => Some(Multiple),
                "More Critical Surface" => Some(MoreCriticalSurface),
                "Middle" => Some(Middle),
                "Not Specified" => Some(NotSpecified),
                "Next Relative Position" => Some(NextRelativePosition),
                "Overall" => Some(Overall),
                "One Side" => Some(OneSide),
                "Outside" => Some(Outside),
                "Relative Position 10" => Some(RelativePosition10),
                "Relative Position 1" => Some(RelativePosition1),
                "Relative Position 2" => Some(RelativePosition2),
                "Relative Position 3" => Some(RelativePosition3),
                "Relative Position 4" => Some(RelativePosition4),
                "Relative Position 5" => Some(RelativePosition5),
                "Relative Position 6" => Some(RelativePosition6),
                "Relative Position 7" => Some(RelativePosition7),
                "Relative Position 8" => Some(RelativePosition8),
                "Relative Position 9" => Some(RelativePosition9),
                "Relative Position 11" => Some(RelativePosition11),
                "Relative Position 12" => Some(RelativePosition12),
                "Relative Position 13" => Some(RelativePosition13),
                "Relative Position 14" => Some(RelativePosition14),
                "Relative Position 15" => Some(RelativePosition15),
                "Relative Position 16" => Some(RelativePosition16),
                "Relative Position 17" => Some(RelativePosition17),
                "Relative Position 18" => Some(RelativePosition18),
                "Relative Position 19" => Some(RelativePosition19),
                "Relative Position 20" => Some(RelativePosition20),
                "Relative Position 21" => Some(RelativePosition21),
                "Relative Position 22" => Some(RelativePosition22),
                "Relative Position 23" => Some(RelativePosition23),
                "Relative Position 24" => Some(RelativePosition24),
                "Relative Position 25" => Some(RelativePosition25),
                "Relative Position 26" => Some(RelativePosition26),
                "Relative Position 27" => Some(RelativePosition27),
                "Relative Position 28" => Some(RelativePosition28),
                "Relative Position 29" => Some(RelativePosition29),
                "Relative Position 30" => Some(RelativePosition30),
                "Relative Position 31" => Some(RelativePosition31),
                "Relative Position 32" => Some(RelativePosition32),
                "Relative Position 33" => Some(RelativePosition33),
                "Relative Position 34" => Some(RelativePosition34),
                "Relative Position 35" => Some(RelativePosition35),
                "Relative Position 36" => Some(RelativePosition36),
                "Relative Position 37" => Some(RelativePosition37),
                "Relative Position 38" => Some(RelativePosition38),
                "Relative Position 39" => Some(RelativePosition39),
                "Relative Position 40" => Some(RelativePosition40),
                "Relative Position 41" => Some(RelativePosition41),
                "Relative Position 42" => Some(RelativePosition42),
                "Relative Position 43" => Some(RelativePosition43),
                "Relative Position 44" => Some(RelativePosition44),
                "Relative Position 45" => Some(RelativePosition45),
                "Relative Position 46" => Some(RelativePosition46),
                "Right" => Some(Right),
                "Relative Position 47" => Some(RelativePosition47),
                "Relative Position 48" => Some(RelativePosition48),
                "Relative Position 49" => Some(RelativePosition49),
                "Relative Position 50" => Some(RelativePosition50),
                "Stone" => Some(Stone),
                "Spare Tire Position" => Some(SpareTirePosition),
                "Rear Spare" => Some(RearSpare),
                "Stucco" => Some(Stucco),
                "Sub-sea" => Some(SubSea),
                "Tank Bottom" => Some(TankBottom),
                "Top" => Some(Top),
                "Two Sides" => Some(TwoSides),
                "Under Cab" => Some(UnderCab),
                "Unilateral" => Some(Unilateral),
                "Upper" => Some(Upper),
                "Upstream Tap" => Some(UpstreamTap),
                "Wood" => Some(Wood),
                _ => None,
            }
        }
    }
}
impl Serialize for SurfaceLayerPositionCode {
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
    type Value = SurfaceLayerPositionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Surface/Layer/Position Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SurfaceLayerPositionCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Surface/Layer/Position Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SurfaceLayerPositionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Surface/Layer/Position Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for SurfaceLayerPositionCode {
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