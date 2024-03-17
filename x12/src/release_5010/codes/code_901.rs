use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**901

See docs at <https://www.stedi.com/edi/x12-005010/element/901>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RejectReasonCode {
    ///01 - Price Authorization Invalid
    PriceAuthorizationInvalid,
    ///02 - Price Authorization Expired
    PriceAuthorizationExpired,
    ///03 - Product not on the price authorization
    ProductNotOnThePriceAuthorization,
    ///04 - Authorized Quantity Exceeded
    AuthorizedQuantityExceeded,
    ///05 - Zero Balance
    ZeroBalance,
    ///06 - Special Cost Incorrect
    SpecialCostIncorrect,
    ///07 - Catalog Cost Incorrect
    CatalogCostIncorrect,
    ///08 - Invalid Ship Location
    InvalidShipLocation,
    ///09 - No Credit Allowed
    NoCreditAllowed,
    ///10 - Administrative Cancellation
    AdministrativeCancellation,
    ///11 - Invalid Debit Number
    InvalidDebitNumber,
    ///12 - Duplicate Sequence Number
    DuplicateSequenceNumber,
    ///13 - Not Valid for Price Protection
    NotValidForPriceProtection,
    ///14 - Invalid part number
    InvalidPartNumber,
    ///15 - Required application data missing
    RequiredApplicationDataMissing,
    ///16 - Unit resale higher than authorized
    UnitResaleHigherThanAuthorized,
    ///17 - Negotiated price was not less than book price
    NegotiatedPriceWasNotLessThanBookPrice,
    ///18 - Ship date must not be after current date
    ShipDateMustNotBeAfterCurrentDate,
    ///19 - Ship date cannot be prior to price authorization issue date
    ShipDateCannotBePriorToPriceAuthorizationIssueDate,
    ///20 - Ship date should not be before price authorization date (for rebills)
    Code20,
    ///21 - Price authorization is a rebill type
    PriceAuthorizationIsARebillType,
    ///23 - Price authorization has been deleted
    PriceAuthorizationHasBeenDeleted,
    ///24 - Price authorization used on a sales order
    PriceAuthorizationUsedOnASalesOrder,
    ///25 - Disposition pending vendor review.
    DispositionPendingVendorReview,
    ///26 - Invalid Customer Number
    InvalidCustomerNumber,
    ///27 - Invalid Ship Date
    InvalidShipDate,
    ///28 - Duplicate Invoice Number
    DuplicateInvoiceNumber,
    ///29 - Claim Submitted Past Exercise Period
    ClaimSubmittedPastExercisePeriod,
    ///30 - Invalid Meet Competition Cost
    InvalidMeetCompetitionCost,
    ///31 - Invalid Book Cost
    InvalidBookCost,
    ///32 - Input Incomplete
    InputIncomplete,
    ///33 - Input Errors
    InputErrors,
    ///34 - No Coverage
    NoCoverage,
    ///35 - Out of Network
    OutOfNetwork,
    ///36 - Testing not Included
    TestingNotIncluded,
    ///37 - Request Forwarded To and Decision Response Forthcoming From an External Review Organization
    RequestForwardedToAndDecisionResponseForthcomingFromAnExternalReviewOrganization,
    ///38 - Claim Can Not Be Identified for Verification
    ClaimCanNotBeIdentifiedForVerification,
    ///39 - Actual Information Different than Reported
    ActualInformationDifferentThanReported,
    ///40 - Actual Information Different - Claim Has Been Re-adjudicated Since Initial Payment
    ActualInformationDifferentClaimHasBeenReAdjudicatedSinceInitialPayment,
    ///41 - Authorization/Access Restrictions
    AuthorizationAccessRestrictions,
    ///42 - Unable to Respond at Current Time
    UnableToRespondAtCurrentTime,
    ///43 - Invalid/Missing Provider Identification
    InvalidMissingProviderIdentification,
    ///44 - Invalid/Missing Provider Name
    InvalidMissingProviderName,
    ///45 - Invalid/Missing Provider Specialty
    InvalidMissingProviderSpecialty,
    ///46 - Invalid/Missing Provider Phone Number
    InvalidMissingProviderPhoneNumber,
    ///47 - Invalid/Missing Provider State
    InvalidMissingProviderState,
    ///48 - Invalid/Missing Referring Provider Identification Number
    InvalidMissingReferringProviderIdentificationNumber,
    ///49 - Provider is Not Primary Care Physician
    ProviderIsNotPrimaryCarePhysician,
    ///50 - Provider Ineligible for Inquiries
    ProviderIneligibleForInquiries,
    ///51 - Provider Not on File
    ProviderNotOnFile,
    ///52 - Service Dates Not Within Provider Plan Enrollment
    ServiceDatesNotWithinProviderPlanEnrollment,
    ///53 - Inquired Benefit Inconsistent with Provider Type
    InquiredBenefitInconsistentWithProviderType,
    ///54 - Inappropriate Product/Service ID Qualifier
    InappropriateProductServiceIdQualifier,
    ///55 - Inappropriate Product/Service ID
    InappropriateProductServiceId,
    ///56 - Inappropriate Date
    InappropriateDate,
    ///57 - Invalid/Missing Date(s) of Service
    Code57,
    ///58 - Invalid/Missing Date-of-Birth
    InvalidMissingDateOfBirth,
    ///59 - Invalid/Missing Date-of-Death
    InvalidMissingDateOfDeath,
    ///60 - Date of Birth Follows Date(s) of Service
    Code60,
    ///61 - Date of Death Precedes Date(s) of Service
    Code61,
    ///62 - Date of Service Not Within Allowable Inquiry Period
    DateOfServiceNotWithinAllowableInquiryPeriod,
    ///63 - Date of Service in Future
    DateOfServiceInFuture,
    ///64 - Invalid/Missing Patient ID
    InvalidMissingPatientId,
    ///65 - Invalid/Missing Patient Name
    InvalidMissingPatientName,
    ///66 - Invalid/Missing Patient Gender Code
    InvalidMissingPatientGenderCode,
    ///67 - Patient Not Found
    PatientNotFound,
    ///68 - Duplicate Patient ID Number
    DuplicatePatientIdNumber,
    ///69 - Inconsistent with Patient's Age
    InconsistentWithPatientsAge,
    ///70 - Inconsistent with Patient's Gender
    InconsistentWithPatientsGender,
    ///71 - Patient Birth Date Does Not Match That for the Patient on the Database
    PatientBirthDateDoesNotMatchThatForThePatientOnTheDatabase,
    ///72 - Invalid/Missing Subscriber/Insured ID
    InvalidMissingSubscriberInsuredId,
    ///73 - Invalid/Missing Subscriber/Insured Name
    InvalidMissingSubscriberInsuredName,
    ///74 - Invalid/Missing Subscriber/Insured Gender Code
    InvalidMissingSubscriberInsuredGenderCode,
    ///75 - Subscriber/Insured Not Found
    SubscriberInsuredNotFound,
    ///76 - Duplicate Subscriber/Insured ID Number
    DuplicateSubscriberInsuredIdNumber,
    ///77 - Subscriber Found, Patient Not Found
    Code77,
    ///78 - Subscriber/Insured Not in Group/Plan Identified
    SubscriberInsuredNotInGroupPlanIdentified,
    ///79 - Invalid Participant Identification
    InvalidParticipantIdentification,
    ///80 - No Response received - Transaction Terminated
    NoResponseReceivedTransactionTerminated,
    ///81 - Invalid or Missing Case Number
    InvalidOrMissingCaseNumber,
    ///82 - Not Medically Necessary
    NotMedicallyNecessary,
    ///83 - Level of Care Not Appropriate
    LevelOfCareNotAppropriate,
    ///84 - Certification Not Required for this Service
    CertificationNotRequiredForThisService,
    ///85 - Certification Responsibility of External Review Organization
    CertificationResponsibilityOfExternalReviewOrganization,
    ///86 - Primary Care Service
    PrimaryCareService,
    ///87 - Exceeds Plan Maximums
    ExceedsPlanMaximums,
    ///88 - Non-covered Service
    NonCoveredService,
    ///89 - No Prior Approval
    NoPriorApproval,
    ///90 - Requested Information Not Received
    RequestedInformationNotReceived,
    ///91 - Duplicate Request
    DuplicateRequest,
    ///92 - Service Inconsistent with Diagnosis
    ServiceInconsistentWithDiagnosis,
    ///95 - Patient Not Eligible
    PatientNotEligible,
    ///96 - Pre-existing Condition
    PreExistingCondition,
    ///97 - Invalid or Missing Provider Address
    InvalidOrMissingProviderAddress,
    ///98 - Experimental Service or Procedure
    ExperimentalServiceOrProcedure,
    ///AA - Authorization Number Not Found
    AuthorizationNumberNotFound,
    ///AB - Air Brakes - Inoperative, etc.
    CodeAB,
    ///AD - Accident Damage - Derail/Sideswiped
    AccidentDamageDerailSideswiped,
    ///AE - Requires Primary Care Physician Authorization
    RequiresPrimaryCarePhysicianAuthorization,
    ///AF - Invalid/Missing Diagnosis Code(s)
    CodeAF,
    ///AG - Invalid/Missing Procedure Code(s)
    CodeAG,
    ///AH - Invalid/Missing Onset of Current Condition or Illness Date
    InvalidMissingOnsetOfCurrentConditionOrIllnessDate,
    ///AI - Invalid/Missing Accident Date
    InvalidMissingAccidentDate,
    ///AJ - Invalid/Missing Last Menstrual Period Date
    InvalidMissingLastMenstrualPeriodDate,
    ///AK - Invalid/Missing Expected Date of Birth
    InvalidMissingExpectedDateOfBirth,
    ///AL - Invalid/Missing Surgery Date
    InvalidMissingSurgeryDate,
    ///AM - Invalid/Missing Admission Date
    InvalidMissingAdmissionDate,
    ///AN - Invalid/Missing Discharge Date
    InvalidMissingDischargeDate,
    ///AO - Additional Patient Condition Information Required
    AdditionalPatientConditionInformationRequired,
    ///AR - Arrivals
    Arrivals,
    ///BA - Reject Due to Air Bags
    RejectDueToAirBags,
    ///BC - Reject Due to No Chains
    RejectDueToNoChains,
    ///BD - Reject Due to Damps
    RejectDueToDamps,
    ///BG - Reject Due to Bearings
    RejectDueToBearings,
    ///BL - Reject Due to Load Divider Bad Order
    RejectDueToLoadDividerBadOrder,
    ///BO - Loaded Car, Unable to Load
    CodeBO,
    ///BP - Reject Due to Bridge Plate
    RejectDueToBridgePlate,
    ///BR - Reject Due to Brake Rigging Beam, Lever
    CodeBR,
    ///BS - Reject Due to Bad Order Slides
    RejectDueToBadOrderSlides,
    ///BV - Reject Due to Bad Order Valves/Piping
    RejectDueToBadOrderValvesPiping,
    ///BW - Reject Due to Bad Order Walls
    RejectDueToBadOrderWalls,
    ///CA - Reject Due to Crank Arm Application
    RejectDueToCrankArmApplication,
    ///CB - Reject Due to Center Bowls, Plates and Pins
    CodeCB,
    ///CI - Certification Information Does Not Match Patient
    CertificationInformationDoesNotMatchPatient,
    ///CL - Complete Loading
    CompleteLoading,
    ///CM - Released as Railroad Company Material
    ReleasedAsRailroadCompanyMaterial,
    ///CN - Car Not Ordered
    CarNotOrdered,
    ///CP - Put on Constructive Placement
    PutOnConstructivePlacement,
    ///CR - Wrong Consignee
    WrongConsignee,
    ///CS - Release Load Through Bill Connecting Road
    ReleaseLoadThroughBillConnectingRoad,
    ///CT - Release as a Cross-town Load
    ReleaseAsACrossTownLoad,
    ///CU - Equipment Not Used
    EquipmentNotUsed,
    ///CW - Wrong Car Type
    WrongCarType,
    ///DD - Reject Due to Doors
    RejectDueToDoors,
    ///DG - Reject Due to Draft Gear - Yoke
    RejectDueToDraftGearYoke,
    ///DM - Dismantle
    Dismantle,
    ///DP - Departed
    Departed,
    ///DR - Reject Due to Dirty
    RejectDueToDirty,
    ///DS - Defective Safety Devices
    DefectiveSafetyDevices,
    ///DV - Reject Due to Load Dividers, Side Filters, Special Equipment
    CodeDV,
    ///E1 - Requested Record Will Not Be Sent; Cannot Identify the Record
    CodeE1,
    ///E2 - Requested Record Will Not Be Sent; Need Student or Parent Permission
    CodeE2,
    ///E3 - Requested Record Will Not Be Sent
    RequestedRecordWillNotBeSent,
    ///E4 - Requested Record Will Not Be Sent; Never Enrolled
    CodeE4,
    ///E5 - Requested Record Will Not Be Sent; No Degree Awarded
    CodeE5,
    ///E6 - Requested Record Will Not Be Sent; No Grades Posted
    CodeE6,
    ///E7 - Requested Record Cannot Be Sent Electronically; Record Resides in Paper Format only which Will Be Sent by Mail
    CodeE7,
    ///E8 - Requires Medical Review
    RequiresMedicalReview,
    ///EA - Empty Equipment Available for Loading
    EmptyEquipmentAvailableForLoading,
    ///ER - Reject Due to Spotted in Error
    RejectDueToSpottedInError,
    ///ET - Empty Trailer Flat Release
    EmptyTrailerFlatRelease,
    ///FD - Freight Damage Claim
    FreightDamageClaim,
    ///FR - Reject Due to Bad Floor
    RejectDueToBadFloor,
    ///GS - Release From Demurrage and Start Storage until Waybilled
    ReleaseFromDemurrageAndStartStorageUntilWaybilled,
    ///HB - Reject Due to Handbrake
    RejectDueToHandbrake,
    ///HH - Reject Due to Hand Hold, Ladder, Step, Running Boards, Platforms, etc.
    CodeHH,
    ///HX - Reject Due to Hot Journal Box
    RejectDueToHotJournalBox,
    ///IA - Invalid Authorization Number Format
    InvalidAuthorizationNumberFormat,
    ///ID - Releases an Idler
    ReleasesAnIdler,
    ///II - Industrial Interchange
    IndustrialInterchange,
    ///IP - Inappropriate Provider Role
    InappropriateProviderRole,
    ///KR - Reject Due to Couplers
    RejectDueToCouplers,
    ///LK - Reject Due to Leaking Contents
    RejectDueToLeakingContents,
    ///LS - Reject Due to Load Shifted
    RejectDueToLoadShifted,
    ///LW - Light Weigh and Restencil
    LightWeighAndRestencil,
    ///MA - Missing Authorization Number
    MissingAuthorizationNumber,
    ///MO - Move from Current Spot to Next
    MoveFromCurrentSpotToNext,
    ///NC - No Certification Information Found
    NoCertificationInformationFound,
    ///OG - Reject Due to Outlet Gate/Valve Lo and Open Hopper
    RejectDueToOutletGateValveLoAndOpenHopper,
    ///OI - Released from Industry to be Inspected
    ReleasedFromIndustryToBeInspected,
    ///OR - Ordered for Replacement
    OrderedForReplacement,
    ///OV - Reject Due to Overloaded
    RejectDueToOverloaded,
    ///PM - Preventative Maintenance
    PreventativeMaintenance,
    ///RB - Released from Industry to Custody of Broker
    ReleasedFromIndustryToCustodyOfBroker,
    ///RD - To be Reloaded
    ToBeReloaded,
    ///RF - Reject Due to Refrigeration Unit
    RejectDueToRefrigerationUnit,
    ///RH - Reject Due to Roof Hatches
    RejectDueToRoofHatches,
    ///RK - Reject Due to Racks (Bi or Tri Levels)
    CodeRK,
    ///RL - Released
    Released,
    ///RN - Bad Order Reinitialing and Numbering
    BadOrderReinitialingAndNumbering,
    ///RP - Released Partially Unloaded
    ReleasedPartiallyUnloaded,
    ///RS - Released Loaded for Line Haul Shipment
    ReleasedLoadedForLineHaulShipment,
    ///RT - Run Through Equipment not Spotted
    RunThroughEquipmentNotSpotted,
    ///SC - Released from Demurrage after being Scrapped
    ReleasedFromDemurrageAfterBeingScrapped,
    ///ST - Released from Shop Track
    ReleasedFromShopTrack,
    ///SU - Reject Due to Superstructure - End, Roof and Sides
    CodeSU,
    ///SW - Local Waybill
    LocalWaybill,
    ///T1 - Cannot Identify Provider as TPO (Third Party Organization) Participant
    CodeT1,
    ///T2 - Cannot Identify Payer as TPO (Third Party Organization) Participant
    CodeT2,
    ///T3 - Cannot Identify Insured as TPO (Third Party Organization) Participant
    CodeT3,
    ///T4 - Payer Name or Identifier Missing
    PayerNameOrIdentifierMissing,
    ///T5 - Certification Information Missing
    CertificationInformationMissing,
    ///T6 - Claim does not contain enough information for re-pricing
    ClaimDoesNotContainEnoughInformationForRePricing,
    ///TC - Bad Order to Transfer Lading
    BadOrderToTransferLading,
    ///TD - Reject Due to Tie Down Devices
    RejectDueToTieDownDevices,
    ///TH - Reject Due to Trailer Hitch
    RejectDueToTrailerHitch,
    ///TL - Reject Due to Train Line, Air Hose, Anglecock
    CodeTL,
    ///TR - Reject Due to Truck, S-Frame, Bolster
    CodeTR,
    ///UC - Reject Due to Uncoupling Rod
    RejectDueToUncouplingRod,
    ///UF - Reject Due to Underframe - Including Sills
    RejectDueToUnderframeIncludingSills,
    ///UG - Bad Order for Upgrading of Car
    BadOrderForUpgradingOfCar,
    ///WA - Reject Due to Wheel/Axle
    RejectDueToWheelAxle,
    ///WK - Bad Order Due to Wreck
    BadOrderDueToWreck,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl RejectReasonCode {
    pub fn code(&self) -> &str {
        {
            use RejectReasonCode::*;
            match self {
                PriceAuthorizationInvalid => "01",
                PriceAuthorizationExpired => "02",
                ProductNotOnThePriceAuthorization => "03",
                AuthorizedQuantityExceeded => "04",
                ZeroBalance => "05",
                SpecialCostIncorrect => "06",
                CatalogCostIncorrect => "07",
                InvalidShipLocation => "08",
                NoCreditAllowed => "09",
                AdministrativeCancellation => "10",
                InvalidDebitNumber => "11",
                DuplicateSequenceNumber => "12",
                NotValidForPriceProtection => "13",
                InvalidPartNumber => "14",
                RequiredApplicationDataMissing => "15",
                UnitResaleHigherThanAuthorized => "16",
                NegotiatedPriceWasNotLessThanBookPrice => "17",
                ShipDateMustNotBeAfterCurrentDate => "18",
                ShipDateCannotBePriorToPriceAuthorizationIssueDate => "19",
                Code20 => "20",
                PriceAuthorizationIsARebillType => "21",
                PriceAuthorizationHasBeenDeleted => "23",
                PriceAuthorizationUsedOnASalesOrder => "24",
                DispositionPendingVendorReview => "25",
                InvalidCustomerNumber => "26",
                InvalidShipDate => "27",
                DuplicateInvoiceNumber => "28",
                ClaimSubmittedPastExercisePeriod => "29",
                InvalidMeetCompetitionCost => "30",
                InvalidBookCost => "31",
                InputIncomplete => "32",
                InputErrors => "33",
                NoCoverage => "34",
                OutOfNetwork => "35",
                TestingNotIncluded => "36",
                RequestForwardedToAndDecisionResponseForthcomingFromAnExternalReviewOrganization => {
                    "37"
                }
                ClaimCanNotBeIdentifiedForVerification => "38",
                ActualInformationDifferentThanReported => "39",
                ActualInformationDifferentClaimHasBeenReAdjudicatedSinceInitialPayment => {
                    "40"
                }
                AuthorizationAccessRestrictions => "41",
                UnableToRespondAtCurrentTime => "42",
                InvalidMissingProviderIdentification => "43",
                InvalidMissingProviderName => "44",
                InvalidMissingProviderSpecialty => "45",
                InvalidMissingProviderPhoneNumber => "46",
                InvalidMissingProviderState => "47",
                InvalidMissingReferringProviderIdentificationNumber => "48",
                ProviderIsNotPrimaryCarePhysician => "49",
                ProviderIneligibleForInquiries => "50",
                ProviderNotOnFile => "51",
                ServiceDatesNotWithinProviderPlanEnrollment => "52",
                InquiredBenefitInconsistentWithProviderType => "53",
                InappropriateProductServiceIdQualifier => "54",
                InappropriateProductServiceId => "55",
                InappropriateDate => "56",
                Code57 => "57",
                InvalidMissingDateOfBirth => "58",
                InvalidMissingDateOfDeath => "59",
                Code60 => "60",
                Code61 => "61",
                DateOfServiceNotWithinAllowableInquiryPeriod => "62",
                DateOfServiceInFuture => "63",
                InvalidMissingPatientId => "64",
                InvalidMissingPatientName => "65",
                InvalidMissingPatientGenderCode => "66",
                PatientNotFound => "67",
                DuplicatePatientIdNumber => "68",
                InconsistentWithPatientsAge => "69",
                InconsistentWithPatientsGender => "70",
                PatientBirthDateDoesNotMatchThatForThePatientOnTheDatabase => "71",
                InvalidMissingSubscriberInsuredId => "72",
                InvalidMissingSubscriberInsuredName => "73",
                InvalidMissingSubscriberInsuredGenderCode => "74",
                SubscriberInsuredNotFound => "75",
                DuplicateSubscriberInsuredIdNumber => "76",
                Code77 => "77",
                SubscriberInsuredNotInGroupPlanIdentified => "78",
                InvalidParticipantIdentification => "79",
                NoResponseReceivedTransactionTerminated => "80",
                InvalidOrMissingCaseNumber => "81",
                NotMedicallyNecessary => "82",
                LevelOfCareNotAppropriate => "83",
                CertificationNotRequiredForThisService => "84",
                CertificationResponsibilityOfExternalReviewOrganization => "85",
                PrimaryCareService => "86",
                ExceedsPlanMaximums => "87",
                NonCoveredService => "88",
                NoPriorApproval => "89",
                RequestedInformationNotReceived => "90",
                DuplicateRequest => "91",
                ServiceInconsistentWithDiagnosis => "92",
                PatientNotEligible => "95",
                PreExistingCondition => "96",
                InvalidOrMissingProviderAddress => "97",
                ExperimentalServiceOrProcedure => "98",
                AuthorizationNumberNotFound => "AA",
                CodeAB => "AB",
                AccidentDamageDerailSideswiped => "AD",
                RequiresPrimaryCarePhysicianAuthorization => "AE",
                CodeAF => "AF",
                CodeAG => "AG",
                InvalidMissingOnsetOfCurrentConditionOrIllnessDate => "AH",
                InvalidMissingAccidentDate => "AI",
                InvalidMissingLastMenstrualPeriodDate => "AJ",
                InvalidMissingExpectedDateOfBirth => "AK",
                InvalidMissingSurgeryDate => "AL",
                InvalidMissingAdmissionDate => "AM",
                InvalidMissingDischargeDate => "AN",
                AdditionalPatientConditionInformationRequired => "AO",
                Arrivals => "AR",
                RejectDueToAirBags => "BA",
                RejectDueToNoChains => "BC",
                RejectDueToDamps => "BD",
                RejectDueToBearings => "BG",
                RejectDueToLoadDividerBadOrder => "BL",
                CodeBO => "BO",
                RejectDueToBridgePlate => "BP",
                CodeBR => "BR",
                RejectDueToBadOrderSlides => "BS",
                RejectDueToBadOrderValvesPiping => "BV",
                RejectDueToBadOrderWalls => "BW",
                RejectDueToCrankArmApplication => "CA",
                CodeCB => "CB",
                CertificationInformationDoesNotMatchPatient => "CI",
                CompleteLoading => "CL",
                ReleasedAsRailroadCompanyMaterial => "CM",
                CarNotOrdered => "CN",
                PutOnConstructivePlacement => "CP",
                WrongConsignee => "CR",
                ReleaseLoadThroughBillConnectingRoad => "CS",
                ReleaseAsACrossTownLoad => "CT",
                EquipmentNotUsed => "CU",
                WrongCarType => "CW",
                RejectDueToDoors => "DD",
                RejectDueToDraftGearYoke => "DG",
                Dismantle => "DM",
                Departed => "DP",
                RejectDueToDirty => "DR",
                DefectiveSafetyDevices => "DS",
                CodeDV => "DV",
                CodeE1 => "E1",
                CodeE2 => "E2",
                RequestedRecordWillNotBeSent => "E3",
                CodeE4 => "E4",
                CodeE5 => "E5",
                CodeE6 => "E6",
                CodeE7 => "E7",
                RequiresMedicalReview => "E8",
                EmptyEquipmentAvailableForLoading => "EA",
                RejectDueToSpottedInError => "ER",
                EmptyTrailerFlatRelease => "ET",
                FreightDamageClaim => "FD",
                RejectDueToBadFloor => "FR",
                ReleaseFromDemurrageAndStartStorageUntilWaybilled => "GS",
                RejectDueToHandbrake => "HB",
                CodeHH => "HH",
                RejectDueToHotJournalBox => "HX",
                InvalidAuthorizationNumberFormat => "IA",
                ReleasesAnIdler => "ID",
                IndustrialInterchange => "II",
                InappropriateProviderRole => "IP",
                RejectDueToCouplers => "KR",
                RejectDueToLeakingContents => "LK",
                RejectDueToLoadShifted => "LS",
                LightWeighAndRestencil => "LW",
                MissingAuthorizationNumber => "MA",
                MoveFromCurrentSpotToNext => "MO",
                NoCertificationInformationFound => "NC",
                RejectDueToOutletGateValveLoAndOpenHopper => "OG",
                ReleasedFromIndustryToBeInspected => "OI",
                OrderedForReplacement => "OR",
                RejectDueToOverloaded => "OV",
                PreventativeMaintenance => "PM",
                ReleasedFromIndustryToCustodyOfBroker => "RB",
                ToBeReloaded => "RD",
                RejectDueToRefrigerationUnit => "RF",
                RejectDueToRoofHatches => "RH",
                CodeRK => "RK",
                Released => "RL",
                BadOrderReinitialingAndNumbering => "RN",
                ReleasedPartiallyUnloaded => "RP",
                ReleasedLoadedForLineHaulShipment => "RS",
                RunThroughEquipmentNotSpotted => "RT",
                ReleasedFromDemurrageAfterBeingScrapped => "SC",
                ReleasedFromShopTrack => "ST",
                CodeSU => "SU",
                LocalWaybill => "SW",
                CodeT1 => "T1",
                CodeT2 => "T2",
                CodeT3 => "T3",
                PayerNameOrIdentifierMissing => "T4",
                CertificationInformationMissing => "T5",
                ClaimDoesNotContainEnoughInformationForRePricing => "T6",
                BadOrderToTransferLading => "TC",
                RejectDueToTieDownDevices => "TD",
                RejectDueToTrailerHitch => "TH",
                CodeTL => "TL",
                CodeTR => "TR",
                RejectDueToUncouplingRod => "UC",
                RejectDueToUnderframeIncludingSills => "UF",
                BadOrderForUpgradingOfCar => "UG",
                RejectDueToWheelAxle => "WA",
                BadOrderDueToWreck => "WK",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<RejectReasonCode> {
        use RejectReasonCode::*;
        match code {
            b"01" => Some(PriceAuthorizationInvalid),
            b"02" => Some(PriceAuthorizationExpired),
            b"03" => Some(ProductNotOnThePriceAuthorization),
            b"04" => Some(AuthorizedQuantityExceeded),
            b"05" => Some(ZeroBalance),
            b"06" => Some(SpecialCostIncorrect),
            b"07" => Some(CatalogCostIncorrect),
            b"08" => Some(InvalidShipLocation),
            b"09" => Some(NoCreditAllowed),
            b"10" => Some(AdministrativeCancellation),
            b"11" => Some(InvalidDebitNumber),
            b"12" => Some(DuplicateSequenceNumber),
            b"13" => Some(NotValidForPriceProtection),
            b"14" => Some(InvalidPartNumber),
            b"15" => Some(RequiredApplicationDataMissing),
            b"16" => Some(UnitResaleHigherThanAuthorized),
            b"17" => Some(NegotiatedPriceWasNotLessThanBookPrice),
            b"18" => Some(ShipDateMustNotBeAfterCurrentDate),
            b"19" => Some(ShipDateCannotBePriorToPriceAuthorizationIssueDate),
            b"20" => Some(Code20),
            b"21" => Some(PriceAuthorizationIsARebillType),
            b"23" => Some(PriceAuthorizationHasBeenDeleted),
            b"24" => Some(PriceAuthorizationUsedOnASalesOrder),
            b"25" => Some(DispositionPendingVendorReview),
            b"26" => Some(InvalidCustomerNumber),
            b"27" => Some(InvalidShipDate),
            b"28" => Some(DuplicateInvoiceNumber),
            b"29" => Some(ClaimSubmittedPastExercisePeriod),
            b"30" => Some(InvalidMeetCompetitionCost),
            b"31" => Some(InvalidBookCost),
            b"32" => Some(InputIncomplete),
            b"33" => Some(InputErrors),
            b"34" => Some(NoCoverage),
            b"35" => Some(OutOfNetwork),
            b"36" => Some(TestingNotIncluded),
            b"37" => {
                Some(
                    RequestForwardedToAndDecisionResponseForthcomingFromAnExternalReviewOrganization,
                )
            }
            b"38" => Some(ClaimCanNotBeIdentifiedForVerification),
            b"39" => Some(ActualInformationDifferentThanReported),
            b"40" => {
                Some(
                    ActualInformationDifferentClaimHasBeenReAdjudicatedSinceInitialPayment,
                )
            }
            b"41" => Some(AuthorizationAccessRestrictions),
            b"42" => Some(UnableToRespondAtCurrentTime),
            b"43" => Some(InvalidMissingProviderIdentification),
            b"44" => Some(InvalidMissingProviderName),
            b"45" => Some(InvalidMissingProviderSpecialty),
            b"46" => Some(InvalidMissingProviderPhoneNumber),
            b"47" => Some(InvalidMissingProviderState),
            b"48" => Some(InvalidMissingReferringProviderIdentificationNumber),
            b"49" => Some(ProviderIsNotPrimaryCarePhysician),
            b"50" => Some(ProviderIneligibleForInquiries),
            b"51" => Some(ProviderNotOnFile),
            b"52" => Some(ServiceDatesNotWithinProviderPlanEnrollment),
            b"53" => Some(InquiredBenefitInconsistentWithProviderType),
            b"54" => Some(InappropriateProductServiceIdQualifier),
            b"55" => Some(InappropriateProductServiceId),
            b"56" => Some(InappropriateDate),
            b"57" => Some(Code57),
            b"58" => Some(InvalidMissingDateOfBirth),
            b"59" => Some(InvalidMissingDateOfDeath),
            b"60" => Some(Code60),
            b"61" => Some(Code61),
            b"62" => Some(DateOfServiceNotWithinAllowableInquiryPeriod),
            b"63" => Some(DateOfServiceInFuture),
            b"64" => Some(InvalidMissingPatientId),
            b"65" => Some(InvalidMissingPatientName),
            b"66" => Some(InvalidMissingPatientGenderCode),
            b"67" => Some(PatientNotFound),
            b"68" => Some(DuplicatePatientIdNumber),
            b"69" => Some(InconsistentWithPatientsAge),
            b"70" => Some(InconsistentWithPatientsGender),
            b"71" => Some(PatientBirthDateDoesNotMatchThatForThePatientOnTheDatabase),
            b"72" => Some(InvalidMissingSubscriberInsuredId),
            b"73" => Some(InvalidMissingSubscriberInsuredName),
            b"74" => Some(InvalidMissingSubscriberInsuredGenderCode),
            b"75" => Some(SubscriberInsuredNotFound),
            b"76" => Some(DuplicateSubscriberInsuredIdNumber),
            b"77" => Some(Code77),
            b"78" => Some(SubscriberInsuredNotInGroupPlanIdentified),
            b"79" => Some(InvalidParticipantIdentification),
            b"80" => Some(NoResponseReceivedTransactionTerminated),
            b"81" => Some(InvalidOrMissingCaseNumber),
            b"82" => Some(NotMedicallyNecessary),
            b"83" => Some(LevelOfCareNotAppropriate),
            b"84" => Some(CertificationNotRequiredForThisService),
            b"85" => Some(CertificationResponsibilityOfExternalReviewOrganization),
            b"86" => Some(PrimaryCareService),
            b"87" => Some(ExceedsPlanMaximums),
            b"88" => Some(NonCoveredService),
            b"89" => Some(NoPriorApproval),
            b"90" => Some(RequestedInformationNotReceived),
            b"91" => Some(DuplicateRequest),
            b"92" => Some(ServiceInconsistentWithDiagnosis),
            b"95" => Some(PatientNotEligible),
            b"96" => Some(PreExistingCondition),
            b"97" => Some(InvalidOrMissingProviderAddress),
            b"98" => Some(ExperimentalServiceOrProcedure),
            b"AA" => Some(AuthorizationNumberNotFound),
            b"AB" => Some(CodeAB),
            b"AD" => Some(AccidentDamageDerailSideswiped),
            b"AE" => Some(RequiresPrimaryCarePhysicianAuthorization),
            b"AF" => Some(CodeAF),
            b"AG" => Some(CodeAG),
            b"AH" => Some(InvalidMissingOnsetOfCurrentConditionOrIllnessDate),
            b"AI" => Some(InvalidMissingAccidentDate),
            b"AJ" => Some(InvalidMissingLastMenstrualPeriodDate),
            b"AK" => Some(InvalidMissingExpectedDateOfBirth),
            b"AL" => Some(InvalidMissingSurgeryDate),
            b"AM" => Some(InvalidMissingAdmissionDate),
            b"AN" => Some(InvalidMissingDischargeDate),
            b"AO" => Some(AdditionalPatientConditionInformationRequired),
            b"AR" => Some(Arrivals),
            b"BA" => Some(RejectDueToAirBags),
            b"BC" => Some(RejectDueToNoChains),
            b"BD" => Some(RejectDueToDamps),
            b"BG" => Some(RejectDueToBearings),
            b"BL" => Some(RejectDueToLoadDividerBadOrder),
            b"BO" => Some(CodeBO),
            b"BP" => Some(RejectDueToBridgePlate),
            b"BR" => Some(CodeBR),
            b"BS" => Some(RejectDueToBadOrderSlides),
            b"BV" => Some(RejectDueToBadOrderValvesPiping),
            b"BW" => Some(RejectDueToBadOrderWalls),
            b"CA" => Some(RejectDueToCrankArmApplication),
            b"CB" => Some(CodeCB),
            b"CI" => Some(CertificationInformationDoesNotMatchPatient),
            b"CL" => Some(CompleteLoading),
            b"CM" => Some(ReleasedAsRailroadCompanyMaterial),
            b"CN" => Some(CarNotOrdered),
            b"CP" => Some(PutOnConstructivePlacement),
            b"CR" => Some(WrongConsignee),
            b"CS" => Some(ReleaseLoadThroughBillConnectingRoad),
            b"CT" => Some(ReleaseAsACrossTownLoad),
            b"CU" => Some(EquipmentNotUsed),
            b"CW" => Some(WrongCarType),
            b"DD" => Some(RejectDueToDoors),
            b"DG" => Some(RejectDueToDraftGearYoke),
            b"DM" => Some(Dismantle),
            b"DP" => Some(Departed),
            b"DR" => Some(RejectDueToDirty),
            b"DS" => Some(DefectiveSafetyDevices),
            b"DV" => Some(CodeDV),
            b"E1" => Some(CodeE1),
            b"E2" => Some(CodeE2),
            b"E3" => Some(RequestedRecordWillNotBeSent),
            b"E4" => Some(CodeE4),
            b"E5" => Some(CodeE5),
            b"E6" => Some(CodeE6),
            b"E7" => Some(CodeE7),
            b"E8" => Some(RequiresMedicalReview),
            b"EA" => Some(EmptyEquipmentAvailableForLoading),
            b"ER" => Some(RejectDueToSpottedInError),
            b"ET" => Some(EmptyTrailerFlatRelease),
            b"FD" => Some(FreightDamageClaim),
            b"FR" => Some(RejectDueToBadFloor),
            b"GS" => Some(ReleaseFromDemurrageAndStartStorageUntilWaybilled),
            b"HB" => Some(RejectDueToHandbrake),
            b"HH" => Some(CodeHH),
            b"HX" => Some(RejectDueToHotJournalBox),
            b"IA" => Some(InvalidAuthorizationNumberFormat),
            b"ID" => Some(ReleasesAnIdler),
            b"II" => Some(IndustrialInterchange),
            b"IP" => Some(InappropriateProviderRole),
            b"KR" => Some(RejectDueToCouplers),
            b"LK" => Some(RejectDueToLeakingContents),
            b"LS" => Some(RejectDueToLoadShifted),
            b"LW" => Some(LightWeighAndRestencil),
            b"MA" => Some(MissingAuthorizationNumber),
            b"MO" => Some(MoveFromCurrentSpotToNext),
            b"NC" => Some(NoCertificationInformationFound),
            b"OG" => Some(RejectDueToOutletGateValveLoAndOpenHopper),
            b"OI" => Some(ReleasedFromIndustryToBeInspected),
            b"OR" => Some(OrderedForReplacement),
            b"OV" => Some(RejectDueToOverloaded),
            b"PM" => Some(PreventativeMaintenance),
            b"RB" => Some(ReleasedFromIndustryToCustodyOfBroker),
            b"RD" => Some(ToBeReloaded),
            b"RF" => Some(RejectDueToRefrigerationUnit),
            b"RH" => Some(RejectDueToRoofHatches),
            b"RK" => Some(CodeRK),
            b"RL" => Some(Released),
            b"RN" => Some(BadOrderReinitialingAndNumbering),
            b"RP" => Some(ReleasedPartiallyUnloaded),
            b"RS" => Some(ReleasedLoadedForLineHaulShipment),
            b"RT" => Some(RunThroughEquipmentNotSpotted),
            b"SC" => Some(ReleasedFromDemurrageAfterBeingScrapped),
            b"ST" => Some(ReleasedFromShopTrack),
            b"SU" => Some(CodeSU),
            b"SW" => Some(LocalWaybill),
            b"T1" => Some(CodeT1),
            b"T2" => Some(CodeT2),
            b"T3" => Some(CodeT3),
            b"T4" => Some(PayerNameOrIdentifierMissing),
            b"T5" => Some(CertificationInformationMissing),
            b"T6" => Some(ClaimDoesNotContainEnoughInformationForRePricing),
            b"TC" => Some(BadOrderToTransferLading),
            b"TD" => Some(RejectDueToTieDownDevices),
            b"TH" => Some(RejectDueToTrailerHitch),
            b"TL" => Some(CodeTL),
            b"TR" => Some(CodeTR),
            b"UC" => Some(RejectDueToUncouplingRod),
            b"UF" => Some(RejectDueToUnderframeIncludingSills),
            b"UG" => Some(BadOrderForUpgradingOfCar),
            b"WA" => Some(RejectDueToWheelAxle),
            b"WK" => Some(BadOrderDueToWreck),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use RejectReasonCode::*;
        match self {
            PriceAuthorizationInvalid => "Price Authorization Invalid",
            PriceAuthorizationExpired => "Price Authorization Expired",
            ProductNotOnThePriceAuthorization => "Product not on the price authorization",
            AuthorizedQuantityExceeded => "Authorized Quantity Exceeded",
            ZeroBalance => "Zero Balance",
            SpecialCostIncorrect => "Special Cost Incorrect",
            CatalogCostIncorrect => "Catalog Cost Incorrect",
            InvalidShipLocation => "Invalid Ship Location",
            NoCreditAllowed => "No Credit Allowed",
            AdministrativeCancellation => "Administrative Cancellation",
            InvalidDebitNumber => "Invalid Debit Number",
            DuplicateSequenceNumber => "Duplicate Sequence Number",
            NotValidForPriceProtection => "Not Valid for Price Protection",
            InvalidPartNumber => "Invalid part number",
            RequiredApplicationDataMissing => "Required application data missing",
            UnitResaleHigherThanAuthorized => "Unit resale higher than authorized",
            NegotiatedPriceWasNotLessThanBookPrice => {
                "Negotiated price was not less than book price"
            }
            ShipDateMustNotBeAfterCurrentDate => {
                "Ship date must not be after current date"
            }
            ShipDateCannotBePriorToPriceAuthorizationIssueDate => {
                "Ship date cannot be prior to price authorization issue date"
            }
            Code20 => {
                "Ship date should not be before price authorization date (for rebills)"
            }
            PriceAuthorizationIsARebillType => "Price authorization is a rebill type",
            PriceAuthorizationHasBeenDeleted => "Price authorization has been deleted",
            PriceAuthorizationUsedOnASalesOrder => {
                "Price authorization used on a sales order"
            }
            DispositionPendingVendorReview => "Disposition pending vendor review.",
            InvalidCustomerNumber => "Invalid Customer Number",
            InvalidShipDate => "Invalid Ship Date",
            DuplicateInvoiceNumber => "Duplicate Invoice Number",
            ClaimSubmittedPastExercisePeriod => "Claim Submitted Past Exercise Period",
            InvalidMeetCompetitionCost => "Invalid Meet Competition Cost",
            InvalidBookCost => "Invalid Book Cost",
            InputIncomplete => "Input Incomplete",
            InputErrors => "Input Errors",
            NoCoverage => "No Coverage",
            OutOfNetwork => "Out of Network",
            TestingNotIncluded => "Testing not Included",
            RequestForwardedToAndDecisionResponseForthcomingFromAnExternalReviewOrganization => {
                "Request Forwarded To and Decision Response Forthcoming From an External Review Organization"
            }
            ClaimCanNotBeIdentifiedForVerification => {
                "Claim Can Not Be Identified for Verification"
            }
            ActualInformationDifferentThanReported => {
                "Actual Information Different than Reported"
            }
            ActualInformationDifferentClaimHasBeenReAdjudicatedSinceInitialPayment => {
                "Actual Information Different - Claim Has Been Re-adjudicated Since Initial Payment"
            }
            AuthorizationAccessRestrictions => "Authorization/Access Restrictions",
            UnableToRespondAtCurrentTime => "Unable to Respond at Current Time",
            InvalidMissingProviderIdentification => {
                "Invalid/Missing Provider Identification"
            }
            InvalidMissingProviderName => "Invalid/Missing Provider Name",
            InvalidMissingProviderSpecialty => "Invalid/Missing Provider Specialty",
            InvalidMissingProviderPhoneNumber => "Invalid/Missing Provider Phone Number",
            InvalidMissingProviderState => "Invalid/Missing Provider State",
            InvalidMissingReferringProviderIdentificationNumber => {
                "Invalid/Missing Referring Provider Identification Number"
            }
            ProviderIsNotPrimaryCarePhysician => "Provider is Not Primary Care Physician",
            ProviderIneligibleForInquiries => "Provider Ineligible for Inquiries",
            ProviderNotOnFile => "Provider Not on File",
            ServiceDatesNotWithinProviderPlanEnrollment => {
                "Service Dates Not Within Provider Plan Enrollment"
            }
            InquiredBenefitInconsistentWithProviderType => {
                "Inquired Benefit Inconsistent with Provider Type"
            }
            InappropriateProductServiceIdQualifier => {
                "Inappropriate Product/Service ID Qualifier"
            }
            InappropriateProductServiceId => "Inappropriate Product/Service ID",
            InappropriateDate => "Inappropriate Date",
            Code57 => "Invalid/Missing Date(s) of Service",
            InvalidMissingDateOfBirth => "Invalid/Missing Date-of-Birth",
            InvalidMissingDateOfDeath => "Invalid/Missing Date-of-Death",
            Code60 => "Date of Birth Follows Date(s) of Service",
            Code61 => "Date of Death Precedes Date(s) of Service",
            DateOfServiceNotWithinAllowableInquiryPeriod => {
                "Date of Service Not Within Allowable Inquiry Period"
            }
            DateOfServiceInFuture => "Date of Service in Future",
            InvalidMissingPatientId => "Invalid/Missing Patient ID",
            InvalidMissingPatientName => "Invalid/Missing Patient Name",
            InvalidMissingPatientGenderCode => "Invalid/Missing Patient Gender Code",
            PatientNotFound => "Patient Not Found",
            DuplicatePatientIdNumber => "Duplicate Patient ID Number",
            InconsistentWithPatientsAge => "Inconsistent with Patient's Age",
            InconsistentWithPatientsGender => "Inconsistent with Patient's Gender",
            PatientBirthDateDoesNotMatchThatForThePatientOnTheDatabase => {
                "Patient Birth Date Does Not Match That for the Patient on the Database"
            }
            InvalidMissingSubscriberInsuredId => "Invalid/Missing Subscriber/Insured ID",
            InvalidMissingSubscriberInsuredName => {
                "Invalid/Missing Subscriber/Insured Name"
            }
            InvalidMissingSubscriberInsuredGenderCode => {
                "Invalid/Missing Subscriber/Insured Gender Code"
            }
            SubscriberInsuredNotFound => "Subscriber/Insured Not Found",
            DuplicateSubscriberInsuredIdNumber => {
                "Duplicate Subscriber/Insured ID Number"
            }
            Code77 => "Subscriber Found, Patient Not Found",
            SubscriberInsuredNotInGroupPlanIdentified => {
                "Subscriber/Insured Not in Group/Plan Identified"
            }
            InvalidParticipantIdentification => "Invalid Participant Identification",
            NoResponseReceivedTransactionTerminated => {
                "No Response received - Transaction Terminated"
            }
            InvalidOrMissingCaseNumber => "Invalid or Missing Case Number",
            NotMedicallyNecessary => "Not Medically Necessary",
            LevelOfCareNotAppropriate => "Level of Care Not Appropriate",
            CertificationNotRequiredForThisService => {
                "Certification Not Required for this Service"
            }
            CertificationResponsibilityOfExternalReviewOrganization => {
                "Certification Responsibility of External Review Organization"
            }
            PrimaryCareService => "Primary Care Service",
            ExceedsPlanMaximums => "Exceeds Plan Maximums",
            NonCoveredService => "Non-covered Service",
            NoPriorApproval => "No Prior Approval",
            RequestedInformationNotReceived => "Requested Information Not Received",
            DuplicateRequest => "Duplicate Request",
            ServiceInconsistentWithDiagnosis => "Service Inconsistent with Diagnosis",
            PatientNotEligible => "Patient Not Eligible",
            PreExistingCondition => "Pre-existing Condition",
            InvalidOrMissingProviderAddress => "Invalid or Missing Provider Address",
            ExperimentalServiceOrProcedure => "Experimental Service or Procedure",
            AuthorizationNumberNotFound => "Authorization Number Not Found",
            CodeAB => "Air Brakes - Inoperative, etc.",
            AccidentDamageDerailSideswiped => "Accident Damage - Derail/Sideswiped",
            RequiresPrimaryCarePhysicianAuthorization => {
                "Requires Primary Care Physician Authorization"
            }
            CodeAF => "Invalid/Missing Diagnosis Code(s)",
            CodeAG => "Invalid/Missing Procedure Code(s)",
            InvalidMissingOnsetOfCurrentConditionOrIllnessDate => {
                "Invalid/Missing Onset of Current Condition or Illness Date"
            }
            InvalidMissingAccidentDate => "Invalid/Missing Accident Date",
            InvalidMissingLastMenstrualPeriodDate => {
                "Invalid/Missing Last Menstrual Period Date"
            }
            InvalidMissingExpectedDateOfBirth => "Invalid/Missing Expected Date of Birth",
            InvalidMissingSurgeryDate => "Invalid/Missing Surgery Date",
            InvalidMissingAdmissionDate => "Invalid/Missing Admission Date",
            InvalidMissingDischargeDate => "Invalid/Missing Discharge Date",
            AdditionalPatientConditionInformationRequired => {
                "Additional Patient Condition Information Required"
            }
            Arrivals => "Arrivals",
            RejectDueToAirBags => "Reject Due to Air Bags",
            RejectDueToNoChains => "Reject Due to No Chains",
            RejectDueToDamps => "Reject Due to Damps",
            RejectDueToBearings => "Reject Due to Bearings",
            RejectDueToLoadDividerBadOrder => "Reject Due to Load Divider Bad Order",
            CodeBO => "Loaded Car, Unable to Load",
            RejectDueToBridgePlate => "Reject Due to Bridge Plate",
            CodeBR => "Reject Due to Brake Rigging Beam, Lever",
            RejectDueToBadOrderSlides => "Reject Due to Bad Order Slides",
            RejectDueToBadOrderValvesPiping => "Reject Due to Bad Order Valves/Piping",
            RejectDueToBadOrderWalls => "Reject Due to Bad Order Walls",
            RejectDueToCrankArmApplication => "Reject Due to Crank Arm Application",
            CodeCB => "Reject Due to Center Bowls, Plates and Pins",
            CertificationInformationDoesNotMatchPatient => {
                "Certification Information Does Not Match Patient"
            }
            CompleteLoading => "Complete Loading",
            ReleasedAsRailroadCompanyMaterial => "Released as Railroad Company Material",
            CarNotOrdered => "Car Not Ordered",
            PutOnConstructivePlacement => "Put on Constructive Placement",
            WrongConsignee => "Wrong Consignee",
            ReleaseLoadThroughBillConnectingRoad => {
                "Release Load Through Bill Connecting Road"
            }
            ReleaseAsACrossTownLoad => "Release as a Cross-town Load",
            EquipmentNotUsed => "Equipment Not Used",
            WrongCarType => "Wrong Car Type",
            RejectDueToDoors => "Reject Due to Doors",
            RejectDueToDraftGearYoke => "Reject Due to Draft Gear - Yoke",
            Dismantle => "Dismantle",
            Departed => "Departed",
            RejectDueToDirty => "Reject Due to Dirty",
            DefectiveSafetyDevices => "Defective Safety Devices",
            CodeDV => "Reject Due to Load Dividers, Side Filters, Special Equipment",
            CodeE1 => "Requested Record Will Not Be Sent; Cannot Identify the Record",
            CodeE2 => {
                "Requested Record Will Not Be Sent; Need Student or Parent Permission"
            }
            RequestedRecordWillNotBeSent => "Requested Record Will Not Be Sent",
            CodeE4 => "Requested Record Will Not Be Sent; Never Enrolled",
            CodeE5 => "Requested Record Will Not Be Sent; No Degree Awarded",
            CodeE6 => "Requested Record Will Not Be Sent; No Grades Posted",
            CodeE7 => {
                "Requested Record Cannot Be Sent Electronically; Record Resides in Paper Format only which Will Be Sent by Mail"
            }
            RequiresMedicalReview => "Requires Medical Review",
            EmptyEquipmentAvailableForLoading => "Empty Equipment Available for Loading",
            RejectDueToSpottedInError => "Reject Due to Spotted in Error",
            EmptyTrailerFlatRelease => "Empty Trailer Flat Release",
            FreightDamageClaim => "Freight Damage Claim",
            RejectDueToBadFloor => "Reject Due to Bad Floor",
            ReleaseFromDemurrageAndStartStorageUntilWaybilled => {
                "Release From Demurrage and Start Storage until Waybilled"
            }
            RejectDueToHandbrake => "Reject Due to Handbrake",
            CodeHH => {
                "Reject Due to Hand Hold, Ladder, Step, Running Boards, Platforms, etc."
            }
            RejectDueToHotJournalBox => "Reject Due to Hot Journal Box",
            InvalidAuthorizationNumberFormat => "Invalid Authorization Number Format",
            ReleasesAnIdler => "Releases an Idler",
            IndustrialInterchange => "Industrial Interchange",
            InappropriateProviderRole => "Inappropriate Provider Role",
            RejectDueToCouplers => "Reject Due to Couplers",
            RejectDueToLeakingContents => "Reject Due to Leaking Contents",
            RejectDueToLoadShifted => "Reject Due to Load Shifted",
            LightWeighAndRestencil => "Light Weigh and Restencil",
            MissingAuthorizationNumber => "Missing Authorization Number",
            MoveFromCurrentSpotToNext => "Move from Current Spot to Next",
            NoCertificationInformationFound => "No Certification Information Found",
            RejectDueToOutletGateValveLoAndOpenHopper => {
                "Reject Due to Outlet Gate/Valve Lo and Open Hopper"
            }
            ReleasedFromIndustryToBeInspected => "Released from Industry to be Inspected",
            OrderedForReplacement => "Ordered for Replacement",
            RejectDueToOverloaded => "Reject Due to Overloaded",
            PreventativeMaintenance => "Preventative Maintenance",
            ReleasedFromIndustryToCustodyOfBroker => {
                "Released from Industry to Custody of Broker"
            }
            ToBeReloaded => "To be Reloaded",
            RejectDueToRefrigerationUnit => "Reject Due to Refrigeration Unit",
            RejectDueToRoofHatches => "Reject Due to Roof Hatches",
            CodeRK => "Reject Due to Racks (Bi or Tri Levels)",
            Released => "Released",
            BadOrderReinitialingAndNumbering => "Bad Order Reinitialing and Numbering",
            ReleasedPartiallyUnloaded => "Released Partially Unloaded",
            ReleasedLoadedForLineHaulShipment => "Released Loaded for Line Haul Shipment",
            RunThroughEquipmentNotSpotted => "Run Through Equipment not Spotted",
            ReleasedFromDemurrageAfterBeingScrapped => {
                "Released from Demurrage after being Scrapped"
            }
            ReleasedFromShopTrack => "Released from Shop Track",
            CodeSU => "Reject Due to Superstructure - End, Roof and Sides",
            LocalWaybill => "Local Waybill",
            CodeT1 => {
                "Cannot Identify Provider as TPO (Third Party Organization) Participant"
            }
            CodeT2 => {
                "Cannot Identify Payer as TPO (Third Party Organization) Participant"
            }
            CodeT3 => {
                "Cannot Identify Insured as TPO (Third Party Organization) Participant"
            }
            PayerNameOrIdentifierMissing => "Payer Name or Identifier Missing",
            CertificationInformationMissing => "Certification Information Missing",
            ClaimDoesNotContainEnoughInformationForRePricing => {
                "Claim does not contain enough information for re-pricing"
            }
            BadOrderToTransferLading => "Bad Order to Transfer Lading",
            RejectDueToTieDownDevices => "Reject Due to Tie Down Devices",
            RejectDueToTrailerHitch => "Reject Due to Trailer Hitch",
            CodeTL => "Reject Due to Train Line, Air Hose, Anglecock",
            CodeTR => "Reject Due to Truck, S-Frame, Bolster",
            RejectDueToUncouplingRod => "Reject Due to Uncoupling Rod",
            RejectDueToUnderframeIncludingSills => {
                "Reject Due to Underframe - Including Sills"
            }
            BadOrderForUpgradingOfCar => "Bad Order for Upgrading of Car",
            RejectDueToWheelAxle => "Reject Due to Wheel/Axle",
            BadOrderDueToWreck => "Bad Order Due to Wreck",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<RejectReasonCode> {
        {
            use RejectReasonCode::*;
            match description {
                "Price Authorization Invalid" => Some(PriceAuthorizationInvalid),
                "Price Authorization Expired" => Some(PriceAuthorizationExpired),
                "Product not on the price authorization" => {
                    Some(ProductNotOnThePriceAuthorization)
                }
                "Authorized Quantity Exceeded" => Some(AuthorizedQuantityExceeded),
                "Zero Balance" => Some(ZeroBalance),
                "Special Cost Incorrect" => Some(SpecialCostIncorrect),
                "Catalog Cost Incorrect" => Some(CatalogCostIncorrect),
                "Invalid Ship Location" => Some(InvalidShipLocation),
                "No Credit Allowed" => Some(NoCreditAllowed),
                "Administrative Cancellation" => Some(AdministrativeCancellation),
                "Invalid Debit Number" => Some(InvalidDebitNumber),
                "Duplicate Sequence Number" => Some(DuplicateSequenceNumber),
                "Not Valid for Price Protection" => Some(NotValidForPriceProtection),
                "Invalid part number" => Some(InvalidPartNumber),
                "Required application data missing" => {
                    Some(RequiredApplicationDataMissing)
                }
                "Unit resale higher than authorized" => {
                    Some(UnitResaleHigherThanAuthorized)
                }
                "Negotiated price was not less than book price" => {
                    Some(NegotiatedPriceWasNotLessThanBookPrice)
                }
                "Ship date must not be after current date" => {
                    Some(ShipDateMustNotBeAfterCurrentDate)
                }
                "Ship date cannot be prior to price authorization issue date" => {
                    Some(ShipDateCannotBePriorToPriceAuthorizationIssueDate)
                }
                "Ship date should not be before price authorization date (for rebills)" => {
                    Some(Code20)
                }
                "Price authorization is a rebill type" => {
                    Some(PriceAuthorizationIsARebillType)
                }
                "Price authorization has been deleted" => {
                    Some(PriceAuthorizationHasBeenDeleted)
                }
                "Price authorization used on a sales order" => {
                    Some(PriceAuthorizationUsedOnASalesOrder)
                }
                "Disposition pending vendor review." => {
                    Some(DispositionPendingVendorReview)
                }
                "Invalid Customer Number" => Some(InvalidCustomerNumber),
                "Invalid Ship Date" => Some(InvalidShipDate),
                "Duplicate Invoice Number" => Some(DuplicateInvoiceNumber),
                "Claim Submitted Past Exercise Period" => {
                    Some(ClaimSubmittedPastExercisePeriod)
                }
                "Invalid Meet Competition Cost" => Some(InvalidMeetCompetitionCost),
                "Invalid Book Cost" => Some(InvalidBookCost),
                "Input Incomplete" => Some(InputIncomplete),
                "Input Errors" => Some(InputErrors),
                "No Coverage" => Some(NoCoverage),
                "Out of Network" => Some(OutOfNetwork),
                "Testing not Included" => Some(TestingNotIncluded),
                "Request Forwarded To and Decision Response Forthcoming From an External Review Organization" => {
                    Some(
                        RequestForwardedToAndDecisionResponseForthcomingFromAnExternalReviewOrganization,
                    )
                }
                "Claim Can Not Be Identified for Verification" => {
                    Some(ClaimCanNotBeIdentifiedForVerification)
                }
                "Actual Information Different than Reported" => {
                    Some(ActualInformationDifferentThanReported)
                }
                "Actual Information Different - Claim Has Been Re-adjudicated Since Initial Payment" => {
                    Some(
                        ActualInformationDifferentClaimHasBeenReAdjudicatedSinceInitialPayment,
                    )
                }
                "Authorization/Access Restrictions" => {
                    Some(AuthorizationAccessRestrictions)
                }
                "Unable to Respond at Current Time" => Some(UnableToRespondAtCurrentTime),
                "Invalid/Missing Provider Identification" => {
                    Some(InvalidMissingProviderIdentification)
                }
                "Invalid/Missing Provider Name" => Some(InvalidMissingProviderName),
                "Invalid/Missing Provider Specialty" => {
                    Some(InvalidMissingProviderSpecialty)
                }
                "Invalid/Missing Provider Phone Number" => {
                    Some(InvalidMissingProviderPhoneNumber)
                }
                "Invalid/Missing Provider State" => Some(InvalidMissingProviderState),
                "Invalid/Missing Referring Provider Identification Number" => {
                    Some(InvalidMissingReferringProviderIdentificationNumber)
                }
                "Provider is Not Primary Care Physician" => {
                    Some(ProviderIsNotPrimaryCarePhysician)
                }
                "Provider Ineligible for Inquiries" => {
                    Some(ProviderIneligibleForInquiries)
                }
                "Provider Not on File" => Some(ProviderNotOnFile),
                "Service Dates Not Within Provider Plan Enrollment" => {
                    Some(ServiceDatesNotWithinProviderPlanEnrollment)
                }
                "Inquired Benefit Inconsistent with Provider Type" => {
                    Some(InquiredBenefitInconsistentWithProviderType)
                }
                "Inappropriate Product/Service ID Qualifier" => {
                    Some(InappropriateProductServiceIdQualifier)
                }
                "Inappropriate Product/Service ID" => Some(InappropriateProductServiceId),
                "Inappropriate Date" => Some(InappropriateDate),
                "Invalid/Missing Date(s) of Service" => Some(Code57),
                "Invalid/Missing Date-of-Birth" => Some(InvalidMissingDateOfBirth),
                "Invalid/Missing Date-of-Death" => Some(InvalidMissingDateOfDeath),
                "Date of Birth Follows Date(s) of Service" => Some(Code60),
                "Date of Death Precedes Date(s) of Service" => Some(Code61),
                "Date of Service Not Within Allowable Inquiry Period" => {
                    Some(DateOfServiceNotWithinAllowableInquiryPeriod)
                }
                "Date of Service in Future" => Some(DateOfServiceInFuture),
                "Invalid/Missing Patient ID" => Some(InvalidMissingPatientId),
                "Invalid/Missing Patient Name" => Some(InvalidMissingPatientName),
                "Invalid/Missing Patient Gender Code" => {
                    Some(InvalidMissingPatientGenderCode)
                }
                "Patient Not Found" => Some(PatientNotFound),
                "Duplicate Patient ID Number" => Some(DuplicatePatientIdNumber),
                "Inconsistent with Patient's Age" => Some(InconsistentWithPatientsAge),
                "Inconsistent with Patient's Gender" => {
                    Some(InconsistentWithPatientsGender)
                }
                "Patient Birth Date Does Not Match That for the Patient on the Database" => {
                    Some(PatientBirthDateDoesNotMatchThatForThePatientOnTheDatabase)
                }
                "Invalid/Missing Subscriber/Insured ID" => {
                    Some(InvalidMissingSubscriberInsuredId)
                }
                "Invalid/Missing Subscriber/Insured Name" => {
                    Some(InvalidMissingSubscriberInsuredName)
                }
                "Invalid/Missing Subscriber/Insured Gender Code" => {
                    Some(InvalidMissingSubscriberInsuredGenderCode)
                }
                "Subscriber/Insured Not Found" => Some(SubscriberInsuredNotFound),
                "Duplicate Subscriber/Insured ID Number" => {
                    Some(DuplicateSubscriberInsuredIdNumber)
                }
                "Subscriber Found, Patient Not Found" => Some(Code77),
                "Subscriber/Insured Not in Group/Plan Identified" => {
                    Some(SubscriberInsuredNotInGroupPlanIdentified)
                }
                "Invalid Participant Identification" => {
                    Some(InvalidParticipantIdentification)
                }
                "No Response received - Transaction Terminated" => {
                    Some(NoResponseReceivedTransactionTerminated)
                }
                "Invalid or Missing Case Number" => Some(InvalidOrMissingCaseNumber),
                "Not Medically Necessary" => Some(NotMedicallyNecessary),
                "Level of Care Not Appropriate" => Some(LevelOfCareNotAppropriate),
                "Certification Not Required for this Service" => {
                    Some(CertificationNotRequiredForThisService)
                }
                "Certification Responsibility of External Review Organization" => {
                    Some(CertificationResponsibilityOfExternalReviewOrganization)
                }
                "Primary Care Service" => Some(PrimaryCareService),
                "Exceeds Plan Maximums" => Some(ExceedsPlanMaximums),
                "Non-covered Service" => Some(NonCoveredService),
                "No Prior Approval" => Some(NoPriorApproval),
                "Requested Information Not Received" => {
                    Some(RequestedInformationNotReceived)
                }
                "Duplicate Request" => Some(DuplicateRequest),
                "Service Inconsistent with Diagnosis" => {
                    Some(ServiceInconsistentWithDiagnosis)
                }
                "Patient Not Eligible" => Some(PatientNotEligible),
                "Pre-existing Condition" => Some(PreExistingCondition),
                "Invalid or Missing Provider Address" => {
                    Some(InvalidOrMissingProviderAddress)
                }
                "Experimental Service or Procedure" => {
                    Some(ExperimentalServiceOrProcedure)
                }
                "Authorization Number Not Found" => Some(AuthorizationNumberNotFound),
                "Air Brakes - Inoperative, etc." => Some(CodeAB),
                "Accident Damage - Derail/Sideswiped" => {
                    Some(AccidentDamageDerailSideswiped)
                }
                "Requires Primary Care Physician Authorization" => {
                    Some(RequiresPrimaryCarePhysicianAuthorization)
                }
                "Invalid/Missing Diagnosis Code(s)" => Some(CodeAF),
                "Invalid/Missing Procedure Code(s)" => Some(CodeAG),
                "Invalid/Missing Onset of Current Condition or Illness Date" => {
                    Some(InvalidMissingOnsetOfCurrentConditionOrIllnessDate)
                }
                "Invalid/Missing Accident Date" => Some(InvalidMissingAccidentDate),
                "Invalid/Missing Last Menstrual Period Date" => {
                    Some(InvalidMissingLastMenstrualPeriodDate)
                }
                "Invalid/Missing Expected Date of Birth" => {
                    Some(InvalidMissingExpectedDateOfBirth)
                }
                "Invalid/Missing Surgery Date" => Some(InvalidMissingSurgeryDate),
                "Invalid/Missing Admission Date" => Some(InvalidMissingAdmissionDate),
                "Invalid/Missing Discharge Date" => Some(InvalidMissingDischargeDate),
                "Additional Patient Condition Information Required" => {
                    Some(AdditionalPatientConditionInformationRequired)
                }
                "Arrivals" => Some(Arrivals),
                "Reject Due to Air Bags" => Some(RejectDueToAirBags),
                "Reject Due to No Chains" => Some(RejectDueToNoChains),
                "Reject Due to Damps" => Some(RejectDueToDamps),
                "Reject Due to Bearings" => Some(RejectDueToBearings),
                "Reject Due to Load Divider Bad Order" => {
                    Some(RejectDueToLoadDividerBadOrder)
                }
                "Loaded Car, Unable to Load" => Some(CodeBO),
                "Reject Due to Bridge Plate" => Some(RejectDueToBridgePlate),
                "Reject Due to Brake Rigging Beam, Lever" => Some(CodeBR),
                "Reject Due to Bad Order Slides" => Some(RejectDueToBadOrderSlides),
                "Reject Due to Bad Order Valves/Piping" => {
                    Some(RejectDueToBadOrderValvesPiping)
                }
                "Reject Due to Bad Order Walls" => Some(RejectDueToBadOrderWalls),
                "Reject Due to Crank Arm Application" => {
                    Some(RejectDueToCrankArmApplication)
                }
                "Reject Due to Center Bowls, Plates and Pins" => Some(CodeCB),
                "Certification Information Does Not Match Patient" => {
                    Some(CertificationInformationDoesNotMatchPatient)
                }
                "Complete Loading" => Some(CompleteLoading),
                "Released as Railroad Company Material" => {
                    Some(ReleasedAsRailroadCompanyMaterial)
                }
                "Car Not Ordered" => Some(CarNotOrdered),
                "Put on Constructive Placement" => Some(PutOnConstructivePlacement),
                "Wrong Consignee" => Some(WrongConsignee),
                "Release Load Through Bill Connecting Road" => {
                    Some(ReleaseLoadThroughBillConnectingRoad)
                }
                "Release as a Cross-town Load" => Some(ReleaseAsACrossTownLoad),
                "Equipment Not Used" => Some(EquipmentNotUsed),
                "Wrong Car Type" => Some(WrongCarType),
                "Reject Due to Doors" => Some(RejectDueToDoors),
                "Reject Due to Draft Gear - Yoke" => Some(RejectDueToDraftGearYoke),
                "Dismantle" => Some(Dismantle),
                "Departed" => Some(Departed),
                "Reject Due to Dirty" => Some(RejectDueToDirty),
                "Defective Safety Devices" => Some(DefectiveSafetyDevices),
                "Reject Due to Load Dividers, Side Filters, Special Equipment" => {
                    Some(CodeDV)
                }
                "Requested Record Will Not Be Sent; Cannot Identify the Record" => {
                    Some(CodeE1)
                }
                "Requested Record Will Not Be Sent; Need Student or Parent Permission" => {
                    Some(CodeE2)
                }
                "Requested Record Will Not Be Sent" => Some(RequestedRecordWillNotBeSent),
                "Requested Record Will Not Be Sent; Never Enrolled" => Some(CodeE4),
                "Requested Record Will Not Be Sent; No Degree Awarded" => Some(CodeE5),
                "Requested Record Will Not Be Sent; No Grades Posted" => Some(CodeE6),
                "Requested Record Cannot Be Sent Electronically; Record Resides in Paper Format only which Will Be Sent by Mail" => {
                    Some(CodeE7)
                }
                "Requires Medical Review" => Some(RequiresMedicalReview),
                "Empty Equipment Available for Loading" => {
                    Some(EmptyEquipmentAvailableForLoading)
                }
                "Reject Due to Spotted in Error" => Some(RejectDueToSpottedInError),
                "Empty Trailer Flat Release" => Some(EmptyTrailerFlatRelease),
                "Freight Damage Claim" => Some(FreightDamageClaim),
                "Reject Due to Bad Floor" => Some(RejectDueToBadFloor),
                "Release From Demurrage and Start Storage until Waybilled" => {
                    Some(ReleaseFromDemurrageAndStartStorageUntilWaybilled)
                }
                "Reject Due to Handbrake" => Some(RejectDueToHandbrake),
                "Reject Due to Hand Hold, Ladder, Step, Running Boards, Platforms, etc." => {
                    Some(CodeHH)
                }
                "Reject Due to Hot Journal Box" => Some(RejectDueToHotJournalBox),
                "Invalid Authorization Number Format" => {
                    Some(InvalidAuthorizationNumberFormat)
                }
                "Releases an Idler" => Some(ReleasesAnIdler),
                "Industrial Interchange" => Some(IndustrialInterchange),
                "Inappropriate Provider Role" => Some(InappropriateProviderRole),
                "Reject Due to Couplers" => Some(RejectDueToCouplers),
                "Reject Due to Leaking Contents" => Some(RejectDueToLeakingContents),
                "Reject Due to Load Shifted" => Some(RejectDueToLoadShifted),
                "Light Weigh and Restencil" => Some(LightWeighAndRestencil),
                "Missing Authorization Number" => Some(MissingAuthorizationNumber),
                "Move from Current Spot to Next" => Some(MoveFromCurrentSpotToNext),
                "No Certification Information Found" => {
                    Some(NoCertificationInformationFound)
                }
                "Reject Due to Outlet Gate/Valve Lo and Open Hopper" => {
                    Some(RejectDueToOutletGateValveLoAndOpenHopper)
                }
                "Released from Industry to be Inspected" => {
                    Some(ReleasedFromIndustryToBeInspected)
                }
                "Ordered for Replacement" => Some(OrderedForReplacement),
                "Reject Due to Overloaded" => Some(RejectDueToOverloaded),
                "Preventative Maintenance" => Some(PreventativeMaintenance),
                "Released from Industry to Custody of Broker" => {
                    Some(ReleasedFromIndustryToCustodyOfBroker)
                }
                "To be Reloaded" => Some(ToBeReloaded),
                "Reject Due to Refrigeration Unit" => Some(RejectDueToRefrigerationUnit),
                "Reject Due to Roof Hatches" => Some(RejectDueToRoofHatches),
                "Reject Due to Racks (Bi or Tri Levels)" => Some(CodeRK),
                "Released" => Some(Released),
                "Bad Order Reinitialing and Numbering" => {
                    Some(BadOrderReinitialingAndNumbering)
                }
                "Released Partially Unloaded" => Some(ReleasedPartiallyUnloaded),
                "Released Loaded for Line Haul Shipment" => {
                    Some(ReleasedLoadedForLineHaulShipment)
                }
                "Run Through Equipment not Spotted" => {
                    Some(RunThroughEquipmentNotSpotted)
                }
                "Released from Demurrage after being Scrapped" => {
                    Some(ReleasedFromDemurrageAfterBeingScrapped)
                }
                "Released from Shop Track" => Some(ReleasedFromShopTrack),
                "Reject Due to Superstructure - End, Roof and Sides" => Some(CodeSU),
                "Local Waybill" => Some(LocalWaybill),
                "Cannot Identify Provider as TPO (Third Party Organization) Participant" => {
                    Some(CodeT1)
                }
                "Cannot Identify Payer as TPO (Third Party Organization) Participant" => {
                    Some(CodeT2)
                }
                "Cannot Identify Insured as TPO (Third Party Organization) Participant" => {
                    Some(CodeT3)
                }
                "Payer Name or Identifier Missing" => Some(PayerNameOrIdentifierMissing),
                "Certification Information Missing" => {
                    Some(CertificationInformationMissing)
                }
                "Claim does not contain enough information for re-pricing" => {
                    Some(ClaimDoesNotContainEnoughInformationForRePricing)
                }
                "Bad Order to Transfer Lading" => Some(BadOrderToTransferLading),
                "Reject Due to Tie Down Devices" => Some(RejectDueToTieDownDevices),
                "Reject Due to Trailer Hitch" => Some(RejectDueToTrailerHitch),
                "Reject Due to Train Line, Air Hose, Anglecock" => Some(CodeTL),
                "Reject Due to Truck, S-Frame, Bolster" => Some(CodeTR),
                "Reject Due to Uncoupling Rod" => Some(RejectDueToUncouplingRod),
                "Reject Due to Underframe - Including Sills" => {
                    Some(RejectDueToUnderframeIncludingSills)
                }
                "Bad Order for Upgrading of Car" => Some(BadOrderForUpgradingOfCar),
                "Reject Due to Wheel/Axle" => Some(RejectDueToWheelAxle),
                "Bad Order Due to Wreck" => Some(BadOrderDueToWreck),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for RejectReasonCode {
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
    type Value = RejectReasonCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Reject Reason Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RejectReasonCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Reject Reason Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RejectReasonCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Reject Reason Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for RejectReasonCode {
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