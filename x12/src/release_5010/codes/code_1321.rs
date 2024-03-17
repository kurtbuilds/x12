use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1321

See docs at <https://www.stedi.com/edi/x12-005010/element/1321>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionIndicator {
    ///00 - Requested
    Requested,
    ///000 - In Progress
    InProgress,
    ///0A - Automated Export System - Post Departure Authorized Special Status (AES-PASS) Standard
    Code0A,
    ///0B - Automated Export System - Post Departure Authorized Special Status (AES-PASS) Expanded
    Code0B,
    ///0C - Automated Export System - Post Departure Authorized Special Status (AES-PASS) Post Departure
    Code0C,
    ///0D - Facility's Emergency Response Plan Includes Information on Emergency Health Care
    FacilitysEmergencyResponsePlanIncludesInformationOnEmergencyHealthCare,
    ///0E - Facility's Emergency Response Plan Includes Procedures for Informing Public and Local Agencies Responsible for Responding to an Accidental Release
    FacilitysEmergencyResponsePlanIncludesProceduresForInformingPublicAndLocalAgenciesResponsibleForRespondingToAnAccidentalRelease,
    ///0F - Facility has a Clean Air Act Title V Operating Permit
    FacilityHasACleanAirActTitleVOperatingPermit,
    ///0G - Facility has a Written Emergency Response Plan
    FacilityHasAWrittenEmergencyResponsePlan,
    ///0H - Facility has Reportable Accidents
    FacilityHasReportableAccidents,
    /**0I - Facility is Covered by the Emergency Planning and
Community Right to Know Act Section 302*/
    Code0I,
    /**0J - Facility is Covered by the Occupational Safety and
Health Act (OSHA) Process Safety Management
Standard*/
    Code0J,
    /**0K - Facility is Included in the Community Emergency
Response Plan*/
    Code0K,
    ///0L - Hazardous Waste Mixed with Resource Conservation Recovery Act (RCRA)-Radioactive Material
    Code0L,
    ///0M - Offsite Responders Notified
    OffsiteRespondersNotified,
    ///0N - Precipitation Present
    PrecipitationPresent,
    ///0O - Disabled Veteran
    DisabledVeteran,
    ///0P - Servicer has Advanced Funds to Pay for Delinquent Taxes on Non-escrowed Mortgage
    ServicerHasAdvancedFundsToPayForDelinquentTaxesOnNonEscrowedMortgage,
    ///0Q - Property Has Fire Insurance Only that was not Lender Placed
    PropertyHasFireInsuranceOnlyThatWasNotLenderPlaced,
    ///0R - Reported but Unconfirmed
    ReportedButUnconfirmed,
    ///0S - Has Smoke Alarms
    HasSmokeAlarms,
    ///0T - Operates as a Holding Company
    OperatesAsAHoldingCompany,
    ///0U - Optimum
    Optimum,
    ///0V - Renewed
    Renewed,
    ///0W - Highest Educational Level
    HighestEducationalLevel,
    ///0X - Principal Certificate
    PrincipalCertificate,
    ///0Y - Inservice Education Completed
    InserviceEducationCompleted,
    ///0Z - Main Assignment
    MainAssignment,
    ///01 - Patient was admitted to a hospital
    PatientWasAdmittedToAHospital,
    ///1A - Patient is receiving anti-fungal therapy
    PatientIsReceivingAntiFungalTherapy,
    ///1B - Property is occupied by owner
    PropertyIsOccupiedByOwner,
    ///1C - Property is occupied by tenant
    PropertyIsOccupiedByTenant,
    ///1D - Property is vacant
    PropertyIsVacant,
    ///1E - Location is urban
    LocationIsUrban,
    ///1F - Location is suburban
    LocationIsSuburban,
    ///1G - Location is rural
    LocationIsRural,
    ///1H - Built-up over 75%
    Code1H,
    ///1I - Built-up 25 - 75%
    Code1I,
    ///1J - Built-up under 25%
    Code1J,
    ///1K - Growth rate is rapid
    GrowthRateIsRapid,
    ///1L - Class I-Left
    ClassILeft,
    ///1M - Growth rate is stable
    GrowthRateIsStable,
    ///1N - Growth rate is slow
    GrowthRateIsSlow,
    ///1O - Property values are increasing
    PropertyValuesAreIncreasing,
    ///1P - Property values are stable
    PropertyValuesAreStable,
    ///1Q - Property values are declining
    PropertyValuesAreDeclining,
    ///1R - Class I-Right
    ClassIRight,
    ///1S - Demand or supply is in shortage
    DemandOrSupplyIsInShortage,
    ///1T - Demand or supply is in balance
    DemandOrSupplyIsInBalance,
    ///1U - Demand or supply is over supply
    DemandOrSupplyIsOverSupply,
    ///1V - Marketing time is under 3 months
    MarketingTimeIsUnder3Months,
    ///1W - Marketing time is 3 to 6 months
    MarketingTimeIs3To6Months,
    ///1X - Marketing time is over 6 months
    MarketingTimeIsOver6Months,
    ///1Y - Predominant occupancy is the owner
    PredominantOccupancyIsTheOwner,
    ///1Z - Predominant occupancy is the tenant
    PredominantOccupancyIsTheTenant,
    ///02 - Patient was bed confined before the ambulance service
    PatientWasBedConfinedBeforeTheAmbulanceService,
    ///2A - Patient is receiving oral anti-fungal therapy
    PatientIsReceivingOralAntiFungalTherapy,
    ///2B - Predominant occupancy is vacant (0-5%)
    Code2B,
    ///2C - Predominant occupancy is vacant (over 5%)
    Code2C,
    ///2D - Developer or builder is in control of the Home Owners Association
    DeveloperOrBuilderIsInControlOfTheHomeOwnersAssociation,
    ///2E - Site is a corner lot
    SiteIsACornerLot,
    ///2F - Zoning compliance is legal
    ZoningComplianceIsLegal,
    ///2G - Zoning compliance is legal nonconforming (grandfather use)
    Code2G,
    ///2H - Zoning compliance is illegal
    ZoningComplianceIsIllegal,
    ///2I - There is no zoning
    ThereIsNoZoning,
    ///2J - Highest and best use as improved is the present use
    HighestAndBestUseAsImprovedIsThePresentUse,
    ///2K - Highest and best use as improved is other use
    HighestAndBestUseAsImprovedIsOtherUse,
    ///2L - Class II-Left
    ClassIiLeft,
    ///2M - Property is located in a Federal Emergency Management Administration special flood hazard area
    PropertyIsLocatedInAFederalEmergencyManagementAdministrationSpecialFloodHazardArea,
    ///2N - Appraisal is made "as is"
    Code2N,
    ///2O - Appraisal is made subject to the repairs, alterations, inspections, or conditions listed
    Code2O,
    ///2P - Appraisal is made subject to the completion per plans and specifications
    AppraisalIsMadeSubjectToTheCompletionPerPlansAndSpecifications,
    ///2Q - Project type is planned unit development (PUD)
    Code2Q,
    ///2R - Class II-Right
    ClassIiRight,
    ///2S - Project type is condominium
    ProjectTypeIsCondominium,
    ///2T - Property rights are fee simple
    PropertyRightsAreFeeSimple,
    ///2U - Property rights are leasehold
    PropertyRightsAreLeasehold,
    ///2V - Supervisor appraiser inspected the property per supervisory appraiser's certification
    SupervisorAppraiserInspectedThePropertyPerSupervisoryAppraisersCertification,
    ///2W - Property was sold within last 12 months
    PropertyWasSoldWithinLast12Months,
    ///2X - Appraiser signed statement of limiting conditions and disclaimer
    AppraiserSignedStatementOfLimitingConditionsAndDisclaimer,
    ///2Y - Ownership interest in a property
    OwnershipInterestInAProperty,
    ///2Z - Termination
    Termination,
    ///03 - Patient was bed confined after the ambulance service
    PatientWasBedConfinedAfterTheAmbulanceService,
    ///3A - Patient is receiving topical anti-fungal therapy
    PatientIsReceivingTopicalAntiFungalTherapy,
    ///3B - Points Paid by Seller
    PointsPaidBySeller,
    ///3C - Points Paid by Buyer
    PointsPaidByBuyer,
    ///3D - Seller Concession
    SellerConcession,
    ///3E - Letter of Certification
    LetterOfCertification,
    ///3F - Verbal Report Needed
    VerbalReportNeeded,
    ///3G - Any Relationship Between Owner and Occupant
    AnyRelationshipBetweenOwnerAndOccupant,
    ///3H - Map and Directions to Remote Properties to Follow
    MapAndDirectionsToRemotePropertiesToFollow,
    ///3I - Ground Lease to Follow
    GroundLeaseToFollow,
    ///3J - Disclosure Statement to Follow
    DisclosureStatementToFollow,
    ///3K - Copy of Property Listing to Follow
    CopyOfPropertyListingToFollow,
    ///3L - Class III-Left
    ClassIiiLeft,
    ///3M - Copy of Title Report Plat Map to Follow
    CopyOfTitleReportPlatMapToFollow,
    ///3N - Property Tax Bill to Follow
    PropertyTaxBillToFollow,
    ///3O - Engineering or Soil Report to Follow
    EngineeringOrSoilReportToFollow,
    ///3P - Sales Contract Available
    SalesContractAvailable,
    ///3Q - Leave Will be Taken
    LeaveWillBeTaken,
    ///3R - Class III-Right
    ClassIiiRight,
    ///3S - Approved
    Approved,
    ///3T - Balance Sheet does not balance
    BalanceSheetDoesNotBalance,
    ///3U - Banking done through Parent Company
    BankingDoneThroughParentCompany,
    ///3V - Banking done through Related Concern
    BankingDoneThroughRelatedConcern,
    ///3W - Banking done through Subsidiary
    BankingDoneThroughSubsidiary,
    ///3X - Can not determine if subject engaged in business
    CanNotDetermineIfSubjectEngagedInBusiness,
    ///3Y - Deteriorated
    Deteriorated,
    ///3Z - Detrimental Information Received
    DetrimentalInformationReceived,
    ///04 - Patient was moved by stretcher
    PatientWasMovedByStretcher,
    ///4A - Services are rendered within Hospice-elected period of coverage
    ServicesAreRenderedWithinHospiceElectedPeriodOfCoverage,
    ///4B - Accidents
    Accidents,
    ///4C - Account Representative Transfer
    AccountRepresentativeTransfer,
    ///4D - Additional Coverage
    AdditionalCoverage,
    ///4E - Advice to Stop
    AdviceToStop,
    ///4F - Agent Replacement
    AgentReplacement,
    ///4G - Backup Withholding
    BackupWithholding,
    ///4H - Current Employer
    CurrentEmployer,
    ///4I - Current Occupation
    CurrentOccupation,
    ///4J - Employer Reimbursement
    EmployerReimbursement,
    ///4K - Employee Retirement Income Security Act (ERISA)
    Code4K,
    ///4L - Expected Changes
    ExpectedChanges,
    ///4M - Experimental
    Experimental,
    ///4N - Foreign Flight
    ForeignFlight,
    ///4O - Future Involvement
    FutureInvolvement,
    ///4P - Grounding, Fine, Reprimand
    Code4P,
    ///4Q - Group Disability Insurance Conversion
    GroupDisabilityInsuranceConversion,
    ///4R - Group Disability Insurance Offset
    GroupDisabilityInsuranceOffset,
    ///4S - Group Disability Insurance Participation
    GroupDisabilityInsuranceParticipation,
    ///4T - Group Disability Insurance Top Up
    GroupDisabilityInsuranceTopUp,
    ///4U - Home Employment
    HomeEmployment,
    ///4V - Information Omitted
    InformationOmitted,
    ///4W - Injury Benefits
    InjuryBenefits,
    ///4X - Issue at Higher Premiums
    IssueAtHigherPremiums,
    ///4Y - Issue With Exclusions
    IssueWithExclusions,
    ///4Z - Issue Without Benefits
    IssueWithoutBenefits,
    ///05 - Patient was unconscious or in shock
    PatientWasUnconsciousOrInShock,
    ///5A - Treatment is rendered related to the terminal illness
    TreatmentIsRenderedRelatedToTheTerminalIllness,
    ///5B - Certified Aftermarket Parts Association (CAPA) Only
    Code5B,
    ///5C - Certified Aftermarket Parts Association (CAPA) Preferred
    Code5C,
    ///5D - Juvenile Seen
    JuvenileSeen,
    ///5E - Medical Treatment
    MedicalTreatment,
    ///5F - Military Aviation
    MilitaryAviation,
    ///5G - New Group
    NewGroup,
    ///5H - Other Coverage Offset
    OtherCoverageOffset,
    ///5I - Other Principals Being Insured
    OtherPrincipalsBeingInsured,
    ///5J - Owner Active in Business
    OwnerActiveInBusiness,
    ///5K - Payroll Deduction
    PayrollDeduction,
    ///5L - Prepaid
    Prepaid,
    ///5M - Previous Application
    PreviousApplication,
    ///5N - Primary Occupation
    PrimaryOccupation,
    ///5O - Racing Accident
    RacingAccident,
    ///5P - Replacement
    Replacement,
    ///5Q - Resides With Applicant
    ResidesWithApplicant,
    ///5R - Gender Distinct
    GenderDistinct,
    ///5S - Sibling Coverage
    SiblingCoverage,
    ///5T - Sickness Benefits
    SicknessBenefits,
    ///5U - Special Dating
    SpecialDating,
    ///5V - Spousal Consent
    SpousalConsent,
    ///5W - Suitability Analysis
    SuitabilityAnalysis,
    ///5X - Suitable for Coverage
    SuitableForCoverage,
    ///5Y - Taxable
    Taxable,
    ///5Z - This Company Replacement
    ThisCompanyReplacement,
    ///06 - Patient was transported in an emergency situation
    PatientWasTransportedInAnEmergencySituation,
    ///6A - Treatment is rendered by a Hospice employed physician
    TreatmentIsRenderedByAHospiceEmployedPhysician,
    ///6B - United States Citizen
    UnitedStatesCitizen,
    ///6C - Permanent Resident Alien
    PermanentResidentAlien,
    ///6D - Borrower is First Time Homebuyer
    BorrowerIsFirstTimeHomebuyer,
    ///6E - Unemployment Claims
    UnemploymentClaims,
    ///6F - Unemployment Insurance Eligibility
    UnemploymentInsuranceEligibility,
    ///6G - Work Status
    WorkStatus,
    ///6H - Workers Compensation Eligible
    WorkersCompensationEligible,
    ///6I - Factored on Recourse Basis
    FactoredOnRecourseBasis,
    ///6J - Factored with Advances
    FactoredWithAdvances,
    ///6K - Figures are Actual
    FiguresAreActual,
    ///6L - Figures are Anticipated
    FiguresAreAnticipated,
    ///6M - Figures are Estimated
    FiguresAreEstimated,
    ///6N - Figures are Modified
    FiguresAreModified,
    ///6O - Figures are Projected
    FiguresAreProjected,
    ///6P - Government Business Number Unavailable
    GovernmentBusinessNumberUnavailable,
    ///6Q - Goodwill Origin Purchased from Bankrupt Company
    GoodwillOriginPurchasedFromBankruptCompany,
    ///6R - Goodwill Origin Rented
    GoodwillOriginRented,
    ///6S - Has no ownership
    HasNoOwnership,
    ///6T - Improved
    Improved,
    ///6U - Intangibles breakdown available
    IntangiblesBreakdownAvailable,
    ///6V - Intangibles include Organizational Expense
    IntangiblesIncludeOrganizationalExpense,
    ///6W - Intercompany relations consist of Loans and Advances
    IntercompanyRelationsConsistOfLoansAndAdvances,
    ///6X - Intercompany relations consist of Merchandise Transactions
    IntercompanyRelationsConsistOfMerchandiseTransactions,
    ///6Y - Intercompany relations consist of Service Transactions
    IntercompanyRelationsConsistOfServiceTransactions,
    ///6Z - Local banking utilized on a transfer account basis
    LocalBankingUtilizedOnATransferAccountBasis,
    ///07 - Patient had to be physically restrained
    PatientHadToBePhysicallyRestrained,
    ///7A - Treatment is rendered by a private attending physician
    TreatmentIsRenderedByAPrivateAttendingPhysician,
    ///7B - Medications Ordered are being Administered Intramuscularly
    MedicationsOrderedAreBeingAdministeredIntramuscularly,
    ///7C - Medications Ordered are being Administered Intravenously
    MedicationsOrderedAreBeingAdministeredIntravenously,
    ///7D - Medications Ordered are being Administered Orally
    MedicationsOrderedAreBeingAdministeredOrally,
    ///7E - Maintains no Inventory
    MaintainsNoInventory,
    ///7F - Medications Ordered are being Administered Subcutaneously
    MedicationsOrderedAreBeingAdministeredSubcutaneously,
    ///7G - Majority
    Majority,
    ///7H - Marketable Securities valued at cost
    MarketableSecuritiesValuedAtCost,
    ///7I - Marketable Securities valued at lower of cost or market
    MarketableSecuritiesValuedAtLowerOfCostOrMarket,
    ///7J - Interior Access Denied
    InteriorAccessDenied,
    ///7K - Repairs are Recommended
    RepairsAreRecommended,
    ///7L - Loan Originated under Shared Equity Plan
    LoanOriginatedUnderSharedEquityPlan,
    ///7M - Title and or Legal Issues Exist
    TitleAndOrLegalIssuesExist,
    ///7N - Environmental Issues Exist
    EnvironmentalIssuesExist,
    ///7O - Property is Listed As Is
    PropertyIsListedAsIs,
    ///7P - Property is Listed as Repaired
    PropertyIsListedAsRepaired,
    ///7Q - Vacancy Rate is Greater Than 5 Percent to 10 Percent
    VacancyRateIsGreaterThan5PercentTo10Percent,
    ///7R - Vacancy Rate is Greater Than 10 Percent to 20 Percent
    VacancyRateIsGreaterThan10PercentTo20Percent,
    ///7S - Vacancy Rate is Greater Than 20 Percent
    VacancyRateIsGreaterThan20Percent,
    ///7T - Most Comparable Property
    MostComparableProperty,
    ///7U - Anticipate Issues which Affect Ability to Secure Financing
    AnticipateIssuesWhichAffectAbilityToSecureFinancing,
    ///7V - Points are Paid by Seller
    PointsArePaidBySeller,
    ///7W - Property Covered by Flood Insurance Policy
    PropertyCoveredByFloodInsurancePolicy,
    ///7X - Property Covered by Earthquake Insurance Policy
    PropertyCoveredByEarthquakeInsurancePolicy,
    ///7Y - Points are Negotiable
    PointsAreNegotiable,
    ///7Z - Property is Currently Listed with a Real Estate Firm
    PropertyIsCurrentlyListedWithARealEstateFirm,
    ///08 - Patient had visible hemorrhaging
    PatientHadVisibleHemorrhaging,
    ///8A - Treatment is curative
    TreatmentIsCurative,
    ///8B - Income or Assets of Another Used
    IncomeOrAssetsOfAnotherUsed,
    ///8C - Disclosure of Someone Else's Liabilities Required
    DisclosureOfSomeoneElsesLiabilitiesRequired,
    ///8D - Property Improvements "to be made"
    Code8D,
    ///8E - Property Improvements "have been made"
    Code8E,
    ///8F - Distant Suburban
    DistantSuburban,
    ///8G - Self Employed
    SelfEmployed,
    ///8H - Liability to be Satisfied
    LiabilityToBeSatisfied,
    ///8I - Are Assets/Liabilities Reported Jointly
    AreAssetsLiabilitiesReportedJointly,
    ///8J - Location is Farm
    LocationIsFarm,
    ///8K - Location is Resort
    LocationIsResort,
    ///8L - Shortage Exist for Competing Listings
    ShortageExistForCompetingListings,
    ///8M - Competing Listings are in Balance
    CompetingListingsAreInBalance,
    ///8N - Oversupply Exist for Competing Listings
    OversupplyExistForCompetingListings,
    ///8O - Incentives are Offered
    IncentivesAreOffered,
    ///8P - Listed Property has been Inspected
    ListedPropertyHasBeenInspected,
    ///8Q - Sale Property has been Inspected
    SalePropertyHasBeenInspected,
    ///8R - General Marketing Condition is Depressed
    GeneralMarketingConditionIsDepressed,
    ///8S - General Marketing Condition is Slow
    GeneralMarketingConditionIsSlow,
    ///8T - General Marketing Condition is Static
    GeneralMarketingConditionIsStatic,
    ///8U - General Marketing Condition is Improving
    GeneralMarketingConditionIsImproving,
    ///8V - General Marketing Condition is Excellent
    GeneralMarketingConditionIsExcellent,
    ///8W - Employment Conditions are Stable
    EmploymentConditionsAreStable,
    ///8X - Employment Conditions are Declining
    EmploymentConditionsAreDeclining,
    ///8Y - Employment Conditions are Increasing
    EmploymentConditionsAreIncreasing,
    ///8Z - Overimprovement Condition Exists
    OverimprovementConditionExists,
    ///09 - Ambulance service was medically necessary
    AmbulanceServiceWasMedicallyNecessary,
    ///9A - Treatment is Palliative
    TreatmentIsPalliative,
    ///9B - Involuntary Committal
    InvoluntaryCommittal,
    ///9C - Lack of Available Equipment
    LackOfAvailableEquipment,
    ///9D - Lack of Appropriate Facility within Reasonable Distance to Treat Patient in the Event of Complications
    LackOfAppropriateFacilityWithinReasonableDistanceToTreatPatientInTheEventOfComplications,
    ///9E - Sudden Onset of Disorientation
    SuddenOnsetOfDisorientation,
    ///9F - Sudden Onset of Severe, Incapacitating Pain
    Code9F,
    ///9G - Continuous Hemorrhage from any Site with Abnormal Lab Values
    ContinuousHemorrhageFromAnySiteWithAbnormalLabValues,
    ///9H - Patient Requires Intensive IV Therapy
    PatientRequiresIntensiveIvTherapy,
    ///9I - Patient Requires Volume Expanders
    PatientRequiresVolumeExpanders,
    ///9J - Patient Requires Protective Isolation
    PatientRequiresProtectiveIsolation,
    ///9K - Patient Requires Frequent Monitoring
    PatientRequiresFrequentMonitoring,
    ///9L - Patient Requires Extended Post-operative Observation
    PatientRequiresExtendedPostOperativeObservation,
    ///9M - Foreclosure Proceedings Have Begun
    ForeclosureProceedingsHaveBegun,
    ///9N - Underimprovement Condition Exists
    UnderimprovementConditionExists,
    ///9O - Marketability of Property is Excellent
    MarketabilityOfPropertyIsExcellent,
    ///9P - Marketability of Property is Good
    MarketabilityOfPropertyIsGood,
    ///9Q - Marketability of Property is Fair
    MarketabilityOfPropertyIsFair,
    ///9R - Marketability of Property is Poor
    MarketabilityOfPropertyIsPoor,
    ///9S - Fees are Current
    FeesAreCurrent,
    ///9T - Fees Include Tennis
    FeesIncludeTennis,
    ///9U - Fees Include Pool
    FeesIncludePool,
    ///9V - Fees Include Insurance
    FeesIncludeInsurance,
    ///9W - Fees Include Landscape
    FeesIncludeLandscape,
    ///9X - Fees Include Other Amenities
    FeesIncludeOtherAmenities,
    ///9Y - Most Likely Buyer is Owner Occupant
    MostLikelyBuyerIsOwnerOccupant,
    ///9Z - Most Likely Buyer is Investor
    MostLikelyBuyerIsInvestor,
    ///10 - Patient is ambulatory
    PatientIsAmbulatory,
    ///11 - Ambulation is Impaired and Walking Aid is Used for Therapy or Mobility
    AmbulationIsImpairedAndWalkingAidIsUsedForTherapyOrMobility,
    ///12 - Patient is confined to a bed or chair
    PatientIsConfinedToABedOrChair,
    ///13 - Patient is Confined to a Room or an Area Without Bathroom Facilities
    PatientIsConfinedToARoomOrAnAreaWithoutBathroomFacilities,
    ///14 - Ambulation is Impaired and Walking Aid is Used for Mobility
    AmbulationIsImpairedAndWalkingAidIsUsedForMobility,
    ///15 - Patient Condition Requires Positioning of the Body or Attachments Which Would Not be Feasible With the Use of an Ordinary Bed
    PatientConditionRequiresPositioningOfTheBodyOrAttachmentsWhichWouldNotBeFeasibleWithTheUseOfAnOrdinaryBed,
    ///16 - Patient needs a trapeze bar to sit up due to respiratory condition or change body positions for other medical reasons
    PatientNeedsATrapezeBarToSitUpDueToRespiratoryConditionOrChangeBodyPositionsForOtherMedicalReasons,
    ///17 - Patient's Ability to Breathe is Severely Impaired
    PatientsAbilityToBreatheIsSeverelyImpaired,
    ///18 - Patient condition requires frequent and/or immediate changes in body positions
    PatientConditionRequiresFrequentAndOrImmediateChangesInBodyPositions,
    ///19 - Patient can operate controls
    PatientCanOperateControls,
    ///20 - Siderails Are to be Attached to a Hospital Bed Owned by the Beneficiary
    SiderailsAreToBeAttachedToAHospitalBedOwnedByTheBeneficiary,
    ///21 - Patient owns equipment
    PatientOwnsEquipment,
    ///22 - Mattress or Siderails are Being Used with Prescribed Medically Necessary Hospital Bed Owned by the Beneficiary
    MattressOrSiderailsAreBeingUsedWithPrescribedMedicallyNecessaryHospitalBedOwnedByTheBeneficiary,
    ///23 - Patient Needs Lift to Get In or Out of Bed or to Assist in Transfer from Bed to Wheelchair
    PatientNeedsLiftToGetInOrOutOfBedOrToAssistInTransferFromBedToWheelchair,
    ///24 - Patient has an orthopedic impairment requiring traction equipment which prevents ambulation during period of use
    PatientHasAnOrthopedicImpairmentRequiringTractionEquipmentWhichPreventsAmbulationDuringPeriodOfUse,
    ///25 - Item has been prescribed as part of a planned regimen of treatment in patient home
    ItemHasBeenPrescribedAsPartOfAPlannedRegimenOfTreatmentInPatientHome,
    ///26 - Patient is highly susceptible to decubitus ulcers
    PatientIsHighlySusceptibleToDecubitusUlcers,
    ///27 - Patient or a care-giver has been instructed in use of equipment
    PatientOrACareGiverHasBeenInstructedInUseOfEquipment,
    ///28 - Patient has poor diabetic control
    PatientHasPoorDiabeticControl,
    ///29 - A 6-7 hour nocturnal study documents 30 episodes of apnea each lasting more than 10 seconds
    A67HourNocturnalStudyDocuments30EpisodesOfApneaEachLastingMoreThan10Seconds,
    ///30 - Without the equipment, the patient would require surgery
    Code30,
    ///31 - Patient has had a total knee replacement
    PatientHasHadATotalKneeReplacement,
    ///32 - Patient has intractable lymphedema of the extremities
    PatientHasIntractableLymphedemaOfTheExtremities,
    ///33 - Patient is in a nursing home
    PatientIsInANursingHome,
    ///34 - Patient is conscious
    PatientIsConscious,
    ///35 - This Feeding is the Only Form of Nutritional Intake for This Patient
    ThisFeedingIsTheOnlyFormOfNutritionalIntakeForThisPatient,
    ///36 - Patient was administered premix
    PatientWasAdministeredPremix,
    ///37 - Oxygen delivery equipment is stationary
    OxygenDeliveryEquipmentIsStationary,
    ///38 - Certification signed by the physician is on file at the supplier's office
    CertificationSignedByThePhysicianIsOnFileAtTheSuppliersOffice,
    ///39 - Patient Has Mobilizing Respiratory Tract Secretions
    PatientHasMobilizingRespiratoryTractSecretions,
    ///40 - Patient or Caregiver is Capable of Using the Equipment Without Technical or Professional Supervision
    PatientOrCaregiverIsCapableOfUsingTheEquipmentWithoutTechnicalOrProfessionalSupervision,
    ///41 - Patient or Caregiver is Unable to Propel or Lift a Standard Weight Wheelchair
    PatientOrCaregiverIsUnableToPropelOrLiftAStandardWeightWheelchair,
    ///42 - Patient Requires Leg Elevation for Edema or Body Alignment
    PatientRequiresLegElevationForEdemaOrBodyAlignment,
    ///43 - Patient Weight or Usage Needs Necessitate a Heavy Duty Wheelchair
    PatientWeightOrUsageNeedsNecessitateAHeavyDutyWheelchair,
    ///44 - Patient Requires Reclining Function of a Wheelchair
    PatientRequiresRecliningFunctionOfAWheelchair,
    ///45 - Patient is Unable to Operate a Wheelchair Manually
    PatientIsUnableToOperateAWheelchairManually,
    ///46 - Patient or Caregiver Requires Side Transfer into Wheelchair, Commode or Other
    Code46,
    ///47 - Advertisement Run Condition
    AdvertisementRunCondition,
    ///48 - Individual Paid for Last Day Worked
    IndividualPaidForLastDayWorked,
    ///49 - Full Wages Paid for Date of Injury
    FullWagesPaidForDateOfInjury,
    ///50 - Citation or Ticket Issued
    CitationOrTicketIssued,
    ///51 - Individual is Member of Policyholder's Household
    IndividualIsMemberOfPolicyholdersHousehold,
    ///52 - Individual Permitted to Use Vehicle
    IndividualPermittedToUseVehicle,
    ///53 - Individual Wore Seatbelt
    IndividualWoreSeatbelt,
    ///54 - Child Restraint Device in Vehicle
    ChildRestraintDeviceInVehicle,
    ///55 - Child Restraint Device Used
    ChildRestraintDeviceUsed,
    ///56 - Individual Injured
    IndividualInjured,
    ///57 - Individual Transported to Another Location
    IndividualTransportedToAnotherLocation,
    ///58 - Durable Medical Equipment (DME) Purchased New
    Code58,
    ///59 - Durable Medical Equipment (DME) Is Under Warranty
    Code59,
    ///60 - Transportation Was To the Nearest Facility
    TransportationWasToTheNearestFacility,
    ///61 - Employee is Exempt
    EmployeeIsExempt,
    ///62 - Claimant is Covered on the Employer's Long-term Disability Plan
    ClaimantIsCoveredOnTheEmployersLongTermDisabilityPlan,
    ///63 - Employee's Job Responsibilities Changed Due to the Disabling Condition
    EmployeesJobResponsibilitiesChangedDueToTheDisablingCondition,
    ///64 - Employer Has a Return to Work Policy for Disabled Employees
    EmployerHasAReturnToWorkPolicyForDisabledEmployees,
    ///65 - Open
    Open,
    ///66 - Normal
    Normal,
    ///67 - Closed-moderate
    ClosedModerate,
    ///68 - Severe
    Severe,
    ///69 - Moderate
    Moderate,
    ///70 - Straight
    Straight,
    ///71 - Convex
    Convex,
    ///72 - Concave
    Concave,
    ///73 - Double Protrusion
    DoubleProtrusion,
    ///74 - No Crossbite
    NoCrossbite,
    ///75 - Posterior
    Posterior,
    ///76 - Anterior
    Anterior,
    ///77 - Maxillary
    Maxillary,
    ///78 - Mandibular
    Mandibular,
    ///79 - Right
    Right,
    ///80 - Left
    Left,
    ///81 - Maxillary Moderate
    MaxillaryModerate,
    ///82 - Mandibular Moderate
    MandibularModerate,
    ///83 - Maxillary Severe
    MaxillarySevere,
    ///84 - Mandibular Severe
    MandibularSevere,
    ///85 - Income Has Been Verified
    IncomeHasBeenVerified,
    ///86 - Person Has Been Interviewed
    PersonHasBeenInterviewed,
    ///87 - Rent Has Been Verified
    RentHasBeenVerified,
    ///88 - Employer Has Been Verified
    EmployerHasBeenVerified,
    ///89 - Position Has Been Verified
    PositionHasBeenVerified,
    ///90 - Inquiry Has Been Verified
    InquiryHasBeenVerified,
    ///91 - Outstanding Judgments
    OutstandingJudgments,
    ///92 - Declared Bankruptcy in Past 7 Years
    DeclaredBankruptcyInPast7Years,
    ///93 - Foreclosure or Deed in Lieu in Past 7 Years
    ForeclosureOrDeedInLieuInPast7Years,
    ///94 - Party to Lawsuit
    PartyToLawsuit,
    ///95 - Obligated on a Loan Foreclosed, Deed in Lieu of Judgment
    Code95,
    ///96 - Currently Delinquent or in Default
    CurrentlyDelinquentOrInDefault,
    ///97 - Obligated to Pay Alimony, Child Support or Maintenance
    Code97,
    ///98 - Part of Down Payment Borrowed
    PartOfDownPaymentBorrowed,
    ///99 - Co-maker or Endorser on a Note
    CoMakerOrEndorserOnANote,
    ///A0 - Liability Coverage Will Transfer
    LiabilityCoverageWillTransfer,
    ///A1 - Most Likely Buyer is Other Person or Entity
    MostLikelyBuyerIsOtherPersonOrEntity,
    ///A2 - Potential Financing is Fannie Mae
    PotentialFinancingIsFannieMae,
    ///A3 - Suppress Paper Endorsement
    SuppressPaperEndorsement,
    ///A4 - Do Not Suppress Paper Endorsement
    DoNotSuppressPaperEndorsement,
    ///A5 - Escrow
    Escrow,
    ///A6 - Teaching Minor
    TeachingMinor,
    ///A7 - Sub-servicer Submitted
    SubServicerSubmitted,
    ///A8 - First Mortgage
    FirstMortgage,
    ///A9 - Second Mortgage
    SecondMortgage,
    ///AA - Amputation
    Amputation,
    ///AB - Address Skip Begin
    AddressSkipBegin,
    ///AC - Address Corrected
    AddressCorrected,
    ///AD - Automatic Drill Time Calculated
    AutomaticDrillTimeCalculated,
    ///AE - Automatic Edging Time Calculated
    AutomaticEdgingTimeCalculated,
    ///AF - Automatically Select
    AutomaticallySelect,
    ///AFM - Accepting Family Members
    AcceptingFamilyMembers,
    ///AG - Agitated
    Agitated,
    ///AH - Automatically Search and List
    AutomaticallySearchAndList,
    ///AI - Address Incorrect
    AddressIncorrect,
    ///AJ - Assumable
    Assumable,
    ///AK - Potential Financing is Cash
    PotentialFinancingIsCash,
    ///AL - Ambulation Limitations
    AmbulationLimitations,
    ///AM - Potential Financing is Outside Lender
    PotentialFinancingIsOutsideLender,
    ///AN - Address Incomplete
    AddressIncomplete,
    ///AO - Accept Certification without Changes
    AcceptCertificationWithoutChanges,
    ///AP - Alley is Public
    AlleyIsPublic,
    ///AQ - Potential Financing is Federal Housing Administration
    PotentialFinancingIsFederalHousingAdministration,
    ///AR - Address Skip Resolved
    AddressSkipResolved,
    ///AS - Address Skip Exhaust
    AddressSkipExhaust,
    ///AT - Accept Statement of Limiting Conditions without Changes
    AcceptStatementOfLimitingConditionsWithoutChanges,
    ///AU - Automatic Underside Time Calculated
    AutomaticUndersideTimeCalculated,
    ///AV - Available - Not Used
    AvailableNotUsed,
    ///AW - Accept Certification with Changes
    AcceptCertificationWithChanges,
    ///AX - Accept Statement of Limiting Conditions with Changes
    AcceptStatementOfLimitingConditionsWithChanges,
    ///AY - Adjacent Track Occupied
    AdjacentTrackOccupied,
    ///AZ - Potential Financing is Veterans Affairs
    PotentialFinancingIsVeteransAffairs,
    ///B0 - Uninsured Motorist Coverage Will Transfer
    UninsuredMotoristCoverageWillTransfer,
    ///B1 - Mortgage in Foreclosure
    MortgageInForeclosure,
    ///B2 - Real Estate Owned (REO) Mortgage
    CodeB2,
    ///B3 - Potential Financing is Contract for Deed
    PotentialFinancingIsContractForDeed,
    ///B4 - Only the Exterior has been Inspected
    OnlyTheExteriorHasBeenInspected,
    ///B5 - Real Estate Owned Property or Foreclosure Property
    RealEstateOwnedPropertyOrForeclosureProperty,
    ///B6 - Number of Comparable Listings is Normal
    NumberOfComparableListingsIsNormal,
    ///B7 - Number of Comparable Listings is an Oversupply
    NumberOfComparableListingsIsAnOversupply,
    ///B8 - Number of Comparable Listings is a Shortage
    NumberOfComparableListingsIsAShortage,
    ///B9 - Property Management Expenses Outstanding
    PropertyManagementExpensesOutstanding,
    ///BA - Borrower Letter Attempt
    BorrowerLetterAttempt,
    ///BB - Building or Mobile Home is in a Coastal Barrier Resources Area
    BuildingOrMobileHomeIsInACoastalBarrierResourcesArea,
    ///BC - Borrower Telephone Contact
    BorrowerTelephoneContact,
    ///BD - Business Pending
    BusinessPending,
    ///BE - Borrower Letter Contact
    BorrowerLetterContact,
    ///BF - Marketable Securities valued at market
    MarketableSecuritiesValuedAtMarket,
    ///BG - Appropriate Improvement Condition Exists
    AppropriateImprovementConditionExists,
    ///BH - Name unknown to local authorities
    NameUnknownToLocalAuthorities,
    ///BI - No manufacturing done on Premises
    NoManufacturingDoneOnPremises,
    ///BJ - Occasional
    Occasional,
    ///BK - Officer or owner in other Businesses
    OfficerOrOwnerInOtherBusinesses,
    ///BL - Bowel Limitations, Bladder Limitations, or both (Incontinence)
    CodeBL,
    ///BM - Old
    Old,
    ///BN - Operates on part time basis
    OperatesOnPartTimeBasis,
    ///BO - Parent Financial Statement Used
    ParentFinancialStatementUsed,
    ///BP - Borrower Payment Received
    BorrowerPaymentReceived,
    ///BPD - Beneficiary is Partially Dependent
    BeneficiaryIsPartiallyDependent,
    ///BQ - Product Information Available
    ProductInformationAvailable,
    ///BR - Bedrest BRP (Bathroom Privileges)
    CodeBR,
    ///BS - Revenue derived from Commissions
    RevenueDerivedFromCommissions,
    ///BT - Borrower Telephone Attempt
    BorrowerTelephoneAttempt,
    ///BTD - Beneficiary is Totally Dependent
    BeneficiaryIsTotallyDependent,
    ///BU - Revenue derived from Donations
    RevenueDerivedFromDonations,
    ///BV - Revenue derived from Fees
    RevenueDerivedFromFees,
    ///BW - Revenue derived from Grants
    RevenueDerivedFromGrants,
    ///BX - Revenue derived from Taxes
    RevenueDerivedFromTaxes,
    ///BY - Sprinkler Equipped
    SprinklerEquipped,
    ///BZ - Statement requested from Government Registry
    StatementRequestedFromGovernmentRegistry,
    ///C0 - Collision Coverage Will Transfer
    CollisionCoverageWillTransfer,
    ///C1 - Advances From Property Management Expenses Outstanding
    AdvancesFromPropertyManagementExpensesOutstanding,
    ///C2 - Final Demand Letter Sent
    FinalDemandLetterSent,
    ///C3 - Lender Request for Assistance
    LenderRequestForAssistance,
    ///C4 - Mortgage has Lender-purchased Mortgage Insurance
    MortgageHasLenderPurchasedMortgageInsurance,
    ///C5 - Insufficient Funds
    InsufficientFunds,
    ///C6 - Credit Enhanced Mortgage
    CreditEnhancedMortgage,
    ///C7 - Corporate Appointment
    CorporateAppointment,
    ///C8 - Special Servicing Required
    SpecialServicingRequired,
    ///C9 - Client Specifically Requested Consideration of Special Financing or an Assumable Loan
    ClientSpecificallyRequestedConsiderationOfSpecialFinancingOrAnAssumableLoan,
    ///CA - Cane Required
    CaneRequired,
    ///CB - Complete Bedrest
    CompleteBedrest,
    ///CC - Collection Card was Left
    CollectionCardWasLeft,
    ///CD - Call to Directory Assistance for Reference Telephone
    CallToDirectoryAssistanceForReferenceTelephone,
    ///CE - Co-signer Telephone Attempt
    CoSignerTelephoneAttempt,
    ///CF - Co-signer Telephone Contact
    CoSignerTelephoneContact,
    ///CFD - Claim is Fraudulent
    ClaimIsFraudulent,
    ///CG - Co-signer Delinquency Letter Sent
    CoSignerDelinquencyLetterSent,
    ///CH - Co-signer Final Demand Letter Sent
    CoSignerFinalDemandLetterSent,
    ///CI - Call to Directory Assistance for Co-signer Telephone
    CallToDirectoryAssistanceForCoSignerTelephone,
    ///CJ - Valid Borrower Address or Phone Attempt with Previous Holder
    ValidBorrowerAddressOrPhoneAttemptWithPreviousHolder,
    ///CK - Convertible
    Convertible,
    ///CL - Claimant had a Pre-existing Injury
    ClaimantHadAPreExistingInjury,
    ///CM - Comatose
    Comatose,
    ///CN - Common Elements are Leased to or by the Home Owners' Association
    CommonElementsAreLeasedToOrByTheHomeOwnersAssociation,
    ///CNJ - Cumulative Injury
    CumulativeInjury,
    ///CO - Contracture
    Contracture,
    ///CP - Case Pending
    CasePending,
    ///CQ - Callable
    Callable,
    ///CR - Crutches Required
    CrutchesRequired,
    ///CS - Community Participates in National Flood Insurance Program
    CommunityParticipatesInNationalFloodInsuranceProgram,
    ///CT - Common Elements are Completed
    CommonElementsAreCompleted,
    ///CU - Curb and Gutter are Public
    CurbAndGutterArePublic,
    ///CV - Cooperative
    Cooperative,
    ///CW - Cooling Water is Low
    CoolingWaterIsLow,
    ///CX - Certification Status
    CertificationStatus,
    ///CY - Car Spaces are Adequate
    CarSpacesAreAdequate,
    ///CZ - Car Spaces are Inadequate
    CarSpacesAreInadequate,
    ///D0 - Comprehensive Coverage Will Transfer
    ComprehensiveCoverageWillTransfer,
    ///D1 - Issue Check Payable to Borrower and Return to Servicer
    IssueCheckPayableToBorrowerAndReturnToServicer,
    ///D2 - Issue Check Payable to Servicer and Return to Servicer
    IssueCheckPayableToServicerAndReturnToServicer,
    ///D3 - Issue Check Payable to Borrower and Send to Borrower
    IssueCheckPayableToBorrowerAndSendToBorrower,
    ///D4 - Issue Check Payable to Servicer or Borrower and Return to Servicer
    IssueCheckPayableToServicerOrBorrowerAndReturnToServicer,
    ///D5 - Issue Check Payable to Other Payee
    IssueCheckPayableToOtherPayee,
    ///D6 - Positive
    Positive,
    ///D7 - Negative
    Negative,
    ///D8 - Taxes are Typical for the Area and Price Range
    TaxesAreTypicalForTheAreaAndPriceRange,
    ///D9 - Improvement Conforms to Zoning Regulations
    ImprovementConformsToZoningRegulations,
    ///DA - Call to Directory Assistance for Borrower Telephone
    CallToDirectoryAssistanceForBorrowerTelephone,
    ///DB - Deferment or Forbearance Begin
    DefermentOrForbearanceBegin,
    ///DC - Declined
    Declined,
    ///DD - Borrower Furnished Demographic Data
    BorrowerFurnishedDemographicData,
    ///DE - Deferment or Forbearance End
    DefermentOrForbearanceEnd,
    ///DF - Funds available for Unsecured Creditors
    FundsAvailableForUnsecuredCreditors,
    ///DFR - Deductible Amount Fully Recovered
    DeductibleAmountFullyRecovered,
    ///DG - Dynamic Brakes are Out
    DynamicBrakesAreOut,
    ///DH - Debtor has been Domiciled
    DebtorHasBeenDomiciled,
    ///DI - Disoriented
    Disoriented,
    ///DJ - Dynamic Brakes are Operational
    DynamicBrakesAreOperational,
    ///DK - Construction Warranty
    ConstructionWarranty,
    ///DL - Construction Warranty Transferable
    ConstructionWarrantyTransferable,
    ///DM - Maintenance Drug under Client's Benefit Plan
    MaintenanceDrugUnderClientsBenefitPlan,
    ///DN - Payment Reduced Because Maximum Allowable Cost Exceeded
    PaymentReducedBecauseMaximumAllowableCostExceeded,
    ///DNR - Deductible Amount Not Fully Recovered
    DeductibleAmountNotFullyRecovered,
    ///DO - Benefits Terminated Prior to Service Date
    BenefitsTerminatedPriorToServiceDate,
    ///DP - Depressed
    Depressed,
    ///DQ - Drug Part of Formulary Data Base
    DrugPartOfFormularyDataBase,
    ///DR - Subject not Engaged in Business
    SubjectNotEngagedInBusiness,
    ///DS - All Door Seals are Intact
    AllDoorSealsAreIntact,
    ///DT - Filing Fee Attached
    FilingFeeAttached,
    ///DU - Subject not Engaged in Business at Requested Address
    SubjectNotEngagedInBusinessAtRequestedAddress,
    ///DV - Suspended
    Suspended,
    ///DW - Total
    Total,
    ///DX - Unable to Respond
    UnableToRespond,
    ///DY - Dyspnea with Minimal Exertion
    DyspneaWithMinimalExertion,
    ///DZ - Uses Own Facilities
    UsesOwnFacilities,
    ///E0 - Figures are Total
    FiguresAreTotal,
    ///E1 - Fixed Asset Breakdown Undisclosed
    FixedAssetBreakdownUndisclosed,
    ///E2 - For the Fiscal Year
    ForTheFiscalYear,
    ///E3 - For the Period
    ForThePeriod,
    ///E4 - Formed by Consolidation
    FormedByConsolidation,
    ///E5 - Formed by Merger
    FormedByMerger,
    ///E6 - Prior Bankruptcy Case Filed in Last 6 Years
    PriorBankruptcyCaseFiledInLast6Years,
    ///E7 - Debtor is not Represented by an Attorney
    DebtorIsNotRepresentedByAnAttorney,
    ///E8 - A Pending Case has been Filed
    APendingCaseHasBeenFiled,
    ///E9 - Guaranteed by Parent Company
    GuaranteedByParentCompany,
    ///EA - Has Authority for All Purchases
    HasAuthorityForAllPurchases,
    ///EB - Has Authority to Purchase Supplies
    HasAuthorityToPurchaseSupplies,
    ///EC - Equipment Certified
    EquipmentCertified,
    ///ED - Has Business Interruption Insurance
    HasBusinessInterruptionInsurance,
    ///EE - Has Class of Stock
    HasClassOfStock,
    ///EF - Has Extended Coverage Insurance
    HasExtendedCoverageInsurance,
    ///EG - Has Fire Insurance
    HasFireInsurance,
    ///EH - Has Joint Authority
    HasJointAuthority,
    ///EI - Has Life Insurance
    HasLifeInsurance,
    ///EJ - Existence of Preliminary Flood Determination
    ExistenceOfPreliminaryFloodDetermination,
    ///EK - Existence of Community Participation in the National Flood Insurance
    ExistenceOfCommunityParticipationInTheNationalFloodInsurance,
    ///EL - Endurance Limitations
    EnduranceLimitations,
    ///EM - Has Marriage Contract
    HasMarriageContract,
    ///EN - Electricity On
    ElectricityOn,
    ///EO - Equipment Is Overhauled
    EquipmentIsOverhauled,
    ///EP - Exercises Prescribed
    ExercisesPrescribed,
    ///EQ - Has No Par Value
    HasNoParValue,
    ///ER - Engine Start-Up Performed with No Problems Reported
    EngineStartUpPerformedWithNoProblemsReported,
    ///ES - Engine Start-Up Performed with Problems Reported
    EngineStartUpPerformedWithProblemsReported,
    ///ET - Electrical Control System Shut Down
    ElectricalControlSystemShutDown,
    ///EU - Has Other Insurance
    HasOtherInsurance,
    ///EV - Has Par Value
    HasParValue,
    ///EW - Has Sole Authority
    HasSoleAuthority,
    ///EX - Excellent
    Excellent,
    ///EY - Has Voting Rights
    HasVotingRights,
    ///EZ - Heading Address in Registered Office Only
    HeadingAddressInRegisteredOfficeOnly,
    ///F0 - High Level
    HighLevel,
    ///F1 - Homeworkers Employed
    HomeworkersEmployed,
    ///F2 - In Subscriber Shares
    InSubscriberShares,
    ///F3 - Inactive
    Inactive,
    ///F4 - Incomplete
    Incomplete,
    ///F5 - Incorporation Details Requested
    IncorporationDetailsRequested,
    ///F6 - Increase or Up
    IncreaseOrUp,
    ///F7 - Information Cannot Be Provided at This Time
    InformationCannotBeProvidedAtThisTime,
    ///F8 - Information in Date
    InformationInDate,
    ///F9 - Information Requires Investigation
    InformationRequiresInvestigation,
    ///FA - Actions has a Significant Environmental Effect
    ActionsHasASignificantEnvironmentalEffect,
    ///FB - Application Includes Complete System
    ApplicationIncludesCompleteSystem,
    ///FC - Antenna is Mounted on a Structure with an Existing Antenna
    AntennaIsMountedOnAStructureWithAnExistingAntenna,
    ///FD - Notice of Construction or Alteration has been Filed
    NoticeOfConstructionOrAlterationHasBeenFiled,
    ///FE - Applicant Wants to Monitor Frequency
    ApplicantWantsToMonitorFrequency,
    ///FF - Applicant has been Denied Government Benefits Due to Use of Drugs
    ApplicantHasBeenDeniedGovernmentBenefitsDueToUseOfDrugs,
    ///FG - Application is Certified
    ApplicationIsCertified,
    ///FH - Application is for other Than a New Station
    ApplicationIsForOtherThanANewStation,
    ///FI - Fee Required
    FeeRequired,
    ///FJ - Flood Status
    FloodStatus,
    ///FK - Flood Insurance Required
    FloodInsuranceRequired,
    ///FL - Federal Flood Insurance is Available (Community Participates)
    CodeFL,
    ///FM - Inventory Valued Using LIFO (Last In/First Out)
    CodeFM,
    ///FN - Not Too High Level
    NotTooHighLevel,
    ///FO - Forgetful
    Forgetful,
    ///FP - Flood Certification with Life of Loan
    FloodCertificationWithLifeOfLoan,
    ///FQ - Street Maintenance is Public
    StreetMaintenanceIsPublic,
    ///FR - Fair
    Fair,
    ///FS - Not Yet Registered
    NotYetRegistered,
    ///FT - Obliged to File Balance Sheet
    ObligedToFileBalanceSheet,
    ///FU - Official Confirmation Received
    OfficialConfirmationReceived,
    ///FV - Old But Well Kept
    OldButWellKept,
    ///FW - Old Established Business
    OldEstablishedBusiness,
    ///FX - Operated at Break Even
    OperatedAtBreakEven,
    ///FY - Operates as Agent
    OperatesAsAgent,
    ///FZ - Flood Zone Status
    FloodZoneStatus,
    ///G0 - Out of Business
    OutOfBusiness,
    ///G1 - Outstanding Claims
    OutstandingClaims,
    ///G2 - Gas On
    GasOn,
    ///G3 - Hazardous Materials are Used or Produced
    HazardousMaterialsAreUsedOrProduced,
    ///G4 - Genetically Engineered Organisms are Used or Produced
    GeneticallyEngineeredOrganismsAreUsedOrProduced,
    ///G5 - This is a Group Proposal
    ThisIsAGroupProposal,
    ///G6 - Historical Sites Are Affected
    HistoricalSitesAreAffected,
    ///G7 - Facilities are Properly Accredited or Authorized
    FacilitiesAreProperlyAccreditedOrAuthorized,
    ///G8 - Proprietary or Privileged Information will be contained in the Application
    ProprietaryOrPrivilegedInformationWillBeContainedInTheApplication,
    ///G9 - This Project has an Actual or Potential Impact on the Environment
    ThisProjectHasAnActualOrPotentialImpactOnTheEnvironment,
    ///GA - Growth Rate is Fully Developed
    GrowthRateIsFullyDeveloped,
    ///GB - Outstanding Social Security Claims
    OutstandingSocialSecurityClaims,
    ///GC - Outstanding Value Added Tax (VAT) Claims
    CodeGC,
    ///GD - Product Demonstration in Effect
    ProductDemonstrationInEffect,
    ///GE - Ownership Acknowledged in Signed Statement
    OwnershipAcknowledgedInSignedStatement,
    ///GF - Ownership Acknowledged Verbally
    OwnershipAcknowledgedVerbally,
    ///GG - Ownership Not Acknowledged
    OwnershipNotAcknowledged,
    ///GH - Owns No Real Estate
    OwnsNoRealEstate,
    ///GI - Owns Real Estate but Details Not Available
    OwnsRealEstateButDetailsNotAvailable,
    ///GJ - Prepared from Books Without Audit
    PreparedFromBooksWithoutAudit,
    ///GK - Prepared from Statement by Accountant
    PreparedFromStatementByAccountant,
    ///GL - Profits Paid to Group
    ProfitsPaidToGroup,
    ///GM - Shelf Set to Manufacturer's Standard
    ShelfSetToManufacturersStandard,
    ///GN - Publicly Traded
    PubliclyTraded,
    ///GO - Good
    Good,
    ///GP - Purchase Authority is Qualified
    PurchaseAuthorityIsQualified,
    ///GQ - Purchases on Floor Plan
    PurchasesOnFloorPlan,
    ///GR - Shelf Set to Retailer's Schematic
    ShelfSetToRetailersSchematic,
    ///GS - Purchases on Letter of Credit
    PurchasesOnLetterOfCredit,
    ///GT - Real Estate Check is Necessary
    RealEstateCheckIsNecessary,
    ///GU - Record of Preferential Claims
    RecordOfPreferentialClaims,
    ///GV - Registered Address is Same as Business Address
    RegisteredAddressIsSameAsBusinessAddress,
    ///GW - Relatives Help in Business
    RelativesHelpInBusiness,
    ///GX - Satisfactory
    Satisfactory,
    ///GY - Seasons are Steady
    SeasonsAreSteady,
    ///GZ - Secured
    Secured,
    ///H0 - Organization Certifies Compliance with Federal Lobbying Regulations
    OrganizationCertifiesComplianceWithFederalLobbyingRegulations,
    ///H1 - Project involves International Co-operative Activities
    ProjectInvolvesInternationalCoOperativeActivities,
    ///H2 - Human Anatomical Substances Are Used
    HumanAnatomicalSubstancesAreUsed,
    ///H3 - Handicap Facilities Are Available
    HandicapFacilitiesAreAvailable,
    ///H4 - Lobbying Activities Have Been Conducted Regarding the Proposal
    LobbyingActivitiesHaveBeenConductedRegardingTheProposal,
    ///H5 - Organization Certifies Compliance With the Drug-Free Workplace Act
    OrganizationCertifiesComplianceWithTheDrugFreeWorkplaceAct,
    ///H6 - Organization Certifies Compliance with the Code of Federal Regulations Regarding Research Misconduct
    OrganizationCertifiesComplianceWithTheCodeOfFederalRegulationsRegardingResearchMisconduct,
    ///H7 - Organization Provides a Smoke Free Workplace
    OrganizationProvidesASmokeFreeWorkplace,
    ///H8 - Organization Certifies Compliance with Federal Discrimination Regulations
    OrganizationCertifiesComplianceWithFederalDiscriminationRegulations,
    ///H9 - Organization Certifies Compliance with the Code of Federal Regulations Regarding Responsibility of Applicants for Promoting Objectivity in Research for which Public Health Service (PHS) Funding is Sought
    CodeH9,
    ///HA - Well Maintained
    WellMaintained,
    ///HB - Interest Rate Buydown
    InterestRateBuydown,
    ///HC - Heating and Cooling for the Individual Units Separately Metered
    HeatingAndCoolingForTheIndividualUnitsSeparatelyMetered,
    ///HD - High Discharge
    HighDischarge,
    ///HE - High Engine Water Pressure
    HighEngineWaterPressure,
    ///HF - Interest Only
    InterestOnly,
    ///HG - Graduated Payment
    GraduatedPayment,
    ///HH - Principal Balance Exceeds Maximum Negative Amortization
    PrincipalBalanceExceedsMaximumNegativeAmortization,
    ///HI - Last Change
    LastChange,
    ///HJ - Liability Released
    LiabilityReleased,
    ///HK - Liability Not Released
    LiabilityNotReleased,
    ///HL - Hearing Limitations
    HearingLimitations,
    ///HM - Liability Determined by Note Holder
    LiabilityDeterminedByNoteHolder,
    ///HN - After Conversion
    AfterConversion,
    ///HO - Hostile
    Hostile,
    ///HP - After Modification
    AfterModification,
    ///HQ - Balloon
    Balloon,
    ///HR - Capitalized Mortgage
    CapitalizedMortgage,
    ///HS - Federal Wages in Effect
    FederalWagesInEffect,
    ///HT - Social Security Number (SSN) Never Issued
    CodeHT,
    ///HU - Name Does Not Match Social Security Number (SSN)
    CodeHU,
    ///HV - Birthdate Does Not Match Social Security Number (SSN)
    CodeHV,
    ///HW - Impossible Social Security Number (SSN)
    CodeHW,
    ///HX - Employee is Ineligible to Work
    EmployeeIsIneligibleToWork,
    ///HY - Metes and Bounds
    MetesAndBounds,
    ///HZ - Consolidation, Extension, Modification of Mortgage Loan (CEM)
    CodeHZ,
    ///I0 - Based on Operating Data
    BasedOnOperatingData,
    ///I1 - Uses Outside Services
    UsesOutsideServices,
    ///I2 - Very High Level
    VeryHighLevel,
    ///I3 - Very Small
    VerySmall,
    ///I4 - Voluntary Bankruptcy
    VoluntaryBankruptcy,
    ///I5 - Well Balanced
    WellBalanced,
    ///I6 - Well Regarded in Business Circles
    WellRegardedInBusinessCircles,
    ///I7 - Organization has Delinquent Federal Debts
    OrganizationHasDelinquentFederalDebts,
    ///I8 - Organization has been Placed on the Federal Debarment and Suspension List
    OrganizationHasBeenPlacedOnTheFederalDebarmentAndSuspensionList,
    ///I9 - No-show Indicator
    NoShowIndicator,
    ///IA - Interest Paid in Advance
    InterestPaidInAdvance,
    ///IB - Interest Paid in Arrears
    InterestPaidInArrears,
    ///IC - Interest Carryover
    InterestCarryover,
    ///ID - Sells Directly
    SellsDirectly,
    ///IE - Sells with Agents
    SellsWithAgents,
    ///IF - Sells with Storage
    SellsWithStorage,
    ///IG - Small
    Small,
    ///IH - Independent at Home
    IndependentAtHome,
    ///II - Some Increase
    SomeIncrease,
    ///IJ - Somewhat Declining Tendency
    SomewhatDecliningTendency,
    ///IK - Started Some Time Ago
    StartedSomeTimeAgo,
    ///IL - Industry Location
    IndustryLocation,
    ///IM - Sufficient
    Sufficient,
    ///IN - Indifferent
    Indifferent,
    ///IO - Termination Date Set
    TerminationDateSet,
    ///IP - Injury occurred on Employer's Premises
    InjuryOccurredOnEmployersPremises,
    ///IQ - Terms Include Lump Sum Payments
    TermsIncludeLumpSumPayments,
    ///IR - Terms Include Progress Payments
    TermsIncludeProgressPayments,
    ///IS - Terms on Cost Plus Basis
    TermsOnCostPlusBasis,
    ///IT - Terms on Fixed Fee Basis
    TermsOnFixedFeeBasis,
    ///IU - Trade Style Registered
    TradeStyleRegistered,
    ///IV - Trading Address of Sole Proprietor
    TradingAddressOfSoleProprietor,
    ///IW - Unchanged Situation
    UnchangedSituation,
    ///IX - Undetermined
    Undetermined,
    ///IY - Unsatisfactory
    Unsatisfactory,
    ///IZ - Unsecured
    Unsecured,
    ///J0 - Qualifies as an Energy Efficient Home
    QualifiesAsAnEnergyEfficientHome,
    ///J1 - Military Services Barred from Recruitment Activities at the Proposing Organization's Site(s)
    CodeJ1,
    ///J2 - Rate Negotiated
    RateNegotiated,
    ///J3 - Under Penalty of Perjury the Information is True and Correct
    UnderPenaltyOfPerjuryTheInformationIsTrueAndCorrect,
    ///J4 - Project Requires Inter-Government Review for Activities that affect State or Local Government or Possible National Security Implications
    ProjectRequiresInterGovernmentReviewForActivitiesThatAffectStateOrLocalGovernmentOrPossibleNationalSecurityImplications,
    ///J5 - Filing on Behalf of Debtor is Authorized
    FilingOnBehalfOfDebtorIsAuthorized,
    ///J6 - Debtor Understands the Relief available under each Bankruptcy Chapter
    DebtorUnderstandsTheReliefAvailableUnderEachBankruptcyChapter,
    ///J7 - Attorney Declares that Debtor has been Informed
    AttorneyDeclaresThatDebtorHasBeenInformed,
    ///J8 - Attorney has Explained the Relief available under each Bankruptcy Chapter
    AttorneyHasExplainedTheReliefAvailableUnderEachBankruptcyChapter,
    ///J9 - There has been a Transfer of a Claim Against the Debtor by or to any Petitioner
    ThereHasBeenATransferOfAClaimAgainstTheDebtorByOrToAnyPetitioner,
    ///JA - Third Party Originated
    ThirdPartyOriginated,
    ///JB - Existing Construction
    ExistingConstruction,
    ///JC - Other Lien
    OtherLien,
    ///JCA - Joint Coverage Applies
    JointCoverageApplies,
    ///JD - Subject Lien
    SubjectLien,
    ///JE - No Evidence of Property Damage Observed such as Dampness, Termites, or Structure Settlement
    CodeJE,
    ///JF - Primary Underwriting System
    PrimaryUnderwritingSystem,
    ///JG - Non New Parts Used
    NonNewPartsUsed,
    ///JH - Pledged Loan
    PledgedLoan,
    ///JI - Security Delivery
    SecurityDelivery,
    ///JJ - Secondary Underwriting System
    SecondaryUnderwritingSystem,
    ///JK - Distribution is Stopped
    DistributionIsStopped,
    ///JL - Sentence was Suspended
    SentenceWasSuspended,
    ///JM - Very Negative Information Exists
    VeryNegativeInformationExists,
    ///JN - Payment Notes Exist
    PaymentNotesExist,
    ///JO - Immigrated
    Immigrated,
    ///JP - Audited with Qualifications
    AuditedWithQualifications,
    ///JQ - Audited
    Audited,
    ///JR - Temporarily Closed
    TemporarilyClosed,
    ///JS - Partial
    Partial,
    ///JT - Telephone Number is Unpublished
    TelephoneNumberIsUnpublished,
    ///JU - Telephone Number is Not in Service
    TelephoneNumberIsNotInService,
    ///JV - Negative Information Exists for the Group
    NegativeInformationExistsForTheGroup,
    ///JW - The More Important Items are Only Included
    TheMoreImportantItemsAreOnlyIncluded,
    ///JX - Interest Owned by Affiliated Company
    InterestOwnedByAffiliatedCompany,
    ///JY - Interest Owned by Subject of Inquiry
    InterestOwnedBySubjectOfInquiry,
    ///JZ - Qualifies as a Government Approved Condominium or Project
    QualifiesAsAGovernmentApprovedCondominiumOrProject,
    ///K0 - Account Receivables Breakdown Undisclosed
    AccountReceivablesBreakdownUndisclosed,
    ///K1 - Additional Record Items Available
    AdditionalRecordItemsAvailable,
    ///K2 - Address is Qualified
    AddressIsQualified,
    ///K3 - All Paid In or Issued
    AllPaidInOrIssued,
    ///K4 - Appears High
    AppearsHigh,
    ///K5 - Appears Not to Guarantee Sufficient Coverage
    AppearsNotToGuaranteeSufficientCoverage,
    ///K6 - Appears Sufficiently High
    AppearsSufficientlyHigh,
    ///K7 - Appears to Indicate a Strained Situation
    AppearsToIndicateAStrainedSituation,
    ///K8 - Banks with Main National Banks
    BanksWithMainNationalBanks,
    ///K9 - Bills Paid from Branch Office
    BillsPaidFromBranchOffice,
    ///KA - Bills Paid from Division Office
    BillsPaidFromDivisionOffice,
    ///KB - Bills Paid from Headquarters Office
    BillsPaidFromHeadquartersOffice,
    ///KC - Bond Information Available
    BondInformationAvailable,
    ///KD - Changed Accounting Date
    ChangedAccountingDate,
    ///KE - Clear
    Clear,
    ///KF - Clear Declining Tendency
    ClearDecliningTendency,
    ///KG - Clear Increase
    ClearIncrease,
    ///KH - Cluttered
    Cluttered,
    ///KI - Company has No Other Locations
    CompanyHasNoOtherLocations,
    ///KJ - Company is Branch of Foreign Entity
    CompanyIsBranchOfForeignEntity,
    ///KK - Company is Perpetual
    CompanyIsPerpetual,
    ///KL - Company is Tax Exempt
    CompanyIsTaxExempt,
    ///KM - Compared to Same Period Last Year
    ComparedToSamePeriodLastYear,
    ///KN - Conducted at a Loss
    ConductedAtALoss,
    ///KO - Inventory Valued using FIFO (First In/First Out)
    CodeKO,
    ///KP - Large
    Large,
    ///KQ - Letter of Agreement Present
    LetterOfAgreementPresent,
    ///KR - Letter of Agreement Withdrawn
    LetterOfAgreementWithdrawn,
    ///KS - Letter of Liability Present
    LetterOfLiabilityPresent,
    ///KT - Letter of Liability Withdrawn
    LetterOfLiabilityWithdrawn,
    ///KU - Location Inquired Upon is a Branch
    LocationInquiredUponIsABranch,
    ///KV - Location Inquired Upon is a Branch; Headquarters is Provided
    CodeKV,
    ///KW - Location inquired upon is a Headquarters
    LocationInquiredUponIsAHeadquarters,
    ///KX - Location is Foreign
    LocationIsForeign,
    ///KY - Means Exhausted
    MeansExhausted,
    ///KZ - Medium to Large
    MediumToLarge,
    ///L0 - Immunization Mandated by State Law for Employment
    ImmunizationMandatedByStateLawForEmployment,
    ///L1 - General Standard of 20 Degree or .5 Diopter Sphere or Cylinder Change Met
    GeneralStandardOf20DegreeOr5DiopterSphereOrCylinderChangeMet,
    ///L2 - Replacement Due to Loss or Theft
    ReplacementDueToLossOrTheft,
    ///L3 - Replacement Due to Breakage or Damage
    ReplacementDueToBreakageOrDamage,
    ///L4 - Replacement Due to Patient Preference
    ReplacementDueToPatientPreference,
    ///L5 - Replacement Due to Medical Reason
    ReplacementDueToMedicalReason,
    ///L6 - Land Contract
    LandContract,
    ///L7 - Account Current
    AccountCurrent,
    ///L8 - Very Good
    VeryGood,
    ///L9 - Restored
    Restored,
    ///LA - Letter of Map Amendment or Letter of Map Revision
    LetterOfMapAmendmentOrLetterOfMapRevision,
    ///LB - Legally Blind
    LegallyBlind,
    ///LC - Producer of Goods
    ProducerOfGoods,
    ///LD - Drawback Indicator
    DrawbackIndicator,
    ///LE - Lethargic
    Lethargic,
    ///LF - Customs Rule Applicable
    CustomsRuleApplicable,
    ///LG - Exported Pursuant to Law Regulation or to Cancel Customs Bond
    ExportedPursuantToLawRegulationOrToCancelCustomsBond,
    ///LH - Country of Origin Information Applies to All Prior Shipments
    CountryOfOriginInformationAppliesToAllPriorShipments,
    ///LI - Price Estimated
    PriceEstimated,
    ///LJ - North American Free Trade Agreement (NAFTA) Preference
    CodeLJ,
    ///LK - Kit Form
    KitForm,
    ///LL - Lockout Effective
    LockoutEffective,
    ///LM - Letter of Appointment
    LetterOfAppointment,
    ///LN - Facility's Emergency Response Plan Includes Specific Actions to be Taken in Response to Accidental Releases of Regulated Substances
    FacilitysEmergencyResponsePlanIncludesSpecificActionsToBeTakenInResponseToAccidentalReleasesOfRegulatedSubstances,
    ///LO - Locomotive is Isolated
    LocomotiveIsIsolated,
    ///LP - Low Engine Oil Pressure
    LowEngineOilPressure,
    ///LQ - Facility had a Safety Inspection
    FacilityHadASafetyInspection,
    ///LR - Locomotive Engine is Running
    LocomotiveEngineIsRunning,
    ///LS - Lessee Signature on File
    LesseeSignatureOnFile,
    ///LSD - List Specialty in Directory
    ListSpecialtyInDirectory,
    ///LT - Lender or Servicer Transfer
    LenderOrServicerTransfer,
    ///LU - Evidence of Dampness
    EvidenceOfDampness,
    ///LV - Evidence of Termites
    EvidenceOfTermites,
    ///LW - Evidence of Structure Settlement
    EvidenceOfStructureSettlement,
    ///LX - Salvage Moved
    SalvageMoved,
    ///LY - Address is Former Location
    AddressIsFormerLocation,
    ///LZ - Address is Occupied by Others
    AddressIsOccupiedByOthers,
    /**M0 - Facility has an Occupational Safety and Health Act
(OSHA) Star or Merit Ranking*/
    CodeM0,
    ///M1 - Data Corrected
    DataCorrected,
    ///M2 - Servicer Record Selected
    ServicerRecordSelected,
    ///M3 - Length of Service is 3 Months or Less
    LengthOfServiceIs3MonthsOrLess,
    ///M4 - Length of Service is 3 Months or more, and Less than 1 Year
    CodeM4,
    ///M5 - Length of Service is 1 Year through 5 Years
    LengthOfServiceIs1YearThrough5Years,
    ///M6 - Length of Service is more than 5 Years
    LengthOfServiceIsMoreThan5Years,
    ///M7 - Cataract or Corneal Transplant or Other Condition such as Keratoconus
    CataractOrCornealTransplantOrOtherConditionSuchAsKeratoconus,
    ///M8 - Vision in Worse Eye Correctable to 20/40 or Better with Regular Lenses
    VisionInWorseEyeCorrectableTo2040OrBetterWithRegularLenses,
    ///M9 - Contact Lenses Corrected Vision in Worse Eye to 20/40 or Better
    ContactLensesCorrectedVisionInWorseEyeTo2040OrBetter,
    ///MA - Major Alarm Flag Reported
    MajorAlarmFlagReported,
    ///MB - Equipment has Modified Configuration
    EquipmentHasModifiedConfiguration,
    ///MC - Other Mental Condition
    OtherMentalCondition,
    ///MD - Marketing Time is 4 to 6 Months
    MarketingTimeIs4To6Months,
    ///ME - Trend Reversed
    TrendReversed,
    ///MF - Microprocessor Fault
    MicroprocessorFault,
    ///MG - Mortgage Insurance Application Included
    MortgageInsuranceApplicationIncluded,
    ///MH - Mortgage Credit Report Included
    MortgageCreditReportIncluded,
    ///MI - Residential Loan Application Included
    ResidentialLoanApplicationIncluded,
    ///MJ - Real Estate Information Report Included
    RealEstateInformationReportIncluded,
    ///MK - Real Estate Title Evidence Included
    RealEstateTitleEvidenceIncluded,
    ///ML - Manually Search and List
    ManuallySearchAndList,
    ///MM - Property is Occupied by Tenant (Market Rent)
    CodeMM,
    ///MN - Property is Occupied by Tenant (Regulated Rent)
    CodeMN,
    ///MO - Cooperative Project Includes or Owns Any Commercial Units
    CooperativeProjectIncludesOrOwnsAnyCommercialUnits,
    ///MP - Units and Project Amenities are Complete
    UnitsAndProjectAmenitiesAreComplete,
    ///MQ - Eligible Trust
    EligibleTrust,
    ///MR - Resale Property
    ResaleProperty,
    ///MS - Miscellaneous Skip-Trace Attempt
    MiscellaneousSkipTraceAttempt,
    ///MT - Photos Match Description
    PhotosMatchDescription,
    ///MU - Photos Show Negative Influence
    PhotosShowNegativeInfluence,
    ///MV - Exclude from Monthly Debt
    ExcludeFromMonthlyDebt,
    ///MW - This Broker Market Analysis is being Completed for Home Market Assistance
    ThisBrokerMarketAnalysisIsBeingCompletedForHomeMarketAssistance,
    ///MX - This Broker Market Analysis is being Completed for Homesale or Buyout
    ThisBrokerMarketAnalysisIsBeingCompletedForHomesaleOrBuyout,
    ///MY - Project Type is Single Family
    ProjectTypeIsSingleFamily,
    ///MZ - Project Type is Other
    ProjectTypeIsOther,
    ///N0 - Hospitalized over-night
    HospitalizedOverNight,
    ///N1 - Claim Involves (a) Day(s) Away From Work
    CodeN1,
    ///N2 - Claim involves Restricted Work Activity Without Days Away from Work
    ClaimInvolvesRestrictedWorkActivityWithoutDaysAwayFromWork,
    ///N3 - Strike or Lockout in Progress
    StrikeOrLockoutInProgress,
    ///N4 - Shutdown or Layoff in Progress
    ShutdownOrLayoffInProgress,
    ///N5 - Work is Seasonal
    WorkIsSeasonal,
    ///N6 - Natural Disaster or Adverse Weather Affecting Work
    NaturalDisasterOrAdverseWeatherAffectingWork,
    ///N7 - Shorter Work Schedules or Fewer Pay Periods than Usual in Effect
    ShorterWorkSchedulesOrFewerPayPeriodsThanUsualInEffect,
    ///N8 - Longer Work Schedules or More Pay Periods than Usual in Effect
    LongerWorkSchedulesOrMorePayPeriodsThanUsualInEffect,
    ///N9 - Other Factors Affect Claim Frequency
    OtherFactorsAffectClaimFrequency,
    ///NA - No User Available
    NoUserAvailable,
    ///NB - Neighborhood Predominately Single Family Dwellings
    NeighborhoodPredominatelySingleFamilyDwellings,
    ///NC - Item has Direct Numerical Control
    ItemHasDirectNumericalControl,
    ///ND - Note Holder Permission Required
    NoteHolderPermissionRequired,
    ///NDP - No Deductible Program
    NoDeductibleProgram,
    ///NE - Notarized
    Notarized,
    ///NF - New Construction
    NewConstruction,
    ///NG - Mortgage Points are Customarily Paid by Seller
    MortgagePointsAreCustomarilyPaidBySeller,
    ///NH - No National Flood Insurance Program map
    NoNationalFloodInsuranceProgramMap,
    ///NI - Seasoned Mortgage
    SeasonedMortgage,
    ///NJ - Issues are Anticipated that would Affect the Ability to Secure Financing of the Subject Property
    IssuesAreAnticipatedThatWouldAffectTheAbilityToSecureFinancingOfTheSubjectProperty,
    ///NK - Citizenship
    Citizenship,
    ///NL - Group Disability Insurance Mandatory
    GroupDisabilityInsuranceMandatory,
    ///NM - Retail Origination
    RetailOrigination,
    ///NN - Answer to Referenced Question is "None"
    CodeNN,
    ///NO - Arm's Length Transaction
    ArmsLengthTransaction,
    ///NP - Certification of a Non-attorney Bankruptcy Petition Preparer
    CertificationOfANonAttorneyBankruptcyPetitionPreparer,
    ///NQ - Eligible for the Fannie Mae Neighbors Program
    EligibleForTheFannieMaeNeighborsProgram,
    ///NR - No Restrictions
    NoRestrictions,
    ///NS - 401K Plan in Effect
    CodeNS,
    ///NT - Lodging Provided
    LodgingProvided,
    ///NU - Not Used
    NotUsed,
    ///NV - Contract Labor
    ContractLabor,
    ///NW - Bonuses Paid
    BonusesPaid,
    ///NX - Minors Employed
    MinorsEmployed,
    ///NY - Meets Requirements for Fannie Mae Community Seconds Program
    MeetsRequirementsForFannieMaeCommunitySecondsProgram,
    ///NZ - Purchase is a Result of Current Employer Sponsored Relocation
    PurchaseIsAResultOfCurrentEmployerSponsoredRelocation,
    ///O0 - Teaching Major
    TeachingMajor,
    ///O1 - Multiple Unspecified Instances
    MultipleUnspecifiedInstances,
    ///O2 - Hires Part Time Employees as Needed
    HiresPartTimeEmployeesAsNeeded,
    ///O3 - Mexican Request
    MexicanRequest,
    ///O4 - Risk Management Plan Requires Predictive Filing
    RiskManagementPlanRequiresPredictiveFiling,
    ///O5 - Sanitized Copy
    SanitizedCopy,
    ///O6 - Site Treated, Disposed, Recycled Waste On-Site or Discharged Waste to Sewer or Publicly Owned Treatment Works
    CodeO6,
    ///O7 - Toxic Chemical Claimed as Trade Secret
    ToxicChemicalClaimedAsTradeSecret,
    ///O8 - Under Control of Reporting Facility or Parent Company
    UnderControlOfReportingFacilityOrParentCompany,
    ///O9 - Weather Conditions Not Known
    WeatherConditionsNotKnown,
    ///OA - Seller Provided Below Market Secondary Financing
    SellerProvidedBelowMarketSecondaryFinancing,
    ///OB - Fixed Site
    FixedSite,
    ///OC - Mobile Facility
    MobileFacility,
    ///OD - Transfer Authorized
    TransferAuthorized,
    ///ODZ - Occupational Disease
    OccupationalDisease,
    ///OE - Transfer Complete
    TransferComplete,
    ///OF - Commercial Driver's License Verified
    CommercialDriversLicenseVerified,
    ///OG - Responsibility Accepted
    ResponsibilityAccepted,
    ///OH - Waterbody Involved
    WaterbodyInvolved,
    ///OI - Charges Pending
    ChargesPending,
    ///OJ - Driver has Proper License Class
    DriverHasProperLicenseClass,
    ///OK - Driver Compliant with License Restrictions
    DriverCompliantWithLicenseRestrictions,
    ///OL - Other Limitation
    OtherLimitation,
    ///OM - Driver has Commercial Driver's License
    DriverHasCommercialDriversLicense,
    ///ON - Driver has Medical Waiver
    DriverHasMedicalWaiver,
    ///OO - Own other Federal Housing Administration Property
    OwnOtherFederalHousingAdministrationProperty,
    ///OP - Out of Range Product Temperature
    OutOfRangeProductTemperature,
    ///OQ - Photographs Taken
    PhotographsTaken,
    ///OR - Other Restrictions
    OtherRestrictions,
    ///OS - Out of Service
    OutOfService,
    ///OT - Oriented
    Oriented,
    ///OU - Police Officer at Scene
    PoliceOfficerAtScene,
    ///OV - Overridden
    Overridden,
    ///OW - Proposed
    Proposed,
    ///OX - Rating is Affected
    RatingIsAffected,
    ///OY - Veteran as Defined by the Federal Housing Administration (FHA), Veterans Administration (VA), or Department of Housing and Urban Development (HUD)
    CodeOY,
    ///OZ - Liability is Contingent or has a Co-signer
    LiabilityIsContingentOrHasACoSigner,
    ///P0 - Terminal Degree
    TerminalDegree,
    ///P1 - Patient was Discharged from the First Facility
    PatientWasDischargedFromTheFirstFacility,
    ///P2 - Patient was Admitted to the Second Facility
    PatientWasAdmittedToTheSecondFacility,
    ///P3 - Property has a Family Room or Den
    PropertyHasAFamilyRoomOrDen,
    ///P4 - Property has Central Air Conditioning
    PropertyHasCentralAirConditioning,
    ///P5 - Property Typical of Neighborhood
    PropertyTypicalOfNeighborhood,
    ///P6 - Property Deferred Maintenance Typical of Neighborhood
    PropertyDeferredMaintenanceTypicalOfNeighborhood,
    ///P7 - Accepting Existing Patients
    AcceptingExistingPatients,
    ///P8 - Accepting New Patients
    AcceptingNewPatients,
    ///P9 - Property Intended to be Occupied as Primary Residence
    PropertyIntendedToBeOccupiedAsPrimaryResidence,
    ///PA - Paralysis
    Paralysis,
    ///PB - Phone Skip Begin
    PhoneSkipBegin,
    ///PC - Plan is Attached
    PlanIsAttached,
    ///PD - Phone Skip Resolved
    PhoneSkipResolved,
    ///PE - Phone Skip Exhaust
    PhoneSkipExhaust,
    ///PF - Paid Outside of Closing
    PaidOutsideOfClosing,
    ///PFB - Previously Failed Board Certification
    PreviouslyFailedBoardCertification,
    ///PG - Project is Subject to Ground Rent
    ProjectIsSubjectToGroundRent,
    ///PH - Prepayable
    Prepayable,
    ///PI - Program
    Program,
    ///PJ - Provider is Participating
    ProviderIsParticipating,
    ///PK - Preliminary Flood Determination
    PreliminaryFloodDetermination,
    ///PL - Provider Certification in the Taxonomy Has Been Verified
    ProviderCertificationInTheTaxonomyHasBeenVerified,
    ///PM - Project and Services Budget is Maintained
    ProjectAndServicesBudgetIsMaintained,
    ///PN - Atypical Physical Condition
    AtypicalPhysicalCondition,
    ///PO - Personal Property Onsite
    PersonalPropertyOnsite,
    ///PP - Property Previously Winterized
    PropertyPreviouslyWinterized,
    ///PQ - Liability will be Resubordinated to the Loan upon Closing
    LiabilityWillBeResubordinatedToTheLoanUponClosing,
    ///PR - Poor
    Poor,
    ///PRD - Prior Damage
    PriorDamage,
    ///PS - Publication is Included in Sharing
    PublicationIsIncludedInSharing,
    ///PT - Project is Complete
    ProjectIsComplete,
    ///PU - Not Paid
    NotPaid,
    ///PV - Property Vacant 0-5 Percent
    PropertyVacant05Percent,
    ///PW - Partial Weight Bearing
    PartialWeightBearing,
    ///PX - Paid by Borrower Before Closing
    PaidByBorrowerBeforeClosing,
    ///PY - Property for Sale
    PropertyForSale,
    ///PZ - Property Vacant Over 5 Percent
    PropertyVacantOver5Percent,
    ///Q0 - Veteran
    Veteran,
    ///Q1 - Export Product
    ExportProduct,
    ///Q2 - Distilled Spirit, Beer or Wine
    CodeQ2,
    ///Q3 - U.S. Goods Returned
    USGoodsReturned,
    ///Q4 - Candidate for U.S. Customs Service Protest
    CandidateForUSCustomsServiceProtest,
    ///Q5 - Domestic Product
    DomesticProduct,
    ///Q6 - Prior Approval Letter and Official Orders on File
    PriorApprovalLetterAndOfficialOrdersOnFile,
    ///Q7 - Importer's Substantiating Statement and Contract are on File
    ImportersSubstantiatingStatementAndContractAreOnFile,
    ///Q8 - International Transport Movement
    InternationalTransportMovement,
    ///Q9 - Piece Count should be Included in the Total Packing List Quantity
    PieceCountShouldBeIncludedInTheTotalPackingListQuantity,
    ///QA - Shipment should be Held at the Port
    ShipmentShouldBeHeldAtThePort,
    ///QB - Multiple States of Origin for this Item
    MultipleStatesOfOriginForThisItem,
    ///QC - Multiple Countries of Origin for this Item
    MultipleCountriesOfOriginForThisItem,
    ///QD - Letter of Credit Restricted to a Specific Bank
    LetterOfCreditRestrictedToASpecificBank,
    ///QE - Letter of Credit Permits Transshipment
    LetterOfCreditPermitsTransshipment,
    ///QF - Letter of Credit Covers Partial Shipments
    LetterOfCreditCoversPartialShipments,
    ///QG - Dutiable Item
    DutiableItem,
    ///QH - Amounts should be Pro-rated across Line Items
    AmountsShouldBeProRatedAcrossLineItems,
    ///QI - Toxic Substance Control Act (TSCA) Certification Required
    CodeQI,
    ///QJ - Visa Required for this Item
    VisaRequiredForThisItem,
    ///QK - Item Subject to Quotas
    ItemSubjectToQuotas,
    ///QL - Item is a Set as Defined by the General Rules of Interpretation Section 3 (GRI3)
    CodeQL,
    ///QM - Item is a Set
    ItemIsASet,
    ///QN - Item is an Ensemble
    ItemIsAnEnsemble,
    ///QO - Item is a Metal Item
    ItemIsAMetalItem,
    ///QP - Item is a Machine Part
    ItemIsAMachinePart,
    ///QQ - Item is a Hazardous Item
    ItemIsAHazardousItem,
    ///QR - Item is Eligible under the Generalized System of Preferences (GSP)
    CodeQR,
    ///QS - Quantity to be Imported has been Approved by the Necessary Agencies
    QuantityToBeImportedHasBeenApprovedByTheNecessaryAgencies,
    ///QT - Filing Data is to be Withheld from Public Inspection
    FilingDataIsToBeWithheldFromPublicInspection,
    ///QU - Property Type Cooperative
    PropertyTypeCooperative,
    ///QV - Paid by Borrower at Closing
    PaidByBorrowerAtClosing,
    ///QW - Paid by Other At or Before Closing
    PaidByOtherAtOrBeforeClosing,
    ///QX - Treated as a Reduction to Income
    TreatedAsAReductionToIncome,
    ///QY - Does Organization Receive Income from the Sale or Lease of Tangible Personal Property, the Lease of Real Property, or the Sale of Taxable Services?
    CodeQY,
    ///QZ - Is organization a contractor-retailer primarily engaged in retail sales?
    CodeQZ,
    ///R0 - Exempt from Public Records Law
    ExemptFromPublicRecordsLaw,
    ///R1 - Debtor Holds Claim to Real Property
    DebtorHoldsClaimToRealProperty,
    ///R2 - Entity Claims to Hold a Secured Interest
    EntityClaimsToHoldASecuredInterest,
    ///R3 - Debtor has Property of the Type Specified
    DebtorHasPropertyOfTheTypeSpecified,
    ///R4 - Debtor Elects the State Exemption
    DebtorElectsTheStateExemption,
    ///R5 - Debtor Elects the Federal Exemption
    DebtorElectsTheFederalExemption,
    ///R6 - Co-debtor may be Jointly Liable
    CoDebtorMayBeJointlyLiable,
    ///R7 - Claim is Contingent
    ClaimIsContingent,
    ///R8 - Claim is Unliquidated
    ClaimIsUnliquidated,
    ///R9 - Claim is Disputed
    ClaimIsDisputed,
    ///RA - Reference Telephone Attempt
    ReferenceTelephoneAttempt,
    ///RB - Debtor has No Creditors Holding Unsecured Priority Claims
    DebtorHasNoCreditorsHoldingUnsecuredPriorityClaims,
    ///RC - Reference Telephone Contact
    ReferenceTelephoneContact,
    ///RCA - Rental Car Arranged
    RentalCarArranged,
    ///RD - Rent Delinquent
    RentDelinquent,
    ///RE - Claim is Subject to Setoff
    ClaimIsSubjectToSetoff,
    ///RF - Debtor has No Executory Contracts or Unexpired Leases
    DebtorHasNoExecutoryContractsOrUnexpiredLeases,
    ///RG - Lease is for Nonresidential Real Property
    LeaseIsForNonresidentialRealProperty,
    ///RH - Debtor has No Co-debtors
    DebtorHasNoCoDebtors,
    ///RI - Debtor is Married
    DebtorIsMarried,
    ///RJ - Debtor's Spouse Maintains a Separate Household
    DebtorsSpouseMaintainsASeparateHousehold,
    ///RK - Real Estate Taxes are Included
    RealEstateTaxesAreIncluded,
    ///RL - Property Insurance is Included
    PropertyInsuranceIsIncluded,
    ///RM - Debtor has No Creditors Holding Secured Claims
    DebtorHasNoCreditorsHoldingSecuredClaims,
    ///RN - Rent Control
    RentControl,
    ///RO - Equipment is Rebuilt
    EquipmentIsRebuilt,
    ///RP - Individual Injured in Performance of Duty
    IndividualInjuredInPerformanceOfDuty,
    ///RQ - Individual Injured by Third Party
    IndividualInjuredByThirdParty,
    ///RR - Quality of Management and its Enforcement of Rules and Regulations Based on General Appearances
    QualityOfManagementAndItsEnforcementOfRulesAndRegulationsBasedOnGeneralAppearances,
    ///RS - Pay Continued
    PayContinued,
    ///RT - Sick Leave Taken
    SickLeaveTaken,
    ///RU - Signature on File
    SignatureOnFile,
    ///RV - Low Refrigerant Capacity Shutdown
    LowRefrigerantCapacityShutdown,
    ///RW - Recent Defrost
    RecentDefrost,
    ///RX - Rated Horsepower can be Produced
    RatedHorsepowerCanBeProduced,
    ///RY - Foreign Military Sale
    ForeignMilitarySale,
    ///RZ - Waiver of Prior Notice
    WaiverOfPriorNotice,
    ///S0 - Alternate Certification Program Participant
    AlternateCertificationProgramParticipant,
    ///S1 - Services Provided at the Second Facility were available at the First Facility
    ServicesProvidedAtTheSecondFacilityWereAvailableAtTheFirstFacility,
    ///S2 - Under Treatment
    UnderTreatment,
    ///S3 - First Time Vacant
    FirstTimeVacant,
    ///S4 - Adverse Easement
    AdverseEasement,
    ///S5 - Disclosure Indicator
    DisclosureIndicator,
    ///S6 - Atypical Off Site Improvements
    AtypicalOffSiteImprovements,
    ///S7 - Toxic Substances
    ToxicSubstances,
    ///S8 - Adverse Encroachment
    AdverseEncroachment,
    ///S9 - Atypical Functional Condition
    AtypicalFunctionalCondition,
    ///SA - Subject Property is Currently Listed
    SubjectPropertyIsCurrentlyListed,
    ///SB - Debtor is a Small Business as Defined in 11 U.S.C. Section 101
    DebtorIsASmallBusinessAsDefinedIn11USCSection101,
    ///SC - Special Services are Mobile Home Only
    SpecialServicesAreMobileHomeOnly,
    ///SD - Special Services are Leasehold or Mobile Home or Both
    SpecialServicesAreLeaseholdOrMobileHomeOrBoth,
    ///SE - Debtor Elects to be Considered as a Small Business Under 11 U.S.C. Section 1121(e)
    CodeSE,
    ///SF - Sensor Fault
    SensorFault,
    ///SG - Street Lights are Public
    StreetLightsArePublic,
    ///SH - Special Services are Leasehold or Subleasehold or Both
    SpecialServicesAreLeaseholdOrSubleaseholdOrBoth,
    ///SI - Hazardous Waste
    HazardousWaste,
    ///SJ - Pest Infestation
    PestInfestation,
    ///SK - Road Maintenance Required
    RoadMaintenanceRequired,
    ///SL - Speech Limitations
    SpeechLimitations,
    ///SM - Currently Serving in Military
    CurrentlyServingInMilitary,
    ///SN - Major Base Support
    MajorBaseSupport,
    ///SO - Critical Support Level Met
    CriticalSupportLevelMet,
    ///SP - Street is Public
    StreetIsPublic,
    ///SPP - Specialty is Primary
    SpecialtyIsPrimary,
    ///SPS - Specialty is Secondary
    SpecialtyIsSecondary,
    ///SQ - Local Wages in Effect
    LocalWagesInEffect,
    ///SR - Federal Worker Displacement
    FederalWorkerDisplacement,
    ///SS - Adverse Zoning
    AdverseZoning,
    ///ST - New Services Requested
    NewServicesRequested,
    ///SU - Continued Services Requested
    ContinuedServicesRequested,
    ///SUB - Subrogation Open
    SubrogationOpen,
    ///SV - Major Corporation/High Tech
    MajorCorporationHighTech,
    ///SW - Sidewalk is Public
    SidewalkIsPublic,
    ///SX - Collective Bargaining Agreement Sent by Mail
    CollectiveBargainingAgreementSentByMail,
    ///SY - Collective Bargaining Agreement Sent by Facsimile
    CollectiveBargainingAgreementSentByFacsimile,
    ///SZ - Contract
    Contract,
    ///T0 - Under Contract
    UnderContract,
    ///T1 - Road Test Performed with No Problems Reported
    RoadTestPerformedWithNoProblemsReported,
    ///T2 - Road Test Performed with Problems Reported
    RoadTestPerformedWithProblemsReported,
    ///T3 - Tires' Brand Match
    TiresBrandMatch,
    ///T4 - Real Estate Taxes are Current
    RealEstateTaxesAreCurrent,
    ///T5 - Hazard Insurance is Current
    HazardInsuranceIsCurrent,
    ///T6 - Terminate Guarantee
    TerminateGuarantee,
    ///T7 - Atypical External Condition
    AtypicalExternalCondition,
    ///T8 - Subsidence (Settlement of Ground Surface Caused by Loss of Support)
    CodeT8,
    ///T9 - Utilities Inadequate
    UtilitiesInadequate,
    ///TA - Collective Bargaining Agreement Sent by Electronic Bulletin Board
    CollectiveBargainingAgreementSentByElectronicBulletinBoard,
    ///TB - Debtor has No Creditors Holding Unsecured Nonpriority Claims
    DebtorHasNoCreditorsHoldingUnsecuredNonpriorityClaims,
    ///TC - Transport via Cargo Aircraft
    TransportViaCargoAircraft,
    ///TD - Annual Leave Taken
    AnnualLeaveTaken,
    ///TE - Item is Special Test Equipment
    ItemIsSpecialTestEquipment,
    ///TF - Operates as Representative For Others
    OperatesAsRepresentativeForOthers,
    ///TG - Claim Involves Work Related Death
    ClaimInvolvesWorkRelatedDeath,
    ///TH - Claim Does Not Involve Work Related Death, Days Away from Work, or Restricted Work Activity
    CodeTH,
    ///TI - Employee Has Not Recovered to Return to Work
    EmployeeHasNotRecoveredToReturnToWork,
    ///TJ - Employee Has Retired
    EmployeeHasRetired,
    ///TK - Employee Has Resigned
    EmployeeHasResigned,
    ///TL - Employee is Permanently and Totally Disabled
    EmployeeIsPermanentlyAndTotallyDisabled,
    ///TM - Traction Motor is Cut Out
    TractionMotorIsCutOut,
    ///TN - Atypical Quality of Construction
    AtypicalQualityOfConstruction,
    ///TNJ - Traumatic Injury
    TraumaticInjury,
    ///TO - Atypical Remodeling
    AtypicalRemodeling,
    ///TP - Transport via Passenger Aircraft
    TransportViaPassengerAircraft,
    ///TQ - Atypical Additions
    AtypicalAdditions,
    ///TR - Transfer to Bed, or Chair, or Both
    CodeTR,
    ///TS - Adverse Marketing Conditions in Subject Property's Neighborhood
    AdverseMarketingConditionsInSubjectPropertysNeighborhood,
    ///TT - Neighborhood Water Source is Public
    NeighborhoodWaterSourceIsPublic,
    ///TU - Neighborhood Sewage Treatment is Public
    NeighborhoodSewageTreatmentIsPublic,
    ///TV - Telephone Number Verified
    TelephoneNumberVerified,
    ///TW - Neighborhood Street is Public
    NeighborhoodStreetIsPublic,
    ///TX - Other Miscellaneous Adverse Characteristics
    OtherMiscellaneousAdverseCharacteristics,
    ///TY - Subject Property's Street is Public
    SubjectPropertysStreetIsPublic,
    ///TZ - Subject Property's Sewage Treatment is Public
    SubjectPropertysSewageTreatmentIsPublic,
    ///U0 - Disability
    Disability,
    ///U1 - Minimal Change
    MinimalChange,
    ///U2 - Neat Appearance
    NeatAppearance,
    ///U3 - Net Worth Computed after Exemptions
    NetWorthComputedAfterExemptions,
    ///U4 - Net Worth Considerably Higher
    NetWorthConsiderablyHigher,
    ///U5 - Net Worth Higher
    NetWorthHigher,
    ///U6 - No Employees
    NoEmployees,
    ///U7 - No Employees - Business Managed by Owner
    NoEmployeesBusinessManagedByOwner,
    ///U8 - No Employees - Business Managed by Partners
    NoEmployeesBusinessManagedByPartners,
    ///U9 - Not Out of Business
    NotOutOfBusiness,
    ///UA - Uninsurable, 1316 Property
    CodeUA,
    ///UB - Conducted at a Profit
    ConductedAtAProfit,
    ///UC - Contingent Debt Indicated
    ContingentDebtIndicated,
    ///UD - Continue
    Continue,
    ///UE - Contracts Obtained by Bid
    ContractsObtainedByBid,
    ///UF - Contracts Obtained by Negotiation
    ContractsObtainedByNegotiation,
    ///UG - Converted to Holding Company
    ConvertedToHoldingCompany,
    ///UH - Cross Claim Filed
    CrossClaimFiled,
    ///UI - Declining Tendency
    DecliningTendency,
    ///UJ - Detrimental Events in Past, Relating to Business
    CodeUJ,
    ///UK - Detrimental Events in Past, Relating to Management
    CodeUK,
    ///UL - Down or Decline or Decreased
    DownOrDeclineOrDecreased,
    ///UM - Employees Include Officers
    EmployeesIncludeOfficers,
    ///UN - Uncooperative
    Uncooperative,
    ///UO - Employees Include Owners
    EmployeesIncludeOwners,
    ///UP - Employees Include Partners
    EmployeesIncludePartners,
    ///UQ - Employees Include Temporary Workers
    EmployeesIncludeTemporaryWorkers,
    ///UR - Employees Vary According to Needs
    EmployeesVaryAccordingToNeeds,
    ///US - Enclosed
    Enclosed,
    ///UT - Up as Tolerated
    UpAsTolerated,
    ///UU - Extent of Audit, if any, Not Indicated
    CodeUU,
    ///UV - Favorable Personal Reputation
    FavorablePersonalReputation,
    ///UW - Figures are Abbreviated
    FiguresAreAbbreviated,
    ///UX - Figures are Converted to Agency Format
    FiguresAreConvertedToAgencyFormat,
    ///UY - Figures are Individual
    FiguresAreIndividual,
    ///UZ - Figures are Restated
    FiguresAreRestated,
    ///V0 - Ultimate Parent Company Financial Statement Used
    UltimateParentCompanyFinancialStatementUsed,
    ///V1 - Valid Borrower Address or Phone Attempt with School Attended
    ValidBorrowerAddressOrPhoneAttemptWithSchoolAttended,
    ///V2 - Lender Determined Borrower Moved Out of State
    LenderDeterminedBorrowerMovedOutOfState,
    ///V3 - Lender Determined Borrower Moved Back into State
    LenderDeterminedBorrowerMovedBackIntoState,
    ///V4 - Lender Determined Borrower Incarcerated
    LenderDeterminedBorrowerIncarcerated,
    ///V5 - Lender Determined Borrower No Longer Incarcerated
    LenderDeterminedBorrowerNoLongerIncarcerated,
    ///V6 - Original
    Original,
    ///V7 - True and Exact Copy
    TrueAndExactCopy,
    ///V8 - Subject Property's Water Source is Public
    SubjectPropertysWaterSourceIsPublic,
    ///V9 - Pictures Required
    PicturesRequired,
    ///VA - Intercompany Relations Exist
    IntercompanyRelationsExist,
    ///VB - Inventory Valued at Lower of Cost or Market
    InventoryValuedAtLowerOfCostOrMarket,
    ///VC - Inventory Valued at Other Methods
    InventoryValuedAtOtherMethods,
    ///VD - Operates as Sole Agent
    OperatesAsSoleAgent,
    ///VE - Without Personal Judgment
    WithoutPersonalJudgment,
    ///VF - Work is Subcontracted
    WorkIsSubcontracted,
    ///VG - Not Registered
    NotRegistered,
    ///VH - Immediate Attention Required
    ImmediateAttentionRequired,
    ///VI - Vehicle Inspection Report Completed
    VehicleInspectionReportCompleted,
    ///VJ - Middle to Medium
    MiddleToMedium,
    ///VK - Rent Control Likely
    RentControlLikely,
    ///VL - Furnished
    Furnished,
    ///VM - Price Range Single Family or Planned Unit Development Not Applicable
    PriceRangeSingleFamilyOrPlannedUnitDevelopmentNotApplicable,
    ///VN - Price Range Condominium Not Applicable
    PriceRangeCondominiumNotApplicable,
    ///VO - Price Range Two to Four Family Not Applicable
    PriceRangeTwoToFourFamilyNotApplicable,
    ///VP - Financial Figures are Projected Based on Sales
    FinancialFiguresAreProjectedBasedOnSales,
    ///VQ - Financial Figures are Projected Based on Employees
    FinancialFiguresAreProjectedBasedOnEmployees,
    ///VR - Parent Company has Bankruptcy
    ParentCompanyHasBankruptcy,
    ///VS - Headquarters has Bankruptcy
    HeadquartersHasBankruptcy,
    ///VT - Commercial Motor Vehicle was Involved in this Conviction
    CommercialMotorVehicleWasInvolvedInThisConviction,
    ///VTL - Vehicle was Declared a Total Loss
    VehicleWasDeclaredATotalLoss,
    ///VU - Commercial Motor Vehicle was Carrying Hazardous Materials when the Offense was Committed
    CommercialMotorVehicleWasCarryingHazardousMaterialsWhenTheOffenseWasCommitted,
    ///VV - Prepared from Internal Book Figures
    PreparedFromInternalBookFigures,
    ///VW - Quantity Declined
    QuantityDeclined,
    ///VX - Quantity Details Unknown
    QuantityDetailsUnknown,
    ///VY - Was tax paid when purchased by seller?
    CodeVY,
    ///VZ - Was item depreciable?
    CodeVZ,
    ///W0 - Statement is on a Trading Trust
    StatementIsOnATradingTrust,
    ///W1 - New Registration
    NewRegistration,
    ///W2 - Mailing Address Change
    MailingAddressChange,
    ///W3 - Residence Address Change
    ResidenceAddressChange,
    ///W4 - Name Change
    NameChange,
    ///W5 - Party Enrollment Change
    PartyEnrollmentChange,
    ///W6 - Needs Absentee Ballot
    NeedsAbsenteeBallot,
    ///W7 - Would Like to be Election Day Worker
    WouldLikeToBeElectionDayWorker,
    ///W8 - Duplicate Registration
    DuplicateRegistration,
    ///W9 - Forwarded Application
    ForwardedApplication,
    ///WA - Walker Required
    WalkerRequired,
    ///WB - Water On
    WaterOn,
    ///WC - Application Incomplete
    ApplicationIncomplete,
    ///WD - Vehicle Plate Surrendered
    VehiclePlateSurrendered,
    ///WE - Written Notice to Note Holder
    WrittenNoticeToNoteHolder,
    ///WF - Written Notice to Borrower
    WrittenNoticeToBorrower,
    ///WG - Within Specified Time Period
    WithinSpecifiedTimePeriod,
    ///WH - Within Specified Range
    WithinSpecifiedRange,
    ///WI - Injury was Work Related
    InjuryWasWorkRelated,
    ///WJ - Dealer Pricing Authorization
    DealerPricingAuthorization,
    ///WK - Summary Level Information
    SummaryLevel,
    ///WL - Detail Level Information
    DetailLevel,
    ///WM - Non-occupant Co-borrower
    NonOccupantCoBorrower,
    ///WN - Unit is a Studio (Efficiency)
    CodeWN,
    ///WO - Equipment in Working Order
    EquipmentInWorkingOrder,
    ///WP - To be Watched
    ToBeWatched,
    ///WQ - Undetermined Out of Business Status
    UndeterminedOutOfBusinessStatus,
    ///WR - Wheelchair Required
    WheelchairRequired,
    ///WS - Balance Sheet Filed
    BalanceSheetFiled,
    ///WT - Winterized Tag Observed
    WinterizedTagObserved,
    ///WU - Material Safety Data Sheet
    MaterialSafetyDataSheet,
    ///WV - Accepts Credit Cards
    AcceptsCreditCards,
    ///WW - All Purchases Made from Headquarters
    AllPurchasesMadeFromHeadquarters,
    ///WX - Busy
    Busy,
    ///WY - Excessive
    Excessive,
    ///WZ - Fairly new
    FairlyNew,
    ///X0 - No Employees - Business Managed by Director(s)
    CodeX0,
    ///X1 - Gross Weekly Amount is Estimated
    GrossWeeklyAmountIsEstimated,
    ///X2 - Waiting Period Disability Days are Non-consecutive
    WaitingPeriodDisabilityDaysAreNonConsecutive,
    ///X3 - Report Depicts Most Recent Data - Interim Period(s) Omitted
    CodeX3,
    ///X4 - Permanent Impairment Paid at Minimum
    PermanentImpairmentPaidAtMinimum,
    ///X5 - Employee's Death is a Result of Work Injury or Illness
    EmployeesDeathIsAResultOfWorkInjuryOrIllness,
    ///X6 - Employee's Written Social Security Number Release is on File
    EmployeesWrittenSocialSecurityNumberReleaseIsOnFile,
    ///X7 - Employee's Medical Records Release Authorization is on File
    EmployeesMedicalRecordsReleaseAuthorizationIsOnFile,
    ///X8 - Employee Returned to Work with Pre-Injury Employer
    EmployeeReturnedToWorkWithPreInjuryEmployer,
    ///X9 - "Cafe" Plan in Effect
    CodeX9,
    ///XA - Figures are Average
    FiguresAreAverage,
    ///XB - Imports
    Imports,
    ///XC - In Process of Establishing
    InProcessOfEstablishing,
    ///XD - Intercompany Relations Consist of Endorsements
    IntercompanyRelationsConsistOfEndorsements,
    ///XE - Intercompany Relations Consist of Guarantees
    IntercompanyRelationsConsistOfGuarantees,
    ///XF - Intercompany Relations Consist of Leasing Arrangements
    IntercompanyRelationsConsistOfLeasingArrangements,
    ///XG - Intercompany Relations Consist of Sharing Accounting
    IntercompanyRelationsConsistOfSharingAccounting,
    ///XH - Intercompany Relations Consist of Sharing Facilities
    IntercompanyRelationsConsistOfSharingFacilities,
    ///XI - Intercompany Relations Consist of Sharing Management
    IntercompanyRelationsConsistOfSharingManagement,
    ///XJ - Intercompany Relations Consist of Sharing Personnel
    IntercompanyRelationsConsistOfSharingPersonnel,
    ///XK - Interest in Other Business(es) Along with Family
    CodeXK,
    ///XL - Interest in Other Business(es) Along with Others in Reported Company
    CodeXL,
    ///XM - Inventory Valued at Company's Estimates
    InventoryValuedAtCompanysEstimates,
    ///XN - Inventory Valued at Cost
    InventoryValuedAtCost,
    ///XO - Inventory Valued using AVCO (Average Cost)
    CodeXO,
    ///XP - Joint Ownership
    JointOwnership,
    ///XQ - Leases with No Rent Payments
    LeasesWithNoRentPayments,
    ///XR - Leases with Option to Buy
    LeasesWithOptionToBuy,
    ///XS - Leases with Token Payment
    LeasesWithTokenPayment,
    ///XT - Limited
    Limited,
    ///XU - Located for Several Years
    LocatedForSeveralYears,
    ///XV - Located Since Opening
    LocatedSinceOpening,
    ///XW - Modern
    Modern,
    ///XX - Non-Existent
    NonExistent,
    ///XY - Officer or Owner in Other Businesses in the Same Field
    OfficerOrOwnerInOtherBusinessesInTheSameField,
    ///XZ - Operates as a Distributor for Others
    OperatesAsADistributorForOthers,
    ///Y0 - Insured Cooperative
    InsuredCooperative,
    ///Y1 - Worked in Industry for Several Years
    WorkedInIndustryForSeveralYears,
    ///Y2 - Aircraft Operation
    AircraftOperation,
    ///Y3 - All Classifications on Policy Accounted For
    AllClassificationsOnPolicyAccountedFor,
    ///Y4 - Board Provided
    BoardProvided,
    ///Y5 - Casual Labor
    CasualLabor,
    ///Y6 - Certificates on File for All Subcontractors
    CertificatesOnFileForAllSubcontractors,
    ///Y7 - Commissions Paid
    CommissionsPaid,
    ///Y8 - Condition or Type of Records Cause Additional Audit Time
    ConditionOrTypeOfRecordsCauseAdditionalAuditTime,
    ///Y9 - Domestic Workers Employed
    DomesticWorkersEmployed,
    ///YA - Operates from Residence
    OperatesFromResidence,
    ///YB - Operates under License by Others
    OperatesUnderLicenseByOthers,
    ///YC - Rents from Month to Month
    RentsFromMonthToMonth,
    ///YD - Semi-modern
    SemiModern,
    ///YE - Under Construction
    UnderConstruction,
    ///YF - Unlimited
    Unlimited,
    ///YG - Used
    Used,
    ///YH - Variable
    Variable,
    ///YI - Holder is a Subsidiary of Reporting Agent
    HolderIsASubsidiaryOfReportingAgent,
    ///YJ - Contact is Unchanged From Previous Report
    ContactIsUnchangedFromPreviousReport,
    ///YK - Report was Filed Last Year by This Agent
    ReportWasFiledLastYearByThisAgent,
    ///YL - Party is Authorized to do Business in This State
    PartyIsAuthorizedToDoBusinessInThisState,
    ///YM - Clear Decrease
    ClearDecrease,
    ///YN - Employees Temporarily Laid Off
    EmployeesTemporarilyLaidOff,
    ///YO - Established in the Industry
    EstablishedInTheIndustry,
    ///YP - Global Business
    GlobalBusiness,
    ///YQ - Information to be Followed Up
    InformationToBeFollowedUp,
    ///YR - Known Details are Listed
    KnownDetailsAreListed,
    ///YS - Land is Rented
    LandIsRented,
    ///YT - Low
    Low,
    ///YU - Prime Commercial Area
    PrimeCommercialArea,
    ///YV - Shares with Affiliated Company(ies)
    CodeYV,
    ///YW - Slightly Higher
    SlightlyHigher,
    ///YX - Slightly Lower
    SlightlyLower,
    ///YY - Stagnant
    Stagnant,
    ///YZ - Territory Information is Available
    TerritoryInformationIsAvailable,
    ///Z0 - Subcontractors Used
    SubcontractorsUsed,
    ///Z1 - Insured Is a Subcontractor
    InsuredIsASubcontractor,
    ///Z2 - Insured Has Multiple Entries
    InsuredHasMultipleEntries,
    ///Z3 - Insured Has Retail Operations
    InsuredHasRetailOperations,
    ///Z4 - Insured Requested Division of Payroll of Employee(s)
    CodeZ4,
    ///Z5 - Owner or Officer Interviewed
    OwnerOrOfficerInterviewed,
    ///Z6 - Premium Overtime Excluded
    PremiumOvertimeExcluded,
    ///Z7 - Records Reflect Proper Division of Employee(s) Payroll
    CodeZ7,
    ///Z8 - Records Satisfactory for Audit
    RecordsSatisfactoryForAudit,
    ///Z9 - Relatives Employed
    RelativesEmployed,
    ///ZA - Customer - Configuration Change is Required
    CustomerConfigurationChangeIsRequired,
    ///ZB - Condition Board of Inspection and Survey (INSURV) is Mission Degrading
    CodeZB,
    ///ZC - Condition Board of Inspection and Survey (INSURV) is Maintenance Related
    CodeZC,
    ///ZD - Condition Board of Inspection and Survey (INSURV) is Safety Related
    CodeZD,
    ///ZE - Repair is Mission Essential
    RepairIsMissionEssential,
    ///ZF - Repair is Safety Essential
    RepairIsSafetyEssential,
    ///ZG - Periodic Maintenance is Required
    PeriodicMaintenanceIsRequired,
    ///ZH - Condition Board of Inspection and Survey (INSURV) Discrepancy is Corrected
    CodeZH,
    ///ZI - Progress is in Jeopardy
    ProgressIsInJeopardy,
    ///ZJ - Employee's Injury or Illness is Work Related
    EmployeesInjuryOrIllnessIsWorkRelated,
    ///ZK - Final - Configuration Change is Required
    FinalConfigurationChangeIsRequired,
    ///ZL - Final - Delivery to Shop is Required
    FinalDeliveryToShopIsRequired,
    ///ZM - Final - Requestor Workforce will Assist
    FinalRequestorWorkforceWillAssist,
    ///ZN - Job is Level 2
    JobIsLevel2,
    ///ZO - Preliminary - Configuration Change is Required
    PreliminaryConfigurationChangeIsRequired,
    ///ZP - Preliminary - Delivery to Shop is Required
    PreliminaryDeliveryToShopIsRequired,
    ///ZQ - Preliminary - Requestor Workforce will Assist
    PreliminaryRequestorWorkforceWillAssist,
    ///ZR - Configuration Change is Associated with Time Meter
    ConfigurationChangeIsAssociatedWithTimeMeter,
    ///ZS - Shop Has Lead Responsibility
    ShopHasLeadResponsibility,
    ///ZT - Estimate is Derived From Job Template
    EstimateIsDerivedFromJobTemplate,
    ///ZU - Requestor Holds Technical Documentation
    RequestorHoldsTechnicalDocumentation,
    ///ZV - Replacement Item
    ReplacementItem,
    ///ZW - Canadian Standards Association (CSA) Approved
    CodeZW,
    ///ZX - Non-convertible
    NonConvertible,
    ///ZY - Underwriters Laboratory (UL) Approved
    CodeZY,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl ConditionIndicator {
    pub fn code(&self) -> &str {
        {
            use ConditionIndicator::*;
            match self {
                Requested => "00",
                InProgress => "000",
                Code0A => "0A",
                Code0B => "0B",
                Code0C => "0C",
                FacilitysEmergencyResponsePlanIncludesInformationOnEmergencyHealthCare => {
                    "0D"
                }
                FacilitysEmergencyResponsePlanIncludesProceduresForInformingPublicAndLocalAgenciesResponsibleForRespondingToAnAccidentalRelease => {
                    "0E"
                }
                FacilityHasACleanAirActTitleVOperatingPermit => "0F",
                FacilityHasAWrittenEmergencyResponsePlan => "0G",
                FacilityHasReportableAccidents => "0H",
                Code0I => "0I",
                Code0J => "0J",
                Code0K => "0K",
                Code0L => "0L",
                OffsiteRespondersNotified => "0M",
                PrecipitationPresent => "0N",
                DisabledVeteran => "0O",
                ServicerHasAdvancedFundsToPayForDelinquentTaxesOnNonEscrowedMortgage => {
                    "0P"
                }
                PropertyHasFireInsuranceOnlyThatWasNotLenderPlaced => "0Q",
                ReportedButUnconfirmed => "0R",
                HasSmokeAlarms => "0S",
                OperatesAsAHoldingCompany => "0T",
                Optimum => "0U",
                Renewed => "0V",
                HighestEducationalLevel => "0W",
                PrincipalCertificate => "0X",
                InserviceEducationCompleted => "0Y",
                MainAssignment => "0Z",
                PatientWasAdmittedToAHospital => "01",
                PatientIsReceivingAntiFungalTherapy => "1A",
                PropertyIsOccupiedByOwner => "1B",
                PropertyIsOccupiedByTenant => "1C",
                PropertyIsVacant => "1D",
                LocationIsUrban => "1E",
                LocationIsSuburban => "1F",
                LocationIsRural => "1G",
                Code1H => "1H",
                Code1I => "1I",
                Code1J => "1J",
                GrowthRateIsRapid => "1K",
                ClassILeft => "1L",
                GrowthRateIsStable => "1M",
                GrowthRateIsSlow => "1N",
                PropertyValuesAreIncreasing => "1O",
                PropertyValuesAreStable => "1P",
                PropertyValuesAreDeclining => "1Q",
                ClassIRight => "1R",
                DemandOrSupplyIsInShortage => "1S",
                DemandOrSupplyIsInBalance => "1T",
                DemandOrSupplyIsOverSupply => "1U",
                MarketingTimeIsUnder3Months => "1V",
                MarketingTimeIs3To6Months => "1W",
                MarketingTimeIsOver6Months => "1X",
                PredominantOccupancyIsTheOwner => "1Y",
                PredominantOccupancyIsTheTenant => "1Z",
                PatientWasBedConfinedBeforeTheAmbulanceService => "02",
                PatientIsReceivingOralAntiFungalTherapy => "2A",
                Code2B => "2B",
                Code2C => "2C",
                DeveloperOrBuilderIsInControlOfTheHomeOwnersAssociation => "2D",
                SiteIsACornerLot => "2E",
                ZoningComplianceIsLegal => "2F",
                Code2G => "2G",
                ZoningComplianceIsIllegal => "2H",
                ThereIsNoZoning => "2I",
                HighestAndBestUseAsImprovedIsThePresentUse => "2J",
                HighestAndBestUseAsImprovedIsOtherUse => "2K",
                ClassIiLeft => "2L",
                PropertyIsLocatedInAFederalEmergencyManagementAdministrationSpecialFloodHazardArea => {
                    "2M"
                }
                Code2N => "2N",
                Code2O => "2O",
                AppraisalIsMadeSubjectToTheCompletionPerPlansAndSpecifications => "2P",
                Code2Q => "2Q",
                ClassIiRight => "2R",
                ProjectTypeIsCondominium => "2S",
                PropertyRightsAreFeeSimple => "2T",
                PropertyRightsAreLeasehold => "2U",
                SupervisorAppraiserInspectedThePropertyPerSupervisoryAppraisersCertification => {
                    "2V"
                }
                PropertyWasSoldWithinLast12Months => "2W",
                AppraiserSignedStatementOfLimitingConditionsAndDisclaimer => "2X",
                OwnershipInterestInAProperty => "2Y",
                Termination => "2Z",
                PatientWasBedConfinedAfterTheAmbulanceService => "03",
                PatientIsReceivingTopicalAntiFungalTherapy => "3A",
                PointsPaidBySeller => "3B",
                PointsPaidByBuyer => "3C",
                SellerConcession => "3D",
                LetterOfCertification => "3E",
                VerbalReportNeeded => "3F",
                AnyRelationshipBetweenOwnerAndOccupant => "3G",
                MapAndDirectionsToRemotePropertiesToFollow => "3H",
                GroundLeaseToFollow => "3I",
                DisclosureStatementToFollow => "3J",
                CopyOfPropertyListingToFollow => "3K",
                ClassIiiLeft => "3L",
                CopyOfTitleReportPlatMapToFollow => "3M",
                PropertyTaxBillToFollow => "3N",
                EngineeringOrSoilReportToFollow => "3O",
                SalesContractAvailable => "3P",
                LeaveWillBeTaken => "3Q",
                ClassIiiRight => "3R",
                Approved => "3S",
                BalanceSheetDoesNotBalance => "3T",
                BankingDoneThroughParentCompany => "3U",
                BankingDoneThroughRelatedConcern => "3V",
                BankingDoneThroughSubsidiary => "3W",
                CanNotDetermineIfSubjectEngagedInBusiness => "3X",
                Deteriorated => "3Y",
                DetrimentalInformationReceived => "3Z",
                PatientWasMovedByStretcher => "04",
                ServicesAreRenderedWithinHospiceElectedPeriodOfCoverage => "4A",
                Accidents => "4B",
                AccountRepresentativeTransfer => "4C",
                AdditionalCoverage => "4D",
                AdviceToStop => "4E",
                AgentReplacement => "4F",
                BackupWithholding => "4G",
                CurrentEmployer => "4H",
                CurrentOccupation => "4I",
                EmployerReimbursement => "4J",
                Code4K => "4K",
                ExpectedChanges => "4L",
                Experimental => "4M",
                ForeignFlight => "4N",
                FutureInvolvement => "4O",
                Code4P => "4P",
                GroupDisabilityInsuranceConversion => "4Q",
                GroupDisabilityInsuranceOffset => "4R",
                GroupDisabilityInsuranceParticipation => "4S",
                GroupDisabilityInsuranceTopUp => "4T",
                HomeEmployment => "4U",
                InformationOmitted => "4V",
                InjuryBenefits => "4W",
                IssueAtHigherPremiums => "4X",
                IssueWithExclusions => "4Y",
                IssueWithoutBenefits => "4Z",
                PatientWasUnconsciousOrInShock => "05",
                TreatmentIsRenderedRelatedToTheTerminalIllness => "5A",
                Code5B => "5B",
                Code5C => "5C",
                JuvenileSeen => "5D",
                MedicalTreatment => "5E",
                MilitaryAviation => "5F",
                NewGroup => "5G",
                OtherCoverageOffset => "5H",
                OtherPrincipalsBeingInsured => "5I",
                OwnerActiveInBusiness => "5J",
                PayrollDeduction => "5K",
                Prepaid => "5L",
                PreviousApplication => "5M",
                PrimaryOccupation => "5N",
                RacingAccident => "5O",
                Replacement => "5P",
                ResidesWithApplicant => "5Q",
                GenderDistinct => "5R",
                SiblingCoverage => "5S",
                SicknessBenefits => "5T",
                SpecialDating => "5U",
                SpousalConsent => "5V",
                SuitabilityAnalysis => "5W",
                SuitableForCoverage => "5X",
                Taxable => "5Y",
                ThisCompanyReplacement => "5Z",
                PatientWasTransportedInAnEmergencySituation => "06",
                TreatmentIsRenderedByAHospiceEmployedPhysician => "6A",
                UnitedStatesCitizen => "6B",
                PermanentResidentAlien => "6C",
                BorrowerIsFirstTimeHomebuyer => "6D",
                UnemploymentClaims => "6E",
                UnemploymentInsuranceEligibility => "6F",
                WorkStatus => "6G",
                WorkersCompensationEligible => "6H",
                FactoredOnRecourseBasis => "6I",
                FactoredWithAdvances => "6J",
                FiguresAreActual => "6K",
                FiguresAreAnticipated => "6L",
                FiguresAreEstimated => "6M",
                FiguresAreModified => "6N",
                FiguresAreProjected => "6O",
                GovernmentBusinessNumberUnavailable => "6P",
                GoodwillOriginPurchasedFromBankruptCompany => "6Q",
                GoodwillOriginRented => "6R",
                HasNoOwnership => "6S",
                Improved => "6T",
                IntangiblesBreakdownAvailable => "6U",
                IntangiblesIncludeOrganizationalExpense => "6V",
                IntercompanyRelationsConsistOfLoansAndAdvances => "6W",
                IntercompanyRelationsConsistOfMerchandiseTransactions => "6X",
                IntercompanyRelationsConsistOfServiceTransactions => "6Y",
                LocalBankingUtilizedOnATransferAccountBasis => "6Z",
                PatientHadToBePhysicallyRestrained => "07",
                TreatmentIsRenderedByAPrivateAttendingPhysician => "7A",
                MedicationsOrderedAreBeingAdministeredIntramuscularly => "7B",
                MedicationsOrderedAreBeingAdministeredIntravenously => "7C",
                MedicationsOrderedAreBeingAdministeredOrally => "7D",
                MaintainsNoInventory => "7E",
                MedicationsOrderedAreBeingAdministeredSubcutaneously => "7F",
                Majority => "7G",
                MarketableSecuritiesValuedAtCost => "7H",
                MarketableSecuritiesValuedAtLowerOfCostOrMarket => "7I",
                InteriorAccessDenied => "7J",
                RepairsAreRecommended => "7K",
                LoanOriginatedUnderSharedEquityPlan => "7L",
                TitleAndOrLegalIssuesExist => "7M",
                EnvironmentalIssuesExist => "7N",
                PropertyIsListedAsIs => "7O",
                PropertyIsListedAsRepaired => "7P",
                VacancyRateIsGreaterThan5PercentTo10Percent => "7Q",
                VacancyRateIsGreaterThan10PercentTo20Percent => "7R",
                VacancyRateIsGreaterThan20Percent => "7S",
                MostComparableProperty => "7T",
                AnticipateIssuesWhichAffectAbilityToSecureFinancing => "7U",
                PointsArePaidBySeller => "7V",
                PropertyCoveredByFloodInsurancePolicy => "7W",
                PropertyCoveredByEarthquakeInsurancePolicy => "7X",
                PointsAreNegotiable => "7Y",
                PropertyIsCurrentlyListedWithARealEstateFirm => "7Z",
                PatientHadVisibleHemorrhaging => "08",
                TreatmentIsCurative => "8A",
                IncomeOrAssetsOfAnotherUsed => "8B",
                DisclosureOfSomeoneElsesLiabilitiesRequired => "8C",
                Code8D => "8D",
                Code8E => "8E",
                DistantSuburban => "8F",
                SelfEmployed => "8G",
                LiabilityToBeSatisfied => "8H",
                AreAssetsLiabilitiesReportedJointly => "8I",
                LocationIsFarm => "8J",
                LocationIsResort => "8K",
                ShortageExistForCompetingListings => "8L",
                CompetingListingsAreInBalance => "8M",
                OversupplyExistForCompetingListings => "8N",
                IncentivesAreOffered => "8O",
                ListedPropertyHasBeenInspected => "8P",
                SalePropertyHasBeenInspected => "8Q",
                GeneralMarketingConditionIsDepressed => "8R",
                GeneralMarketingConditionIsSlow => "8S",
                GeneralMarketingConditionIsStatic => "8T",
                GeneralMarketingConditionIsImproving => "8U",
                GeneralMarketingConditionIsExcellent => "8V",
                EmploymentConditionsAreStable => "8W",
                EmploymentConditionsAreDeclining => "8X",
                EmploymentConditionsAreIncreasing => "8Y",
                OverimprovementConditionExists => "8Z",
                AmbulanceServiceWasMedicallyNecessary => "09",
                TreatmentIsPalliative => "9A",
                InvoluntaryCommittal => "9B",
                LackOfAvailableEquipment => "9C",
                LackOfAppropriateFacilityWithinReasonableDistanceToTreatPatientInTheEventOfComplications => {
                    "9D"
                }
                SuddenOnsetOfDisorientation => "9E",
                Code9F => "9F",
                ContinuousHemorrhageFromAnySiteWithAbnormalLabValues => "9G",
                PatientRequiresIntensiveIvTherapy => "9H",
                PatientRequiresVolumeExpanders => "9I",
                PatientRequiresProtectiveIsolation => "9J",
                PatientRequiresFrequentMonitoring => "9K",
                PatientRequiresExtendedPostOperativeObservation => "9L",
                ForeclosureProceedingsHaveBegun => "9M",
                UnderimprovementConditionExists => "9N",
                MarketabilityOfPropertyIsExcellent => "9O",
                MarketabilityOfPropertyIsGood => "9P",
                MarketabilityOfPropertyIsFair => "9Q",
                MarketabilityOfPropertyIsPoor => "9R",
                FeesAreCurrent => "9S",
                FeesIncludeTennis => "9T",
                FeesIncludePool => "9U",
                FeesIncludeInsurance => "9V",
                FeesIncludeLandscape => "9W",
                FeesIncludeOtherAmenities => "9X",
                MostLikelyBuyerIsOwnerOccupant => "9Y",
                MostLikelyBuyerIsInvestor => "9Z",
                PatientIsAmbulatory => "10",
                AmbulationIsImpairedAndWalkingAidIsUsedForTherapyOrMobility => "11",
                PatientIsConfinedToABedOrChair => "12",
                PatientIsConfinedToARoomOrAnAreaWithoutBathroomFacilities => "13",
                AmbulationIsImpairedAndWalkingAidIsUsedForMobility => "14",
                PatientConditionRequiresPositioningOfTheBodyOrAttachmentsWhichWouldNotBeFeasibleWithTheUseOfAnOrdinaryBed => {
                    "15"
                }
                PatientNeedsATrapezeBarToSitUpDueToRespiratoryConditionOrChangeBodyPositionsForOtherMedicalReasons => {
                    "16"
                }
                PatientsAbilityToBreatheIsSeverelyImpaired => "17",
                PatientConditionRequiresFrequentAndOrImmediateChangesInBodyPositions => {
                    "18"
                }
                PatientCanOperateControls => "19",
                SiderailsAreToBeAttachedToAHospitalBedOwnedByTheBeneficiary => "20",
                PatientOwnsEquipment => "21",
                MattressOrSiderailsAreBeingUsedWithPrescribedMedicallyNecessaryHospitalBedOwnedByTheBeneficiary => {
                    "22"
                }
                PatientNeedsLiftToGetInOrOutOfBedOrToAssistInTransferFromBedToWheelchair => {
                    "23"
                }
                PatientHasAnOrthopedicImpairmentRequiringTractionEquipmentWhichPreventsAmbulationDuringPeriodOfUse => {
                    "24"
                }
                ItemHasBeenPrescribedAsPartOfAPlannedRegimenOfTreatmentInPatientHome => {
                    "25"
                }
                PatientIsHighlySusceptibleToDecubitusUlcers => "26",
                PatientOrACareGiverHasBeenInstructedInUseOfEquipment => "27",
                PatientHasPoorDiabeticControl => "28",
                A67HourNocturnalStudyDocuments30EpisodesOfApneaEachLastingMoreThan10Seconds => {
                    "29"
                }
                Code30 => "30",
                PatientHasHadATotalKneeReplacement => "31",
                PatientHasIntractableLymphedemaOfTheExtremities => "32",
                PatientIsInANursingHome => "33",
                PatientIsConscious => "34",
                ThisFeedingIsTheOnlyFormOfNutritionalIntakeForThisPatient => "35",
                PatientWasAdministeredPremix => "36",
                OxygenDeliveryEquipmentIsStationary => "37",
                CertificationSignedByThePhysicianIsOnFileAtTheSuppliersOffice => "38",
                PatientHasMobilizingRespiratoryTractSecretions => "39",
                PatientOrCaregiverIsCapableOfUsingTheEquipmentWithoutTechnicalOrProfessionalSupervision => {
                    "40"
                }
                PatientOrCaregiverIsUnableToPropelOrLiftAStandardWeightWheelchair => "41",
                PatientRequiresLegElevationForEdemaOrBodyAlignment => "42",
                PatientWeightOrUsageNeedsNecessitateAHeavyDutyWheelchair => "43",
                PatientRequiresRecliningFunctionOfAWheelchair => "44",
                PatientIsUnableToOperateAWheelchairManually => "45",
                Code46 => "46",
                AdvertisementRunCondition => "47",
                IndividualPaidForLastDayWorked => "48",
                FullWagesPaidForDateOfInjury => "49",
                CitationOrTicketIssued => "50",
                IndividualIsMemberOfPolicyholdersHousehold => "51",
                IndividualPermittedToUseVehicle => "52",
                IndividualWoreSeatbelt => "53",
                ChildRestraintDeviceInVehicle => "54",
                ChildRestraintDeviceUsed => "55",
                IndividualInjured => "56",
                IndividualTransportedToAnotherLocation => "57",
                Code58 => "58",
                Code59 => "59",
                TransportationWasToTheNearestFacility => "60",
                EmployeeIsExempt => "61",
                ClaimantIsCoveredOnTheEmployersLongTermDisabilityPlan => "62",
                EmployeesJobResponsibilitiesChangedDueToTheDisablingCondition => "63",
                EmployerHasAReturnToWorkPolicyForDisabledEmployees => "64",
                Open => "65",
                Normal => "66",
                ClosedModerate => "67",
                Severe => "68",
                Moderate => "69",
                Straight => "70",
                Convex => "71",
                Concave => "72",
                DoubleProtrusion => "73",
                NoCrossbite => "74",
                Posterior => "75",
                Anterior => "76",
                Maxillary => "77",
                Mandibular => "78",
                Right => "79",
                Left => "80",
                MaxillaryModerate => "81",
                MandibularModerate => "82",
                MaxillarySevere => "83",
                MandibularSevere => "84",
                IncomeHasBeenVerified => "85",
                PersonHasBeenInterviewed => "86",
                RentHasBeenVerified => "87",
                EmployerHasBeenVerified => "88",
                PositionHasBeenVerified => "89",
                InquiryHasBeenVerified => "90",
                OutstandingJudgments => "91",
                DeclaredBankruptcyInPast7Years => "92",
                ForeclosureOrDeedInLieuInPast7Years => "93",
                PartyToLawsuit => "94",
                Code95 => "95",
                CurrentlyDelinquentOrInDefault => "96",
                Code97 => "97",
                PartOfDownPaymentBorrowed => "98",
                CoMakerOrEndorserOnANote => "99",
                LiabilityCoverageWillTransfer => "A0",
                MostLikelyBuyerIsOtherPersonOrEntity => "A1",
                PotentialFinancingIsFannieMae => "A2",
                SuppressPaperEndorsement => "A3",
                DoNotSuppressPaperEndorsement => "A4",
                Escrow => "A5",
                TeachingMinor => "A6",
                SubServicerSubmitted => "A7",
                FirstMortgage => "A8",
                SecondMortgage => "A9",
                Amputation => "AA",
                AddressSkipBegin => "AB",
                AddressCorrected => "AC",
                AutomaticDrillTimeCalculated => "AD",
                AutomaticEdgingTimeCalculated => "AE",
                AutomaticallySelect => "AF",
                AcceptingFamilyMembers => "AFM",
                Agitated => "AG",
                AutomaticallySearchAndList => "AH",
                AddressIncorrect => "AI",
                Assumable => "AJ",
                PotentialFinancingIsCash => "AK",
                AmbulationLimitations => "AL",
                PotentialFinancingIsOutsideLender => "AM",
                AddressIncomplete => "AN",
                AcceptCertificationWithoutChanges => "AO",
                AlleyIsPublic => "AP",
                PotentialFinancingIsFederalHousingAdministration => "AQ",
                AddressSkipResolved => "AR",
                AddressSkipExhaust => "AS",
                AcceptStatementOfLimitingConditionsWithoutChanges => "AT",
                AutomaticUndersideTimeCalculated => "AU",
                AvailableNotUsed => "AV",
                AcceptCertificationWithChanges => "AW",
                AcceptStatementOfLimitingConditionsWithChanges => "AX",
                AdjacentTrackOccupied => "AY",
                PotentialFinancingIsVeteransAffairs => "AZ",
                UninsuredMotoristCoverageWillTransfer => "B0",
                MortgageInForeclosure => "B1",
                CodeB2 => "B2",
                PotentialFinancingIsContractForDeed => "B3",
                OnlyTheExteriorHasBeenInspected => "B4",
                RealEstateOwnedPropertyOrForeclosureProperty => "B5",
                NumberOfComparableListingsIsNormal => "B6",
                NumberOfComparableListingsIsAnOversupply => "B7",
                NumberOfComparableListingsIsAShortage => "B8",
                PropertyManagementExpensesOutstanding => "B9",
                BorrowerLetterAttempt => "BA",
                BuildingOrMobileHomeIsInACoastalBarrierResourcesArea => "BB",
                BorrowerTelephoneContact => "BC",
                BusinessPending => "BD",
                BorrowerLetterContact => "BE",
                MarketableSecuritiesValuedAtMarket => "BF",
                AppropriateImprovementConditionExists => "BG",
                NameUnknownToLocalAuthorities => "BH",
                NoManufacturingDoneOnPremises => "BI",
                Occasional => "BJ",
                OfficerOrOwnerInOtherBusinesses => "BK",
                CodeBL => "BL",
                Old => "BM",
                OperatesOnPartTimeBasis => "BN",
                ParentFinancialStatementUsed => "BO",
                BorrowerPaymentReceived => "BP",
                BeneficiaryIsPartiallyDependent => "BPD",
                ProductInformationAvailable => "BQ",
                CodeBR => "BR",
                RevenueDerivedFromCommissions => "BS",
                BorrowerTelephoneAttempt => "BT",
                BeneficiaryIsTotallyDependent => "BTD",
                RevenueDerivedFromDonations => "BU",
                RevenueDerivedFromFees => "BV",
                RevenueDerivedFromGrants => "BW",
                RevenueDerivedFromTaxes => "BX",
                SprinklerEquipped => "BY",
                StatementRequestedFromGovernmentRegistry => "BZ",
                CollisionCoverageWillTransfer => "C0",
                AdvancesFromPropertyManagementExpensesOutstanding => "C1",
                FinalDemandLetterSent => "C2",
                LenderRequestForAssistance => "C3",
                MortgageHasLenderPurchasedMortgageInsurance => "C4",
                InsufficientFunds => "C5",
                CreditEnhancedMortgage => "C6",
                CorporateAppointment => "C7",
                SpecialServicingRequired => "C8",
                ClientSpecificallyRequestedConsiderationOfSpecialFinancingOrAnAssumableLoan => {
                    "C9"
                }
                CaneRequired => "CA",
                CompleteBedrest => "CB",
                CollectionCardWasLeft => "CC",
                CallToDirectoryAssistanceForReferenceTelephone => "CD",
                CoSignerTelephoneAttempt => "CE",
                CoSignerTelephoneContact => "CF",
                ClaimIsFraudulent => "CFD",
                CoSignerDelinquencyLetterSent => "CG",
                CoSignerFinalDemandLetterSent => "CH",
                CallToDirectoryAssistanceForCoSignerTelephone => "CI",
                ValidBorrowerAddressOrPhoneAttemptWithPreviousHolder => "CJ",
                Convertible => "CK",
                ClaimantHadAPreExistingInjury => "CL",
                Comatose => "CM",
                CommonElementsAreLeasedToOrByTheHomeOwnersAssociation => "CN",
                CumulativeInjury => "CNJ",
                Contracture => "CO",
                CasePending => "CP",
                Callable => "CQ",
                CrutchesRequired => "CR",
                CommunityParticipatesInNationalFloodInsuranceProgram => "CS",
                CommonElementsAreCompleted => "CT",
                CurbAndGutterArePublic => "CU",
                Cooperative => "CV",
                CoolingWaterIsLow => "CW",
                CertificationStatus => "CX",
                CarSpacesAreAdequate => "CY",
                CarSpacesAreInadequate => "CZ",
                ComprehensiveCoverageWillTransfer => "D0",
                IssueCheckPayableToBorrowerAndReturnToServicer => "D1",
                IssueCheckPayableToServicerAndReturnToServicer => "D2",
                IssueCheckPayableToBorrowerAndSendToBorrower => "D3",
                IssueCheckPayableToServicerOrBorrowerAndReturnToServicer => "D4",
                IssueCheckPayableToOtherPayee => "D5",
                Positive => "D6",
                Negative => "D7",
                TaxesAreTypicalForTheAreaAndPriceRange => "D8",
                ImprovementConformsToZoningRegulations => "D9",
                CallToDirectoryAssistanceForBorrowerTelephone => "DA",
                DefermentOrForbearanceBegin => "DB",
                Declined => "DC",
                BorrowerFurnishedDemographicData => "DD",
                DefermentOrForbearanceEnd => "DE",
                FundsAvailableForUnsecuredCreditors => "DF",
                DeductibleAmountFullyRecovered => "DFR",
                DynamicBrakesAreOut => "DG",
                DebtorHasBeenDomiciled => "DH",
                Disoriented => "DI",
                DynamicBrakesAreOperational => "DJ",
                ConstructionWarranty => "DK",
                ConstructionWarrantyTransferable => "DL",
                MaintenanceDrugUnderClientsBenefitPlan => "DM",
                PaymentReducedBecauseMaximumAllowableCostExceeded => "DN",
                DeductibleAmountNotFullyRecovered => "DNR",
                BenefitsTerminatedPriorToServiceDate => "DO",
                Depressed => "DP",
                DrugPartOfFormularyDataBase => "DQ",
                SubjectNotEngagedInBusiness => "DR",
                AllDoorSealsAreIntact => "DS",
                FilingFeeAttached => "DT",
                SubjectNotEngagedInBusinessAtRequestedAddress => "DU",
                Suspended => "DV",
                Total => "DW",
                UnableToRespond => "DX",
                DyspneaWithMinimalExertion => "DY",
                UsesOwnFacilities => "DZ",
                FiguresAreTotal => "E0",
                FixedAssetBreakdownUndisclosed => "E1",
                ForTheFiscalYear => "E2",
                ForThePeriod => "E3",
                FormedByConsolidation => "E4",
                FormedByMerger => "E5",
                PriorBankruptcyCaseFiledInLast6Years => "E6",
                DebtorIsNotRepresentedByAnAttorney => "E7",
                APendingCaseHasBeenFiled => "E8",
                GuaranteedByParentCompany => "E9",
                HasAuthorityForAllPurchases => "EA",
                HasAuthorityToPurchaseSupplies => "EB",
                EquipmentCertified => "EC",
                HasBusinessInterruptionInsurance => "ED",
                HasClassOfStock => "EE",
                HasExtendedCoverageInsurance => "EF",
                HasFireInsurance => "EG",
                HasJointAuthority => "EH",
                HasLifeInsurance => "EI",
                ExistenceOfPreliminaryFloodDetermination => "EJ",
                ExistenceOfCommunityParticipationInTheNationalFloodInsurance => "EK",
                EnduranceLimitations => "EL",
                HasMarriageContract => "EM",
                ElectricityOn => "EN",
                EquipmentIsOverhauled => "EO",
                ExercisesPrescribed => "EP",
                HasNoParValue => "EQ",
                EngineStartUpPerformedWithNoProblemsReported => "ER",
                EngineStartUpPerformedWithProblemsReported => "ES",
                ElectricalControlSystemShutDown => "ET",
                HasOtherInsurance => "EU",
                HasParValue => "EV",
                HasSoleAuthority => "EW",
                Excellent => "EX",
                HasVotingRights => "EY",
                HeadingAddressInRegisteredOfficeOnly => "EZ",
                HighLevel => "F0",
                HomeworkersEmployed => "F1",
                InSubscriberShares => "F2",
                Inactive => "F3",
                Incomplete => "F4",
                IncorporationDetailsRequested => "F5",
                IncreaseOrUp => "F6",
                InformationCannotBeProvidedAtThisTime => "F7",
                InformationInDate => "F8",
                InformationRequiresInvestigation => "F9",
                ActionsHasASignificantEnvironmentalEffect => "FA",
                ApplicationIncludesCompleteSystem => "FB",
                AntennaIsMountedOnAStructureWithAnExistingAntenna => "FC",
                NoticeOfConstructionOrAlterationHasBeenFiled => "FD",
                ApplicantWantsToMonitorFrequency => "FE",
                ApplicantHasBeenDeniedGovernmentBenefitsDueToUseOfDrugs => "FF",
                ApplicationIsCertified => "FG",
                ApplicationIsForOtherThanANewStation => "FH",
                FeeRequired => "FI",
                FloodStatus => "FJ",
                FloodInsuranceRequired => "FK",
                CodeFL => "FL",
                CodeFM => "FM",
                NotTooHighLevel => "FN",
                Forgetful => "FO",
                FloodCertificationWithLifeOfLoan => "FP",
                StreetMaintenanceIsPublic => "FQ",
                Fair => "FR",
                NotYetRegistered => "FS",
                ObligedToFileBalanceSheet => "FT",
                OfficialConfirmationReceived => "FU",
                OldButWellKept => "FV",
                OldEstablishedBusiness => "FW",
                OperatedAtBreakEven => "FX",
                OperatesAsAgent => "FY",
                FloodZoneStatus => "FZ",
                OutOfBusiness => "G0",
                OutstandingClaims => "G1",
                GasOn => "G2",
                HazardousMaterialsAreUsedOrProduced => "G3",
                GeneticallyEngineeredOrganismsAreUsedOrProduced => "G4",
                ThisIsAGroupProposal => "G5",
                HistoricalSitesAreAffected => "G6",
                FacilitiesAreProperlyAccreditedOrAuthorized => "G7",
                ProprietaryOrPrivilegedInformationWillBeContainedInTheApplication => "G8",
                ThisProjectHasAnActualOrPotentialImpactOnTheEnvironment => "G9",
                GrowthRateIsFullyDeveloped => "GA",
                OutstandingSocialSecurityClaims => "GB",
                CodeGC => "GC",
                ProductDemonstrationInEffect => "GD",
                OwnershipAcknowledgedInSignedStatement => "GE",
                OwnershipAcknowledgedVerbally => "GF",
                OwnershipNotAcknowledged => "GG",
                OwnsNoRealEstate => "GH",
                OwnsRealEstateButDetailsNotAvailable => "GI",
                PreparedFromBooksWithoutAudit => "GJ",
                PreparedFromStatementByAccountant => "GK",
                ProfitsPaidToGroup => "GL",
                ShelfSetToManufacturersStandard => "GM",
                PubliclyTraded => "GN",
                Good => "GO",
                PurchaseAuthorityIsQualified => "GP",
                PurchasesOnFloorPlan => "GQ",
                ShelfSetToRetailersSchematic => "GR",
                PurchasesOnLetterOfCredit => "GS",
                RealEstateCheckIsNecessary => "GT",
                RecordOfPreferentialClaims => "GU",
                RegisteredAddressIsSameAsBusinessAddress => "GV",
                RelativesHelpInBusiness => "GW",
                Satisfactory => "GX",
                SeasonsAreSteady => "GY",
                Secured => "GZ",
                OrganizationCertifiesComplianceWithFederalLobbyingRegulations => "H0",
                ProjectInvolvesInternationalCoOperativeActivities => "H1",
                HumanAnatomicalSubstancesAreUsed => "H2",
                HandicapFacilitiesAreAvailable => "H3",
                LobbyingActivitiesHaveBeenConductedRegardingTheProposal => "H4",
                OrganizationCertifiesComplianceWithTheDrugFreeWorkplaceAct => "H5",
                OrganizationCertifiesComplianceWithTheCodeOfFederalRegulationsRegardingResearchMisconduct => {
                    "H6"
                }
                OrganizationProvidesASmokeFreeWorkplace => "H7",
                OrganizationCertifiesComplianceWithFederalDiscriminationRegulations => {
                    "H8"
                }
                CodeH9 => "H9",
                WellMaintained => "HA",
                InterestRateBuydown => "HB",
                HeatingAndCoolingForTheIndividualUnitsSeparatelyMetered => "HC",
                HighDischarge => "HD",
                HighEngineWaterPressure => "HE",
                InterestOnly => "HF",
                GraduatedPayment => "HG",
                PrincipalBalanceExceedsMaximumNegativeAmortization => "HH",
                LastChange => "HI",
                LiabilityReleased => "HJ",
                LiabilityNotReleased => "HK",
                HearingLimitations => "HL",
                LiabilityDeterminedByNoteHolder => "HM",
                AfterConversion => "HN",
                Hostile => "HO",
                AfterModification => "HP",
                Balloon => "HQ",
                CapitalizedMortgage => "HR",
                FederalWagesInEffect => "HS",
                CodeHT => "HT",
                CodeHU => "HU",
                CodeHV => "HV",
                CodeHW => "HW",
                EmployeeIsIneligibleToWork => "HX",
                MetesAndBounds => "HY",
                CodeHZ => "HZ",
                BasedOnOperatingData => "I0",
                UsesOutsideServices => "I1",
                VeryHighLevel => "I2",
                VerySmall => "I3",
                VoluntaryBankruptcy => "I4",
                WellBalanced => "I5",
                WellRegardedInBusinessCircles => "I6",
                OrganizationHasDelinquentFederalDebts => "I7",
                OrganizationHasBeenPlacedOnTheFederalDebarmentAndSuspensionList => "I8",
                NoShowIndicator => "I9",
                InterestPaidInAdvance => "IA",
                InterestPaidInArrears => "IB",
                InterestCarryover => "IC",
                SellsDirectly => "ID",
                SellsWithAgents => "IE",
                SellsWithStorage => "IF",
                Small => "IG",
                IndependentAtHome => "IH",
                SomeIncrease => "II",
                SomewhatDecliningTendency => "IJ",
                StartedSomeTimeAgo => "IK",
                IndustryLocation => "IL",
                Sufficient => "IM",
                Indifferent => "IN",
                TerminationDateSet => "IO",
                InjuryOccurredOnEmployersPremises => "IP",
                TermsIncludeLumpSumPayments => "IQ",
                TermsIncludeProgressPayments => "IR",
                TermsOnCostPlusBasis => "IS",
                TermsOnFixedFeeBasis => "IT",
                TradeStyleRegistered => "IU",
                TradingAddressOfSoleProprietor => "IV",
                UnchangedSituation => "IW",
                Undetermined => "IX",
                Unsatisfactory => "IY",
                Unsecured => "IZ",
                QualifiesAsAnEnergyEfficientHome => "J0",
                CodeJ1 => "J1",
                RateNegotiated => "J2",
                UnderPenaltyOfPerjuryTheInformationIsTrueAndCorrect => "J3",
                ProjectRequiresInterGovernmentReviewForActivitiesThatAffectStateOrLocalGovernmentOrPossibleNationalSecurityImplications => {
                    "J4"
                }
                FilingOnBehalfOfDebtorIsAuthorized => "J5",
                DebtorUnderstandsTheReliefAvailableUnderEachBankruptcyChapter => "J6",
                AttorneyDeclaresThatDebtorHasBeenInformed => "J7",
                AttorneyHasExplainedTheReliefAvailableUnderEachBankruptcyChapter => "J8",
                ThereHasBeenATransferOfAClaimAgainstTheDebtorByOrToAnyPetitioner => "J9",
                ThirdPartyOriginated => "JA",
                ExistingConstruction => "JB",
                OtherLien => "JC",
                JointCoverageApplies => "JCA",
                SubjectLien => "JD",
                CodeJE => "JE",
                PrimaryUnderwritingSystem => "JF",
                NonNewPartsUsed => "JG",
                PledgedLoan => "JH",
                SecurityDelivery => "JI",
                SecondaryUnderwritingSystem => "JJ",
                DistributionIsStopped => "JK",
                SentenceWasSuspended => "JL",
                VeryNegativeInformationExists => "JM",
                PaymentNotesExist => "JN",
                Immigrated => "JO",
                AuditedWithQualifications => "JP",
                Audited => "JQ",
                TemporarilyClosed => "JR",
                Partial => "JS",
                TelephoneNumberIsUnpublished => "JT",
                TelephoneNumberIsNotInService => "JU",
                NegativeInformationExistsForTheGroup => "JV",
                TheMoreImportantItemsAreOnlyIncluded => "JW",
                InterestOwnedByAffiliatedCompany => "JX",
                InterestOwnedBySubjectOfInquiry => "JY",
                QualifiesAsAGovernmentApprovedCondominiumOrProject => "JZ",
                AccountReceivablesBreakdownUndisclosed => "K0",
                AdditionalRecordItemsAvailable => "K1",
                AddressIsQualified => "K2",
                AllPaidInOrIssued => "K3",
                AppearsHigh => "K4",
                AppearsNotToGuaranteeSufficientCoverage => "K5",
                AppearsSufficientlyHigh => "K6",
                AppearsToIndicateAStrainedSituation => "K7",
                BanksWithMainNationalBanks => "K8",
                BillsPaidFromBranchOffice => "K9",
                BillsPaidFromDivisionOffice => "KA",
                BillsPaidFromHeadquartersOffice => "KB",
                BondInformationAvailable => "KC",
                ChangedAccountingDate => "KD",
                Clear => "KE",
                ClearDecliningTendency => "KF",
                ClearIncrease => "KG",
                Cluttered => "KH",
                CompanyHasNoOtherLocations => "KI",
                CompanyIsBranchOfForeignEntity => "KJ",
                CompanyIsPerpetual => "KK",
                CompanyIsTaxExempt => "KL",
                ComparedToSamePeriodLastYear => "KM",
                ConductedAtALoss => "KN",
                CodeKO => "KO",
                Large => "KP",
                LetterOfAgreementPresent => "KQ",
                LetterOfAgreementWithdrawn => "KR",
                LetterOfLiabilityPresent => "KS",
                LetterOfLiabilityWithdrawn => "KT",
                LocationInquiredUponIsABranch => "KU",
                CodeKV => "KV",
                LocationInquiredUponIsAHeadquarters => "KW",
                LocationIsForeign => "KX",
                MeansExhausted => "KY",
                MediumToLarge => "KZ",
                ImmunizationMandatedByStateLawForEmployment => "L0",
                GeneralStandardOf20DegreeOr5DiopterSphereOrCylinderChangeMet => "L1",
                ReplacementDueToLossOrTheft => "L2",
                ReplacementDueToBreakageOrDamage => "L3",
                ReplacementDueToPatientPreference => "L4",
                ReplacementDueToMedicalReason => "L5",
                LandContract => "L6",
                AccountCurrent => "L7",
                VeryGood => "L8",
                Restored => "L9",
                LetterOfMapAmendmentOrLetterOfMapRevision => "LA",
                LegallyBlind => "LB",
                ProducerOfGoods => "LC",
                DrawbackIndicator => "LD",
                Lethargic => "LE",
                CustomsRuleApplicable => "LF",
                ExportedPursuantToLawRegulationOrToCancelCustomsBond => "LG",
                CountryOfOriginInformationAppliesToAllPriorShipments => "LH",
                PriceEstimated => "LI",
                CodeLJ => "LJ",
                KitForm => "LK",
                LockoutEffective => "LL",
                LetterOfAppointment => "LM",
                FacilitysEmergencyResponsePlanIncludesSpecificActionsToBeTakenInResponseToAccidentalReleasesOfRegulatedSubstances => {
                    "LN"
                }
                LocomotiveIsIsolated => "LO",
                LowEngineOilPressure => "LP",
                FacilityHadASafetyInspection => "LQ",
                LocomotiveEngineIsRunning => "LR",
                LesseeSignatureOnFile => "LS",
                ListSpecialtyInDirectory => "LSD",
                LenderOrServicerTransfer => "LT",
                EvidenceOfDampness => "LU",
                EvidenceOfTermites => "LV",
                EvidenceOfStructureSettlement => "LW",
                SalvageMoved => "LX",
                AddressIsFormerLocation => "LY",
                AddressIsOccupiedByOthers => "LZ",
                CodeM0 => "M0",
                DataCorrected => "M1",
                ServicerRecordSelected => "M2",
                LengthOfServiceIs3MonthsOrLess => "M3",
                CodeM4 => "M4",
                LengthOfServiceIs1YearThrough5Years => "M5",
                LengthOfServiceIsMoreThan5Years => "M6",
                CataractOrCornealTransplantOrOtherConditionSuchAsKeratoconus => "M7",
                VisionInWorseEyeCorrectableTo2040OrBetterWithRegularLenses => "M8",
                ContactLensesCorrectedVisionInWorseEyeTo2040OrBetter => "M9",
                MajorAlarmFlagReported => "MA",
                EquipmentHasModifiedConfiguration => "MB",
                OtherMentalCondition => "MC",
                MarketingTimeIs4To6Months => "MD",
                TrendReversed => "ME",
                MicroprocessorFault => "MF",
                MortgageInsuranceApplicationIncluded => "MG",
                MortgageCreditReportIncluded => "MH",
                ResidentialLoanApplicationIncluded => "MI",
                RealEstateInformationReportIncluded => "MJ",
                RealEstateTitleEvidenceIncluded => "MK",
                ManuallySearchAndList => "ML",
                CodeMM => "MM",
                CodeMN => "MN",
                CooperativeProjectIncludesOrOwnsAnyCommercialUnits => "MO",
                UnitsAndProjectAmenitiesAreComplete => "MP",
                EligibleTrust => "MQ",
                ResaleProperty => "MR",
                MiscellaneousSkipTraceAttempt => "MS",
                PhotosMatchDescription => "MT",
                PhotosShowNegativeInfluence => "MU",
                ExcludeFromMonthlyDebt => "MV",
                ThisBrokerMarketAnalysisIsBeingCompletedForHomeMarketAssistance => "MW",
                ThisBrokerMarketAnalysisIsBeingCompletedForHomesaleOrBuyout => "MX",
                ProjectTypeIsSingleFamily => "MY",
                ProjectTypeIsOther => "MZ",
                HospitalizedOverNight => "N0",
                CodeN1 => "N1",
                ClaimInvolvesRestrictedWorkActivityWithoutDaysAwayFromWork => "N2",
                StrikeOrLockoutInProgress => "N3",
                ShutdownOrLayoffInProgress => "N4",
                WorkIsSeasonal => "N5",
                NaturalDisasterOrAdverseWeatherAffectingWork => "N6",
                ShorterWorkSchedulesOrFewerPayPeriodsThanUsualInEffect => "N7",
                LongerWorkSchedulesOrMorePayPeriodsThanUsualInEffect => "N8",
                OtherFactorsAffectClaimFrequency => "N9",
                NoUserAvailable => "NA",
                NeighborhoodPredominatelySingleFamilyDwellings => "NB",
                ItemHasDirectNumericalControl => "NC",
                NoteHolderPermissionRequired => "ND",
                NoDeductibleProgram => "NDP",
                Notarized => "NE",
                NewConstruction => "NF",
                MortgagePointsAreCustomarilyPaidBySeller => "NG",
                NoNationalFloodInsuranceProgramMap => "NH",
                SeasonedMortgage => "NI",
                IssuesAreAnticipatedThatWouldAffectTheAbilityToSecureFinancingOfTheSubjectProperty => {
                    "NJ"
                }
                Citizenship => "NK",
                GroupDisabilityInsuranceMandatory => "NL",
                RetailOrigination => "NM",
                CodeNN => "NN",
                ArmsLengthTransaction => "NO",
                CertificationOfANonAttorneyBankruptcyPetitionPreparer => "NP",
                EligibleForTheFannieMaeNeighborsProgram => "NQ",
                NoRestrictions => "NR",
                CodeNS => "NS",
                LodgingProvided => "NT",
                NotUsed => "NU",
                ContractLabor => "NV",
                BonusesPaid => "NW",
                MinorsEmployed => "NX",
                MeetsRequirementsForFannieMaeCommunitySecondsProgram => "NY",
                PurchaseIsAResultOfCurrentEmployerSponsoredRelocation => "NZ",
                TeachingMajor => "O0",
                MultipleUnspecifiedInstances => "O1",
                HiresPartTimeEmployeesAsNeeded => "O2",
                MexicanRequest => "O3",
                RiskManagementPlanRequiresPredictiveFiling => "O4",
                SanitizedCopy => "O5",
                CodeO6 => "O6",
                ToxicChemicalClaimedAsTradeSecret => "O7",
                UnderControlOfReportingFacilityOrParentCompany => "O8",
                WeatherConditionsNotKnown => "O9",
                SellerProvidedBelowMarketSecondaryFinancing => "OA",
                FixedSite => "OB",
                MobileFacility => "OC",
                TransferAuthorized => "OD",
                OccupationalDisease => "ODZ",
                TransferComplete => "OE",
                CommercialDriversLicenseVerified => "OF",
                ResponsibilityAccepted => "OG",
                WaterbodyInvolved => "OH",
                ChargesPending => "OI",
                DriverHasProperLicenseClass => "OJ",
                DriverCompliantWithLicenseRestrictions => "OK",
                OtherLimitation => "OL",
                DriverHasCommercialDriversLicense => "OM",
                DriverHasMedicalWaiver => "ON",
                OwnOtherFederalHousingAdministrationProperty => "OO",
                OutOfRangeProductTemperature => "OP",
                PhotographsTaken => "OQ",
                OtherRestrictions => "OR",
                OutOfService => "OS",
                Oriented => "OT",
                PoliceOfficerAtScene => "OU",
                Overridden => "OV",
                Proposed => "OW",
                RatingIsAffected => "OX",
                CodeOY => "OY",
                LiabilityIsContingentOrHasACoSigner => "OZ",
                TerminalDegree => "P0",
                PatientWasDischargedFromTheFirstFacility => "P1",
                PatientWasAdmittedToTheSecondFacility => "P2",
                PropertyHasAFamilyRoomOrDen => "P3",
                PropertyHasCentralAirConditioning => "P4",
                PropertyTypicalOfNeighborhood => "P5",
                PropertyDeferredMaintenanceTypicalOfNeighborhood => "P6",
                AcceptingExistingPatients => "P7",
                AcceptingNewPatients => "P8",
                PropertyIntendedToBeOccupiedAsPrimaryResidence => "P9",
                Paralysis => "PA",
                PhoneSkipBegin => "PB",
                PlanIsAttached => "PC",
                PhoneSkipResolved => "PD",
                PhoneSkipExhaust => "PE",
                PaidOutsideOfClosing => "PF",
                PreviouslyFailedBoardCertification => "PFB",
                ProjectIsSubjectToGroundRent => "PG",
                Prepayable => "PH",
                Program => "PI",
                ProviderIsParticipating => "PJ",
                PreliminaryFloodDetermination => "PK",
                ProviderCertificationInTheTaxonomyHasBeenVerified => "PL",
                ProjectAndServicesBudgetIsMaintained => "PM",
                AtypicalPhysicalCondition => "PN",
                PersonalPropertyOnsite => "PO",
                PropertyPreviouslyWinterized => "PP",
                LiabilityWillBeResubordinatedToTheLoanUponClosing => "PQ",
                Poor => "PR",
                PriorDamage => "PRD",
                PublicationIsIncludedInSharing => "PS",
                ProjectIsComplete => "PT",
                NotPaid => "PU",
                PropertyVacant05Percent => "PV",
                PartialWeightBearing => "PW",
                PaidByBorrowerBeforeClosing => "PX",
                PropertyForSale => "PY",
                PropertyVacantOver5Percent => "PZ",
                Veteran => "Q0",
                ExportProduct => "Q1",
                CodeQ2 => "Q2",
                USGoodsReturned => "Q3",
                CandidateForUSCustomsServiceProtest => "Q4",
                DomesticProduct => "Q5",
                PriorApprovalLetterAndOfficialOrdersOnFile => "Q6",
                ImportersSubstantiatingStatementAndContractAreOnFile => "Q7",
                InternationalTransportMovement => "Q8",
                PieceCountShouldBeIncludedInTheTotalPackingListQuantity => "Q9",
                ShipmentShouldBeHeldAtThePort => "QA",
                MultipleStatesOfOriginForThisItem => "QB",
                MultipleCountriesOfOriginForThisItem => "QC",
                LetterOfCreditRestrictedToASpecificBank => "QD",
                LetterOfCreditPermitsTransshipment => "QE",
                LetterOfCreditCoversPartialShipments => "QF",
                DutiableItem => "QG",
                AmountsShouldBeProRatedAcrossLineItems => "QH",
                CodeQI => "QI",
                VisaRequiredForThisItem => "QJ",
                ItemSubjectToQuotas => "QK",
                CodeQL => "QL",
                ItemIsASet => "QM",
                ItemIsAnEnsemble => "QN",
                ItemIsAMetalItem => "QO",
                ItemIsAMachinePart => "QP",
                ItemIsAHazardousItem => "QQ",
                CodeQR => "QR",
                QuantityToBeImportedHasBeenApprovedByTheNecessaryAgencies => "QS",
                FilingDataIsToBeWithheldFromPublicInspection => "QT",
                PropertyTypeCooperative => "QU",
                PaidByBorrowerAtClosing => "QV",
                PaidByOtherAtOrBeforeClosing => "QW",
                TreatedAsAReductionToIncome => "QX",
                CodeQY => "QY",
                CodeQZ => "QZ",
                ExemptFromPublicRecordsLaw => "R0",
                DebtorHoldsClaimToRealProperty => "R1",
                EntityClaimsToHoldASecuredInterest => "R2",
                DebtorHasPropertyOfTheTypeSpecified => "R3",
                DebtorElectsTheStateExemption => "R4",
                DebtorElectsTheFederalExemption => "R5",
                CoDebtorMayBeJointlyLiable => "R6",
                ClaimIsContingent => "R7",
                ClaimIsUnliquidated => "R8",
                ClaimIsDisputed => "R9",
                ReferenceTelephoneAttempt => "RA",
                DebtorHasNoCreditorsHoldingUnsecuredPriorityClaims => "RB",
                ReferenceTelephoneContact => "RC",
                RentalCarArranged => "RCA",
                RentDelinquent => "RD",
                ClaimIsSubjectToSetoff => "RE",
                DebtorHasNoExecutoryContractsOrUnexpiredLeases => "RF",
                LeaseIsForNonresidentialRealProperty => "RG",
                DebtorHasNoCoDebtors => "RH",
                DebtorIsMarried => "RI",
                DebtorsSpouseMaintainsASeparateHousehold => "RJ",
                RealEstateTaxesAreIncluded => "RK",
                PropertyInsuranceIsIncluded => "RL",
                DebtorHasNoCreditorsHoldingSecuredClaims => "RM",
                RentControl => "RN",
                EquipmentIsRebuilt => "RO",
                IndividualInjuredInPerformanceOfDuty => "RP",
                IndividualInjuredByThirdParty => "RQ",
                QualityOfManagementAndItsEnforcementOfRulesAndRegulationsBasedOnGeneralAppearances => {
                    "RR"
                }
                PayContinued => "RS",
                SickLeaveTaken => "RT",
                SignatureOnFile => "RU",
                LowRefrigerantCapacityShutdown => "RV",
                RecentDefrost => "RW",
                RatedHorsepowerCanBeProduced => "RX",
                ForeignMilitarySale => "RY",
                WaiverOfPriorNotice => "RZ",
                AlternateCertificationProgramParticipant => "S0",
                ServicesProvidedAtTheSecondFacilityWereAvailableAtTheFirstFacility => {
                    "S1"
                }
                UnderTreatment => "S2",
                FirstTimeVacant => "S3",
                AdverseEasement => "S4",
                DisclosureIndicator => "S5",
                AtypicalOffSiteImprovements => "S6",
                ToxicSubstances => "S7",
                AdverseEncroachment => "S8",
                AtypicalFunctionalCondition => "S9",
                SubjectPropertyIsCurrentlyListed => "SA",
                DebtorIsASmallBusinessAsDefinedIn11USCSection101 => "SB",
                SpecialServicesAreMobileHomeOnly => "SC",
                SpecialServicesAreLeaseholdOrMobileHomeOrBoth => "SD",
                CodeSE => "SE",
                SensorFault => "SF",
                StreetLightsArePublic => "SG",
                SpecialServicesAreLeaseholdOrSubleaseholdOrBoth => "SH",
                HazardousWaste => "SI",
                PestInfestation => "SJ",
                RoadMaintenanceRequired => "SK",
                SpeechLimitations => "SL",
                CurrentlyServingInMilitary => "SM",
                MajorBaseSupport => "SN",
                CriticalSupportLevelMet => "SO",
                StreetIsPublic => "SP",
                SpecialtyIsPrimary => "SPP",
                SpecialtyIsSecondary => "SPS",
                LocalWagesInEffect => "SQ",
                FederalWorkerDisplacement => "SR",
                AdverseZoning => "SS",
                NewServicesRequested => "ST",
                ContinuedServicesRequested => "SU",
                SubrogationOpen => "SUB",
                MajorCorporationHighTech => "SV",
                SidewalkIsPublic => "SW",
                CollectiveBargainingAgreementSentByMail => "SX",
                CollectiveBargainingAgreementSentByFacsimile => "SY",
                Contract => "SZ",
                UnderContract => "T0",
                RoadTestPerformedWithNoProblemsReported => "T1",
                RoadTestPerformedWithProblemsReported => "T2",
                TiresBrandMatch => "T3",
                RealEstateTaxesAreCurrent => "T4",
                HazardInsuranceIsCurrent => "T5",
                TerminateGuarantee => "T6",
                AtypicalExternalCondition => "T7",
                CodeT8 => "T8",
                UtilitiesInadequate => "T9",
                CollectiveBargainingAgreementSentByElectronicBulletinBoard => "TA",
                DebtorHasNoCreditorsHoldingUnsecuredNonpriorityClaims => "TB",
                TransportViaCargoAircraft => "TC",
                AnnualLeaveTaken => "TD",
                ItemIsSpecialTestEquipment => "TE",
                OperatesAsRepresentativeForOthers => "TF",
                ClaimInvolvesWorkRelatedDeath => "TG",
                CodeTH => "TH",
                EmployeeHasNotRecoveredToReturnToWork => "TI",
                EmployeeHasRetired => "TJ",
                EmployeeHasResigned => "TK",
                EmployeeIsPermanentlyAndTotallyDisabled => "TL",
                TractionMotorIsCutOut => "TM",
                AtypicalQualityOfConstruction => "TN",
                TraumaticInjury => "TNJ",
                AtypicalRemodeling => "TO",
                TransportViaPassengerAircraft => "TP",
                AtypicalAdditions => "TQ",
                CodeTR => "TR",
                AdverseMarketingConditionsInSubjectPropertysNeighborhood => "TS",
                NeighborhoodWaterSourceIsPublic => "TT",
                NeighborhoodSewageTreatmentIsPublic => "TU",
                TelephoneNumberVerified => "TV",
                NeighborhoodStreetIsPublic => "TW",
                OtherMiscellaneousAdverseCharacteristics => "TX",
                SubjectPropertysStreetIsPublic => "TY",
                SubjectPropertysSewageTreatmentIsPublic => "TZ",
                Disability => "U0",
                MinimalChange => "U1",
                NeatAppearance => "U2",
                NetWorthComputedAfterExemptions => "U3",
                NetWorthConsiderablyHigher => "U4",
                NetWorthHigher => "U5",
                NoEmployees => "U6",
                NoEmployeesBusinessManagedByOwner => "U7",
                NoEmployeesBusinessManagedByPartners => "U8",
                NotOutOfBusiness => "U9",
                CodeUA => "UA",
                ConductedAtAProfit => "UB",
                ContingentDebtIndicated => "UC",
                Continue => "UD",
                ContractsObtainedByBid => "UE",
                ContractsObtainedByNegotiation => "UF",
                ConvertedToHoldingCompany => "UG",
                CrossClaimFiled => "UH",
                DecliningTendency => "UI",
                CodeUJ => "UJ",
                CodeUK => "UK",
                DownOrDeclineOrDecreased => "UL",
                EmployeesIncludeOfficers => "UM",
                Uncooperative => "UN",
                EmployeesIncludeOwners => "UO",
                EmployeesIncludePartners => "UP",
                EmployeesIncludeTemporaryWorkers => "UQ",
                EmployeesVaryAccordingToNeeds => "UR",
                Enclosed => "US",
                UpAsTolerated => "UT",
                CodeUU => "UU",
                FavorablePersonalReputation => "UV",
                FiguresAreAbbreviated => "UW",
                FiguresAreConvertedToAgencyFormat => "UX",
                FiguresAreIndividual => "UY",
                FiguresAreRestated => "UZ",
                UltimateParentCompanyFinancialStatementUsed => "V0",
                ValidBorrowerAddressOrPhoneAttemptWithSchoolAttended => "V1",
                LenderDeterminedBorrowerMovedOutOfState => "V2",
                LenderDeterminedBorrowerMovedBackIntoState => "V3",
                LenderDeterminedBorrowerIncarcerated => "V4",
                LenderDeterminedBorrowerNoLongerIncarcerated => "V5",
                Original => "V6",
                TrueAndExactCopy => "V7",
                SubjectPropertysWaterSourceIsPublic => "V8",
                PicturesRequired => "V9",
                IntercompanyRelationsExist => "VA",
                InventoryValuedAtLowerOfCostOrMarket => "VB",
                InventoryValuedAtOtherMethods => "VC",
                OperatesAsSoleAgent => "VD",
                WithoutPersonalJudgment => "VE",
                WorkIsSubcontracted => "VF",
                NotRegistered => "VG",
                ImmediateAttentionRequired => "VH",
                VehicleInspectionReportCompleted => "VI",
                MiddleToMedium => "VJ",
                RentControlLikely => "VK",
                Furnished => "VL",
                PriceRangeSingleFamilyOrPlannedUnitDevelopmentNotApplicable => "VM",
                PriceRangeCondominiumNotApplicable => "VN",
                PriceRangeTwoToFourFamilyNotApplicable => "VO",
                FinancialFiguresAreProjectedBasedOnSales => "VP",
                FinancialFiguresAreProjectedBasedOnEmployees => "VQ",
                ParentCompanyHasBankruptcy => "VR",
                HeadquartersHasBankruptcy => "VS",
                CommercialMotorVehicleWasInvolvedInThisConviction => "VT",
                VehicleWasDeclaredATotalLoss => "VTL",
                CommercialMotorVehicleWasCarryingHazardousMaterialsWhenTheOffenseWasCommitted => {
                    "VU"
                }
                PreparedFromInternalBookFigures => "VV",
                QuantityDeclined => "VW",
                QuantityDetailsUnknown => "VX",
                CodeVY => "VY",
                CodeVZ => "VZ",
                StatementIsOnATradingTrust => "W0",
                NewRegistration => "W1",
                MailingAddressChange => "W2",
                ResidenceAddressChange => "W3",
                NameChange => "W4",
                PartyEnrollmentChange => "W5",
                NeedsAbsenteeBallot => "W6",
                WouldLikeToBeElectionDayWorker => "W7",
                DuplicateRegistration => "W8",
                ForwardedApplication => "W9",
                WalkerRequired => "WA",
                WaterOn => "WB",
                ApplicationIncomplete => "WC",
                VehiclePlateSurrendered => "WD",
                WrittenNoticeToNoteHolder => "WE",
                WrittenNoticeToBorrower => "WF",
                WithinSpecifiedTimePeriod => "WG",
                WithinSpecifiedRange => "WH",
                InjuryWasWorkRelated => "WI",
                DealerPricingAuthorization => "WJ",
                SummaryLevel => "WK",
                DetailLevel => "WL",
                NonOccupantCoBorrower => "WM",
                CodeWN => "WN",
                EquipmentInWorkingOrder => "WO",
                ToBeWatched => "WP",
                UndeterminedOutOfBusinessStatus => "WQ",
                WheelchairRequired => "WR",
                BalanceSheetFiled => "WS",
                WinterizedTagObserved => "WT",
                MaterialSafetyDataSheet => "WU",
                AcceptsCreditCards => "WV",
                AllPurchasesMadeFromHeadquarters => "WW",
                Busy => "WX",
                Excessive => "WY",
                FairlyNew => "WZ",
                CodeX0 => "X0",
                GrossWeeklyAmountIsEstimated => "X1",
                WaitingPeriodDisabilityDaysAreNonConsecutive => "X2",
                CodeX3 => "X3",
                PermanentImpairmentPaidAtMinimum => "X4",
                EmployeesDeathIsAResultOfWorkInjuryOrIllness => "X5",
                EmployeesWrittenSocialSecurityNumberReleaseIsOnFile => "X6",
                EmployeesMedicalRecordsReleaseAuthorizationIsOnFile => "X7",
                EmployeeReturnedToWorkWithPreInjuryEmployer => "X8",
                CodeX9 => "X9",
                FiguresAreAverage => "XA",
                Imports => "XB",
                InProcessOfEstablishing => "XC",
                IntercompanyRelationsConsistOfEndorsements => "XD",
                IntercompanyRelationsConsistOfGuarantees => "XE",
                IntercompanyRelationsConsistOfLeasingArrangements => "XF",
                IntercompanyRelationsConsistOfSharingAccounting => "XG",
                IntercompanyRelationsConsistOfSharingFacilities => "XH",
                IntercompanyRelationsConsistOfSharingManagement => "XI",
                IntercompanyRelationsConsistOfSharingPersonnel => "XJ",
                CodeXK => "XK",
                CodeXL => "XL",
                InventoryValuedAtCompanysEstimates => "XM",
                InventoryValuedAtCost => "XN",
                CodeXO => "XO",
                JointOwnership => "XP",
                LeasesWithNoRentPayments => "XQ",
                LeasesWithOptionToBuy => "XR",
                LeasesWithTokenPayment => "XS",
                Limited => "XT",
                LocatedForSeveralYears => "XU",
                LocatedSinceOpening => "XV",
                Modern => "XW",
                NonExistent => "XX",
                OfficerOrOwnerInOtherBusinessesInTheSameField => "XY",
                OperatesAsADistributorForOthers => "XZ",
                InsuredCooperative => "Y0",
                WorkedInIndustryForSeveralYears => "Y1",
                AircraftOperation => "Y2",
                AllClassificationsOnPolicyAccountedFor => "Y3",
                BoardProvided => "Y4",
                CasualLabor => "Y5",
                CertificatesOnFileForAllSubcontractors => "Y6",
                CommissionsPaid => "Y7",
                ConditionOrTypeOfRecordsCauseAdditionalAuditTime => "Y8",
                DomesticWorkersEmployed => "Y9",
                OperatesFromResidence => "YA",
                OperatesUnderLicenseByOthers => "YB",
                RentsFromMonthToMonth => "YC",
                SemiModern => "YD",
                UnderConstruction => "YE",
                Unlimited => "YF",
                Used => "YG",
                Variable => "YH",
                HolderIsASubsidiaryOfReportingAgent => "YI",
                ContactIsUnchangedFromPreviousReport => "YJ",
                ReportWasFiledLastYearByThisAgent => "YK",
                PartyIsAuthorizedToDoBusinessInThisState => "YL",
                ClearDecrease => "YM",
                EmployeesTemporarilyLaidOff => "YN",
                EstablishedInTheIndustry => "YO",
                GlobalBusiness => "YP",
                InformationToBeFollowedUp => "YQ",
                KnownDetailsAreListed => "YR",
                LandIsRented => "YS",
                Low => "YT",
                PrimeCommercialArea => "YU",
                CodeYV => "YV",
                SlightlyHigher => "YW",
                SlightlyLower => "YX",
                Stagnant => "YY",
                TerritoryInformationIsAvailable => "YZ",
                SubcontractorsUsed => "Z0",
                InsuredIsASubcontractor => "Z1",
                InsuredHasMultipleEntries => "Z2",
                InsuredHasRetailOperations => "Z3",
                CodeZ4 => "Z4",
                OwnerOrOfficerInterviewed => "Z5",
                PremiumOvertimeExcluded => "Z6",
                CodeZ7 => "Z7",
                RecordsSatisfactoryForAudit => "Z8",
                RelativesEmployed => "Z9",
                CustomerConfigurationChangeIsRequired => "ZA",
                CodeZB => "ZB",
                CodeZC => "ZC",
                CodeZD => "ZD",
                RepairIsMissionEssential => "ZE",
                RepairIsSafetyEssential => "ZF",
                PeriodicMaintenanceIsRequired => "ZG",
                CodeZH => "ZH",
                ProgressIsInJeopardy => "ZI",
                EmployeesInjuryOrIllnessIsWorkRelated => "ZJ",
                FinalConfigurationChangeIsRequired => "ZK",
                FinalDeliveryToShopIsRequired => "ZL",
                FinalRequestorWorkforceWillAssist => "ZM",
                JobIsLevel2 => "ZN",
                PreliminaryConfigurationChangeIsRequired => "ZO",
                PreliminaryDeliveryToShopIsRequired => "ZP",
                PreliminaryRequestorWorkforceWillAssist => "ZQ",
                ConfigurationChangeIsAssociatedWithTimeMeter => "ZR",
                ShopHasLeadResponsibility => "ZS",
                EstimateIsDerivedFromJobTemplate => "ZT",
                RequestorHoldsTechnicalDocumentation => "ZU",
                ReplacementItem => "ZV",
                CodeZW => "ZW",
                NonConvertible => "ZX",
                CodeZY => "ZY",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ConditionIndicator> {
        use ConditionIndicator::*;
        match code {
            b"00" => Some(Requested),
            b"000" => Some(InProgress),
            b"0A" => Some(Code0A),
            b"0B" => Some(Code0B),
            b"0C" => Some(Code0C),
            b"0D" => {
                Some(
                    FacilitysEmergencyResponsePlanIncludesInformationOnEmergencyHealthCare,
                )
            }
            b"0E" => {
                Some(
                    FacilitysEmergencyResponsePlanIncludesProceduresForInformingPublicAndLocalAgenciesResponsibleForRespondingToAnAccidentalRelease,
                )
            }
            b"0F" => Some(FacilityHasACleanAirActTitleVOperatingPermit),
            b"0G" => Some(FacilityHasAWrittenEmergencyResponsePlan),
            b"0H" => Some(FacilityHasReportableAccidents),
            b"0I" => Some(Code0I),
            b"0J" => Some(Code0J),
            b"0K" => Some(Code0K),
            b"0L" => Some(Code0L),
            b"0M" => Some(OffsiteRespondersNotified),
            b"0N" => Some(PrecipitationPresent),
            b"0O" => Some(DisabledVeteran),
            b"0P" => {
                Some(
                    ServicerHasAdvancedFundsToPayForDelinquentTaxesOnNonEscrowedMortgage,
                )
            }
            b"0Q" => Some(PropertyHasFireInsuranceOnlyThatWasNotLenderPlaced),
            b"0R" => Some(ReportedButUnconfirmed),
            b"0S" => Some(HasSmokeAlarms),
            b"0T" => Some(OperatesAsAHoldingCompany),
            b"0U" => Some(Optimum),
            b"0V" => Some(Renewed),
            b"0W" => Some(HighestEducationalLevel),
            b"0X" => Some(PrincipalCertificate),
            b"0Y" => Some(InserviceEducationCompleted),
            b"0Z" => Some(MainAssignment),
            b"01" => Some(PatientWasAdmittedToAHospital),
            b"1A" => Some(PatientIsReceivingAntiFungalTherapy),
            b"1B" => Some(PropertyIsOccupiedByOwner),
            b"1C" => Some(PropertyIsOccupiedByTenant),
            b"1D" => Some(PropertyIsVacant),
            b"1E" => Some(LocationIsUrban),
            b"1F" => Some(LocationIsSuburban),
            b"1G" => Some(LocationIsRural),
            b"1H" => Some(Code1H),
            b"1I" => Some(Code1I),
            b"1J" => Some(Code1J),
            b"1K" => Some(GrowthRateIsRapid),
            b"1L" => Some(ClassILeft),
            b"1M" => Some(GrowthRateIsStable),
            b"1N" => Some(GrowthRateIsSlow),
            b"1O" => Some(PropertyValuesAreIncreasing),
            b"1P" => Some(PropertyValuesAreStable),
            b"1Q" => Some(PropertyValuesAreDeclining),
            b"1R" => Some(ClassIRight),
            b"1S" => Some(DemandOrSupplyIsInShortage),
            b"1T" => Some(DemandOrSupplyIsInBalance),
            b"1U" => Some(DemandOrSupplyIsOverSupply),
            b"1V" => Some(MarketingTimeIsUnder3Months),
            b"1W" => Some(MarketingTimeIs3To6Months),
            b"1X" => Some(MarketingTimeIsOver6Months),
            b"1Y" => Some(PredominantOccupancyIsTheOwner),
            b"1Z" => Some(PredominantOccupancyIsTheTenant),
            b"02" => Some(PatientWasBedConfinedBeforeTheAmbulanceService),
            b"2A" => Some(PatientIsReceivingOralAntiFungalTherapy),
            b"2B" => Some(Code2B),
            b"2C" => Some(Code2C),
            b"2D" => Some(DeveloperOrBuilderIsInControlOfTheHomeOwnersAssociation),
            b"2E" => Some(SiteIsACornerLot),
            b"2F" => Some(ZoningComplianceIsLegal),
            b"2G" => Some(Code2G),
            b"2H" => Some(ZoningComplianceIsIllegal),
            b"2I" => Some(ThereIsNoZoning),
            b"2J" => Some(HighestAndBestUseAsImprovedIsThePresentUse),
            b"2K" => Some(HighestAndBestUseAsImprovedIsOtherUse),
            b"2L" => Some(ClassIiLeft),
            b"2M" => {
                Some(
                    PropertyIsLocatedInAFederalEmergencyManagementAdministrationSpecialFloodHazardArea,
                )
            }
            b"2N" => Some(Code2N),
            b"2O" => Some(Code2O),
            b"2P" => Some(AppraisalIsMadeSubjectToTheCompletionPerPlansAndSpecifications),
            b"2Q" => Some(Code2Q),
            b"2R" => Some(ClassIiRight),
            b"2S" => Some(ProjectTypeIsCondominium),
            b"2T" => Some(PropertyRightsAreFeeSimple),
            b"2U" => Some(PropertyRightsAreLeasehold),
            b"2V" => {
                Some(
                    SupervisorAppraiserInspectedThePropertyPerSupervisoryAppraisersCertification,
                )
            }
            b"2W" => Some(PropertyWasSoldWithinLast12Months),
            b"2X" => Some(AppraiserSignedStatementOfLimitingConditionsAndDisclaimer),
            b"2Y" => Some(OwnershipInterestInAProperty),
            b"2Z" => Some(Termination),
            b"03" => Some(PatientWasBedConfinedAfterTheAmbulanceService),
            b"3A" => Some(PatientIsReceivingTopicalAntiFungalTherapy),
            b"3B" => Some(PointsPaidBySeller),
            b"3C" => Some(PointsPaidByBuyer),
            b"3D" => Some(SellerConcession),
            b"3E" => Some(LetterOfCertification),
            b"3F" => Some(VerbalReportNeeded),
            b"3G" => Some(AnyRelationshipBetweenOwnerAndOccupant),
            b"3H" => Some(MapAndDirectionsToRemotePropertiesToFollow),
            b"3I" => Some(GroundLeaseToFollow),
            b"3J" => Some(DisclosureStatementToFollow),
            b"3K" => Some(CopyOfPropertyListingToFollow),
            b"3L" => Some(ClassIiiLeft),
            b"3M" => Some(CopyOfTitleReportPlatMapToFollow),
            b"3N" => Some(PropertyTaxBillToFollow),
            b"3O" => Some(EngineeringOrSoilReportToFollow),
            b"3P" => Some(SalesContractAvailable),
            b"3Q" => Some(LeaveWillBeTaken),
            b"3R" => Some(ClassIiiRight),
            b"3S" => Some(Approved),
            b"3T" => Some(BalanceSheetDoesNotBalance),
            b"3U" => Some(BankingDoneThroughParentCompany),
            b"3V" => Some(BankingDoneThroughRelatedConcern),
            b"3W" => Some(BankingDoneThroughSubsidiary),
            b"3X" => Some(CanNotDetermineIfSubjectEngagedInBusiness),
            b"3Y" => Some(Deteriorated),
            b"3Z" => Some(DetrimentalInformationReceived),
            b"04" => Some(PatientWasMovedByStretcher),
            b"4A" => Some(ServicesAreRenderedWithinHospiceElectedPeriodOfCoverage),
            b"4B" => Some(Accidents),
            b"4C" => Some(AccountRepresentativeTransfer),
            b"4D" => Some(AdditionalCoverage),
            b"4E" => Some(AdviceToStop),
            b"4F" => Some(AgentReplacement),
            b"4G" => Some(BackupWithholding),
            b"4H" => Some(CurrentEmployer),
            b"4I" => Some(CurrentOccupation),
            b"4J" => Some(EmployerReimbursement),
            b"4K" => Some(Code4K),
            b"4L" => Some(ExpectedChanges),
            b"4M" => Some(Experimental),
            b"4N" => Some(ForeignFlight),
            b"4O" => Some(FutureInvolvement),
            b"4P" => Some(Code4P),
            b"4Q" => Some(GroupDisabilityInsuranceConversion),
            b"4R" => Some(GroupDisabilityInsuranceOffset),
            b"4S" => Some(GroupDisabilityInsuranceParticipation),
            b"4T" => Some(GroupDisabilityInsuranceTopUp),
            b"4U" => Some(HomeEmployment),
            b"4V" => Some(InformationOmitted),
            b"4W" => Some(InjuryBenefits),
            b"4X" => Some(IssueAtHigherPremiums),
            b"4Y" => Some(IssueWithExclusions),
            b"4Z" => Some(IssueWithoutBenefits),
            b"05" => Some(PatientWasUnconsciousOrInShock),
            b"5A" => Some(TreatmentIsRenderedRelatedToTheTerminalIllness),
            b"5B" => Some(Code5B),
            b"5C" => Some(Code5C),
            b"5D" => Some(JuvenileSeen),
            b"5E" => Some(MedicalTreatment),
            b"5F" => Some(MilitaryAviation),
            b"5G" => Some(NewGroup),
            b"5H" => Some(OtherCoverageOffset),
            b"5I" => Some(OtherPrincipalsBeingInsured),
            b"5J" => Some(OwnerActiveInBusiness),
            b"5K" => Some(PayrollDeduction),
            b"5L" => Some(Prepaid),
            b"5M" => Some(PreviousApplication),
            b"5N" => Some(PrimaryOccupation),
            b"5O" => Some(RacingAccident),
            b"5P" => Some(Replacement),
            b"5Q" => Some(ResidesWithApplicant),
            b"5R" => Some(GenderDistinct),
            b"5S" => Some(SiblingCoverage),
            b"5T" => Some(SicknessBenefits),
            b"5U" => Some(SpecialDating),
            b"5V" => Some(SpousalConsent),
            b"5W" => Some(SuitabilityAnalysis),
            b"5X" => Some(SuitableForCoverage),
            b"5Y" => Some(Taxable),
            b"5Z" => Some(ThisCompanyReplacement),
            b"06" => Some(PatientWasTransportedInAnEmergencySituation),
            b"6A" => Some(TreatmentIsRenderedByAHospiceEmployedPhysician),
            b"6B" => Some(UnitedStatesCitizen),
            b"6C" => Some(PermanentResidentAlien),
            b"6D" => Some(BorrowerIsFirstTimeHomebuyer),
            b"6E" => Some(UnemploymentClaims),
            b"6F" => Some(UnemploymentInsuranceEligibility),
            b"6G" => Some(WorkStatus),
            b"6H" => Some(WorkersCompensationEligible),
            b"6I" => Some(FactoredOnRecourseBasis),
            b"6J" => Some(FactoredWithAdvances),
            b"6K" => Some(FiguresAreActual),
            b"6L" => Some(FiguresAreAnticipated),
            b"6M" => Some(FiguresAreEstimated),
            b"6N" => Some(FiguresAreModified),
            b"6O" => Some(FiguresAreProjected),
            b"6P" => Some(GovernmentBusinessNumberUnavailable),
            b"6Q" => Some(GoodwillOriginPurchasedFromBankruptCompany),
            b"6R" => Some(GoodwillOriginRented),
            b"6S" => Some(HasNoOwnership),
            b"6T" => Some(Improved),
            b"6U" => Some(IntangiblesBreakdownAvailable),
            b"6V" => Some(IntangiblesIncludeOrganizationalExpense),
            b"6W" => Some(IntercompanyRelationsConsistOfLoansAndAdvances),
            b"6X" => Some(IntercompanyRelationsConsistOfMerchandiseTransactions),
            b"6Y" => Some(IntercompanyRelationsConsistOfServiceTransactions),
            b"6Z" => Some(LocalBankingUtilizedOnATransferAccountBasis),
            b"07" => Some(PatientHadToBePhysicallyRestrained),
            b"7A" => Some(TreatmentIsRenderedByAPrivateAttendingPhysician),
            b"7B" => Some(MedicationsOrderedAreBeingAdministeredIntramuscularly),
            b"7C" => Some(MedicationsOrderedAreBeingAdministeredIntravenously),
            b"7D" => Some(MedicationsOrderedAreBeingAdministeredOrally),
            b"7E" => Some(MaintainsNoInventory),
            b"7F" => Some(MedicationsOrderedAreBeingAdministeredSubcutaneously),
            b"7G" => Some(Majority),
            b"7H" => Some(MarketableSecuritiesValuedAtCost),
            b"7I" => Some(MarketableSecuritiesValuedAtLowerOfCostOrMarket),
            b"7J" => Some(InteriorAccessDenied),
            b"7K" => Some(RepairsAreRecommended),
            b"7L" => Some(LoanOriginatedUnderSharedEquityPlan),
            b"7M" => Some(TitleAndOrLegalIssuesExist),
            b"7N" => Some(EnvironmentalIssuesExist),
            b"7O" => Some(PropertyIsListedAsIs),
            b"7P" => Some(PropertyIsListedAsRepaired),
            b"7Q" => Some(VacancyRateIsGreaterThan5PercentTo10Percent),
            b"7R" => Some(VacancyRateIsGreaterThan10PercentTo20Percent),
            b"7S" => Some(VacancyRateIsGreaterThan20Percent),
            b"7T" => Some(MostComparableProperty),
            b"7U" => Some(AnticipateIssuesWhichAffectAbilityToSecureFinancing),
            b"7V" => Some(PointsArePaidBySeller),
            b"7W" => Some(PropertyCoveredByFloodInsurancePolicy),
            b"7X" => Some(PropertyCoveredByEarthquakeInsurancePolicy),
            b"7Y" => Some(PointsAreNegotiable),
            b"7Z" => Some(PropertyIsCurrentlyListedWithARealEstateFirm),
            b"08" => Some(PatientHadVisibleHemorrhaging),
            b"8A" => Some(TreatmentIsCurative),
            b"8B" => Some(IncomeOrAssetsOfAnotherUsed),
            b"8C" => Some(DisclosureOfSomeoneElsesLiabilitiesRequired),
            b"8D" => Some(Code8D),
            b"8E" => Some(Code8E),
            b"8F" => Some(DistantSuburban),
            b"8G" => Some(SelfEmployed),
            b"8H" => Some(LiabilityToBeSatisfied),
            b"8I" => Some(AreAssetsLiabilitiesReportedJointly),
            b"8J" => Some(LocationIsFarm),
            b"8K" => Some(LocationIsResort),
            b"8L" => Some(ShortageExistForCompetingListings),
            b"8M" => Some(CompetingListingsAreInBalance),
            b"8N" => Some(OversupplyExistForCompetingListings),
            b"8O" => Some(IncentivesAreOffered),
            b"8P" => Some(ListedPropertyHasBeenInspected),
            b"8Q" => Some(SalePropertyHasBeenInspected),
            b"8R" => Some(GeneralMarketingConditionIsDepressed),
            b"8S" => Some(GeneralMarketingConditionIsSlow),
            b"8T" => Some(GeneralMarketingConditionIsStatic),
            b"8U" => Some(GeneralMarketingConditionIsImproving),
            b"8V" => Some(GeneralMarketingConditionIsExcellent),
            b"8W" => Some(EmploymentConditionsAreStable),
            b"8X" => Some(EmploymentConditionsAreDeclining),
            b"8Y" => Some(EmploymentConditionsAreIncreasing),
            b"8Z" => Some(OverimprovementConditionExists),
            b"09" => Some(AmbulanceServiceWasMedicallyNecessary),
            b"9A" => Some(TreatmentIsPalliative),
            b"9B" => Some(InvoluntaryCommittal),
            b"9C" => Some(LackOfAvailableEquipment),
            b"9D" => {
                Some(
                    LackOfAppropriateFacilityWithinReasonableDistanceToTreatPatientInTheEventOfComplications,
                )
            }
            b"9E" => Some(SuddenOnsetOfDisorientation),
            b"9F" => Some(Code9F),
            b"9G" => Some(ContinuousHemorrhageFromAnySiteWithAbnormalLabValues),
            b"9H" => Some(PatientRequiresIntensiveIvTherapy),
            b"9I" => Some(PatientRequiresVolumeExpanders),
            b"9J" => Some(PatientRequiresProtectiveIsolation),
            b"9K" => Some(PatientRequiresFrequentMonitoring),
            b"9L" => Some(PatientRequiresExtendedPostOperativeObservation),
            b"9M" => Some(ForeclosureProceedingsHaveBegun),
            b"9N" => Some(UnderimprovementConditionExists),
            b"9O" => Some(MarketabilityOfPropertyIsExcellent),
            b"9P" => Some(MarketabilityOfPropertyIsGood),
            b"9Q" => Some(MarketabilityOfPropertyIsFair),
            b"9R" => Some(MarketabilityOfPropertyIsPoor),
            b"9S" => Some(FeesAreCurrent),
            b"9T" => Some(FeesIncludeTennis),
            b"9U" => Some(FeesIncludePool),
            b"9V" => Some(FeesIncludeInsurance),
            b"9W" => Some(FeesIncludeLandscape),
            b"9X" => Some(FeesIncludeOtherAmenities),
            b"9Y" => Some(MostLikelyBuyerIsOwnerOccupant),
            b"9Z" => Some(MostLikelyBuyerIsInvestor),
            b"10" => Some(PatientIsAmbulatory),
            b"11" => Some(AmbulationIsImpairedAndWalkingAidIsUsedForTherapyOrMobility),
            b"12" => Some(PatientIsConfinedToABedOrChair),
            b"13" => Some(PatientIsConfinedToARoomOrAnAreaWithoutBathroomFacilities),
            b"14" => Some(AmbulationIsImpairedAndWalkingAidIsUsedForMobility),
            b"15" => {
                Some(
                    PatientConditionRequiresPositioningOfTheBodyOrAttachmentsWhichWouldNotBeFeasibleWithTheUseOfAnOrdinaryBed,
                )
            }
            b"16" => {
                Some(
                    PatientNeedsATrapezeBarToSitUpDueToRespiratoryConditionOrChangeBodyPositionsForOtherMedicalReasons,
                )
            }
            b"17" => Some(PatientsAbilityToBreatheIsSeverelyImpaired),
            b"18" => {
                Some(
                    PatientConditionRequiresFrequentAndOrImmediateChangesInBodyPositions,
                )
            }
            b"19" => Some(PatientCanOperateControls),
            b"20" => Some(SiderailsAreToBeAttachedToAHospitalBedOwnedByTheBeneficiary),
            b"21" => Some(PatientOwnsEquipment),
            b"22" => {
                Some(
                    MattressOrSiderailsAreBeingUsedWithPrescribedMedicallyNecessaryHospitalBedOwnedByTheBeneficiary,
                )
            }
            b"23" => {
                Some(
                    PatientNeedsLiftToGetInOrOutOfBedOrToAssistInTransferFromBedToWheelchair,
                )
            }
            b"24" => {
                Some(
                    PatientHasAnOrthopedicImpairmentRequiringTractionEquipmentWhichPreventsAmbulationDuringPeriodOfUse,
                )
            }
            b"25" => {
                Some(
                    ItemHasBeenPrescribedAsPartOfAPlannedRegimenOfTreatmentInPatientHome,
                )
            }
            b"26" => Some(PatientIsHighlySusceptibleToDecubitusUlcers),
            b"27" => Some(PatientOrACareGiverHasBeenInstructedInUseOfEquipment),
            b"28" => Some(PatientHasPoorDiabeticControl),
            b"29" => {
                Some(
                    A67HourNocturnalStudyDocuments30EpisodesOfApneaEachLastingMoreThan10Seconds,
                )
            }
            b"30" => Some(Code30),
            b"31" => Some(PatientHasHadATotalKneeReplacement),
            b"32" => Some(PatientHasIntractableLymphedemaOfTheExtremities),
            b"33" => Some(PatientIsInANursingHome),
            b"34" => Some(PatientIsConscious),
            b"35" => Some(ThisFeedingIsTheOnlyFormOfNutritionalIntakeForThisPatient),
            b"36" => Some(PatientWasAdministeredPremix),
            b"37" => Some(OxygenDeliveryEquipmentIsStationary),
            b"38" => Some(CertificationSignedByThePhysicianIsOnFileAtTheSuppliersOffice),
            b"39" => Some(PatientHasMobilizingRespiratoryTractSecretions),
            b"40" => {
                Some(
                    PatientOrCaregiverIsCapableOfUsingTheEquipmentWithoutTechnicalOrProfessionalSupervision,
                )
            }
            b"41" => {
                Some(PatientOrCaregiverIsUnableToPropelOrLiftAStandardWeightWheelchair)
            }
            b"42" => Some(PatientRequiresLegElevationForEdemaOrBodyAlignment),
            b"43" => Some(PatientWeightOrUsageNeedsNecessitateAHeavyDutyWheelchair),
            b"44" => Some(PatientRequiresRecliningFunctionOfAWheelchair),
            b"45" => Some(PatientIsUnableToOperateAWheelchairManually),
            b"46" => Some(Code46),
            b"47" => Some(AdvertisementRunCondition),
            b"48" => Some(IndividualPaidForLastDayWorked),
            b"49" => Some(FullWagesPaidForDateOfInjury),
            b"50" => Some(CitationOrTicketIssued),
            b"51" => Some(IndividualIsMemberOfPolicyholdersHousehold),
            b"52" => Some(IndividualPermittedToUseVehicle),
            b"53" => Some(IndividualWoreSeatbelt),
            b"54" => Some(ChildRestraintDeviceInVehicle),
            b"55" => Some(ChildRestraintDeviceUsed),
            b"56" => Some(IndividualInjured),
            b"57" => Some(IndividualTransportedToAnotherLocation),
            b"58" => Some(Code58),
            b"59" => Some(Code59),
            b"60" => Some(TransportationWasToTheNearestFacility),
            b"61" => Some(EmployeeIsExempt),
            b"62" => Some(ClaimantIsCoveredOnTheEmployersLongTermDisabilityPlan),
            b"63" => Some(EmployeesJobResponsibilitiesChangedDueToTheDisablingCondition),
            b"64" => Some(EmployerHasAReturnToWorkPolicyForDisabledEmployees),
            b"65" => Some(Open),
            b"66" => Some(Normal),
            b"67" => Some(ClosedModerate),
            b"68" => Some(Severe),
            b"69" => Some(Moderate),
            b"70" => Some(Straight),
            b"71" => Some(Convex),
            b"72" => Some(Concave),
            b"73" => Some(DoubleProtrusion),
            b"74" => Some(NoCrossbite),
            b"75" => Some(Posterior),
            b"76" => Some(Anterior),
            b"77" => Some(Maxillary),
            b"78" => Some(Mandibular),
            b"79" => Some(Right),
            b"80" => Some(Left),
            b"81" => Some(MaxillaryModerate),
            b"82" => Some(MandibularModerate),
            b"83" => Some(MaxillarySevere),
            b"84" => Some(MandibularSevere),
            b"85" => Some(IncomeHasBeenVerified),
            b"86" => Some(PersonHasBeenInterviewed),
            b"87" => Some(RentHasBeenVerified),
            b"88" => Some(EmployerHasBeenVerified),
            b"89" => Some(PositionHasBeenVerified),
            b"90" => Some(InquiryHasBeenVerified),
            b"91" => Some(OutstandingJudgments),
            b"92" => Some(DeclaredBankruptcyInPast7Years),
            b"93" => Some(ForeclosureOrDeedInLieuInPast7Years),
            b"94" => Some(PartyToLawsuit),
            b"95" => Some(Code95),
            b"96" => Some(CurrentlyDelinquentOrInDefault),
            b"97" => Some(Code97),
            b"98" => Some(PartOfDownPaymentBorrowed),
            b"99" => Some(CoMakerOrEndorserOnANote),
            b"A0" => Some(LiabilityCoverageWillTransfer),
            b"A1" => Some(MostLikelyBuyerIsOtherPersonOrEntity),
            b"A2" => Some(PotentialFinancingIsFannieMae),
            b"A3" => Some(SuppressPaperEndorsement),
            b"A4" => Some(DoNotSuppressPaperEndorsement),
            b"A5" => Some(Escrow),
            b"A6" => Some(TeachingMinor),
            b"A7" => Some(SubServicerSubmitted),
            b"A8" => Some(FirstMortgage),
            b"A9" => Some(SecondMortgage),
            b"AA" => Some(Amputation),
            b"AB" => Some(AddressSkipBegin),
            b"AC" => Some(AddressCorrected),
            b"AD" => Some(AutomaticDrillTimeCalculated),
            b"AE" => Some(AutomaticEdgingTimeCalculated),
            b"AF" => Some(AutomaticallySelect),
            b"AFM" => Some(AcceptingFamilyMembers),
            b"AG" => Some(Agitated),
            b"AH" => Some(AutomaticallySearchAndList),
            b"AI" => Some(AddressIncorrect),
            b"AJ" => Some(Assumable),
            b"AK" => Some(PotentialFinancingIsCash),
            b"AL" => Some(AmbulationLimitations),
            b"AM" => Some(PotentialFinancingIsOutsideLender),
            b"AN" => Some(AddressIncomplete),
            b"AO" => Some(AcceptCertificationWithoutChanges),
            b"AP" => Some(AlleyIsPublic),
            b"AQ" => Some(PotentialFinancingIsFederalHousingAdministration),
            b"AR" => Some(AddressSkipResolved),
            b"AS" => Some(AddressSkipExhaust),
            b"AT" => Some(AcceptStatementOfLimitingConditionsWithoutChanges),
            b"AU" => Some(AutomaticUndersideTimeCalculated),
            b"AV" => Some(AvailableNotUsed),
            b"AW" => Some(AcceptCertificationWithChanges),
            b"AX" => Some(AcceptStatementOfLimitingConditionsWithChanges),
            b"AY" => Some(AdjacentTrackOccupied),
            b"AZ" => Some(PotentialFinancingIsVeteransAffairs),
            b"B0" => Some(UninsuredMotoristCoverageWillTransfer),
            b"B1" => Some(MortgageInForeclosure),
            b"B2" => Some(CodeB2),
            b"B3" => Some(PotentialFinancingIsContractForDeed),
            b"B4" => Some(OnlyTheExteriorHasBeenInspected),
            b"B5" => Some(RealEstateOwnedPropertyOrForeclosureProperty),
            b"B6" => Some(NumberOfComparableListingsIsNormal),
            b"B7" => Some(NumberOfComparableListingsIsAnOversupply),
            b"B8" => Some(NumberOfComparableListingsIsAShortage),
            b"B9" => Some(PropertyManagementExpensesOutstanding),
            b"BA" => Some(BorrowerLetterAttempt),
            b"BB" => Some(BuildingOrMobileHomeIsInACoastalBarrierResourcesArea),
            b"BC" => Some(BorrowerTelephoneContact),
            b"BD" => Some(BusinessPending),
            b"BE" => Some(BorrowerLetterContact),
            b"BF" => Some(MarketableSecuritiesValuedAtMarket),
            b"BG" => Some(AppropriateImprovementConditionExists),
            b"BH" => Some(NameUnknownToLocalAuthorities),
            b"BI" => Some(NoManufacturingDoneOnPremises),
            b"BJ" => Some(Occasional),
            b"BK" => Some(OfficerOrOwnerInOtherBusinesses),
            b"BL" => Some(CodeBL),
            b"BM" => Some(Old),
            b"BN" => Some(OperatesOnPartTimeBasis),
            b"BO" => Some(ParentFinancialStatementUsed),
            b"BP" => Some(BorrowerPaymentReceived),
            b"BPD" => Some(BeneficiaryIsPartiallyDependent),
            b"BQ" => Some(ProductInformationAvailable),
            b"BR" => Some(CodeBR),
            b"BS" => Some(RevenueDerivedFromCommissions),
            b"BT" => Some(BorrowerTelephoneAttempt),
            b"BTD" => Some(BeneficiaryIsTotallyDependent),
            b"BU" => Some(RevenueDerivedFromDonations),
            b"BV" => Some(RevenueDerivedFromFees),
            b"BW" => Some(RevenueDerivedFromGrants),
            b"BX" => Some(RevenueDerivedFromTaxes),
            b"BY" => Some(SprinklerEquipped),
            b"BZ" => Some(StatementRequestedFromGovernmentRegistry),
            b"C0" => Some(CollisionCoverageWillTransfer),
            b"C1" => Some(AdvancesFromPropertyManagementExpensesOutstanding),
            b"C2" => Some(FinalDemandLetterSent),
            b"C3" => Some(LenderRequestForAssistance),
            b"C4" => Some(MortgageHasLenderPurchasedMortgageInsurance),
            b"C5" => Some(InsufficientFunds),
            b"C6" => Some(CreditEnhancedMortgage),
            b"C7" => Some(CorporateAppointment),
            b"C8" => Some(SpecialServicingRequired),
            b"C9" => {
                Some(
                    ClientSpecificallyRequestedConsiderationOfSpecialFinancingOrAnAssumableLoan,
                )
            }
            b"CA" => Some(CaneRequired),
            b"CB" => Some(CompleteBedrest),
            b"CC" => Some(CollectionCardWasLeft),
            b"CD" => Some(CallToDirectoryAssistanceForReferenceTelephone),
            b"CE" => Some(CoSignerTelephoneAttempt),
            b"CF" => Some(CoSignerTelephoneContact),
            b"CFD" => Some(ClaimIsFraudulent),
            b"CG" => Some(CoSignerDelinquencyLetterSent),
            b"CH" => Some(CoSignerFinalDemandLetterSent),
            b"CI" => Some(CallToDirectoryAssistanceForCoSignerTelephone),
            b"CJ" => Some(ValidBorrowerAddressOrPhoneAttemptWithPreviousHolder),
            b"CK" => Some(Convertible),
            b"CL" => Some(ClaimantHadAPreExistingInjury),
            b"CM" => Some(Comatose),
            b"CN" => Some(CommonElementsAreLeasedToOrByTheHomeOwnersAssociation),
            b"CNJ" => Some(CumulativeInjury),
            b"CO" => Some(Contracture),
            b"CP" => Some(CasePending),
            b"CQ" => Some(Callable),
            b"CR" => Some(CrutchesRequired),
            b"CS" => Some(CommunityParticipatesInNationalFloodInsuranceProgram),
            b"CT" => Some(CommonElementsAreCompleted),
            b"CU" => Some(CurbAndGutterArePublic),
            b"CV" => Some(Cooperative),
            b"CW" => Some(CoolingWaterIsLow),
            b"CX" => Some(CertificationStatus),
            b"CY" => Some(CarSpacesAreAdequate),
            b"CZ" => Some(CarSpacesAreInadequate),
            b"D0" => Some(ComprehensiveCoverageWillTransfer),
            b"D1" => Some(IssueCheckPayableToBorrowerAndReturnToServicer),
            b"D2" => Some(IssueCheckPayableToServicerAndReturnToServicer),
            b"D3" => Some(IssueCheckPayableToBorrowerAndSendToBorrower),
            b"D4" => Some(IssueCheckPayableToServicerOrBorrowerAndReturnToServicer),
            b"D5" => Some(IssueCheckPayableToOtherPayee),
            b"D6" => Some(Positive),
            b"D7" => Some(Negative),
            b"D8" => Some(TaxesAreTypicalForTheAreaAndPriceRange),
            b"D9" => Some(ImprovementConformsToZoningRegulations),
            b"DA" => Some(CallToDirectoryAssistanceForBorrowerTelephone),
            b"DB" => Some(DefermentOrForbearanceBegin),
            b"DC" => Some(Declined),
            b"DD" => Some(BorrowerFurnishedDemographicData),
            b"DE" => Some(DefermentOrForbearanceEnd),
            b"DF" => Some(FundsAvailableForUnsecuredCreditors),
            b"DFR" => Some(DeductibleAmountFullyRecovered),
            b"DG" => Some(DynamicBrakesAreOut),
            b"DH" => Some(DebtorHasBeenDomiciled),
            b"DI" => Some(Disoriented),
            b"DJ" => Some(DynamicBrakesAreOperational),
            b"DK" => Some(ConstructionWarranty),
            b"DL" => Some(ConstructionWarrantyTransferable),
            b"DM" => Some(MaintenanceDrugUnderClientsBenefitPlan),
            b"DN" => Some(PaymentReducedBecauseMaximumAllowableCostExceeded),
            b"DNR" => Some(DeductibleAmountNotFullyRecovered),
            b"DO" => Some(BenefitsTerminatedPriorToServiceDate),
            b"DP" => Some(Depressed),
            b"DQ" => Some(DrugPartOfFormularyDataBase),
            b"DR" => Some(SubjectNotEngagedInBusiness),
            b"DS" => Some(AllDoorSealsAreIntact),
            b"DT" => Some(FilingFeeAttached),
            b"DU" => Some(SubjectNotEngagedInBusinessAtRequestedAddress),
            b"DV" => Some(Suspended),
            b"DW" => Some(Total),
            b"DX" => Some(UnableToRespond),
            b"DY" => Some(DyspneaWithMinimalExertion),
            b"DZ" => Some(UsesOwnFacilities),
            b"E0" => Some(FiguresAreTotal),
            b"E1" => Some(FixedAssetBreakdownUndisclosed),
            b"E2" => Some(ForTheFiscalYear),
            b"E3" => Some(ForThePeriod),
            b"E4" => Some(FormedByConsolidation),
            b"E5" => Some(FormedByMerger),
            b"E6" => Some(PriorBankruptcyCaseFiledInLast6Years),
            b"E7" => Some(DebtorIsNotRepresentedByAnAttorney),
            b"E8" => Some(APendingCaseHasBeenFiled),
            b"E9" => Some(GuaranteedByParentCompany),
            b"EA" => Some(HasAuthorityForAllPurchases),
            b"EB" => Some(HasAuthorityToPurchaseSupplies),
            b"EC" => Some(EquipmentCertified),
            b"ED" => Some(HasBusinessInterruptionInsurance),
            b"EE" => Some(HasClassOfStock),
            b"EF" => Some(HasExtendedCoverageInsurance),
            b"EG" => Some(HasFireInsurance),
            b"EH" => Some(HasJointAuthority),
            b"EI" => Some(HasLifeInsurance),
            b"EJ" => Some(ExistenceOfPreliminaryFloodDetermination),
            b"EK" => Some(ExistenceOfCommunityParticipationInTheNationalFloodInsurance),
            b"EL" => Some(EnduranceLimitations),
            b"EM" => Some(HasMarriageContract),
            b"EN" => Some(ElectricityOn),
            b"EO" => Some(EquipmentIsOverhauled),
            b"EP" => Some(ExercisesPrescribed),
            b"EQ" => Some(HasNoParValue),
            b"ER" => Some(EngineStartUpPerformedWithNoProblemsReported),
            b"ES" => Some(EngineStartUpPerformedWithProblemsReported),
            b"ET" => Some(ElectricalControlSystemShutDown),
            b"EU" => Some(HasOtherInsurance),
            b"EV" => Some(HasParValue),
            b"EW" => Some(HasSoleAuthority),
            b"EX" => Some(Excellent),
            b"EY" => Some(HasVotingRights),
            b"EZ" => Some(HeadingAddressInRegisteredOfficeOnly),
            b"F0" => Some(HighLevel),
            b"F1" => Some(HomeworkersEmployed),
            b"F2" => Some(InSubscriberShares),
            b"F3" => Some(Inactive),
            b"F4" => Some(Incomplete),
            b"F5" => Some(IncorporationDetailsRequested),
            b"F6" => Some(IncreaseOrUp),
            b"F7" => Some(InformationCannotBeProvidedAtThisTime),
            b"F8" => Some(InformationInDate),
            b"F9" => Some(InformationRequiresInvestigation),
            b"FA" => Some(ActionsHasASignificantEnvironmentalEffect),
            b"FB" => Some(ApplicationIncludesCompleteSystem),
            b"FC" => Some(AntennaIsMountedOnAStructureWithAnExistingAntenna),
            b"FD" => Some(NoticeOfConstructionOrAlterationHasBeenFiled),
            b"FE" => Some(ApplicantWantsToMonitorFrequency),
            b"FF" => Some(ApplicantHasBeenDeniedGovernmentBenefitsDueToUseOfDrugs),
            b"FG" => Some(ApplicationIsCertified),
            b"FH" => Some(ApplicationIsForOtherThanANewStation),
            b"FI" => Some(FeeRequired),
            b"FJ" => Some(FloodStatus),
            b"FK" => Some(FloodInsuranceRequired),
            b"FL" => Some(CodeFL),
            b"FM" => Some(CodeFM),
            b"FN" => Some(NotTooHighLevel),
            b"FO" => Some(Forgetful),
            b"FP" => Some(FloodCertificationWithLifeOfLoan),
            b"FQ" => Some(StreetMaintenanceIsPublic),
            b"FR" => Some(Fair),
            b"FS" => Some(NotYetRegistered),
            b"FT" => Some(ObligedToFileBalanceSheet),
            b"FU" => Some(OfficialConfirmationReceived),
            b"FV" => Some(OldButWellKept),
            b"FW" => Some(OldEstablishedBusiness),
            b"FX" => Some(OperatedAtBreakEven),
            b"FY" => Some(OperatesAsAgent),
            b"FZ" => Some(FloodZoneStatus),
            b"G0" => Some(OutOfBusiness),
            b"G1" => Some(OutstandingClaims),
            b"G2" => Some(GasOn),
            b"G3" => Some(HazardousMaterialsAreUsedOrProduced),
            b"G4" => Some(GeneticallyEngineeredOrganismsAreUsedOrProduced),
            b"G5" => Some(ThisIsAGroupProposal),
            b"G6" => Some(HistoricalSitesAreAffected),
            b"G7" => Some(FacilitiesAreProperlyAccreditedOrAuthorized),
            b"G8" => {
                Some(ProprietaryOrPrivilegedInformationWillBeContainedInTheApplication)
            }
            b"G9" => Some(ThisProjectHasAnActualOrPotentialImpactOnTheEnvironment),
            b"GA" => Some(GrowthRateIsFullyDeveloped),
            b"GB" => Some(OutstandingSocialSecurityClaims),
            b"GC" => Some(CodeGC),
            b"GD" => Some(ProductDemonstrationInEffect),
            b"GE" => Some(OwnershipAcknowledgedInSignedStatement),
            b"GF" => Some(OwnershipAcknowledgedVerbally),
            b"GG" => Some(OwnershipNotAcknowledged),
            b"GH" => Some(OwnsNoRealEstate),
            b"GI" => Some(OwnsRealEstateButDetailsNotAvailable),
            b"GJ" => Some(PreparedFromBooksWithoutAudit),
            b"GK" => Some(PreparedFromStatementByAccountant),
            b"GL" => Some(ProfitsPaidToGroup),
            b"GM" => Some(ShelfSetToManufacturersStandard),
            b"GN" => Some(PubliclyTraded),
            b"GO" => Some(Good),
            b"GP" => Some(PurchaseAuthorityIsQualified),
            b"GQ" => Some(PurchasesOnFloorPlan),
            b"GR" => Some(ShelfSetToRetailersSchematic),
            b"GS" => Some(PurchasesOnLetterOfCredit),
            b"GT" => Some(RealEstateCheckIsNecessary),
            b"GU" => Some(RecordOfPreferentialClaims),
            b"GV" => Some(RegisteredAddressIsSameAsBusinessAddress),
            b"GW" => Some(RelativesHelpInBusiness),
            b"GX" => Some(Satisfactory),
            b"GY" => Some(SeasonsAreSteady),
            b"GZ" => Some(Secured),
            b"H0" => Some(OrganizationCertifiesComplianceWithFederalLobbyingRegulations),
            b"H1" => Some(ProjectInvolvesInternationalCoOperativeActivities),
            b"H2" => Some(HumanAnatomicalSubstancesAreUsed),
            b"H3" => Some(HandicapFacilitiesAreAvailable),
            b"H4" => Some(LobbyingActivitiesHaveBeenConductedRegardingTheProposal),
            b"H5" => Some(OrganizationCertifiesComplianceWithTheDrugFreeWorkplaceAct),
            b"H6" => {
                Some(
                    OrganizationCertifiesComplianceWithTheCodeOfFederalRegulationsRegardingResearchMisconduct,
                )
            }
            b"H7" => Some(OrganizationProvidesASmokeFreeWorkplace),
            b"H8" => {
                Some(OrganizationCertifiesComplianceWithFederalDiscriminationRegulations)
            }
            b"H9" => Some(CodeH9),
            b"HA" => Some(WellMaintained),
            b"HB" => Some(InterestRateBuydown),
            b"HC" => Some(HeatingAndCoolingForTheIndividualUnitsSeparatelyMetered),
            b"HD" => Some(HighDischarge),
            b"HE" => Some(HighEngineWaterPressure),
            b"HF" => Some(InterestOnly),
            b"HG" => Some(GraduatedPayment),
            b"HH" => Some(PrincipalBalanceExceedsMaximumNegativeAmortization),
            b"HI" => Some(LastChange),
            b"HJ" => Some(LiabilityReleased),
            b"HK" => Some(LiabilityNotReleased),
            b"HL" => Some(HearingLimitations),
            b"HM" => Some(LiabilityDeterminedByNoteHolder),
            b"HN" => Some(AfterConversion),
            b"HO" => Some(Hostile),
            b"HP" => Some(AfterModification),
            b"HQ" => Some(Balloon),
            b"HR" => Some(CapitalizedMortgage),
            b"HS" => Some(FederalWagesInEffect),
            b"HT" => Some(CodeHT),
            b"HU" => Some(CodeHU),
            b"HV" => Some(CodeHV),
            b"HW" => Some(CodeHW),
            b"HX" => Some(EmployeeIsIneligibleToWork),
            b"HY" => Some(MetesAndBounds),
            b"HZ" => Some(CodeHZ),
            b"I0" => Some(BasedOnOperatingData),
            b"I1" => Some(UsesOutsideServices),
            b"I2" => Some(VeryHighLevel),
            b"I3" => Some(VerySmall),
            b"I4" => Some(VoluntaryBankruptcy),
            b"I5" => Some(WellBalanced),
            b"I6" => Some(WellRegardedInBusinessCircles),
            b"I7" => Some(OrganizationHasDelinquentFederalDebts),
            b"I8" => {
                Some(OrganizationHasBeenPlacedOnTheFederalDebarmentAndSuspensionList)
            }
            b"I9" => Some(NoShowIndicator),
            b"IA" => Some(InterestPaidInAdvance),
            b"IB" => Some(InterestPaidInArrears),
            b"IC" => Some(InterestCarryover),
            b"ID" => Some(SellsDirectly),
            b"IE" => Some(SellsWithAgents),
            b"IF" => Some(SellsWithStorage),
            b"IG" => Some(Small),
            b"IH" => Some(IndependentAtHome),
            b"II" => Some(SomeIncrease),
            b"IJ" => Some(SomewhatDecliningTendency),
            b"IK" => Some(StartedSomeTimeAgo),
            b"IL" => Some(IndustryLocation),
            b"IM" => Some(Sufficient),
            b"IN" => Some(Indifferent),
            b"IO" => Some(TerminationDateSet),
            b"IP" => Some(InjuryOccurredOnEmployersPremises),
            b"IQ" => Some(TermsIncludeLumpSumPayments),
            b"IR" => Some(TermsIncludeProgressPayments),
            b"IS" => Some(TermsOnCostPlusBasis),
            b"IT" => Some(TermsOnFixedFeeBasis),
            b"IU" => Some(TradeStyleRegistered),
            b"IV" => Some(TradingAddressOfSoleProprietor),
            b"IW" => Some(UnchangedSituation),
            b"IX" => Some(Undetermined),
            b"IY" => Some(Unsatisfactory),
            b"IZ" => Some(Unsecured),
            b"J0" => Some(QualifiesAsAnEnergyEfficientHome),
            b"J1" => Some(CodeJ1),
            b"J2" => Some(RateNegotiated),
            b"J3" => Some(UnderPenaltyOfPerjuryTheInformationIsTrueAndCorrect),
            b"J4" => {
                Some(
                    ProjectRequiresInterGovernmentReviewForActivitiesThatAffectStateOrLocalGovernmentOrPossibleNationalSecurityImplications,
                )
            }
            b"J5" => Some(FilingOnBehalfOfDebtorIsAuthorized),
            b"J6" => Some(DebtorUnderstandsTheReliefAvailableUnderEachBankruptcyChapter),
            b"J7" => Some(AttorneyDeclaresThatDebtorHasBeenInformed),
            b"J8" => {
                Some(AttorneyHasExplainedTheReliefAvailableUnderEachBankruptcyChapter)
            }
            b"J9" => {
                Some(ThereHasBeenATransferOfAClaimAgainstTheDebtorByOrToAnyPetitioner)
            }
            b"JA" => Some(ThirdPartyOriginated),
            b"JB" => Some(ExistingConstruction),
            b"JC" => Some(OtherLien),
            b"JCA" => Some(JointCoverageApplies),
            b"JD" => Some(SubjectLien),
            b"JE" => Some(CodeJE),
            b"JF" => Some(PrimaryUnderwritingSystem),
            b"JG" => Some(NonNewPartsUsed),
            b"JH" => Some(PledgedLoan),
            b"JI" => Some(SecurityDelivery),
            b"JJ" => Some(SecondaryUnderwritingSystem),
            b"JK" => Some(DistributionIsStopped),
            b"JL" => Some(SentenceWasSuspended),
            b"JM" => Some(VeryNegativeInformationExists),
            b"JN" => Some(PaymentNotesExist),
            b"JO" => Some(Immigrated),
            b"JP" => Some(AuditedWithQualifications),
            b"JQ" => Some(Audited),
            b"JR" => Some(TemporarilyClosed),
            b"JS" => Some(Partial),
            b"JT" => Some(TelephoneNumberIsUnpublished),
            b"JU" => Some(TelephoneNumberIsNotInService),
            b"JV" => Some(NegativeInformationExistsForTheGroup),
            b"JW" => Some(TheMoreImportantItemsAreOnlyIncluded),
            b"JX" => Some(InterestOwnedByAffiliatedCompany),
            b"JY" => Some(InterestOwnedBySubjectOfInquiry),
            b"JZ" => Some(QualifiesAsAGovernmentApprovedCondominiumOrProject),
            b"K0" => Some(AccountReceivablesBreakdownUndisclosed),
            b"K1" => Some(AdditionalRecordItemsAvailable),
            b"K2" => Some(AddressIsQualified),
            b"K3" => Some(AllPaidInOrIssued),
            b"K4" => Some(AppearsHigh),
            b"K5" => Some(AppearsNotToGuaranteeSufficientCoverage),
            b"K6" => Some(AppearsSufficientlyHigh),
            b"K7" => Some(AppearsToIndicateAStrainedSituation),
            b"K8" => Some(BanksWithMainNationalBanks),
            b"K9" => Some(BillsPaidFromBranchOffice),
            b"KA" => Some(BillsPaidFromDivisionOffice),
            b"KB" => Some(BillsPaidFromHeadquartersOffice),
            b"KC" => Some(BondInformationAvailable),
            b"KD" => Some(ChangedAccountingDate),
            b"KE" => Some(Clear),
            b"KF" => Some(ClearDecliningTendency),
            b"KG" => Some(ClearIncrease),
            b"KH" => Some(Cluttered),
            b"KI" => Some(CompanyHasNoOtherLocations),
            b"KJ" => Some(CompanyIsBranchOfForeignEntity),
            b"KK" => Some(CompanyIsPerpetual),
            b"KL" => Some(CompanyIsTaxExempt),
            b"KM" => Some(ComparedToSamePeriodLastYear),
            b"KN" => Some(ConductedAtALoss),
            b"KO" => Some(CodeKO),
            b"KP" => Some(Large),
            b"KQ" => Some(LetterOfAgreementPresent),
            b"KR" => Some(LetterOfAgreementWithdrawn),
            b"KS" => Some(LetterOfLiabilityPresent),
            b"KT" => Some(LetterOfLiabilityWithdrawn),
            b"KU" => Some(LocationInquiredUponIsABranch),
            b"KV" => Some(CodeKV),
            b"KW" => Some(LocationInquiredUponIsAHeadquarters),
            b"KX" => Some(LocationIsForeign),
            b"KY" => Some(MeansExhausted),
            b"KZ" => Some(MediumToLarge),
            b"L0" => Some(ImmunizationMandatedByStateLawForEmployment),
            b"L1" => Some(GeneralStandardOf20DegreeOr5DiopterSphereOrCylinderChangeMet),
            b"L2" => Some(ReplacementDueToLossOrTheft),
            b"L3" => Some(ReplacementDueToBreakageOrDamage),
            b"L4" => Some(ReplacementDueToPatientPreference),
            b"L5" => Some(ReplacementDueToMedicalReason),
            b"L6" => Some(LandContract),
            b"L7" => Some(AccountCurrent),
            b"L8" => Some(VeryGood),
            b"L9" => Some(Restored),
            b"LA" => Some(LetterOfMapAmendmentOrLetterOfMapRevision),
            b"LB" => Some(LegallyBlind),
            b"LC" => Some(ProducerOfGoods),
            b"LD" => Some(DrawbackIndicator),
            b"LE" => Some(Lethargic),
            b"LF" => Some(CustomsRuleApplicable),
            b"LG" => Some(ExportedPursuantToLawRegulationOrToCancelCustomsBond),
            b"LH" => Some(CountryOfOriginInformationAppliesToAllPriorShipments),
            b"LI" => Some(PriceEstimated),
            b"LJ" => Some(CodeLJ),
            b"LK" => Some(KitForm),
            b"LL" => Some(LockoutEffective),
            b"LM" => Some(LetterOfAppointment),
            b"LN" => {
                Some(
                    FacilitysEmergencyResponsePlanIncludesSpecificActionsToBeTakenInResponseToAccidentalReleasesOfRegulatedSubstances,
                )
            }
            b"LO" => Some(LocomotiveIsIsolated),
            b"LP" => Some(LowEngineOilPressure),
            b"LQ" => Some(FacilityHadASafetyInspection),
            b"LR" => Some(LocomotiveEngineIsRunning),
            b"LS" => Some(LesseeSignatureOnFile),
            b"LSD" => Some(ListSpecialtyInDirectory),
            b"LT" => Some(LenderOrServicerTransfer),
            b"LU" => Some(EvidenceOfDampness),
            b"LV" => Some(EvidenceOfTermites),
            b"LW" => Some(EvidenceOfStructureSettlement),
            b"LX" => Some(SalvageMoved),
            b"LY" => Some(AddressIsFormerLocation),
            b"LZ" => Some(AddressIsOccupiedByOthers),
            b"M0" => Some(CodeM0),
            b"M1" => Some(DataCorrected),
            b"M2" => Some(ServicerRecordSelected),
            b"M3" => Some(LengthOfServiceIs3MonthsOrLess),
            b"M4" => Some(CodeM4),
            b"M5" => Some(LengthOfServiceIs1YearThrough5Years),
            b"M6" => Some(LengthOfServiceIsMoreThan5Years),
            b"M7" => Some(CataractOrCornealTransplantOrOtherConditionSuchAsKeratoconus),
            b"M8" => Some(VisionInWorseEyeCorrectableTo2040OrBetterWithRegularLenses),
            b"M9" => Some(ContactLensesCorrectedVisionInWorseEyeTo2040OrBetter),
            b"MA" => Some(MajorAlarmFlagReported),
            b"MB" => Some(EquipmentHasModifiedConfiguration),
            b"MC" => Some(OtherMentalCondition),
            b"MD" => Some(MarketingTimeIs4To6Months),
            b"ME" => Some(TrendReversed),
            b"MF" => Some(MicroprocessorFault),
            b"MG" => Some(MortgageInsuranceApplicationIncluded),
            b"MH" => Some(MortgageCreditReportIncluded),
            b"MI" => Some(ResidentialLoanApplicationIncluded),
            b"MJ" => Some(RealEstateInformationReportIncluded),
            b"MK" => Some(RealEstateTitleEvidenceIncluded),
            b"ML" => Some(ManuallySearchAndList),
            b"MM" => Some(CodeMM),
            b"MN" => Some(CodeMN),
            b"MO" => Some(CooperativeProjectIncludesOrOwnsAnyCommercialUnits),
            b"MP" => Some(UnitsAndProjectAmenitiesAreComplete),
            b"MQ" => Some(EligibleTrust),
            b"MR" => Some(ResaleProperty),
            b"MS" => Some(MiscellaneousSkipTraceAttempt),
            b"MT" => Some(PhotosMatchDescription),
            b"MU" => Some(PhotosShowNegativeInfluence),
            b"MV" => Some(ExcludeFromMonthlyDebt),
            b"MW" => {
                Some(ThisBrokerMarketAnalysisIsBeingCompletedForHomeMarketAssistance)
            }
            b"MX" => Some(ThisBrokerMarketAnalysisIsBeingCompletedForHomesaleOrBuyout),
            b"MY" => Some(ProjectTypeIsSingleFamily),
            b"MZ" => Some(ProjectTypeIsOther),
            b"N0" => Some(HospitalizedOverNight),
            b"N1" => Some(CodeN1),
            b"N2" => Some(ClaimInvolvesRestrictedWorkActivityWithoutDaysAwayFromWork),
            b"N3" => Some(StrikeOrLockoutInProgress),
            b"N4" => Some(ShutdownOrLayoffInProgress),
            b"N5" => Some(WorkIsSeasonal),
            b"N6" => Some(NaturalDisasterOrAdverseWeatherAffectingWork),
            b"N7" => Some(ShorterWorkSchedulesOrFewerPayPeriodsThanUsualInEffect),
            b"N8" => Some(LongerWorkSchedulesOrMorePayPeriodsThanUsualInEffect),
            b"N9" => Some(OtherFactorsAffectClaimFrequency),
            b"NA" => Some(NoUserAvailable),
            b"NB" => Some(NeighborhoodPredominatelySingleFamilyDwellings),
            b"NC" => Some(ItemHasDirectNumericalControl),
            b"ND" => Some(NoteHolderPermissionRequired),
            b"NDP" => Some(NoDeductibleProgram),
            b"NE" => Some(Notarized),
            b"NF" => Some(NewConstruction),
            b"NG" => Some(MortgagePointsAreCustomarilyPaidBySeller),
            b"NH" => Some(NoNationalFloodInsuranceProgramMap),
            b"NI" => Some(SeasonedMortgage),
            b"NJ" => {
                Some(
                    IssuesAreAnticipatedThatWouldAffectTheAbilityToSecureFinancingOfTheSubjectProperty,
                )
            }
            b"NK" => Some(Citizenship),
            b"NL" => Some(GroupDisabilityInsuranceMandatory),
            b"NM" => Some(RetailOrigination),
            b"NN" => Some(CodeNN),
            b"NO" => Some(ArmsLengthTransaction),
            b"NP" => Some(CertificationOfANonAttorneyBankruptcyPetitionPreparer),
            b"NQ" => Some(EligibleForTheFannieMaeNeighborsProgram),
            b"NR" => Some(NoRestrictions),
            b"NS" => Some(CodeNS),
            b"NT" => Some(LodgingProvided),
            b"NU" => Some(NotUsed),
            b"NV" => Some(ContractLabor),
            b"NW" => Some(BonusesPaid),
            b"NX" => Some(MinorsEmployed),
            b"NY" => Some(MeetsRequirementsForFannieMaeCommunitySecondsProgram),
            b"NZ" => Some(PurchaseIsAResultOfCurrentEmployerSponsoredRelocation),
            b"O0" => Some(TeachingMajor),
            b"O1" => Some(MultipleUnspecifiedInstances),
            b"O2" => Some(HiresPartTimeEmployeesAsNeeded),
            b"O3" => Some(MexicanRequest),
            b"O4" => Some(RiskManagementPlanRequiresPredictiveFiling),
            b"O5" => Some(SanitizedCopy),
            b"O6" => Some(CodeO6),
            b"O7" => Some(ToxicChemicalClaimedAsTradeSecret),
            b"O8" => Some(UnderControlOfReportingFacilityOrParentCompany),
            b"O9" => Some(WeatherConditionsNotKnown),
            b"OA" => Some(SellerProvidedBelowMarketSecondaryFinancing),
            b"OB" => Some(FixedSite),
            b"OC" => Some(MobileFacility),
            b"OD" => Some(TransferAuthorized),
            b"ODZ" => Some(OccupationalDisease),
            b"OE" => Some(TransferComplete),
            b"OF" => Some(CommercialDriversLicenseVerified),
            b"OG" => Some(ResponsibilityAccepted),
            b"OH" => Some(WaterbodyInvolved),
            b"OI" => Some(ChargesPending),
            b"OJ" => Some(DriverHasProperLicenseClass),
            b"OK" => Some(DriverCompliantWithLicenseRestrictions),
            b"OL" => Some(OtherLimitation),
            b"OM" => Some(DriverHasCommercialDriversLicense),
            b"ON" => Some(DriverHasMedicalWaiver),
            b"OO" => Some(OwnOtherFederalHousingAdministrationProperty),
            b"OP" => Some(OutOfRangeProductTemperature),
            b"OQ" => Some(PhotographsTaken),
            b"OR" => Some(OtherRestrictions),
            b"OS" => Some(OutOfService),
            b"OT" => Some(Oriented),
            b"OU" => Some(PoliceOfficerAtScene),
            b"OV" => Some(Overridden),
            b"OW" => Some(Proposed),
            b"OX" => Some(RatingIsAffected),
            b"OY" => Some(CodeOY),
            b"OZ" => Some(LiabilityIsContingentOrHasACoSigner),
            b"P0" => Some(TerminalDegree),
            b"P1" => Some(PatientWasDischargedFromTheFirstFacility),
            b"P2" => Some(PatientWasAdmittedToTheSecondFacility),
            b"P3" => Some(PropertyHasAFamilyRoomOrDen),
            b"P4" => Some(PropertyHasCentralAirConditioning),
            b"P5" => Some(PropertyTypicalOfNeighborhood),
            b"P6" => Some(PropertyDeferredMaintenanceTypicalOfNeighborhood),
            b"P7" => Some(AcceptingExistingPatients),
            b"P8" => Some(AcceptingNewPatients),
            b"P9" => Some(PropertyIntendedToBeOccupiedAsPrimaryResidence),
            b"PA" => Some(Paralysis),
            b"PB" => Some(PhoneSkipBegin),
            b"PC" => Some(PlanIsAttached),
            b"PD" => Some(PhoneSkipResolved),
            b"PE" => Some(PhoneSkipExhaust),
            b"PF" => Some(PaidOutsideOfClosing),
            b"PFB" => Some(PreviouslyFailedBoardCertification),
            b"PG" => Some(ProjectIsSubjectToGroundRent),
            b"PH" => Some(Prepayable),
            b"PI" => Some(Program),
            b"PJ" => Some(ProviderIsParticipating),
            b"PK" => Some(PreliminaryFloodDetermination),
            b"PL" => Some(ProviderCertificationInTheTaxonomyHasBeenVerified),
            b"PM" => Some(ProjectAndServicesBudgetIsMaintained),
            b"PN" => Some(AtypicalPhysicalCondition),
            b"PO" => Some(PersonalPropertyOnsite),
            b"PP" => Some(PropertyPreviouslyWinterized),
            b"PQ" => Some(LiabilityWillBeResubordinatedToTheLoanUponClosing),
            b"PR" => Some(Poor),
            b"PRD" => Some(PriorDamage),
            b"PS" => Some(PublicationIsIncludedInSharing),
            b"PT" => Some(ProjectIsComplete),
            b"PU" => Some(NotPaid),
            b"PV" => Some(PropertyVacant05Percent),
            b"PW" => Some(PartialWeightBearing),
            b"PX" => Some(PaidByBorrowerBeforeClosing),
            b"PY" => Some(PropertyForSale),
            b"PZ" => Some(PropertyVacantOver5Percent),
            b"Q0" => Some(Veteran),
            b"Q1" => Some(ExportProduct),
            b"Q2" => Some(CodeQ2),
            b"Q3" => Some(USGoodsReturned),
            b"Q4" => Some(CandidateForUSCustomsServiceProtest),
            b"Q5" => Some(DomesticProduct),
            b"Q6" => Some(PriorApprovalLetterAndOfficialOrdersOnFile),
            b"Q7" => Some(ImportersSubstantiatingStatementAndContractAreOnFile),
            b"Q8" => Some(InternationalTransportMovement),
            b"Q9" => Some(PieceCountShouldBeIncludedInTheTotalPackingListQuantity),
            b"QA" => Some(ShipmentShouldBeHeldAtThePort),
            b"QB" => Some(MultipleStatesOfOriginForThisItem),
            b"QC" => Some(MultipleCountriesOfOriginForThisItem),
            b"QD" => Some(LetterOfCreditRestrictedToASpecificBank),
            b"QE" => Some(LetterOfCreditPermitsTransshipment),
            b"QF" => Some(LetterOfCreditCoversPartialShipments),
            b"QG" => Some(DutiableItem),
            b"QH" => Some(AmountsShouldBeProRatedAcrossLineItems),
            b"QI" => Some(CodeQI),
            b"QJ" => Some(VisaRequiredForThisItem),
            b"QK" => Some(ItemSubjectToQuotas),
            b"QL" => Some(CodeQL),
            b"QM" => Some(ItemIsASet),
            b"QN" => Some(ItemIsAnEnsemble),
            b"QO" => Some(ItemIsAMetalItem),
            b"QP" => Some(ItemIsAMachinePart),
            b"QQ" => Some(ItemIsAHazardousItem),
            b"QR" => Some(CodeQR),
            b"QS" => Some(QuantityToBeImportedHasBeenApprovedByTheNecessaryAgencies),
            b"QT" => Some(FilingDataIsToBeWithheldFromPublicInspection),
            b"QU" => Some(PropertyTypeCooperative),
            b"QV" => Some(PaidByBorrowerAtClosing),
            b"QW" => Some(PaidByOtherAtOrBeforeClosing),
            b"QX" => Some(TreatedAsAReductionToIncome),
            b"QY" => Some(CodeQY),
            b"QZ" => Some(CodeQZ),
            b"R0" => Some(ExemptFromPublicRecordsLaw),
            b"R1" => Some(DebtorHoldsClaimToRealProperty),
            b"R2" => Some(EntityClaimsToHoldASecuredInterest),
            b"R3" => Some(DebtorHasPropertyOfTheTypeSpecified),
            b"R4" => Some(DebtorElectsTheStateExemption),
            b"R5" => Some(DebtorElectsTheFederalExemption),
            b"R6" => Some(CoDebtorMayBeJointlyLiable),
            b"R7" => Some(ClaimIsContingent),
            b"R8" => Some(ClaimIsUnliquidated),
            b"R9" => Some(ClaimIsDisputed),
            b"RA" => Some(ReferenceTelephoneAttempt),
            b"RB" => Some(DebtorHasNoCreditorsHoldingUnsecuredPriorityClaims),
            b"RC" => Some(ReferenceTelephoneContact),
            b"RCA" => Some(RentalCarArranged),
            b"RD" => Some(RentDelinquent),
            b"RE" => Some(ClaimIsSubjectToSetoff),
            b"RF" => Some(DebtorHasNoExecutoryContractsOrUnexpiredLeases),
            b"RG" => Some(LeaseIsForNonresidentialRealProperty),
            b"RH" => Some(DebtorHasNoCoDebtors),
            b"RI" => Some(DebtorIsMarried),
            b"RJ" => Some(DebtorsSpouseMaintainsASeparateHousehold),
            b"RK" => Some(RealEstateTaxesAreIncluded),
            b"RL" => Some(PropertyInsuranceIsIncluded),
            b"RM" => Some(DebtorHasNoCreditorsHoldingSecuredClaims),
            b"RN" => Some(RentControl),
            b"RO" => Some(EquipmentIsRebuilt),
            b"RP" => Some(IndividualInjuredInPerformanceOfDuty),
            b"RQ" => Some(IndividualInjuredByThirdParty),
            b"RR" => {
                Some(
                    QualityOfManagementAndItsEnforcementOfRulesAndRegulationsBasedOnGeneralAppearances,
                )
            }
            b"RS" => Some(PayContinued),
            b"RT" => Some(SickLeaveTaken),
            b"RU" => Some(SignatureOnFile),
            b"RV" => Some(LowRefrigerantCapacityShutdown),
            b"RW" => Some(RecentDefrost),
            b"RX" => Some(RatedHorsepowerCanBeProduced),
            b"RY" => Some(ForeignMilitarySale),
            b"RZ" => Some(WaiverOfPriorNotice),
            b"S0" => Some(AlternateCertificationProgramParticipant),
            b"S1" => {
                Some(ServicesProvidedAtTheSecondFacilityWereAvailableAtTheFirstFacility)
            }
            b"S2" => Some(UnderTreatment),
            b"S3" => Some(FirstTimeVacant),
            b"S4" => Some(AdverseEasement),
            b"S5" => Some(DisclosureIndicator),
            b"S6" => Some(AtypicalOffSiteImprovements),
            b"S7" => Some(ToxicSubstances),
            b"S8" => Some(AdverseEncroachment),
            b"S9" => Some(AtypicalFunctionalCondition),
            b"SA" => Some(SubjectPropertyIsCurrentlyListed),
            b"SB" => Some(DebtorIsASmallBusinessAsDefinedIn11USCSection101),
            b"SC" => Some(SpecialServicesAreMobileHomeOnly),
            b"SD" => Some(SpecialServicesAreLeaseholdOrMobileHomeOrBoth),
            b"SE" => Some(CodeSE),
            b"SF" => Some(SensorFault),
            b"SG" => Some(StreetLightsArePublic),
            b"SH" => Some(SpecialServicesAreLeaseholdOrSubleaseholdOrBoth),
            b"SI" => Some(HazardousWaste),
            b"SJ" => Some(PestInfestation),
            b"SK" => Some(RoadMaintenanceRequired),
            b"SL" => Some(SpeechLimitations),
            b"SM" => Some(CurrentlyServingInMilitary),
            b"SN" => Some(MajorBaseSupport),
            b"SO" => Some(CriticalSupportLevelMet),
            b"SP" => Some(StreetIsPublic),
            b"SPP" => Some(SpecialtyIsPrimary),
            b"SPS" => Some(SpecialtyIsSecondary),
            b"SQ" => Some(LocalWagesInEffect),
            b"SR" => Some(FederalWorkerDisplacement),
            b"SS" => Some(AdverseZoning),
            b"ST" => Some(NewServicesRequested),
            b"SU" => Some(ContinuedServicesRequested),
            b"SUB" => Some(SubrogationOpen),
            b"SV" => Some(MajorCorporationHighTech),
            b"SW" => Some(SidewalkIsPublic),
            b"SX" => Some(CollectiveBargainingAgreementSentByMail),
            b"SY" => Some(CollectiveBargainingAgreementSentByFacsimile),
            b"SZ" => Some(Contract),
            b"T0" => Some(UnderContract),
            b"T1" => Some(RoadTestPerformedWithNoProblemsReported),
            b"T2" => Some(RoadTestPerformedWithProblemsReported),
            b"T3" => Some(TiresBrandMatch),
            b"T4" => Some(RealEstateTaxesAreCurrent),
            b"T5" => Some(HazardInsuranceIsCurrent),
            b"T6" => Some(TerminateGuarantee),
            b"T7" => Some(AtypicalExternalCondition),
            b"T8" => Some(CodeT8),
            b"T9" => Some(UtilitiesInadequate),
            b"TA" => Some(CollectiveBargainingAgreementSentByElectronicBulletinBoard),
            b"TB" => Some(DebtorHasNoCreditorsHoldingUnsecuredNonpriorityClaims),
            b"TC" => Some(TransportViaCargoAircraft),
            b"TD" => Some(AnnualLeaveTaken),
            b"TE" => Some(ItemIsSpecialTestEquipment),
            b"TF" => Some(OperatesAsRepresentativeForOthers),
            b"TG" => Some(ClaimInvolvesWorkRelatedDeath),
            b"TH" => Some(CodeTH),
            b"TI" => Some(EmployeeHasNotRecoveredToReturnToWork),
            b"TJ" => Some(EmployeeHasRetired),
            b"TK" => Some(EmployeeHasResigned),
            b"TL" => Some(EmployeeIsPermanentlyAndTotallyDisabled),
            b"TM" => Some(TractionMotorIsCutOut),
            b"TN" => Some(AtypicalQualityOfConstruction),
            b"TNJ" => Some(TraumaticInjury),
            b"TO" => Some(AtypicalRemodeling),
            b"TP" => Some(TransportViaPassengerAircraft),
            b"TQ" => Some(AtypicalAdditions),
            b"TR" => Some(CodeTR),
            b"TS" => Some(AdverseMarketingConditionsInSubjectPropertysNeighborhood),
            b"TT" => Some(NeighborhoodWaterSourceIsPublic),
            b"TU" => Some(NeighborhoodSewageTreatmentIsPublic),
            b"TV" => Some(TelephoneNumberVerified),
            b"TW" => Some(NeighborhoodStreetIsPublic),
            b"TX" => Some(OtherMiscellaneousAdverseCharacteristics),
            b"TY" => Some(SubjectPropertysStreetIsPublic),
            b"TZ" => Some(SubjectPropertysSewageTreatmentIsPublic),
            b"U0" => Some(Disability),
            b"U1" => Some(MinimalChange),
            b"U2" => Some(NeatAppearance),
            b"U3" => Some(NetWorthComputedAfterExemptions),
            b"U4" => Some(NetWorthConsiderablyHigher),
            b"U5" => Some(NetWorthHigher),
            b"U6" => Some(NoEmployees),
            b"U7" => Some(NoEmployeesBusinessManagedByOwner),
            b"U8" => Some(NoEmployeesBusinessManagedByPartners),
            b"U9" => Some(NotOutOfBusiness),
            b"UA" => Some(CodeUA),
            b"UB" => Some(ConductedAtAProfit),
            b"UC" => Some(ContingentDebtIndicated),
            b"UD" => Some(Continue),
            b"UE" => Some(ContractsObtainedByBid),
            b"UF" => Some(ContractsObtainedByNegotiation),
            b"UG" => Some(ConvertedToHoldingCompany),
            b"UH" => Some(CrossClaimFiled),
            b"UI" => Some(DecliningTendency),
            b"UJ" => Some(CodeUJ),
            b"UK" => Some(CodeUK),
            b"UL" => Some(DownOrDeclineOrDecreased),
            b"UM" => Some(EmployeesIncludeOfficers),
            b"UN" => Some(Uncooperative),
            b"UO" => Some(EmployeesIncludeOwners),
            b"UP" => Some(EmployeesIncludePartners),
            b"UQ" => Some(EmployeesIncludeTemporaryWorkers),
            b"UR" => Some(EmployeesVaryAccordingToNeeds),
            b"US" => Some(Enclosed),
            b"UT" => Some(UpAsTolerated),
            b"UU" => Some(CodeUU),
            b"UV" => Some(FavorablePersonalReputation),
            b"UW" => Some(FiguresAreAbbreviated),
            b"UX" => Some(FiguresAreConvertedToAgencyFormat),
            b"UY" => Some(FiguresAreIndividual),
            b"UZ" => Some(FiguresAreRestated),
            b"V0" => Some(UltimateParentCompanyFinancialStatementUsed),
            b"V1" => Some(ValidBorrowerAddressOrPhoneAttemptWithSchoolAttended),
            b"V2" => Some(LenderDeterminedBorrowerMovedOutOfState),
            b"V3" => Some(LenderDeterminedBorrowerMovedBackIntoState),
            b"V4" => Some(LenderDeterminedBorrowerIncarcerated),
            b"V5" => Some(LenderDeterminedBorrowerNoLongerIncarcerated),
            b"V6" => Some(Original),
            b"V7" => Some(TrueAndExactCopy),
            b"V8" => Some(SubjectPropertysWaterSourceIsPublic),
            b"V9" => Some(PicturesRequired),
            b"VA" => Some(IntercompanyRelationsExist),
            b"VB" => Some(InventoryValuedAtLowerOfCostOrMarket),
            b"VC" => Some(InventoryValuedAtOtherMethods),
            b"VD" => Some(OperatesAsSoleAgent),
            b"VE" => Some(WithoutPersonalJudgment),
            b"VF" => Some(WorkIsSubcontracted),
            b"VG" => Some(NotRegistered),
            b"VH" => Some(ImmediateAttentionRequired),
            b"VI" => Some(VehicleInspectionReportCompleted),
            b"VJ" => Some(MiddleToMedium),
            b"VK" => Some(RentControlLikely),
            b"VL" => Some(Furnished),
            b"VM" => Some(PriceRangeSingleFamilyOrPlannedUnitDevelopmentNotApplicable),
            b"VN" => Some(PriceRangeCondominiumNotApplicable),
            b"VO" => Some(PriceRangeTwoToFourFamilyNotApplicable),
            b"VP" => Some(FinancialFiguresAreProjectedBasedOnSales),
            b"VQ" => Some(FinancialFiguresAreProjectedBasedOnEmployees),
            b"VR" => Some(ParentCompanyHasBankruptcy),
            b"VS" => Some(HeadquartersHasBankruptcy),
            b"VT" => Some(CommercialMotorVehicleWasInvolvedInThisConviction),
            b"VTL" => Some(VehicleWasDeclaredATotalLoss),
            b"VU" => {
                Some(
                    CommercialMotorVehicleWasCarryingHazardousMaterialsWhenTheOffenseWasCommitted,
                )
            }
            b"VV" => Some(PreparedFromInternalBookFigures),
            b"VW" => Some(QuantityDeclined),
            b"VX" => Some(QuantityDetailsUnknown),
            b"VY" => Some(CodeVY),
            b"VZ" => Some(CodeVZ),
            b"W0" => Some(StatementIsOnATradingTrust),
            b"W1" => Some(NewRegistration),
            b"W2" => Some(MailingAddressChange),
            b"W3" => Some(ResidenceAddressChange),
            b"W4" => Some(NameChange),
            b"W5" => Some(PartyEnrollmentChange),
            b"W6" => Some(NeedsAbsenteeBallot),
            b"W7" => Some(WouldLikeToBeElectionDayWorker),
            b"W8" => Some(DuplicateRegistration),
            b"W9" => Some(ForwardedApplication),
            b"WA" => Some(WalkerRequired),
            b"WB" => Some(WaterOn),
            b"WC" => Some(ApplicationIncomplete),
            b"WD" => Some(VehiclePlateSurrendered),
            b"WE" => Some(WrittenNoticeToNoteHolder),
            b"WF" => Some(WrittenNoticeToBorrower),
            b"WG" => Some(WithinSpecifiedTimePeriod),
            b"WH" => Some(WithinSpecifiedRange),
            b"WI" => Some(InjuryWasWorkRelated),
            b"WJ" => Some(DealerPricingAuthorization),
            b"WK" => Some(SummaryLevel),
            b"WL" => Some(DetailLevel),
            b"WM" => Some(NonOccupantCoBorrower),
            b"WN" => Some(CodeWN),
            b"WO" => Some(EquipmentInWorkingOrder),
            b"WP" => Some(ToBeWatched),
            b"WQ" => Some(UndeterminedOutOfBusinessStatus),
            b"WR" => Some(WheelchairRequired),
            b"WS" => Some(BalanceSheetFiled),
            b"WT" => Some(WinterizedTagObserved),
            b"WU" => Some(MaterialSafetyDataSheet),
            b"WV" => Some(AcceptsCreditCards),
            b"WW" => Some(AllPurchasesMadeFromHeadquarters),
            b"WX" => Some(Busy),
            b"WY" => Some(Excessive),
            b"WZ" => Some(FairlyNew),
            b"X0" => Some(CodeX0),
            b"X1" => Some(GrossWeeklyAmountIsEstimated),
            b"X2" => Some(WaitingPeriodDisabilityDaysAreNonConsecutive),
            b"X3" => Some(CodeX3),
            b"X4" => Some(PermanentImpairmentPaidAtMinimum),
            b"X5" => Some(EmployeesDeathIsAResultOfWorkInjuryOrIllness),
            b"X6" => Some(EmployeesWrittenSocialSecurityNumberReleaseIsOnFile),
            b"X7" => Some(EmployeesMedicalRecordsReleaseAuthorizationIsOnFile),
            b"X8" => Some(EmployeeReturnedToWorkWithPreInjuryEmployer),
            b"X9" => Some(CodeX9),
            b"XA" => Some(FiguresAreAverage),
            b"XB" => Some(Imports),
            b"XC" => Some(InProcessOfEstablishing),
            b"XD" => Some(IntercompanyRelationsConsistOfEndorsements),
            b"XE" => Some(IntercompanyRelationsConsistOfGuarantees),
            b"XF" => Some(IntercompanyRelationsConsistOfLeasingArrangements),
            b"XG" => Some(IntercompanyRelationsConsistOfSharingAccounting),
            b"XH" => Some(IntercompanyRelationsConsistOfSharingFacilities),
            b"XI" => Some(IntercompanyRelationsConsistOfSharingManagement),
            b"XJ" => Some(IntercompanyRelationsConsistOfSharingPersonnel),
            b"XK" => Some(CodeXK),
            b"XL" => Some(CodeXL),
            b"XM" => Some(InventoryValuedAtCompanysEstimates),
            b"XN" => Some(InventoryValuedAtCost),
            b"XO" => Some(CodeXO),
            b"XP" => Some(JointOwnership),
            b"XQ" => Some(LeasesWithNoRentPayments),
            b"XR" => Some(LeasesWithOptionToBuy),
            b"XS" => Some(LeasesWithTokenPayment),
            b"XT" => Some(Limited),
            b"XU" => Some(LocatedForSeveralYears),
            b"XV" => Some(LocatedSinceOpening),
            b"XW" => Some(Modern),
            b"XX" => Some(NonExistent),
            b"XY" => Some(OfficerOrOwnerInOtherBusinessesInTheSameField),
            b"XZ" => Some(OperatesAsADistributorForOthers),
            b"Y0" => Some(InsuredCooperative),
            b"Y1" => Some(WorkedInIndustryForSeveralYears),
            b"Y2" => Some(AircraftOperation),
            b"Y3" => Some(AllClassificationsOnPolicyAccountedFor),
            b"Y4" => Some(BoardProvided),
            b"Y5" => Some(CasualLabor),
            b"Y6" => Some(CertificatesOnFileForAllSubcontractors),
            b"Y7" => Some(CommissionsPaid),
            b"Y8" => Some(ConditionOrTypeOfRecordsCauseAdditionalAuditTime),
            b"Y9" => Some(DomesticWorkersEmployed),
            b"YA" => Some(OperatesFromResidence),
            b"YB" => Some(OperatesUnderLicenseByOthers),
            b"YC" => Some(RentsFromMonthToMonth),
            b"YD" => Some(SemiModern),
            b"YE" => Some(UnderConstruction),
            b"YF" => Some(Unlimited),
            b"YG" => Some(Used),
            b"YH" => Some(Variable),
            b"YI" => Some(HolderIsASubsidiaryOfReportingAgent),
            b"YJ" => Some(ContactIsUnchangedFromPreviousReport),
            b"YK" => Some(ReportWasFiledLastYearByThisAgent),
            b"YL" => Some(PartyIsAuthorizedToDoBusinessInThisState),
            b"YM" => Some(ClearDecrease),
            b"YN" => Some(EmployeesTemporarilyLaidOff),
            b"YO" => Some(EstablishedInTheIndustry),
            b"YP" => Some(GlobalBusiness),
            b"YQ" => Some(InformationToBeFollowedUp),
            b"YR" => Some(KnownDetailsAreListed),
            b"YS" => Some(LandIsRented),
            b"YT" => Some(Low),
            b"YU" => Some(PrimeCommercialArea),
            b"YV" => Some(CodeYV),
            b"YW" => Some(SlightlyHigher),
            b"YX" => Some(SlightlyLower),
            b"YY" => Some(Stagnant),
            b"YZ" => Some(TerritoryInformationIsAvailable),
            b"Z0" => Some(SubcontractorsUsed),
            b"Z1" => Some(InsuredIsASubcontractor),
            b"Z2" => Some(InsuredHasMultipleEntries),
            b"Z3" => Some(InsuredHasRetailOperations),
            b"Z4" => Some(CodeZ4),
            b"Z5" => Some(OwnerOrOfficerInterviewed),
            b"Z6" => Some(PremiumOvertimeExcluded),
            b"Z7" => Some(CodeZ7),
            b"Z8" => Some(RecordsSatisfactoryForAudit),
            b"Z9" => Some(RelativesEmployed),
            b"ZA" => Some(CustomerConfigurationChangeIsRequired),
            b"ZB" => Some(CodeZB),
            b"ZC" => Some(CodeZC),
            b"ZD" => Some(CodeZD),
            b"ZE" => Some(RepairIsMissionEssential),
            b"ZF" => Some(RepairIsSafetyEssential),
            b"ZG" => Some(PeriodicMaintenanceIsRequired),
            b"ZH" => Some(CodeZH),
            b"ZI" => Some(ProgressIsInJeopardy),
            b"ZJ" => Some(EmployeesInjuryOrIllnessIsWorkRelated),
            b"ZK" => Some(FinalConfigurationChangeIsRequired),
            b"ZL" => Some(FinalDeliveryToShopIsRequired),
            b"ZM" => Some(FinalRequestorWorkforceWillAssist),
            b"ZN" => Some(JobIsLevel2),
            b"ZO" => Some(PreliminaryConfigurationChangeIsRequired),
            b"ZP" => Some(PreliminaryDeliveryToShopIsRequired),
            b"ZQ" => Some(PreliminaryRequestorWorkforceWillAssist),
            b"ZR" => Some(ConfigurationChangeIsAssociatedWithTimeMeter),
            b"ZS" => Some(ShopHasLeadResponsibility),
            b"ZT" => Some(EstimateIsDerivedFromJobTemplate),
            b"ZU" => Some(RequestorHoldsTechnicalDocumentation),
            b"ZV" => Some(ReplacementItem),
            b"ZW" => Some(CodeZW),
            b"ZX" => Some(NonConvertible),
            b"ZY" => Some(CodeZY),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ConditionIndicator::*;
        match self {
            Requested => "Requested",
            InProgress => "In Progress",
            Code0A => {
                "Automated Export System - Post Departure Authorized Special Status (AES-PASS) Standard"
            }
            Code0B => {
                "Automated Export System - Post Departure Authorized Special Status (AES-PASS) Expanded"
            }
            Code0C => {
                "Automated Export System - Post Departure Authorized Special Status (AES-PASS) Post Departure"
            }
            FacilitysEmergencyResponsePlanIncludesInformationOnEmergencyHealthCare => {
                "Facility's Emergency Response Plan Includes Information on Emergency Health Care"
            }
            FacilitysEmergencyResponsePlanIncludesProceduresForInformingPublicAndLocalAgenciesResponsibleForRespondingToAnAccidentalRelease => {
                "Facility's Emergency Response Plan Includes Procedures for Informing Public and Local Agencies Responsible for Responding to an Accidental Release"
            }
            FacilityHasACleanAirActTitleVOperatingPermit => {
                "Facility has a Clean Air Act Title V Operating Permit"
            }
            FacilityHasAWrittenEmergencyResponsePlan => {
                "Facility has a Written Emergency Response Plan"
            }
            FacilityHasReportableAccidents => "Facility has Reportable Accidents",
            Code0I => {
                "Facility is Covered by the Emergency Planning and\nCommunity Right to Know Act Section 302"
            }
            Code0J => {
                "Facility is Covered by the Occupational Safety and\nHealth Act (OSHA) Process Safety Management\nStandard"
            }
            Code0K => "Facility is Included in the Community Emergency\nResponse Plan",
            Code0L => {
                "Hazardous Waste Mixed with Resource Conservation Recovery Act (RCRA)-Radioactive Material"
            }
            OffsiteRespondersNotified => "Offsite Responders Notified",
            PrecipitationPresent => "Precipitation Present",
            DisabledVeteran => "Disabled Veteran",
            ServicerHasAdvancedFundsToPayForDelinquentTaxesOnNonEscrowedMortgage => {
                "Servicer has Advanced Funds to Pay for Delinquent Taxes on Non-escrowed Mortgage"
            }
            PropertyHasFireInsuranceOnlyThatWasNotLenderPlaced => {
                "Property Has Fire Insurance Only that was not Lender Placed"
            }
            ReportedButUnconfirmed => "Reported but Unconfirmed",
            HasSmokeAlarms => "Has Smoke Alarms",
            OperatesAsAHoldingCompany => "Operates as a Holding Company",
            Optimum => "Optimum",
            Renewed => "Renewed",
            HighestEducationalLevel => "Highest Educational Level",
            PrincipalCertificate => "Principal Certificate",
            InserviceEducationCompleted => "Inservice Education Completed",
            MainAssignment => "Main Assignment",
            PatientWasAdmittedToAHospital => "Patient was admitted to a hospital",
            PatientIsReceivingAntiFungalTherapy => {
                "Patient is receiving anti-fungal therapy"
            }
            PropertyIsOccupiedByOwner => "Property is occupied by owner",
            PropertyIsOccupiedByTenant => "Property is occupied by tenant",
            PropertyIsVacant => "Property is vacant",
            LocationIsUrban => "Location is urban",
            LocationIsSuburban => "Location is suburban",
            LocationIsRural => "Location is rural",
            Code1H => "Built-up over 75%",
            Code1I => "Built-up 25 - 75%",
            Code1J => "Built-up under 25%",
            GrowthRateIsRapid => "Growth rate is rapid",
            ClassILeft => "Class I-Left",
            GrowthRateIsStable => "Growth rate is stable",
            GrowthRateIsSlow => "Growth rate is slow",
            PropertyValuesAreIncreasing => "Property values are increasing",
            PropertyValuesAreStable => "Property values are stable",
            PropertyValuesAreDeclining => "Property values are declining",
            ClassIRight => "Class I-Right",
            DemandOrSupplyIsInShortage => "Demand or supply is in shortage",
            DemandOrSupplyIsInBalance => "Demand or supply is in balance",
            DemandOrSupplyIsOverSupply => "Demand or supply is over supply",
            MarketingTimeIsUnder3Months => "Marketing time is under 3 months",
            MarketingTimeIs3To6Months => "Marketing time is 3 to 6 months",
            MarketingTimeIsOver6Months => "Marketing time is over 6 months",
            PredominantOccupancyIsTheOwner => "Predominant occupancy is the owner",
            PredominantOccupancyIsTheTenant => "Predominant occupancy is the tenant",
            PatientWasBedConfinedBeforeTheAmbulanceService => {
                "Patient was bed confined before the ambulance service"
            }
            PatientIsReceivingOralAntiFungalTherapy => {
                "Patient is receiving oral anti-fungal therapy"
            }
            Code2B => "Predominant occupancy is vacant (0-5%)",
            Code2C => "Predominant occupancy is vacant (over 5%)",
            DeveloperOrBuilderIsInControlOfTheHomeOwnersAssociation => {
                "Developer or builder is in control of the Home Owners Association"
            }
            SiteIsACornerLot => "Site is a corner lot",
            ZoningComplianceIsLegal => "Zoning compliance is legal",
            Code2G => "Zoning compliance is legal nonconforming (grandfather use)",
            ZoningComplianceIsIllegal => "Zoning compliance is illegal",
            ThereIsNoZoning => "There is no zoning",
            HighestAndBestUseAsImprovedIsThePresentUse => {
                "Highest and best use as improved is the present use"
            }
            HighestAndBestUseAsImprovedIsOtherUse => {
                "Highest and best use as improved is other use"
            }
            ClassIiLeft => "Class II-Left",
            PropertyIsLocatedInAFederalEmergencyManagementAdministrationSpecialFloodHazardArea => {
                "Property is located in a Federal Emergency Management Administration special flood hazard area"
            }
            Code2N => "Appraisal is made \"as is\"",
            Code2O => {
                "Appraisal is made subject to the repairs, alterations, inspections, or conditions listed"
            }
            AppraisalIsMadeSubjectToTheCompletionPerPlansAndSpecifications => {
                "Appraisal is made subject to the completion per plans and specifications"
            }
            Code2Q => "Project type is planned unit development (PUD)",
            ClassIiRight => "Class II-Right",
            ProjectTypeIsCondominium => "Project type is condominium",
            PropertyRightsAreFeeSimple => "Property rights are fee simple",
            PropertyRightsAreLeasehold => "Property rights are leasehold",
            SupervisorAppraiserInspectedThePropertyPerSupervisoryAppraisersCertification => {
                "Supervisor appraiser inspected the property per supervisory appraiser's certification"
            }
            PropertyWasSoldWithinLast12Months => {
                "Property was sold within last 12 months"
            }
            AppraiserSignedStatementOfLimitingConditionsAndDisclaimer => {
                "Appraiser signed statement of limiting conditions and disclaimer"
            }
            OwnershipInterestInAProperty => "Ownership interest in a property",
            Termination => "Termination",
            PatientWasBedConfinedAfterTheAmbulanceService => {
                "Patient was bed confined after the ambulance service"
            }
            PatientIsReceivingTopicalAntiFungalTherapy => {
                "Patient is receiving topical anti-fungal therapy"
            }
            PointsPaidBySeller => "Points Paid by Seller",
            PointsPaidByBuyer => "Points Paid by Buyer",
            SellerConcession => "Seller Concession",
            LetterOfCertification => "Letter of Certification",
            VerbalReportNeeded => "Verbal Report Needed",
            AnyRelationshipBetweenOwnerAndOccupant => {
                "Any Relationship Between Owner and Occupant"
            }
            MapAndDirectionsToRemotePropertiesToFollow => {
                "Map and Directions to Remote Properties to Follow"
            }
            GroundLeaseToFollow => "Ground Lease to Follow",
            DisclosureStatementToFollow => "Disclosure Statement to Follow",
            CopyOfPropertyListingToFollow => "Copy of Property Listing to Follow",
            ClassIiiLeft => "Class III-Left",
            CopyOfTitleReportPlatMapToFollow => "Copy of Title Report Plat Map to Follow",
            PropertyTaxBillToFollow => "Property Tax Bill to Follow",
            EngineeringOrSoilReportToFollow => "Engineering or Soil Report to Follow",
            SalesContractAvailable => "Sales Contract Available",
            LeaveWillBeTaken => "Leave Will be Taken",
            ClassIiiRight => "Class III-Right",
            Approved => "Approved",
            BalanceSheetDoesNotBalance => "Balance Sheet does not balance",
            BankingDoneThroughParentCompany => "Banking done through Parent Company",
            BankingDoneThroughRelatedConcern => "Banking done through Related Concern",
            BankingDoneThroughSubsidiary => "Banking done through Subsidiary",
            CanNotDetermineIfSubjectEngagedInBusiness => {
                "Can not determine if subject engaged in business"
            }
            Deteriorated => "Deteriorated",
            DetrimentalInformationReceived => "Detrimental Information Received",
            PatientWasMovedByStretcher => "Patient was moved by stretcher",
            ServicesAreRenderedWithinHospiceElectedPeriodOfCoverage => {
                "Services are rendered within Hospice-elected period of coverage"
            }
            Accidents => "Accidents",
            AccountRepresentativeTransfer => "Account Representative Transfer",
            AdditionalCoverage => "Additional Coverage",
            AdviceToStop => "Advice to Stop",
            AgentReplacement => "Agent Replacement",
            BackupWithholding => "Backup Withholding",
            CurrentEmployer => "Current Employer",
            CurrentOccupation => "Current Occupation",
            EmployerReimbursement => "Employer Reimbursement",
            Code4K => "Employee Retirement Income Security Act (ERISA)",
            ExpectedChanges => "Expected Changes",
            Experimental => "Experimental",
            ForeignFlight => "Foreign Flight",
            FutureInvolvement => "Future Involvement",
            Code4P => "Grounding, Fine, Reprimand",
            GroupDisabilityInsuranceConversion => "Group Disability Insurance Conversion",
            GroupDisabilityInsuranceOffset => "Group Disability Insurance Offset",
            GroupDisabilityInsuranceParticipation => {
                "Group Disability Insurance Participation"
            }
            GroupDisabilityInsuranceTopUp => "Group Disability Insurance Top Up",
            HomeEmployment => "Home Employment",
            InformationOmitted => "Information Omitted",
            InjuryBenefits => "Injury Benefits",
            IssueAtHigherPremiums => "Issue at Higher Premiums",
            IssueWithExclusions => "Issue With Exclusions",
            IssueWithoutBenefits => "Issue Without Benefits",
            PatientWasUnconsciousOrInShock => "Patient was unconscious or in shock",
            TreatmentIsRenderedRelatedToTheTerminalIllness => {
                "Treatment is rendered related to the terminal illness"
            }
            Code5B => "Certified Aftermarket Parts Association (CAPA) Only",
            Code5C => "Certified Aftermarket Parts Association (CAPA) Preferred",
            JuvenileSeen => "Juvenile Seen",
            MedicalTreatment => "Medical Treatment",
            MilitaryAviation => "Military Aviation",
            NewGroup => "New Group",
            OtherCoverageOffset => "Other Coverage Offset",
            OtherPrincipalsBeingInsured => "Other Principals Being Insured",
            OwnerActiveInBusiness => "Owner Active in Business",
            PayrollDeduction => "Payroll Deduction",
            Prepaid => "Prepaid",
            PreviousApplication => "Previous Application",
            PrimaryOccupation => "Primary Occupation",
            RacingAccident => "Racing Accident",
            Replacement => "Replacement",
            ResidesWithApplicant => "Resides With Applicant",
            GenderDistinct => "Gender Distinct",
            SiblingCoverage => "Sibling Coverage",
            SicknessBenefits => "Sickness Benefits",
            SpecialDating => "Special Dating",
            SpousalConsent => "Spousal Consent",
            SuitabilityAnalysis => "Suitability Analysis",
            SuitableForCoverage => "Suitable for Coverage",
            Taxable => "Taxable",
            ThisCompanyReplacement => "This Company Replacement",
            PatientWasTransportedInAnEmergencySituation => {
                "Patient was transported in an emergency situation"
            }
            TreatmentIsRenderedByAHospiceEmployedPhysician => {
                "Treatment is rendered by a Hospice employed physician"
            }
            UnitedStatesCitizen => "United States Citizen",
            PermanentResidentAlien => "Permanent Resident Alien",
            BorrowerIsFirstTimeHomebuyer => "Borrower is First Time Homebuyer",
            UnemploymentClaims => "Unemployment Claims",
            UnemploymentInsuranceEligibility => "Unemployment Insurance Eligibility",
            WorkStatus => "Work Status",
            WorkersCompensationEligible => "Workers Compensation Eligible",
            FactoredOnRecourseBasis => "Factored on Recourse Basis",
            FactoredWithAdvances => "Factored with Advances",
            FiguresAreActual => "Figures are Actual",
            FiguresAreAnticipated => "Figures are Anticipated",
            FiguresAreEstimated => "Figures are Estimated",
            FiguresAreModified => "Figures are Modified",
            FiguresAreProjected => "Figures are Projected",
            GovernmentBusinessNumberUnavailable => {
                "Government Business Number Unavailable"
            }
            GoodwillOriginPurchasedFromBankruptCompany => {
                "Goodwill Origin Purchased from Bankrupt Company"
            }
            GoodwillOriginRented => "Goodwill Origin Rented",
            HasNoOwnership => "Has no ownership",
            Improved => "Improved",
            IntangiblesBreakdownAvailable => "Intangibles breakdown available",
            IntangiblesIncludeOrganizationalExpense => {
                "Intangibles include Organizational Expense"
            }
            IntercompanyRelationsConsistOfLoansAndAdvances => {
                "Intercompany relations consist of Loans and Advances"
            }
            IntercompanyRelationsConsistOfMerchandiseTransactions => {
                "Intercompany relations consist of Merchandise Transactions"
            }
            IntercompanyRelationsConsistOfServiceTransactions => {
                "Intercompany relations consist of Service Transactions"
            }
            LocalBankingUtilizedOnATransferAccountBasis => {
                "Local banking utilized on a transfer account basis"
            }
            PatientHadToBePhysicallyRestrained => {
                "Patient had to be physically restrained"
            }
            TreatmentIsRenderedByAPrivateAttendingPhysician => {
                "Treatment is rendered by a private attending physician"
            }
            MedicationsOrderedAreBeingAdministeredIntramuscularly => {
                "Medications Ordered are being Administered Intramuscularly"
            }
            MedicationsOrderedAreBeingAdministeredIntravenously => {
                "Medications Ordered are being Administered Intravenously"
            }
            MedicationsOrderedAreBeingAdministeredOrally => {
                "Medications Ordered are being Administered Orally"
            }
            MaintainsNoInventory => "Maintains no Inventory",
            MedicationsOrderedAreBeingAdministeredSubcutaneously => {
                "Medications Ordered are being Administered Subcutaneously"
            }
            Majority => "Majority",
            MarketableSecuritiesValuedAtCost => "Marketable Securities valued at cost",
            MarketableSecuritiesValuedAtLowerOfCostOrMarket => {
                "Marketable Securities valued at lower of cost or market"
            }
            InteriorAccessDenied => "Interior Access Denied",
            RepairsAreRecommended => "Repairs are Recommended",
            LoanOriginatedUnderSharedEquityPlan => {
                "Loan Originated under Shared Equity Plan"
            }
            TitleAndOrLegalIssuesExist => "Title and or Legal Issues Exist",
            EnvironmentalIssuesExist => "Environmental Issues Exist",
            PropertyIsListedAsIs => "Property is Listed As Is",
            PropertyIsListedAsRepaired => "Property is Listed as Repaired",
            VacancyRateIsGreaterThan5PercentTo10Percent => {
                "Vacancy Rate is Greater Than 5 Percent to 10 Percent"
            }
            VacancyRateIsGreaterThan10PercentTo20Percent => {
                "Vacancy Rate is Greater Than 10 Percent to 20 Percent"
            }
            VacancyRateIsGreaterThan20Percent => {
                "Vacancy Rate is Greater Than 20 Percent"
            }
            MostComparableProperty => "Most Comparable Property",
            AnticipateIssuesWhichAffectAbilityToSecureFinancing => {
                "Anticipate Issues which Affect Ability to Secure Financing"
            }
            PointsArePaidBySeller => "Points are Paid by Seller",
            PropertyCoveredByFloodInsurancePolicy => {
                "Property Covered by Flood Insurance Policy"
            }
            PropertyCoveredByEarthquakeInsurancePolicy => {
                "Property Covered by Earthquake Insurance Policy"
            }
            PointsAreNegotiable => "Points are Negotiable",
            PropertyIsCurrentlyListedWithARealEstateFirm => {
                "Property is Currently Listed with a Real Estate Firm"
            }
            PatientHadVisibleHemorrhaging => "Patient had visible hemorrhaging",
            TreatmentIsCurative => "Treatment is curative",
            IncomeOrAssetsOfAnotherUsed => "Income or Assets of Another Used",
            DisclosureOfSomeoneElsesLiabilitiesRequired => {
                "Disclosure of Someone Else's Liabilities Required"
            }
            Code8D => "Property Improvements \"to be made\"",
            Code8E => "Property Improvements \"have been made\"",
            DistantSuburban => "Distant Suburban",
            SelfEmployed => "Self Employed",
            LiabilityToBeSatisfied => "Liability to be Satisfied",
            AreAssetsLiabilitiesReportedJointly => {
                "Are Assets/Liabilities Reported Jointly"
            }
            LocationIsFarm => "Location is Farm",
            LocationIsResort => "Location is Resort",
            ShortageExistForCompetingListings => "Shortage Exist for Competing Listings",
            CompetingListingsAreInBalance => "Competing Listings are in Balance",
            OversupplyExistForCompetingListings => {
                "Oversupply Exist for Competing Listings"
            }
            IncentivesAreOffered => "Incentives are Offered",
            ListedPropertyHasBeenInspected => "Listed Property has been Inspected",
            SalePropertyHasBeenInspected => "Sale Property has been Inspected",
            GeneralMarketingConditionIsDepressed => {
                "General Marketing Condition is Depressed"
            }
            GeneralMarketingConditionIsSlow => "General Marketing Condition is Slow",
            GeneralMarketingConditionIsStatic => "General Marketing Condition is Static",
            GeneralMarketingConditionIsImproving => {
                "General Marketing Condition is Improving"
            }
            GeneralMarketingConditionIsExcellent => {
                "General Marketing Condition is Excellent"
            }
            EmploymentConditionsAreStable => "Employment Conditions are Stable",
            EmploymentConditionsAreDeclining => "Employment Conditions are Declining",
            EmploymentConditionsAreIncreasing => "Employment Conditions are Increasing",
            OverimprovementConditionExists => "Overimprovement Condition Exists",
            AmbulanceServiceWasMedicallyNecessary => {
                "Ambulance service was medically necessary"
            }
            TreatmentIsPalliative => "Treatment is Palliative",
            InvoluntaryCommittal => "Involuntary Committal",
            LackOfAvailableEquipment => "Lack of Available Equipment",
            LackOfAppropriateFacilityWithinReasonableDistanceToTreatPatientInTheEventOfComplications => {
                "Lack of Appropriate Facility within Reasonable Distance to Treat Patient in the Event of Complications"
            }
            SuddenOnsetOfDisorientation => "Sudden Onset of Disorientation",
            Code9F => "Sudden Onset of Severe, Incapacitating Pain",
            ContinuousHemorrhageFromAnySiteWithAbnormalLabValues => {
                "Continuous Hemorrhage from any Site with Abnormal Lab Values"
            }
            PatientRequiresIntensiveIvTherapy => "Patient Requires Intensive IV Therapy",
            PatientRequiresVolumeExpanders => "Patient Requires Volume Expanders",
            PatientRequiresProtectiveIsolation => "Patient Requires Protective Isolation",
            PatientRequiresFrequentMonitoring => "Patient Requires Frequent Monitoring",
            PatientRequiresExtendedPostOperativeObservation => {
                "Patient Requires Extended Post-operative Observation"
            }
            ForeclosureProceedingsHaveBegun => "Foreclosure Proceedings Have Begun",
            UnderimprovementConditionExists => "Underimprovement Condition Exists",
            MarketabilityOfPropertyIsExcellent => {
                "Marketability of Property is Excellent"
            }
            MarketabilityOfPropertyIsGood => "Marketability of Property is Good",
            MarketabilityOfPropertyIsFair => "Marketability of Property is Fair",
            MarketabilityOfPropertyIsPoor => "Marketability of Property is Poor",
            FeesAreCurrent => "Fees are Current",
            FeesIncludeTennis => "Fees Include Tennis",
            FeesIncludePool => "Fees Include Pool",
            FeesIncludeInsurance => "Fees Include Insurance",
            FeesIncludeLandscape => "Fees Include Landscape",
            FeesIncludeOtherAmenities => "Fees Include Other Amenities",
            MostLikelyBuyerIsOwnerOccupant => "Most Likely Buyer is Owner Occupant",
            MostLikelyBuyerIsInvestor => "Most Likely Buyer is Investor",
            PatientIsAmbulatory => "Patient is ambulatory",
            AmbulationIsImpairedAndWalkingAidIsUsedForTherapyOrMobility => {
                "Ambulation is Impaired and Walking Aid is Used for Therapy or Mobility"
            }
            PatientIsConfinedToABedOrChair => "Patient is confined to a bed or chair",
            PatientIsConfinedToARoomOrAnAreaWithoutBathroomFacilities => {
                "Patient is Confined to a Room or an Area Without Bathroom Facilities"
            }
            AmbulationIsImpairedAndWalkingAidIsUsedForMobility => {
                "Ambulation is Impaired and Walking Aid is Used for Mobility"
            }
            PatientConditionRequiresPositioningOfTheBodyOrAttachmentsWhichWouldNotBeFeasibleWithTheUseOfAnOrdinaryBed => {
                "Patient Condition Requires Positioning of the Body or Attachments Which Would Not be Feasible With the Use of an Ordinary Bed"
            }
            PatientNeedsATrapezeBarToSitUpDueToRespiratoryConditionOrChangeBodyPositionsForOtherMedicalReasons => {
                "Patient needs a trapeze bar to sit up due to respiratory condition or change body positions for other medical reasons"
            }
            PatientsAbilityToBreatheIsSeverelyImpaired => {
                "Patient's Ability to Breathe is Severely Impaired"
            }
            PatientConditionRequiresFrequentAndOrImmediateChangesInBodyPositions => {
                "Patient condition requires frequent and/or immediate changes in body positions"
            }
            PatientCanOperateControls => "Patient can operate controls",
            SiderailsAreToBeAttachedToAHospitalBedOwnedByTheBeneficiary => {
                "Siderails Are to be Attached to a Hospital Bed Owned by the Beneficiary"
            }
            PatientOwnsEquipment => "Patient owns equipment",
            MattressOrSiderailsAreBeingUsedWithPrescribedMedicallyNecessaryHospitalBedOwnedByTheBeneficiary => {
                "Mattress or Siderails are Being Used with Prescribed Medically Necessary Hospital Bed Owned by the Beneficiary"
            }
            PatientNeedsLiftToGetInOrOutOfBedOrToAssistInTransferFromBedToWheelchair => {
                "Patient Needs Lift to Get In or Out of Bed or to Assist in Transfer from Bed to Wheelchair"
            }
            PatientHasAnOrthopedicImpairmentRequiringTractionEquipmentWhichPreventsAmbulationDuringPeriodOfUse => {
                "Patient has an orthopedic impairment requiring traction equipment which prevents ambulation during period of use"
            }
            ItemHasBeenPrescribedAsPartOfAPlannedRegimenOfTreatmentInPatientHome => {
                "Item has been prescribed as part of a planned regimen of treatment in patient home"
            }
            PatientIsHighlySusceptibleToDecubitusUlcers => {
                "Patient is highly susceptible to decubitus ulcers"
            }
            PatientOrACareGiverHasBeenInstructedInUseOfEquipment => {
                "Patient or a care-giver has been instructed in use of equipment"
            }
            PatientHasPoorDiabeticControl => "Patient has poor diabetic control",
            A67HourNocturnalStudyDocuments30EpisodesOfApneaEachLastingMoreThan10Seconds => {
                "A 6-7 hour nocturnal study documents 30 episodes of apnea each lasting more than 10 seconds"
            }
            Code30 => "Without the equipment, the patient would require surgery",
            PatientHasHadATotalKneeReplacement => {
                "Patient has had a total knee replacement"
            }
            PatientHasIntractableLymphedemaOfTheExtremities => {
                "Patient has intractable lymphedema of the extremities"
            }
            PatientIsInANursingHome => "Patient is in a nursing home",
            PatientIsConscious => "Patient is conscious",
            ThisFeedingIsTheOnlyFormOfNutritionalIntakeForThisPatient => {
                "This Feeding is the Only Form of Nutritional Intake for This Patient"
            }
            PatientWasAdministeredPremix => "Patient was administered premix",
            OxygenDeliveryEquipmentIsStationary => {
                "Oxygen delivery equipment is stationary"
            }
            CertificationSignedByThePhysicianIsOnFileAtTheSuppliersOffice => {
                "Certification signed by the physician is on file at the supplier's office"
            }
            PatientHasMobilizingRespiratoryTractSecretions => {
                "Patient Has Mobilizing Respiratory Tract Secretions"
            }
            PatientOrCaregiverIsCapableOfUsingTheEquipmentWithoutTechnicalOrProfessionalSupervision => {
                "Patient or Caregiver is Capable of Using the Equipment Without Technical or Professional Supervision"
            }
            PatientOrCaregiverIsUnableToPropelOrLiftAStandardWeightWheelchair => {
                "Patient or Caregiver is Unable to Propel or Lift a Standard Weight Wheelchair"
            }
            PatientRequiresLegElevationForEdemaOrBodyAlignment => {
                "Patient Requires Leg Elevation for Edema or Body Alignment"
            }
            PatientWeightOrUsageNeedsNecessitateAHeavyDutyWheelchair => {
                "Patient Weight or Usage Needs Necessitate a Heavy Duty Wheelchair"
            }
            PatientRequiresRecliningFunctionOfAWheelchair => {
                "Patient Requires Reclining Function of a Wheelchair"
            }
            PatientIsUnableToOperateAWheelchairManually => {
                "Patient is Unable to Operate a Wheelchair Manually"
            }
            Code46 => {
                "Patient or Caregiver Requires Side Transfer into Wheelchair, Commode or Other"
            }
            AdvertisementRunCondition => "Advertisement Run Condition",
            IndividualPaidForLastDayWorked => "Individual Paid for Last Day Worked",
            FullWagesPaidForDateOfInjury => "Full Wages Paid for Date of Injury",
            CitationOrTicketIssued => "Citation or Ticket Issued",
            IndividualIsMemberOfPolicyholdersHousehold => {
                "Individual is Member of Policyholder's Household"
            }
            IndividualPermittedToUseVehicle => "Individual Permitted to Use Vehicle",
            IndividualWoreSeatbelt => "Individual Wore Seatbelt",
            ChildRestraintDeviceInVehicle => "Child Restraint Device in Vehicle",
            ChildRestraintDeviceUsed => "Child Restraint Device Used",
            IndividualInjured => "Individual Injured",
            IndividualTransportedToAnotherLocation => {
                "Individual Transported to Another Location"
            }
            Code58 => "Durable Medical Equipment (DME) Purchased New",
            Code59 => "Durable Medical Equipment (DME) Is Under Warranty",
            TransportationWasToTheNearestFacility => {
                "Transportation Was To the Nearest Facility"
            }
            EmployeeIsExempt => "Employee is Exempt",
            ClaimantIsCoveredOnTheEmployersLongTermDisabilityPlan => {
                "Claimant is Covered on the Employer's Long-term Disability Plan"
            }
            EmployeesJobResponsibilitiesChangedDueToTheDisablingCondition => {
                "Employee's Job Responsibilities Changed Due to the Disabling Condition"
            }
            EmployerHasAReturnToWorkPolicyForDisabledEmployees => {
                "Employer Has a Return to Work Policy for Disabled Employees"
            }
            Open => "Open",
            Normal => "Normal",
            ClosedModerate => "Closed-moderate",
            Severe => "Severe",
            Moderate => "Moderate",
            Straight => "Straight",
            Convex => "Convex",
            Concave => "Concave",
            DoubleProtrusion => "Double Protrusion",
            NoCrossbite => "No Crossbite",
            Posterior => "Posterior",
            Anterior => "Anterior",
            Maxillary => "Maxillary",
            Mandibular => "Mandibular",
            Right => "Right",
            Left => "Left",
            MaxillaryModerate => "Maxillary Moderate",
            MandibularModerate => "Mandibular Moderate",
            MaxillarySevere => "Maxillary Severe",
            MandibularSevere => "Mandibular Severe",
            IncomeHasBeenVerified => "Income Has Been Verified",
            PersonHasBeenInterviewed => "Person Has Been Interviewed",
            RentHasBeenVerified => "Rent Has Been Verified",
            EmployerHasBeenVerified => "Employer Has Been Verified",
            PositionHasBeenVerified => "Position Has Been Verified",
            InquiryHasBeenVerified => "Inquiry Has Been Verified",
            OutstandingJudgments => "Outstanding Judgments",
            DeclaredBankruptcyInPast7Years => "Declared Bankruptcy in Past 7 Years",
            ForeclosureOrDeedInLieuInPast7Years => {
                "Foreclosure or Deed in Lieu in Past 7 Years"
            }
            PartyToLawsuit => "Party to Lawsuit",
            Code95 => "Obligated on a Loan Foreclosed, Deed in Lieu of Judgment",
            CurrentlyDelinquentOrInDefault => "Currently Delinquent or in Default",
            Code97 => "Obligated to Pay Alimony, Child Support or Maintenance",
            PartOfDownPaymentBorrowed => "Part of Down Payment Borrowed",
            CoMakerOrEndorserOnANote => "Co-maker or Endorser on a Note",
            LiabilityCoverageWillTransfer => "Liability Coverage Will Transfer",
            MostLikelyBuyerIsOtherPersonOrEntity => {
                "Most Likely Buyer is Other Person or Entity"
            }
            PotentialFinancingIsFannieMae => "Potential Financing is Fannie Mae",
            SuppressPaperEndorsement => "Suppress Paper Endorsement",
            DoNotSuppressPaperEndorsement => "Do Not Suppress Paper Endorsement",
            Escrow => "Escrow",
            TeachingMinor => "Teaching Minor",
            SubServicerSubmitted => "Sub-servicer Submitted",
            FirstMortgage => "First Mortgage",
            SecondMortgage => "Second Mortgage",
            Amputation => "Amputation",
            AddressSkipBegin => "Address Skip Begin",
            AddressCorrected => "Address Corrected",
            AutomaticDrillTimeCalculated => "Automatic Drill Time Calculated",
            AutomaticEdgingTimeCalculated => "Automatic Edging Time Calculated",
            AutomaticallySelect => "Automatically Select",
            AcceptingFamilyMembers => "Accepting Family Members",
            Agitated => "Agitated",
            AutomaticallySearchAndList => "Automatically Search and List",
            AddressIncorrect => "Address Incorrect",
            Assumable => "Assumable",
            PotentialFinancingIsCash => "Potential Financing is Cash",
            AmbulationLimitations => "Ambulation Limitations",
            PotentialFinancingIsOutsideLender => "Potential Financing is Outside Lender",
            AddressIncomplete => "Address Incomplete",
            AcceptCertificationWithoutChanges => "Accept Certification without Changes",
            AlleyIsPublic => "Alley is Public",
            PotentialFinancingIsFederalHousingAdministration => {
                "Potential Financing is Federal Housing Administration"
            }
            AddressSkipResolved => "Address Skip Resolved",
            AddressSkipExhaust => "Address Skip Exhaust",
            AcceptStatementOfLimitingConditionsWithoutChanges => {
                "Accept Statement of Limiting Conditions without Changes"
            }
            AutomaticUndersideTimeCalculated => "Automatic Underside Time Calculated",
            AvailableNotUsed => "Available - Not Used",
            AcceptCertificationWithChanges => "Accept Certification with Changes",
            AcceptStatementOfLimitingConditionsWithChanges => {
                "Accept Statement of Limiting Conditions with Changes"
            }
            AdjacentTrackOccupied => "Adjacent Track Occupied",
            PotentialFinancingIsVeteransAffairs => {
                "Potential Financing is Veterans Affairs"
            }
            UninsuredMotoristCoverageWillTransfer => {
                "Uninsured Motorist Coverage Will Transfer"
            }
            MortgageInForeclosure => "Mortgage in Foreclosure",
            CodeB2 => "Real Estate Owned (REO) Mortgage",
            PotentialFinancingIsContractForDeed => {
                "Potential Financing is Contract for Deed"
            }
            OnlyTheExteriorHasBeenInspected => "Only the Exterior has been Inspected",
            RealEstateOwnedPropertyOrForeclosureProperty => {
                "Real Estate Owned Property or Foreclosure Property"
            }
            NumberOfComparableListingsIsNormal => {
                "Number of Comparable Listings is Normal"
            }
            NumberOfComparableListingsIsAnOversupply => {
                "Number of Comparable Listings is an Oversupply"
            }
            NumberOfComparableListingsIsAShortage => {
                "Number of Comparable Listings is a Shortage"
            }
            PropertyManagementExpensesOutstanding => {
                "Property Management Expenses Outstanding"
            }
            BorrowerLetterAttempt => "Borrower Letter Attempt",
            BuildingOrMobileHomeIsInACoastalBarrierResourcesArea => {
                "Building or Mobile Home is in a Coastal Barrier Resources Area"
            }
            BorrowerTelephoneContact => "Borrower Telephone Contact",
            BusinessPending => "Business Pending",
            BorrowerLetterContact => "Borrower Letter Contact",
            MarketableSecuritiesValuedAtMarket => {
                "Marketable Securities valued at market"
            }
            AppropriateImprovementConditionExists => {
                "Appropriate Improvement Condition Exists"
            }
            NameUnknownToLocalAuthorities => "Name unknown to local authorities",
            NoManufacturingDoneOnPremises => "No manufacturing done on Premises",
            Occasional => "Occasional",
            OfficerOrOwnerInOtherBusinesses => "Officer or owner in other Businesses",
            CodeBL => "Bowel Limitations, Bladder Limitations, or both (Incontinence)",
            Old => "Old",
            OperatesOnPartTimeBasis => "Operates on part time basis",
            ParentFinancialStatementUsed => "Parent Financial Statement Used",
            BorrowerPaymentReceived => "Borrower Payment Received",
            BeneficiaryIsPartiallyDependent => "Beneficiary is Partially Dependent",
            ProductInformationAvailable => "Product Information Available",
            CodeBR => "Bedrest BRP (Bathroom Privileges)",
            RevenueDerivedFromCommissions => "Revenue derived from Commissions",
            BorrowerTelephoneAttempt => "Borrower Telephone Attempt",
            BeneficiaryIsTotallyDependent => "Beneficiary is Totally Dependent",
            RevenueDerivedFromDonations => "Revenue derived from Donations",
            RevenueDerivedFromFees => "Revenue derived from Fees",
            RevenueDerivedFromGrants => "Revenue derived from Grants",
            RevenueDerivedFromTaxes => "Revenue derived from Taxes",
            SprinklerEquipped => "Sprinkler Equipped",
            StatementRequestedFromGovernmentRegistry => {
                "Statement requested from Government Registry"
            }
            CollisionCoverageWillTransfer => "Collision Coverage Will Transfer",
            AdvancesFromPropertyManagementExpensesOutstanding => {
                "Advances From Property Management Expenses Outstanding"
            }
            FinalDemandLetterSent => "Final Demand Letter Sent",
            LenderRequestForAssistance => "Lender Request for Assistance",
            MortgageHasLenderPurchasedMortgageInsurance => {
                "Mortgage has Lender-purchased Mortgage Insurance"
            }
            InsufficientFunds => "Insufficient Funds",
            CreditEnhancedMortgage => "Credit Enhanced Mortgage",
            CorporateAppointment => "Corporate Appointment",
            SpecialServicingRequired => "Special Servicing Required",
            ClientSpecificallyRequestedConsiderationOfSpecialFinancingOrAnAssumableLoan => {
                "Client Specifically Requested Consideration of Special Financing or an Assumable Loan"
            }
            CaneRequired => "Cane Required",
            CompleteBedrest => "Complete Bedrest",
            CollectionCardWasLeft => "Collection Card was Left",
            CallToDirectoryAssistanceForReferenceTelephone => {
                "Call to Directory Assistance for Reference Telephone"
            }
            CoSignerTelephoneAttempt => "Co-signer Telephone Attempt",
            CoSignerTelephoneContact => "Co-signer Telephone Contact",
            ClaimIsFraudulent => "Claim is Fraudulent",
            CoSignerDelinquencyLetterSent => "Co-signer Delinquency Letter Sent",
            CoSignerFinalDemandLetterSent => "Co-signer Final Demand Letter Sent",
            CallToDirectoryAssistanceForCoSignerTelephone => {
                "Call to Directory Assistance for Co-signer Telephone"
            }
            ValidBorrowerAddressOrPhoneAttemptWithPreviousHolder => {
                "Valid Borrower Address or Phone Attempt with Previous Holder"
            }
            Convertible => "Convertible",
            ClaimantHadAPreExistingInjury => "Claimant had a Pre-existing Injury",
            Comatose => "Comatose",
            CommonElementsAreLeasedToOrByTheHomeOwnersAssociation => {
                "Common Elements are Leased to or by the Home Owners' Association"
            }
            CumulativeInjury => "Cumulative Injury",
            Contracture => "Contracture",
            CasePending => "Case Pending",
            Callable => "Callable",
            CrutchesRequired => "Crutches Required",
            CommunityParticipatesInNationalFloodInsuranceProgram => {
                "Community Participates in National Flood Insurance Program"
            }
            CommonElementsAreCompleted => "Common Elements are Completed",
            CurbAndGutterArePublic => "Curb and Gutter are Public",
            Cooperative => "Cooperative",
            CoolingWaterIsLow => "Cooling Water is Low",
            CertificationStatus => "Certification Status",
            CarSpacesAreAdequate => "Car Spaces are Adequate",
            CarSpacesAreInadequate => "Car Spaces are Inadequate",
            ComprehensiveCoverageWillTransfer => "Comprehensive Coverage Will Transfer",
            IssueCheckPayableToBorrowerAndReturnToServicer => {
                "Issue Check Payable to Borrower and Return to Servicer"
            }
            IssueCheckPayableToServicerAndReturnToServicer => {
                "Issue Check Payable to Servicer and Return to Servicer"
            }
            IssueCheckPayableToBorrowerAndSendToBorrower => {
                "Issue Check Payable to Borrower and Send to Borrower"
            }
            IssueCheckPayableToServicerOrBorrowerAndReturnToServicer => {
                "Issue Check Payable to Servicer or Borrower and Return to Servicer"
            }
            IssueCheckPayableToOtherPayee => "Issue Check Payable to Other Payee",
            Positive => "Positive",
            Negative => "Negative",
            TaxesAreTypicalForTheAreaAndPriceRange => {
                "Taxes are Typical for the Area and Price Range"
            }
            ImprovementConformsToZoningRegulations => {
                "Improvement Conforms to Zoning Regulations"
            }
            CallToDirectoryAssistanceForBorrowerTelephone => {
                "Call to Directory Assistance for Borrower Telephone"
            }
            DefermentOrForbearanceBegin => "Deferment or Forbearance Begin",
            Declined => "Declined",
            BorrowerFurnishedDemographicData => "Borrower Furnished Demographic Data",
            DefermentOrForbearanceEnd => "Deferment or Forbearance End",
            FundsAvailableForUnsecuredCreditors => {
                "Funds available for Unsecured Creditors"
            }
            DeductibleAmountFullyRecovered => "Deductible Amount Fully Recovered",
            DynamicBrakesAreOut => "Dynamic Brakes are Out",
            DebtorHasBeenDomiciled => "Debtor has been Domiciled",
            Disoriented => "Disoriented",
            DynamicBrakesAreOperational => "Dynamic Brakes are Operational",
            ConstructionWarranty => "Construction Warranty",
            ConstructionWarrantyTransferable => "Construction Warranty Transferable",
            MaintenanceDrugUnderClientsBenefitPlan => {
                "Maintenance Drug under Client's Benefit Plan"
            }
            PaymentReducedBecauseMaximumAllowableCostExceeded => {
                "Payment Reduced Because Maximum Allowable Cost Exceeded"
            }
            DeductibleAmountNotFullyRecovered => "Deductible Amount Not Fully Recovered",
            BenefitsTerminatedPriorToServiceDate => {
                "Benefits Terminated Prior to Service Date"
            }
            Depressed => "Depressed",
            DrugPartOfFormularyDataBase => "Drug Part of Formulary Data Base",
            SubjectNotEngagedInBusiness => "Subject not Engaged in Business",
            AllDoorSealsAreIntact => "All Door Seals are Intact",
            FilingFeeAttached => "Filing Fee Attached",
            SubjectNotEngagedInBusinessAtRequestedAddress => {
                "Subject not Engaged in Business at Requested Address"
            }
            Suspended => "Suspended",
            Total => "Total",
            UnableToRespond => "Unable to Respond",
            DyspneaWithMinimalExertion => "Dyspnea with Minimal Exertion",
            UsesOwnFacilities => "Uses Own Facilities",
            FiguresAreTotal => "Figures are Total",
            FixedAssetBreakdownUndisclosed => "Fixed Asset Breakdown Undisclosed",
            ForTheFiscalYear => "For the Fiscal Year",
            ForThePeriod => "For the Period",
            FormedByConsolidation => "Formed by Consolidation",
            FormedByMerger => "Formed by Merger",
            PriorBankruptcyCaseFiledInLast6Years => {
                "Prior Bankruptcy Case Filed in Last 6 Years"
            }
            DebtorIsNotRepresentedByAnAttorney => {
                "Debtor is not Represented by an Attorney"
            }
            APendingCaseHasBeenFiled => "A Pending Case has been Filed",
            GuaranteedByParentCompany => "Guaranteed by Parent Company",
            HasAuthorityForAllPurchases => "Has Authority for All Purchases",
            HasAuthorityToPurchaseSupplies => "Has Authority to Purchase Supplies",
            EquipmentCertified => "Equipment Certified",
            HasBusinessInterruptionInsurance => "Has Business Interruption Insurance",
            HasClassOfStock => "Has Class of Stock",
            HasExtendedCoverageInsurance => "Has Extended Coverage Insurance",
            HasFireInsurance => "Has Fire Insurance",
            HasJointAuthority => "Has Joint Authority",
            HasLifeInsurance => "Has Life Insurance",
            ExistenceOfPreliminaryFloodDetermination => {
                "Existence of Preliminary Flood Determination"
            }
            ExistenceOfCommunityParticipationInTheNationalFloodInsurance => {
                "Existence of Community Participation in the National Flood Insurance"
            }
            EnduranceLimitations => "Endurance Limitations",
            HasMarriageContract => "Has Marriage Contract",
            ElectricityOn => "Electricity On",
            EquipmentIsOverhauled => "Equipment Is Overhauled",
            ExercisesPrescribed => "Exercises Prescribed",
            HasNoParValue => "Has No Par Value",
            EngineStartUpPerformedWithNoProblemsReported => {
                "Engine Start-Up Performed with No Problems Reported"
            }
            EngineStartUpPerformedWithProblemsReported => {
                "Engine Start-Up Performed with Problems Reported"
            }
            ElectricalControlSystemShutDown => "Electrical Control System Shut Down",
            HasOtherInsurance => "Has Other Insurance",
            HasParValue => "Has Par Value",
            HasSoleAuthority => "Has Sole Authority",
            Excellent => "Excellent",
            HasVotingRights => "Has Voting Rights",
            HeadingAddressInRegisteredOfficeOnly => {
                "Heading Address in Registered Office Only"
            }
            HighLevel => "High Level",
            HomeworkersEmployed => "Homeworkers Employed",
            InSubscriberShares => "In Subscriber Shares",
            Inactive => "Inactive",
            Incomplete => "Incomplete",
            IncorporationDetailsRequested => "Incorporation Details Requested",
            IncreaseOrUp => "Increase or Up",
            InformationCannotBeProvidedAtThisTime => {
                "Information Cannot Be Provided at This Time"
            }
            InformationInDate => "Information in Date",
            InformationRequiresInvestigation => "Information Requires Investigation",
            ActionsHasASignificantEnvironmentalEffect => {
                "Actions has a Significant Environmental Effect"
            }
            ApplicationIncludesCompleteSystem => "Application Includes Complete System",
            AntennaIsMountedOnAStructureWithAnExistingAntenna => {
                "Antenna is Mounted on a Structure with an Existing Antenna"
            }
            NoticeOfConstructionOrAlterationHasBeenFiled => {
                "Notice of Construction or Alteration has been Filed"
            }
            ApplicantWantsToMonitorFrequency => "Applicant Wants to Monitor Frequency",
            ApplicantHasBeenDeniedGovernmentBenefitsDueToUseOfDrugs => {
                "Applicant has been Denied Government Benefits Due to Use of Drugs"
            }
            ApplicationIsCertified => "Application is Certified",
            ApplicationIsForOtherThanANewStation => {
                "Application is for other Than a New Station"
            }
            FeeRequired => "Fee Required",
            FloodStatus => "Flood Status",
            FloodInsuranceRequired => "Flood Insurance Required",
            CodeFL => "Federal Flood Insurance is Available (Community Participates)",
            CodeFM => "Inventory Valued Using LIFO (Last In/First Out)",
            NotTooHighLevel => "Not Too High Level",
            Forgetful => "Forgetful",
            FloodCertificationWithLifeOfLoan => "Flood Certification with Life of Loan",
            StreetMaintenanceIsPublic => "Street Maintenance is Public",
            Fair => "Fair",
            NotYetRegistered => "Not Yet Registered",
            ObligedToFileBalanceSheet => "Obliged to File Balance Sheet",
            OfficialConfirmationReceived => "Official Confirmation Received",
            OldButWellKept => "Old But Well Kept",
            OldEstablishedBusiness => "Old Established Business",
            OperatedAtBreakEven => "Operated at Break Even",
            OperatesAsAgent => "Operates as Agent",
            FloodZoneStatus => "Flood Zone Status",
            OutOfBusiness => "Out of Business",
            OutstandingClaims => "Outstanding Claims",
            GasOn => "Gas On",
            HazardousMaterialsAreUsedOrProduced => {
                "Hazardous Materials are Used or Produced"
            }
            GeneticallyEngineeredOrganismsAreUsedOrProduced => {
                "Genetically Engineered Organisms are Used or Produced"
            }
            ThisIsAGroupProposal => "This is a Group Proposal",
            HistoricalSitesAreAffected => "Historical Sites Are Affected",
            FacilitiesAreProperlyAccreditedOrAuthorized => {
                "Facilities are Properly Accredited or Authorized"
            }
            ProprietaryOrPrivilegedInformationWillBeContainedInTheApplication => {
                "Proprietary or Privileged Information will be contained in the Application"
            }
            ThisProjectHasAnActualOrPotentialImpactOnTheEnvironment => {
                "This Project has an Actual or Potential Impact on the Environment"
            }
            GrowthRateIsFullyDeveloped => "Growth Rate is Fully Developed",
            OutstandingSocialSecurityClaims => "Outstanding Social Security Claims",
            CodeGC => "Outstanding Value Added Tax (VAT) Claims",
            ProductDemonstrationInEffect => "Product Demonstration in Effect",
            OwnershipAcknowledgedInSignedStatement => {
                "Ownership Acknowledged in Signed Statement"
            }
            OwnershipAcknowledgedVerbally => "Ownership Acknowledged Verbally",
            OwnershipNotAcknowledged => "Ownership Not Acknowledged",
            OwnsNoRealEstate => "Owns No Real Estate",
            OwnsRealEstateButDetailsNotAvailable => {
                "Owns Real Estate but Details Not Available"
            }
            PreparedFromBooksWithoutAudit => "Prepared from Books Without Audit",
            PreparedFromStatementByAccountant => "Prepared from Statement by Accountant",
            ProfitsPaidToGroup => "Profits Paid to Group",
            ShelfSetToManufacturersStandard => "Shelf Set to Manufacturer's Standard",
            PubliclyTraded => "Publicly Traded",
            Good => "Good",
            PurchaseAuthorityIsQualified => "Purchase Authority is Qualified",
            PurchasesOnFloorPlan => "Purchases on Floor Plan",
            ShelfSetToRetailersSchematic => "Shelf Set to Retailer's Schematic",
            PurchasesOnLetterOfCredit => "Purchases on Letter of Credit",
            RealEstateCheckIsNecessary => "Real Estate Check is Necessary",
            RecordOfPreferentialClaims => "Record of Preferential Claims",
            RegisteredAddressIsSameAsBusinessAddress => {
                "Registered Address is Same as Business Address"
            }
            RelativesHelpInBusiness => "Relatives Help in Business",
            Satisfactory => "Satisfactory",
            SeasonsAreSteady => "Seasons are Steady",
            Secured => "Secured",
            OrganizationCertifiesComplianceWithFederalLobbyingRegulations => {
                "Organization Certifies Compliance with Federal Lobbying Regulations"
            }
            ProjectInvolvesInternationalCoOperativeActivities => {
                "Project involves International Co-operative Activities"
            }
            HumanAnatomicalSubstancesAreUsed => "Human Anatomical Substances Are Used",
            HandicapFacilitiesAreAvailable => "Handicap Facilities Are Available",
            LobbyingActivitiesHaveBeenConductedRegardingTheProposal => {
                "Lobbying Activities Have Been Conducted Regarding the Proposal"
            }
            OrganizationCertifiesComplianceWithTheDrugFreeWorkplaceAct => {
                "Organization Certifies Compliance With the Drug-Free Workplace Act"
            }
            OrganizationCertifiesComplianceWithTheCodeOfFederalRegulationsRegardingResearchMisconduct => {
                "Organization Certifies Compliance with the Code of Federal Regulations Regarding Research Misconduct"
            }
            OrganizationProvidesASmokeFreeWorkplace => {
                "Organization Provides a Smoke Free Workplace"
            }
            OrganizationCertifiesComplianceWithFederalDiscriminationRegulations => {
                "Organization Certifies Compliance with Federal Discrimination Regulations"
            }
            CodeH9 => {
                "Organization Certifies Compliance with the Code of Federal Regulations Regarding Responsibility of Applicants for Promoting Objectivity in Research for which Public Health Service (PHS) Funding is Sought"
            }
            WellMaintained => "Well Maintained",
            InterestRateBuydown => "Interest Rate Buydown",
            HeatingAndCoolingForTheIndividualUnitsSeparatelyMetered => {
                "Heating and Cooling for the Individual Units Separately Metered"
            }
            HighDischarge => "High Discharge",
            HighEngineWaterPressure => "High Engine Water Pressure",
            InterestOnly => "Interest Only",
            GraduatedPayment => "Graduated Payment",
            PrincipalBalanceExceedsMaximumNegativeAmortization => {
                "Principal Balance Exceeds Maximum Negative Amortization"
            }
            LastChange => "Last Change",
            LiabilityReleased => "Liability Released",
            LiabilityNotReleased => "Liability Not Released",
            HearingLimitations => "Hearing Limitations",
            LiabilityDeterminedByNoteHolder => "Liability Determined by Note Holder",
            AfterConversion => "After Conversion",
            Hostile => "Hostile",
            AfterModification => "After Modification",
            Balloon => "Balloon",
            CapitalizedMortgage => "Capitalized Mortgage",
            FederalWagesInEffect => "Federal Wages in Effect",
            CodeHT => "Social Security Number (SSN) Never Issued",
            CodeHU => "Name Does Not Match Social Security Number (SSN)",
            CodeHV => "Birthdate Does Not Match Social Security Number (SSN)",
            CodeHW => "Impossible Social Security Number (SSN)",
            EmployeeIsIneligibleToWork => "Employee is Ineligible to Work",
            MetesAndBounds => "Metes and Bounds",
            CodeHZ => "Consolidation, Extension, Modification of Mortgage Loan (CEM)",
            BasedOnOperatingData => "Based on Operating Data",
            UsesOutsideServices => "Uses Outside Services",
            VeryHighLevel => "Very High Level",
            VerySmall => "Very Small",
            VoluntaryBankruptcy => "Voluntary Bankruptcy",
            WellBalanced => "Well Balanced",
            WellRegardedInBusinessCircles => "Well Regarded in Business Circles",
            OrganizationHasDelinquentFederalDebts => {
                "Organization has Delinquent Federal Debts"
            }
            OrganizationHasBeenPlacedOnTheFederalDebarmentAndSuspensionList => {
                "Organization has been Placed on the Federal Debarment and Suspension List"
            }
            NoShowIndicator => "No-show Indicator",
            InterestPaidInAdvance => "Interest Paid in Advance",
            InterestPaidInArrears => "Interest Paid in Arrears",
            InterestCarryover => "Interest Carryover",
            SellsDirectly => "Sells Directly",
            SellsWithAgents => "Sells with Agents",
            SellsWithStorage => "Sells with Storage",
            Small => "Small",
            IndependentAtHome => "Independent at Home",
            SomeIncrease => "Some Increase",
            SomewhatDecliningTendency => "Somewhat Declining Tendency",
            StartedSomeTimeAgo => "Started Some Time Ago",
            IndustryLocation => "Industry Location",
            Sufficient => "Sufficient",
            Indifferent => "Indifferent",
            TerminationDateSet => "Termination Date Set",
            InjuryOccurredOnEmployersPremises => "Injury occurred on Employer's Premises",
            TermsIncludeLumpSumPayments => "Terms Include Lump Sum Payments",
            TermsIncludeProgressPayments => "Terms Include Progress Payments",
            TermsOnCostPlusBasis => "Terms on Cost Plus Basis",
            TermsOnFixedFeeBasis => "Terms on Fixed Fee Basis",
            TradeStyleRegistered => "Trade Style Registered",
            TradingAddressOfSoleProprietor => "Trading Address of Sole Proprietor",
            UnchangedSituation => "Unchanged Situation",
            Undetermined => "Undetermined",
            Unsatisfactory => "Unsatisfactory",
            Unsecured => "Unsecured",
            QualifiesAsAnEnergyEfficientHome => "Qualifies as an Energy Efficient Home",
            CodeJ1 => {
                "Military Services Barred from Recruitment Activities at the Proposing Organization's Site(s)"
            }
            RateNegotiated => "Rate Negotiated",
            UnderPenaltyOfPerjuryTheInformationIsTrueAndCorrect => {
                "Under Penalty of Perjury the Information is True and Correct"
            }
            ProjectRequiresInterGovernmentReviewForActivitiesThatAffectStateOrLocalGovernmentOrPossibleNationalSecurityImplications => {
                "Project Requires Inter-Government Review for Activities that affect State or Local Government or Possible National Security Implications"
            }
            FilingOnBehalfOfDebtorIsAuthorized => {
                "Filing on Behalf of Debtor is Authorized"
            }
            DebtorUnderstandsTheReliefAvailableUnderEachBankruptcyChapter => {
                "Debtor Understands the Relief available under each Bankruptcy Chapter"
            }
            AttorneyDeclaresThatDebtorHasBeenInformed => {
                "Attorney Declares that Debtor has been Informed"
            }
            AttorneyHasExplainedTheReliefAvailableUnderEachBankruptcyChapter => {
                "Attorney has Explained the Relief available under each Bankruptcy Chapter"
            }
            ThereHasBeenATransferOfAClaimAgainstTheDebtorByOrToAnyPetitioner => {
                "There has been a Transfer of a Claim Against the Debtor by or to any Petitioner"
            }
            ThirdPartyOriginated => "Third Party Originated",
            ExistingConstruction => "Existing Construction",
            OtherLien => "Other Lien",
            JointCoverageApplies => "Joint Coverage Applies",
            SubjectLien => "Subject Lien",
            CodeJE => {
                "No Evidence of Property Damage Observed such as Dampness, Termites, or Structure Settlement"
            }
            PrimaryUnderwritingSystem => "Primary Underwriting System",
            NonNewPartsUsed => "Non New Parts Used",
            PledgedLoan => "Pledged Loan",
            SecurityDelivery => "Security Delivery",
            SecondaryUnderwritingSystem => "Secondary Underwriting System",
            DistributionIsStopped => "Distribution is Stopped",
            SentenceWasSuspended => "Sentence was Suspended",
            VeryNegativeInformationExists => "Very Negative Information Exists",
            PaymentNotesExist => "Payment Notes Exist",
            Immigrated => "Immigrated",
            AuditedWithQualifications => "Audited with Qualifications",
            Audited => "Audited",
            TemporarilyClosed => "Temporarily Closed",
            Partial => "Partial",
            TelephoneNumberIsUnpublished => "Telephone Number is Unpublished",
            TelephoneNumberIsNotInService => "Telephone Number is Not in Service",
            NegativeInformationExistsForTheGroup => {
                "Negative Information Exists for the Group"
            }
            TheMoreImportantItemsAreOnlyIncluded => {
                "The More Important Items are Only Included"
            }
            InterestOwnedByAffiliatedCompany => "Interest Owned by Affiliated Company",
            InterestOwnedBySubjectOfInquiry => "Interest Owned by Subject of Inquiry",
            QualifiesAsAGovernmentApprovedCondominiumOrProject => {
                "Qualifies as a Government Approved Condominium or Project"
            }
            AccountReceivablesBreakdownUndisclosed => {
                "Account Receivables Breakdown Undisclosed"
            }
            AdditionalRecordItemsAvailable => "Additional Record Items Available",
            AddressIsQualified => "Address is Qualified",
            AllPaidInOrIssued => "All Paid In or Issued",
            AppearsHigh => "Appears High",
            AppearsNotToGuaranteeSufficientCoverage => {
                "Appears Not to Guarantee Sufficient Coverage"
            }
            AppearsSufficientlyHigh => "Appears Sufficiently High",
            AppearsToIndicateAStrainedSituation => {
                "Appears to Indicate a Strained Situation"
            }
            BanksWithMainNationalBanks => "Banks with Main National Banks",
            BillsPaidFromBranchOffice => "Bills Paid from Branch Office",
            BillsPaidFromDivisionOffice => "Bills Paid from Division Office",
            BillsPaidFromHeadquartersOffice => "Bills Paid from Headquarters Office",
            BondInformationAvailable => "Bond Information Available",
            ChangedAccountingDate => "Changed Accounting Date",
            Clear => "Clear",
            ClearDecliningTendency => "Clear Declining Tendency",
            ClearIncrease => "Clear Increase",
            Cluttered => "Cluttered",
            CompanyHasNoOtherLocations => "Company has No Other Locations",
            CompanyIsBranchOfForeignEntity => "Company is Branch of Foreign Entity",
            CompanyIsPerpetual => "Company is Perpetual",
            CompanyIsTaxExempt => "Company is Tax Exempt",
            ComparedToSamePeriodLastYear => "Compared to Same Period Last Year",
            ConductedAtALoss => "Conducted at a Loss",
            CodeKO => "Inventory Valued using FIFO (First In/First Out)",
            Large => "Large",
            LetterOfAgreementPresent => "Letter of Agreement Present",
            LetterOfAgreementWithdrawn => "Letter of Agreement Withdrawn",
            LetterOfLiabilityPresent => "Letter of Liability Present",
            LetterOfLiabilityWithdrawn => "Letter of Liability Withdrawn",
            LocationInquiredUponIsABranch => "Location Inquired Upon is a Branch",
            CodeKV => "Location Inquired Upon is a Branch; Headquarters is Provided",
            LocationInquiredUponIsAHeadquarters => {
                "Location inquired upon is a Headquarters"
            }
            LocationIsForeign => "Location is Foreign",
            MeansExhausted => "Means Exhausted",
            MediumToLarge => "Medium to Large",
            ImmunizationMandatedByStateLawForEmployment => {
                "Immunization Mandated by State Law for Employment"
            }
            GeneralStandardOf20DegreeOr5DiopterSphereOrCylinderChangeMet => {
                "General Standard of 20 Degree or .5 Diopter Sphere or Cylinder Change Met"
            }
            ReplacementDueToLossOrTheft => "Replacement Due to Loss or Theft",
            ReplacementDueToBreakageOrDamage => "Replacement Due to Breakage or Damage",
            ReplacementDueToPatientPreference => "Replacement Due to Patient Preference",
            ReplacementDueToMedicalReason => "Replacement Due to Medical Reason",
            LandContract => "Land Contract",
            AccountCurrent => "Account Current",
            VeryGood => "Very Good",
            Restored => "Restored",
            LetterOfMapAmendmentOrLetterOfMapRevision => {
                "Letter of Map Amendment or Letter of Map Revision"
            }
            LegallyBlind => "Legally Blind",
            ProducerOfGoods => "Producer of Goods",
            DrawbackIndicator => "Drawback Indicator",
            Lethargic => "Lethargic",
            CustomsRuleApplicable => "Customs Rule Applicable",
            ExportedPursuantToLawRegulationOrToCancelCustomsBond => {
                "Exported Pursuant to Law Regulation or to Cancel Customs Bond"
            }
            CountryOfOriginInformationAppliesToAllPriorShipments => {
                "Country of Origin Information Applies to All Prior Shipments"
            }
            PriceEstimated => "Price Estimated",
            CodeLJ => "North American Free Trade Agreement (NAFTA) Preference",
            KitForm => "Kit Form",
            LockoutEffective => "Lockout Effective",
            LetterOfAppointment => "Letter of Appointment",
            FacilitysEmergencyResponsePlanIncludesSpecificActionsToBeTakenInResponseToAccidentalReleasesOfRegulatedSubstances => {
                "Facility's Emergency Response Plan Includes Specific Actions to be Taken in Response to Accidental Releases of Regulated Substances"
            }
            LocomotiveIsIsolated => "Locomotive is Isolated",
            LowEngineOilPressure => "Low Engine Oil Pressure",
            FacilityHadASafetyInspection => "Facility had a Safety Inspection",
            LocomotiveEngineIsRunning => "Locomotive Engine is Running",
            LesseeSignatureOnFile => "Lessee Signature on File",
            ListSpecialtyInDirectory => "List Specialty in Directory",
            LenderOrServicerTransfer => "Lender or Servicer Transfer",
            EvidenceOfDampness => "Evidence of Dampness",
            EvidenceOfTermites => "Evidence of Termites",
            EvidenceOfStructureSettlement => "Evidence of Structure Settlement",
            SalvageMoved => "Salvage Moved",
            AddressIsFormerLocation => "Address is Former Location",
            AddressIsOccupiedByOthers => "Address is Occupied by Others",
            CodeM0 => {
                "Facility has an Occupational Safety and Health Act\n(OSHA) Star or Merit Ranking"
            }
            DataCorrected => "Data Corrected",
            ServicerRecordSelected => "Servicer Record Selected",
            LengthOfServiceIs3MonthsOrLess => "Length of Service is 3 Months or Less",
            CodeM4 => "Length of Service is 3 Months or more, and Less than 1 Year",
            LengthOfServiceIs1YearThrough5Years => {
                "Length of Service is 1 Year through 5 Years"
            }
            LengthOfServiceIsMoreThan5Years => "Length of Service is more than 5 Years",
            CataractOrCornealTransplantOrOtherConditionSuchAsKeratoconus => {
                "Cataract or Corneal Transplant or Other Condition such as Keratoconus"
            }
            VisionInWorseEyeCorrectableTo2040OrBetterWithRegularLenses => {
                "Vision in Worse Eye Correctable to 20/40 or Better with Regular Lenses"
            }
            ContactLensesCorrectedVisionInWorseEyeTo2040OrBetter => {
                "Contact Lenses Corrected Vision in Worse Eye to 20/40 or Better"
            }
            MajorAlarmFlagReported => "Major Alarm Flag Reported",
            EquipmentHasModifiedConfiguration => "Equipment has Modified Configuration",
            OtherMentalCondition => "Other Mental Condition",
            MarketingTimeIs4To6Months => "Marketing Time is 4 to 6 Months",
            TrendReversed => "Trend Reversed",
            MicroprocessorFault => "Microprocessor Fault",
            MortgageInsuranceApplicationIncluded => {
                "Mortgage Insurance Application Included"
            }
            MortgageCreditReportIncluded => "Mortgage Credit Report Included",
            ResidentialLoanApplicationIncluded => "Residential Loan Application Included",
            RealEstateInformationReportIncluded => {
                "Real Estate Information Report Included"
            }
            RealEstateTitleEvidenceIncluded => "Real Estate Title Evidence Included",
            ManuallySearchAndList => "Manually Search and List",
            CodeMM => "Property is Occupied by Tenant (Market Rent)",
            CodeMN => "Property is Occupied by Tenant (Regulated Rent)",
            CooperativeProjectIncludesOrOwnsAnyCommercialUnits => {
                "Cooperative Project Includes or Owns Any Commercial Units"
            }
            UnitsAndProjectAmenitiesAreComplete => {
                "Units and Project Amenities are Complete"
            }
            EligibleTrust => "Eligible Trust",
            ResaleProperty => "Resale Property",
            MiscellaneousSkipTraceAttempt => "Miscellaneous Skip-Trace Attempt",
            PhotosMatchDescription => "Photos Match Description",
            PhotosShowNegativeInfluence => "Photos Show Negative Influence",
            ExcludeFromMonthlyDebt => "Exclude from Monthly Debt",
            ThisBrokerMarketAnalysisIsBeingCompletedForHomeMarketAssistance => {
                "This Broker Market Analysis is being Completed for Home Market Assistance"
            }
            ThisBrokerMarketAnalysisIsBeingCompletedForHomesaleOrBuyout => {
                "This Broker Market Analysis is being Completed for Homesale or Buyout"
            }
            ProjectTypeIsSingleFamily => "Project Type is Single Family",
            ProjectTypeIsOther => "Project Type is Other",
            HospitalizedOverNight => "Hospitalized over-night",
            CodeN1 => "Claim Involves (a) Day(s) Away From Work",
            ClaimInvolvesRestrictedWorkActivityWithoutDaysAwayFromWork => {
                "Claim involves Restricted Work Activity Without Days Away from Work"
            }
            StrikeOrLockoutInProgress => "Strike or Lockout in Progress",
            ShutdownOrLayoffInProgress => "Shutdown or Layoff in Progress",
            WorkIsSeasonal => "Work is Seasonal",
            NaturalDisasterOrAdverseWeatherAffectingWork => {
                "Natural Disaster or Adverse Weather Affecting Work"
            }
            ShorterWorkSchedulesOrFewerPayPeriodsThanUsualInEffect => {
                "Shorter Work Schedules or Fewer Pay Periods than Usual in Effect"
            }
            LongerWorkSchedulesOrMorePayPeriodsThanUsualInEffect => {
                "Longer Work Schedules or More Pay Periods than Usual in Effect"
            }
            OtherFactorsAffectClaimFrequency => "Other Factors Affect Claim Frequency",
            NoUserAvailable => "No User Available",
            NeighborhoodPredominatelySingleFamilyDwellings => {
                "Neighborhood Predominately Single Family Dwellings"
            }
            ItemHasDirectNumericalControl => "Item has Direct Numerical Control",
            NoteHolderPermissionRequired => "Note Holder Permission Required",
            NoDeductibleProgram => "No Deductible Program",
            Notarized => "Notarized",
            NewConstruction => "New Construction",
            MortgagePointsAreCustomarilyPaidBySeller => {
                "Mortgage Points are Customarily Paid by Seller"
            }
            NoNationalFloodInsuranceProgramMap => {
                "No National Flood Insurance Program map"
            }
            SeasonedMortgage => "Seasoned Mortgage",
            IssuesAreAnticipatedThatWouldAffectTheAbilityToSecureFinancingOfTheSubjectProperty => {
                "Issues are Anticipated that would Affect the Ability to Secure Financing of the Subject Property"
            }
            Citizenship => "Citizenship",
            GroupDisabilityInsuranceMandatory => "Group Disability Insurance Mandatory",
            RetailOrigination => "Retail Origination",
            CodeNN => "Answer to Referenced Question is \"None\"",
            ArmsLengthTransaction => "Arm's Length Transaction",
            CertificationOfANonAttorneyBankruptcyPetitionPreparer => {
                "Certification of a Non-attorney Bankruptcy Petition Preparer"
            }
            EligibleForTheFannieMaeNeighborsProgram => {
                "Eligible for the Fannie Mae Neighbors Program"
            }
            NoRestrictions => "No Restrictions",
            CodeNS => "401K Plan in Effect",
            LodgingProvided => "Lodging Provided",
            NotUsed => "Not Used",
            ContractLabor => "Contract Labor",
            BonusesPaid => "Bonuses Paid",
            MinorsEmployed => "Minors Employed",
            MeetsRequirementsForFannieMaeCommunitySecondsProgram => {
                "Meets Requirements for Fannie Mae Community Seconds Program"
            }
            PurchaseIsAResultOfCurrentEmployerSponsoredRelocation => {
                "Purchase is a Result of Current Employer Sponsored Relocation"
            }
            TeachingMajor => "Teaching Major",
            MultipleUnspecifiedInstances => "Multiple Unspecified Instances",
            HiresPartTimeEmployeesAsNeeded => "Hires Part Time Employees as Needed",
            MexicanRequest => "Mexican Request",
            RiskManagementPlanRequiresPredictiveFiling => {
                "Risk Management Plan Requires Predictive Filing"
            }
            SanitizedCopy => "Sanitized Copy",
            CodeO6 => {
                "Site Treated, Disposed, Recycled Waste On-Site or Discharged Waste to Sewer or Publicly Owned Treatment Works"
            }
            ToxicChemicalClaimedAsTradeSecret => "Toxic Chemical Claimed as Trade Secret",
            UnderControlOfReportingFacilityOrParentCompany => {
                "Under Control of Reporting Facility or Parent Company"
            }
            WeatherConditionsNotKnown => "Weather Conditions Not Known",
            SellerProvidedBelowMarketSecondaryFinancing => {
                "Seller Provided Below Market Secondary Financing"
            }
            FixedSite => "Fixed Site",
            MobileFacility => "Mobile Facility",
            TransferAuthorized => "Transfer Authorized",
            OccupationalDisease => "Occupational Disease",
            TransferComplete => "Transfer Complete",
            CommercialDriversLicenseVerified => "Commercial Driver's License Verified",
            ResponsibilityAccepted => "Responsibility Accepted",
            WaterbodyInvolved => "Waterbody Involved",
            ChargesPending => "Charges Pending",
            DriverHasProperLicenseClass => "Driver has Proper License Class",
            DriverCompliantWithLicenseRestrictions => {
                "Driver Compliant with License Restrictions"
            }
            OtherLimitation => "Other Limitation",
            DriverHasCommercialDriversLicense => "Driver has Commercial Driver's License",
            DriverHasMedicalWaiver => "Driver has Medical Waiver",
            OwnOtherFederalHousingAdministrationProperty => {
                "Own other Federal Housing Administration Property"
            }
            OutOfRangeProductTemperature => "Out of Range Product Temperature",
            PhotographsTaken => "Photographs Taken",
            OtherRestrictions => "Other Restrictions",
            OutOfService => "Out of Service",
            Oriented => "Oriented",
            PoliceOfficerAtScene => "Police Officer at Scene",
            Overridden => "Overridden",
            Proposed => "Proposed",
            RatingIsAffected => "Rating is Affected",
            CodeOY => {
                "Veteran as Defined by the Federal Housing Administration (FHA), Veterans Administration (VA), or Department of Housing and Urban Development (HUD)"
            }
            LiabilityIsContingentOrHasACoSigner => {
                "Liability is Contingent or has a Co-signer"
            }
            TerminalDegree => "Terminal Degree",
            PatientWasDischargedFromTheFirstFacility => {
                "Patient was Discharged from the First Facility"
            }
            PatientWasAdmittedToTheSecondFacility => {
                "Patient was Admitted to the Second Facility"
            }
            PropertyHasAFamilyRoomOrDen => "Property has a Family Room or Den",
            PropertyHasCentralAirConditioning => "Property has Central Air Conditioning",
            PropertyTypicalOfNeighborhood => "Property Typical of Neighborhood",
            PropertyDeferredMaintenanceTypicalOfNeighborhood => {
                "Property Deferred Maintenance Typical of Neighborhood"
            }
            AcceptingExistingPatients => "Accepting Existing Patients",
            AcceptingNewPatients => "Accepting New Patients",
            PropertyIntendedToBeOccupiedAsPrimaryResidence => {
                "Property Intended to be Occupied as Primary Residence"
            }
            Paralysis => "Paralysis",
            PhoneSkipBegin => "Phone Skip Begin",
            PlanIsAttached => "Plan is Attached",
            PhoneSkipResolved => "Phone Skip Resolved",
            PhoneSkipExhaust => "Phone Skip Exhaust",
            PaidOutsideOfClosing => "Paid Outside of Closing",
            PreviouslyFailedBoardCertification => "Previously Failed Board Certification",
            ProjectIsSubjectToGroundRent => "Project is Subject to Ground Rent",
            Prepayable => "Prepayable",
            Program => "Program",
            ProviderIsParticipating => "Provider is Participating",
            PreliminaryFloodDetermination => "Preliminary Flood Determination",
            ProviderCertificationInTheTaxonomyHasBeenVerified => {
                "Provider Certification in the Taxonomy Has Been Verified"
            }
            ProjectAndServicesBudgetIsMaintained => {
                "Project and Services Budget is Maintained"
            }
            AtypicalPhysicalCondition => "Atypical Physical Condition",
            PersonalPropertyOnsite => "Personal Property Onsite",
            PropertyPreviouslyWinterized => "Property Previously Winterized",
            LiabilityWillBeResubordinatedToTheLoanUponClosing => {
                "Liability will be Resubordinated to the Loan upon Closing"
            }
            Poor => "Poor",
            PriorDamage => "Prior Damage",
            PublicationIsIncludedInSharing => "Publication is Included in Sharing",
            ProjectIsComplete => "Project is Complete",
            NotPaid => "Not Paid",
            PropertyVacant05Percent => "Property Vacant 0-5 Percent",
            PartialWeightBearing => "Partial Weight Bearing",
            PaidByBorrowerBeforeClosing => "Paid by Borrower Before Closing",
            PropertyForSale => "Property for Sale",
            PropertyVacantOver5Percent => "Property Vacant Over 5 Percent",
            Veteran => "Veteran",
            ExportProduct => "Export Product",
            CodeQ2 => "Distilled Spirit, Beer or Wine",
            USGoodsReturned => "U.S. Goods Returned",
            CandidateForUSCustomsServiceProtest => {
                "Candidate for U.S. Customs Service Protest"
            }
            DomesticProduct => "Domestic Product",
            PriorApprovalLetterAndOfficialOrdersOnFile => {
                "Prior Approval Letter and Official Orders on File"
            }
            ImportersSubstantiatingStatementAndContractAreOnFile => {
                "Importer's Substantiating Statement and Contract are on File"
            }
            InternationalTransportMovement => "International Transport Movement",
            PieceCountShouldBeIncludedInTheTotalPackingListQuantity => {
                "Piece Count should be Included in the Total Packing List Quantity"
            }
            ShipmentShouldBeHeldAtThePort => "Shipment should be Held at the Port",
            MultipleStatesOfOriginForThisItem => {
                "Multiple States of Origin for this Item"
            }
            MultipleCountriesOfOriginForThisItem => {
                "Multiple Countries of Origin for this Item"
            }
            LetterOfCreditRestrictedToASpecificBank => {
                "Letter of Credit Restricted to a Specific Bank"
            }
            LetterOfCreditPermitsTransshipment => {
                "Letter of Credit Permits Transshipment"
            }
            LetterOfCreditCoversPartialShipments => {
                "Letter of Credit Covers Partial Shipments"
            }
            DutiableItem => "Dutiable Item",
            AmountsShouldBeProRatedAcrossLineItems => {
                "Amounts should be Pro-rated across Line Items"
            }
            CodeQI => "Toxic Substance Control Act (TSCA) Certification Required",
            VisaRequiredForThisItem => "Visa Required for this Item",
            ItemSubjectToQuotas => "Item Subject to Quotas",
            CodeQL => {
                "Item is a Set as Defined by the General Rules of Interpretation Section 3 (GRI3)"
            }
            ItemIsASet => "Item is a Set",
            ItemIsAnEnsemble => "Item is an Ensemble",
            ItemIsAMetalItem => "Item is a Metal Item",
            ItemIsAMachinePart => "Item is a Machine Part",
            ItemIsAHazardousItem => "Item is a Hazardous Item",
            CodeQR => {
                "Item is Eligible under the Generalized System of Preferences (GSP)"
            }
            QuantityToBeImportedHasBeenApprovedByTheNecessaryAgencies => {
                "Quantity to be Imported has been Approved by the Necessary Agencies"
            }
            FilingDataIsToBeWithheldFromPublicInspection => {
                "Filing Data is to be Withheld from Public Inspection"
            }
            PropertyTypeCooperative => "Property Type Cooperative",
            PaidByBorrowerAtClosing => "Paid by Borrower at Closing",
            PaidByOtherAtOrBeforeClosing => "Paid by Other At or Before Closing",
            TreatedAsAReductionToIncome => "Treated as a Reduction to Income",
            CodeQY => {
                "Does Organization Receive Income from the Sale or Lease of Tangible Personal Property, the Lease of Real Property, or the Sale of Taxable Services?"
            }
            CodeQZ => {
                "Is organization a contractor-retailer primarily engaged in retail sales?"
            }
            ExemptFromPublicRecordsLaw => "Exempt from Public Records Law",
            DebtorHoldsClaimToRealProperty => "Debtor Holds Claim to Real Property",
            EntityClaimsToHoldASecuredInterest => {
                "Entity Claims to Hold a Secured Interest"
            }
            DebtorHasPropertyOfTheTypeSpecified => {
                "Debtor has Property of the Type Specified"
            }
            DebtorElectsTheStateExemption => "Debtor Elects the State Exemption",
            DebtorElectsTheFederalExemption => "Debtor Elects the Federal Exemption",
            CoDebtorMayBeJointlyLiable => "Co-debtor may be Jointly Liable",
            ClaimIsContingent => "Claim is Contingent",
            ClaimIsUnliquidated => "Claim is Unliquidated",
            ClaimIsDisputed => "Claim is Disputed",
            ReferenceTelephoneAttempt => "Reference Telephone Attempt",
            DebtorHasNoCreditorsHoldingUnsecuredPriorityClaims => {
                "Debtor has No Creditors Holding Unsecured Priority Claims"
            }
            ReferenceTelephoneContact => "Reference Telephone Contact",
            RentalCarArranged => "Rental Car Arranged",
            RentDelinquent => "Rent Delinquent",
            ClaimIsSubjectToSetoff => "Claim is Subject to Setoff",
            DebtorHasNoExecutoryContractsOrUnexpiredLeases => {
                "Debtor has No Executory Contracts or Unexpired Leases"
            }
            LeaseIsForNonresidentialRealProperty => {
                "Lease is for Nonresidential Real Property"
            }
            DebtorHasNoCoDebtors => "Debtor has No Co-debtors",
            DebtorIsMarried => "Debtor is Married",
            DebtorsSpouseMaintainsASeparateHousehold => {
                "Debtor's Spouse Maintains a Separate Household"
            }
            RealEstateTaxesAreIncluded => "Real Estate Taxes are Included",
            PropertyInsuranceIsIncluded => "Property Insurance is Included",
            DebtorHasNoCreditorsHoldingSecuredClaims => {
                "Debtor has No Creditors Holding Secured Claims"
            }
            RentControl => "Rent Control",
            EquipmentIsRebuilt => "Equipment is Rebuilt",
            IndividualInjuredInPerformanceOfDuty => {
                "Individual Injured in Performance of Duty"
            }
            IndividualInjuredByThirdParty => "Individual Injured by Third Party",
            QualityOfManagementAndItsEnforcementOfRulesAndRegulationsBasedOnGeneralAppearances => {
                "Quality of Management and its Enforcement of Rules and Regulations Based on General Appearances"
            }
            PayContinued => "Pay Continued",
            SickLeaveTaken => "Sick Leave Taken",
            SignatureOnFile => "Signature on File",
            LowRefrigerantCapacityShutdown => "Low Refrigerant Capacity Shutdown",
            RecentDefrost => "Recent Defrost",
            RatedHorsepowerCanBeProduced => "Rated Horsepower can be Produced",
            ForeignMilitarySale => "Foreign Military Sale",
            WaiverOfPriorNotice => "Waiver of Prior Notice",
            AlternateCertificationProgramParticipant => {
                "Alternate Certification Program Participant"
            }
            ServicesProvidedAtTheSecondFacilityWereAvailableAtTheFirstFacility => {
                "Services Provided at the Second Facility were available at the First Facility"
            }
            UnderTreatment => "Under Treatment",
            FirstTimeVacant => "First Time Vacant",
            AdverseEasement => "Adverse Easement",
            DisclosureIndicator => "Disclosure Indicator",
            AtypicalOffSiteImprovements => "Atypical Off Site Improvements",
            ToxicSubstances => "Toxic Substances",
            AdverseEncroachment => "Adverse Encroachment",
            AtypicalFunctionalCondition => "Atypical Functional Condition",
            SubjectPropertyIsCurrentlyListed => "Subject Property is Currently Listed",
            DebtorIsASmallBusinessAsDefinedIn11USCSection101 => {
                "Debtor is a Small Business as Defined in 11 U.S.C. Section 101"
            }
            SpecialServicesAreMobileHomeOnly => "Special Services are Mobile Home Only",
            SpecialServicesAreLeaseholdOrMobileHomeOrBoth => {
                "Special Services are Leasehold or Mobile Home or Both"
            }
            CodeSE => {
                "Debtor Elects to be Considered as a Small Business Under 11 U.S.C. Section 1121(e)"
            }
            SensorFault => "Sensor Fault",
            StreetLightsArePublic => "Street Lights are Public",
            SpecialServicesAreLeaseholdOrSubleaseholdOrBoth => {
                "Special Services are Leasehold or Subleasehold or Both"
            }
            HazardousWaste => "Hazardous Waste",
            PestInfestation => "Pest Infestation",
            RoadMaintenanceRequired => "Road Maintenance Required",
            SpeechLimitations => "Speech Limitations",
            CurrentlyServingInMilitary => "Currently Serving in Military",
            MajorBaseSupport => "Major Base Support",
            CriticalSupportLevelMet => "Critical Support Level Met",
            StreetIsPublic => "Street is Public",
            SpecialtyIsPrimary => "Specialty is Primary",
            SpecialtyIsSecondary => "Specialty is Secondary",
            LocalWagesInEffect => "Local Wages in Effect",
            FederalWorkerDisplacement => "Federal Worker Displacement",
            AdverseZoning => "Adverse Zoning",
            NewServicesRequested => "New Services Requested",
            ContinuedServicesRequested => "Continued Services Requested",
            SubrogationOpen => "Subrogation Open",
            MajorCorporationHighTech => "Major Corporation/High Tech",
            SidewalkIsPublic => "Sidewalk is Public",
            CollectiveBargainingAgreementSentByMail => {
                "Collective Bargaining Agreement Sent by Mail"
            }
            CollectiveBargainingAgreementSentByFacsimile => {
                "Collective Bargaining Agreement Sent by Facsimile"
            }
            Contract => "Contract",
            UnderContract => "Under Contract",
            RoadTestPerformedWithNoProblemsReported => {
                "Road Test Performed with No Problems Reported"
            }
            RoadTestPerformedWithProblemsReported => {
                "Road Test Performed with Problems Reported"
            }
            TiresBrandMatch => "Tires' Brand Match",
            RealEstateTaxesAreCurrent => "Real Estate Taxes are Current",
            HazardInsuranceIsCurrent => "Hazard Insurance is Current",
            TerminateGuarantee => "Terminate Guarantee",
            AtypicalExternalCondition => "Atypical External Condition",
            CodeT8 => {
                "Subsidence (Settlement of Ground Surface Caused by Loss of Support)"
            }
            UtilitiesInadequate => "Utilities Inadequate",
            CollectiveBargainingAgreementSentByElectronicBulletinBoard => {
                "Collective Bargaining Agreement Sent by Electronic Bulletin Board"
            }
            DebtorHasNoCreditorsHoldingUnsecuredNonpriorityClaims => {
                "Debtor has No Creditors Holding Unsecured Nonpriority Claims"
            }
            TransportViaCargoAircraft => "Transport via Cargo Aircraft",
            AnnualLeaveTaken => "Annual Leave Taken",
            ItemIsSpecialTestEquipment => "Item is Special Test Equipment",
            OperatesAsRepresentativeForOthers => "Operates as Representative For Others",
            ClaimInvolvesWorkRelatedDeath => "Claim Involves Work Related Death",
            CodeTH => {
                "Claim Does Not Involve Work Related Death, Days Away from Work, or Restricted Work Activity"
            }
            EmployeeHasNotRecoveredToReturnToWork => {
                "Employee Has Not Recovered to Return to Work"
            }
            EmployeeHasRetired => "Employee Has Retired",
            EmployeeHasResigned => "Employee Has Resigned",
            EmployeeIsPermanentlyAndTotallyDisabled => {
                "Employee is Permanently and Totally Disabled"
            }
            TractionMotorIsCutOut => "Traction Motor is Cut Out",
            AtypicalQualityOfConstruction => "Atypical Quality of Construction",
            TraumaticInjury => "Traumatic Injury",
            AtypicalRemodeling => "Atypical Remodeling",
            TransportViaPassengerAircraft => "Transport via Passenger Aircraft",
            AtypicalAdditions => "Atypical Additions",
            CodeTR => "Transfer to Bed, or Chair, or Both",
            AdverseMarketingConditionsInSubjectPropertysNeighborhood => {
                "Adverse Marketing Conditions in Subject Property's Neighborhood"
            }
            NeighborhoodWaterSourceIsPublic => "Neighborhood Water Source is Public",
            NeighborhoodSewageTreatmentIsPublic => {
                "Neighborhood Sewage Treatment is Public"
            }
            TelephoneNumberVerified => "Telephone Number Verified",
            NeighborhoodStreetIsPublic => "Neighborhood Street is Public",
            OtherMiscellaneousAdverseCharacteristics => {
                "Other Miscellaneous Adverse Characteristics"
            }
            SubjectPropertysStreetIsPublic => "Subject Property's Street is Public",
            SubjectPropertysSewageTreatmentIsPublic => {
                "Subject Property's Sewage Treatment is Public"
            }
            Disability => "Disability",
            MinimalChange => "Minimal Change",
            NeatAppearance => "Neat Appearance",
            NetWorthComputedAfterExemptions => "Net Worth Computed after Exemptions",
            NetWorthConsiderablyHigher => "Net Worth Considerably Higher",
            NetWorthHigher => "Net Worth Higher",
            NoEmployees => "No Employees",
            NoEmployeesBusinessManagedByOwner => {
                "No Employees - Business Managed by Owner"
            }
            NoEmployeesBusinessManagedByPartners => {
                "No Employees - Business Managed by Partners"
            }
            NotOutOfBusiness => "Not Out of Business",
            CodeUA => "Uninsurable, 1316 Property",
            ConductedAtAProfit => "Conducted at a Profit",
            ContingentDebtIndicated => "Contingent Debt Indicated",
            Continue => "Continue",
            ContractsObtainedByBid => "Contracts Obtained by Bid",
            ContractsObtainedByNegotiation => "Contracts Obtained by Negotiation",
            ConvertedToHoldingCompany => "Converted to Holding Company",
            CrossClaimFiled => "Cross Claim Filed",
            DecliningTendency => "Declining Tendency",
            CodeUJ => "Detrimental Events in Past, Relating to Business",
            CodeUK => "Detrimental Events in Past, Relating to Management",
            DownOrDeclineOrDecreased => "Down or Decline or Decreased",
            EmployeesIncludeOfficers => "Employees Include Officers",
            Uncooperative => "Uncooperative",
            EmployeesIncludeOwners => "Employees Include Owners",
            EmployeesIncludePartners => "Employees Include Partners",
            EmployeesIncludeTemporaryWorkers => "Employees Include Temporary Workers",
            EmployeesVaryAccordingToNeeds => "Employees Vary According to Needs",
            Enclosed => "Enclosed",
            UpAsTolerated => "Up as Tolerated",
            CodeUU => "Extent of Audit, if any, Not Indicated",
            FavorablePersonalReputation => "Favorable Personal Reputation",
            FiguresAreAbbreviated => "Figures are Abbreviated",
            FiguresAreConvertedToAgencyFormat => "Figures are Converted to Agency Format",
            FiguresAreIndividual => "Figures are Individual",
            FiguresAreRestated => "Figures are Restated",
            UltimateParentCompanyFinancialStatementUsed => {
                "Ultimate Parent Company Financial Statement Used"
            }
            ValidBorrowerAddressOrPhoneAttemptWithSchoolAttended => {
                "Valid Borrower Address or Phone Attempt with School Attended"
            }
            LenderDeterminedBorrowerMovedOutOfState => {
                "Lender Determined Borrower Moved Out of State"
            }
            LenderDeterminedBorrowerMovedBackIntoState => {
                "Lender Determined Borrower Moved Back into State"
            }
            LenderDeterminedBorrowerIncarcerated => {
                "Lender Determined Borrower Incarcerated"
            }
            LenderDeterminedBorrowerNoLongerIncarcerated => {
                "Lender Determined Borrower No Longer Incarcerated"
            }
            Original => "Original",
            TrueAndExactCopy => "True and Exact Copy",
            SubjectPropertysWaterSourceIsPublic => {
                "Subject Property's Water Source is Public"
            }
            PicturesRequired => "Pictures Required",
            IntercompanyRelationsExist => "Intercompany Relations Exist",
            InventoryValuedAtLowerOfCostOrMarket => {
                "Inventory Valued at Lower of Cost or Market"
            }
            InventoryValuedAtOtherMethods => "Inventory Valued at Other Methods",
            OperatesAsSoleAgent => "Operates as Sole Agent",
            WithoutPersonalJudgment => "Without Personal Judgment",
            WorkIsSubcontracted => "Work is Subcontracted",
            NotRegistered => "Not Registered",
            ImmediateAttentionRequired => "Immediate Attention Required",
            VehicleInspectionReportCompleted => "Vehicle Inspection Report Completed",
            MiddleToMedium => "Middle to Medium",
            RentControlLikely => "Rent Control Likely",
            Furnished => "Furnished",
            PriceRangeSingleFamilyOrPlannedUnitDevelopmentNotApplicable => {
                "Price Range Single Family or Planned Unit Development Not Applicable"
            }
            PriceRangeCondominiumNotApplicable => {
                "Price Range Condominium Not Applicable"
            }
            PriceRangeTwoToFourFamilyNotApplicable => {
                "Price Range Two to Four Family Not Applicable"
            }
            FinancialFiguresAreProjectedBasedOnSales => {
                "Financial Figures are Projected Based on Sales"
            }
            FinancialFiguresAreProjectedBasedOnEmployees => {
                "Financial Figures are Projected Based on Employees"
            }
            ParentCompanyHasBankruptcy => "Parent Company has Bankruptcy",
            HeadquartersHasBankruptcy => "Headquarters has Bankruptcy",
            CommercialMotorVehicleWasInvolvedInThisConviction => {
                "Commercial Motor Vehicle was Involved in this Conviction"
            }
            VehicleWasDeclaredATotalLoss => "Vehicle was Declared a Total Loss",
            CommercialMotorVehicleWasCarryingHazardousMaterialsWhenTheOffenseWasCommitted => {
                "Commercial Motor Vehicle was Carrying Hazardous Materials when the Offense was Committed"
            }
            PreparedFromInternalBookFigures => "Prepared from Internal Book Figures",
            QuantityDeclined => "Quantity Declined",
            QuantityDetailsUnknown => "Quantity Details Unknown",
            CodeVY => "Was tax paid when purchased by seller?",
            CodeVZ => "Was item depreciable?",
            StatementIsOnATradingTrust => "Statement is on a Trading Trust",
            NewRegistration => "New Registration",
            MailingAddressChange => "Mailing Address Change",
            ResidenceAddressChange => "Residence Address Change",
            NameChange => "Name Change",
            PartyEnrollmentChange => "Party Enrollment Change",
            NeedsAbsenteeBallot => "Needs Absentee Ballot",
            WouldLikeToBeElectionDayWorker => "Would Like to be Election Day Worker",
            DuplicateRegistration => "Duplicate Registration",
            ForwardedApplication => "Forwarded Application",
            WalkerRequired => "Walker Required",
            WaterOn => "Water On",
            ApplicationIncomplete => "Application Incomplete",
            VehiclePlateSurrendered => "Vehicle Plate Surrendered",
            WrittenNoticeToNoteHolder => "Written Notice to Note Holder",
            WrittenNoticeToBorrower => "Written Notice to Borrower",
            WithinSpecifiedTimePeriod => "Within Specified Time Period",
            WithinSpecifiedRange => "Within Specified Range",
            InjuryWasWorkRelated => "Injury was Work Related",
            DealerPricingAuthorization => "Dealer Pricing Authorization",
            SummaryLevel => "Summary Level Information",
            DetailLevel => "Detail Level Information",
            NonOccupantCoBorrower => "Non-occupant Co-borrower",
            CodeWN => "Unit is a Studio (Efficiency)",
            EquipmentInWorkingOrder => "Equipment in Working Order",
            ToBeWatched => "To be Watched",
            UndeterminedOutOfBusinessStatus => "Undetermined Out of Business Status",
            WheelchairRequired => "Wheelchair Required",
            BalanceSheetFiled => "Balance Sheet Filed",
            WinterizedTagObserved => "Winterized Tag Observed",
            MaterialSafetyDataSheet => "Material Safety Data Sheet",
            AcceptsCreditCards => "Accepts Credit Cards",
            AllPurchasesMadeFromHeadquarters => "All Purchases Made from Headquarters",
            Busy => "Busy",
            Excessive => "Excessive",
            FairlyNew => "Fairly new",
            CodeX0 => "No Employees - Business Managed by Director(s)",
            GrossWeeklyAmountIsEstimated => "Gross Weekly Amount is Estimated",
            WaitingPeriodDisabilityDaysAreNonConsecutive => {
                "Waiting Period Disability Days are Non-consecutive"
            }
            CodeX3 => "Report Depicts Most Recent Data - Interim Period(s) Omitted",
            PermanentImpairmentPaidAtMinimum => "Permanent Impairment Paid at Minimum",
            EmployeesDeathIsAResultOfWorkInjuryOrIllness => {
                "Employee's Death is a Result of Work Injury or Illness"
            }
            EmployeesWrittenSocialSecurityNumberReleaseIsOnFile => {
                "Employee's Written Social Security Number Release is on File"
            }
            EmployeesMedicalRecordsReleaseAuthorizationIsOnFile => {
                "Employee's Medical Records Release Authorization is on File"
            }
            EmployeeReturnedToWorkWithPreInjuryEmployer => {
                "Employee Returned to Work with Pre-Injury Employer"
            }
            CodeX9 => "\"Cafe\" Plan in Effect",
            FiguresAreAverage => "Figures are Average",
            Imports => "Imports",
            InProcessOfEstablishing => "In Process of Establishing",
            IntercompanyRelationsConsistOfEndorsements => {
                "Intercompany Relations Consist of Endorsements"
            }
            IntercompanyRelationsConsistOfGuarantees => {
                "Intercompany Relations Consist of Guarantees"
            }
            IntercompanyRelationsConsistOfLeasingArrangements => {
                "Intercompany Relations Consist of Leasing Arrangements"
            }
            IntercompanyRelationsConsistOfSharingAccounting => {
                "Intercompany Relations Consist of Sharing Accounting"
            }
            IntercompanyRelationsConsistOfSharingFacilities => {
                "Intercompany Relations Consist of Sharing Facilities"
            }
            IntercompanyRelationsConsistOfSharingManagement => {
                "Intercompany Relations Consist of Sharing Management"
            }
            IntercompanyRelationsConsistOfSharingPersonnel => {
                "Intercompany Relations Consist of Sharing Personnel"
            }
            CodeXK => "Interest in Other Business(es) Along with Family",
            CodeXL => {
                "Interest in Other Business(es) Along with Others in Reported Company"
            }
            InventoryValuedAtCompanysEstimates => {
                "Inventory Valued at Company's Estimates"
            }
            InventoryValuedAtCost => "Inventory Valued at Cost",
            CodeXO => "Inventory Valued using AVCO (Average Cost)",
            JointOwnership => "Joint Ownership",
            LeasesWithNoRentPayments => "Leases with No Rent Payments",
            LeasesWithOptionToBuy => "Leases with Option to Buy",
            LeasesWithTokenPayment => "Leases with Token Payment",
            Limited => "Limited",
            LocatedForSeveralYears => "Located for Several Years",
            LocatedSinceOpening => "Located Since Opening",
            Modern => "Modern",
            NonExistent => "Non-Existent",
            OfficerOrOwnerInOtherBusinessesInTheSameField => {
                "Officer or Owner in Other Businesses in the Same Field"
            }
            OperatesAsADistributorForOthers => "Operates as a Distributor for Others",
            InsuredCooperative => "Insured Cooperative",
            WorkedInIndustryForSeveralYears => "Worked in Industry for Several Years",
            AircraftOperation => "Aircraft Operation",
            AllClassificationsOnPolicyAccountedFor => {
                "All Classifications on Policy Accounted For"
            }
            BoardProvided => "Board Provided",
            CasualLabor => "Casual Labor",
            CertificatesOnFileForAllSubcontractors => {
                "Certificates on File for All Subcontractors"
            }
            CommissionsPaid => "Commissions Paid",
            ConditionOrTypeOfRecordsCauseAdditionalAuditTime => {
                "Condition or Type of Records Cause Additional Audit Time"
            }
            DomesticWorkersEmployed => "Domestic Workers Employed",
            OperatesFromResidence => "Operates from Residence",
            OperatesUnderLicenseByOthers => "Operates under License by Others",
            RentsFromMonthToMonth => "Rents from Month to Month",
            SemiModern => "Semi-modern",
            UnderConstruction => "Under Construction",
            Unlimited => "Unlimited",
            Used => "Used",
            Variable => "Variable",
            HolderIsASubsidiaryOfReportingAgent => {
                "Holder is a Subsidiary of Reporting Agent"
            }
            ContactIsUnchangedFromPreviousReport => {
                "Contact is Unchanged From Previous Report"
            }
            ReportWasFiledLastYearByThisAgent => {
                "Report was Filed Last Year by This Agent"
            }
            PartyIsAuthorizedToDoBusinessInThisState => {
                "Party is Authorized to do Business in This State"
            }
            ClearDecrease => "Clear Decrease",
            EmployeesTemporarilyLaidOff => "Employees Temporarily Laid Off",
            EstablishedInTheIndustry => "Established in the Industry",
            GlobalBusiness => "Global Business",
            InformationToBeFollowedUp => "Information to be Followed Up",
            KnownDetailsAreListed => "Known Details are Listed",
            LandIsRented => "Land is Rented",
            Low => "Low",
            PrimeCommercialArea => "Prime Commercial Area",
            CodeYV => "Shares with Affiliated Company(ies)",
            SlightlyHigher => "Slightly Higher",
            SlightlyLower => "Slightly Lower",
            Stagnant => "Stagnant",
            TerritoryInformationIsAvailable => "Territory Information is Available",
            SubcontractorsUsed => "Subcontractors Used",
            InsuredIsASubcontractor => "Insured Is a Subcontractor",
            InsuredHasMultipleEntries => "Insured Has Multiple Entries",
            InsuredHasRetailOperations => "Insured Has Retail Operations",
            CodeZ4 => "Insured Requested Division of Payroll of Employee(s)",
            OwnerOrOfficerInterviewed => "Owner or Officer Interviewed",
            PremiumOvertimeExcluded => "Premium Overtime Excluded",
            CodeZ7 => "Records Reflect Proper Division of Employee(s) Payroll",
            RecordsSatisfactoryForAudit => "Records Satisfactory for Audit",
            RelativesEmployed => "Relatives Employed",
            CustomerConfigurationChangeIsRequired => {
                "Customer - Configuration Change is Required"
            }
            CodeZB => {
                "Condition Board of Inspection and Survey (INSURV) is Mission Degrading"
            }
            CodeZC => {
                "Condition Board of Inspection and Survey (INSURV) is Maintenance Related"
            }
            CodeZD => {
                "Condition Board of Inspection and Survey (INSURV) is Safety Related"
            }
            RepairIsMissionEssential => "Repair is Mission Essential",
            RepairIsSafetyEssential => "Repair is Safety Essential",
            PeriodicMaintenanceIsRequired => "Periodic Maintenance is Required",
            CodeZH => {
                "Condition Board of Inspection and Survey (INSURV) Discrepancy is Corrected"
            }
            ProgressIsInJeopardy => "Progress is in Jeopardy",
            EmployeesInjuryOrIllnessIsWorkRelated => {
                "Employee's Injury or Illness is Work Related"
            }
            FinalConfigurationChangeIsRequired => {
                "Final - Configuration Change is Required"
            }
            FinalDeliveryToShopIsRequired => "Final - Delivery to Shop is Required",
            FinalRequestorWorkforceWillAssist => {
                "Final - Requestor Workforce will Assist"
            }
            JobIsLevel2 => "Job is Level 2",
            PreliminaryConfigurationChangeIsRequired => {
                "Preliminary - Configuration Change is Required"
            }
            PreliminaryDeliveryToShopIsRequired => {
                "Preliminary - Delivery to Shop is Required"
            }
            PreliminaryRequestorWorkforceWillAssist => {
                "Preliminary - Requestor Workforce will Assist"
            }
            ConfigurationChangeIsAssociatedWithTimeMeter => {
                "Configuration Change is Associated with Time Meter"
            }
            ShopHasLeadResponsibility => "Shop Has Lead Responsibility",
            EstimateIsDerivedFromJobTemplate => "Estimate is Derived From Job Template",
            RequestorHoldsTechnicalDocumentation => {
                "Requestor Holds Technical Documentation"
            }
            ReplacementItem => "Replacement Item",
            CodeZW => "Canadian Standards Association (CSA) Approved",
            NonConvertible => "Non-convertible",
            CodeZY => "Underwriters Laboratory (UL) Approved",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<ConditionIndicator> {
        {
            use ConditionIndicator::*;
            match description {
                "Requested" => Some(Requested),
                "In Progress" => Some(InProgress),
                "Automated Export System - Post Departure Authorized Special Status (AES-PASS) Standard" => {
                    Some(Code0A)
                }
                "Automated Export System - Post Departure Authorized Special Status (AES-PASS) Expanded" => {
                    Some(Code0B)
                }
                "Automated Export System - Post Departure Authorized Special Status (AES-PASS) Post Departure" => {
                    Some(Code0C)
                }
                "Facility's Emergency Response Plan Includes Information on Emergency Health Care" => {
                    Some(
                        FacilitysEmergencyResponsePlanIncludesInformationOnEmergencyHealthCare,
                    )
                }
                "Facility's Emergency Response Plan Includes Procedures for Informing Public and Local Agencies Responsible for Responding to an Accidental Release" => {
                    Some(
                        FacilitysEmergencyResponsePlanIncludesProceduresForInformingPublicAndLocalAgenciesResponsibleForRespondingToAnAccidentalRelease,
                    )
                }
                "Facility has a Clean Air Act Title V Operating Permit" => {
                    Some(FacilityHasACleanAirActTitleVOperatingPermit)
                }
                "Facility has a Written Emergency Response Plan" => {
                    Some(FacilityHasAWrittenEmergencyResponsePlan)
                }
                "Facility has Reportable Accidents" => {
                    Some(FacilityHasReportableAccidents)
                }
                "Facility is Covered by the Emergency Planning and\nCommunity Right to Know Act Section 302" => {
                    Some(Code0I)
                }
                "Facility is Covered by the Occupational Safety and\nHealth Act (OSHA) Process Safety Management\nStandard" => {
                    Some(Code0J)
                }
                "Facility is Included in the Community Emergency\nResponse Plan" => {
                    Some(Code0K)
                }
                "Hazardous Waste Mixed with Resource Conservation Recovery Act (RCRA)-Radioactive Material" => {
                    Some(Code0L)
                }
                "Offsite Responders Notified" => Some(OffsiteRespondersNotified),
                "Precipitation Present" => Some(PrecipitationPresent),
                "Disabled Veteran" => Some(DisabledVeteran),
                "Servicer has Advanced Funds to Pay for Delinquent Taxes on Non-escrowed Mortgage" => {
                    Some(
                        ServicerHasAdvancedFundsToPayForDelinquentTaxesOnNonEscrowedMortgage,
                    )
                }
                "Property Has Fire Insurance Only that was not Lender Placed" => {
                    Some(PropertyHasFireInsuranceOnlyThatWasNotLenderPlaced)
                }
                "Reported but Unconfirmed" => Some(ReportedButUnconfirmed),
                "Has Smoke Alarms" => Some(HasSmokeAlarms),
                "Operates as a Holding Company" => Some(OperatesAsAHoldingCompany),
                "Optimum" => Some(Optimum),
                "Renewed" => Some(Renewed),
                "Highest Educational Level" => Some(HighestEducationalLevel),
                "Principal Certificate" => Some(PrincipalCertificate),
                "Inservice Education Completed" => Some(InserviceEducationCompleted),
                "Main Assignment" => Some(MainAssignment),
                "Patient was admitted to a hospital" => {
                    Some(PatientWasAdmittedToAHospital)
                }
                "Patient is receiving anti-fungal therapy" => {
                    Some(PatientIsReceivingAntiFungalTherapy)
                }
                "Property is occupied by owner" => Some(PropertyIsOccupiedByOwner),
                "Property is occupied by tenant" => Some(PropertyIsOccupiedByTenant),
                "Property is vacant" => Some(PropertyIsVacant),
                "Location is urban" => Some(LocationIsUrban),
                "Location is suburban" => Some(LocationIsSuburban),
                "Location is rural" => Some(LocationIsRural),
                "Built-up over 75%" => Some(Code1H),
                "Built-up 25 - 75%" => Some(Code1I),
                "Built-up under 25%" => Some(Code1J),
                "Growth rate is rapid" => Some(GrowthRateIsRapid),
                "Class I-Left" => Some(ClassILeft),
                "Growth rate is stable" => Some(GrowthRateIsStable),
                "Growth rate is slow" => Some(GrowthRateIsSlow),
                "Property values are increasing" => Some(PropertyValuesAreIncreasing),
                "Property values are stable" => Some(PropertyValuesAreStable),
                "Property values are declining" => Some(PropertyValuesAreDeclining),
                "Class I-Right" => Some(ClassIRight),
                "Demand or supply is in shortage" => Some(DemandOrSupplyIsInShortage),
                "Demand or supply is in balance" => Some(DemandOrSupplyIsInBalance),
                "Demand or supply is over supply" => Some(DemandOrSupplyIsOverSupply),
                "Marketing time is under 3 months" => Some(MarketingTimeIsUnder3Months),
                "Marketing time is 3 to 6 months" => Some(MarketingTimeIs3To6Months),
                "Marketing time is over 6 months" => Some(MarketingTimeIsOver6Months),
                "Predominant occupancy is the owner" => {
                    Some(PredominantOccupancyIsTheOwner)
                }
                "Predominant occupancy is the tenant" => {
                    Some(PredominantOccupancyIsTheTenant)
                }
                "Patient was bed confined before the ambulance service" => {
                    Some(PatientWasBedConfinedBeforeTheAmbulanceService)
                }
                "Patient is receiving oral anti-fungal therapy" => {
                    Some(PatientIsReceivingOralAntiFungalTherapy)
                }
                "Predominant occupancy is vacant (0-5%)" => Some(Code2B),
                "Predominant occupancy is vacant (over 5%)" => Some(Code2C),
                "Developer or builder is in control of the Home Owners Association" => {
                    Some(DeveloperOrBuilderIsInControlOfTheHomeOwnersAssociation)
                }
                "Site is a corner lot" => Some(SiteIsACornerLot),
                "Zoning compliance is legal" => Some(ZoningComplianceIsLegal),
                "Zoning compliance is legal nonconforming (grandfather use)" => {
                    Some(Code2G)
                }
                "Zoning compliance is illegal" => Some(ZoningComplianceIsIllegal),
                "There is no zoning" => Some(ThereIsNoZoning),
                "Highest and best use as improved is the present use" => {
                    Some(HighestAndBestUseAsImprovedIsThePresentUse)
                }
                "Highest and best use as improved is other use" => {
                    Some(HighestAndBestUseAsImprovedIsOtherUse)
                }
                "Class II-Left" => Some(ClassIiLeft),
                "Property is located in a Federal Emergency Management Administration special flood hazard area" => {
                    Some(
                        PropertyIsLocatedInAFederalEmergencyManagementAdministrationSpecialFloodHazardArea,
                    )
                }
                "Appraisal is made \"as is\"" => Some(Code2N),
                "Appraisal is made subject to the repairs, alterations, inspections, or conditions listed" => {
                    Some(Code2O)
                }
                "Appraisal is made subject to the completion per plans and specifications" => {
                    Some(AppraisalIsMadeSubjectToTheCompletionPerPlansAndSpecifications)
                }
                "Project type is planned unit development (PUD)" => Some(Code2Q),
                "Class II-Right" => Some(ClassIiRight),
                "Project type is condominium" => Some(ProjectTypeIsCondominium),
                "Property rights are fee simple" => Some(PropertyRightsAreFeeSimple),
                "Property rights are leasehold" => Some(PropertyRightsAreLeasehold),
                "Supervisor appraiser inspected the property per supervisory appraiser's certification" => {
                    Some(
                        SupervisorAppraiserInspectedThePropertyPerSupervisoryAppraisersCertification,
                    )
                }
                "Property was sold within last 12 months" => {
                    Some(PropertyWasSoldWithinLast12Months)
                }
                "Appraiser signed statement of limiting conditions and disclaimer" => {
                    Some(AppraiserSignedStatementOfLimitingConditionsAndDisclaimer)
                }
                "Ownership interest in a property" => Some(OwnershipInterestInAProperty),
                "Termination" => Some(Termination),
                "Patient was bed confined after the ambulance service" => {
                    Some(PatientWasBedConfinedAfterTheAmbulanceService)
                }
                "Patient is receiving topical anti-fungal therapy" => {
                    Some(PatientIsReceivingTopicalAntiFungalTherapy)
                }
                "Points Paid by Seller" => Some(PointsPaidBySeller),
                "Points Paid by Buyer" => Some(PointsPaidByBuyer),
                "Seller Concession" => Some(SellerConcession),
                "Letter of Certification" => Some(LetterOfCertification),
                "Verbal Report Needed" => Some(VerbalReportNeeded),
                "Any Relationship Between Owner and Occupant" => {
                    Some(AnyRelationshipBetweenOwnerAndOccupant)
                }
                "Map and Directions to Remote Properties to Follow" => {
                    Some(MapAndDirectionsToRemotePropertiesToFollow)
                }
                "Ground Lease to Follow" => Some(GroundLeaseToFollow),
                "Disclosure Statement to Follow" => Some(DisclosureStatementToFollow),
                "Copy of Property Listing to Follow" => {
                    Some(CopyOfPropertyListingToFollow)
                }
                "Class III-Left" => Some(ClassIiiLeft),
                "Copy of Title Report Plat Map to Follow" => {
                    Some(CopyOfTitleReportPlatMapToFollow)
                }
                "Property Tax Bill to Follow" => Some(PropertyTaxBillToFollow),
                "Engineering or Soil Report to Follow" => {
                    Some(EngineeringOrSoilReportToFollow)
                }
                "Sales Contract Available" => Some(SalesContractAvailable),
                "Leave Will be Taken" => Some(LeaveWillBeTaken),
                "Class III-Right" => Some(ClassIiiRight),
                "Approved" => Some(Approved),
                "Balance Sheet does not balance" => Some(BalanceSheetDoesNotBalance),
                "Banking done through Parent Company" => {
                    Some(BankingDoneThroughParentCompany)
                }
                "Banking done through Related Concern" => {
                    Some(BankingDoneThroughRelatedConcern)
                }
                "Banking done through Subsidiary" => Some(BankingDoneThroughSubsidiary),
                "Can not determine if subject engaged in business" => {
                    Some(CanNotDetermineIfSubjectEngagedInBusiness)
                }
                "Deteriorated" => Some(Deteriorated),
                "Detrimental Information Received" => {
                    Some(DetrimentalInformationReceived)
                }
                "Patient was moved by stretcher" => Some(PatientWasMovedByStretcher),
                "Services are rendered within Hospice-elected period of coverage" => {
                    Some(ServicesAreRenderedWithinHospiceElectedPeriodOfCoverage)
                }
                "Accidents" => Some(Accidents),
                "Account Representative Transfer" => Some(AccountRepresentativeTransfer),
                "Additional Coverage" => Some(AdditionalCoverage),
                "Advice to Stop" => Some(AdviceToStop),
                "Agent Replacement" => Some(AgentReplacement),
                "Backup Withholding" => Some(BackupWithholding),
                "Current Employer" => Some(CurrentEmployer),
                "Current Occupation" => Some(CurrentOccupation),
                "Employer Reimbursement" => Some(EmployerReimbursement),
                "Employee Retirement Income Security Act (ERISA)" => Some(Code4K),
                "Expected Changes" => Some(ExpectedChanges),
                "Experimental" => Some(Experimental),
                "Foreign Flight" => Some(ForeignFlight),
                "Future Involvement" => Some(FutureInvolvement),
                "Grounding, Fine, Reprimand" => Some(Code4P),
                "Group Disability Insurance Conversion" => {
                    Some(GroupDisabilityInsuranceConversion)
                }
                "Group Disability Insurance Offset" => {
                    Some(GroupDisabilityInsuranceOffset)
                }
                "Group Disability Insurance Participation" => {
                    Some(GroupDisabilityInsuranceParticipation)
                }
                "Group Disability Insurance Top Up" => {
                    Some(GroupDisabilityInsuranceTopUp)
                }
                "Home Employment" => Some(HomeEmployment),
                "Information Omitted" => Some(InformationOmitted),
                "Injury Benefits" => Some(InjuryBenefits),
                "Issue at Higher Premiums" => Some(IssueAtHigherPremiums),
                "Issue With Exclusions" => Some(IssueWithExclusions),
                "Issue Without Benefits" => Some(IssueWithoutBenefits),
                "Patient was unconscious or in shock" => {
                    Some(PatientWasUnconsciousOrInShock)
                }
                "Treatment is rendered related to the terminal illness" => {
                    Some(TreatmentIsRenderedRelatedToTheTerminalIllness)
                }
                "Certified Aftermarket Parts Association (CAPA) Only" => Some(Code5B),
                "Certified Aftermarket Parts Association (CAPA) Preferred" => {
                    Some(Code5C)
                }
                "Juvenile Seen" => Some(JuvenileSeen),
                "Medical Treatment" => Some(MedicalTreatment),
                "Military Aviation" => Some(MilitaryAviation),
                "New Group" => Some(NewGroup),
                "Other Coverage Offset" => Some(OtherCoverageOffset),
                "Other Principals Being Insured" => Some(OtherPrincipalsBeingInsured),
                "Owner Active in Business" => Some(OwnerActiveInBusiness),
                "Payroll Deduction" => Some(PayrollDeduction),
                "Prepaid" => Some(Prepaid),
                "Previous Application" => Some(PreviousApplication),
                "Primary Occupation" => Some(PrimaryOccupation),
                "Racing Accident" => Some(RacingAccident),
                "Replacement" => Some(Replacement),
                "Resides With Applicant" => Some(ResidesWithApplicant),
                "Gender Distinct" => Some(GenderDistinct),
                "Sibling Coverage" => Some(SiblingCoverage),
                "Sickness Benefits" => Some(SicknessBenefits),
                "Special Dating" => Some(SpecialDating),
                "Spousal Consent" => Some(SpousalConsent),
                "Suitability Analysis" => Some(SuitabilityAnalysis),
                "Suitable for Coverage" => Some(SuitableForCoverage),
                "Taxable" => Some(Taxable),
                "This Company Replacement" => Some(ThisCompanyReplacement),
                "Patient was transported in an emergency situation" => {
                    Some(PatientWasTransportedInAnEmergencySituation)
                }
                "Treatment is rendered by a Hospice employed physician" => {
                    Some(TreatmentIsRenderedByAHospiceEmployedPhysician)
                }
                "United States Citizen" => Some(UnitedStatesCitizen),
                "Permanent Resident Alien" => Some(PermanentResidentAlien),
                "Borrower is First Time Homebuyer" => Some(BorrowerIsFirstTimeHomebuyer),
                "Unemployment Claims" => Some(UnemploymentClaims),
                "Unemployment Insurance Eligibility" => {
                    Some(UnemploymentInsuranceEligibility)
                }
                "Work Status" => Some(WorkStatus),
                "Workers Compensation Eligible" => Some(WorkersCompensationEligible),
                "Factored on Recourse Basis" => Some(FactoredOnRecourseBasis),
                "Factored with Advances" => Some(FactoredWithAdvances),
                "Figures are Actual" => Some(FiguresAreActual),
                "Figures are Anticipated" => Some(FiguresAreAnticipated),
                "Figures are Estimated" => Some(FiguresAreEstimated),
                "Figures are Modified" => Some(FiguresAreModified),
                "Figures are Projected" => Some(FiguresAreProjected),
                "Government Business Number Unavailable" => {
                    Some(GovernmentBusinessNumberUnavailable)
                }
                "Goodwill Origin Purchased from Bankrupt Company" => {
                    Some(GoodwillOriginPurchasedFromBankruptCompany)
                }
                "Goodwill Origin Rented" => Some(GoodwillOriginRented),
                "Has no ownership" => Some(HasNoOwnership),
                "Improved" => Some(Improved),
                "Intangibles breakdown available" => Some(IntangiblesBreakdownAvailable),
                "Intangibles include Organizational Expense" => {
                    Some(IntangiblesIncludeOrganizationalExpense)
                }
                "Intercompany relations consist of Loans and Advances" => {
                    Some(IntercompanyRelationsConsistOfLoansAndAdvances)
                }
                "Intercompany relations consist of Merchandise Transactions" => {
                    Some(IntercompanyRelationsConsistOfMerchandiseTransactions)
                }
                "Intercompany relations consist of Service Transactions" => {
                    Some(IntercompanyRelationsConsistOfServiceTransactions)
                }
                "Local banking utilized on a transfer account basis" => {
                    Some(LocalBankingUtilizedOnATransferAccountBasis)
                }
                "Patient had to be physically restrained" => {
                    Some(PatientHadToBePhysicallyRestrained)
                }
                "Treatment is rendered by a private attending physician" => {
                    Some(TreatmentIsRenderedByAPrivateAttendingPhysician)
                }
                "Medications Ordered are being Administered Intramuscularly" => {
                    Some(MedicationsOrderedAreBeingAdministeredIntramuscularly)
                }
                "Medications Ordered are being Administered Intravenously" => {
                    Some(MedicationsOrderedAreBeingAdministeredIntravenously)
                }
                "Medications Ordered are being Administered Orally" => {
                    Some(MedicationsOrderedAreBeingAdministeredOrally)
                }
                "Maintains no Inventory" => Some(MaintainsNoInventory),
                "Medications Ordered are being Administered Subcutaneously" => {
                    Some(MedicationsOrderedAreBeingAdministeredSubcutaneously)
                }
                "Majority" => Some(Majority),
                "Marketable Securities valued at cost" => {
                    Some(MarketableSecuritiesValuedAtCost)
                }
                "Marketable Securities valued at lower of cost or market" => {
                    Some(MarketableSecuritiesValuedAtLowerOfCostOrMarket)
                }
                "Interior Access Denied" => Some(InteriorAccessDenied),
                "Repairs are Recommended" => Some(RepairsAreRecommended),
                "Loan Originated under Shared Equity Plan" => {
                    Some(LoanOriginatedUnderSharedEquityPlan)
                }
                "Title and or Legal Issues Exist" => Some(TitleAndOrLegalIssuesExist),
                "Environmental Issues Exist" => Some(EnvironmentalIssuesExist),
                "Property is Listed As Is" => Some(PropertyIsListedAsIs),
                "Property is Listed as Repaired" => Some(PropertyIsListedAsRepaired),
                "Vacancy Rate is Greater Than 5 Percent to 10 Percent" => {
                    Some(VacancyRateIsGreaterThan5PercentTo10Percent)
                }
                "Vacancy Rate is Greater Than 10 Percent to 20 Percent" => {
                    Some(VacancyRateIsGreaterThan10PercentTo20Percent)
                }
                "Vacancy Rate is Greater Than 20 Percent" => {
                    Some(VacancyRateIsGreaterThan20Percent)
                }
                "Most Comparable Property" => Some(MostComparableProperty),
                "Anticipate Issues which Affect Ability to Secure Financing" => {
                    Some(AnticipateIssuesWhichAffectAbilityToSecureFinancing)
                }
                "Points are Paid by Seller" => Some(PointsArePaidBySeller),
                "Property Covered by Flood Insurance Policy" => {
                    Some(PropertyCoveredByFloodInsurancePolicy)
                }
                "Property Covered by Earthquake Insurance Policy" => {
                    Some(PropertyCoveredByEarthquakeInsurancePolicy)
                }
                "Points are Negotiable" => Some(PointsAreNegotiable),
                "Property is Currently Listed with a Real Estate Firm" => {
                    Some(PropertyIsCurrentlyListedWithARealEstateFirm)
                }
                "Patient had visible hemorrhaging" => Some(PatientHadVisibleHemorrhaging),
                "Treatment is curative" => Some(TreatmentIsCurative),
                "Income or Assets of Another Used" => Some(IncomeOrAssetsOfAnotherUsed),
                "Disclosure of Someone Else's Liabilities Required" => {
                    Some(DisclosureOfSomeoneElsesLiabilitiesRequired)
                }
                "Property Improvements \"to be made\"" => Some(Code8D),
                "Property Improvements \"have been made\"" => Some(Code8E),
                "Distant Suburban" => Some(DistantSuburban),
                "Self Employed" => Some(SelfEmployed),
                "Liability to be Satisfied" => Some(LiabilityToBeSatisfied),
                "Are Assets/Liabilities Reported Jointly" => {
                    Some(AreAssetsLiabilitiesReportedJointly)
                }
                "Location is Farm" => Some(LocationIsFarm),
                "Location is Resort" => Some(LocationIsResort),
                "Shortage Exist for Competing Listings" => {
                    Some(ShortageExistForCompetingListings)
                }
                "Competing Listings are in Balance" => {
                    Some(CompetingListingsAreInBalance)
                }
                "Oversupply Exist for Competing Listings" => {
                    Some(OversupplyExistForCompetingListings)
                }
                "Incentives are Offered" => Some(IncentivesAreOffered),
                "Listed Property has been Inspected" => {
                    Some(ListedPropertyHasBeenInspected)
                }
                "Sale Property has been Inspected" => Some(SalePropertyHasBeenInspected),
                "General Marketing Condition is Depressed" => {
                    Some(GeneralMarketingConditionIsDepressed)
                }
                "General Marketing Condition is Slow" => {
                    Some(GeneralMarketingConditionIsSlow)
                }
                "General Marketing Condition is Static" => {
                    Some(GeneralMarketingConditionIsStatic)
                }
                "General Marketing Condition is Improving" => {
                    Some(GeneralMarketingConditionIsImproving)
                }
                "General Marketing Condition is Excellent" => {
                    Some(GeneralMarketingConditionIsExcellent)
                }
                "Employment Conditions are Stable" => Some(EmploymentConditionsAreStable),
                "Employment Conditions are Declining" => {
                    Some(EmploymentConditionsAreDeclining)
                }
                "Employment Conditions are Increasing" => {
                    Some(EmploymentConditionsAreIncreasing)
                }
                "Overimprovement Condition Exists" => {
                    Some(OverimprovementConditionExists)
                }
                "Ambulance service was medically necessary" => {
                    Some(AmbulanceServiceWasMedicallyNecessary)
                }
                "Treatment is Palliative" => Some(TreatmentIsPalliative),
                "Involuntary Committal" => Some(InvoluntaryCommittal),
                "Lack of Available Equipment" => Some(LackOfAvailableEquipment),
                "Lack of Appropriate Facility within Reasonable Distance to Treat Patient in the Event of Complications" => {
                    Some(
                        LackOfAppropriateFacilityWithinReasonableDistanceToTreatPatientInTheEventOfComplications,
                    )
                }
                "Sudden Onset of Disorientation" => Some(SuddenOnsetOfDisorientation),
                "Sudden Onset of Severe, Incapacitating Pain" => Some(Code9F),
                "Continuous Hemorrhage from any Site with Abnormal Lab Values" => {
                    Some(ContinuousHemorrhageFromAnySiteWithAbnormalLabValues)
                }
                "Patient Requires Intensive IV Therapy" => {
                    Some(PatientRequiresIntensiveIvTherapy)
                }
                "Patient Requires Volume Expanders" => {
                    Some(PatientRequiresVolumeExpanders)
                }
                "Patient Requires Protective Isolation" => {
                    Some(PatientRequiresProtectiveIsolation)
                }
                "Patient Requires Frequent Monitoring" => {
                    Some(PatientRequiresFrequentMonitoring)
                }
                "Patient Requires Extended Post-operative Observation" => {
                    Some(PatientRequiresExtendedPostOperativeObservation)
                }
                "Foreclosure Proceedings Have Begun" => {
                    Some(ForeclosureProceedingsHaveBegun)
                }
                "Underimprovement Condition Exists" => {
                    Some(UnderimprovementConditionExists)
                }
                "Marketability of Property is Excellent" => {
                    Some(MarketabilityOfPropertyIsExcellent)
                }
                "Marketability of Property is Good" => {
                    Some(MarketabilityOfPropertyIsGood)
                }
                "Marketability of Property is Fair" => {
                    Some(MarketabilityOfPropertyIsFair)
                }
                "Marketability of Property is Poor" => {
                    Some(MarketabilityOfPropertyIsPoor)
                }
                "Fees are Current" => Some(FeesAreCurrent),
                "Fees Include Tennis" => Some(FeesIncludeTennis),
                "Fees Include Pool" => Some(FeesIncludePool),
                "Fees Include Insurance" => Some(FeesIncludeInsurance),
                "Fees Include Landscape" => Some(FeesIncludeLandscape),
                "Fees Include Other Amenities" => Some(FeesIncludeOtherAmenities),
                "Most Likely Buyer is Owner Occupant" => {
                    Some(MostLikelyBuyerIsOwnerOccupant)
                }
                "Most Likely Buyer is Investor" => Some(MostLikelyBuyerIsInvestor),
                "Patient is ambulatory" => Some(PatientIsAmbulatory),
                "Ambulation is Impaired and Walking Aid is Used for Therapy or Mobility" => {
                    Some(AmbulationIsImpairedAndWalkingAidIsUsedForTherapyOrMobility)
                }
                "Patient is confined to a bed or chair" => {
                    Some(PatientIsConfinedToABedOrChair)
                }
                "Patient is Confined to a Room or an Area Without Bathroom Facilities" => {
                    Some(PatientIsConfinedToARoomOrAnAreaWithoutBathroomFacilities)
                }
                "Ambulation is Impaired and Walking Aid is Used for Mobility" => {
                    Some(AmbulationIsImpairedAndWalkingAidIsUsedForMobility)
                }
                "Patient Condition Requires Positioning of the Body or Attachments Which Would Not be Feasible With the Use of an Ordinary Bed" => {
                    Some(
                        PatientConditionRequiresPositioningOfTheBodyOrAttachmentsWhichWouldNotBeFeasibleWithTheUseOfAnOrdinaryBed,
                    )
                }
                "Patient needs a trapeze bar to sit up due to respiratory condition or change body positions for other medical reasons" => {
                    Some(
                        PatientNeedsATrapezeBarToSitUpDueToRespiratoryConditionOrChangeBodyPositionsForOtherMedicalReasons,
                    )
                }
                "Patient's Ability to Breathe is Severely Impaired" => {
                    Some(PatientsAbilityToBreatheIsSeverelyImpaired)
                }
                "Patient condition requires frequent and/or immediate changes in body positions" => {
                    Some(
                        PatientConditionRequiresFrequentAndOrImmediateChangesInBodyPositions,
                    )
                }
                "Patient can operate controls" => Some(PatientCanOperateControls),
                "Siderails Are to be Attached to a Hospital Bed Owned by the Beneficiary" => {
                    Some(SiderailsAreToBeAttachedToAHospitalBedOwnedByTheBeneficiary)
                }
                "Patient owns equipment" => Some(PatientOwnsEquipment),
                "Mattress or Siderails are Being Used with Prescribed Medically Necessary Hospital Bed Owned by the Beneficiary" => {
                    Some(
                        MattressOrSiderailsAreBeingUsedWithPrescribedMedicallyNecessaryHospitalBedOwnedByTheBeneficiary,
                    )
                }
                "Patient Needs Lift to Get In or Out of Bed or to Assist in Transfer from Bed to Wheelchair" => {
                    Some(
                        PatientNeedsLiftToGetInOrOutOfBedOrToAssistInTransferFromBedToWheelchair,
                    )
                }
                "Patient has an orthopedic impairment requiring traction equipment which prevents ambulation during period of use" => {
                    Some(
                        PatientHasAnOrthopedicImpairmentRequiringTractionEquipmentWhichPreventsAmbulationDuringPeriodOfUse,
                    )
                }
                "Item has been prescribed as part of a planned regimen of treatment in patient home" => {
                    Some(
                        ItemHasBeenPrescribedAsPartOfAPlannedRegimenOfTreatmentInPatientHome,
                    )
                }
                "Patient is highly susceptible to decubitus ulcers" => {
                    Some(PatientIsHighlySusceptibleToDecubitusUlcers)
                }
                "Patient or a care-giver has been instructed in use of equipment" => {
                    Some(PatientOrACareGiverHasBeenInstructedInUseOfEquipment)
                }
                "Patient has poor diabetic control" => {
                    Some(PatientHasPoorDiabeticControl)
                }
                "A 6-7 hour nocturnal study documents 30 episodes of apnea each lasting more than 10 seconds" => {
                    Some(
                        A67HourNocturnalStudyDocuments30EpisodesOfApneaEachLastingMoreThan10Seconds,
                    )
                }
                "Without the equipment, the patient would require surgery" => {
                    Some(Code30)
                }
                "Patient has had a total knee replacement" => {
                    Some(PatientHasHadATotalKneeReplacement)
                }
                "Patient has intractable lymphedema of the extremities" => {
                    Some(PatientHasIntractableLymphedemaOfTheExtremities)
                }
                "Patient is in a nursing home" => Some(PatientIsInANursingHome),
                "Patient is conscious" => Some(PatientIsConscious),
                "This Feeding is the Only Form of Nutritional Intake for This Patient" => {
                    Some(ThisFeedingIsTheOnlyFormOfNutritionalIntakeForThisPatient)
                }
                "Patient was administered premix" => Some(PatientWasAdministeredPremix),
                "Oxygen delivery equipment is stationary" => {
                    Some(OxygenDeliveryEquipmentIsStationary)
                }
                "Certification signed by the physician is on file at the supplier's office" => {
                    Some(CertificationSignedByThePhysicianIsOnFileAtTheSuppliersOffice)
                }
                "Patient Has Mobilizing Respiratory Tract Secretions" => {
                    Some(PatientHasMobilizingRespiratoryTractSecretions)
                }
                "Patient or Caregiver is Capable of Using the Equipment Without Technical or Professional Supervision" => {
                    Some(
                        PatientOrCaregiverIsCapableOfUsingTheEquipmentWithoutTechnicalOrProfessionalSupervision,
                    )
                }
                "Patient or Caregiver is Unable to Propel or Lift a Standard Weight Wheelchair" => {
                    Some(
                        PatientOrCaregiverIsUnableToPropelOrLiftAStandardWeightWheelchair,
                    )
                }
                "Patient Requires Leg Elevation for Edema or Body Alignment" => {
                    Some(PatientRequiresLegElevationForEdemaOrBodyAlignment)
                }
                "Patient Weight or Usage Needs Necessitate a Heavy Duty Wheelchair" => {
                    Some(PatientWeightOrUsageNeedsNecessitateAHeavyDutyWheelchair)
                }
                "Patient Requires Reclining Function of a Wheelchair" => {
                    Some(PatientRequiresRecliningFunctionOfAWheelchair)
                }
                "Patient is Unable to Operate a Wheelchair Manually" => {
                    Some(PatientIsUnableToOperateAWheelchairManually)
                }
                "Patient or Caregiver Requires Side Transfer into Wheelchair, Commode or Other" => {
                    Some(Code46)
                }
                "Advertisement Run Condition" => Some(AdvertisementRunCondition),
                "Individual Paid for Last Day Worked" => {
                    Some(IndividualPaidForLastDayWorked)
                }
                "Full Wages Paid for Date of Injury" => {
                    Some(FullWagesPaidForDateOfInjury)
                }
                "Citation or Ticket Issued" => Some(CitationOrTicketIssued),
                "Individual is Member of Policyholder's Household" => {
                    Some(IndividualIsMemberOfPolicyholdersHousehold)
                }
                "Individual Permitted to Use Vehicle" => {
                    Some(IndividualPermittedToUseVehicle)
                }
                "Individual Wore Seatbelt" => Some(IndividualWoreSeatbelt),
                "Child Restraint Device in Vehicle" => {
                    Some(ChildRestraintDeviceInVehicle)
                }
                "Child Restraint Device Used" => Some(ChildRestraintDeviceUsed),
                "Individual Injured" => Some(IndividualInjured),
                "Individual Transported to Another Location" => {
                    Some(IndividualTransportedToAnotherLocation)
                }
                "Durable Medical Equipment (DME) Purchased New" => Some(Code58),
                "Durable Medical Equipment (DME) Is Under Warranty" => Some(Code59),
                "Transportation Was To the Nearest Facility" => {
                    Some(TransportationWasToTheNearestFacility)
                }
                "Employee is Exempt" => Some(EmployeeIsExempt),
                "Claimant is Covered on the Employer's Long-term Disability Plan" => {
                    Some(ClaimantIsCoveredOnTheEmployersLongTermDisabilityPlan)
                }
                "Employee's Job Responsibilities Changed Due to the Disabling Condition" => {
                    Some(EmployeesJobResponsibilitiesChangedDueToTheDisablingCondition)
                }
                "Employer Has a Return to Work Policy for Disabled Employees" => {
                    Some(EmployerHasAReturnToWorkPolicyForDisabledEmployees)
                }
                "Open" => Some(Open),
                "Normal" => Some(Normal),
                "Closed-moderate" => Some(ClosedModerate),
                "Severe" => Some(Severe),
                "Moderate" => Some(Moderate),
                "Straight" => Some(Straight),
                "Convex" => Some(Convex),
                "Concave" => Some(Concave),
                "Double Protrusion" => Some(DoubleProtrusion),
                "No Crossbite" => Some(NoCrossbite),
                "Posterior" => Some(Posterior),
                "Anterior" => Some(Anterior),
                "Maxillary" => Some(Maxillary),
                "Mandibular" => Some(Mandibular),
                "Right" => Some(Right),
                "Left" => Some(Left),
                "Maxillary Moderate" => Some(MaxillaryModerate),
                "Mandibular Moderate" => Some(MandibularModerate),
                "Maxillary Severe" => Some(MaxillarySevere),
                "Mandibular Severe" => Some(MandibularSevere),
                "Income Has Been Verified" => Some(IncomeHasBeenVerified),
                "Person Has Been Interviewed" => Some(PersonHasBeenInterviewed),
                "Rent Has Been Verified" => Some(RentHasBeenVerified),
                "Employer Has Been Verified" => Some(EmployerHasBeenVerified),
                "Position Has Been Verified" => Some(PositionHasBeenVerified),
                "Inquiry Has Been Verified" => Some(InquiryHasBeenVerified),
                "Outstanding Judgments" => Some(OutstandingJudgments),
                "Declared Bankruptcy in Past 7 Years" => {
                    Some(DeclaredBankruptcyInPast7Years)
                }
                "Foreclosure or Deed in Lieu in Past 7 Years" => {
                    Some(ForeclosureOrDeedInLieuInPast7Years)
                }
                "Party to Lawsuit" => Some(PartyToLawsuit),
                "Obligated on a Loan Foreclosed, Deed in Lieu of Judgment" => {
                    Some(Code95)
                }
                "Currently Delinquent or in Default" => {
                    Some(CurrentlyDelinquentOrInDefault)
                }
                "Obligated to Pay Alimony, Child Support or Maintenance" => Some(Code97),
                "Part of Down Payment Borrowed" => Some(PartOfDownPaymentBorrowed),
                "Co-maker or Endorser on a Note" => Some(CoMakerOrEndorserOnANote),
                "Liability Coverage Will Transfer" => Some(LiabilityCoverageWillTransfer),
                "Most Likely Buyer is Other Person or Entity" => {
                    Some(MostLikelyBuyerIsOtherPersonOrEntity)
                }
                "Potential Financing is Fannie Mae" => {
                    Some(PotentialFinancingIsFannieMae)
                }
                "Suppress Paper Endorsement" => Some(SuppressPaperEndorsement),
                "Do Not Suppress Paper Endorsement" => {
                    Some(DoNotSuppressPaperEndorsement)
                }
                "Escrow" => Some(Escrow),
                "Teaching Minor" => Some(TeachingMinor),
                "Sub-servicer Submitted" => Some(SubServicerSubmitted),
                "First Mortgage" => Some(FirstMortgage),
                "Second Mortgage" => Some(SecondMortgage),
                "Amputation" => Some(Amputation),
                "Address Skip Begin" => Some(AddressSkipBegin),
                "Address Corrected" => Some(AddressCorrected),
                "Automatic Drill Time Calculated" => Some(AutomaticDrillTimeCalculated),
                "Automatic Edging Time Calculated" => Some(AutomaticEdgingTimeCalculated),
                "Automatically Select" => Some(AutomaticallySelect),
                "Accepting Family Members" => Some(AcceptingFamilyMembers),
                "Agitated" => Some(Agitated),
                "Automatically Search and List" => Some(AutomaticallySearchAndList),
                "Address Incorrect" => Some(AddressIncorrect),
                "Assumable" => Some(Assumable),
                "Potential Financing is Cash" => Some(PotentialFinancingIsCash),
                "Ambulation Limitations" => Some(AmbulationLimitations),
                "Potential Financing is Outside Lender" => {
                    Some(PotentialFinancingIsOutsideLender)
                }
                "Address Incomplete" => Some(AddressIncomplete),
                "Accept Certification without Changes" => {
                    Some(AcceptCertificationWithoutChanges)
                }
                "Alley is Public" => Some(AlleyIsPublic),
                "Potential Financing is Federal Housing Administration" => {
                    Some(PotentialFinancingIsFederalHousingAdministration)
                }
                "Address Skip Resolved" => Some(AddressSkipResolved),
                "Address Skip Exhaust" => Some(AddressSkipExhaust),
                "Accept Statement of Limiting Conditions without Changes" => {
                    Some(AcceptStatementOfLimitingConditionsWithoutChanges)
                }
                "Automatic Underside Time Calculated" => {
                    Some(AutomaticUndersideTimeCalculated)
                }
                "Available - Not Used" => Some(AvailableNotUsed),
                "Accept Certification with Changes" => {
                    Some(AcceptCertificationWithChanges)
                }
                "Accept Statement of Limiting Conditions with Changes" => {
                    Some(AcceptStatementOfLimitingConditionsWithChanges)
                }
                "Adjacent Track Occupied" => Some(AdjacentTrackOccupied),
                "Potential Financing is Veterans Affairs" => {
                    Some(PotentialFinancingIsVeteransAffairs)
                }
                "Uninsured Motorist Coverage Will Transfer" => {
                    Some(UninsuredMotoristCoverageWillTransfer)
                }
                "Mortgage in Foreclosure" => Some(MortgageInForeclosure),
                "Real Estate Owned (REO) Mortgage" => Some(CodeB2),
                "Potential Financing is Contract for Deed" => {
                    Some(PotentialFinancingIsContractForDeed)
                }
                "Only the Exterior has been Inspected" => {
                    Some(OnlyTheExteriorHasBeenInspected)
                }
                "Real Estate Owned Property or Foreclosure Property" => {
                    Some(RealEstateOwnedPropertyOrForeclosureProperty)
                }
                "Number of Comparable Listings is Normal" => {
                    Some(NumberOfComparableListingsIsNormal)
                }
                "Number of Comparable Listings is an Oversupply" => {
                    Some(NumberOfComparableListingsIsAnOversupply)
                }
                "Number of Comparable Listings is a Shortage" => {
                    Some(NumberOfComparableListingsIsAShortage)
                }
                "Property Management Expenses Outstanding" => {
                    Some(PropertyManagementExpensesOutstanding)
                }
                "Borrower Letter Attempt" => Some(BorrowerLetterAttempt),
                "Building or Mobile Home is in a Coastal Barrier Resources Area" => {
                    Some(BuildingOrMobileHomeIsInACoastalBarrierResourcesArea)
                }
                "Borrower Telephone Contact" => Some(BorrowerTelephoneContact),
                "Business Pending" => Some(BusinessPending),
                "Borrower Letter Contact" => Some(BorrowerLetterContact),
                "Marketable Securities valued at market" => {
                    Some(MarketableSecuritiesValuedAtMarket)
                }
                "Appropriate Improvement Condition Exists" => {
                    Some(AppropriateImprovementConditionExists)
                }
                "Name unknown to local authorities" => {
                    Some(NameUnknownToLocalAuthorities)
                }
                "No manufacturing done on Premises" => {
                    Some(NoManufacturingDoneOnPremises)
                }
                "Occasional" => Some(Occasional),
                "Officer or owner in other Businesses" => {
                    Some(OfficerOrOwnerInOtherBusinesses)
                }
                "Bowel Limitations, Bladder Limitations, or both (Incontinence)" => {
                    Some(CodeBL)
                }
                "Old" => Some(Old),
                "Operates on part time basis" => Some(OperatesOnPartTimeBasis),
                "Parent Financial Statement Used" => Some(ParentFinancialStatementUsed),
                "Borrower Payment Received" => Some(BorrowerPaymentReceived),
                "Beneficiary is Partially Dependent" => {
                    Some(BeneficiaryIsPartiallyDependent)
                }
                "Product Information Available" => Some(ProductInformationAvailable),
                "Bedrest BRP (Bathroom Privileges)" => Some(CodeBR),
                "Revenue derived from Commissions" => Some(RevenueDerivedFromCommissions),
                "Borrower Telephone Attempt" => Some(BorrowerTelephoneAttempt),
                "Beneficiary is Totally Dependent" => Some(BeneficiaryIsTotallyDependent),
                "Revenue derived from Donations" => Some(RevenueDerivedFromDonations),
                "Revenue derived from Fees" => Some(RevenueDerivedFromFees),
                "Revenue derived from Grants" => Some(RevenueDerivedFromGrants),
                "Revenue derived from Taxes" => Some(RevenueDerivedFromTaxes),
                "Sprinkler Equipped" => Some(SprinklerEquipped),
                "Statement requested from Government Registry" => {
                    Some(StatementRequestedFromGovernmentRegistry)
                }
                "Collision Coverage Will Transfer" => Some(CollisionCoverageWillTransfer),
                "Advances From Property Management Expenses Outstanding" => {
                    Some(AdvancesFromPropertyManagementExpensesOutstanding)
                }
                "Final Demand Letter Sent" => Some(FinalDemandLetterSent),
                "Lender Request for Assistance" => Some(LenderRequestForAssistance),
                "Mortgage has Lender-purchased Mortgage Insurance" => {
                    Some(MortgageHasLenderPurchasedMortgageInsurance)
                }
                "Insufficient Funds" => Some(InsufficientFunds),
                "Credit Enhanced Mortgage" => Some(CreditEnhancedMortgage),
                "Corporate Appointment" => Some(CorporateAppointment),
                "Special Servicing Required" => Some(SpecialServicingRequired),
                "Client Specifically Requested Consideration of Special Financing or an Assumable Loan" => {
                    Some(
                        ClientSpecificallyRequestedConsiderationOfSpecialFinancingOrAnAssumableLoan,
                    )
                }
                "Cane Required" => Some(CaneRequired),
                "Complete Bedrest" => Some(CompleteBedrest),
                "Collection Card was Left" => Some(CollectionCardWasLeft),
                "Call to Directory Assistance for Reference Telephone" => {
                    Some(CallToDirectoryAssistanceForReferenceTelephone)
                }
                "Co-signer Telephone Attempt" => Some(CoSignerTelephoneAttempt),
                "Co-signer Telephone Contact" => Some(CoSignerTelephoneContact),
                "Claim is Fraudulent" => Some(ClaimIsFraudulent),
                "Co-signer Delinquency Letter Sent" => {
                    Some(CoSignerDelinquencyLetterSent)
                }
                "Co-signer Final Demand Letter Sent" => {
                    Some(CoSignerFinalDemandLetterSent)
                }
                "Call to Directory Assistance for Co-signer Telephone" => {
                    Some(CallToDirectoryAssistanceForCoSignerTelephone)
                }
                "Valid Borrower Address or Phone Attempt with Previous Holder" => {
                    Some(ValidBorrowerAddressOrPhoneAttemptWithPreviousHolder)
                }
                "Convertible" => Some(Convertible),
                "Claimant had a Pre-existing Injury" => {
                    Some(ClaimantHadAPreExistingInjury)
                }
                "Comatose" => Some(Comatose),
                "Common Elements are Leased to or by the Home Owners' Association" => {
                    Some(CommonElementsAreLeasedToOrByTheHomeOwnersAssociation)
                }
                "Cumulative Injury" => Some(CumulativeInjury),
                "Contracture" => Some(Contracture),
                "Case Pending" => Some(CasePending),
                "Callable" => Some(Callable),
                "Crutches Required" => Some(CrutchesRequired),
                "Community Participates in National Flood Insurance Program" => {
                    Some(CommunityParticipatesInNationalFloodInsuranceProgram)
                }
                "Common Elements are Completed" => Some(CommonElementsAreCompleted),
                "Curb and Gutter are Public" => Some(CurbAndGutterArePublic),
                "Cooperative" => Some(Cooperative),
                "Cooling Water is Low" => Some(CoolingWaterIsLow),
                "Certification Status" => Some(CertificationStatus),
                "Car Spaces are Adequate" => Some(CarSpacesAreAdequate),
                "Car Spaces are Inadequate" => Some(CarSpacesAreInadequate),
                "Comprehensive Coverage Will Transfer" => {
                    Some(ComprehensiveCoverageWillTransfer)
                }
                "Issue Check Payable to Borrower and Return to Servicer" => {
                    Some(IssueCheckPayableToBorrowerAndReturnToServicer)
                }
                "Issue Check Payable to Servicer and Return to Servicer" => {
                    Some(IssueCheckPayableToServicerAndReturnToServicer)
                }
                "Issue Check Payable to Borrower and Send to Borrower" => {
                    Some(IssueCheckPayableToBorrowerAndSendToBorrower)
                }
                "Issue Check Payable to Servicer or Borrower and Return to Servicer" => {
                    Some(IssueCheckPayableToServicerOrBorrowerAndReturnToServicer)
                }
                "Issue Check Payable to Other Payee" => {
                    Some(IssueCheckPayableToOtherPayee)
                }
                "Positive" => Some(Positive),
                "Negative" => Some(Negative),
                "Taxes are Typical for the Area and Price Range" => {
                    Some(TaxesAreTypicalForTheAreaAndPriceRange)
                }
                "Improvement Conforms to Zoning Regulations" => {
                    Some(ImprovementConformsToZoningRegulations)
                }
                "Call to Directory Assistance for Borrower Telephone" => {
                    Some(CallToDirectoryAssistanceForBorrowerTelephone)
                }
                "Deferment or Forbearance Begin" => Some(DefermentOrForbearanceBegin),
                "Declined" => Some(Declined),
                "Borrower Furnished Demographic Data" => {
                    Some(BorrowerFurnishedDemographicData)
                }
                "Deferment or Forbearance End" => Some(DefermentOrForbearanceEnd),
                "Funds available for Unsecured Creditors" => {
                    Some(FundsAvailableForUnsecuredCreditors)
                }
                "Deductible Amount Fully Recovered" => {
                    Some(DeductibleAmountFullyRecovered)
                }
                "Dynamic Brakes are Out" => Some(DynamicBrakesAreOut),
                "Debtor has been Domiciled" => Some(DebtorHasBeenDomiciled),
                "Disoriented" => Some(Disoriented),
                "Dynamic Brakes are Operational" => Some(DynamicBrakesAreOperational),
                "Construction Warranty" => Some(ConstructionWarranty),
                "Construction Warranty Transferable" => {
                    Some(ConstructionWarrantyTransferable)
                }
                "Maintenance Drug under Client's Benefit Plan" => {
                    Some(MaintenanceDrugUnderClientsBenefitPlan)
                }
                "Payment Reduced Because Maximum Allowable Cost Exceeded" => {
                    Some(PaymentReducedBecauseMaximumAllowableCostExceeded)
                }
                "Deductible Amount Not Fully Recovered" => {
                    Some(DeductibleAmountNotFullyRecovered)
                }
                "Benefits Terminated Prior to Service Date" => {
                    Some(BenefitsTerminatedPriorToServiceDate)
                }
                "Depressed" => Some(Depressed),
                "Drug Part of Formulary Data Base" => Some(DrugPartOfFormularyDataBase),
                "Subject not Engaged in Business" => Some(SubjectNotEngagedInBusiness),
                "All Door Seals are Intact" => Some(AllDoorSealsAreIntact),
                "Filing Fee Attached" => Some(FilingFeeAttached),
                "Subject not Engaged in Business at Requested Address" => {
                    Some(SubjectNotEngagedInBusinessAtRequestedAddress)
                }
                "Suspended" => Some(Suspended),
                "Total" => Some(Total),
                "Unable to Respond" => Some(UnableToRespond),
                "Dyspnea with Minimal Exertion" => Some(DyspneaWithMinimalExertion),
                "Uses Own Facilities" => Some(UsesOwnFacilities),
                "Figures are Total" => Some(FiguresAreTotal),
                "Fixed Asset Breakdown Undisclosed" => {
                    Some(FixedAssetBreakdownUndisclosed)
                }
                "For the Fiscal Year" => Some(ForTheFiscalYear),
                "For the Period" => Some(ForThePeriod),
                "Formed by Consolidation" => Some(FormedByConsolidation),
                "Formed by Merger" => Some(FormedByMerger),
                "Prior Bankruptcy Case Filed in Last 6 Years" => {
                    Some(PriorBankruptcyCaseFiledInLast6Years)
                }
                "Debtor is not Represented by an Attorney" => {
                    Some(DebtorIsNotRepresentedByAnAttorney)
                }
                "A Pending Case has been Filed" => Some(APendingCaseHasBeenFiled),
                "Guaranteed by Parent Company" => Some(GuaranteedByParentCompany),
                "Has Authority for All Purchases" => Some(HasAuthorityForAllPurchases),
                "Has Authority to Purchase Supplies" => {
                    Some(HasAuthorityToPurchaseSupplies)
                }
                "Equipment Certified" => Some(EquipmentCertified),
                "Has Business Interruption Insurance" => {
                    Some(HasBusinessInterruptionInsurance)
                }
                "Has Class of Stock" => Some(HasClassOfStock),
                "Has Extended Coverage Insurance" => Some(HasExtendedCoverageInsurance),
                "Has Fire Insurance" => Some(HasFireInsurance),
                "Has Joint Authority" => Some(HasJointAuthority),
                "Has Life Insurance" => Some(HasLifeInsurance),
                "Existence of Preliminary Flood Determination" => {
                    Some(ExistenceOfPreliminaryFloodDetermination)
                }
                "Existence of Community Participation in the National Flood Insurance" => {
                    Some(ExistenceOfCommunityParticipationInTheNationalFloodInsurance)
                }
                "Endurance Limitations" => Some(EnduranceLimitations),
                "Has Marriage Contract" => Some(HasMarriageContract),
                "Electricity On" => Some(ElectricityOn),
                "Equipment Is Overhauled" => Some(EquipmentIsOverhauled),
                "Exercises Prescribed" => Some(ExercisesPrescribed),
                "Has No Par Value" => Some(HasNoParValue),
                "Engine Start-Up Performed with No Problems Reported" => {
                    Some(EngineStartUpPerformedWithNoProblemsReported)
                }
                "Engine Start-Up Performed with Problems Reported" => {
                    Some(EngineStartUpPerformedWithProblemsReported)
                }
                "Electrical Control System Shut Down" => {
                    Some(ElectricalControlSystemShutDown)
                }
                "Has Other Insurance" => Some(HasOtherInsurance),
                "Has Par Value" => Some(HasParValue),
                "Has Sole Authority" => Some(HasSoleAuthority),
                "Excellent" => Some(Excellent),
                "Has Voting Rights" => Some(HasVotingRights),
                "Heading Address in Registered Office Only" => {
                    Some(HeadingAddressInRegisteredOfficeOnly)
                }
                "High Level" => Some(HighLevel),
                "Homeworkers Employed" => Some(HomeworkersEmployed),
                "In Subscriber Shares" => Some(InSubscriberShares),
                "Inactive" => Some(Inactive),
                "Incomplete" => Some(Incomplete),
                "Incorporation Details Requested" => Some(IncorporationDetailsRequested),
                "Increase or Up" => Some(IncreaseOrUp),
                "Information Cannot Be Provided at This Time" => {
                    Some(InformationCannotBeProvidedAtThisTime)
                }
                "Information in Date" => Some(InformationInDate),
                "Information Requires Investigation" => {
                    Some(InformationRequiresInvestigation)
                }
                "Actions has a Significant Environmental Effect" => {
                    Some(ActionsHasASignificantEnvironmentalEffect)
                }
                "Application Includes Complete System" => {
                    Some(ApplicationIncludesCompleteSystem)
                }
                "Antenna is Mounted on a Structure with an Existing Antenna" => {
                    Some(AntennaIsMountedOnAStructureWithAnExistingAntenna)
                }
                "Notice of Construction or Alteration has been Filed" => {
                    Some(NoticeOfConstructionOrAlterationHasBeenFiled)
                }
                "Applicant Wants to Monitor Frequency" => {
                    Some(ApplicantWantsToMonitorFrequency)
                }
                "Applicant has been Denied Government Benefits Due to Use of Drugs" => {
                    Some(ApplicantHasBeenDeniedGovernmentBenefitsDueToUseOfDrugs)
                }
                "Application is Certified" => Some(ApplicationIsCertified),
                "Application is for other Than a New Station" => {
                    Some(ApplicationIsForOtherThanANewStation)
                }
                "Fee Required" => Some(FeeRequired),
                "Flood Status" => Some(FloodStatus),
                "Flood Insurance Required" => Some(FloodInsuranceRequired),
                "Federal Flood Insurance is Available (Community Participates)" => {
                    Some(CodeFL)
                }
                "Inventory Valued Using LIFO (Last In/First Out)" => Some(CodeFM),
                "Not Too High Level" => Some(NotTooHighLevel),
                "Forgetful" => Some(Forgetful),
                "Flood Certification with Life of Loan" => {
                    Some(FloodCertificationWithLifeOfLoan)
                }
                "Street Maintenance is Public" => Some(StreetMaintenanceIsPublic),
                "Fair" => Some(Fair),
                "Not Yet Registered" => Some(NotYetRegistered),
                "Obliged to File Balance Sheet" => Some(ObligedToFileBalanceSheet),
                "Official Confirmation Received" => Some(OfficialConfirmationReceived),
                "Old But Well Kept" => Some(OldButWellKept),
                "Old Established Business" => Some(OldEstablishedBusiness),
                "Operated at Break Even" => Some(OperatedAtBreakEven),
                "Operates as Agent" => Some(OperatesAsAgent),
                "Flood Zone Status" => Some(FloodZoneStatus),
                "Out of Business" => Some(OutOfBusiness),
                "Outstanding Claims" => Some(OutstandingClaims),
                "Gas On" => Some(GasOn),
                "Hazardous Materials are Used or Produced" => {
                    Some(HazardousMaterialsAreUsedOrProduced)
                }
                "Genetically Engineered Organisms are Used or Produced" => {
                    Some(GeneticallyEngineeredOrganismsAreUsedOrProduced)
                }
                "This is a Group Proposal" => Some(ThisIsAGroupProposal),
                "Historical Sites Are Affected" => Some(HistoricalSitesAreAffected),
                "Facilities are Properly Accredited or Authorized" => {
                    Some(FacilitiesAreProperlyAccreditedOrAuthorized)
                }
                "Proprietary or Privileged Information will be contained in the Application" => {
                    Some(
                        ProprietaryOrPrivilegedInformationWillBeContainedInTheApplication,
                    )
                }
                "This Project has an Actual or Potential Impact on the Environment" => {
                    Some(ThisProjectHasAnActualOrPotentialImpactOnTheEnvironment)
                }
                "Growth Rate is Fully Developed" => Some(GrowthRateIsFullyDeveloped),
                "Outstanding Social Security Claims" => {
                    Some(OutstandingSocialSecurityClaims)
                }
                "Outstanding Value Added Tax (VAT) Claims" => Some(CodeGC),
                "Product Demonstration in Effect" => Some(ProductDemonstrationInEffect),
                "Ownership Acknowledged in Signed Statement" => {
                    Some(OwnershipAcknowledgedInSignedStatement)
                }
                "Ownership Acknowledged Verbally" => Some(OwnershipAcknowledgedVerbally),
                "Ownership Not Acknowledged" => Some(OwnershipNotAcknowledged),
                "Owns No Real Estate" => Some(OwnsNoRealEstate),
                "Owns Real Estate but Details Not Available" => {
                    Some(OwnsRealEstateButDetailsNotAvailable)
                }
                "Prepared from Books Without Audit" => {
                    Some(PreparedFromBooksWithoutAudit)
                }
                "Prepared from Statement by Accountant" => {
                    Some(PreparedFromStatementByAccountant)
                }
                "Profits Paid to Group" => Some(ProfitsPaidToGroup),
                "Shelf Set to Manufacturer's Standard" => {
                    Some(ShelfSetToManufacturersStandard)
                }
                "Publicly Traded" => Some(PubliclyTraded),
                "Good" => Some(Good),
                "Purchase Authority is Qualified" => Some(PurchaseAuthorityIsQualified),
                "Purchases on Floor Plan" => Some(PurchasesOnFloorPlan),
                "Shelf Set to Retailer's Schematic" => Some(ShelfSetToRetailersSchematic),
                "Purchases on Letter of Credit" => Some(PurchasesOnLetterOfCredit),
                "Real Estate Check is Necessary" => Some(RealEstateCheckIsNecessary),
                "Record of Preferential Claims" => Some(RecordOfPreferentialClaims),
                "Registered Address is Same as Business Address" => {
                    Some(RegisteredAddressIsSameAsBusinessAddress)
                }
                "Relatives Help in Business" => Some(RelativesHelpInBusiness),
                "Satisfactory" => Some(Satisfactory),
                "Seasons are Steady" => Some(SeasonsAreSteady),
                "Secured" => Some(Secured),
                "Organization Certifies Compliance with Federal Lobbying Regulations" => {
                    Some(OrganizationCertifiesComplianceWithFederalLobbyingRegulations)
                }
                "Project involves International Co-operative Activities" => {
                    Some(ProjectInvolvesInternationalCoOperativeActivities)
                }
                "Human Anatomical Substances Are Used" => {
                    Some(HumanAnatomicalSubstancesAreUsed)
                }
                "Handicap Facilities Are Available" => {
                    Some(HandicapFacilitiesAreAvailable)
                }
                "Lobbying Activities Have Been Conducted Regarding the Proposal" => {
                    Some(LobbyingActivitiesHaveBeenConductedRegardingTheProposal)
                }
                "Organization Certifies Compliance With the Drug-Free Workplace Act" => {
                    Some(OrganizationCertifiesComplianceWithTheDrugFreeWorkplaceAct)
                }
                "Organization Certifies Compliance with the Code of Federal Regulations Regarding Research Misconduct" => {
                    Some(
                        OrganizationCertifiesComplianceWithTheCodeOfFederalRegulationsRegardingResearchMisconduct,
                    )
                }
                "Organization Provides a Smoke Free Workplace" => {
                    Some(OrganizationProvidesASmokeFreeWorkplace)
                }
                "Organization Certifies Compliance with Federal Discrimination Regulations" => {
                    Some(
                        OrganizationCertifiesComplianceWithFederalDiscriminationRegulations,
                    )
                }
                "Organization Certifies Compliance with the Code of Federal Regulations Regarding Responsibility of Applicants for Promoting Objectivity in Research for which Public Health Service (PHS) Funding is Sought" => {
                    Some(CodeH9)
                }
                "Well Maintained" => Some(WellMaintained),
                "Interest Rate Buydown" => Some(InterestRateBuydown),
                "Heating and Cooling for the Individual Units Separately Metered" => {
                    Some(HeatingAndCoolingForTheIndividualUnitsSeparatelyMetered)
                }
                "High Discharge" => Some(HighDischarge),
                "High Engine Water Pressure" => Some(HighEngineWaterPressure),
                "Interest Only" => Some(InterestOnly),
                "Graduated Payment" => Some(GraduatedPayment),
                "Principal Balance Exceeds Maximum Negative Amortization" => {
                    Some(PrincipalBalanceExceedsMaximumNegativeAmortization)
                }
                "Last Change" => Some(LastChange),
                "Liability Released" => Some(LiabilityReleased),
                "Liability Not Released" => Some(LiabilityNotReleased),
                "Hearing Limitations" => Some(HearingLimitations),
                "Liability Determined by Note Holder" => {
                    Some(LiabilityDeterminedByNoteHolder)
                }
                "After Conversion" => Some(AfterConversion),
                "Hostile" => Some(Hostile),
                "After Modification" => Some(AfterModification),
                "Balloon" => Some(Balloon),
                "Capitalized Mortgage" => Some(CapitalizedMortgage),
                "Federal Wages in Effect" => Some(FederalWagesInEffect),
                "Social Security Number (SSN) Never Issued" => Some(CodeHT),
                "Name Does Not Match Social Security Number (SSN)" => Some(CodeHU),
                "Birthdate Does Not Match Social Security Number (SSN)" => Some(CodeHV),
                "Impossible Social Security Number (SSN)" => Some(CodeHW),
                "Employee is Ineligible to Work" => Some(EmployeeIsIneligibleToWork),
                "Metes and Bounds" => Some(MetesAndBounds),
                "Consolidation, Extension, Modification of Mortgage Loan (CEM)" => {
                    Some(CodeHZ)
                }
                "Based on Operating Data" => Some(BasedOnOperatingData),
                "Uses Outside Services" => Some(UsesOutsideServices),
                "Very High Level" => Some(VeryHighLevel),
                "Very Small" => Some(VerySmall),
                "Voluntary Bankruptcy" => Some(VoluntaryBankruptcy),
                "Well Balanced" => Some(WellBalanced),
                "Well Regarded in Business Circles" => {
                    Some(WellRegardedInBusinessCircles)
                }
                "Organization has Delinquent Federal Debts" => {
                    Some(OrganizationHasDelinquentFederalDebts)
                }
                "Organization has been Placed on the Federal Debarment and Suspension List" => {
                    Some(OrganizationHasBeenPlacedOnTheFederalDebarmentAndSuspensionList)
                }
                "No-show Indicator" => Some(NoShowIndicator),
                "Interest Paid in Advance" => Some(InterestPaidInAdvance),
                "Interest Paid in Arrears" => Some(InterestPaidInArrears),
                "Interest Carryover" => Some(InterestCarryover),
                "Sells Directly" => Some(SellsDirectly),
                "Sells with Agents" => Some(SellsWithAgents),
                "Sells with Storage" => Some(SellsWithStorage),
                "Small" => Some(Small),
                "Independent at Home" => Some(IndependentAtHome),
                "Some Increase" => Some(SomeIncrease),
                "Somewhat Declining Tendency" => Some(SomewhatDecliningTendency),
                "Started Some Time Ago" => Some(StartedSomeTimeAgo),
                "Industry Location" => Some(IndustryLocation),
                "Sufficient" => Some(Sufficient),
                "Indifferent" => Some(Indifferent),
                "Termination Date Set" => Some(TerminationDateSet),
                "Injury occurred on Employer's Premises" => {
                    Some(InjuryOccurredOnEmployersPremises)
                }
                "Terms Include Lump Sum Payments" => Some(TermsIncludeLumpSumPayments),
                "Terms Include Progress Payments" => Some(TermsIncludeProgressPayments),
                "Terms on Cost Plus Basis" => Some(TermsOnCostPlusBasis),
                "Terms on Fixed Fee Basis" => Some(TermsOnFixedFeeBasis),
                "Trade Style Registered" => Some(TradeStyleRegistered),
                "Trading Address of Sole Proprietor" => {
                    Some(TradingAddressOfSoleProprietor)
                }
                "Unchanged Situation" => Some(UnchangedSituation),
                "Undetermined" => Some(Undetermined),
                "Unsatisfactory" => Some(Unsatisfactory),
                "Unsecured" => Some(Unsecured),
                "Qualifies as an Energy Efficient Home" => {
                    Some(QualifiesAsAnEnergyEfficientHome)
                }
                "Military Services Barred from Recruitment Activities at the Proposing Organization's Site(s)" => {
                    Some(CodeJ1)
                }
                "Rate Negotiated" => Some(RateNegotiated),
                "Under Penalty of Perjury the Information is True and Correct" => {
                    Some(UnderPenaltyOfPerjuryTheInformationIsTrueAndCorrect)
                }
                "Project Requires Inter-Government Review for Activities that affect State or Local Government or Possible National Security Implications" => {
                    Some(
                        ProjectRequiresInterGovernmentReviewForActivitiesThatAffectStateOrLocalGovernmentOrPossibleNationalSecurityImplications,
                    )
                }
                "Filing on Behalf of Debtor is Authorized" => {
                    Some(FilingOnBehalfOfDebtorIsAuthorized)
                }
                "Debtor Understands the Relief available under each Bankruptcy Chapter" => {
                    Some(DebtorUnderstandsTheReliefAvailableUnderEachBankruptcyChapter)
                }
                "Attorney Declares that Debtor has been Informed" => {
                    Some(AttorneyDeclaresThatDebtorHasBeenInformed)
                }
                "Attorney has Explained the Relief available under each Bankruptcy Chapter" => {
                    Some(
                        AttorneyHasExplainedTheReliefAvailableUnderEachBankruptcyChapter,
                    )
                }
                "There has been a Transfer of a Claim Against the Debtor by or to any Petitioner" => {
                    Some(
                        ThereHasBeenATransferOfAClaimAgainstTheDebtorByOrToAnyPetitioner,
                    )
                }
                "Third Party Originated" => Some(ThirdPartyOriginated),
                "Existing Construction" => Some(ExistingConstruction),
                "Other Lien" => Some(OtherLien),
                "Joint Coverage Applies" => Some(JointCoverageApplies),
                "Subject Lien" => Some(SubjectLien),
                "No Evidence of Property Damage Observed such as Dampness, Termites, or Structure Settlement" => {
                    Some(CodeJE)
                }
                "Primary Underwriting System" => Some(PrimaryUnderwritingSystem),
                "Non New Parts Used" => Some(NonNewPartsUsed),
                "Pledged Loan" => Some(PledgedLoan),
                "Security Delivery" => Some(SecurityDelivery),
                "Secondary Underwriting System" => Some(SecondaryUnderwritingSystem),
                "Distribution is Stopped" => Some(DistributionIsStopped),
                "Sentence was Suspended" => Some(SentenceWasSuspended),
                "Very Negative Information Exists" => Some(VeryNegativeInformationExists),
                "Payment Notes Exist" => Some(PaymentNotesExist),
                "Immigrated" => Some(Immigrated),
                "Audited with Qualifications" => Some(AuditedWithQualifications),
                "Audited" => Some(Audited),
                "Temporarily Closed" => Some(TemporarilyClosed),
                "Partial" => Some(Partial),
                "Telephone Number is Unpublished" => Some(TelephoneNumberIsUnpublished),
                "Telephone Number is Not in Service" => {
                    Some(TelephoneNumberIsNotInService)
                }
                "Negative Information Exists for the Group" => {
                    Some(NegativeInformationExistsForTheGroup)
                }
                "The More Important Items are Only Included" => {
                    Some(TheMoreImportantItemsAreOnlyIncluded)
                }
                "Interest Owned by Affiliated Company" => {
                    Some(InterestOwnedByAffiliatedCompany)
                }
                "Interest Owned by Subject of Inquiry" => {
                    Some(InterestOwnedBySubjectOfInquiry)
                }
                "Qualifies as a Government Approved Condominium or Project" => {
                    Some(QualifiesAsAGovernmentApprovedCondominiumOrProject)
                }
                "Account Receivables Breakdown Undisclosed" => {
                    Some(AccountReceivablesBreakdownUndisclosed)
                }
                "Additional Record Items Available" => {
                    Some(AdditionalRecordItemsAvailable)
                }
                "Address is Qualified" => Some(AddressIsQualified),
                "All Paid In or Issued" => Some(AllPaidInOrIssued),
                "Appears High" => Some(AppearsHigh),
                "Appears Not to Guarantee Sufficient Coverage" => {
                    Some(AppearsNotToGuaranteeSufficientCoverage)
                }
                "Appears Sufficiently High" => Some(AppearsSufficientlyHigh),
                "Appears to Indicate a Strained Situation" => {
                    Some(AppearsToIndicateAStrainedSituation)
                }
                "Banks with Main National Banks" => Some(BanksWithMainNationalBanks),
                "Bills Paid from Branch Office" => Some(BillsPaidFromBranchOffice),
                "Bills Paid from Division Office" => Some(BillsPaidFromDivisionOffice),
                "Bills Paid from Headquarters Office" => {
                    Some(BillsPaidFromHeadquartersOffice)
                }
                "Bond Information Available" => Some(BondInformationAvailable),
                "Changed Accounting Date" => Some(ChangedAccountingDate),
                "Clear" => Some(Clear),
                "Clear Declining Tendency" => Some(ClearDecliningTendency),
                "Clear Increase" => Some(ClearIncrease),
                "Cluttered" => Some(Cluttered),
                "Company has No Other Locations" => Some(CompanyHasNoOtherLocations),
                "Company is Branch of Foreign Entity" => {
                    Some(CompanyIsBranchOfForeignEntity)
                }
                "Company is Perpetual" => Some(CompanyIsPerpetual),
                "Company is Tax Exempt" => Some(CompanyIsTaxExempt),
                "Compared to Same Period Last Year" => Some(ComparedToSamePeriodLastYear),
                "Conducted at a Loss" => Some(ConductedAtALoss),
                "Inventory Valued using FIFO (First In/First Out)" => Some(CodeKO),
                "Large" => Some(Large),
                "Letter of Agreement Present" => Some(LetterOfAgreementPresent),
                "Letter of Agreement Withdrawn" => Some(LetterOfAgreementWithdrawn),
                "Letter of Liability Present" => Some(LetterOfLiabilityPresent),
                "Letter of Liability Withdrawn" => Some(LetterOfLiabilityWithdrawn),
                "Location Inquired Upon is a Branch" => {
                    Some(LocationInquiredUponIsABranch)
                }
                "Location Inquired Upon is a Branch; Headquarters is Provided" => {
                    Some(CodeKV)
                }
                "Location inquired upon is a Headquarters" => {
                    Some(LocationInquiredUponIsAHeadquarters)
                }
                "Location is Foreign" => Some(LocationIsForeign),
                "Means Exhausted" => Some(MeansExhausted),
                "Medium to Large" => Some(MediumToLarge),
                "Immunization Mandated by State Law for Employment" => {
                    Some(ImmunizationMandatedByStateLawForEmployment)
                }
                "General Standard of 20 Degree or .5 Diopter Sphere or Cylinder Change Met" => {
                    Some(GeneralStandardOf20DegreeOr5DiopterSphereOrCylinderChangeMet)
                }
                "Replacement Due to Loss or Theft" => Some(ReplacementDueToLossOrTheft),
                "Replacement Due to Breakage or Damage" => {
                    Some(ReplacementDueToBreakageOrDamage)
                }
                "Replacement Due to Patient Preference" => {
                    Some(ReplacementDueToPatientPreference)
                }
                "Replacement Due to Medical Reason" => {
                    Some(ReplacementDueToMedicalReason)
                }
                "Land Contract" => Some(LandContract),
                "Account Current" => Some(AccountCurrent),
                "Very Good" => Some(VeryGood),
                "Restored" => Some(Restored),
                "Letter of Map Amendment or Letter of Map Revision" => {
                    Some(LetterOfMapAmendmentOrLetterOfMapRevision)
                }
                "Legally Blind" => Some(LegallyBlind),
                "Producer of Goods" => Some(ProducerOfGoods),
                "Drawback Indicator" => Some(DrawbackIndicator),
                "Lethargic" => Some(Lethargic),
                "Customs Rule Applicable" => Some(CustomsRuleApplicable),
                "Exported Pursuant to Law Regulation or to Cancel Customs Bond" => {
                    Some(ExportedPursuantToLawRegulationOrToCancelCustomsBond)
                }
                "Country of Origin Information Applies to All Prior Shipments" => {
                    Some(CountryOfOriginInformationAppliesToAllPriorShipments)
                }
                "Price Estimated" => Some(PriceEstimated),
                "North American Free Trade Agreement (NAFTA) Preference" => Some(CodeLJ),
                "Kit Form" => Some(KitForm),
                "Lockout Effective" => Some(LockoutEffective),
                "Letter of Appointment" => Some(LetterOfAppointment),
                "Facility's Emergency Response Plan Includes Specific Actions to be Taken in Response to Accidental Releases of Regulated Substances" => {
                    Some(
                        FacilitysEmergencyResponsePlanIncludesSpecificActionsToBeTakenInResponseToAccidentalReleasesOfRegulatedSubstances,
                    )
                }
                "Locomotive is Isolated" => Some(LocomotiveIsIsolated),
                "Low Engine Oil Pressure" => Some(LowEngineOilPressure),
                "Facility had a Safety Inspection" => Some(FacilityHadASafetyInspection),
                "Locomotive Engine is Running" => Some(LocomotiveEngineIsRunning),
                "Lessee Signature on File" => Some(LesseeSignatureOnFile),
                "List Specialty in Directory" => Some(ListSpecialtyInDirectory),
                "Lender or Servicer Transfer" => Some(LenderOrServicerTransfer),
                "Evidence of Dampness" => Some(EvidenceOfDampness),
                "Evidence of Termites" => Some(EvidenceOfTermites),
                "Evidence of Structure Settlement" => Some(EvidenceOfStructureSettlement),
                "Salvage Moved" => Some(SalvageMoved),
                "Address is Former Location" => Some(AddressIsFormerLocation),
                "Address is Occupied by Others" => Some(AddressIsOccupiedByOthers),
                "Facility has an Occupational Safety and Health Act\n(OSHA) Star or Merit Ranking" => {
                    Some(CodeM0)
                }
                "Data Corrected" => Some(DataCorrected),
                "Servicer Record Selected" => Some(ServicerRecordSelected),
                "Length of Service is 3 Months or Less" => {
                    Some(LengthOfServiceIs3MonthsOrLess)
                }
                "Length of Service is 3 Months or more, and Less than 1 Year" => {
                    Some(CodeM4)
                }
                "Length of Service is 1 Year through 5 Years" => {
                    Some(LengthOfServiceIs1YearThrough5Years)
                }
                "Length of Service is more than 5 Years" => {
                    Some(LengthOfServiceIsMoreThan5Years)
                }
                "Cataract or Corneal Transplant or Other Condition such as Keratoconus" => {
                    Some(CataractOrCornealTransplantOrOtherConditionSuchAsKeratoconus)
                }
                "Vision in Worse Eye Correctable to 20/40 or Better with Regular Lenses" => {
                    Some(VisionInWorseEyeCorrectableTo2040OrBetterWithRegularLenses)
                }
                "Contact Lenses Corrected Vision in Worse Eye to 20/40 or Better" => {
                    Some(ContactLensesCorrectedVisionInWorseEyeTo2040OrBetter)
                }
                "Major Alarm Flag Reported" => Some(MajorAlarmFlagReported),
                "Equipment has Modified Configuration" => {
                    Some(EquipmentHasModifiedConfiguration)
                }
                "Other Mental Condition" => Some(OtherMentalCondition),
                "Marketing Time is 4 to 6 Months" => Some(MarketingTimeIs4To6Months),
                "Trend Reversed" => Some(TrendReversed),
                "Microprocessor Fault" => Some(MicroprocessorFault),
                "Mortgage Insurance Application Included" => {
                    Some(MortgageInsuranceApplicationIncluded)
                }
                "Mortgage Credit Report Included" => Some(MortgageCreditReportIncluded),
                "Residential Loan Application Included" => {
                    Some(ResidentialLoanApplicationIncluded)
                }
                "Real Estate Information Report Included" => {
                    Some(RealEstateInformationReportIncluded)
                }
                "Real Estate Title Evidence Included" => {
                    Some(RealEstateTitleEvidenceIncluded)
                }
                "Manually Search and List" => Some(ManuallySearchAndList),
                "Property is Occupied by Tenant (Market Rent)" => Some(CodeMM),
                "Property is Occupied by Tenant (Regulated Rent)" => Some(CodeMN),
                "Cooperative Project Includes or Owns Any Commercial Units" => {
                    Some(CooperativeProjectIncludesOrOwnsAnyCommercialUnits)
                }
                "Units and Project Amenities are Complete" => {
                    Some(UnitsAndProjectAmenitiesAreComplete)
                }
                "Eligible Trust" => Some(EligibleTrust),
                "Resale Property" => Some(ResaleProperty),
                "Miscellaneous Skip-Trace Attempt" => Some(MiscellaneousSkipTraceAttempt),
                "Photos Match Description" => Some(PhotosMatchDescription),
                "Photos Show Negative Influence" => Some(PhotosShowNegativeInfluence),
                "Exclude from Monthly Debt" => Some(ExcludeFromMonthlyDebt),
                "This Broker Market Analysis is being Completed for Home Market Assistance" => {
                    Some(ThisBrokerMarketAnalysisIsBeingCompletedForHomeMarketAssistance)
                }
                "This Broker Market Analysis is being Completed for Homesale or Buyout" => {
                    Some(ThisBrokerMarketAnalysisIsBeingCompletedForHomesaleOrBuyout)
                }
                "Project Type is Single Family" => Some(ProjectTypeIsSingleFamily),
                "Project Type is Other" => Some(ProjectTypeIsOther),
                "Hospitalized over-night" => Some(HospitalizedOverNight),
                "Claim Involves (a) Day(s) Away From Work" => Some(CodeN1),
                "Claim involves Restricted Work Activity Without Days Away from Work" => {
                    Some(ClaimInvolvesRestrictedWorkActivityWithoutDaysAwayFromWork)
                }
                "Strike or Lockout in Progress" => Some(StrikeOrLockoutInProgress),
                "Shutdown or Layoff in Progress" => Some(ShutdownOrLayoffInProgress),
                "Work is Seasonal" => Some(WorkIsSeasonal),
                "Natural Disaster or Adverse Weather Affecting Work" => {
                    Some(NaturalDisasterOrAdverseWeatherAffectingWork)
                }
                "Shorter Work Schedules or Fewer Pay Periods than Usual in Effect" => {
                    Some(ShorterWorkSchedulesOrFewerPayPeriodsThanUsualInEffect)
                }
                "Longer Work Schedules or More Pay Periods than Usual in Effect" => {
                    Some(LongerWorkSchedulesOrMorePayPeriodsThanUsualInEffect)
                }
                "Other Factors Affect Claim Frequency" => {
                    Some(OtherFactorsAffectClaimFrequency)
                }
                "No User Available" => Some(NoUserAvailable),
                "Neighborhood Predominately Single Family Dwellings" => {
                    Some(NeighborhoodPredominatelySingleFamilyDwellings)
                }
                "Item has Direct Numerical Control" => {
                    Some(ItemHasDirectNumericalControl)
                }
                "Note Holder Permission Required" => Some(NoteHolderPermissionRequired),
                "No Deductible Program" => Some(NoDeductibleProgram),
                "Notarized" => Some(Notarized),
                "New Construction" => Some(NewConstruction),
                "Mortgage Points are Customarily Paid by Seller" => {
                    Some(MortgagePointsAreCustomarilyPaidBySeller)
                }
                "No National Flood Insurance Program map" => {
                    Some(NoNationalFloodInsuranceProgramMap)
                }
                "Seasoned Mortgage" => Some(SeasonedMortgage),
                "Issues are Anticipated that would Affect the Ability to Secure Financing of the Subject Property" => {
                    Some(
                        IssuesAreAnticipatedThatWouldAffectTheAbilityToSecureFinancingOfTheSubjectProperty,
                    )
                }
                "Citizenship" => Some(Citizenship),
                "Group Disability Insurance Mandatory" => {
                    Some(GroupDisabilityInsuranceMandatory)
                }
                "Retail Origination" => Some(RetailOrigination),
                "Answer to Referenced Question is \"None\"" => Some(CodeNN),
                "Arm's Length Transaction" => Some(ArmsLengthTransaction),
                "Certification of a Non-attorney Bankruptcy Petition Preparer" => {
                    Some(CertificationOfANonAttorneyBankruptcyPetitionPreparer)
                }
                "Eligible for the Fannie Mae Neighbors Program" => {
                    Some(EligibleForTheFannieMaeNeighborsProgram)
                }
                "No Restrictions" => Some(NoRestrictions),
                "401K Plan in Effect" => Some(CodeNS),
                "Lodging Provided" => Some(LodgingProvided),
                "Not Used" => Some(NotUsed),
                "Contract Labor" => Some(ContractLabor),
                "Bonuses Paid" => Some(BonusesPaid),
                "Minors Employed" => Some(MinorsEmployed),
                "Meets Requirements for Fannie Mae Community Seconds Program" => {
                    Some(MeetsRequirementsForFannieMaeCommunitySecondsProgram)
                }
                "Purchase is a Result of Current Employer Sponsored Relocation" => {
                    Some(PurchaseIsAResultOfCurrentEmployerSponsoredRelocation)
                }
                "Teaching Major" => Some(TeachingMajor),
                "Multiple Unspecified Instances" => Some(MultipleUnspecifiedInstances),
                "Hires Part Time Employees as Needed" => {
                    Some(HiresPartTimeEmployeesAsNeeded)
                }
                "Mexican Request" => Some(MexicanRequest),
                "Risk Management Plan Requires Predictive Filing" => {
                    Some(RiskManagementPlanRequiresPredictiveFiling)
                }
                "Sanitized Copy" => Some(SanitizedCopy),
                "Site Treated, Disposed, Recycled Waste On-Site or Discharged Waste to Sewer or Publicly Owned Treatment Works" => {
                    Some(CodeO6)
                }
                "Toxic Chemical Claimed as Trade Secret" => {
                    Some(ToxicChemicalClaimedAsTradeSecret)
                }
                "Under Control of Reporting Facility or Parent Company" => {
                    Some(UnderControlOfReportingFacilityOrParentCompany)
                }
                "Weather Conditions Not Known" => Some(WeatherConditionsNotKnown),
                "Seller Provided Below Market Secondary Financing" => {
                    Some(SellerProvidedBelowMarketSecondaryFinancing)
                }
                "Fixed Site" => Some(FixedSite),
                "Mobile Facility" => Some(MobileFacility),
                "Transfer Authorized" => Some(TransferAuthorized),
                "Occupational Disease" => Some(OccupationalDisease),
                "Transfer Complete" => Some(TransferComplete),
                "Commercial Driver's License Verified" => {
                    Some(CommercialDriversLicenseVerified)
                }
                "Responsibility Accepted" => Some(ResponsibilityAccepted),
                "Waterbody Involved" => Some(WaterbodyInvolved),
                "Charges Pending" => Some(ChargesPending),
                "Driver has Proper License Class" => Some(DriverHasProperLicenseClass),
                "Driver Compliant with License Restrictions" => {
                    Some(DriverCompliantWithLicenseRestrictions)
                }
                "Other Limitation" => Some(OtherLimitation),
                "Driver has Commercial Driver's License" => {
                    Some(DriverHasCommercialDriversLicense)
                }
                "Driver has Medical Waiver" => Some(DriverHasMedicalWaiver),
                "Own other Federal Housing Administration Property" => {
                    Some(OwnOtherFederalHousingAdministrationProperty)
                }
                "Out of Range Product Temperature" => Some(OutOfRangeProductTemperature),
                "Photographs Taken" => Some(PhotographsTaken),
                "Other Restrictions" => Some(OtherRestrictions),
                "Out of Service" => Some(OutOfService),
                "Oriented" => Some(Oriented),
                "Police Officer at Scene" => Some(PoliceOfficerAtScene),
                "Overridden" => Some(Overridden),
                "Proposed" => Some(Proposed),
                "Rating is Affected" => Some(RatingIsAffected),
                "Veteran as Defined by the Federal Housing Administration (FHA), Veterans Administration (VA), or Department of Housing and Urban Development (HUD)" => {
                    Some(CodeOY)
                }
                "Liability is Contingent or has a Co-signer" => {
                    Some(LiabilityIsContingentOrHasACoSigner)
                }
                "Terminal Degree" => Some(TerminalDegree),
                "Patient was Discharged from the First Facility" => {
                    Some(PatientWasDischargedFromTheFirstFacility)
                }
                "Patient was Admitted to the Second Facility" => {
                    Some(PatientWasAdmittedToTheSecondFacility)
                }
                "Property has a Family Room or Den" => Some(PropertyHasAFamilyRoomOrDen),
                "Property has Central Air Conditioning" => {
                    Some(PropertyHasCentralAirConditioning)
                }
                "Property Typical of Neighborhood" => Some(PropertyTypicalOfNeighborhood),
                "Property Deferred Maintenance Typical of Neighborhood" => {
                    Some(PropertyDeferredMaintenanceTypicalOfNeighborhood)
                }
                "Accepting Existing Patients" => Some(AcceptingExistingPatients),
                "Accepting New Patients" => Some(AcceptingNewPatients),
                "Property Intended to be Occupied as Primary Residence" => {
                    Some(PropertyIntendedToBeOccupiedAsPrimaryResidence)
                }
                "Paralysis" => Some(Paralysis),
                "Phone Skip Begin" => Some(PhoneSkipBegin),
                "Plan is Attached" => Some(PlanIsAttached),
                "Phone Skip Resolved" => Some(PhoneSkipResolved),
                "Phone Skip Exhaust" => Some(PhoneSkipExhaust),
                "Paid Outside of Closing" => Some(PaidOutsideOfClosing),
                "Previously Failed Board Certification" => {
                    Some(PreviouslyFailedBoardCertification)
                }
                "Project is Subject to Ground Rent" => Some(ProjectIsSubjectToGroundRent),
                "Prepayable" => Some(Prepayable),
                "Program" => Some(Program),
                "Provider is Participating" => Some(ProviderIsParticipating),
                "Preliminary Flood Determination" => Some(PreliminaryFloodDetermination),
                "Provider Certification in the Taxonomy Has Been Verified" => {
                    Some(ProviderCertificationInTheTaxonomyHasBeenVerified)
                }
                "Project and Services Budget is Maintained" => {
                    Some(ProjectAndServicesBudgetIsMaintained)
                }
                "Atypical Physical Condition" => Some(AtypicalPhysicalCondition),
                "Personal Property Onsite" => Some(PersonalPropertyOnsite),
                "Property Previously Winterized" => Some(PropertyPreviouslyWinterized),
                "Liability will be Resubordinated to the Loan upon Closing" => {
                    Some(LiabilityWillBeResubordinatedToTheLoanUponClosing)
                }
                "Poor" => Some(Poor),
                "Prior Damage" => Some(PriorDamage),
                "Publication is Included in Sharing" => {
                    Some(PublicationIsIncludedInSharing)
                }
                "Project is Complete" => Some(ProjectIsComplete),
                "Not Paid" => Some(NotPaid),
                "Property Vacant 0-5 Percent" => Some(PropertyVacant05Percent),
                "Partial Weight Bearing" => Some(PartialWeightBearing),
                "Paid by Borrower Before Closing" => Some(PaidByBorrowerBeforeClosing),
                "Property for Sale" => Some(PropertyForSale),
                "Property Vacant Over 5 Percent" => Some(PropertyVacantOver5Percent),
                "Veteran" => Some(Veteran),
                "Export Product" => Some(ExportProduct),
                "Distilled Spirit, Beer or Wine" => Some(CodeQ2),
                "U.S. Goods Returned" => Some(USGoodsReturned),
                "Candidate for U.S. Customs Service Protest" => {
                    Some(CandidateForUSCustomsServiceProtest)
                }
                "Domestic Product" => Some(DomesticProduct),
                "Prior Approval Letter and Official Orders on File" => {
                    Some(PriorApprovalLetterAndOfficialOrdersOnFile)
                }
                "Importer's Substantiating Statement and Contract are on File" => {
                    Some(ImportersSubstantiatingStatementAndContractAreOnFile)
                }
                "International Transport Movement" => {
                    Some(InternationalTransportMovement)
                }
                "Piece Count should be Included in the Total Packing List Quantity" => {
                    Some(PieceCountShouldBeIncludedInTheTotalPackingListQuantity)
                }
                "Shipment should be Held at the Port" => {
                    Some(ShipmentShouldBeHeldAtThePort)
                }
                "Multiple States of Origin for this Item" => {
                    Some(MultipleStatesOfOriginForThisItem)
                }
                "Multiple Countries of Origin for this Item" => {
                    Some(MultipleCountriesOfOriginForThisItem)
                }
                "Letter of Credit Restricted to a Specific Bank" => {
                    Some(LetterOfCreditRestrictedToASpecificBank)
                }
                "Letter of Credit Permits Transshipment" => {
                    Some(LetterOfCreditPermitsTransshipment)
                }
                "Letter of Credit Covers Partial Shipments" => {
                    Some(LetterOfCreditCoversPartialShipments)
                }
                "Dutiable Item" => Some(DutiableItem),
                "Amounts should be Pro-rated across Line Items" => {
                    Some(AmountsShouldBeProRatedAcrossLineItems)
                }
                "Toxic Substance Control Act (TSCA) Certification Required" => {
                    Some(CodeQI)
                }
                "Visa Required for this Item" => Some(VisaRequiredForThisItem),
                "Item Subject to Quotas" => Some(ItemSubjectToQuotas),
                "Item is a Set as Defined by the General Rules of Interpretation Section 3 (GRI3)" => {
                    Some(CodeQL)
                }
                "Item is a Set" => Some(ItemIsASet),
                "Item is an Ensemble" => Some(ItemIsAnEnsemble),
                "Item is a Metal Item" => Some(ItemIsAMetalItem),
                "Item is a Machine Part" => Some(ItemIsAMachinePart),
                "Item is a Hazardous Item" => Some(ItemIsAHazardousItem),
                "Item is Eligible under the Generalized System of Preferences (GSP)" => {
                    Some(CodeQR)
                }
                "Quantity to be Imported has been Approved by the Necessary Agencies" => {
                    Some(QuantityToBeImportedHasBeenApprovedByTheNecessaryAgencies)
                }
                "Filing Data is to be Withheld from Public Inspection" => {
                    Some(FilingDataIsToBeWithheldFromPublicInspection)
                }
                "Property Type Cooperative" => Some(PropertyTypeCooperative),
                "Paid by Borrower at Closing" => Some(PaidByBorrowerAtClosing),
                "Paid by Other At or Before Closing" => {
                    Some(PaidByOtherAtOrBeforeClosing)
                }
                "Treated as a Reduction to Income" => Some(TreatedAsAReductionToIncome),
                "Does Organization Receive Income from the Sale or Lease of Tangible Personal Property, the Lease of Real Property, or the Sale of Taxable Services?" => {
                    Some(CodeQY)
                }
                "Is organization a contractor-retailer primarily engaged in retail sales?" => {
                    Some(CodeQZ)
                }
                "Exempt from Public Records Law" => Some(ExemptFromPublicRecordsLaw),
                "Debtor Holds Claim to Real Property" => {
                    Some(DebtorHoldsClaimToRealProperty)
                }
                "Entity Claims to Hold a Secured Interest" => {
                    Some(EntityClaimsToHoldASecuredInterest)
                }
                "Debtor has Property of the Type Specified" => {
                    Some(DebtorHasPropertyOfTheTypeSpecified)
                }
                "Debtor Elects the State Exemption" => {
                    Some(DebtorElectsTheStateExemption)
                }
                "Debtor Elects the Federal Exemption" => {
                    Some(DebtorElectsTheFederalExemption)
                }
                "Co-debtor may be Jointly Liable" => Some(CoDebtorMayBeJointlyLiable),
                "Claim is Contingent" => Some(ClaimIsContingent),
                "Claim is Unliquidated" => Some(ClaimIsUnliquidated),
                "Claim is Disputed" => Some(ClaimIsDisputed),
                "Reference Telephone Attempt" => Some(ReferenceTelephoneAttempt),
                "Debtor has No Creditors Holding Unsecured Priority Claims" => {
                    Some(DebtorHasNoCreditorsHoldingUnsecuredPriorityClaims)
                }
                "Reference Telephone Contact" => Some(ReferenceTelephoneContact),
                "Rental Car Arranged" => Some(RentalCarArranged),
                "Rent Delinquent" => Some(RentDelinquent),
                "Claim is Subject to Setoff" => Some(ClaimIsSubjectToSetoff),
                "Debtor has No Executory Contracts or Unexpired Leases" => {
                    Some(DebtorHasNoExecutoryContractsOrUnexpiredLeases)
                }
                "Lease is for Nonresidential Real Property" => {
                    Some(LeaseIsForNonresidentialRealProperty)
                }
                "Debtor has No Co-debtors" => Some(DebtorHasNoCoDebtors),
                "Debtor is Married" => Some(DebtorIsMarried),
                "Debtor's Spouse Maintains a Separate Household" => {
                    Some(DebtorsSpouseMaintainsASeparateHousehold)
                }
                "Real Estate Taxes are Included" => Some(RealEstateTaxesAreIncluded),
                "Property Insurance is Included" => Some(PropertyInsuranceIsIncluded),
                "Debtor has No Creditors Holding Secured Claims" => {
                    Some(DebtorHasNoCreditorsHoldingSecuredClaims)
                }
                "Rent Control" => Some(RentControl),
                "Equipment is Rebuilt" => Some(EquipmentIsRebuilt),
                "Individual Injured in Performance of Duty" => {
                    Some(IndividualInjuredInPerformanceOfDuty)
                }
                "Individual Injured by Third Party" => {
                    Some(IndividualInjuredByThirdParty)
                }
                "Quality of Management and its Enforcement of Rules and Regulations Based on General Appearances" => {
                    Some(
                        QualityOfManagementAndItsEnforcementOfRulesAndRegulationsBasedOnGeneralAppearances,
                    )
                }
                "Pay Continued" => Some(PayContinued),
                "Sick Leave Taken" => Some(SickLeaveTaken),
                "Signature on File" => Some(SignatureOnFile),
                "Low Refrigerant Capacity Shutdown" => {
                    Some(LowRefrigerantCapacityShutdown)
                }
                "Recent Defrost" => Some(RecentDefrost),
                "Rated Horsepower can be Produced" => Some(RatedHorsepowerCanBeProduced),
                "Foreign Military Sale" => Some(ForeignMilitarySale),
                "Waiver of Prior Notice" => Some(WaiverOfPriorNotice),
                "Alternate Certification Program Participant" => {
                    Some(AlternateCertificationProgramParticipant)
                }
                "Services Provided at the Second Facility were available at the First Facility" => {
                    Some(
                        ServicesProvidedAtTheSecondFacilityWereAvailableAtTheFirstFacility,
                    )
                }
                "Under Treatment" => Some(UnderTreatment),
                "First Time Vacant" => Some(FirstTimeVacant),
                "Adverse Easement" => Some(AdverseEasement),
                "Disclosure Indicator" => Some(DisclosureIndicator),
                "Atypical Off Site Improvements" => Some(AtypicalOffSiteImprovements),
                "Toxic Substances" => Some(ToxicSubstances),
                "Adverse Encroachment" => Some(AdverseEncroachment),
                "Atypical Functional Condition" => Some(AtypicalFunctionalCondition),
                "Subject Property is Currently Listed" => {
                    Some(SubjectPropertyIsCurrentlyListed)
                }
                "Debtor is a Small Business as Defined in 11 U.S.C. Section 101" => {
                    Some(DebtorIsASmallBusinessAsDefinedIn11USCSection101)
                }
                "Special Services are Mobile Home Only" => {
                    Some(SpecialServicesAreMobileHomeOnly)
                }
                "Special Services are Leasehold or Mobile Home or Both" => {
                    Some(SpecialServicesAreLeaseholdOrMobileHomeOrBoth)
                }
                "Debtor Elects to be Considered as a Small Business Under 11 U.S.C. Section 1121(e)" => {
                    Some(CodeSE)
                }
                "Sensor Fault" => Some(SensorFault),
                "Street Lights are Public" => Some(StreetLightsArePublic),
                "Special Services are Leasehold or Subleasehold or Both" => {
                    Some(SpecialServicesAreLeaseholdOrSubleaseholdOrBoth)
                }
                "Hazardous Waste" => Some(HazardousWaste),
                "Pest Infestation" => Some(PestInfestation),
                "Road Maintenance Required" => Some(RoadMaintenanceRequired),
                "Speech Limitations" => Some(SpeechLimitations),
                "Currently Serving in Military" => Some(CurrentlyServingInMilitary),
                "Major Base Support" => Some(MajorBaseSupport),
                "Critical Support Level Met" => Some(CriticalSupportLevelMet),
                "Street is Public" => Some(StreetIsPublic),
                "Specialty is Primary" => Some(SpecialtyIsPrimary),
                "Specialty is Secondary" => Some(SpecialtyIsSecondary),
                "Local Wages in Effect" => Some(LocalWagesInEffect),
                "Federal Worker Displacement" => Some(FederalWorkerDisplacement),
                "Adverse Zoning" => Some(AdverseZoning),
                "New Services Requested" => Some(NewServicesRequested),
                "Continued Services Requested" => Some(ContinuedServicesRequested),
                "Subrogation Open" => Some(SubrogationOpen),
                "Major Corporation/High Tech" => Some(MajorCorporationHighTech),
                "Sidewalk is Public" => Some(SidewalkIsPublic),
                "Collective Bargaining Agreement Sent by Mail" => {
                    Some(CollectiveBargainingAgreementSentByMail)
                }
                "Collective Bargaining Agreement Sent by Facsimile" => {
                    Some(CollectiveBargainingAgreementSentByFacsimile)
                }
                "Contract" => Some(Contract),
                "Under Contract" => Some(UnderContract),
                "Road Test Performed with No Problems Reported" => {
                    Some(RoadTestPerformedWithNoProblemsReported)
                }
                "Road Test Performed with Problems Reported" => {
                    Some(RoadTestPerformedWithProblemsReported)
                }
                "Tires' Brand Match" => Some(TiresBrandMatch),
                "Real Estate Taxes are Current" => Some(RealEstateTaxesAreCurrent),
                "Hazard Insurance is Current" => Some(HazardInsuranceIsCurrent),
                "Terminate Guarantee" => Some(TerminateGuarantee),
                "Atypical External Condition" => Some(AtypicalExternalCondition),
                "Subsidence (Settlement of Ground Surface Caused by Loss of Support)" => {
                    Some(CodeT8)
                }
                "Utilities Inadequate" => Some(UtilitiesInadequate),
                "Collective Bargaining Agreement Sent by Electronic Bulletin Board" => {
                    Some(CollectiveBargainingAgreementSentByElectronicBulletinBoard)
                }
                "Debtor has No Creditors Holding Unsecured Nonpriority Claims" => {
                    Some(DebtorHasNoCreditorsHoldingUnsecuredNonpriorityClaims)
                }
                "Transport via Cargo Aircraft" => Some(TransportViaCargoAircraft),
                "Annual Leave Taken" => Some(AnnualLeaveTaken),
                "Item is Special Test Equipment" => Some(ItemIsSpecialTestEquipment),
                "Operates as Representative For Others" => {
                    Some(OperatesAsRepresentativeForOthers)
                }
                "Claim Involves Work Related Death" => {
                    Some(ClaimInvolvesWorkRelatedDeath)
                }
                "Claim Does Not Involve Work Related Death, Days Away from Work, or Restricted Work Activity" => {
                    Some(CodeTH)
                }
                "Employee Has Not Recovered to Return to Work" => {
                    Some(EmployeeHasNotRecoveredToReturnToWork)
                }
                "Employee Has Retired" => Some(EmployeeHasRetired),
                "Employee Has Resigned" => Some(EmployeeHasResigned),
                "Employee is Permanently and Totally Disabled" => {
                    Some(EmployeeIsPermanentlyAndTotallyDisabled)
                }
                "Traction Motor is Cut Out" => Some(TractionMotorIsCutOut),
                "Atypical Quality of Construction" => Some(AtypicalQualityOfConstruction),
                "Traumatic Injury" => Some(TraumaticInjury),
                "Atypical Remodeling" => Some(AtypicalRemodeling),
                "Transport via Passenger Aircraft" => Some(TransportViaPassengerAircraft),
                "Atypical Additions" => Some(AtypicalAdditions),
                "Transfer to Bed, or Chair, or Both" => Some(CodeTR),
                "Adverse Marketing Conditions in Subject Property's Neighborhood" => {
                    Some(AdverseMarketingConditionsInSubjectPropertysNeighborhood)
                }
                "Neighborhood Water Source is Public" => {
                    Some(NeighborhoodWaterSourceIsPublic)
                }
                "Neighborhood Sewage Treatment is Public" => {
                    Some(NeighborhoodSewageTreatmentIsPublic)
                }
                "Telephone Number Verified" => Some(TelephoneNumberVerified),
                "Neighborhood Street is Public" => Some(NeighborhoodStreetIsPublic),
                "Other Miscellaneous Adverse Characteristics" => {
                    Some(OtherMiscellaneousAdverseCharacteristics)
                }
                "Subject Property's Street is Public" => {
                    Some(SubjectPropertysStreetIsPublic)
                }
                "Subject Property's Sewage Treatment is Public" => {
                    Some(SubjectPropertysSewageTreatmentIsPublic)
                }
                "Disability" => Some(Disability),
                "Minimal Change" => Some(MinimalChange),
                "Neat Appearance" => Some(NeatAppearance),
                "Net Worth Computed after Exemptions" => {
                    Some(NetWorthComputedAfterExemptions)
                }
                "Net Worth Considerably Higher" => Some(NetWorthConsiderablyHigher),
                "Net Worth Higher" => Some(NetWorthHigher),
                "No Employees" => Some(NoEmployees),
                "No Employees - Business Managed by Owner" => {
                    Some(NoEmployeesBusinessManagedByOwner)
                }
                "No Employees - Business Managed by Partners" => {
                    Some(NoEmployeesBusinessManagedByPartners)
                }
                "Not Out of Business" => Some(NotOutOfBusiness),
                "Uninsurable, 1316 Property" => Some(CodeUA),
                "Conducted at a Profit" => Some(ConductedAtAProfit),
                "Contingent Debt Indicated" => Some(ContingentDebtIndicated),
                "Continue" => Some(Continue),
                "Contracts Obtained by Bid" => Some(ContractsObtainedByBid),
                "Contracts Obtained by Negotiation" => {
                    Some(ContractsObtainedByNegotiation)
                }
                "Converted to Holding Company" => Some(ConvertedToHoldingCompany),
                "Cross Claim Filed" => Some(CrossClaimFiled),
                "Declining Tendency" => Some(DecliningTendency),
                "Detrimental Events in Past, Relating to Business" => Some(CodeUJ),
                "Detrimental Events in Past, Relating to Management" => Some(CodeUK),
                "Down or Decline or Decreased" => Some(DownOrDeclineOrDecreased),
                "Employees Include Officers" => Some(EmployeesIncludeOfficers),
                "Uncooperative" => Some(Uncooperative),
                "Employees Include Owners" => Some(EmployeesIncludeOwners),
                "Employees Include Partners" => Some(EmployeesIncludePartners),
                "Employees Include Temporary Workers" => {
                    Some(EmployeesIncludeTemporaryWorkers)
                }
                "Employees Vary According to Needs" => {
                    Some(EmployeesVaryAccordingToNeeds)
                }
                "Enclosed" => Some(Enclosed),
                "Up as Tolerated" => Some(UpAsTolerated),
                "Extent of Audit, if any, Not Indicated" => Some(CodeUU),
                "Favorable Personal Reputation" => Some(FavorablePersonalReputation),
                "Figures are Abbreviated" => Some(FiguresAreAbbreviated),
                "Figures are Converted to Agency Format" => {
                    Some(FiguresAreConvertedToAgencyFormat)
                }
                "Figures are Individual" => Some(FiguresAreIndividual),
                "Figures are Restated" => Some(FiguresAreRestated),
                "Ultimate Parent Company Financial Statement Used" => {
                    Some(UltimateParentCompanyFinancialStatementUsed)
                }
                "Valid Borrower Address or Phone Attempt with School Attended" => {
                    Some(ValidBorrowerAddressOrPhoneAttemptWithSchoolAttended)
                }
                "Lender Determined Borrower Moved Out of State" => {
                    Some(LenderDeterminedBorrowerMovedOutOfState)
                }
                "Lender Determined Borrower Moved Back into State" => {
                    Some(LenderDeterminedBorrowerMovedBackIntoState)
                }
                "Lender Determined Borrower Incarcerated" => {
                    Some(LenderDeterminedBorrowerIncarcerated)
                }
                "Lender Determined Borrower No Longer Incarcerated" => {
                    Some(LenderDeterminedBorrowerNoLongerIncarcerated)
                }
                "Original" => Some(Original),
                "True and Exact Copy" => Some(TrueAndExactCopy),
                "Subject Property's Water Source is Public" => {
                    Some(SubjectPropertysWaterSourceIsPublic)
                }
                "Pictures Required" => Some(PicturesRequired),
                "Intercompany Relations Exist" => Some(IntercompanyRelationsExist),
                "Inventory Valued at Lower of Cost or Market" => {
                    Some(InventoryValuedAtLowerOfCostOrMarket)
                }
                "Inventory Valued at Other Methods" => {
                    Some(InventoryValuedAtOtherMethods)
                }
                "Operates as Sole Agent" => Some(OperatesAsSoleAgent),
                "Without Personal Judgment" => Some(WithoutPersonalJudgment),
                "Work is Subcontracted" => Some(WorkIsSubcontracted),
                "Not Registered" => Some(NotRegistered),
                "Immediate Attention Required" => Some(ImmediateAttentionRequired),
                "Vehicle Inspection Report Completed" => {
                    Some(VehicleInspectionReportCompleted)
                }
                "Middle to Medium" => Some(MiddleToMedium),
                "Rent Control Likely" => Some(RentControlLikely),
                "Furnished" => Some(Furnished),
                "Price Range Single Family or Planned Unit Development Not Applicable" => {
                    Some(PriceRangeSingleFamilyOrPlannedUnitDevelopmentNotApplicable)
                }
                "Price Range Condominium Not Applicable" => {
                    Some(PriceRangeCondominiumNotApplicable)
                }
                "Price Range Two to Four Family Not Applicable" => {
                    Some(PriceRangeTwoToFourFamilyNotApplicable)
                }
                "Financial Figures are Projected Based on Sales" => {
                    Some(FinancialFiguresAreProjectedBasedOnSales)
                }
                "Financial Figures are Projected Based on Employees" => {
                    Some(FinancialFiguresAreProjectedBasedOnEmployees)
                }
                "Parent Company has Bankruptcy" => Some(ParentCompanyHasBankruptcy),
                "Headquarters has Bankruptcy" => Some(HeadquartersHasBankruptcy),
                "Commercial Motor Vehicle was Involved in this Conviction" => {
                    Some(CommercialMotorVehicleWasInvolvedInThisConviction)
                }
                "Vehicle was Declared a Total Loss" => Some(VehicleWasDeclaredATotalLoss),
                "Commercial Motor Vehicle was Carrying Hazardous Materials when the Offense was Committed" => {
                    Some(
                        CommercialMotorVehicleWasCarryingHazardousMaterialsWhenTheOffenseWasCommitted,
                    )
                }
                "Prepared from Internal Book Figures" => {
                    Some(PreparedFromInternalBookFigures)
                }
                "Quantity Declined" => Some(QuantityDeclined),
                "Quantity Details Unknown" => Some(QuantityDetailsUnknown),
                "Was tax paid when purchased by seller?" => Some(CodeVY),
                "Was item depreciable?" => Some(CodeVZ),
                "Statement is on a Trading Trust" => Some(StatementIsOnATradingTrust),
                "New Registration" => Some(NewRegistration),
                "Mailing Address Change" => Some(MailingAddressChange),
                "Residence Address Change" => Some(ResidenceAddressChange),
                "Name Change" => Some(NameChange),
                "Party Enrollment Change" => Some(PartyEnrollmentChange),
                "Needs Absentee Ballot" => Some(NeedsAbsenteeBallot),
                "Would Like to be Election Day Worker" => {
                    Some(WouldLikeToBeElectionDayWorker)
                }
                "Duplicate Registration" => Some(DuplicateRegistration),
                "Forwarded Application" => Some(ForwardedApplication),
                "Walker Required" => Some(WalkerRequired),
                "Water On" => Some(WaterOn),
                "Application Incomplete" => Some(ApplicationIncomplete),
                "Vehicle Plate Surrendered" => Some(VehiclePlateSurrendered),
                "Written Notice to Note Holder" => Some(WrittenNoticeToNoteHolder),
                "Written Notice to Borrower" => Some(WrittenNoticeToBorrower),
                "Within Specified Time Period" => Some(WithinSpecifiedTimePeriod),
                "Within Specified Range" => Some(WithinSpecifiedRange),
                "Injury was Work Related" => Some(InjuryWasWorkRelated),
                "Dealer Pricing Authorization" => Some(DealerPricingAuthorization),
                "Summary Level Information" => Some(SummaryLevel),
                "Detail Level Information" => Some(DetailLevel),
                "Non-occupant Co-borrower" => Some(NonOccupantCoBorrower),
                "Unit is a Studio (Efficiency)" => Some(CodeWN),
                "Equipment in Working Order" => Some(EquipmentInWorkingOrder),
                "To be Watched" => Some(ToBeWatched),
                "Undetermined Out of Business Status" => {
                    Some(UndeterminedOutOfBusinessStatus)
                }
                "Wheelchair Required" => Some(WheelchairRequired),
                "Balance Sheet Filed" => Some(BalanceSheetFiled),
                "Winterized Tag Observed" => Some(WinterizedTagObserved),
                "Material Safety Data Sheet" => Some(MaterialSafetyDataSheet),
                "Accepts Credit Cards" => Some(AcceptsCreditCards),
                "All Purchases Made from Headquarters" => {
                    Some(AllPurchasesMadeFromHeadquarters)
                }
                "Busy" => Some(Busy),
                "Excessive" => Some(Excessive),
                "Fairly new" => Some(FairlyNew),
                "No Employees - Business Managed by Director(s)" => Some(CodeX0),
                "Gross Weekly Amount is Estimated" => Some(GrossWeeklyAmountIsEstimated),
                "Waiting Period Disability Days are Non-consecutive" => {
                    Some(WaitingPeriodDisabilityDaysAreNonConsecutive)
                }
                "Report Depicts Most Recent Data - Interim Period(s) Omitted" => {
                    Some(CodeX3)
                }
                "Permanent Impairment Paid at Minimum" => {
                    Some(PermanentImpairmentPaidAtMinimum)
                }
                "Employee's Death is a Result of Work Injury or Illness" => {
                    Some(EmployeesDeathIsAResultOfWorkInjuryOrIllness)
                }
                "Employee's Written Social Security Number Release is on File" => {
                    Some(EmployeesWrittenSocialSecurityNumberReleaseIsOnFile)
                }
                "Employee's Medical Records Release Authorization is on File" => {
                    Some(EmployeesMedicalRecordsReleaseAuthorizationIsOnFile)
                }
                "Employee Returned to Work with Pre-Injury Employer" => {
                    Some(EmployeeReturnedToWorkWithPreInjuryEmployer)
                }
                "\"Cafe\" Plan in Effect" => Some(CodeX9),
                "Figures are Average" => Some(FiguresAreAverage),
                "Imports" => Some(Imports),
                "In Process of Establishing" => Some(InProcessOfEstablishing),
                "Intercompany Relations Consist of Endorsements" => {
                    Some(IntercompanyRelationsConsistOfEndorsements)
                }
                "Intercompany Relations Consist of Guarantees" => {
                    Some(IntercompanyRelationsConsistOfGuarantees)
                }
                "Intercompany Relations Consist of Leasing Arrangements" => {
                    Some(IntercompanyRelationsConsistOfLeasingArrangements)
                }
                "Intercompany Relations Consist of Sharing Accounting" => {
                    Some(IntercompanyRelationsConsistOfSharingAccounting)
                }
                "Intercompany Relations Consist of Sharing Facilities" => {
                    Some(IntercompanyRelationsConsistOfSharingFacilities)
                }
                "Intercompany Relations Consist of Sharing Management" => {
                    Some(IntercompanyRelationsConsistOfSharingManagement)
                }
                "Intercompany Relations Consist of Sharing Personnel" => {
                    Some(IntercompanyRelationsConsistOfSharingPersonnel)
                }
                "Interest in Other Business(es) Along with Family" => Some(CodeXK),
                "Interest in Other Business(es) Along with Others in Reported Company" => {
                    Some(CodeXL)
                }
                "Inventory Valued at Company's Estimates" => {
                    Some(InventoryValuedAtCompanysEstimates)
                }
                "Inventory Valued at Cost" => Some(InventoryValuedAtCost),
                "Inventory Valued using AVCO (Average Cost)" => Some(CodeXO),
                "Joint Ownership" => Some(JointOwnership),
                "Leases with No Rent Payments" => Some(LeasesWithNoRentPayments),
                "Leases with Option to Buy" => Some(LeasesWithOptionToBuy),
                "Leases with Token Payment" => Some(LeasesWithTokenPayment),
                "Limited" => Some(Limited),
                "Located for Several Years" => Some(LocatedForSeveralYears),
                "Located Since Opening" => Some(LocatedSinceOpening),
                "Modern" => Some(Modern),
                "Non-Existent" => Some(NonExistent),
                "Officer or Owner in Other Businesses in the Same Field" => {
                    Some(OfficerOrOwnerInOtherBusinessesInTheSameField)
                }
                "Operates as a Distributor for Others" => {
                    Some(OperatesAsADistributorForOthers)
                }
                "Insured Cooperative" => Some(InsuredCooperative),
                "Worked in Industry for Several Years" => {
                    Some(WorkedInIndustryForSeveralYears)
                }
                "Aircraft Operation" => Some(AircraftOperation),
                "All Classifications on Policy Accounted For" => {
                    Some(AllClassificationsOnPolicyAccountedFor)
                }
                "Board Provided" => Some(BoardProvided),
                "Casual Labor" => Some(CasualLabor),
                "Certificates on File for All Subcontractors" => {
                    Some(CertificatesOnFileForAllSubcontractors)
                }
                "Commissions Paid" => Some(CommissionsPaid),
                "Condition or Type of Records Cause Additional Audit Time" => {
                    Some(ConditionOrTypeOfRecordsCauseAdditionalAuditTime)
                }
                "Domestic Workers Employed" => Some(DomesticWorkersEmployed),
                "Operates from Residence" => Some(OperatesFromResidence),
                "Operates under License by Others" => Some(OperatesUnderLicenseByOthers),
                "Rents from Month to Month" => Some(RentsFromMonthToMonth),
                "Semi-modern" => Some(SemiModern),
                "Under Construction" => Some(UnderConstruction),
                "Unlimited" => Some(Unlimited),
                "Used" => Some(Used),
                "Variable" => Some(Variable),
                "Holder is a Subsidiary of Reporting Agent" => {
                    Some(HolderIsASubsidiaryOfReportingAgent)
                }
                "Contact is Unchanged From Previous Report" => {
                    Some(ContactIsUnchangedFromPreviousReport)
                }
                "Report was Filed Last Year by This Agent" => {
                    Some(ReportWasFiledLastYearByThisAgent)
                }
                "Party is Authorized to do Business in This State" => {
                    Some(PartyIsAuthorizedToDoBusinessInThisState)
                }
                "Clear Decrease" => Some(ClearDecrease),
                "Employees Temporarily Laid Off" => Some(EmployeesTemporarilyLaidOff),
                "Established in the Industry" => Some(EstablishedInTheIndustry),
                "Global Business" => Some(GlobalBusiness),
                "Information to be Followed Up" => Some(InformationToBeFollowedUp),
                "Known Details are Listed" => Some(KnownDetailsAreListed),
                "Land is Rented" => Some(LandIsRented),
                "Low" => Some(Low),
                "Prime Commercial Area" => Some(PrimeCommercialArea),
                "Shares with Affiliated Company(ies)" => Some(CodeYV),
                "Slightly Higher" => Some(SlightlyHigher),
                "Slightly Lower" => Some(SlightlyLower),
                "Stagnant" => Some(Stagnant),
                "Territory Information is Available" => {
                    Some(TerritoryInformationIsAvailable)
                }
                "Subcontractors Used" => Some(SubcontractorsUsed),
                "Insured Is a Subcontractor" => Some(InsuredIsASubcontractor),
                "Insured Has Multiple Entries" => Some(InsuredHasMultipleEntries),
                "Insured Has Retail Operations" => Some(InsuredHasRetailOperations),
                "Insured Requested Division of Payroll of Employee(s)" => Some(CodeZ4),
                "Owner or Officer Interviewed" => Some(OwnerOrOfficerInterviewed),
                "Premium Overtime Excluded" => Some(PremiumOvertimeExcluded),
                "Records Reflect Proper Division of Employee(s) Payroll" => Some(CodeZ7),
                "Records Satisfactory for Audit" => Some(RecordsSatisfactoryForAudit),
                "Relatives Employed" => Some(RelativesEmployed),
                "Customer - Configuration Change is Required" => {
                    Some(CustomerConfigurationChangeIsRequired)
                }
                "Condition Board of Inspection and Survey (INSURV) is Mission Degrading" => {
                    Some(CodeZB)
                }
                "Condition Board of Inspection and Survey (INSURV) is Maintenance Related" => {
                    Some(CodeZC)
                }
                "Condition Board of Inspection and Survey (INSURV) is Safety Related" => {
                    Some(CodeZD)
                }
                "Repair is Mission Essential" => Some(RepairIsMissionEssential),
                "Repair is Safety Essential" => Some(RepairIsSafetyEssential),
                "Periodic Maintenance is Required" => Some(PeriodicMaintenanceIsRequired),
                "Condition Board of Inspection and Survey (INSURV) Discrepancy is Corrected" => {
                    Some(CodeZH)
                }
                "Progress is in Jeopardy" => Some(ProgressIsInJeopardy),
                "Employee's Injury or Illness is Work Related" => {
                    Some(EmployeesInjuryOrIllnessIsWorkRelated)
                }
                "Final - Configuration Change is Required" => {
                    Some(FinalConfigurationChangeIsRequired)
                }
                "Final - Delivery to Shop is Required" => {
                    Some(FinalDeliveryToShopIsRequired)
                }
                "Final - Requestor Workforce will Assist" => {
                    Some(FinalRequestorWorkforceWillAssist)
                }
                "Job is Level 2" => Some(JobIsLevel2),
                "Preliminary - Configuration Change is Required" => {
                    Some(PreliminaryConfigurationChangeIsRequired)
                }
                "Preliminary - Delivery to Shop is Required" => {
                    Some(PreliminaryDeliveryToShopIsRequired)
                }
                "Preliminary - Requestor Workforce will Assist" => {
                    Some(PreliminaryRequestorWorkforceWillAssist)
                }
                "Configuration Change is Associated with Time Meter" => {
                    Some(ConfigurationChangeIsAssociatedWithTimeMeter)
                }
                "Shop Has Lead Responsibility" => Some(ShopHasLeadResponsibility),
                "Estimate is Derived From Job Template" => {
                    Some(EstimateIsDerivedFromJobTemplate)
                }
                "Requestor Holds Technical Documentation" => {
                    Some(RequestorHoldsTechnicalDocumentation)
                }
                "Replacement Item" => Some(ReplacementItem),
                "Canadian Standards Association (CSA) Approved" => Some(CodeZW),
                "Non-convertible" => Some(NonConvertible),
                "Underwriters Laboratory (UL) Approved" => Some(CodeZY),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for ConditionIndicator {
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
    type Value = ConditionIndicator;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Condition Indicator")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ConditionIndicator::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Condition Indicator: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ConditionIndicator::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Condition Indicator: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ConditionIndicator {
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