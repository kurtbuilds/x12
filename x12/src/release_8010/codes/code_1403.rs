use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1403

See docs at <https://www.stedi.com/edi/x12/element/1403>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImplantTypeCode {
    ///0 - High Risk Implantable Device
    HighRiskImplantableDevice,
    ///1 - Pulse Generator
    PulseGenerator,
    ///2 - Atrial Lead
    AtrialLead,
    ///3 - Ventricular Lead
    VentricularLead,
    ///4 - Implantable Cardioverter Defibrillator (ICDs)
    Code4,
    ///5 - Implantable Cardiac Pacemaker (ICPs)
    Code5,
    ///6 - Cardiac Resynchronization Therapy Devices (CRTs)
    Code6,
    ///7 - Pacing Leads
    PacingLeads,
    ///8 - Pacing Battery
    PacingBattery,
    ///9 - Coronary Stent
    CoronaryStent,
    ///A - Peripheral Stent
    PeripheralStent,
    ///B - Heart Valve
    HeartValve,
    ///C - Ventricular-Assist Device
    VentricularAssistDevice,
    ///D - Implantable Heart Monitor
    ImplantableHeartMonitor,
    ///E - Knee
    Knee,
    ///F - Hip
    Hip,
    ///G - Shoulder
    Shoulder,
    ///H - Elbow
    Elbow,
    ///I - Ankle
    Ankle,
    ///J - Toe
    Toe,
    ///K - Finger
    Finger,
    ///L - Spinal
    Spinal,
    ///M - Orthobiologic
    Orthobiologic,
    ///N - Tissue
    Tissue,
    ///O - Facial
    Facial,
    ///P - Dental
    Dental,
    ///Q - Ophthalmic
    Ophthalmic,
    ///R - Gynecologic
    Gynecologic,
    ///S - Urogynecologic
    Urogynecologic,
    ///T - Drug
    Drug,
    ///U - Otolaryngeal
    Otolaryngeal,
    ///V - Cochlear
    Cochlear,
    ///W - Cosmetic
    Cosmetic,
    ///X - Gastroenterologic
    Gastroenterologic,
    ///Y - Urologic
    Urologic,
    ///Z - Other
    Other,
}
impl ImplantTypeCode {
    pub fn code(&self) -> &str {
        {
            use ImplantTypeCode::*;
            match self {
                HighRiskImplantableDevice => "0",
                PulseGenerator => "1",
                AtrialLead => "2",
                VentricularLead => "3",
                Code4 => "4",
                Code5 => "5",
                Code6 => "6",
                PacingLeads => "7",
                PacingBattery => "8",
                CoronaryStent => "9",
                PeripheralStent => "A",
                HeartValve => "B",
                VentricularAssistDevice => "C",
                ImplantableHeartMonitor => "D",
                Knee => "E",
                Hip => "F",
                Shoulder => "G",
                Elbow => "H",
                Ankle => "I",
                Toe => "J",
                Finger => "K",
                Spinal => "L",
                Orthobiologic => "M",
                Tissue => "N",
                Facial => "O",
                Dental => "P",
                Ophthalmic => "Q",
                Gynecologic => "R",
                Urogynecologic => "S",
                Drug => "T",
                Otolaryngeal => "U",
                Cochlear => "V",
                Cosmetic => "W",
                Gastroenterologic => "X",
                Urologic => "Y",
                Other => "Z",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ImplantTypeCode> {
        use ImplantTypeCode::*;
        match code {
            b"0" => Some(HighRiskImplantableDevice),
            b"1" => Some(PulseGenerator),
            b"2" => Some(AtrialLead),
            b"3" => Some(VentricularLead),
            b"4" => Some(Code4),
            b"5" => Some(Code5),
            b"6" => Some(Code6),
            b"7" => Some(PacingLeads),
            b"8" => Some(PacingBattery),
            b"9" => Some(CoronaryStent),
            b"A" => Some(PeripheralStent),
            b"B" => Some(HeartValve),
            b"C" => Some(VentricularAssistDevice),
            b"D" => Some(ImplantableHeartMonitor),
            b"E" => Some(Knee),
            b"F" => Some(Hip),
            b"G" => Some(Shoulder),
            b"H" => Some(Elbow),
            b"I" => Some(Ankle),
            b"J" => Some(Toe),
            b"K" => Some(Finger),
            b"L" => Some(Spinal),
            b"M" => Some(Orthobiologic),
            b"N" => Some(Tissue),
            b"O" => Some(Facial),
            b"P" => Some(Dental),
            b"Q" => Some(Ophthalmic),
            b"R" => Some(Gynecologic),
            b"S" => Some(Urogynecologic),
            b"T" => Some(Drug),
            b"U" => Some(Otolaryngeal),
            b"V" => Some(Cochlear),
            b"W" => Some(Cosmetic),
            b"X" => Some(Gastroenterologic),
            b"Y" => Some(Urologic),
            b"Z" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ImplantTypeCode::*;
        match self {
            HighRiskImplantableDevice => "High Risk Implantable Device",
            PulseGenerator => "Pulse Generator",
            AtrialLead => "Atrial Lead",
            VentricularLead => "Ventricular Lead",
            Code4 => "Implantable Cardioverter Defibrillator (ICDs)",
            Code5 => "Implantable Cardiac Pacemaker (ICPs)",
            Code6 => "Cardiac Resynchronization Therapy Devices (CRTs)",
            PacingLeads => "Pacing Leads",
            PacingBattery => "Pacing Battery",
            CoronaryStent => "Coronary Stent",
            PeripheralStent => "Peripheral Stent",
            HeartValve => "Heart Valve",
            VentricularAssistDevice => "Ventricular-Assist Device",
            ImplantableHeartMonitor => "Implantable Heart Monitor",
            Knee => "Knee",
            Hip => "Hip",
            Shoulder => "Shoulder",
            Elbow => "Elbow",
            Ankle => "Ankle",
            Toe => "Toe",
            Finger => "Finger",
            Spinal => "Spinal",
            Orthobiologic => "Orthobiologic",
            Tissue => "Tissue",
            Facial => "Facial",
            Dental => "Dental",
            Ophthalmic => "Ophthalmic",
            Gynecologic => "Gynecologic",
            Urogynecologic => "Urogynecologic",
            Drug => "Drug",
            Otolaryngeal => "Otolaryngeal",
            Cochlear => "Cochlear",
            Cosmetic => "Cosmetic",
            Gastroenterologic => "Gastroenterologic",
            Urologic => "Urologic",
            Other => "Other",
        }
    }
    fn from_description(description: &str) -> Option<ImplantTypeCode> {
        {
            use ImplantTypeCode::*;
            match description {
                "High Risk Implantable Device" => Some(HighRiskImplantableDevice),
                "Pulse Generator" => Some(PulseGenerator),
                "Atrial Lead" => Some(AtrialLead),
                "Ventricular Lead" => Some(VentricularLead),
                "Implantable Cardioverter Defibrillator (ICDs)" => Some(Code4),
                "Implantable Cardiac Pacemaker (ICPs)" => Some(Code5),
                "Cardiac Resynchronization Therapy Devices (CRTs)" => Some(Code6),
                "Pacing Leads" => Some(PacingLeads),
                "Pacing Battery" => Some(PacingBattery),
                "Coronary Stent" => Some(CoronaryStent),
                "Peripheral Stent" => Some(PeripheralStent),
                "Heart Valve" => Some(HeartValve),
                "Ventricular-Assist Device" => Some(VentricularAssistDevice),
                "Implantable Heart Monitor" => Some(ImplantableHeartMonitor),
                "Knee" => Some(Knee),
                "Hip" => Some(Hip),
                "Shoulder" => Some(Shoulder),
                "Elbow" => Some(Elbow),
                "Ankle" => Some(Ankle),
                "Toe" => Some(Toe),
                "Finger" => Some(Finger),
                "Spinal" => Some(Spinal),
                "Orthobiologic" => Some(Orthobiologic),
                "Tissue" => Some(Tissue),
                "Facial" => Some(Facial),
                "Dental" => Some(Dental),
                "Ophthalmic" => Some(Ophthalmic),
                "Gynecologic" => Some(Gynecologic),
                "Urogynecologic" => Some(Urogynecologic),
                "Drug" => Some(Drug),
                "Otolaryngeal" => Some(Otolaryngeal),
                "Cochlear" => Some(Cochlear),
                "Cosmetic" => Some(Cosmetic),
                "Gastroenterologic" => Some(Gastroenterologic),
                "Urologic" => Some(Urologic),
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for ImplantTypeCode {
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
    type Value = ImplantTypeCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Implant Type Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImplantTypeCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Implant Type Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImplantTypeCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Implant Type Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ImplantTypeCode {
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