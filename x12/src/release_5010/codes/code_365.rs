use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**365

See docs at <https://www.stedi.com/edi/x12-005010/element/365>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommunicationNumberQualifier {
    ///AA - International Telephone Access Code
    InternationalTelephoneAccessCode,
    ///AB - Joint Facsimile and Phone Number
    JointFacsimileAndPhoneNumber,
    ///AC - Message Only Voice Number
    MessageOnlyVoiceNumber,
    ///AD - Delivery Location Phone
    DeliveryLocationPhone,
    ///AE - Area Code
    AreaCode,
    ///AP - Alternate Telephone
    AlternateTelephone,
    ///AS - Answering Service
    AnsweringService,
    ///AU - Defense Switched Network
    DefenseSwitchedNetwork,
    ///BN - Beeper Number
    BeeperNumber,
    ///BT - BTX Number
    BtxNumber,
    ///CA - Cable
    Cable,
    ///CP - Cellular Phone
    CellularPhone,
    ///DN - Defense Data Network (DDN)
    CodeDN,
    ///EA - Internet Email Address
    InternetEmailAddress,
    ///ED - Electronic Data Interchange Access Number
    ElectronicDataInterchangeAccessNumber,
    ///EM - Electronic Mail
    ElectronicMail,
    ///EX - Telephone Extension
    TelephoneExtension,
    ///FT - Federal Telecommunications System (FTS)
    CodeFT,
    ///FU - Facsimile User Identifier
    FacsimileUser,
    ///FX - Facsimile
    Facsimile,
    ///HF - Home Facsimile Number
    HomeFacsimileNumber,
    ///HP - Home Phone Number
    HomePhoneNumber,
    ///IT - International Telephone
    InternationalTelephone,
    ///MN - Modem Number
    ModemNumber,
    ///NP - Night Telephone
    NightTelephone,
    ///OF - Other Residential Facsimile Number
    OtherResidentialFacsimileNumber,
    ///OT - Other Residential Telephone Number
    OtherResidentialTelephoneNumber,
    ///PA - Appointment Phone
    AppointmentPhone,
    ///PC - Personal Cellular
    PersonalCellular,
    ///PP - Personal Phone
    PersonalPhone,
    ///PS - Packet Switching
    PacketSwitching,
    ///SP - Showing Phone
    ShowingPhone,
    ///TE - Telephone
    Telephone,
    ///TL - Telex
    Telex,
    ///TM - Telemail
    Telemail,
    ///TN - Teletex Number
    TeletexNumber,
    ///TX - TWX
    Twx,
    ///UR - Uniform Resource Locator (URL)
    CodeUR,
    ///VM - Voice Mail
    VoiceMail,
    ///WC - Work Cellular
    WorkCellular,
    ///WF - Work Facsimile Number
    WorkFacsimileNumber,
    ///WP - Work Phone Number
    WorkPhoneNumber,
}
impl CommunicationNumberQualifier {
    pub fn code(&self) -> &str {
        {
            use CommunicationNumberQualifier::*;
            match self {
                InternationalTelephoneAccessCode => "AA",
                JointFacsimileAndPhoneNumber => "AB",
                MessageOnlyVoiceNumber => "AC",
                DeliveryLocationPhone => "AD",
                AreaCode => "AE",
                AlternateTelephone => "AP",
                AnsweringService => "AS",
                DefenseSwitchedNetwork => "AU",
                BeeperNumber => "BN",
                BtxNumber => "BT",
                Cable => "CA",
                CellularPhone => "CP",
                CodeDN => "DN",
                InternetEmailAddress => "EA",
                ElectronicDataInterchangeAccessNumber => "ED",
                ElectronicMail => "EM",
                TelephoneExtension => "EX",
                CodeFT => "FT",
                FacsimileUser => "FU",
                Facsimile => "FX",
                HomeFacsimileNumber => "HF",
                HomePhoneNumber => "HP",
                InternationalTelephone => "IT",
                ModemNumber => "MN",
                NightTelephone => "NP",
                OtherResidentialFacsimileNumber => "OF",
                OtherResidentialTelephoneNumber => "OT",
                AppointmentPhone => "PA",
                PersonalCellular => "PC",
                PersonalPhone => "PP",
                PacketSwitching => "PS",
                ShowingPhone => "SP",
                Telephone => "TE",
                Telex => "TL",
                Telemail => "TM",
                TeletexNumber => "TN",
                Twx => "TX",
                CodeUR => "UR",
                VoiceMail => "VM",
                WorkCellular => "WC",
                WorkFacsimileNumber => "WF",
                WorkPhoneNumber => "WP",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CommunicationNumberQualifier> {
        use CommunicationNumberQualifier::*;
        match code {
            b"AA" => Some(InternationalTelephoneAccessCode),
            b"AB" => Some(JointFacsimileAndPhoneNumber),
            b"AC" => Some(MessageOnlyVoiceNumber),
            b"AD" => Some(DeliveryLocationPhone),
            b"AE" => Some(AreaCode),
            b"AP" => Some(AlternateTelephone),
            b"AS" => Some(AnsweringService),
            b"AU" => Some(DefenseSwitchedNetwork),
            b"BN" => Some(BeeperNumber),
            b"BT" => Some(BtxNumber),
            b"CA" => Some(Cable),
            b"CP" => Some(CellularPhone),
            b"DN" => Some(CodeDN),
            b"EA" => Some(InternetEmailAddress),
            b"ED" => Some(ElectronicDataInterchangeAccessNumber),
            b"EM" => Some(ElectronicMail),
            b"EX" => Some(TelephoneExtension),
            b"FT" => Some(CodeFT),
            b"FU" => Some(FacsimileUser),
            b"FX" => Some(Facsimile),
            b"HF" => Some(HomeFacsimileNumber),
            b"HP" => Some(HomePhoneNumber),
            b"IT" => Some(InternationalTelephone),
            b"MN" => Some(ModemNumber),
            b"NP" => Some(NightTelephone),
            b"OF" => Some(OtherResidentialFacsimileNumber),
            b"OT" => Some(OtherResidentialTelephoneNumber),
            b"PA" => Some(AppointmentPhone),
            b"PC" => Some(PersonalCellular),
            b"PP" => Some(PersonalPhone),
            b"PS" => Some(PacketSwitching),
            b"SP" => Some(ShowingPhone),
            b"TE" => Some(Telephone),
            b"TL" => Some(Telex),
            b"TM" => Some(Telemail),
            b"TN" => Some(TeletexNumber),
            b"TX" => Some(Twx),
            b"UR" => Some(CodeUR),
            b"VM" => Some(VoiceMail),
            b"WC" => Some(WorkCellular),
            b"WF" => Some(WorkFacsimileNumber),
            b"WP" => Some(WorkPhoneNumber),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CommunicationNumberQualifier::*;
        match self {
            InternationalTelephoneAccessCode => "International Telephone Access Code",
            JointFacsimileAndPhoneNumber => "Joint Facsimile and Phone Number",
            MessageOnlyVoiceNumber => "Message Only Voice Number",
            DeliveryLocationPhone => "Delivery Location Phone",
            AreaCode => "Area Code",
            AlternateTelephone => "Alternate Telephone",
            AnsweringService => "Answering Service",
            DefenseSwitchedNetwork => "Defense Switched Network",
            BeeperNumber => "Beeper Number",
            BtxNumber => "BTX Number",
            Cable => "Cable",
            CellularPhone => "Cellular Phone",
            CodeDN => "Defense Data Network (DDN)",
            InternetEmailAddress => "Internet Email Address",
            ElectronicDataInterchangeAccessNumber => {
                "Electronic Data Interchange Access Number"
            }
            ElectronicMail => "Electronic Mail",
            TelephoneExtension => "Telephone Extension",
            CodeFT => "Federal Telecommunications System (FTS)",
            FacsimileUser => "Facsimile User Identifier",
            Facsimile => "Facsimile",
            HomeFacsimileNumber => "Home Facsimile Number",
            HomePhoneNumber => "Home Phone Number",
            InternationalTelephone => "International Telephone",
            ModemNumber => "Modem Number",
            NightTelephone => "Night Telephone",
            OtherResidentialFacsimileNumber => "Other Residential Facsimile Number",
            OtherResidentialTelephoneNumber => "Other Residential Telephone Number",
            AppointmentPhone => "Appointment Phone",
            PersonalCellular => "Personal Cellular",
            PersonalPhone => "Personal Phone",
            PacketSwitching => "Packet Switching",
            ShowingPhone => "Showing Phone",
            Telephone => "Telephone",
            Telex => "Telex",
            Telemail => "Telemail",
            TeletexNumber => "Teletex Number",
            Twx => "TWX",
            CodeUR => "Uniform Resource Locator (URL)",
            VoiceMail => "Voice Mail",
            WorkCellular => "Work Cellular",
            WorkFacsimileNumber => "Work Facsimile Number",
            WorkPhoneNumber => "Work Phone Number",
        }
    }
    fn from_description(description: &str) -> Option<CommunicationNumberQualifier> {
        {
            use CommunicationNumberQualifier::*;
            match description {
                "International Telephone Access Code" => {
                    Some(InternationalTelephoneAccessCode)
                }
                "Joint Facsimile and Phone Number" => Some(JointFacsimileAndPhoneNumber),
                "Message Only Voice Number" => Some(MessageOnlyVoiceNumber),
                "Delivery Location Phone" => Some(DeliveryLocationPhone),
                "Area Code" => Some(AreaCode),
                "Alternate Telephone" => Some(AlternateTelephone),
                "Answering Service" => Some(AnsweringService),
                "Defense Switched Network" => Some(DefenseSwitchedNetwork),
                "Beeper Number" => Some(BeeperNumber),
                "BTX Number" => Some(BtxNumber),
                "Cable" => Some(Cable),
                "Cellular Phone" => Some(CellularPhone),
                "Defense Data Network (DDN)" => Some(CodeDN),
                "Internet Email Address" => Some(InternetEmailAddress),
                "Electronic Data Interchange Access Number" => {
                    Some(ElectronicDataInterchangeAccessNumber)
                }
                "Electronic Mail" => Some(ElectronicMail),
                "Telephone Extension" => Some(TelephoneExtension),
                "Federal Telecommunications System (FTS)" => Some(CodeFT),
                "Facsimile User Identifier" => Some(FacsimileUser),
                "Facsimile" => Some(Facsimile),
                "Home Facsimile Number" => Some(HomeFacsimileNumber),
                "Home Phone Number" => Some(HomePhoneNumber),
                "International Telephone" => Some(InternationalTelephone),
                "Modem Number" => Some(ModemNumber),
                "Night Telephone" => Some(NightTelephone),
                "Other Residential Facsimile Number" => {
                    Some(OtherResidentialFacsimileNumber)
                }
                "Other Residential Telephone Number" => {
                    Some(OtherResidentialTelephoneNumber)
                }
                "Appointment Phone" => Some(AppointmentPhone),
                "Personal Cellular" => Some(PersonalCellular),
                "Personal Phone" => Some(PersonalPhone),
                "Packet Switching" => Some(PacketSwitching),
                "Showing Phone" => Some(ShowingPhone),
                "Telephone" => Some(Telephone),
                "Telex" => Some(Telex),
                "Telemail" => Some(Telemail),
                "Teletex Number" => Some(TeletexNumber),
                "TWX" => Some(Twx),
                "Uniform Resource Locator (URL)" => Some(CodeUR),
                "Voice Mail" => Some(VoiceMail),
                "Work Cellular" => Some(WorkCellular),
                "Work Facsimile Number" => Some(WorkFacsimileNumber),
                "Work Phone Number" => Some(WorkPhoneNumber),
                _ => None,
            }
        }
    }
}
impl Serialize for CommunicationNumberQualifier {
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
    type Value = CommunicationNumberQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Communication Number Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CommunicationNumberQualifier::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Communication Number Qualifier: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CommunicationNumberQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Communication Number Qualifier: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for CommunicationNumberQualifier {
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