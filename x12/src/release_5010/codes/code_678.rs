use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**678

See docs at <https://www.stedi.com/edi/x12-005010/element/678>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShipDeliveryOrCalendarPatternCode {
    ///1 - 1st Week of the Month
    Code1,
    ///2 - 2nd Week of the Month
    Code2,
    ///3 - 3rd Week of the Month
    Code3,
    ///4 - 4th Week of the Month
    Code4,
    ///5 - 5th Week of the Month
    Code5,
    ///6 - 1st & 3rd Weeks of the Month
    Code6,
    ///7 - 2nd & 4th Weeks of the Month
    Code7,
    ///8 - 1st Working Day of Period
    Code8,
    ///9 - Last Working Day of Period
    LastWorkingDayOfPeriod,
    ///A - Monday through Friday
    MondayThroughFriday,
    ///B - Monday through Saturday
    MondayThroughSaturday,
    ///C - Monday through Sunday
    MondayThroughSunday,
    ///D - Monday
    Monday,
    ///E - Tuesday
    Tuesday,
    ///F - Wednesday
    Wednesday,
    ///G - Thursday
    Thursday,
    ///H - Friday
    Friday,
    ///J - Saturday
    Saturday,
    ///K - Sunday
    Sunday,
    ///L - Monday through Thursday
    MondayThroughThursday,
    ///M - Immediately
    Immediately,
    ///N - As Directed
    AsDirected,
    ///O - Daily Mon. through Fri.
    DailyMonThroughFri,
    ///P - 1/2 Mon. & 1/2 Thurs.
    CodeP,
    ///Q - 1/2 Tues. & 1/2 Thurs.
    CodeQ,
    ///R - 1/2 Wed. & 1/2 Fri.
    CodeR,
    ///S - Once Anytime Mon. through Fri.
    OnceAnytimeMonThroughFri,
    ///SA - Sunday, Monday, Thursday, Friday, Saturday
    CodeSA,
    ///SB - Tuesday through Saturday
    TuesdayThroughSaturday,
    ///SC - Sunday, Wednesday, Thursday, Friday, Saturday
    CodeSC,
    ///SD - Monday, Wednesday, Thursday, Friday, Saturday
    CodeSD,
    ///SG - Tuesday through Friday
    TuesdayThroughFriday,
    ///SL - Monday, Tuesday and Thursday
    CodeSL,
    ///SP - Monday, Tuesday and Friday
    CodeSP,
    ///SX - Wednesday and Thursday
    WednesdayAndThursday,
    ///SY - Monday, Wednesday and Thursday
    CodeSY,
    ///SZ - Tuesday, Thursday and Friday
    CodeSZ,
    ///T - 1/2 Tue. & 1/2 Fri.
    CodeT,
    ///U - 1/2 Mon. & 1/2 Wed.
    CodeU,
    ///V - 1/3 Mon., 1/3 Wed., 1/3 Fri.
    CodeV,
    ///W - Whenever Necessary
    WheneverNecessary,
    ///WE - Weekend
    Weekend,
    ///X - 1/2 By Wed., Bal. By Fri.
    CodeX,
    ///Y - None (Also Used to Cancel or Override a Previous Pattern)
    CodeY,
    ///Z - Mutually Defined
    MutuallyDefined,
}
impl ShipDeliveryOrCalendarPatternCode {
    pub fn code(&self) -> &str {
        {
            use ShipDeliveryOrCalendarPatternCode::*;
            match self {
                Code1 => "1",
                Code2 => "2",
                Code3 => "3",
                Code4 => "4",
                Code5 => "5",
                Code6 => "6",
                Code7 => "7",
                Code8 => "8",
                LastWorkingDayOfPeriod => "9",
                MondayThroughFriday => "A",
                MondayThroughSaturday => "B",
                MondayThroughSunday => "C",
                Monday => "D",
                Tuesday => "E",
                Wednesday => "F",
                Thursday => "G",
                Friday => "H",
                Saturday => "J",
                Sunday => "K",
                MondayThroughThursday => "L",
                Immediately => "M",
                AsDirected => "N",
                DailyMonThroughFri => "O",
                CodeP => "P",
                CodeQ => "Q",
                CodeR => "R",
                OnceAnytimeMonThroughFri => "S",
                CodeSA => "SA",
                TuesdayThroughSaturday => "SB",
                CodeSC => "SC",
                CodeSD => "SD",
                TuesdayThroughFriday => "SG",
                CodeSL => "SL",
                CodeSP => "SP",
                WednesdayAndThursday => "SX",
                CodeSY => "SY",
                CodeSZ => "SZ",
                CodeT => "T",
                CodeU => "U",
                CodeV => "V",
                WheneverNecessary => "W",
                Weekend => "WE",
                CodeX => "X",
                CodeY => "Y",
                MutuallyDefined => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ShipDeliveryOrCalendarPatternCode> {
        use ShipDeliveryOrCalendarPatternCode::*;
        match code {
            b"1" => Some(Code1),
            b"2" => Some(Code2),
            b"3" => Some(Code3),
            b"4" => Some(Code4),
            b"5" => Some(Code5),
            b"6" => Some(Code6),
            b"7" => Some(Code7),
            b"8" => Some(Code8),
            b"9" => Some(LastWorkingDayOfPeriod),
            b"A" => Some(MondayThroughFriday),
            b"B" => Some(MondayThroughSaturday),
            b"C" => Some(MondayThroughSunday),
            b"D" => Some(Monday),
            b"E" => Some(Tuesday),
            b"F" => Some(Wednesday),
            b"G" => Some(Thursday),
            b"H" => Some(Friday),
            b"J" => Some(Saturday),
            b"K" => Some(Sunday),
            b"L" => Some(MondayThroughThursday),
            b"M" => Some(Immediately),
            b"N" => Some(AsDirected),
            b"O" => Some(DailyMonThroughFri),
            b"P" => Some(CodeP),
            b"Q" => Some(CodeQ),
            b"R" => Some(CodeR),
            b"S" => Some(OnceAnytimeMonThroughFri),
            b"SA" => Some(CodeSA),
            b"SB" => Some(TuesdayThroughSaturday),
            b"SC" => Some(CodeSC),
            b"SD" => Some(CodeSD),
            b"SG" => Some(TuesdayThroughFriday),
            b"SL" => Some(CodeSL),
            b"SP" => Some(CodeSP),
            b"SX" => Some(WednesdayAndThursday),
            b"SY" => Some(CodeSY),
            b"SZ" => Some(CodeSZ),
            b"T" => Some(CodeT),
            b"U" => Some(CodeU),
            b"V" => Some(CodeV),
            b"W" => Some(WheneverNecessary),
            b"WE" => Some(Weekend),
            b"X" => Some(CodeX),
            b"Y" => Some(CodeY),
            b"Z" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ShipDeliveryOrCalendarPatternCode::*;
        match self {
            Code1 => "1st Week of the Month",
            Code2 => "2nd Week of the Month",
            Code3 => "3rd Week of the Month",
            Code4 => "4th Week of the Month",
            Code5 => "5th Week of the Month",
            Code6 => "1st & 3rd Weeks of the Month",
            Code7 => "2nd & 4th Weeks of the Month",
            Code8 => "1st Working Day of Period",
            LastWorkingDayOfPeriod => "Last Working Day of Period",
            MondayThroughFriday => "Monday through Friday",
            MondayThroughSaturday => "Monday through Saturday",
            MondayThroughSunday => "Monday through Sunday",
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday",
            MondayThroughThursday => "Monday through Thursday",
            Immediately => "Immediately",
            AsDirected => "As Directed",
            DailyMonThroughFri => "Daily Mon. through Fri.",
            CodeP => "1/2 Mon. & 1/2 Thurs.",
            CodeQ => "1/2 Tues. & 1/2 Thurs.",
            CodeR => "1/2 Wed. & 1/2 Fri.",
            OnceAnytimeMonThroughFri => "Once Anytime Mon. through Fri.",
            CodeSA => "Sunday, Monday, Thursday, Friday, Saturday",
            TuesdayThroughSaturday => "Tuesday through Saturday",
            CodeSC => "Sunday, Wednesday, Thursday, Friday, Saturday",
            CodeSD => "Monday, Wednesday, Thursday, Friday, Saturday",
            TuesdayThroughFriday => "Tuesday through Friday",
            CodeSL => "Monday, Tuesday and Thursday",
            CodeSP => "Monday, Tuesday and Friday",
            WednesdayAndThursday => "Wednesday and Thursday",
            CodeSY => "Monday, Wednesday and Thursday",
            CodeSZ => "Tuesday, Thursday and Friday",
            CodeT => "1/2 Tue. & 1/2 Fri.",
            CodeU => "1/2 Mon. & 1/2 Wed.",
            CodeV => "1/3 Mon., 1/3 Wed., 1/3 Fri.",
            WheneverNecessary => "Whenever Necessary",
            Weekend => "Weekend",
            CodeX => "1/2 By Wed., Bal. By Fri.",
            CodeY => "None (Also Used to Cancel or Override a Previous Pattern)",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ShipDeliveryOrCalendarPatternCode> {
        {
            use ShipDeliveryOrCalendarPatternCode::*;
            match description {
                "1st Week of the Month" => Some(Code1),
                "2nd Week of the Month" => Some(Code2),
                "3rd Week of the Month" => Some(Code3),
                "4th Week of the Month" => Some(Code4),
                "5th Week of the Month" => Some(Code5),
                "1st & 3rd Weeks of the Month" => Some(Code6),
                "2nd & 4th Weeks of the Month" => Some(Code7),
                "1st Working Day of Period" => Some(Code8),
                "Last Working Day of Period" => Some(LastWorkingDayOfPeriod),
                "Monday through Friday" => Some(MondayThroughFriday),
                "Monday through Saturday" => Some(MondayThroughSaturday),
                "Monday through Sunday" => Some(MondayThroughSunday),
                "Monday" => Some(Monday),
                "Tuesday" => Some(Tuesday),
                "Wednesday" => Some(Wednesday),
                "Thursday" => Some(Thursday),
                "Friday" => Some(Friday),
                "Saturday" => Some(Saturday),
                "Sunday" => Some(Sunday),
                "Monday through Thursday" => Some(MondayThroughThursday),
                "Immediately" => Some(Immediately),
                "As Directed" => Some(AsDirected),
                "Daily Mon. through Fri." => Some(DailyMonThroughFri),
                "1/2 Mon. & 1/2 Thurs." => Some(CodeP),
                "1/2 Tues. & 1/2 Thurs." => Some(CodeQ),
                "1/2 Wed. & 1/2 Fri." => Some(CodeR),
                "Once Anytime Mon. through Fri." => Some(OnceAnytimeMonThroughFri),
                "Sunday, Monday, Thursday, Friday, Saturday" => Some(CodeSA),
                "Tuesday through Saturday" => Some(TuesdayThroughSaturday),
                "Sunday, Wednesday, Thursday, Friday, Saturday" => Some(CodeSC),
                "Monday, Wednesday, Thursday, Friday, Saturday" => Some(CodeSD),
                "Tuesday through Friday" => Some(TuesdayThroughFriday),
                "Monday, Tuesday and Thursday" => Some(CodeSL),
                "Monday, Tuesday and Friday" => Some(CodeSP),
                "Wednesday and Thursday" => Some(WednesdayAndThursday),
                "Monday, Wednesday and Thursday" => Some(CodeSY),
                "Tuesday, Thursday and Friday" => Some(CodeSZ),
                "1/2 Tue. & 1/2 Fri." => Some(CodeT),
                "1/2 Mon. & 1/2 Wed." => Some(CodeU),
                "1/3 Mon., 1/3 Wed., 1/3 Fri." => Some(CodeV),
                "Whenever Necessary" => Some(WheneverNecessary),
                "Weekend" => Some(Weekend),
                "1/2 By Wed., Bal. By Fri." => Some(CodeX),
                "None (Also Used to Cancel or Override a Previous Pattern)" => {
                    Some(CodeY)
                }
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ShipDeliveryOrCalendarPatternCode {
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
    type Value = ShipDeliveryOrCalendarPatternCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Ship/Delivery or Calendar Pattern Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ShipDeliveryOrCalendarPatternCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Ship/Delivery or Calendar Pattern Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ShipDeliveryOrCalendarPatternCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Ship/Delivery or Calendar Pattern Code: {}",
                    std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ShipDeliveryOrCalendarPatternCode {
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