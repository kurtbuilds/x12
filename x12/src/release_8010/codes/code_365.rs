use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**365

See docs at <https://www.stedi.com/edi/x12/element/365>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommunicationNumberQualifier {
    ///01 - Additional Trade Item Description URL
    AdditionalTradeItemDescriptionUrl,
    ///02 - Cash Register Description URL
    CashRegisterDescriptionUrl,
    ///03 - Consumer Usage Storage Instructions URL
    ConsumerUsageStorageInstructionsUrl,
    ///04 - Cooking Suggestions URL
    CookingSuggestionsUrl,
    ///05 - Customer Service URL
    CustomerServiceUrl,
    ///06 - Direct Product Markings Description URL
    DirectProductMarkingsDescriptionUrl,
    ///07 - Feature and Benefits URL
    FeatureAndBenefitsUrl,
    ///08 - Finish Description and Image URL
    FinishDescriptionAndImageUrl,
    ///09 - Beauty Image URL
    BeautyImageUrl,
    ///10 - Image Information URL
    ImageInformationUrl,
    ///11 - Label Storage Information URL
    LabelStorageInformationUrl,
    ///12 - Marketing Message URL
    MarketingMessageUrl,
    ///13 - Promotion Markings URL
    PromotionMarkingsUrl,
    ///14 - Precautionary Statement URL
    PrecautionaryStatementUrl,
    ///15 - Preparation Instructions URL
    PreparationInstructionsUrl,
    ///16 - Product Information URL
    ProductInformationUrl,
    ///17 - Serving Suggestions URL
    ServingSuggestionsUrl,
    ///18 - Shape Description URL
    ShapeDescriptionUrl,
    ///19 - Warranty Information URL
    WarrantyInformationUrl,
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
    ///AL - Allergen Information URL
    AllergenInformationUrl,
    ///AP - Alternate Telephone
    AlternateTelephone,
    ///AS - Answering Service
    AnsweringService,
    ///AU - Defense Switched Network
    DefenseSwitchedNetwork,
    ///BN - Beeper Number
    BeeperNumber,
    ///BR - Brochure URL Address
    BrochureUrlAddress,
    ///BT - BTX Number
    BtxNumber,
    ///CA - Cable
    Cable,
    ///CI - Conflict Mineral Information URL
    ConflictMineralInformationUrl,
    ///CM - Certification Mark Image URL
    CertificationMarkImageUrl,
    ///CN - Consumer Support Phone Number
    ConsumerSupportPhoneNumber,
    ///CP - Cellular Phone
    CellularPhone,
    ///CS - Allergen Consumer Support Phone Number
    AllergenConsumerSupportPhoneNumber,
    ///CT - Certification URL
    CertificationUrl,
    ///DN - Defense Data Network (DDN)
    CodeDN,
    ///EA - Internet Email Address
    InternetEmailAddress,
    ///ED - Electronic Data Interchange Access Number
    ElectronicDataInterchangeAccessNumber,
    ///EM - Electronic Mail
    ElectronicMail,
    ///EN - Full Engineering Drawing URL
    FullEngineeringDrawingUrl,
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
    ///Q1 - Image Link
    ImageLink,
    ///Q2 - Image Information Link
    ImageInformationLink,
    ///Q3 - Allergen Information Link
    AllergenInformationLink,
    ///Q4, S4 - Warranty Information Link
    WarrantyInformationLink,
    ///Q5 - Customer Service Link
    CustomerServiceLink,
    ///Q6 - Product Information Link
    ProductInformationLink,
    ///Q7 - Organic Information Link
    OrganicInformationLink,
    ///Q8 - Organic Certification Link (agency)
    CodeQ8,
    ///Q9 - Diet Type Certification Link
    DietTypeCertificationLink,
    ///QA - Image Thumbnail URL
    ImageThumbnailUrl,
    ///R1 - Diet Type Consumer Support Phone Number
    DietTypeConsumerSupportPhoneNumber,
    ///R2 - Installation Video URL
    InstallationVideoUrl,
    ///R3 - Technical Drawing URL
    TechnicalDrawingUrl,
    ///R4 - Manufacturer Product Website URL
    ManufacturerProductWebsiteUrl,
    ///R5 - Material Safety Data Sheet (MSDS or SDS) URL
    CodeR5,
    ///R6 - User Manual URL
    UserManualUrl,
    ///R7 - Pallet Configuration URL
    PalletConfigurationUrl,
    ///R8 - Product Specification URL
    ProductSpecificationUrl,
    ///R9 - Restrictions on Hazardous Substances (RoHS) Material Composition URL
    CodeR9,
    ///RA - Installation Instructions URL
    InstallationInstructionsUrl,
    ///S1 - Service Manual URL
    ServiceManualUrl,
    ///S2 - Technical Bulletin URL
    TechnicalBulletinUrl,
    ///S3 - Video Clip URL
    VideoClipUrl,
    ///S5 - Wiring Diagram URL
    WiringDiagramUrl,
    ///SP - Showing Phone
    ShowingPhone,
    ///TD - 3D Rendering URL
    CodeTD,
    ///TE - Telephone
    Telephone,
    ///TL - Telex
    Telex,
    ///TM - Telemail
    Telemail,
    ///TN - Teletex Number
    TeletexNumber,
    ///TS - Text Message Address
    TextMessageAddress,
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
                AdditionalTradeItemDescriptionUrl => "01",
                CashRegisterDescriptionUrl => "02",
                ConsumerUsageStorageInstructionsUrl => "03",
                CookingSuggestionsUrl => "04",
                CustomerServiceUrl => "05",
                DirectProductMarkingsDescriptionUrl => "06",
                FeatureAndBenefitsUrl => "07",
                FinishDescriptionAndImageUrl => "08",
                BeautyImageUrl => "09",
                ImageInformationUrl => "10",
                LabelStorageInformationUrl => "11",
                MarketingMessageUrl => "12",
                PromotionMarkingsUrl => "13",
                PrecautionaryStatementUrl => "14",
                PreparationInstructionsUrl => "15",
                ProductInformationUrl => "16",
                ServingSuggestionsUrl => "17",
                ShapeDescriptionUrl => "18",
                WarrantyInformationUrl => "19",
                InternationalTelephoneAccessCode => "AA",
                JointFacsimileAndPhoneNumber => "AB",
                MessageOnlyVoiceNumber => "AC",
                DeliveryLocationPhone => "AD",
                AreaCode => "AE",
                AllergenInformationUrl => "AL",
                AlternateTelephone => "AP",
                AnsweringService => "AS",
                DefenseSwitchedNetwork => "AU",
                BeeperNumber => "BN",
                BrochureUrlAddress => "BR",
                BtxNumber => "BT",
                Cable => "CA",
                ConflictMineralInformationUrl => "CI",
                CertificationMarkImageUrl => "CM",
                ConsumerSupportPhoneNumber => "CN",
                CellularPhone => "CP",
                AllergenConsumerSupportPhoneNumber => "CS",
                CertificationUrl => "CT",
                CodeDN => "DN",
                InternetEmailAddress => "EA",
                ElectronicDataInterchangeAccessNumber => "ED",
                ElectronicMail => "EM",
                FullEngineeringDrawingUrl => "EN",
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
                ImageLink => "Q1",
                ImageInformationLink => "Q2",
                AllergenInformationLink => "Q3",
                WarrantyInformationLink => "Q4",
                CustomerServiceLink => "Q5",
                ProductInformationLink => "Q6",
                OrganicInformationLink => "Q7",
                CodeQ8 => "Q8",
                DietTypeCertificationLink => "Q9",
                ImageThumbnailUrl => "QA",
                DietTypeConsumerSupportPhoneNumber => "R1",
                InstallationVideoUrl => "R2",
                TechnicalDrawingUrl => "R3",
                ManufacturerProductWebsiteUrl => "R4",
                CodeR5 => "R5",
                UserManualUrl => "R6",
                PalletConfigurationUrl => "R7",
                ProductSpecificationUrl => "R8",
                CodeR9 => "R9",
                InstallationInstructionsUrl => "RA",
                ServiceManualUrl => "S1",
                TechnicalBulletinUrl => "S2",
                VideoClipUrl => "S3",
                WiringDiagramUrl => "S5",
                ShowingPhone => "SP",
                CodeTD => "TD",
                Telephone => "TE",
                Telex => "TL",
                Telemail => "TM",
                TeletexNumber => "TN",
                TextMessageAddress => "TS",
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
            b"01" => Some(AdditionalTradeItemDescriptionUrl),
            b"02" => Some(CashRegisterDescriptionUrl),
            b"03" => Some(ConsumerUsageStorageInstructionsUrl),
            b"04" => Some(CookingSuggestionsUrl),
            b"05" => Some(CustomerServiceUrl),
            b"06" => Some(DirectProductMarkingsDescriptionUrl),
            b"07" => Some(FeatureAndBenefitsUrl),
            b"08" => Some(FinishDescriptionAndImageUrl),
            b"09" => Some(BeautyImageUrl),
            b"10" => Some(ImageInformationUrl),
            b"11" => Some(LabelStorageInformationUrl),
            b"12" => Some(MarketingMessageUrl),
            b"13" => Some(PromotionMarkingsUrl),
            b"14" => Some(PrecautionaryStatementUrl),
            b"15" => Some(PreparationInstructionsUrl),
            b"16" => Some(ProductInformationUrl),
            b"17" => Some(ServingSuggestionsUrl),
            b"18" => Some(ShapeDescriptionUrl),
            b"19" => Some(WarrantyInformationUrl),
            b"AA" => Some(InternationalTelephoneAccessCode),
            b"AB" => Some(JointFacsimileAndPhoneNumber),
            b"AC" => Some(MessageOnlyVoiceNumber),
            b"AD" => Some(DeliveryLocationPhone),
            b"AE" => Some(AreaCode),
            b"AL" => Some(AllergenInformationUrl),
            b"AP" => Some(AlternateTelephone),
            b"AS" => Some(AnsweringService),
            b"AU" => Some(DefenseSwitchedNetwork),
            b"BN" => Some(BeeperNumber),
            b"BR" => Some(BrochureUrlAddress),
            b"BT" => Some(BtxNumber),
            b"CA" => Some(Cable),
            b"CI" => Some(ConflictMineralInformationUrl),
            b"CM" => Some(CertificationMarkImageUrl),
            b"CN" => Some(ConsumerSupportPhoneNumber),
            b"CP" => Some(CellularPhone),
            b"CS" => Some(AllergenConsumerSupportPhoneNumber),
            b"CT" => Some(CertificationUrl),
            b"DN" => Some(CodeDN),
            b"EA" => Some(InternetEmailAddress),
            b"ED" => Some(ElectronicDataInterchangeAccessNumber),
            b"EM" => Some(ElectronicMail),
            b"EN" => Some(FullEngineeringDrawingUrl),
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
            b"Q1" => Some(ImageLink),
            b"Q2" => Some(ImageInformationLink),
            b"Q3" => Some(AllergenInformationLink),
            b"Q4" => Some(WarrantyInformationLink),
            b"S4" => Some(WarrantyInformationLink),
            b"Q5" => Some(CustomerServiceLink),
            b"Q6" => Some(ProductInformationLink),
            b"Q7" => Some(OrganicInformationLink),
            b"Q8" => Some(CodeQ8),
            b"Q9" => Some(DietTypeCertificationLink),
            b"QA" => Some(ImageThumbnailUrl),
            b"R1" => Some(DietTypeConsumerSupportPhoneNumber),
            b"R2" => Some(InstallationVideoUrl),
            b"R3" => Some(TechnicalDrawingUrl),
            b"R4" => Some(ManufacturerProductWebsiteUrl),
            b"R5" => Some(CodeR5),
            b"R6" => Some(UserManualUrl),
            b"R7" => Some(PalletConfigurationUrl),
            b"R8" => Some(ProductSpecificationUrl),
            b"R9" => Some(CodeR9),
            b"RA" => Some(InstallationInstructionsUrl),
            b"S1" => Some(ServiceManualUrl),
            b"S2" => Some(TechnicalBulletinUrl),
            b"S3" => Some(VideoClipUrl),
            b"S5" => Some(WiringDiagramUrl),
            b"SP" => Some(ShowingPhone),
            b"TD" => Some(CodeTD),
            b"TE" => Some(Telephone),
            b"TL" => Some(Telex),
            b"TM" => Some(Telemail),
            b"TN" => Some(TeletexNumber),
            b"TS" => Some(TextMessageAddress),
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
            AdditionalTradeItemDescriptionUrl => "Additional Trade Item Description URL",
            CashRegisterDescriptionUrl => "Cash Register Description URL",
            ConsumerUsageStorageInstructionsUrl => {
                "Consumer Usage Storage Instructions URL"
            }
            CookingSuggestionsUrl => "Cooking Suggestions URL",
            CustomerServiceUrl => "Customer Service URL",
            DirectProductMarkingsDescriptionUrl => {
                "Direct Product Markings Description URL"
            }
            FeatureAndBenefitsUrl => "Feature and Benefits URL",
            FinishDescriptionAndImageUrl => "Finish Description and Image URL",
            BeautyImageUrl => "Beauty Image URL",
            ImageInformationUrl => "Image Information URL",
            LabelStorageInformationUrl => "Label Storage Information URL",
            MarketingMessageUrl => "Marketing Message URL",
            PromotionMarkingsUrl => "Promotion Markings URL",
            PrecautionaryStatementUrl => "Precautionary Statement URL",
            PreparationInstructionsUrl => "Preparation Instructions URL",
            ProductInformationUrl => "Product Information URL",
            ServingSuggestionsUrl => "Serving Suggestions URL",
            ShapeDescriptionUrl => "Shape Description URL",
            WarrantyInformationUrl => "Warranty Information URL",
            InternationalTelephoneAccessCode => "International Telephone Access Code",
            JointFacsimileAndPhoneNumber => "Joint Facsimile and Phone Number",
            MessageOnlyVoiceNumber => "Message Only Voice Number",
            DeliveryLocationPhone => "Delivery Location Phone",
            AreaCode => "Area Code",
            AllergenInformationUrl => "Allergen Information URL",
            AlternateTelephone => "Alternate Telephone",
            AnsweringService => "Answering Service",
            DefenseSwitchedNetwork => "Defense Switched Network",
            BeeperNumber => "Beeper Number",
            BrochureUrlAddress => "Brochure URL Address",
            BtxNumber => "BTX Number",
            Cable => "Cable",
            ConflictMineralInformationUrl => "Conflict Mineral Information URL",
            CertificationMarkImageUrl => "Certification Mark Image URL",
            ConsumerSupportPhoneNumber => "Consumer Support Phone Number",
            CellularPhone => "Cellular Phone",
            AllergenConsumerSupportPhoneNumber => {
                "Allergen Consumer Support Phone Number"
            }
            CertificationUrl => "Certification URL",
            CodeDN => "Defense Data Network (DDN)",
            InternetEmailAddress => "Internet Email Address",
            ElectronicDataInterchangeAccessNumber => {
                "Electronic Data Interchange Access Number"
            }
            ElectronicMail => "Electronic Mail",
            FullEngineeringDrawingUrl => "Full Engineering Drawing URL",
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
            ImageLink => "Image Link",
            ImageInformationLink => "Image Information Link",
            AllergenInformationLink => "Allergen Information Link",
            WarrantyInformationLink => "Warranty Information Link",
            CustomerServiceLink => "Customer Service Link",
            ProductInformationLink => "Product Information Link",
            OrganicInformationLink => "Organic Information Link",
            CodeQ8 => "Organic Certification Link (agency)",
            DietTypeCertificationLink => "Diet Type Certification Link",
            ImageThumbnailUrl => "Image Thumbnail URL",
            DietTypeConsumerSupportPhoneNumber => {
                "Diet Type Consumer Support Phone Number"
            }
            InstallationVideoUrl => "Installation Video URL",
            TechnicalDrawingUrl => "Technical Drawing URL",
            ManufacturerProductWebsiteUrl => "Manufacturer Product Website URL",
            CodeR5 => "Material Safety Data Sheet (MSDS or SDS) URL",
            UserManualUrl => "User Manual URL",
            PalletConfigurationUrl => "Pallet Configuration URL",
            ProductSpecificationUrl => "Product Specification URL",
            CodeR9 => {
                "Restrictions on Hazardous Substances (RoHS) Material Composition URL"
            }
            InstallationInstructionsUrl => "Installation Instructions URL",
            ServiceManualUrl => "Service Manual URL",
            TechnicalBulletinUrl => "Technical Bulletin URL",
            VideoClipUrl => "Video Clip URL",
            WiringDiagramUrl => "Wiring Diagram URL",
            ShowingPhone => "Showing Phone",
            CodeTD => "3D Rendering URL",
            Telephone => "Telephone",
            Telex => "Telex",
            Telemail => "Telemail",
            TeletexNumber => "Teletex Number",
            TextMessageAddress => "Text Message Address",
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
                "Additional Trade Item Description URL" => {
                    Some(AdditionalTradeItemDescriptionUrl)
                }
                "Cash Register Description URL" => Some(CashRegisterDescriptionUrl),
                "Consumer Usage Storage Instructions URL" => {
                    Some(ConsumerUsageStorageInstructionsUrl)
                }
                "Cooking Suggestions URL" => Some(CookingSuggestionsUrl),
                "Customer Service URL" => Some(CustomerServiceUrl),
                "Direct Product Markings Description URL" => {
                    Some(DirectProductMarkingsDescriptionUrl)
                }
                "Feature and Benefits URL" => Some(FeatureAndBenefitsUrl),
                "Finish Description and Image URL" => Some(FinishDescriptionAndImageUrl),
                "Beauty Image URL" => Some(BeautyImageUrl),
                "Image Information URL" => Some(ImageInformationUrl),
                "Label Storage Information URL" => Some(LabelStorageInformationUrl),
                "Marketing Message URL" => Some(MarketingMessageUrl),
                "Promotion Markings URL" => Some(PromotionMarkingsUrl),
                "Precautionary Statement URL" => Some(PrecautionaryStatementUrl),
                "Preparation Instructions URL" => Some(PreparationInstructionsUrl),
                "Product Information URL" => Some(ProductInformationUrl),
                "Serving Suggestions URL" => Some(ServingSuggestionsUrl),
                "Shape Description URL" => Some(ShapeDescriptionUrl),
                "Warranty Information URL" => Some(WarrantyInformationUrl),
                "International Telephone Access Code" => {
                    Some(InternationalTelephoneAccessCode)
                }
                "Joint Facsimile and Phone Number" => Some(JointFacsimileAndPhoneNumber),
                "Message Only Voice Number" => Some(MessageOnlyVoiceNumber),
                "Delivery Location Phone" => Some(DeliveryLocationPhone),
                "Area Code" => Some(AreaCode),
                "Allergen Information URL" => Some(AllergenInformationUrl),
                "Alternate Telephone" => Some(AlternateTelephone),
                "Answering Service" => Some(AnsweringService),
                "Defense Switched Network" => Some(DefenseSwitchedNetwork),
                "Beeper Number" => Some(BeeperNumber),
                "Brochure URL Address" => Some(BrochureUrlAddress),
                "BTX Number" => Some(BtxNumber),
                "Cable" => Some(Cable),
                "Conflict Mineral Information URL" => Some(ConflictMineralInformationUrl),
                "Certification Mark Image URL" => Some(CertificationMarkImageUrl),
                "Consumer Support Phone Number" => Some(ConsumerSupportPhoneNumber),
                "Cellular Phone" => Some(CellularPhone),
                "Allergen Consumer Support Phone Number" => {
                    Some(AllergenConsumerSupportPhoneNumber)
                }
                "Certification URL" => Some(CertificationUrl),
                "Defense Data Network (DDN)" => Some(CodeDN),
                "Internet Email Address" => Some(InternetEmailAddress),
                "Electronic Data Interchange Access Number" => {
                    Some(ElectronicDataInterchangeAccessNumber)
                }
                "Electronic Mail" => Some(ElectronicMail),
                "Full Engineering Drawing URL" => Some(FullEngineeringDrawingUrl),
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
                "Image Link" => Some(ImageLink),
                "Image Information Link" => Some(ImageInformationLink),
                "Allergen Information Link" => Some(AllergenInformationLink),
                "Warranty Information Link" => Some(WarrantyInformationLink),
                "Customer Service Link" => Some(CustomerServiceLink),
                "Product Information Link" => Some(ProductInformationLink),
                "Organic Information Link" => Some(OrganicInformationLink),
                "Organic Certification Link (agency)" => Some(CodeQ8),
                "Diet Type Certification Link" => Some(DietTypeCertificationLink),
                "Image Thumbnail URL" => Some(ImageThumbnailUrl),
                "Diet Type Consumer Support Phone Number" => {
                    Some(DietTypeConsumerSupportPhoneNumber)
                }
                "Installation Video URL" => Some(InstallationVideoUrl),
                "Technical Drawing URL" => Some(TechnicalDrawingUrl),
                "Manufacturer Product Website URL" => Some(ManufacturerProductWebsiteUrl),
                "Material Safety Data Sheet (MSDS or SDS) URL" => Some(CodeR5),
                "User Manual URL" => Some(UserManualUrl),
                "Pallet Configuration URL" => Some(PalletConfigurationUrl),
                "Product Specification URL" => Some(ProductSpecificationUrl),
                "Restrictions on Hazardous Substances (RoHS) Material Composition URL" => {
                    Some(CodeR9)
                }
                "Installation Instructions URL" => Some(InstallationInstructionsUrl),
                "Service Manual URL" => Some(ServiceManualUrl),
                "Technical Bulletin URL" => Some(TechnicalBulletinUrl),
                "Video Clip URL" => Some(VideoClipUrl),
                "Wiring Diagram URL" => Some(WiringDiagramUrl),
                "Showing Phone" => Some(ShowingPhone),
                "3D Rendering URL" => Some(CodeTD),
                "Telephone" => Some(Telephone),
                "Telex" => Some(Telex),
                "Telemail" => Some(Telemail),
                "Teletex Number" => Some(TeletexNumber),
                "Text Message Address" => Some(TextMessageAddress),
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