use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**673

See docs at <https://www.stedi.com/edi/x12/element/673>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantityQualifier {
    ///00 - Hospital/Homebound Individuals
    HospitalHomeboundIndividuals,
    ///0A - Number of Hours Per Day
    NumberOfHoursPerDay,
    ///0B - Number of Hours Per Week
    NumberOfHoursPerWeek,
    ///0C - Number of Months Per Year
    NumberOfMonthsPerYear,
    ///0D - Number of Periods Per Week
    NumberOfPeriodsPerWeek,
    ///0E - Expected Expenditure Quantity
    ExpectedExpenditureQuantity,
    ///0F - Number of Hours Per Year
    NumberOfHoursPerYear,
    ///0G - Pre-Kindergarten Students
    PreKindergartenStudents,
    ///0H - First Grade Students
    FirstGradeStudents,
    ///0I - Second Grade Students
    SecondGradeStudents,
    ///0J - Third Grade Students
    ThirdGradeStudents,
    ///0K - Fourth Grade Students
    FourthGradeStudents,
    ///0L - Fifth Grade Students
    FifthGradeStudents,
    ///0M - Sixth Grade Students
    SixthGradeStudents,
    ///0N - Seventh Grade Students
    SeventhGradeStudents,
    ///0O - Eighth Grade Students
    EighthGradeStudents,
    ///0P - Ninth Grade Students
    NinthGradeStudents,
    ///0Q - Carnegie Units
    CarnegieUnits,
    ///0R - Number of Disability Types
    NumberOfDisabilityTypes,
    ///0S - Number of Males
    NumberOfMales,
    ///0T - Number of Females
    NumberOfFemales,
    ///0U - Individuals with Multiple Disabilities
    IndividualsWithMultipleDisabilities,
    ///0V - Individuals with Serious Emotional Disturbance
    IndividualsWithSeriousEmotionalDisturbance,
    ///0W - Individuals with Speech or Language Impairment
    IndividualsWithSpeechOrLanguageImpairment,
    ///0X - Individuals with Traumatic Brain Injury
    IndividualsWithTraumaticBrainInjury,
    ///0Y - Blind Individuals
    BlindIndividuals,
    ///0Z - Deaf Individuals
    DeafIndividuals,
    ///01 - Discrete Quantity
    DiscreteQuantity,
    ///1A - Original Duration (in calendar units)
    Code1A,
    ///1B - Current Duration (in calendar units)
    Code1B,
    ///1C - Remaining Duration (in calendar units)
    Code1C,
    ///1D - Total Float (in calendar units)
    Code1D,
    ///1E - Free Float (in calendar units)
    Code1E,
    ///1F - Lag (as in Lag Time - in calendar units)
    Code1F,
    ///1G - Lead Time (in calendar units)
    Code1G,
    ///1H - Started
    Started,
    ///1I - Completed
    Completed,
    ///1J - Due
    Due,
    ///1K - Time Units
    TimeUnits,
    ///1L - Shifts
    Shifts,
    ///1M - Time units per shift
    TimeUnitsPerShift,
    ///1N - Scrap allowed
    ScrapAllowed,
    ///1O - Calendar Units
    CalendarUnits,
    ///1P - Resource (Quantity) available
    Code1P,
    ///1Q - Total Resource (Quantity)
    Code1Q,
    ///1R - Level Resource (Quantity)
    Code1R,
    ///1S - Late
    Late,
    ///1T - Number of Delinquent Installments
    NumberOfDelinquentInstallments,
    ///1U - Number of Loans
    NumberOfLoans,
    ///1V - Total Number of Mortgagees
    TotalNumberOfMortgagees,
    ///1W - Total Number of Loan Detail Records
    TotalNumberOfLoanDetailRecords,
    ///1X - Prescription Effective Period
    PrescriptionEffectivePeriod,
    ///1Y - Rate Per Day (RPD)
    Code1Y,
    ///1Z - End Of Month Inventory Prior To Ship
    EndOfMonthInventoryPriorToShip,
    ///02 - Cumulative Quantity
    CumulativeQuantity,
    ///2A - Commitment Period
    CommitmentPeriod,
    ///2B - Number of Borrowers
    NumberOfBorrowers,
    ///2C - Number of Adjustment Periods
    NumberOfAdjustmentPeriods,
    ///2D - Age Nearest
    AgeNearest,
    ///2E - Total Other Properties Owned and Financed
    TotalOtherPropertiesOwnedAndFinanced,
    ///2F - Age Next
    AgeNext,
    ///2G - Reconsideration Period
    ReconsiderationPeriod,
    ///2H - Flat Extra Premium
    FlatExtraPremium,
    ///2I - CO2 Injection Volume
    Co2InjectionVolume,
    ///2J - Accounts Placed for Collection
    AccountsPlacedForCollection,
    ///2K - Changes
    Changes,
    ///2L - Companies in Same Activity for a Period
    CompaniesInSameActivityForAPeriod,
    ///2M - Comparison Period
    ComparisonPeriod,
    ///2N - Departments
    Departments,
    ///2O - Employees Shared
    EmployeesShared,
    ///2P - Estimated Accounts
    EstimatedAccounts,
    ///2Q - Installed Capacity
    InstalledCapacity,
    ///2R - Levels Occupied
    LevelsOccupied,
    ///2S - Registered Brands Distributed
    RegisteredBrandsDistributed,
    ///2T - Electronic Signatures
    ElectronicSignatures,
    ///2U - Bytes
    Bytes,
    ///2V - Employed at this Location
    EmployedAtThisLocation,
    ///2W - Segments
    Segments,
    ///2X - Registered Brands Manufactured
    RegisteredBrandsManufactured,
    ///2Y - Functional Groups
    FunctionalGroups,
    ///2Z - Transaction Sets
    TransactionSets,
    ///03 - Discreet Quantity - Rejected Material
    DiscreetQuantityRejectedMaterial,
    ///3A - Total Credits Accepted
    TotalCreditsAccepted,
    ///3B - Total Credits Rejected
    TotalCreditsRejected,
    ///3C - Total Debits Accepted
    TotalDebitsAccepted,
    ///3D - Total Debits Rejected
    TotalDebitsRejected,
    ///3E - Total Payments Rejected
    TotalPaymentsRejected,
    ///3F - Total Pre-advices Accepted
    TotalPreAdvicesAccepted,
    ///3G - Total Pre-advices Rejected
    TotalPreAdvicesRejected,
    ///3H - Total Prenotes Accepted
    TotalPrenotesAccepted,
    ///3I - Total Prenotes Rejected
    TotalPrenotesRejected,
    ///3J - Total Post-advices Accepted
    TotalPostAdvicesAccepted,
    ///3K - Total Post-advices Rejected
    TotalPostAdvicesRejected,
    ///3L - Total Unidentified Transactions Rejected
    TotalUnidentifiedTransactionsRejected,
    ///3M - Total Credits Received
    TotalCreditsReceived,
    ///3N - Total Debits Received
    TotalDebitsReceived,
    ///3O - Individuals with Noncategorical Preschool Disability
    IndividualsWithNoncategoricalPreschoolDisability,
    ///3P - Total Pre-advices Received
    TotalPreAdvicesReceived,
    ///3Q - Total Prenotes Received
    TotalPrenotesReceived,
    ///3R - Total Post-advices Received
    TotalPostAdvicesReceived,
    ///3S - Total Debits
    TotalDebits,
    ///3T - Total Credits
    TotalCredits,
    ///3V - Minimum Transfer
    MinimumTransfer,
    ///3W - Maximum Transfer
    MaximumTransfer,
    ///3X - Speed Capacity
    SpeedCapacity,
    ///3Y - Subcontractors
    Subcontractors,
    ///3Z - Students
    Students,
    ///04 - Discrete Quantity - Rejected Material: Disposition Replacement
    DiscreteQuantityRejectedMaterialDispositionReplacement,
    ///4A - Accounts
    Accounts,
    ///4B - Agents
    Agents,
    ///4C - Authorized Shares
    AuthorizedShares,
    ///4D - Clerks
    Clerks,
    ///4E - Design Employees
    DesignEmployees,
    ///4F - Foreign Related Entities
    ForeignRelatedEntities,
    ///4G - Group Employees
    GroupEmployees,
    ///4H - Issued Shares
    IssuedShares,
    ///4I - Laborers
    Laborers,
    ///4J - Other Employee Type
    OtherEmployeeType,
    ///4K - Part Time Employees
    PartTimeEmployees,
    ///4L - Related Entities
    RelatedEntities,
    ///4M - Relatives Employed
    RelativesEmployed,
    ///4N - Salespersons
    Salespersons,
    ///4O - Space Occupied
    SpaceOccupied,
    ///4P - Special Partners
    SpecialPartners,
    ///4Q - Suppliers' Credit
    SuppliersCredit,
    ///4R - Technicians
    Technicians,
    ///4S - Trainees
    Trainees,
    ///4T - Warehouse Employees
    WarehouseEmployees,
    ///4U - Shareholders
    Shareholders,
    ///4V - Available Units
    AvailableUnits,
    ///4W - Total Unduplicated Headcount
    TotalUnduplicatedHeadcount,
    ///4X - Meals Per Week
    MealsPerWeek,
    ///4Y - Programs Offered
    ProgramsOffered,
    ///4Z - Typical Credit Hours Taken per Graduate Student (Full Time)
    Code4Z,
    ///05 - Discrete Quantity - Rejected Material: Disposition Credit
    DiscreteQuantityRejectedMaterialDispositionCredit,
    ///5A - Aggregate Benefit Period
    AggregateBenefitPeriod,
    ///5B - Anticipated Length of Service
    AnticipatedLengthOfService,
    ///5C - Approval/Offer Duration
    ApprovalOfferDuration,
    ///5D - Benefit Amount
    BenefitAmount,
    ///5E - Benefit Period
    BenefitPeriod,
    ///5F - Brothers Deceased
    BrothersDeceased,
    ///5G - Brothers Living
    BrothersLiving,
    ///5H - Children
    Children,
    ///5I - Citations
    Citations,
    ///5J - Claim Period
    ClaimPeriod,
    ///5K - Coverage
    Coverage,
    ///5L - Elimination Period
    EliminationPeriod,
    ///5M - Elimination Period - Accident
    EliminationPeriodAccident,
    ///5N - Elimination Period - Sickness
    EliminationPeriodSickness,
    ///5O - Employees - Nonowner
    EmployeesNonowner,
    ///5P - Employees - Owner
    EmployeesOwner,
    ///5Q - Employees - Part Time
    EmployeesPartTime,
    ///5R - Employees - Same Duties
    EmployeesSameDuties,
    ///5S - Employees - Same Occupation
    EmployeesSameOccupation,
    ///5T - Expense
    Expense,
    ///5U - Frequency
    Frequency,
    ///5V - General Elimination Period
    GeneralEliminationPeriod,
    ///5W - Guarantee Period
    GuaranteePeriod,
    ///5X - Height
    Height,
    ///5Y - Hours Flown - Aircraft Type/Life
    HoursFlownAircraftTypeLife,
    ///5Z - Hours Flown - Aircraft Type/Period
    HoursFlownAircraftTypePeriod,
    ///06 - Discrete Quantity - Rejected Material: Disposition Pending
    DiscreteQuantityRejectedMaterialDispositionPending,
    ///6A - Hours Flown - Aircraft/Type Flying
    HoursFlownAircraftTypeFlying,
    ///6B - Hours Flown - Lifetime
    HoursFlownLifetime,
    ///6C - Hours Flown - Type Flying
    HoursFlownTypeFlying,
    ///6D - Impairment Duration
    ImpairmentDuration,
    ///6E - Impairment Frequency
    ImpairmentFrequency,
    ///6F - Installment Frequency
    InstallmentFrequency,
    ///6G - Installments
    Installments,
    ///6H - Intended Change Time Period
    IntendedChangeTimePeriod,
    ///6I - Interim Term Period
    InterimTermPeriod,
    ///6J - Involvement Period
    InvolvementPeriod,
    ///6K - Loan Rate
    LoanRate,
    ///6L - Maximum Age
    MaximumAge,
    ///6M - Maximum Benefit Period - Accident
    MaximumBenefitPeriodAccident,
    ///6N - Maximum Benefit Period - Sickness
    MaximumBenefitPeriodSickness,
    ///6O - Maximum Benefit Period
    MaximumBenefitPeriod,
    ///6P - Medication Duration
    MedicationDuration,
    ///6Q - Minimum Age
    MinimumAge,
    ///6R - Own Occupation Qualification Period
    OwnOccupationQualificationPeriod,
    ///6S - Owner's Equity
    OwnersEquity,
    ///6T - Ownership Change Age
    OwnershipChangeAge,
    ///6U - Ownership Duration
    OwnershipDuration,
    ///6V - Ownership Percentage
    OwnershipPercentage,
    ///6W - Payment Frequency
    PaymentFrequency,
    ///6X - Payments Number
    PaymentsNumber,
    ///6Y - Arrests
    Arrests,
    ///6Z - Placement Period Expiration
    PlacementPeriodExpiration,
    ///07 - Cumulative Quantity - Rejected Material
    CumulativeQuantityRejectedMaterial,
    ///7A - Previous Benefits
    PreviousBenefits,
    ///7B - Qualification Period
    QualificationPeriod,
    ///7C - Range Average
    RangeAverage,
    ///7D - Range Maximum
    RangeMaximum,
    ///7E - Range Minimum
    RangeMinimum,
    ///7F - Relationship Duration
    RelationshipDuration,
    ///7G - Replaced Amount
    ReplacedAmount,
    ///7H - Residence Duration
    ResidenceDuration,
    ///7I - Sisters Deceased
    SistersDeceased,
    ///7J - Sisters Living
    SistersLiving,
    ///7K - Time Frame
    TimeFrame,
    ///7L - Time in Country
    TimeInCountry,
    ///7M - Time Since Hospitalization
    TimeSinceHospitalization,
    ///7N - Time Since Last Application
    TimeSinceLastApplication,
    ///7O - Time Since Last Civilian Flight
    TimeSinceLastCivilianFlight,
    ///7P - Time Since Last Insurance Medical
    TimeSinceLastInsuranceMedical,
    ///7Q - Time Since Last Military Flight
    TimeSinceLastMilitaryFlight,
    ///7R - Time Since Medical Consult
    TimeSinceMedicalConsult,
    ///7S - Time Since Medication End
    TimeSinceMedicationEnd,
    ///7T - Time Since Medication Start
    TimeSinceMedicationStart,
    ///7U - Time Since Onset
    TimeSinceOnset,
    ///7V - Time Since Surgery
    TimeSinceSurgery,
    ///7W - Time Since Trip
    TimeSinceTrip,
    ///7X - Travel Frequency
    TravelFrequency,
    ///7Y - Travel Period
    TravelPeriod,
    ///7Z - Trip Duration
    TripDuration,
    ///08 - Cumulative Quantity - Rejected Material: Disposition Replacement
    CumulativeQuantityRejectedMaterialDispositionReplacement,
    ///8A - Visitation Frequency
    VisitationFrequency,
    ///8B - Weight
    Weight,
    ///8C - Weight Change Period
    WeightChangePeriod,
    ///8D - Work Period
    WorkPeriod,
    ///8E - Existence Limit Period
    ExistenceLimitPeriod,
    ///8F - Shares
    Shares,
    ///8G - Directors
    Directors,
    ///8H - Minimum
    Minimum,
    ///8I - Voting Shares Held
    VotingSharesHeld,
    ///8J - Outstanding Shares
    OutstandingShares,
    ///8K - Shares Held as Treasury Stock
    SharesHeldAsTreasuryStock,
    ///8L - Shares Subscribed but Not Issued
    SharesSubscribedButNotIssued,
    ///8M - Total Shares of Stock
    TotalSharesOfStock,
    ///8N - Shares Owned by In-State Residents
    SharesOwnedByInStateResidents,
    ///8O - Shares Owned by Out-of-State Residents
    SharesOwnedByOutOfStateResidents,
    ///8P - Partners
    Partners,
    ///8Q - Land Holding
    LandHolding,
    ///8R - Non-Domestic Stockholders
    NonDomesticStockholders,
    ///8S - Shares Subscribed
    SharesSubscribed,
    ///8T - Maximum Number Free Miles
    MaximumNumberFreeMiles,
    ///8U - Typical Credit Hours Taken per Undergraduate Student (Full Time)
    Code8U,
    ///8V - Typical Credit Hours Taken per First-Professional Student (Full Time)
    Code8V,
    ///8W - Full-time Equivalents
    FullTimeEquivalents,
    ///8X - Total Credit Hours
    TotalCreditHours,
    ///8Y - Total Non-Credit Hours
    TotalNonCreditHours,
    ///8Z - Total Contact Hours
    TotalContactHours,
    ///09 - Cumulative Quantity - Rejected Material: Disposition Credit
    CumulativeQuantityRejectedMaterialDispositionCredit,
    ///9A - Time Expended
    TimeExpended,
    ///9C - Primary Meter Reading Value
    PrimaryMeterReadingValue,
    ///9D - Engineered Standard
    EngineeredStandard,
    ///9E - Active Maintenance Time
    ActiveMaintenanceTime,
    ///9F - Actual Duration
    ActualDuration,
    ///9H - Estimated Duration
    EstimatedDuration,
    ///9J - Gross Estimate
    GrossEstimate,
    ///9K - Finish Offset
    FinishOffset,
    ///9L - Start Offset
    StartOffset,
    ///9M - Picture Count
    PictureCount,
    ///9N - Component Meter Reading Count
    ComponentMeterReadingCount,
    ///9P - Total Clock Hours
    TotalClockHours,
    ///9R - Enrollees
    Enrollees,
    ///9S - Total Days Submitted
    TotalDaysSubmitted,
    ///9V - Total Days Approved
    TotalDaysApproved,
    ///10 - Cumulative Quantity - Rejected Material: Disposition Pending
    CumulativeQuantityRejectedMaterialDispositionPending,
    ///11 - Split Quantity
    SplitQuantity,
    ///12 - Ship Notice Quantity
    ShipNoticeQuantity,
    ///13 - Collateral Requirements
    CollateralRequirements,
    ///14 - Quantity in Float
    QuantityInFloat,
    ///15 - Quantity in Hold Out
    QuantityInHoldOut,
    ///16 - Line Thread Quantity
    LineThreadQuantity,
    ///17 - Quantity on Hand
    QuantityOnHand,
    ///18 - Previous Week Quantity
    PreviousWeekQuantity,
    ///19 - Unverified Receipts
    UnverifiedReceipts,
    ///20 - Unusable Quantity
    UnusableQuantity,
    ///21 - Cumulative Quantity Shipped Short - Disposition Pending
    CumulativeQuantityShippedShortDispositionPending,
    ///22 - Cumulative Quantity Shipped Short - Disposition Challenged
    CumulativeQuantityShippedShortDispositionChallenged,
    ///23 - Cumulative Quantity Shipped Long - Disposition Pending
    CumulativeQuantityShippedLongDispositionPending,
    ///24 - Cumulative Quantity Shipped Long - Disposition Challenged
    CumulativeQuantityShippedLongDispositionChallenged,
    ///25 - OEM Inventory
    OemInventory,
    ///26 - Total Inventory
    TotalInventory,
    ///27 - Committed Quantity
    CommittedQuantity,
    ///28 - Quantity Available for Return
    QuantityAvailableForReturn,
    ///29 - Projected Available Inventory
    ProjectedAvailableInventory,
    ///30 - Quote Quantity on Inventory
    QuoteQuantityOnInventory,
    ///31 - Additional Demand Quantity
    AdditionalDemandQuantity,
    ///32 - Quantity Sold
    QuantitySold,
    ///33 - Quantity Available for Sale (stock quantity)
    Code33,
    ///34 - Noncommitted Inventory on Shelf
    NoncommittedInventoryOnShelf,
    ///35 - Inventory on Shelf + Work in Progress
    InventoryOnShelfWorkInProgress,
    ///36 - Distributor Inventory
    DistributorInventory,
    ///37 - Work In Process
    WorkInProcess,
    ///38 - Original Quantity
    OriginalQuantity,
    ///39 - Shipped Quantity
    ShippedQuantity,
    ///40 - Remaining Quantity
    RemainingQuantity,
    ///41 - Number of Batches
    NumberOfBatches,
    ///42 - Number of Checks
    NumberOfChecks,
    ///43 - Talk Paths
    TalkPaths,
    ///44 - Number of Patient Admissions
    NumberOfPatientAdmissions,
    ///45 - Cumulative quantity on order
    CumulativeQuantityOnOrder,
    ///46 - Total transactions
    TotalTransactions,
    ///47 - Primary Net Quantity
    PrimaryNetQuantity,
    ///48 - Secondary Net Quantity
    SecondaryNetQuantity,
    ///49 - Number of Signed Bills of Lading
    NumberOfSignedBillsOfLading,
    ///50 - Number of Copies of Bill of Lading
    NumberOfCopiesOfBillOfLading,
    ///51 - Number of Unsigned Bills of Lading
    NumberOfUnsignedBillsOfLading,
    ///52 - Number of Originals
    NumberOfOriginals,
    ///53 - Original payment item count.
    OriginalPaymentItemCount,
    ///54 - Bank reject item count.
    BankRejectItemCount,
    ///55 - Net to pay item count.
    NetToPayItemCount,
    ///56 - Minimum Contract Quantity
    MinimumContractQuantity,
    ///57 - Minimum Order Quantity
    MinimumOrderQuantity,
    ///58 - Payment Cancellation Item Count
    PaymentCancellationItemCount,
    ///59 - Individuals with Developmental Delay
    IndividualsWithDevelopmentalDelay,
    ///60 - Total Authorized Quantity
    TotalAuthorizedQuantity,
    ///61 - Remaining Authorized Quantity
    RemainingAuthorizedQuantity,
    ///62 - Number of Days Covered by Inventory
    NumberOfDaysCoveredByInventory,
    ///63 - On Order Quantity
    OnOrderQuantity,
    ///64 - Past Due Quantity
    PastDueQuantity,
    ///65 - Previous Month's Usage
    PreviousMonthsUsage,
    ///66 - Minimum Fabrication Quantity
    MinimumFabricationQuantity,
    ///67 - Minimum Ship Quantity
    MinimumShipQuantity,
    ///68 - Maximum Number of Shipments Allowed
    MaximumNumberOfShipmentsAllowed,
    ///69 - Incremental Order Quantity
    IncrementalOrderQuantity,
    ///70 - Maximum Order Quantity
    MaximumOrderQuantity,
    ///71 - Educable Mentally Retarded Individuals
    EducableMentallyRetardedIndividuals,
    ///72 - Minimum Stock Level
    MinimumStockLevel,
    ///73 - Maximum Stock Level
    MaximumStockLevel,
    ///74 - Damaged Goods
    DamagedGoods,
    ///75 - Receipts
    Receipts,
    ///76 - Returns
    Returns,
    ///77 - Stock Transfers In
    StockTransfersIn,
    ///78 - Stock Transfers Out
    StockTransfersOut,
    ///79 - Billing Unit(s) Per Pricing Unit
    Code79,
    ///80 - Pricing Unit(s) Per Billing Unit
    Code80,
    ///81 - Prepaid Quantity Shipped
    PrepaidQuantityShipped,
    ///82 - Prepaid Quantity Not Shipped
    PrepaidQuantityNotShipped,
    ///83 - Submitted Quantity Sold
    SubmittedQuantitySold,
    ///84 - Submitted Quantity Returned
    SubmittedQuantityReturned,
    ///85 - Lot Size
    LotSize,
    ///86 - Nonconformance Quantity
    NonconformanceQuantity,
    ///87 - Quantity Received
    QuantityReceived,
    ///88 - Beds
    Beds,
    ///89 - Operating Beds
    OperatingBeds,
    ///90 - Acknowledged Quantity
    AcknowledgedQuantity,
    ///91 - Additional Usage Quantity
    AdditionalUsageQuantity,
    ///92 - Allotted Usage Quantity
    AllottedUsageQuantity,
    ///93 - Attendant-Handled Quantity
    AttendantHandledQuantity,
    ///94 - Billable Quantity
    BillableQuantity,
    ///95 - Data Storage Quantity
    DataStorageQuantity,
    ///96 - Non-Billable Quantity
    NonBillableQuantity,
    ///97 - Non-Urgent Delivery Quantity
    NonUrgentDeliveryQuantity,
    ///98 - Overflow Quantity
    OverflowQuantity,
    ///99 - Quantity Used
    QuantityUsed,
    ///A0 - Severely Mentally Retarded Individuals
    SeverelyMentallyRetardedIndividuals,
    ///A1 - Acceptable Unserviceable Quantity
    AcceptableUnserviceableQuantity,
    ///A2 - Optimistic Duration
    OptimisticDuration,
    ///A3 - Most Likely Duration
    MostLikelyDuration,
    ///A4 - Pessimistic Duration
    PessimisticDuration,
    ///A5 - Adjusted Quantity
    AdjustedQuantity,
    ///A6 - Accidents
    Accidents,
    ///A7 - Years in School
    YearsInSchool,
    ///A8 - Number of Dependents
    NumberOfDependents,
    ///A9 - Years on Job
    YearsOnJob,
    ///AA - Unacknowledged Quantity
    UnacknowledgedQuantity,
    ///AB - Urgent Delivery Quantity
    UrgentDeliveryQuantity,
    ///AC - Voice Storage Quantity
    VoiceStorageQuantity,
    ///AD - Maintenance Units
    MaintenanceUnits,
    ///AE - Minimum Average Time Requirement (MATR) Units
    CodeAE,
    ///AF - Wide Area Telephone Service (WATS)/800 Service Units
    CodeAF,
    ///AG - Number of End Users
    NumberOfEndUsers,
    ///AH - Number of Message Recipients
    NumberOfMessageRecipients,
    ///AI - Number of Operator Credits
    NumberOfOperatorCredits,
    ///AJ - Daily Adjustments
    DailyAdjustments,
    ///AK - Years in this Line of Work/Profession
    YearsInThisLineOfWorkProfession,
    ///AL - Area per Units
    AreaPerUnits,
    ///AM - Trainable Mentally Retarded Individuals
    TrainableMentallyRetardedIndividuals,
    ///AN - Age at Death
    AgeAtDeath,
    ///AO - Verified Receipts
    VerifiedReceipts,
    ///AP - Order Quantity Multiple
    OrderQuantityMultiple,
    ///AQ - Contribution Total
    ContributionTotal,
    ///AR - Loan Repayment Total
    LoanRepaymentTotal,
    ///AS - Participant Total
    ParticipantTotal,
    ///AT - Actual
    Actual,
    ///AU - Cumulative Actual
    CumulativeActual,
    ///AV - Budget
    Budget,
    ///AW - Cumulative Budget
    CumulativeBudget,
    ///AX - Number of Insured Lives
    NumberOfInsuredLives,
    ///AY - Forecast
    Forecast,
    ///AZ - Forecast at Complete
    ForecastAtComplete,
    ///B1 - Number of Mortgagors
    NumberOfMortgagors,
    ///B2 - Mortgage Pool Count
    MortgagePoolCount,
    ///B3 - Requested Amount
    RequestedAmount,
    ///B4 - Approved Amount
    ApprovedAmount,
    ///B5 - Additional Amount
    AdditionalAmount,
    ///B6 - Pre-op Days
    PreOpDays,
    ///B7 - Post-op Days
    PostOpDays,
    ///B8 - Average
    Average,
    ///B9 - Period Beginning Imbalance Quantity
    PeriodBeginningImbalanceQuantity,
    ///BA - Due-In
    DueIn,
    ///BB - Contractor Cumulative to Date
    ContractorCumulativeToDate,
    ///BC - Budget At Complete
    BudgetAtComplete,
    ///BD - Contractor at Complete
    ContractorAtComplete,
    ///BE - Subcontractor Cumulative to Date
    SubcontractorCumulativeToDate,
    ///BF - Age Modifying Units
    AgeModifyingUnits,
    ///BG - Subcontractor at Complete
    SubcontractorAtComplete,
    ///BH - Book Order Quantity
    BookOrderQuantity,
    ///BI - Book Inventory
    BookInventory,
    ///BJ - Bedroom Count
    BedroomCount,
    ///BK - Bathroom Count
    BathroomCount,
    ///BL - Betterment Hours
    BettermentHours,
    ///BM - Depreciation Hours
    DepreciationHours,
    ///BN - System Adjusted Hours
    SystemAdjustedHours,
    ///BO - User Adjusted Hours
    UserAdjustedHours,
    ///BP - Period Ending Imbalance Quantity
    PeriodEndingImbalanceQuantity,
    ///BQ - Backorder Quantity
    BackorderQuantity,
    ///BR - Blood Record
    BloodRecord,
    ///BS - Cumulative Beginning Imbalance Quantity
    CumulativeBeginningImbalanceQuantity,
    ///BT - Cumulative Current Period Imbalance Quantity
    CumulativeCurrentPeriodImbalanceQuantity,
    ///BU - Cumulative Prior Period Adjustment
    CumulativePriorPeriodAdjustment,
    ///BV - Cumulative Ending Imbalance Quantity
    CumulativeEndingImbalanceQuantity,
    ///BW - Birth Weight
    BirthWeight,
    ///BX - Current Period Imbalance Quantity
    CurrentPeriodImbalanceQuantity,
    ///BY - Production Delivery Quantity
    ProductionDeliveryQuantity,
    ///BZ - Entitlement Quantity
    EntitlementQuantity,
    ///C0 - Creditors
    Creditors,
    ///C1 - Payment Experiences in Last 12 Months
    PaymentExperiencesInLast12Months,
    ///C2 - Payment Experiences in Last 3 Months
    PaymentExperiencesInLast3Months,
    ///C3 - Area Damaged
    AreaDamaged,
    ///C4 - Other Unlisted Stockholders
    OtherUnlistedStockholders,
    ///C5 - Other Unlisted Participants
    OtherUnlistedParticipants,
    ///CA - Covered - Actual
    CoveredActual,
    ///CB - Closing Statement Balance
    ClosingStatementBalance,
    ///CC - Current Days on Market
    CurrentDaysOnMarket,
    ///CD - Co-insured - Actual
    CoInsuredActual,
    ///CE - Covered - Estimated
    CoveredEstimated,
    ///CF - Co-insured - Estimated
    CoInsuredEstimated,
    ///CG - Cumulative Gas Volume
    CumulativeGasVolume,
    ///CH - Cumulative Effect of Prior Period Adjustment
    CumulativeEffectOfPriorPeriodAdjustment,
    ///CI - Cumulative Gas Injection Volume
    CumulativeGasInjectionVolume,
    ///CL - Cumulative Liquid Injection Volume
    CumulativeLiquidInjectionVolume,
    ///CM - Number of Components
    NumberOfComponents,
    ///CN - Continuance Duration
    ContinuanceDuration,
    ///CO - Cumulative Oil/Condensate Volume
    CumulativeOilCondensateVolume,
    ///CP - Current Period Imbalance
    CurrentPeriodImbalance,
    ///CR - Certified Registered Nurse Anesthetist (CRNA) Number of Concurrent Procedures
    CodeCR,
    ///CS - Current Service Life
    CurrentServiceLife,
    ///CW - Cumulative Water Volume
    CumulativeWaterVolume,
    ///CY - Convictions Sent
    ConvictionsSent,
    ///CZ - Total Number of Convictions
    TotalNumberOfConvictions,
    ///D0 - Engineers
    Engineers,
    ///D1 - Billed
    Billed,
    ///D2 - Executives
    Executives,
    ///D3 - Number of Co-insurance Days
    NumberOfCoInsuranceDays,
    ///D4 - Field Workers
    FieldWorkers,
    ///D5 - Installers
    Installers,
    ///D6 - Members in Group
    MembersInGroup,
    ///D7 - Non-Consolidated Total-Domestic Subsidiaries
    NonConsolidatedTotalDomesticSubsidiaries,
    ///D8 - Non-Consolidated Total-Foreign Subsidiaries
    NonConsolidatedTotalForeignSubsidiaries,
    ///D9 - Non-Union Employees
    NonUnionEmployees,
    ///DA - Dependent's Age
    DependentsAge,
    ///DB - Deductible Blood Units
    DeductibleBloodUnits,
    ///DC - Dependent Count
    DependentCount,
    ///DD - Distributed
    Distributed,
    ///DE - Debited
    Debited,
    ///DF - Deleted
    Deleted,
    ///DG - Gas Used for Drilling
    GasUsedForDrilling,
    ///DH - Maximum Benefit Period Accident to Age
    MaximumBenefitPeriodAccidentToAge,
    ///DI - Disposed
    Disposed,
    ///DJ - Maximum Benefit Period Sickness to Age
    MaximumBenefitPeriodSicknessToAge,
    ///DK - Airline Attendants
    AirlineAttendants,
    ///DL - Companies Included in Consolidation
    CompaniesIncludedInConsolidation,
    ///DM - Total Consolidated Domestic Subsidiaries
    TotalConsolidatedDomesticSubsidiaries,
    ///DN - Default Notification Response Period
    DefaultNotificationResponsePeriod,
    ///DO - Days Operated
    DaysOperated,
    ///DP - Days Produced
    DaysProduced,
    ///DQ - Total Consolidated Foreign Subsidiaries
    TotalConsolidatedForeignSubsidiaries,
    ///DR - Direct Workers
    DirectWorkers,
    ///DS - Dose
    Dose,
    ///DT - Dependent Total
    DependentTotal,
    ///DU - Counter Clerks
    CounterClerks,
    ///DV - Design Capacity
    DesignCapacity,
    ///DW - Domestic Affiliated Companies
    DomesticAffiliatedCompanies,
    ///DX - Drivers
    Drivers,
    ///DY - Days
    Days,
    ///DZ - Employed at Location
    EmployedAtLocation,
    ///E1 - Course Segments
    CourseSegments,
    ///E2 - Degree Segments
    DegreeSegments,
    ///E3 - Employed on this job
    EmployedOnThisJob,
    ///E4 - Employed in this Profession
    EmployedInThisProfession,
    ///E5 - Employed by this Company
    EmployedByThisCompany,
    ///E8 - Number of Entitled Exemptions
    NumberOfEntitledExemptions,
    ///E9 - Number of Withholding Exemptions
    NumberOfWithholdingExemptions,
    ///EA - Exclusive Uses
    ExclusiveUses,
    ///EB - Nonexclusive Uses
    NonexclusiveUses,
    ///EC - Use of Extracorporeal Circulation
    UseOfExtracorporealCirculation,
    ///ED - Domestic Uses
    DomesticUses,
    ///EE - Small Business Uses
    SmallBusinessUses,
    ///EF - Nurses
    Nurses,
    ///EG - Office Workers
    OfficeWorkers,
    ///EH - Paid in Common Shares
    PaidInCommonShares,
    ///EI - Paid in Preferred Shares
    PaidInPreferredShares,
    ///EJ - Pilots
    Pilots,
    ///EK - Plant Workers
    PlantWorkers,
    ///EL - Principals Included as Employees
    PrincipalsIncludedAsEmployees,
    ///EM - Emergency Modifying Units
    EmergencyModifyingUnits,
    ///EN - Suppliers
    Suppliers,
    ///EO - Teachers
    Teachers,
    ///EP - Product Exchange Amount
    ProductExchangeAmount,
    ///EQ - Equity Security Holder
    EquitySecurityHolder,
    ///ER - Estimated Remaining Economic Life
    EstimatedRemainingEconomicLife,
    ///ES - Ending Stock
    EndingStock,
    ///ET - Employee Total
    EmployeeTotal,
    ///EU - Total Consolidated Subsidiaries
    TotalConsolidatedSubsidiaries,
    ///EV - Total Non-Consolidated Subsidiaries
    TotalNonConsolidatedSubsidiaries,
    ///EW - Evaporated Water
    EvaporatedWater,
    ///EX - Union Employees
    UnionEmployees,
    ///EY - Ported Telephone Lines
    PortedTelephoneLines,
    ///EZ - Service Resale
    ServiceResale,
    ///F0 - Total claims with skin diseases or disorders
    TotalClaimsWithSkinDiseasesOrDisorders,
    ///F1 - Off Lease Fuel
    OffLeaseFuel,
    ///F3 - Total deaths as a Result of Injury
    TotalDeathsAsAResultOfInjury,
    ///F4 - Total deaths as a Result of Illness
    TotalDeathsAsAResultOfIllness,
    ///F5 - Total injury Claims with Days Away from Work or Restricted Work Activity
    TotalInjuryClaimsWithDaysAwayFromWorkOrRestrictedWorkActivity,
    ///F6 - Total injury Claims with Days Away from Work
    TotalInjuryClaimsWithDaysAwayFromWork,
    ///F7 - Total injury Claims without Lost Work Days
    TotalInjuryClaimsWithoutLostWorkDays,
    ///F8 - Total Days Away from Work Due to Injury
    TotalDaysAwayFromWorkDueToInjury,
    ///F9 - Total Days with Restricted Work Activity Due to Injury
    TotalDaysWithRestrictedWorkActivityDueToInjury,
    ///FA - Full Baths
    FullBaths,
    ///FB - Furnished Blood Units
    FurnishedBloodUnits,
    ///FC - Fuel Consumed or Burned Amount
    FuelConsumedOrBurnedAmount,
    ///FD - Vehicular Radios
    VehicularRadios,
    ///FE - Portable Radios
    PortableRadios,
    ///FF - Flare or Flash
    FlareOrFlash,
    ///FG - Marine Radios
    MarineRadios,
    ///FH - Pagers
    Pagers,
    ///FI - Conventional Mobiles
    ConventionalMobiles,
    ///FJ - Trunked Channels
    TrunkedChannels,
    ///FK - Mobile Loading Allocation
    MobileLoadingAllocation,
    ///FL - Units
    Units,
    ///FM - Aircraft Radios
    AircraftRadios,
    ///FN - Total Claims with Dust Diseases of the Lungs
    TotalClaimsWithDustDiseasesOfTheLungs,
    ///FO - Total Claims with Respiratory Conditions Due to Toxic Agents
    TotalClaimsWithRespiratoryConditionsDueToToxicAgents,
    ///FP - Total Claims with Poisoning Illnesses
    TotalClaimsWithPoisoningIllnesses,
    ///FQ - Total Claims with Disorders Due to Physical Agents
    TotalClaimsWithDisordersDueToPhysicalAgents,
    ///FS - Gas Used for Fuel System
    GasUsedForFuelSystem,
    ///FT - Forecast to Complete
    ForecastToComplete,
    ///FU - Total Claims Associated with Repeated Trauma
    TotalClaimsAssociatedWithRepeatedTrauma,
    ///FV - Total illness Claims with occupational illnesses not otherwise classified
    TotalIllnessClaimsWithOccupationalIllnessesNotOtherwiseClassified,
    ///FW - Total Days Away from Work Due to Illness
    TotalDaysAwayFromWorkDueToIllness,
    ///FX - Total Days of Restricted Work Activity Due to Illness
    TotalDaysOfRestrictedWorkActivityDueToIllness,
    ///FY - Total illness with Lost Work Days or Restricted Work Activity
    TotalIllnessWithLostWorkDaysOrRestrictedWorkActivity,
    ///FZ - Total illness Claims with Days Away from Work
    TotalIllnessClaimsWithDaysAwayFromWork,
    ///G0 - Discharge Quantity
    DischargeQuantity,
    ///G1 - Estimated Discharge Quantity
    EstimatedDischargeQuantity,
    ///G2 - Estimated Transfer Quantity
    EstimatedTransferQuantity,
    ///G3 - Excursions
    Excursions,
    ///G4 - Non-production Quantity
    NonProductionQuantity,
    ///G5 - Number of Deaths
    NumberOfDeaths,
    ///G6 - Number of Hospitalizations
    NumberOfHospitalizations,
    ///G7 - Number of Injuries
    NumberOfInjuries,
    ///G8 - Number of Injuries Requiring Medical Treatment
    NumberOfInjuriesRequiringMedicalTreatment,
    ///G9 - Number of People Evacuated
    NumberOfPeopleEvacuated,
    ///GA - Gross Building Area
    GrossBuildingArea,
    ///GB - Gross Annual Income Multiplier
    GrossAnnualIncomeMultiplier,
    ///GC - Gross Living Area
    GrossLivingArea,
    ///GD - Total illness Claims without Lost Work Days
    TotalIllnessClaimsWithoutLostWorkDays,
    ///GE - Original Term In Years
    OriginalTermInYears,
    ///GF - Years Remaining
    YearsRemaining,
    ///GG - Average Number of Employees
    AverageNumberOfEmployees,
    ///GH - Total Worked by All Employees
    TotalWorkedByAllEmployees,
    ///GI - Gas Injection Volume
    GasInjectionVolume,
    ///GL - Gas Lift Volume
    GasLiftVolume,
    ///GM - Episode
    Episode,
    ///GN - Period(s)
    CodeGN,
    ///GO - Session(s)
    CodeGO,
    ///GP - Gross Production
    GrossProduction,
    ///GQ - Government Reporting Quantity
    GovernmentReportingQuantity,
    ///GR - Gas Receipt Volume
    GasReceiptVolume,
    ///GS - Gas Sold
    GasSold,
    ///GT - Grade Transfer Amount
    GradeTransferAmount,
    ///GU - Employee Total First Month of Quarter
    EmployeeTotalFirstMonthOfQuarter,
    ///GV - Gas Volume
    GasVolume,
    ///GW - Employee Total Second Month of Quarter
    EmployeeTotalSecondMonthOfQuarter,
    ///GX - Employee Total Third Month of Quarter
    EmployeeTotalThirdMonthOfQuarter,
    ///GZ - Active Listings
    ActiveListings,
    ///H0 - Number of People Sheltered-in-Place
    NumberOfPeopleShelteredInPlace,
    ///H1 - Quantity Recovered
    QuantityRecovered,
    ///H2 - Quantity Recycled
    QuantityRecycled,
    ///H3 - Quantity Released
    QuantityReleased,
    ///H4 - Quantity Treated
    QuantityTreated,
    ///H5 - Total Hazardous Waste Generated
    TotalHazardousWasteGenerated,
    ///H6 - Operational Quantity
    OperationalQuantity,
    ///H7 - Penalty Variance Quantity
    PenaltyVarianceQuantity,
    ///H8 - Allocated Quantity
    AllocatedQuantity,
    ///H9 - Scheduled Quantity
    ScheduledQuantity,
    ///HA - Market Price Change
    MarketPriceChange,
    ///HB - Unpaid
    Unpaid,
    ///HC - Branches
    Branches,
    ///HD - Subsidiaries
    Subsidiaries,
    ///HE - Age of Financial Information
    AgeOfFinancial,
    ///HF - Invoices
    Invoices,
    ///HG - Financial Coverage Period
    FinancialCoveragePeriod,
    ///HH - Maximum Number of Employees at Location
    MaximumNumberOfEmployeesAtLocation,
    ///HI - Previous Number of Accounts
    PreviousNumberOfAccounts,
    ///HJ - Collection Period
    CollectionPeriod,
    ///HK - Disbursement Period
    DisbursementPeriod,
    ///HL - Seats
    Seats,
    ///HM - Use of Hypothermia
    UseOfHypothermia,
    ///HN - Previous Number of Employees
    PreviousNumberOfEmployees,
    ///HO - Use of Hypotension
    UseOfHypotension,
    ///HP - Use of Hyperbaric Pressurization
    UseOfHyperbaricPressurization,
    ///HQ - Kindergarten Students
    KindergartenStudents,
    ///HR - Use of Hypertension
    UseOfHypertension,
    ///HS - Hours
    Hours,
    ///HT - Employee's Age
    EmployeesAge,
    ///HU - Employee's Number of Days Away from Work Due to Injury
    EmployeesNumberOfDaysAwayFromWorkDueToInjury,
    ///HV - Employee's Number of Days of Restricted Work Activity Due to Injury
    EmployeesNumberOfDaysOfRestrictedWorkActivityDueToInjury,
    ///HW - Employee's Total Number of Days Away from Work Due to Illness
    EmployeesTotalNumberOfDaysAwayFromWorkDueToIllness,
    ///HY - Total Death Claims
    TotalDeathClaims,
    ///HZ - Total Claims with Days Away from Work
    TotalClaimsWithDaysAwayFromWork,
    ///I0 - Tenth Grade Students
    TenthGradeStudents,
    ///I1 - Eleventh Grade Students
    EleventhGradeStudents,
    ///I2 - Twelfth Grade Students
    TwelfthGradeStudents,
    ///I3 - Prior Teaching Experience
    PriorTeachingExperience,
    ///I4 - Prior Full-time Teaching Experience
    PriorFullTimeTeachingExperience,
    ///I5 - Prior Part-time Teaching Experience
    PriorPartTimeTeachingExperience,
    ///I6 - Prior Experience in Education
    PriorExperienceInEducation,
    ///I7 - Prior Full-time Experience in Education
    PriorFullTimeExperienceInEducation,
    ///I8 - Prior Part-time Experience in Education
    PriorPartTimeExperienceInEducation,
    ///I9 - Prior Experience Related to Job
    PriorExperienceRelatedToJob,
    ///IA - Local Country Employees
    LocalCountryEmployees,
    ///IB - Foreign Employees
    ForeignEmployees,
    ///IC - Prior Full-time Experience Related to Job
    PriorFullTimeExperienceRelatedToJob,
    ///ID - Prior Part-time Experience Related to Job
    PriorPartTimeExperienceRelatedToJob,
    ///IE - Total Prior Experience
    TotalPriorExperience,
    ///IF - Total Full-time Prior Experience
    TotalFullTimePriorExperience,
    ///IG - Total Part-time Prior Experience
    TotalPartTimePriorExperience,
    ///IH - Total Years of Educational Service
    TotalYearsOfEducationalService,
    ///II - Number of Irregular Interest Payments
    NumberOfIrregularInterestPayments,
    ///IJ - Total Years of Educational Service in this District
    TotalYearsOfEducationalServiceInThisDistrict,
    ///IK - Years of Experience as School Principal
    YearsOfExperienceAsSchoolPrincipal,
    ///IL - Years of Experience as Classroom Teacher
    YearsOfExperienceAsClassroomTeacher,
    ///IM - Years Worked for this System
    YearsWorkedForThisSystem,
    ///IN - Indirect Workers
    IndirectWorkers,
    ///IP - Number of Interest Payments
    NumberOfInterestPayments,
    ///IQ - In-Transit Quantity
    InTransitQuantity,
    ///IS - Information Provider Standardized Motor Vehicle Penalty Points
    InformationProviderStandardizedMotorVehiclePenaltyPoints,
    ///IT - Intertank Transfer Amount
    IntertankTransferAmount,
    ///J0 - Ending Storage Balance
    EndingStorageBalance,
    ///J1 - Location Ending Storage Balance
    LocationEndingStorageBalance,
    ///J2 - Location Ending Storage Balance - Firm
    LocationEndingStorageBalanceFirm,
    ///J3 - Location Ending Storage Balance - Interruptible
    LocationEndingStorageBalanceInterruptible,
    ///J4 - Maximum Available Daily Injection Quantity
    MaximumAvailableDailyInjectionQuantity,
    ///J5 - Maximum Available Daily Withdrawal Quantity
    MaximumAvailableDailyWithdrawalQuantity,
    ///J6 - Minimum Required Daily Injection Quantity
    MinimumRequiredDailyInjectionQuantity,
    ///J7 - Minimum Required Daily Withdrawal Quantity
    MinimumRequiredDailyWithdrawalQuantity,
    ///JA - Activity Codes
    ActivityCodes,
    ///JB - Associates
    Associates,
    ///JC - Average Employees
    AverageEmployees,
    ///JD - Cooperative Shares
    CooperativeShares,
    ///JE - Estimated Employees at Location
    EstimatedEmployeesAtLocation,
    ///JF - Estimated Total Employees
    EstimatedTotalEmployees,
    ///JG - Financial Institutions
    FinancialInstitutions,
    ///JH - Judgments
    Judgments,
    ///JI - Land Size
    LandSize,
    ///JJ - Liens
    Liens,
    ///JK - Minimum Employees at Location
    MinimumEmployeesAtLocation,
    ///JL - Office Size
    OfficeSize,
    ///JM - Owner
    Owner,
    ///JN - Plant Size
    PlantSize,
    ///JO - Previous Number of Branches
    PreviousNumberOfBranches,
    ///JP - Protested Bills
    ProtestedBills,
    ///JQ - Suits
    Suits,
    ///JR - Uniform Commercial Code (UCC) Filings
    CodeJR,
    ///JS - Judicial Stay Duration
    JudicialStayDuration,
    ///JT - Warehouse Size
    WarehouseSize,
    ///JU - Total Days Away from Work
    TotalDaysAwayFromWork,
    ///JV - Total Days of Restricted Work Activity
    TotalDaysOfRestrictedWorkActivity,
    ///JW - Total Claims without Days Away from Work and without Restricted Work Activity
    TotalClaimsWithoutDaysAwayFromWorkAndWithoutRestrictedWorkActivity,
    ///JX - Secretaries
    Secretaries,
    ///JY - Mechanics
    Mechanics,
    ///JZ - Auditors
    Auditors,
    ///K1 - Messengers
    Messengers,
    ///K2 - Primary Managers
    PrimaryManagers,
    ///K3 - Participation Shares
    ParticipationShares,
    ///K4 - Detrimental Legal Filings
    DetrimentalLegalFilings,
    ///K5 - Petitions Filed
    PetitionsFiled,
    ///K6 - Drafts
    Drafts,
    ///K7 - Business Failure National Average Incidence
    BusinessFailureNationalAverageIncidence,
    ///K8 - Business Failure Industry Incidence
    BusinessFailureIndustryIncidence,
    ///K9 - Business Failure Class Incidence
    BusinessFailureClassIncidence,
    ///KA - Estimated
    Estimated,
    ///KB - Net Quantity Increase
    NetQuantityIncrease,
    ///KC - Net Quantity Decrease
    NetQuantityDecrease,
    ///KD - Expenditure Quantity
    ExpenditureQuantity,
    ///KE - Originals
    Originals,
    ///KF - Duplicates
    Duplicates,
    ///KG - Completed Line Items
    CompletedLineItems,
    ///KH - Completed Contracts
    CompletedContracts,
    ///KI - Active Contracts Delinquent-Buying Party Caused
    ActiveContractsDelinquentBuyingPartyCaused,
    ///KJ - Active Contracts Delinquent
    ActiveContractsDelinquent,
    ///KK - Active Contracts Delinquent-Contractor Caused
    ActiveContractsDelinquentContractorCaused,
    ///KL - Active Contracts Delinquent-Unknown Causes
    ActiveContractsDelinquentUnknownCauses,
    ///KM - Active Line Items Delinquent
    ActiveLineItemsDelinquent,
    ///KN - Active Line Items Delinquent-Buying Party Caused
    ActiveLineItemsDelinquentBuyingPartyCaused,
    ///KO - Active Line Items Delinquent-Contractor Caused
    ActiveLineItemsDelinquentContractorCaused,
    ///KP - Active Line Items Delinquent-Unknown Causes
    ActiveLineItemsDelinquentUnknownCauses,
    ///KQ - Contracts Completed Delinquent-Buying Party Caused
    ContractsCompletedDelinquentBuyingPartyCaused,
    ///KR - Contract Completed Delinquent-Contractor Caused
    ContractCompletedDelinquentContractorCaused,
    ///KS - Contracts Completed Delinquent-Unknown Causes
    ContractsCompletedDelinquentUnknownCauses,
    ///KU - Reported Deficiencies
    ReportedDeficiencies,
    ///KV - Line Items Completed Delinquent-Buying Party Caused
    LineItemsCompletedDelinquentBuyingPartyCaused,
    ///KW - Line Items Completed Delinquent-Contractor Caused
    LineItemsCompletedDelinquentContractorCaused,
    ///KX - Line Items Completed Delinquent-Unknown Causes
    LineItemsCompletedDelinquentUnknownCauses,
    ///KY - Corrective Action Requests-Verbal
    CorrectiveActionRequestsVerbal,
    ///KZ - Corrective Action Requests-Written
    CorrectiveActionRequestsWritten,
    ///L2 - Guarantee Fee Buyup Maximum
    GuaranteeFeeBuyupMaximum,
    ///L3 - Contract Buyup
    ContractBuyup,
    ///L4 - Contract Buydown
    ContractBuydown,
    ///L5 - Guarantee Fee Rate after Alternate Payment Method
    GuaranteeFeeRateAfterAlternatePaymentMethod,
    ///L6 - Guarantee Fee Rate after Buyup or Buydown
    GuaranteeFeeRateAfterBuyupOrBuydown,
    ///L7 - Buyup or Buydown Rate per Basis Point
    BuyupOrBuydownRatePerBasisPoint,
    ///L8 - Location Net Capacity
    LocationNetCapacity,
    ///L9 - Subject to loss or elimination
    SubjectToLossOrElimination,
    ///LA - Life-time Reserve - Actual
    LifeTimeReserveActual,
    ///LB - Loss Allowance
    LossAllowance,
    ///LC - Late Payment Period
    LatePaymentPeriod,
    ///LD - Limit Value
    LimitValue,
    ///LE - Life-time Reserve - Estimated
    LifeTimeReserveEstimated,
    ///LG - Loss or Gain
    LossOrGain,
    ///LH - Lost Gas
    LostGas,
    ///LI - Liquid Injection Volume
    LiquidInjectionVolume,
    ///LK - Corrective Action Requests-Method C
    CorrectiveActionRequestsMethodC,
    ///LL - Corrective Action Requests-Method D
    CorrectiveActionRequestsMethodD,
    ///LM - Corrective Action Requests-Method E
    CorrectiveActionRequestsMethodE,
    ///LN - Aged Active Line Items Delinquent-Contractor Caused
    AgedActiveLineItemsDelinquentContractorCaused,
    ///LO - Lost Oil
    LostOil,
    ///LP - Lease Periods
    LeasePeriods,
    ///LQ - Aged Line Items Delinquent
    AgedLineItemsDelinquent,
    ///LR - Aged Line Items Completed-Contractor Caused
    AgedLineItemsCompletedContractorCaused,
    ///LS - Oil Condensate Sold
    OilCondensateSold,
    ///LT - Tariff Loss Allowance
    TariffLossAllowance,
    ///LU - Lifetime Reserve Days - Applied to this Claim
    LifetimeReserveDaysAppliedToThisClaim,
    ///LV - Oil/Condensate Volume
    OilCondensateVolume,
    ///LW - Lost Work Time Actual
    LostWorkTimeActual,
    ///LX - Lost Work Time Estimated
    LostWorkTimeEstimated,
    ///LY - Length of Residency
    LengthOfResidency,
    ///LZ - Lanes
    Lanes,
    ///M1 - Matching Equipment
    MatchingEquipment,
    ///M2 - Maximum
    Maximum,
    ///M3 - Total Federal Points
    TotalFederalPoints,
    ///M4 - Contributions
    Contributions,
    ///M5 - Contributors
    Contributors,
    ///M6 - Endorsers
    Endorsers,
    ///M7 - Functions
    Functions,
    ///M8 - Guarantors
    Guarantors,
    ///M9 - Points
    Points,
    ///MA - Miscellaneous Allowance
    MiscellaneousAllowance,
    ///MB - Number of Public Officials
    NumberOfPublicOfficials,
    ///MC - Total Non-Federal Points
    TotalNonFederalPoints,
    ///MD - Million Dollar Roundtable Credits
    MillionDollarRoundtableCredits,
    ///ME - Minimum Number of Employees
    MinimumNumberOfEmployees,
    ///MF - Manufactured
    Manufactured,
    ///MG - Pledges
    Pledges,
    ///MH - Total Points
    TotalPoints,
    ///MI - Miles
    Miles,
    ///MJ - Attendees
    Attendees,
    ///MK - Tickets Sold
    TicketsSold,
    ///ML - Total Number of Manifest Lines
    TotalNumberOfManifestLines,
    ///MM - Maximum Maturity Extension
    MaximumMaturityExtension,
    ///MN - Month
    Month,
    ///MO - Minimum Order Package Level
    MinimumOrderPackageLevel,
    ///MP - Total Number of Maps in a Pack
    TotalNumberOfMapsInAPack,
    ///MQ - Maximum Ship Quantity
    MaximumShipQuantity,
    ///MR, NX - Quantity of next lower level trade item
    QuantityOfNextLowerLevelTradeItem,
    ///MS - Measured Quantity
    MeasuredQuantity,
    ///MT - Resterilization Maximum
    ResterilizationMaximum,
    ///MU - Recommended Number of Uses
    RecommendedNumberOfUses,
    ///MV - Total Units
    TotalUnits,
    ///MX - Maximum Number of Employees
    MaximumNumberOfEmployees,
    ///MY - Stacking Factor
    StackingFactor,
    ///MZ - Component Quantity
    ComponentQuantity,
    ///N1 - Number of Attacks or Occurrences
    NumberOfAttacksOrOccurrences,
    ///N2 - Number of Dead
    NumberOfDead,
    ///N3 - Number of Living
    NumberOfLiving,
    ///N4 - Number of Times
    NumberOfTimes,
    ///N5 - Minimum Forecast Quantity
    MinimumForecastQuantity,
    ///N6 - Maximum Forecast Quantity
    MaximumForecastQuantity,
    ///N7 - Requested Receipt Quantity
    RequestedReceiptQuantity,
    ///N8 - Requested Delivery Quantity
    RequestedDeliveryQuantity,
    ///NA - Number of Non-covered Days
    NumberOfNonCoveredDays,
    ///NB - Number of Units (Housing)
    CodeNB,
    ///NC - Number of Claimants
    NumberOfClaimants,
    ///ND - Number of Late Charges
    NumberOfLateCharges,
    ///NE - Non-Covered - Estimated
    NonCoveredEstimated,
    ///NF - Number of Full-Time Employees
    NumberOfFullTimeEmployees,
    ///NG - Number of Nonsufficient Fund Items
    NumberOfNonsufficientFundItems,
    ///NH - Noncovered - Actual
    NoncoveredActual,
    ///NL - Number of Levels
    NumberOfLevels,
    ///NN - Number of Hospitals
    NumberOfHospitals,
    ///NO - Number of Physicians
    NumberOfPhysicians,
    ///NP - Number of Members
    NumberOfMembers,
    ///NQ - Number of Franchisees
    NumberOfFranchisees,
    ///NR - Not Replaced Blood Units
    NotReplacedBloodUnits,
    ///NS - Number of Stations
    NumberOfStations,
    ///NT - Reports
    Reports,
    ///NU - Since Last Travel
    SinceLastTravel,
    ///NV - Net
    Net,
    ///NW - Until Next Travel
    UntilNextTravel,
    ///O1 - Scheduled Receipt
    ScheduledReceipt,
    ///O2 - Scheduled Delivery
    ScheduledDelivery,
    ///O3 - Operational Receipt
    OperationalReceipt,
    ///O4 - Operational Delivery
    OperationalDelivery,
    ///O5 - Allocated Receipt
    AllocatedReceipt,
    ///O6 - Allocated Delivery
    AllocatedDelivery,
    ///O7 - Distributed Confirmed Receipt
    DistributedConfirmedReceipt,
    ///O8 - Distributed Confirmed Delivery
    DistributedConfirmedDelivery,
    ///O9 - Scheduling Tolerance Receipt
    SchedulingToleranceReceipt,
    ///OA - Scheduling Tolerance Delivery
    SchedulingToleranceDelivery,
    ///OB - Energy
    Energy,
    ///OC - Order Count
    OrderCount,
    ///OD - Other Miscellaneous Disposition
    OtherMiscellaneousDisposition,
    ///OE - Number of Weeks Per Year
    NumberOfWeeksPerYear,
    ///OF - Off Premise Sales Quantity
    OffPremiseSalesQuantity,
    ///OG - Other Gas Disposition
    OtherGasDisposition,
    ///OH - Other Injection Volume
    OtherInjectionVolume,
    ///OI - Opening Statement Balance
    OpeningStatementBalance,
    ///OJ - Order Sizing Factor
    OrderSizingFactor,
    ///OL - Original Loan Term
    OriginalLoanTerm,
    ///ON - On Premise Sales Quantity
    OnPremiseSalesQuantity,
    ///OO - Other Oil Condensate Disposition
    OtherOilCondensateDisposition,
    ///OQ - Optimum Order Quantity
    OptimumOrderQuantity,
    ///OR - Original
    Original,
    ///OT - Number of Operating Periods at Failure
    NumberOfOperatingPeriodsAtFailure,
    ///OU - Outlier Days
    OutlierDays,
    ///OV - Overage
    Overage,
    ///OW - Other Water Disposition
    OtherWaterDisposition,
    ///P1 - Project Phases
    ProjectPhases,
    ///P3 - Physical Status III
    PhysicalStatusIii,
    ///P4 - Physical Status IV
    PhysicalStatusIv,
    ///P5 - Physical Status V
    PhysicalStatusV,
    ///P6 - Number of Services or Procedures
    NumberOfServicesOrProcedures,
    ///P7 - Prescription Dosage
    PrescriptionDosage,
    ///P8 - Prescription Frequency
    PrescriptionFrequency,
    ///P9 - Number of People Living at Residence
    NumberOfPeopleLivingAtResidence,
    ///PA - Pipeline Adjustment or Allowance
    PipelineAdjustmentOrAllowance,
    ///PB - Pressure Base
    PressureBase,
    ///PC - Prior Cumulative Imbalance
    PriorCumulativeImbalance,
    ///PD - Payment Duration Weeks
    PaymentDurationWeeks,
    ///PE - Period of Employment
    PeriodOfEmployment,
    ///PF - Gas Used for Plant Fuel
    GasUsedForPlantFuel,
    ///PG - Persistency
    Persistency,
    ///PH - Promotional
    Promotional,
    ///PK - Parking Spaces
    ParkingSpaces,
    ///PL - Partial Baths
    PartialBaths,
    ///PO - Percentage of Ordered Quantity
    PercentageOfOrderedQuantity,
    ///PP - Purchase of Product
    PurchaseOfProduct,
    ///PQ - Cumulative Quantity Required Prior to the First Scheduled Period
    CumulativeQuantityRequiredPriorToTheFirstScheduledPeriod,
    ///PR - Requirement Quantity that was Previously Released
    RequirementQuantityThatWasPreviouslyReleased,
    ///PS - Prescription
    Prescription,
    ///PT - Patients
    Patients,
    ///PW - Pitted Water
    PittedWater,
    ///PX - Prior Units Accepted
    PriorUnitsAccepted,
    ///PY - Paid
    Paid,
    ///Q1 - Minimum quantity to which tax rate applies
    MinimumQuantityToWhichTaxRateApplies,
    ///Q2 - Maximum quantity to which tax rate applies
    MaximumQuantityToWhichTaxRateApplies,
    ///Q3 - Quantity Earned
    QuantityEarned,
    ///Q4 - Quantity Carried Forward
    QuantityCarriedForward,
    ///Q5 - Number of 3 to 4 Year Olds
    NumberOf3To4YearOlds,
    ///Q6 - Autistic Individuals
    AutisticIndividuals,
    ///Q7 - Deaf-blind Individuals
    DeafBlindIndividuals,
    ///Q8 - Hearing Impaired Individuals
    HearingImpairedIndividuals,
    ///Q9 - Mentally Retarded Individuals
    MentallyRetardedIndividuals,
    ///QA - Quantity Approved
    QuantityApproved,
    ///QB - Quantity Dispensed
    QuantityDispensed,
    ///QC - Quantity Disapproved
    QuantityDisapproved,
    ///QD - Quantity Delivered
    QuantityDelivered,
    ///QE - Quantity Deferred
    QuantityDeferred,
    ///QF - High Fabrication Authorization Quantity
    HighFabricationAuthorizationQuantity,
    ///QH - Quantity on Hold
    QuantityOnHold,
    ///QI - Community Service Duration
    CommunityServiceDuration,
    ///QJ - Number of Times Deported
    NumberOfTimesDeported,
    ///QK - Quantity of Inner Packs
    QuantityOfInnerPacks,
    ///QL - Jail Sentence Duration
    JailSentenceDuration,
    ///QM - Probation Duration
    ProbationDuration,
    ///QN - Restriction Duration
    RestrictionDuration,
    ///QO - Operating Quantity
    OperatingQuantity,
    ///QP - Quantity by Position
    QuantityByPosition,
    ///QQ - Suspended Duration
    SuspendedDuration,
    ///QR - High Raw Material Authorization Quantity
    HighRawMaterialAuthorizationQuantity,
    ///QS - Quantity Per Skid
    QuantityPerSkid,
    ///QT - Plant Thermal Reduction
    PlantThermalReduction,
    ///QU - Quantity Serviced
    QuantityServiced,
    ///QV - Quantity Cancelled
    QuantityCancelled,
    ///QW - Quantity Withdrawn
    QuantityWithdrawn,
    ///QX - Qualifying Weeks
    QualifyingWeeks,
    ///QY - Repayment Plan Term
    RepaymentPlanTerm,
    ///R1 - Replenishment (Fill)
    CodeR1,
    ///R2 - Individuals with Orthopedic Impairment
    IndividualsWithOrthopedicImpairment,
    ///R3 - Estimated Remaining Physical Life
    EstimatedRemainingPhysicalLife,
    ///R4 - Individuals with Specific Learning Disability
    IndividualsWithSpecificLearningDisability,
    ///R5 - Axles
    Axles,
    ///R6 - Platform Count
    PlatformCount,
    ///R7 - Individuals with Visual Impairment
    IndividualsWithVisualImpairment,
    ///R8 - Individuals with Other Health Impairment
    IndividualsWithOtherHealthImpairment,
    ///R9 - Fuel
    Fuel,
    ///RA - Refills Authorized
    RefillsAuthorized,
    ///RB - Replaced Blood Units
    ReplacedBloodUnits,
    ///RC - Number of Items Authorized at Store
    NumberOfItemsAuthorizedAtStore,
    ///RD - Number of Items Authorized at Warehouse
    NumberOfItemsAuthorizedAtWarehouse,
    ///RE - Gas Returned to Earth
    GasReturnedToEarth,
    ///RF - Number of Items in Stock
    NumberOfItemsInStock,
    ///RG - Gas Used for Repressuring or Pressure Maintenance
    GasUsedForRepressuringOrPressureMaintenance,
    ///RH - Number of Shelf Tags
    NumberOfShelfTags,
    ///RJ - Quantity Available on Shelf
    QuantityAvailableOnShelf,
    ///RL - Gas Returned to Property for fuel
    GasReturnedToPropertyForFuel,
    ///RM - Room Count
    RoomCount,
    ///RN - Units Rented
    UnitsRented,
    ///RP - Retail Demand Quantity
    RetailDemandQuantity,
    ///RQ - Royalty
    Royalty,
    ///RS - Number of Shelf Facings
    NumberOfShelfFacings,
    ///RT - Retail Sales Quantity
    RetailSalesQuantity,
    ///RW - Water Re-injected on Property
    WaterReInjectedOnProperty,
    ///RY - Requirement Quantity
    RequirementQuantity,
    ///S1 - Planned Unit Development (PUD) Units
    CodeS1,
    ///S2 - Rooms, Finished Area Above Grade
    CodeS2,
    ///S3 - Dwelling Area
    DwellingArea,
    ///S4 - Garage or Carport Area
    GarageOrCarportArea,
    ///S5 - Units for Sale
    UnitsForSale,
    ///S6 - Gross Rent Multiplier
    GrossRentMultiplier,
    ///S7 - Age, High Value
    CodeS7,
    ///S8 - Age, Low Value
    CodeS8,
    ///S9 - Bedrooms, Finished Area Above Grade
    CodeS9,
    ///SA - Shipments
    Shipments,
    ///SB - Solicited
    Solicited,
    ///SC - Bathrooms, Finished Area Above Grade
    CodeSC,
    ///SD - Criminal Sentence Duration
    CriminalSentenceDuration,
    ///SE - Gross Living, Finished Area Above Grade
    CodeSE,
    ///SF - Site
    Site,
    ///SG - Swan-Ganz
    SwanGanz,
    ///SH - Shortage
    Shortage,
    ///SI - Rooms
    Rooms,
    ///SJ - Area of Level
    AreaOfLevel,
    ///SK - Gas Shrinkage
    GasShrinkage,
    ///SL - Predominate Age
    PredominateAge,
    ///SM - Minimum Criminal Sentence Duration
    MinimumCriminalSentenceDuration,
    ///SN - Age
    Age,
    ///SO - Oil Sedimentation
    OilSedimentation,
    ///SP - Days Supply
    DaysSupply,
    ///SQ - Product Sales Amount
    ProductSalesAmount,
    ///SR - Effective Age
    EffectiveAge,
    ///SS - Shares of Preferred Stock
    SharesOfPreferredStock,
    ///ST - Standard
    Standard,
    ///SU - Forecasted Scanned Quantity
    ForecastedScannedQuantity,
    ///SV - Shares of Common Stock
    SharesOfCommonStock,
    ///SW - Sample Amount
    SampleAmount,
    ///SX - Maximum Criminal Sentence Duration
    MaximumCriminalSentenceDuration,
    ///SY - State or Province Motor Vehicle Penalty Points
    StateOrProvinceMotorVehiclePenaltyPoints,
    ///SZ - Seasonal
    Seasonal,
    ///T1 - Time Units Known
    TimeUnitsKnown,
    ///T2 - Time Units Spent on Duty
    TimeUnitsSpentOnDuty,
    ///T3 - Total Days on Market
    TotalDaysOnMarket,
    ///T4 - Total Rooms
    TotalRooms,
    ///T5 - Total Number of Units
    TotalNumberOfUnits,
    ///T6 - Total Number of Units for Sale
    TotalNumberOfUnitsForSale,
    ///T7 - Tires
    Tires,
    ///TA - Tank Allowance
    TankAllowance,
    ///TB - Oil Theft
    OilTheft,
    ///TC - Total at Complete
    TotalAtComplete,
    ///TD - Total to Date
    TotalToDate,
    ///TE - Number of Theatres
    NumberOfTheatres,
    ///TG - Total Gas Injection Volume
    TotalGasInjectionVolume,
    ///TH - Theoretical Quantity
    TheoreticalQuantity,
    ///TI - Total Oil and/or Condensate Injection Volume
    TotalOilAndOrCondensateInjectionVolume,
    ///TJ - Duration in Current Job
    DurationInCurrentJob,
    ///TK - Total Oil and/or Condensate Disposition
    TotalOilAndOrCondensateDisposition,
    ///TM - Total Water Disposition
    TotalWaterDisposition,
    ///TN - Total Beginning Inventory
    TotalBeginningInventory,
    ///TO - Total
    Total,
    ///TP - Time in Position
    TimeInPosition,
    ///TQ - Total Quantity of All Buys
    TotalQuantityOfAllBuys,
    ///TR - Trips
    Trips,
    ///TS - Total Number of Parking Spaces
    TotalNumberOfParkingSpaces,
    ///TT - Total Production Volume
    TotalProductionVolume,
    ///TU - Total Adjustments Volume
    TotalAdjustmentsVolume,
    ///TV - Total Gas Disposition
    TotalGasDisposition,
    ///TW - Total Water Injection Volume
    TotalWaterInjectionVolume,
    ///TX - Total Ending Inventory
    TotalEndingInventory,
    ///TY - Total Sales Volume
    TotalSalesVolume,
    ///U1 - Freelance Collectors
    FreelanceCollectors,
    ///U2 - Branch Locations Owned
    BranchLocationsOwned,
    ///U3 - Branch Locations Leased
    BranchLocationsLeased,
    ///UA - Units Completed
    UnitsCompleted,
    ///UB - Poultry
    Poultry,
    ///UC - Livestock
    Livestock,
    ///UD - Passengers
    Passengers,
    ///UE - Trainers
    Trainers,
    ///UF - Operators
    Operators,
    ///UG - Gas Used on Property
    GasUsedOnProperty,
    ///UH - Inspectors
    Inspectors,
    ///UI - Collectors
    Collectors,
    ///UJ - Professionals
    Professionals,
    ///UK - Supervisors
    Supervisors,
    ///UL - Approximate Number of Units for Sale Projected
    ApproximateNumberOfUnitsForSaleProjected,
    ///UM - Administrators
    Administrators,
    ///UN - Promoters
    Promoters,
    ///UO - Oil Condensate Used on Property
    OilCondensateUsedOnProperty,
    ///UP - Divisions
    Divisions,
    ///UQ - Tables
    Tables,
    ///UR - Fuel Pumps
    FuelPumps,
    ///US - In-Use
    InUse,
    ///UT - Machines
    Machines,
    ///UU - Used
    Used,
    ///UV - Trademarks Used
    TrademarksUsed,
    ///UW - Available for Cultivation
    AvailableForCultivation,
    ///UX - Foremen
    Foremen,
    ///UY - Travelling Employees
    TravellingEmployees,
    ///UZ - Freelance Salespersons
    FreelanceSalespersons,
    ///V1 - Retention Quantity
    RetentionQuantity,
    ///V2 - Available Quantity
    AvailableQuantity,
    ///V3 - Transfer Quantity
    TransferQuantity,
    ///V4 - Surveys in Average Rating
    SurveysInAverageRating,
    ///V5 - Vacancies
    Vacancies,
    ///V8 - Unsubscribed Capacity
    UnsubscribedCapacity,
    ///V9 - Shipping Container Quantity
    ShippingContainerQuantity,
    ///VA - Volume Shrinkage Adjustment or Allowance
    VolumeShrinkageAdjustmentOrAllowance,
    ///VB - Blank Votes
    BlankVotes,
    ///VC - Cumulative Earned Value
    CumulativeEarnedValue,
    ///VD - Scattered Votes
    ScatteredVotes,
    ///VE - Earned Value
    EarnedValue,
    ///VF - Federal Votes
    FederalVotes,
    ///VG - Gas Vented
    GasVented,
    ///VH - Schedule Variance
    ScheduleVariance,
    ///VI - Cumulative Schedule Variance
    CumulativeScheduleVariance,
    ///VJ - Cumulative Variance
    CumulativeVariance,
    ///VK - Estimate at Complete
    EstimateAtComplete,
    ///VL - At Complete Variance
    AtCompleteVariance,
    ///VM - Variance Adjustment
    VarianceAdjustment,
    ///VN - No Votes
    NoVotes,
    ///VP - Presidential Votes
    PresidentialVotes,
    ///VQ - Utilization Service Life
    UtilizationServiceLife,
    ///VR - Variance
    Variance,
    ///VS - Visits
    Visits,
    ///VT - Votes
    Votes,
    ///VU - Recommended Service Life
    RecommendedServiceLife,
    ///VV - Void Votes
    VoidVotes,
    ///VW - Shelf Life Period
    ShelfLifePeriod,
    ///VY - Yes Votes
    YesVotes,
    ///W0 - Bankruptcy Petitions
    BankruptcyPetitions,
    ///W1 - Buyers
    Buyers,
    ///W2 - Debentures
    Debentures,
    ///W3 - Debentures Filed against Directors
    DebenturesFiledAgainstDirectors,
    ///W4 - Detrimental Legal Filings against Directors
    DetrimentalLegalFilingsAgainstDirectors,
    ///W5 - Failed Businesses of Directors
    FailedBusinessesOfDirectors,
    ///W6 - Professors
    Professors,
    ///W7 - Sellers
    Sellers,
    ///W8 - Skilled Workers
    SkilledWorkers,
    ///W9 - Trademarks Represented
    TrademarksRepresented,
    ///WA - Total number of Workers' Compensation First Reports
    TotalNumberOfWorkersCompensationFirstReports,
    ///WB - Total number of Workers' Compensation Subsequent Reports
    TotalNumberOfWorkersCompensationSubsequentReports,
    ///WC - Total number of Workers' Compensation Combined Reports
    TotalNumberOfWorkersCompensationCombinedReports,
    ///WD - Units Worked per Day
    UnitsWorkedPerDay,
    ///WE - Limited Quantity
    LimitedQuantity,
    ///WG - Weight Gain
    WeightGain,
    ///WL - Weight Loss
    WeightLoss,
    ///WO - Operator's Working Interest
    OperatorsWorkingInterest,
    ///WP - Number of Producing Wells Remaining on Property or Facility
    NumberOfProducingWellsRemainingOnPropertyOrFacility,
    ///WR - Number of Producing Wells Remaining on Royalty Account
    NumberOfProducingWellsRemainingOnRoyaltyAccount,
    ///WT - Total Working Interest
    TotalWorkingInterest,
    ///WV - Water Volume
    WaterVolume,
    ///WW - Weeks Worked
    WeeksWorked,
    ///WX - License Withdrawal Duration
    LicenseWithdrawalDuration,
    ///WY - License Withdrawals Sent
    LicenseWithdrawalsSent,
    ///X1 - Producing Wells
    ProducingWells,
    ///X2 - Gross
    Gross,
    ///X6 - Assessment Hours
    AssessmentHours,
    ///X7 - Duty Days
    DutyDays,
    ///X8 - Contract Days
    ContractDays,
    ///X9 - Number of Days Employed
    NumberOfDaysEmployed,
    ///XA - Total of Issuable Assets
    TotalOfIssuableAssets,
    ///XB - Total System Backorder Quantity, High Priority
    CodeXB,
    ///XC - Total Service Backorder Quantity, High Priority
    CodeXC,
    ///XD - Total System Backorder Quantity, Low Priority
    CodeXD,
    ///XE - Total Service Backorder Quantity, Low Priority
    CodeXE,
    ///XG - On Hand and Due-In
    OnHandAndDueIn,
    ///XI - Installment Payments
    InstallmentPayments,
    ///XJ - Other War Reserve Material Requirements Protectable (OWRMRP) Quantity
    CodeXJ,
    ///XL - Approximate Number of Units Projected
    ApproximateNumberOfUnitsProjected,
    ///XN - Approximate Number of Holders
    ApproximateNumberOfHolders,
    ///XO - Circulating Oil
    CirculatingOil,
    ///XR - Stock Objective and Insurance Quantity
    StockObjectiveAndInsuranceQuantity,
    ///XT - Protected Quantity
    ProtectedQuantity,
    ///XU - Reserved
    Reserved,
    ///XV - Requisitioning Objective
    RequisitioningObjective,
    ///XX - Authorized Retention Level
    AuthorizedRetentionLevel,
    ///XY - Safety Level
    SafetyLevel,
    ///XZ - Backorder Lines
    BackorderLines,
    ///Y1 - Number of Lost Cards
    NumberOfLostCards,
    ///Y2 - Number of Stolen Cards
    NumberOfStolenCards,
    ///Y3 - Number of Cards not Received
    NumberOfCardsNotReceived,
    ///Y4 - Number of Active Accounts This Cycle
    NumberOfActiveAccountsThisCycle,
    ///Y5 - Number of Open Accounts
    NumberOfOpenAccounts,
    ///Y6 - Number of Accounts Past Due
    NumberOfAccountsPastDue,
    ///Y7 - Number of Cards Outstanding
    NumberOfCardsOutstanding,
    ///Y8 - On Hand plus Pipeline
    OnHandPlusPipeline,
    ///YA - Total Demand Quantity
    TotalDemandQuantity,
    ///YB - Total Demand Orders
    TotalDemandOrders,
    ///YC - First Quarter Recurring Demand
    FirstQuarterRecurringDemand,
    ///YD - First Quarter Recurring Orders
    FirstQuarterRecurringOrders,
    ///YE - First Quarter Non-recurring Demand
    FirstQuarterNonRecurringDemand,
    ///YF - First Quarter Non-recurring Orders
    FirstQuarterNonRecurringOrders,
    ///YG - Second Quarter Recurring Demand
    SecondQuarterRecurringDemand,
    ///YH - Second Quarter Recurring Orders
    SecondQuarterRecurringOrders,
    ///YJ - Second Quarter Non-recurring Demand
    SecondQuarterNonRecurringDemand,
    ///YK - Second Quarter Non-recurring Orders
    SecondQuarterNonRecurringOrders,
    ///YL - Third Quarter Recurring Demand
    ThirdQuarterRecurringDemand,
    ///YM - Third Quarter Recurring Orders
    ThirdQuarterRecurringOrders,
    ///YN - Third Quarter Non-recurring Demand
    ThirdQuarterNonRecurringDemand,
    ///YP - Third Quarter Non-recurring Orders
    ThirdQuarterNonRecurringOrders,
    ///YQ - Fourth Quarter Recurring Demand
    FourthQuarterRecurringDemand,
    ///YR - Fourth Quarter Recurring Orders
    FourthQuarterRecurringOrders,
    ///YS - Fourth Quarter Non-recurring Demand
    FourthQuarterNonRecurringDemand,
    ///YT - Fourth Quarter Non-recurring Orders
    FourthQuarterNonRecurringOrders,
    ///YU - Trailers
    Trailers,
    ///YW - Reorder Point Quantity
    ReorderPointQuantity,
    ///YX - Contract Line Item Quantity
    ContractLineItemQuantity,
    ///YY - Years
    Years,
    ///YZ - Maximum Quantity of Free Service Calls
    MaximumQuantityOfFreeServiceCalls,
    ///Z1 - Units Worked Last Day
    UnitsWorkedLastDay,
    ///Z2 - Units Worked per Week
    UnitsWorkedPerWeek,
    ///Z3 - Units Worked per Quarter
    UnitsWorkedPerQuarter,
    ///Z4 - Number Weeks Paid
    NumberWeeksPaid,
    ///Z6 - Unused Accumulated Sick Days
    UnusedAccumulatedSickDays,
    ///Z7 - Delivery Point Reduction Quantity
    DeliveryPointReductionQuantity,
    ///Z8 - Receipt Point Reduction Quantity
    ReceiptPointReductionQuantity,
    ///Z9 - Reduction Quantity
    ReductionQuantity,
    ///ZA - Federal Medicare or Medicaid Claim Mandate - Category 1
    FederalMedicareOrMedicaidClaimMandateCategory1,
    ///ZB - Federal Medicare or Medicaid Claim Mandate - Category 2
    FederalMedicareOrMedicaidClaimMandateCategory2,
    ///ZC - Federal Medicare or Medicaid Claim Mandate - Category 3
    FederalMedicareOrMedicaidClaimMandateCategory3,
    ///ZD - Federal Medicare or Medicaid Claim Mandate - Category 4
    FederalMedicareOrMedicaidClaimMandateCategory4,
    ///ZE - Federal Medicare or Medicaid Claim Mandate - Category 5
    FederalMedicareOrMedicaidClaimMandateCategory5,
    ///ZF - Federal Pension Mandate - Category 1
    FederalPensionMandateCategory1,
    ///ZG - Federal Pension Mandate - Category 2
    FederalPensionMandateCategory2,
    ///ZH - Federal Pension Mandate - Category 3
    FederalPensionMandateCategory3,
    ///ZI - Holding Period
    HoldingPeriod,
    ///ZJ - Federal Pension Mandate - Category 5
    FederalPensionMandateCategory5,
    ///ZK - Federal Medicare or Medicaid Payment Mandate - Category 1
    FederalMedicareOrMedicaidPaymentMandateCategory1,
    ///ZL - Federal Medicare or Medicaid Payment Mandate - Category 2
    FederalMedicareOrMedicaidPaymentMandateCategory2,
    ///ZM - Federal Medicare or Medicaid Payment Mandate - Category 3
    FederalMedicareOrMedicaidPaymentMandateCategory3,
    ///ZN - Federal Medicare or Medicaid Payment Mandate - Category 4
    FederalMedicareOrMedicaidPaymentMandateCategory4,
    ///ZO - Federal Medicare or Medicaid Payment Mandate - Category 5
    FederalMedicareOrMedicaidPaymentMandateCategory5,
    ///ZP - Federal Pension Mandate - Category 4
    FederalPensionMandateCategory4,
    ///ZQ - Shares Added
    SharesAdded,
    ///ZR - Extended Term
    ExtendedTerm,
    ///ZS - Amortization Term
    AmortizationTerm,
    ///ZT - Beginning Shares
    BeginningShares,
    ///ZU - Shares Deleted
    SharesDeleted,
    ///ZV - Quantity of Dealer License Plates
    QuantityOfDealerLicensePlates,
    ///ZW - Current Share Balance
    CurrentShareBalance,
    ///ZX - Size of Household
    SizeOfHousehold,
    ///ZY - Project Units Sold
    ProjectUnitsSold,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl QuantityQualifier {
    pub fn code(&self) -> &str {
        {
            use QuantityQualifier::*;
            match self {
                HospitalHomeboundIndividuals => "00",
                NumberOfHoursPerDay => "0A",
                NumberOfHoursPerWeek => "0B",
                NumberOfMonthsPerYear => "0C",
                NumberOfPeriodsPerWeek => "0D",
                ExpectedExpenditureQuantity => "0E",
                NumberOfHoursPerYear => "0F",
                PreKindergartenStudents => "0G",
                FirstGradeStudents => "0H",
                SecondGradeStudents => "0I",
                ThirdGradeStudents => "0J",
                FourthGradeStudents => "0K",
                FifthGradeStudents => "0L",
                SixthGradeStudents => "0M",
                SeventhGradeStudents => "0N",
                EighthGradeStudents => "0O",
                NinthGradeStudents => "0P",
                CarnegieUnits => "0Q",
                NumberOfDisabilityTypes => "0R",
                NumberOfMales => "0S",
                NumberOfFemales => "0T",
                IndividualsWithMultipleDisabilities => "0U",
                IndividualsWithSeriousEmotionalDisturbance => "0V",
                IndividualsWithSpeechOrLanguageImpairment => "0W",
                IndividualsWithTraumaticBrainInjury => "0X",
                BlindIndividuals => "0Y",
                DeafIndividuals => "0Z",
                DiscreteQuantity => "01",
                Code1A => "1A",
                Code1B => "1B",
                Code1C => "1C",
                Code1D => "1D",
                Code1E => "1E",
                Code1F => "1F",
                Code1G => "1G",
                Started => "1H",
                Completed => "1I",
                Due => "1J",
                TimeUnits => "1K",
                Shifts => "1L",
                TimeUnitsPerShift => "1M",
                ScrapAllowed => "1N",
                CalendarUnits => "1O",
                Code1P => "1P",
                Code1Q => "1Q",
                Code1R => "1R",
                Late => "1S",
                NumberOfDelinquentInstallments => "1T",
                NumberOfLoans => "1U",
                TotalNumberOfMortgagees => "1V",
                TotalNumberOfLoanDetailRecords => "1W",
                PrescriptionEffectivePeriod => "1X",
                Code1Y => "1Y",
                EndOfMonthInventoryPriorToShip => "1Z",
                CumulativeQuantity => "02",
                CommitmentPeriod => "2A",
                NumberOfBorrowers => "2B",
                NumberOfAdjustmentPeriods => "2C",
                AgeNearest => "2D",
                TotalOtherPropertiesOwnedAndFinanced => "2E",
                AgeNext => "2F",
                ReconsiderationPeriod => "2G",
                FlatExtraPremium => "2H",
                Co2InjectionVolume => "2I",
                AccountsPlacedForCollection => "2J",
                Changes => "2K",
                CompaniesInSameActivityForAPeriod => "2L",
                ComparisonPeriod => "2M",
                Departments => "2N",
                EmployeesShared => "2O",
                EstimatedAccounts => "2P",
                InstalledCapacity => "2Q",
                LevelsOccupied => "2R",
                RegisteredBrandsDistributed => "2S",
                ElectronicSignatures => "2T",
                Bytes => "2U",
                EmployedAtThisLocation => "2V",
                Segments => "2W",
                RegisteredBrandsManufactured => "2X",
                FunctionalGroups => "2Y",
                TransactionSets => "2Z",
                DiscreetQuantityRejectedMaterial => "03",
                TotalCreditsAccepted => "3A",
                TotalCreditsRejected => "3B",
                TotalDebitsAccepted => "3C",
                TotalDebitsRejected => "3D",
                TotalPaymentsRejected => "3E",
                TotalPreAdvicesAccepted => "3F",
                TotalPreAdvicesRejected => "3G",
                TotalPrenotesAccepted => "3H",
                TotalPrenotesRejected => "3I",
                TotalPostAdvicesAccepted => "3J",
                TotalPostAdvicesRejected => "3K",
                TotalUnidentifiedTransactionsRejected => "3L",
                TotalCreditsReceived => "3M",
                TotalDebitsReceived => "3N",
                IndividualsWithNoncategoricalPreschoolDisability => "3O",
                TotalPreAdvicesReceived => "3P",
                TotalPrenotesReceived => "3Q",
                TotalPostAdvicesReceived => "3R",
                TotalDebits => "3S",
                TotalCredits => "3T",
                MinimumTransfer => "3V",
                MaximumTransfer => "3W",
                SpeedCapacity => "3X",
                Subcontractors => "3Y",
                Students => "3Z",
                DiscreteQuantityRejectedMaterialDispositionReplacement => "04",
                Accounts => "4A",
                Agents => "4B",
                AuthorizedShares => "4C",
                Clerks => "4D",
                DesignEmployees => "4E",
                ForeignRelatedEntities => "4F",
                GroupEmployees => "4G",
                IssuedShares => "4H",
                Laborers => "4I",
                OtherEmployeeType => "4J",
                PartTimeEmployees => "4K",
                RelatedEntities => "4L",
                RelativesEmployed => "4M",
                Salespersons => "4N",
                SpaceOccupied => "4O",
                SpecialPartners => "4P",
                SuppliersCredit => "4Q",
                Technicians => "4R",
                Trainees => "4S",
                WarehouseEmployees => "4T",
                Shareholders => "4U",
                AvailableUnits => "4V",
                TotalUnduplicatedHeadcount => "4W",
                MealsPerWeek => "4X",
                ProgramsOffered => "4Y",
                Code4Z => "4Z",
                DiscreteQuantityRejectedMaterialDispositionCredit => "05",
                AggregateBenefitPeriod => "5A",
                AnticipatedLengthOfService => "5B",
                ApprovalOfferDuration => "5C",
                BenefitAmount => "5D",
                BenefitPeriod => "5E",
                BrothersDeceased => "5F",
                BrothersLiving => "5G",
                Children => "5H",
                Citations => "5I",
                ClaimPeriod => "5J",
                Coverage => "5K",
                EliminationPeriod => "5L",
                EliminationPeriodAccident => "5M",
                EliminationPeriodSickness => "5N",
                EmployeesNonowner => "5O",
                EmployeesOwner => "5P",
                EmployeesPartTime => "5Q",
                EmployeesSameDuties => "5R",
                EmployeesSameOccupation => "5S",
                Expense => "5T",
                Frequency => "5U",
                GeneralEliminationPeriod => "5V",
                GuaranteePeriod => "5W",
                Height => "5X",
                HoursFlownAircraftTypeLife => "5Y",
                HoursFlownAircraftTypePeriod => "5Z",
                DiscreteQuantityRejectedMaterialDispositionPending => "06",
                HoursFlownAircraftTypeFlying => "6A",
                HoursFlownLifetime => "6B",
                HoursFlownTypeFlying => "6C",
                ImpairmentDuration => "6D",
                ImpairmentFrequency => "6E",
                InstallmentFrequency => "6F",
                Installments => "6G",
                IntendedChangeTimePeriod => "6H",
                InterimTermPeriod => "6I",
                InvolvementPeriod => "6J",
                LoanRate => "6K",
                MaximumAge => "6L",
                MaximumBenefitPeriodAccident => "6M",
                MaximumBenefitPeriodSickness => "6N",
                MaximumBenefitPeriod => "6O",
                MedicationDuration => "6P",
                MinimumAge => "6Q",
                OwnOccupationQualificationPeriod => "6R",
                OwnersEquity => "6S",
                OwnershipChangeAge => "6T",
                OwnershipDuration => "6U",
                OwnershipPercentage => "6V",
                PaymentFrequency => "6W",
                PaymentsNumber => "6X",
                Arrests => "6Y",
                PlacementPeriodExpiration => "6Z",
                CumulativeQuantityRejectedMaterial => "07",
                PreviousBenefits => "7A",
                QualificationPeriod => "7B",
                RangeAverage => "7C",
                RangeMaximum => "7D",
                RangeMinimum => "7E",
                RelationshipDuration => "7F",
                ReplacedAmount => "7G",
                ResidenceDuration => "7H",
                SistersDeceased => "7I",
                SistersLiving => "7J",
                TimeFrame => "7K",
                TimeInCountry => "7L",
                TimeSinceHospitalization => "7M",
                TimeSinceLastApplication => "7N",
                TimeSinceLastCivilianFlight => "7O",
                TimeSinceLastInsuranceMedical => "7P",
                TimeSinceLastMilitaryFlight => "7Q",
                TimeSinceMedicalConsult => "7R",
                TimeSinceMedicationEnd => "7S",
                TimeSinceMedicationStart => "7T",
                TimeSinceOnset => "7U",
                TimeSinceSurgery => "7V",
                TimeSinceTrip => "7W",
                TravelFrequency => "7X",
                TravelPeriod => "7Y",
                TripDuration => "7Z",
                CumulativeQuantityRejectedMaterialDispositionReplacement => "08",
                VisitationFrequency => "8A",
                Weight => "8B",
                WeightChangePeriod => "8C",
                WorkPeriod => "8D",
                ExistenceLimitPeriod => "8E",
                Shares => "8F",
                Directors => "8G",
                Minimum => "8H",
                VotingSharesHeld => "8I",
                OutstandingShares => "8J",
                SharesHeldAsTreasuryStock => "8K",
                SharesSubscribedButNotIssued => "8L",
                TotalSharesOfStock => "8M",
                SharesOwnedByInStateResidents => "8N",
                SharesOwnedByOutOfStateResidents => "8O",
                Partners => "8P",
                LandHolding => "8Q",
                NonDomesticStockholders => "8R",
                SharesSubscribed => "8S",
                MaximumNumberFreeMiles => "8T",
                Code8U => "8U",
                Code8V => "8V",
                FullTimeEquivalents => "8W",
                TotalCreditHours => "8X",
                TotalNonCreditHours => "8Y",
                TotalContactHours => "8Z",
                CumulativeQuantityRejectedMaterialDispositionCredit => "09",
                TimeExpended => "9A",
                PrimaryMeterReadingValue => "9C",
                EngineeredStandard => "9D",
                ActiveMaintenanceTime => "9E",
                ActualDuration => "9F",
                EstimatedDuration => "9H",
                GrossEstimate => "9J",
                FinishOffset => "9K",
                StartOffset => "9L",
                PictureCount => "9M",
                ComponentMeterReadingCount => "9N",
                TotalClockHours => "9P",
                Enrollees => "9R",
                TotalDaysSubmitted => "9S",
                TotalDaysApproved => "9V",
                CumulativeQuantityRejectedMaterialDispositionPending => "10",
                SplitQuantity => "11",
                ShipNoticeQuantity => "12",
                CollateralRequirements => "13",
                QuantityInFloat => "14",
                QuantityInHoldOut => "15",
                LineThreadQuantity => "16",
                QuantityOnHand => "17",
                PreviousWeekQuantity => "18",
                UnverifiedReceipts => "19",
                UnusableQuantity => "20",
                CumulativeQuantityShippedShortDispositionPending => "21",
                CumulativeQuantityShippedShortDispositionChallenged => "22",
                CumulativeQuantityShippedLongDispositionPending => "23",
                CumulativeQuantityShippedLongDispositionChallenged => "24",
                OemInventory => "25",
                TotalInventory => "26",
                CommittedQuantity => "27",
                QuantityAvailableForReturn => "28",
                ProjectedAvailableInventory => "29",
                QuoteQuantityOnInventory => "30",
                AdditionalDemandQuantity => "31",
                QuantitySold => "32",
                Code33 => "33",
                NoncommittedInventoryOnShelf => "34",
                InventoryOnShelfWorkInProgress => "35",
                DistributorInventory => "36",
                WorkInProcess => "37",
                OriginalQuantity => "38",
                ShippedQuantity => "39",
                RemainingQuantity => "40",
                NumberOfBatches => "41",
                NumberOfChecks => "42",
                TalkPaths => "43",
                NumberOfPatientAdmissions => "44",
                CumulativeQuantityOnOrder => "45",
                TotalTransactions => "46",
                PrimaryNetQuantity => "47",
                SecondaryNetQuantity => "48",
                NumberOfSignedBillsOfLading => "49",
                NumberOfCopiesOfBillOfLading => "50",
                NumberOfUnsignedBillsOfLading => "51",
                NumberOfOriginals => "52",
                OriginalPaymentItemCount => "53",
                BankRejectItemCount => "54",
                NetToPayItemCount => "55",
                MinimumContractQuantity => "56",
                MinimumOrderQuantity => "57",
                PaymentCancellationItemCount => "58",
                IndividualsWithDevelopmentalDelay => "59",
                TotalAuthorizedQuantity => "60",
                RemainingAuthorizedQuantity => "61",
                NumberOfDaysCoveredByInventory => "62",
                OnOrderQuantity => "63",
                PastDueQuantity => "64",
                PreviousMonthsUsage => "65",
                MinimumFabricationQuantity => "66",
                MinimumShipQuantity => "67",
                MaximumNumberOfShipmentsAllowed => "68",
                IncrementalOrderQuantity => "69",
                MaximumOrderQuantity => "70",
                EducableMentallyRetardedIndividuals => "71",
                MinimumStockLevel => "72",
                MaximumStockLevel => "73",
                DamagedGoods => "74",
                Receipts => "75",
                Returns => "76",
                StockTransfersIn => "77",
                StockTransfersOut => "78",
                Code79 => "79",
                Code80 => "80",
                PrepaidQuantityShipped => "81",
                PrepaidQuantityNotShipped => "82",
                SubmittedQuantitySold => "83",
                SubmittedQuantityReturned => "84",
                LotSize => "85",
                NonconformanceQuantity => "86",
                QuantityReceived => "87",
                Beds => "88",
                OperatingBeds => "89",
                AcknowledgedQuantity => "90",
                AdditionalUsageQuantity => "91",
                AllottedUsageQuantity => "92",
                AttendantHandledQuantity => "93",
                BillableQuantity => "94",
                DataStorageQuantity => "95",
                NonBillableQuantity => "96",
                NonUrgentDeliveryQuantity => "97",
                OverflowQuantity => "98",
                QuantityUsed => "99",
                SeverelyMentallyRetardedIndividuals => "A0",
                AcceptableUnserviceableQuantity => "A1",
                OptimisticDuration => "A2",
                MostLikelyDuration => "A3",
                PessimisticDuration => "A4",
                AdjustedQuantity => "A5",
                Accidents => "A6",
                YearsInSchool => "A7",
                NumberOfDependents => "A8",
                YearsOnJob => "A9",
                UnacknowledgedQuantity => "AA",
                UrgentDeliveryQuantity => "AB",
                VoiceStorageQuantity => "AC",
                MaintenanceUnits => "AD",
                CodeAE => "AE",
                CodeAF => "AF",
                NumberOfEndUsers => "AG",
                NumberOfMessageRecipients => "AH",
                NumberOfOperatorCredits => "AI",
                DailyAdjustments => "AJ",
                YearsInThisLineOfWorkProfession => "AK",
                AreaPerUnits => "AL",
                TrainableMentallyRetardedIndividuals => "AM",
                AgeAtDeath => "AN",
                VerifiedReceipts => "AO",
                OrderQuantityMultiple => "AP",
                ContributionTotal => "AQ",
                LoanRepaymentTotal => "AR",
                ParticipantTotal => "AS",
                Actual => "AT",
                CumulativeActual => "AU",
                Budget => "AV",
                CumulativeBudget => "AW",
                NumberOfInsuredLives => "AX",
                Forecast => "AY",
                ForecastAtComplete => "AZ",
                NumberOfMortgagors => "B1",
                MortgagePoolCount => "B2",
                RequestedAmount => "B3",
                ApprovedAmount => "B4",
                AdditionalAmount => "B5",
                PreOpDays => "B6",
                PostOpDays => "B7",
                Average => "B8",
                PeriodBeginningImbalanceQuantity => "B9",
                DueIn => "BA",
                ContractorCumulativeToDate => "BB",
                BudgetAtComplete => "BC",
                ContractorAtComplete => "BD",
                SubcontractorCumulativeToDate => "BE",
                AgeModifyingUnits => "BF",
                SubcontractorAtComplete => "BG",
                BookOrderQuantity => "BH",
                BookInventory => "BI",
                BedroomCount => "BJ",
                BathroomCount => "BK",
                BettermentHours => "BL",
                DepreciationHours => "BM",
                SystemAdjustedHours => "BN",
                UserAdjustedHours => "BO",
                PeriodEndingImbalanceQuantity => "BP",
                BackorderQuantity => "BQ",
                BloodRecord => "BR",
                CumulativeBeginningImbalanceQuantity => "BS",
                CumulativeCurrentPeriodImbalanceQuantity => "BT",
                CumulativePriorPeriodAdjustment => "BU",
                CumulativeEndingImbalanceQuantity => "BV",
                BirthWeight => "BW",
                CurrentPeriodImbalanceQuantity => "BX",
                ProductionDeliveryQuantity => "BY",
                EntitlementQuantity => "BZ",
                Creditors => "C0",
                PaymentExperiencesInLast12Months => "C1",
                PaymentExperiencesInLast3Months => "C2",
                AreaDamaged => "C3",
                OtherUnlistedStockholders => "C4",
                OtherUnlistedParticipants => "C5",
                CoveredActual => "CA",
                ClosingStatementBalance => "CB",
                CurrentDaysOnMarket => "CC",
                CoInsuredActual => "CD",
                CoveredEstimated => "CE",
                CoInsuredEstimated => "CF",
                CumulativeGasVolume => "CG",
                CumulativeEffectOfPriorPeriodAdjustment => "CH",
                CumulativeGasInjectionVolume => "CI",
                CumulativeLiquidInjectionVolume => "CL",
                NumberOfComponents => "CM",
                ContinuanceDuration => "CN",
                CumulativeOilCondensateVolume => "CO",
                CurrentPeriodImbalance => "CP",
                CodeCR => "CR",
                CurrentServiceLife => "CS",
                CumulativeWaterVolume => "CW",
                ConvictionsSent => "CY",
                TotalNumberOfConvictions => "CZ",
                Engineers => "D0",
                Billed => "D1",
                Executives => "D2",
                NumberOfCoInsuranceDays => "D3",
                FieldWorkers => "D4",
                Installers => "D5",
                MembersInGroup => "D6",
                NonConsolidatedTotalDomesticSubsidiaries => "D7",
                NonConsolidatedTotalForeignSubsidiaries => "D8",
                NonUnionEmployees => "D9",
                DependentsAge => "DA",
                DeductibleBloodUnits => "DB",
                DependentCount => "DC",
                Distributed => "DD",
                Debited => "DE",
                Deleted => "DF",
                GasUsedForDrilling => "DG",
                MaximumBenefitPeriodAccidentToAge => "DH",
                Disposed => "DI",
                MaximumBenefitPeriodSicknessToAge => "DJ",
                AirlineAttendants => "DK",
                CompaniesIncludedInConsolidation => "DL",
                TotalConsolidatedDomesticSubsidiaries => "DM",
                DefaultNotificationResponsePeriod => "DN",
                DaysOperated => "DO",
                DaysProduced => "DP",
                TotalConsolidatedForeignSubsidiaries => "DQ",
                DirectWorkers => "DR",
                Dose => "DS",
                DependentTotal => "DT",
                CounterClerks => "DU",
                DesignCapacity => "DV",
                DomesticAffiliatedCompanies => "DW",
                Drivers => "DX",
                Days => "DY",
                EmployedAtLocation => "DZ",
                CourseSegments => "E1",
                DegreeSegments => "E2",
                EmployedOnThisJob => "E3",
                EmployedInThisProfession => "E4",
                EmployedByThisCompany => "E5",
                NumberOfEntitledExemptions => "E8",
                NumberOfWithholdingExemptions => "E9",
                ExclusiveUses => "EA",
                NonexclusiveUses => "EB",
                UseOfExtracorporealCirculation => "EC",
                DomesticUses => "ED",
                SmallBusinessUses => "EE",
                Nurses => "EF",
                OfficeWorkers => "EG",
                PaidInCommonShares => "EH",
                PaidInPreferredShares => "EI",
                Pilots => "EJ",
                PlantWorkers => "EK",
                PrincipalsIncludedAsEmployees => "EL",
                EmergencyModifyingUnits => "EM",
                Suppliers => "EN",
                Teachers => "EO",
                ProductExchangeAmount => "EP",
                EquitySecurityHolder => "EQ",
                EstimatedRemainingEconomicLife => "ER",
                EndingStock => "ES",
                EmployeeTotal => "ET",
                TotalConsolidatedSubsidiaries => "EU",
                TotalNonConsolidatedSubsidiaries => "EV",
                EvaporatedWater => "EW",
                UnionEmployees => "EX",
                PortedTelephoneLines => "EY",
                ServiceResale => "EZ",
                TotalClaimsWithSkinDiseasesOrDisorders => "F0",
                OffLeaseFuel => "F1",
                TotalDeathsAsAResultOfInjury => "F3",
                TotalDeathsAsAResultOfIllness => "F4",
                TotalInjuryClaimsWithDaysAwayFromWorkOrRestrictedWorkActivity => "F5",
                TotalInjuryClaimsWithDaysAwayFromWork => "F6",
                TotalInjuryClaimsWithoutLostWorkDays => "F7",
                TotalDaysAwayFromWorkDueToInjury => "F8",
                TotalDaysWithRestrictedWorkActivityDueToInjury => "F9",
                FullBaths => "FA",
                FurnishedBloodUnits => "FB",
                FuelConsumedOrBurnedAmount => "FC",
                VehicularRadios => "FD",
                PortableRadios => "FE",
                FlareOrFlash => "FF",
                MarineRadios => "FG",
                Pagers => "FH",
                ConventionalMobiles => "FI",
                TrunkedChannels => "FJ",
                MobileLoadingAllocation => "FK",
                Units => "FL",
                AircraftRadios => "FM",
                TotalClaimsWithDustDiseasesOfTheLungs => "FN",
                TotalClaimsWithRespiratoryConditionsDueToToxicAgents => "FO",
                TotalClaimsWithPoisoningIllnesses => "FP",
                TotalClaimsWithDisordersDueToPhysicalAgents => "FQ",
                GasUsedForFuelSystem => "FS",
                ForecastToComplete => "FT",
                TotalClaimsAssociatedWithRepeatedTrauma => "FU",
                TotalIllnessClaimsWithOccupationalIllnessesNotOtherwiseClassified => "FV",
                TotalDaysAwayFromWorkDueToIllness => "FW",
                TotalDaysOfRestrictedWorkActivityDueToIllness => "FX",
                TotalIllnessWithLostWorkDaysOrRestrictedWorkActivity => "FY",
                TotalIllnessClaimsWithDaysAwayFromWork => "FZ",
                DischargeQuantity => "G0",
                EstimatedDischargeQuantity => "G1",
                EstimatedTransferQuantity => "G2",
                Excursions => "G3",
                NonProductionQuantity => "G4",
                NumberOfDeaths => "G5",
                NumberOfHospitalizations => "G6",
                NumberOfInjuries => "G7",
                NumberOfInjuriesRequiringMedicalTreatment => "G8",
                NumberOfPeopleEvacuated => "G9",
                GrossBuildingArea => "GA",
                GrossAnnualIncomeMultiplier => "GB",
                GrossLivingArea => "GC",
                TotalIllnessClaimsWithoutLostWorkDays => "GD",
                OriginalTermInYears => "GE",
                YearsRemaining => "GF",
                AverageNumberOfEmployees => "GG",
                TotalWorkedByAllEmployees => "GH",
                GasInjectionVolume => "GI",
                GasLiftVolume => "GL",
                Episode => "GM",
                CodeGN => "GN",
                CodeGO => "GO",
                GrossProduction => "GP",
                GovernmentReportingQuantity => "GQ",
                GasReceiptVolume => "GR",
                GasSold => "GS",
                GradeTransferAmount => "GT",
                EmployeeTotalFirstMonthOfQuarter => "GU",
                GasVolume => "GV",
                EmployeeTotalSecondMonthOfQuarter => "GW",
                EmployeeTotalThirdMonthOfQuarter => "GX",
                ActiveListings => "GZ",
                NumberOfPeopleShelteredInPlace => "H0",
                QuantityRecovered => "H1",
                QuantityRecycled => "H2",
                QuantityReleased => "H3",
                QuantityTreated => "H4",
                TotalHazardousWasteGenerated => "H5",
                OperationalQuantity => "H6",
                PenaltyVarianceQuantity => "H7",
                AllocatedQuantity => "H8",
                ScheduledQuantity => "H9",
                MarketPriceChange => "HA",
                Unpaid => "HB",
                Branches => "HC",
                Subsidiaries => "HD",
                AgeOfFinancial => "HE",
                Invoices => "HF",
                FinancialCoveragePeriod => "HG",
                MaximumNumberOfEmployeesAtLocation => "HH",
                PreviousNumberOfAccounts => "HI",
                CollectionPeriod => "HJ",
                DisbursementPeriod => "HK",
                Seats => "HL",
                UseOfHypothermia => "HM",
                PreviousNumberOfEmployees => "HN",
                UseOfHypotension => "HO",
                UseOfHyperbaricPressurization => "HP",
                KindergartenStudents => "HQ",
                UseOfHypertension => "HR",
                Hours => "HS",
                EmployeesAge => "HT",
                EmployeesNumberOfDaysAwayFromWorkDueToInjury => "HU",
                EmployeesNumberOfDaysOfRestrictedWorkActivityDueToInjury => "HV",
                EmployeesTotalNumberOfDaysAwayFromWorkDueToIllness => "HW",
                TotalDeathClaims => "HY",
                TotalClaimsWithDaysAwayFromWork => "HZ",
                TenthGradeStudents => "I0",
                EleventhGradeStudents => "I1",
                TwelfthGradeStudents => "I2",
                PriorTeachingExperience => "I3",
                PriorFullTimeTeachingExperience => "I4",
                PriorPartTimeTeachingExperience => "I5",
                PriorExperienceInEducation => "I6",
                PriorFullTimeExperienceInEducation => "I7",
                PriorPartTimeExperienceInEducation => "I8",
                PriorExperienceRelatedToJob => "I9",
                LocalCountryEmployees => "IA",
                ForeignEmployees => "IB",
                PriorFullTimeExperienceRelatedToJob => "IC",
                PriorPartTimeExperienceRelatedToJob => "ID",
                TotalPriorExperience => "IE",
                TotalFullTimePriorExperience => "IF",
                TotalPartTimePriorExperience => "IG",
                TotalYearsOfEducationalService => "IH",
                NumberOfIrregularInterestPayments => "II",
                TotalYearsOfEducationalServiceInThisDistrict => "IJ",
                YearsOfExperienceAsSchoolPrincipal => "IK",
                YearsOfExperienceAsClassroomTeacher => "IL",
                YearsWorkedForThisSystem => "IM",
                IndirectWorkers => "IN",
                NumberOfInterestPayments => "IP",
                InTransitQuantity => "IQ",
                InformationProviderStandardizedMotorVehiclePenaltyPoints => "IS",
                IntertankTransferAmount => "IT",
                EndingStorageBalance => "J0",
                LocationEndingStorageBalance => "J1",
                LocationEndingStorageBalanceFirm => "J2",
                LocationEndingStorageBalanceInterruptible => "J3",
                MaximumAvailableDailyInjectionQuantity => "J4",
                MaximumAvailableDailyWithdrawalQuantity => "J5",
                MinimumRequiredDailyInjectionQuantity => "J6",
                MinimumRequiredDailyWithdrawalQuantity => "J7",
                ActivityCodes => "JA",
                Associates => "JB",
                AverageEmployees => "JC",
                CooperativeShares => "JD",
                EstimatedEmployeesAtLocation => "JE",
                EstimatedTotalEmployees => "JF",
                FinancialInstitutions => "JG",
                Judgments => "JH",
                LandSize => "JI",
                Liens => "JJ",
                MinimumEmployeesAtLocation => "JK",
                OfficeSize => "JL",
                Owner => "JM",
                PlantSize => "JN",
                PreviousNumberOfBranches => "JO",
                ProtestedBills => "JP",
                Suits => "JQ",
                CodeJR => "JR",
                JudicialStayDuration => "JS",
                WarehouseSize => "JT",
                TotalDaysAwayFromWork => "JU",
                TotalDaysOfRestrictedWorkActivity => "JV",
                TotalClaimsWithoutDaysAwayFromWorkAndWithoutRestrictedWorkActivity => {
                    "JW"
                }
                Secretaries => "JX",
                Mechanics => "JY",
                Auditors => "JZ",
                Messengers => "K1",
                PrimaryManagers => "K2",
                ParticipationShares => "K3",
                DetrimentalLegalFilings => "K4",
                PetitionsFiled => "K5",
                Drafts => "K6",
                BusinessFailureNationalAverageIncidence => "K7",
                BusinessFailureIndustryIncidence => "K8",
                BusinessFailureClassIncidence => "K9",
                Estimated => "KA",
                NetQuantityIncrease => "KB",
                NetQuantityDecrease => "KC",
                ExpenditureQuantity => "KD",
                Originals => "KE",
                Duplicates => "KF",
                CompletedLineItems => "KG",
                CompletedContracts => "KH",
                ActiveContractsDelinquentBuyingPartyCaused => "KI",
                ActiveContractsDelinquent => "KJ",
                ActiveContractsDelinquentContractorCaused => "KK",
                ActiveContractsDelinquentUnknownCauses => "KL",
                ActiveLineItemsDelinquent => "KM",
                ActiveLineItemsDelinquentBuyingPartyCaused => "KN",
                ActiveLineItemsDelinquentContractorCaused => "KO",
                ActiveLineItemsDelinquentUnknownCauses => "KP",
                ContractsCompletedDelinquentBuyingPartyCaused => "KQ",
                ContractCompletedDelinquentContractorCaused => "KR",
                ContractsCompletedDelinquentUnknownCauses => "KS",
                ReportedDeficiencies => "KU",
                LineItemsCompletedDelinquentBuyingPartyCaused => "KV",
                LineItemsCompletedDelinquentContractorCaused => "KW",
                LineItemsCompletedDelinquentUnknownCauses => "KX",
                CorrectiveActionRequestsVerbal => "KY",
                CorrectiveActionRequestsWritten => "KZ",
                GuaranteeFeeBuyupMaximum => "L2",
                ContractBuyup => "L3",
                ContractBuydown => "L4",
                GuaranteeFeeRateAfterAlternatePaymentMethod => "L5",
                GuaranteeFeeRateAfterBuyupOrBuydown => "L6",
                BuyupOrBuydownRatePerBasisPoint => "L7",
                LocationNetCapacity => "L8",
                SubjectToLossOrElimination => "L9",
                LifeTimeReserveActual => "LA",
                LossAllowance => "LB",
                LatePaymentPeriod => "LC",
                LimitValue => "LD",
                LifeTimeReserveEstimated => "LE",
                LossOrGain => "LG",
                LostGas => "LH",
                LiquidInjectionVolume => "LI",
                CorrectiveActionRequestsMethodC => "LK",
                CorrectiveActionRequestsMethodD => "LL",
                CorrectiveActionRequestsMethodE => "LM",
                AgedActiveLineItemsDelinquentContractorCaused => "LN",
                LostOil => "LO",
                LeasePeriods => "LP",
                AgedLineItemsDelinquent => "LQ",
                AgedLineItemsCompletedContractorCaused => "LR",
                OilCondensateSold => "LS",
                TariffLossAllowance => "LT",
                LifetimeReserveDaysAppliedToThisClaim => "LU",
                OilCondensateVolume => "LV",
                LostWorkTimeActual => "LW",
                LostWorkTimeEstimated => "LX",
                LengthOfResidency => "LY",
                Lanes => "LZ",
                MatchingEquipment => "M1",
                Maximum => "M2",
                TotalFederalPoints => "M3",
                Contributions => "M4",
                Contributors => "M5",
                Endorsers => "M6",
                Functions => "M7",
                Guarantors => "M8",
                Points => "M9",
                MiscellaneousAllowance => "MA",
                NumberOfPublicOfficials => "MB",
                TotalNonFederalPoints => "MC",
                MillionDollarRoundtableCredits => "MD",
                MinimumNumberOfEmployees => "ME",
                Manufactured => "MF",
                Pledges => "MG",
                TotalPoints => "MH",
                Miles => "MI",
                Attendees => "MJ",
                TicketsSold => "MK",
                TotalNumberOfManifestLines => "ML",
                MaximumMaturityExtension => "MM",
                Month => "MN",
                MinimumOrderPackageLevel => "MO",
                TotalNumberOfMapsInAPack => "MP",
                MaximumShipQuantity => "MQ",
                QuantityOfNextLowerLevelTradeItem => "MR",
                MeasuredQuantity => "MS",
                ResterilizationMaximum => "MT",
                RecommendedNumberOfUses => "MU",
                TotalUnits => "MV",
                MaximumNumberOfEmployees => "MX",
                StackingFactor => "MY",
                ComponentQuantity => "MZ",
                NumberOfAttacksOrOccurrences => "N1",
                NumberOfDead => "N2",
                NumberOfLiving => "N3",
                NumberOfTimes => "N4",
                MinimumForecastQuantity => "N5",
                MaximumForecastQuantity => "N6",
                RequestedReceiptQuantity => "N7",
                RequestedDeliveryQuantity => "N8",
                NumberOfNonCoveredDays => "NA",
                CodeNB => "NB",
                NumberOfClaimants => "NC",
                NumberOfLateCharges => "ND",
                NonCoveredEstimated => "NE",
                NumberOfFullTimeEmployees => "NF",
                NumberOfNonsufficientFundItems => "NG",
                NoncoveredActual => "NH",
                NumberOfLevels => "NL",
                NumberOfHospitals => "NN",
                NumberOfPhysicians => "NO",
                NumberOfMembers => "NP",
                NumberOfFranchisees => "NQ",
                NotReplacedBloodUnits => "NR",
                NumberOfStations => "NS",
                Reports => "NT",
                SinceLastTravel => "NU",
                Net => "NV",
                UntilNextTravel => "NW",
                ScheduledReceipt => "O1",
                ScheduledDelivery => "O2",
                OperationalReceipt => "O3",
                OperationalDelivery => "O4",
                AllocatedReceipt => "O5",
                AllocatedDelivery => "O6",
                DistributedConfirmedReceipt => "O7",
                DistributedConfirmedDelivery => "O8",
                SchedulingToleranceReceipt => "O9",
                SchedulingToleranceDelivery => "OA",
                Energy => "OB",
                OrderCount => "OC",
                OtherMiscellaneousDisposition => "OD",
                NumberOfWeeksPerYear => "OE",
                OffPremiseSalesQuantity => "OF",
                OtherGasDisposition => "OG",
                OtherInjectionVolume => "OH",
                OpeningStatementBalance => "OI",
                OrderSizingFactor => "OJ",
                OriginalLoanTerm => "OL",
                OnPremiseSalesQuantity => "ON",
                OtherOilCondensateDisposition => "OO",
                OptimumOrderQuantity => "OQ",
                Original => "OR",
                NumberOfOperatingPeriodsAtFailure => "OT",
                OutlierDays => "OU",
                Overage => "OV",
                OtherWaterDisposition => "OW",
                ProjectPhases => "P1",
                PhysicalStatusIii => "P3",
                PhysicalStatusIv => "P4",
                PhysicalStatusV => "P5",
                NumberOfServicesOrProcedures => "P6",
                PrescriptionDosage => "P7",
                PrescriptionFrequency => "P8",
                NumberOfPeopleLivingAtResidence => "P9",
                PipelineAdjustmentOrAllowance => "PA",
                PressureBase => "PB",
                PriorCumulativeImbalance => "PC",
                PaymentDurationWeeks => "PD",
                PeriodOfEmployment => "PE",
                GasUsedForPlantFuel => "PF",
                Persistency => "PG",
                Promotional => "PH",
                ParkingSpaces => "PK",
                PartialBaths => "PL",
                PercentageOfOrderedQuantity => "PO",
                PurchaseOfProduct => "PP",
                CumulativeQuantityRequiredPriorToTheFirstScheduledPeriod => "PQ",
                RequirementQuantityThatWasPreviouslyReleased => "PR",
                Prescription => "PS",
                Patients => "PT",
                PittedWater => "PW",
                PriorUnitsAccepted => "PX",
                Paid => "PY",
                MinimumQuantityToWhichTaxRateApplies => "Q1",
                MaximumQuantityToWhichTaxRateApplies => "Q2",
                QuantityEarned => "Q3",
                QuantityCarriedForward => "Q4",
                NumberOf3To4YearOlds => "Q5",
                AutisticIndividuals => "Q6",
                DeafBlindIndividuals => "Q7",
                HearingImpairedIndividuals => "Q8",
                MentallyRetardedIndividuals => "Q9",
                QuantityApproved => "QA",
                QuantityDispensed => "QB",
                QuantityDisapproved => "QC",
                QuantityDelivered => "QD",
                QuantityDeferred => "QE",
                HighFabricationAuthorizationQuantity => "QF",
                QuantityOnHold => "QH",
                CommunityServiceDuration => "QI",
                NumberOfTimesDeported => "QJ",
                QuantityOfInnerPacks => "QK",
                JailSentenceDuration => "QL",
                ProbationDuration => "QM",
                RestrictionDuration => "QN",
                OperatingQuantity => "QO",
                QuantityByPosition => "QP",
                SuspendedDuration => "QQ",
                HighRawMaterialAuthorizationQuantity => "QR",
                QuantityPerSkid => "QS",
                PlantThermalReduction => "QT",
                QuantityServiced => "QU",
                QuantityCancelled => "QV",
                QuantityWithdrawn => "QW",
                QualifyingWeeks => "QX",
                RepaymentPlanTerm => "QY",
                CodeR1 => "R1",
                IndividualsWithOrthopedicImpairment => "R2",
                EstimatedRemainingPhysicalLife => "R3",
                IndividualsWithSpecificLearningDisability => "R4",
                Axles => "R5",
                PlatformCount => "R6",
                IndividualsWithVisualImpairment => "R7",
                IndividualsWithOtherHealthImpairment => "R8",
                Fuel => "R9",
                RefillsAuthorized => "RA",
                ReplacedBloodUnits => "RB",
                NumberOfItemsAuthorizedAtStore => "RC",
                NumberOfItemsAuthorizedAtWarehouse => "RD",
                GasReturnedToEarth => "RE",
                NumberOfItemsInStock => "RF",
                GasUsedForRepressuringOrPressureMaintenance => "RG",
                NumberOfShelfTags => "RH",
                QuantityAvailableOnShelf => "RJ",
                GasReturnedToPropertyForFuel => "RL",
                RoomCount => "RM",
                UnitsRented => "RN",
                RetailDemandQuantity => "RP",
                Royalty => "RQ",
                NumberOfShelfFacings => "RS",
                RetailSalesQuantity => "RT",
                WaterReInjectedOnProperty => "RW",
                RequirementQuantity => "RY",
                CodeS1 => "S1",
                CodeS2 => "S2",
                DwellingArea => "S3",
                GarageOrCarportArea => "S4",
                UnitsForSale => "S5",
                GrossRentMultiplier => "S6",
                CodeS7 => "S7",
                CodeS8 => "S8",
                CodeS9 => "S9",
                Shipments => "SA",
                Solicited => "SB",
                CodeSC => "SC",
                CriminalSentenceDuration => "SD",
                CodeSE => "SE",
                Site => "SF",
                SwanGanz => "SG",
                Shortage => "SH",
                Rooms => "SI",
                AreaOfLevel => "SJ",
                GasShrinkage => "SK",
                PredominateAge => "SL",
                MinimumCriminalSentenceDuration => "SM",
                Age => "SN",
                OilSedimentation => "SO",
                DaysSupply => "SP",
                ProductSalesAmount => "SQ",
                EffectiveAge => "SR",
                SharesOfPreferredStock => "SS",
                Standard => "ST",
                ForecastedScannedQuantity => "SU",
                SharesOfCommonStock => "SV",
                SampleAmount => "SW",
                MaximumCriminalSentenceDuration => "SX",
                StateOrProvinceMotorVehiclePenaltyPoints => "SY",
                Seasonal => "SZ",
                TimeUnitsKnown => "T1",
                TimeUnitsSpentOnDuty => "T2",
                TotalDaysOnMarket => "T3",
                TotalRooms => "T4",
                TotalNumberOfUnits => "T5",
                TotalNumberOfUnitsForSale => "T6",
                Tires => "T7",
                TankAllowance => "TA",
                OilTheft => "TB",
                TotalAtComplete => "TC",
                TotalToDate => "TD",
                NumberOfTheatres => "TE",
                TotalGasInjectionVolume => "TG",
                TheoreticalQuantity => "TH",
                TotalOilAndOrCondensateInjectionVolume => "TI",
                DurationInCurrentJob => "TJ",
                TotalOilAndOrCondensateDisposition => "TK",
                TotalWaterDisposition => "TM",
                TotalBeginningInventory => "TN",
                Total => "TO",
                TimeInPosition => "TP",
                TotalQuantityOfAllBuys => "TQ",
                Trips => "TR",
                TotalNumberOfParkingSpaces => "TS",
                TotalProductionVolume => "TT",
                TotalAdjustmentsVolume => "TU",
                TotalGasDisposition => "TV",
                TotalWaterInjectionVolume => "TW",
                TotalEndingInventory => "TX",
                TotalSalesVolume => "TY",
                FreelanceCollectors => "U1",
                BranchLocationsOwned => "U2",
                BranchLocationsLeased => "U3",
                UnitsCompleted => "UA",
                Poultry => "UB",
                Livestock => "UC",
                Passengers => "UD",
                Trainers => "UE",
                Operators => "UF",
                GasUsedOnProperty => "UG",
                Inspectors => "UH",
                Collectors => "UI",
                Professionals => "UJ",
                Supervisors => "UK",
                ApproximateNumberOfUnitsForSaleProjected => "UL",
                Administrators => "UM",
                Promoters => "UN",
                OilCondensateUsedOnProperty => "UO",
                Divisions => "UP",
                Tables => "UQ",
                FuelPumps => "UR",
                InUse => "US",
                Machines => "UT",
                Used => "UU",
                TrademarksUsed => "UV",
                AvailableForCultivation => "UW",
                Foremen => "UX",
                TravellingEmployees => "UY",
                FreelanceSalespersons => "UZ",
                RetentionQuantity => "V1",
                AvailableQuantity => "V2",
                TransferQuantity => "V3",
                SurveysInAverageRating => "V4",
                Vacancies => "V5",
                UnsubscribedCapacity => "V8",
                ShippingContainerQuantity => "V9",
                VolumeShrinkageAdjustmentOrAllowance => "VA",
                BlankVotes => "VB",
                CumulativeEarnedValue => "VC",
                ScatteredVotes => "VD",
                EarnedValue => "VE",
                FederalVotes => "VF",
                GasVented => "VG",
                ScheduleVariance => "VH",
                CumulativeScheduleVariance => "VI",
                CumulativeVariance => "VJ",
                EstimateAtComplete => "VK",
                AtCompleteVariance => "VL",
                VarianceAdjustment => "VM",
                NoVotes => "VN",
                PresidentialVotes => "VP",
                UtilizationServiceLife => "VQ",
                Variance => "VR",
                Visits => "VS",
                Votes => "VT",
                RecommendedServiceLife => "VU",
                VoidVotes => "VV",
                ShelfLifePeriod => "VW",
                YesVotes => "VY",
                BankruptcyPetitions => "W0",
                Buyers => "W1",
                Debentures => "W2",
                DebenturesFiledAgainstDirectors => "W3",
                DetrimentalLegalFilingsAgainstDirectors => "W4",
                FailedBusinessesOfDirectors => "W5",
                Professors => "W6",
                Sellers => "W7",
                SkilledWorkers => "W8",
                TrademarksRepresented => "W9",
                TotalNumberOfWorkersCompensationFirstReports => "WA",
                TotalNumberOfWorkersCompensationSubsequentReports => "WB",
                TotalNumberOfWorkersCompensationCombinedReports => "WC",
                UnitsWorkedPerDay => "WD",
                LimitedQuantity => "WE",
                WeightGain => "WG",
                WeightLoss => "WL",
                OperatorsWorkingInterest => "WO",
                NumberOfProducingWellsRemainingOnPropertyOrFacility => "WP",
                NumberOfProducingWellsRemainingOnRoyaltyAccount => "WR",
                TotalWorkingInterest => "WT",
                WaterVolume => "WV",
                WeeksWorked => "WW",
                LicenseWithdrawalDuration => "WX",
                LicenseWithdrawalsSent => "WY",
                ProducingWells => "X1",
                Gross => "X2",
                AssessmentHours => "X6",
                DutyDays => "X7",
                ContractDays => "X8",
                NumberOfDaysEmployed => "X9",
                TotalOfIssuableAssets => "XA",
                CodeXB => "XB",
                CodeXC => "XC",
                CodeXD => "XD",
                CodeXE => "XE",
                OnHandAndDueIn => "XG",
                InstallmentPayments => "XI",
                CodeXJ => "XJ",
                ApproximateNumberOfUnitsProjected => "XL",
                ApproximateNumberOfHolders => "XN",
                CirculatingOil => "XO",
                StockObjectiveAndInsuranceQuantity => "XR",
                ProtectedQuantity => "XT",
                Reserved => "XU",
                RequisitioningObjective => "XV",
                AuthorizedRetentionLevel => "XX",
                SafetyLevel => "XY",
                BackorderLines => "XZ",
                NumberOfLostCards => "Y1",
                NumberOfStolenCards => "Y2",
                NumberOfCardsNotReceived => "Y3",
                NumberOfActiveAccountsThisCycle => "Y4",
                NumberOfOpenAccounts => "Y5",
                NumberOfAccountsPastDue => "Y6",
                NumberOfCardsOutstanding => "Y7",
                OnHandPlusPipeline => "Y8",
                TotalDemandQuantity => "YA",
                TotalDemandOrders => "YB",
                FirstQuarterRecurringDemand => "YC",
                FirstQuarterRecurringOrders => "YD",
                FirstQuarterNonRecurringDemand => "YE",
                FirstQuarterNonRecurringOrders => "YF",
                SecondQuarterRecurringDemand => "YG",
                SecondQuarterRecurringOrders => "YH",
                SecondQuarterNonRecurringDemand => "YJ",
                SecondQuarterNonRecurringOrders => "YK",
                ThirdQuarterRecurringDemand => "YL",
                ThirdQuarterRecurringOrders => "YM",
                ThirdQuarterNonRecurringDemand => "YN",
                ThirdQuarterNonRecurringOrders => "YP",
                FourthQuarterRecurringDemand => "YQ",
                FourthQuarterRecurringOrders => "YR",
                FourthQuarterNonRecurringDemand => "YS",
                FourthQuarterNonRecurringOrders => "YT",
                Trailers => "YU",
                ReorderPointQuantity => "YW",
                ContractLineItemQuantity => "YX",
                Years => "YY",
                MaximumQuantityOfFreeServiceCalls => "YZ",
                UnitsWorkedLastDay => "Z1",
                UnitsWorkedPerWeek => "Z2",
                UnitsWorkedPerQuarter => "Z3",
                NumberWeeksPaid => "Z4",
                UnusedAccumulatedSickDays => "Z6",
                DeliveryPointReductionQuantity => "Z7",
                ReceiptPointReductionQuantity => "Z8",
                ReductionQuantity => "Z9",
                FederalMedicareOrMedicaidClaimMandateCategory1 => "ZA",
                FederalMedicareOrMedicaidClaimMandateCategory2 => "ZB",
                FederalMedicareOrMedicaidClaimMandateCategory3 => "ZC",
                FederalMedicareOrMedicaidClaimMandateCategory4 => "ZD",
                FederalMedicareOrMedicaidClaimMandateCategory5 => "ZE",
                FederalPensionMandateCategory1 => "ZF",
                FederalPensionMandateCategory2 => "ZG",
                FederalPensionMandateCategory3 => "ZH",
                HoldingPeriod => "ZI",
                FederalPensionMandateCategory5 => "ZJ",
                FederalMedicareOrMedicaidPaymentMandateCategory1 => "ZK",
                FederalMedicareOrMedicaidPaymentMandateCategory2 => "ZL",
                FederalMedicareOrMedicaidPaymentMandateCategory3 => "ZM",
                FederalMedicareOrMedicaidPaymentMandateCategory4 => "ZN",
                FederalMedicareOrMedicaidPaymentMandateCategory5 => "ZO",
                FederalPensionMandateCategory4 => "ZP",
                SharesAdded => "ZQ",
                ExtendedTerm => "ZR",
                AmortizationTerm => "ZS",
                BeginningShares => "ZT",
                SharesDeleted => "ZU",
                QuantityOfDealerLicensePlates => "ZV",
                CurrentShareBalance => "ZW",
                SizeOfHousehold => "ZX",
                ProjectUnitsSold => "ZY",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<QuantityQualifier> {
        use QuantityQualifier::*;
        match code {
            b"00" => Some(HospitalHomeboundIndividuals),
            b"0A" => Some(NumberOfHoursPerDay),
            b"0B" => Some(NumberOfHoursPerWeek),
            b"0C" => Some(NumberOfMonthsPerYear),
            b"0D" => Some(NumberOfPeriodsPerWeek),
            b"0E" => Some(ExpectedExpenditureQuantity),
            b"0F" => Some(NumberOfHoursPerYear),
            b"0G" => Some(PreKindergartenStudents),
            b"0H" => Some(FirstGradeStudents),
            b"0I" => Some(SecondGradeStudents),
            b"0J" => Some(ThirdGradeStudents),
            b"0K" => Some(FourthGradeStudents),
            b"0L" => Some(FifthGradeStudents),
            b"0M" => Some(SixthGradeStudents),
            b"0N" => Some(SeventhGradeStudents),
            b"0O" => Some(EighthGradeStudents),
            b"0P" => Some(NinthGradeStudents),
            b"0Q" => Some(CarnegieUnits),
            b"0R" => Some(NumberOfDisabilityTypes),
            b"0S" => Some(NumberOfMales),
            b"0T" => Some(NumberOfFemales),
            b"0U" => Some(IndividualsWithMultipleDisabilities),
            b"0V" => Some(IndividualsWithSeriousEmotionalDisturbance),
            b"0W" => Some(IndividualsWithSpeechOrLanguageImpairment),
            b"0X" => Some(IndividualsWithTraumaticBrainInjury),
            b"0Y" => Some(BlindIndividuals),
            b"0Z" => Some(DeafIndividuals),
            b"01" => Some(DiscreteQuantity),
            b"1A" => Some(Code1A),
            b"1B" => Some(Code1B),
            b"1C" => Some(Code1C),
            b"1D" => Some(Code1D),
            b"1E" => Some(Code1E),
            b"1F" => Some(Code1F),
            b"1G" => Some(Code1G),
            b"1H" => Some(Started),
            b"1I" => Some(Completed),
            b"1J" => Some(Due),
            b"1K" => Some(TimeUnits),
            b"1L" => Some(Shifts),
            b"1M" => Some(TimeUnitsPerShift),
            b"1N" => Some(ScrapAllowed),
            b"1O" => Some(CalendarUnits),
            b"1P" => Some(Code1P),
            b"1Q" => Some(Code1Q),
            b"1R" => Some(Code1R),
            b"1S" => Some(Late),
            b"1T" => Some(NumberOfDelinquentInstallments),
            b"1U" => Some(NumberOfLoans),
            b"1V" => Some(TotalNumberOfMortgagees),
            b"1W" => Some(TotalNumberOfLoanDetailRecords),
            b"1X" => Some(PrescriptionEffectivePeriod),
            b"1Y" => Some(Code1Y),
            b"1Z" => Some(EndOfMonthInventoryPriorToShip),
            b"02" => Some(CumulativeQuantity),
            b"2A" => Some(CommitmentPeriod),
            b"2B" => Some(NumberOfBorrowers),
            b"2C" => Some(NumberOfAdjustmentPeriods),
            b"2D" => Some(AgeNearest),
            b"2E" => Some(TotalOtherPropertiesOwnedAndFinanced),
            b"2F" => Some(AgeNext),
            b"2G" => Some(ReconsiderationPeriod),
            b"2H" => Some(FlatExtraPremium),
            b"2I" => Some(Co2InjectionVolume),
            b"2J" => Some(AccountsPlacedForCollection),
            b"2K" => Some(Changes),
            b"2L" => Some(CompaniesInSameActivityForAPeriod),
            b"2M" => Some(ComparisonPeriod),
            b"2N" => Some(Departments),
            b"2O" => Some(EmployeesShared),
            b"2P" => Some(EstimatedAccounts),
            b"2Q" => Some(InstalledCapacity),
            b"2R" => Some(LevelsOccupied),
            b"2S" => Some(RegisteredBrandsDistributed),
            b"2T" => Some(ElectronicSignatures),
            b"2U" => Some(Bytes),
            b"2V" => Some(EmployedAtThisLocation),
            b"2W" => Some(Segments),
            b"2X" => Some(RegisteredBrandsManufactured),
            b"2Y" => Some(FunctionalGroups),
            b"2Z" => Some(TransactionSets),
            b"03" => Some(DiscreetQuantityRejectedMaterial),
            b"3A" => Some(TotalCreditsAccepted),
            b"3B" => Some(TotalCreditsRejected),
            b"3C" => Some(TotalDebitsAccepted),
            b"3D" => Some(TotalDebitsRejected),
            b"3E" => Some(TotalPaymentsRejected),
            b"3F" => Some(TotalPreAdvicesAccepted),
            b"3G" => Some(TotalPreAdvicesRejected),
            b"3H" => Some(TotalPrenotesAccepted),
            b"3I" => Some(TotalPrenotesRejected),
            b"3J" => Some(TotalPostAdvicesAccepted),
            b"3K" => Some(TotalPostAdvicesRejected),
            b"3L" => Some(TotalUnidentifiedTransactionsRejected),
            b"3M" => Some(TotalCreditsReceived),
            b"3N" => Some(TotalDebitsReceived),
            b"3O" => Some(IndividualsWithNoncategoricalPreschoolDisability),
            b"3P" => Some(TotalPreAdvicesReceived),
            b"3Q" => Some(TotalPrenotesReceived),
            b"3R" => Some(TotalPostAdvicesReceived),
            b"3S" => Some(TotalDebits),
            b"3T" => Some(TotalCredits),
            b"3V" => Some(MinimumTransfer),
            b"3W" => Some(MaximumTransfer),
            b"3X" => Some(SpeedCapacity),
            b"3Y" => Some(Subcontractors),
            b"3Z" => Some(Students),
            b"04" => Some(DiscreteQuantityRejectedMaterialDispositionReplacement),
            b"4A" => Some(Accounts),
            b"4B" => Some(Agents),
            b"4C" => Some(AuthorizedShares),
            b"4D" => Some(Clerks),
            b"4E" => Some(DesignEmployees),
            b"4F" => Some(ForeignRelatedEntities),
            b"4G" => Some(GroupEmployees),
            b"4H" => Some(IssuedShares),
            b"4I" => Some(Laborers),
            b"4J" => Some(OtherEmployeeType),
            b"4K" => Some(PartTimeEmployees),
            b"4L" => Some(RelatedEntities),
            b"4M" => Some(RelativesEmployed),
            b"4N" => Some(Salespersons),
            b"4O" => Some(SpaceOccupied),
            b"4P" => Some(SpecialPartners),
            b"4Q" => Some(SuppliersCredit),
            b"4R" => Some(Technicians),
            b"4S" => Some(Trainees),
            b"4T" => Some(WarehouseEmployees),
            b"4U" => Some(Shareholders),
            b"4V" => Some(AvailableUnits),
            b"4W" => Some(TotalUnduplicatedHeadcount),
            b"4X" => Some(MealsPerWeek),
            b"4Y" => Some(ProgramsOffered),
            b"4Z" => Some(Code4Z),
            b"05" => Some(DiscreteQuantityRejectedMaterialDispositionCredit),
            b"5A" => Some(AggregateBenefitPeriod),
            b"5B" => Some(AnticipatedLengthOfService),
            b"5C" => Some(ApprovalOfferDuration),
            b"5D" => Some(BenefitAmount),
            b"5E" => Some(BenefitPeriod),
            b"5F" => Some(BrothersDeceased),
            b"5G" => Some(BrothersLiving),
            b"5H" => Some(Children),
            b"5I" => Some(Citations),
            b"5J" => Some(ClaimPeriod),
            b"5K" => Some(Coverage),
            b"5L" => Some(EliminationPeriod),
            b"5M" => Some(EliminationPeriodAccident),
            b"5N" => Some(EliminationPeriodSickness),
            b"5O" => Some(EmployeesNonowner),
            b"5P" => Some(EmployeesOwner),
            b"5Q" => Some(EmployeesPartTime),
            b"5R" => Some(EmployeesSameDuties),
            b"5S" => Some(EmployeesSameOccupation),
            b"5T" => Some(Expense),
            b"5U" => Some(Frequency),
            b"5V" => Some(GeneralEliminationPeriod),
            b"5W" => Some(GuaranteePeriod),
            b"5X" => Some(Height),
            b"5Y" => Some(HoursFlownAircraftTypeLife),
            b"5Z" => Some(HoursFlownAircraftTypePeriod),
            b"06" => Some(DiscreteQuantityRejectedMaterialDispositionPending),
            b"6A" => Some(HoursFlownAircraftTypeFlying),
            b"6B" => Some(HoursFlownLifetime),
            b"6C" => Some(HoursFlownTypeFlying),
            b"6D" => Some(ImpairmentDuration),
            b"6E" => Some(ImpairmentFrequency),
            b"6F" => Some(InstallmentFrequency),
            b"6G" => Some(Installments),
            b"6H" => Some(IntendedChangeTimePeriod),
            b"6I" => Some(InterimTermPeriod),
            b"6J" => Some(InvolvementPeriod),
            b"6K" => Some(LoanRate),
            b"6L" => Some(MaximumAge),
            b"6M" => Some(MaximumBenefitPeriodAccident),
            b"6N" => Some(MaximumBenefitPeriodSickness),
            b"6O" => Some(MaximumBenefitPeriod),
            b"6P" => Some(MedicationDuration),
            b"6Q" => Some(MinimumAge),
            b"6R" => Some(OwnOccupationQualificationPeriod),
            b"6S" => Some(OwnersEquity),
            b"6T" => Some(OwnershipChangeAge),
            b"6U" => Some(OwnershipDuration),
            b"6V" => Some(OwnershipPercentage),
            b"6W" => Some(PaymentFrequency),
            b"6X" => Some(PaymentsNumber),
            b"6Y" => Some(Arrests),
            b"6Z" => Some(PlacementPeriodExpiration),
            b"07" => Some(CumulativeQuantityRejectedMaterial),
            b"7A" => Some(PreviousBenefits),
            b"7B" => Some(QualificationPeriod),
            b"7C" => Some(RangeAverage),
            b"7D" => Some(RangeMaximum),
            b"7E" => Some(RangeMinimum),
            b"7F" => Some(RelationshipDuration),
            b"7G" => Some(ReplacedAmount),
            b"7H" => Some(ResidenceDuration),
            b"7I" => Some(SistersDeceased),
            b"7J" => Some(SistersLiving),
            b"7K" => Some(TimeFrame),
            b"7L" => Some(TimeInCountry),
            b"7M" => Some(TimeSinceHospitalization),
            b"7N" => Some(TimeSinceLastApplication),
            b"7O" => Some(TimeSinceLastCivilianFlight),
            b"7P" => Some(TimeSinceLastInsuranceMedical),
            b"7Q" => Some(TimeSinceLastMilitaryFlight),
            b"7R" => Some(TimeSinceMedicalConsult),
            b"7S" => Some(TimeSinceMedicationEnd),
            b"7T" => Some(TimeSinceMedicationStart),
            b"7U" => Some(TimeSinceOnset),
            b"7V" => Some(TimeSinceSurgery),
            b"7W" => Some(TimeSinceTrip),
            b"7X" => Some(TravelFrequency),
            b"7Y" => Some(TravelPeriod),
            b"7Z" => Some(TripDuration),
            b"08" => Some(CumulativeQuantityRejectedMaterialDispositionReplacement),
            b"8A" => Some(VisitationFrequency),
            b"8B" => Some(Weight),
            b"8C" => Some(WeightChangePeriod),
            b"8D" => Some(WorkPeriod),
            b"8E" => Some(ExistenceLimitPeriod),
            b"8F" => Some(Shares),
            b"8G" => Some(Directors),
            b"8H" => Some(Minimum),
            b"8I" => Some(VotingSharesHeld),
            b"8J" => Some(OutstandingShares),
            b"8K" => Some(SharesHeldAsTreasuryStock),
            b"8L" => Some(SharesSubscribedButNotIssued),
            b"8M" => Some(TotalSharesOfStock),
            b"8N" => Some(SharesOwnedByInStateResidents),
            b"8O" => Some(SharesOwnedByOutOfStateResidents),
            b"8P" => Some(Partners),
            b"8Q" => Some(LandHolding),
            b"8R" => Some(NonDomesticStockholders),
            b"8S" => Some(SharesSubscribed),
            b"8T" => Some(MaximumNumberFreeMiles),
            b"8U" => Some(Code8U),
            b"8V" => Some(Code8V),
            b"8W" => Some(FullTimeEquivalents),
            b"8X" => Some(TotalCreditHours),
            b"8Y" => Some(TotalNonCreditHours),
            b"8Z" => Some(TotalContactHours),
            b"09" => Some(CumulativeQuantityRejectedMaterialDispositionCredit),
            b"9A" => Some(TimeExpended),
            b"9C" => Some(PrimaryMeterReadingValue),
            b"9D" => Some(EngineeredStandard),
            b"9E" => Some(ActiveMaintenanceTime),
            b"9F" => Some(ActualDuration),
            b"9H" => Some(EstimatedDuration),
            b"9J" => Some(GrossEstimate),
            b"9K" => Some(FinishOffset),
            b"9L" => Some(StartOffset),
            b"9M" => Some(PictureCount),
            b"9N" => Some(ComponentMeterReadingCount),
            b"9P" => Some(TotalClockHours),
            b"9R" => Some(Enrollees),
            b"9S" => Some(TotalDaysSubmitted),
            b"9V" => Some(TotalDaysApproved),
            b"10" => Some(CumulativeQuantityRejectedMaterialDispositionPending),
            b"11" => Some(SplitQuantity),
            b"12" => Some(ShipNoticeQuantity),
            b"13" => Some(CollateralRequirements),
            b"14" => Some(QuantityInFloat),
            b"15" => Some(QuantityInHoldOut),
            b"16" => Some(LineThreadQuantity),
            b"17" => Some(QuantityOnHand),
            b"18" => Some(PreviousWeekQuantity),
            b"19" => Some(UnverifiedReceipts),
            b"20" => Some(UnusableQuantity),
            b"21" => Some(CumulativeQuantityShippedShortDispositionPending),
            b"22" => Some(CumulativeQuantityShippedShortDispositionChallenged),
            b"23" => Some(CumulativeQuantityShippedLongDispositionPending),
            b"24" => Some(CumulativeQuantityShippedLongDispositionChallenged),
            b"25" => Some(OemInventory),
            b"26" => Some(TotalInventory),
            b"27" => Some(CommittedQuantity),
            b"28" => Some(QuantityAvailableForReturn),
            b"29" => Some(ProjectedAvailableInventory),
            b"30" => Some(QuoteQuantityOnInventory),
            b"31" => Some(AdditionalDemandQuantity),
            b"32" => Some(QuantitySold),
            b"33" => Some(Code33),
            b"34" => Some(NoncommittedInventoryOnShelf),
            b"35" => Some(InventoryOnShelfWorkInProgress),
            b"36" => Some(DistributorInventory),
            b"37" => Some(WorkInProcess),
            b"38" => Some(OriginalQuantity),
            b"39" => Some(ShippedQuantity),
            b"40" => Some(RemainingQuantity),
            b"41" => Some(NumberOfBatches),
            b"42" => Some(NumberOfChecks),
            b"43" => Some(TalkPaths),
            b"44" => Some(NumberOfPatientAdmissions),
            b"45" => Some(CumulativeQuantityOnOrder),
            b"46" => Some(TotalTransactions),
            b"47" => Some(PrimaryNetQuantity),
            b"48" => Some(SecondaryNetQuantity),
            b"49" => Some(NumberOfSignedBillsOfLading),
            b"50" => Some(NumberOfCopiesOfBillOfLading),
            b"51" => Some(NumberOfUnsignedBillsOfLading),
            b"52" => Some(NumberOfOriginals),
            b"53" => Some(OriginalPaymentItemCount),
            b"54" => Some(BankRejectItemCount),
            b"55" => Some(NetToPayItemCount),
            b"56" => Some(MinimumContractQuantity),
            b"57" => Some(MinimumOrderQuantity),
            b"58" => Some(PaymentCancellationItemCount),
            b"59" => Some(IndividualsWithDevelopmentalDelay),
            b"60" => Some(TotalAuthorizedQuantity),
            b"61" => Some(RemainingAuthorizedQuantity),
            b"62" => Some(NumberOfDaysCoveredByInventory),
            b"63" => Some(OnOrderQuantity),
            b"64" => Some(PastDueQuantity),
            b"65" => Some(PreviousMonthsUsage),
            b"66" => Some(MinimumFabricationQuantity),
            b"67" => Some(MinimumShipQuantity),
            b"68" => Some(MaximumNumberOfShipmentsAllowed),
            b"69" => Some(IncrementalOrderQuantity),
            b"70" => Some(MaximumOrderQuantity),
            b"71" => Some(EducableMentallyRetardedIndividuals),
            b"72" => Some(MinimumStockLevel),
            b"73" => Some(MaximumStockLevel),
            b"74" => Some(DamagedGoods),
            b"75" => Some(Receipts),
            b"76" => Some(Returns),
            b"77" => Some(StockTransfersIn),
            b"78" => Some(StockTransfersOut),
            b"79" => Some(Code79),
            b"80" => Some(Code80),
            b"81" => Some(PrepaidQuantityShipped),
            b"82" => Some(PrepaidQuantityNotShipped),
            b"83" => Some(SubmittedQuantitySold),
            b"84" => Some(SubmittedQuantityReturned),
            b"85" => Some(LotSize),
            b"86" => Some(NonconformanceQuantity),
            b"87" => Some(QuantityReceived),
            b"88" => Some(Beds),
            b"89" => Some(OperatingBeds),
            b"90" => Some(AcknowledgedQuantity),
            b"91" => Some(AdditionalUsageQuantity),
            b"92" => Some(AllottedUsageQuantity),
            b"93" => Some(AttendantHandledQuantity),
            b"94" => Some(BillableQuantity),
            b"95" => Some(DataStorageQuantity),
            b"96" => Some(NonBillableQuantity),
            b"97" => Some(NonUrgentDeliveryQuantity),
            b"98" => Some(OverflowQuantity),
            b"99" => Some(QuantityUsed),
            b"A0" => Some(SeverelyMentallyRetardedIndividuals),
            b"A1" => Some(AcceptableUnserviceableQuantity),
            b"A2" => Some(OptimisticDuration),
            b"A3" => Some(MostLikelyDuration),
            b"A4" => Some(PessimisticDuration),
            b"A5" => Some(AdjustedQuantity),
            b"A6" => Some(Accidents),
            b"A7" => Some(YearsInSchool),
            b"A8" => Some(NumberOfDependents),
            b"A9" => Some(YearsOnJob),
            b"AA" => Some(UnacknowledgedQuantity),
            b"AB" => Some(UrgentDeliveryQuantity),
            b"AC" => Some(VoiceStorageQuantity),
            b"AD" => Some(MaintenanceUnits),
            b"AE" => Some(CodeAE),
            b"AF" => Some(CodeAF),
            b"AG" => Some(NumberOfEndUsers),
            b"AH" => Some(NumberOfMessageRecipients),
            b"AI" => Some(NumberOfOperatorCredits),
            b"AJ" => Some(DailyAdjustments),
            b"AK" => Some(YearsInThisLineOfWorkProfession),
            b"AL" => Some(AreaPerUnits),
            b"AM" => Some(TrainableMentallyRetardedIndividuals),
            b"AN" => Some(AgeAtDeath),
            b"AO" => Some(VerifiedReceipts),
            b"AP" => Some(OrderQuantityMultiple),
            b"AQ" => Some(ContributionTotal),
            b"AR" => Some(LoanRepaymentTotal),
            b"AS" => Some(ParticipantTotal),
            b"AT" => Some(Actual),
            b"AU" => Some(CumulativeActual),
            b"AV" => Some(Budget),
            b"AW" => Some(CumulativeBudget),
            b"AX" => Some(NumberOfInsuredLives),
            b"AY" => Some(Forecast),
            b"AZ" => Some(ForecastAtComplete),
            b"B1" => Some(NumberOfMortgagors),
            b"B2" => Some(MortgagePoolCount),
            b"B3" => Some(RequestedAmount),
            b"B4" => Some(ApprovedAmount),
            b"B5" => Some(AdditionalAmount),
            b"B6" => Some(PreOpDays),
            b"B7" => Some(PostOpDays),
            b"B8" => Some(Average),
            b"B9" => Some(PeriodBeginningImbalanceQuantity),
            b"BA" => Some(DueIn),
            b"BB" => Some(ContractorCumulativeToDate),
            b"BC" => Some(BudgetAtComplete),
            b"BD" => Some(ContractorAtComplete),
            b"BE" => Some(SubcontractorCumulativeToDate),
            b"BF" => Some(AgeModifyingUnits),
            b"BG" => Some(SubcontractorAtComplete),
            b"BH" => Some(BookOrderQuantity),
            b"BI" => Some(BookInventory),
            b"BJ" => Some(BedroomCount),
            b"BK" => Some(BathroomCount),
            b"BL" => Some(BettermentHours),
            b"BM" => Some(DepreciationHours),
            b"BN" => Some(SystemAdjustedHours),
            b"BO" => Some(UserAdjustedHours),
            b"BP" => Some(PeriodEndingImbalanceQuantity),
            b"BQ" => Some(BackorderQuantity),
            b"BR" => Some(BloodRecord),
            b"BS" => Some(CumulativeBeginningImbalanceQuantity),
            b"BT" => Some(CumulativeCurrentPeriodImbalanceQuantity),
            b"BU" => Some(CumulativePriorPeriodAdjustment),
            b"BV" => Some(CumulativeEndingImbalanceQuantity),
            b"BW" => Some(BirthWeight),
            b"BX" => Some(CurrentPeriodImbalanceQuantity),
            b"BY" => Some(ProductionDeliveryQuantity),
            b"BZ" => Some(EntitlementQuantity),
            b"C0" => Some(Creditors),
            b"C1" => Some(PaymentExperiencesInLast12Months),
            b"C2" => Some(PaymentExperiencesInLast3Months),
            b"C3" => Some(AreaDamaged),
            b"C4" => Some(OtherUnlistedStockholders),
            b"C5" => Some(OtherUnlistedParticipants),
            b"CA" => Some(CoveredActual),
            b"CB" => Some(ClosingStatementBalance),
            b"CC" => Some(CurrentDaysOnMarket),
            b"CD" => Some(CoInsuredActual),
            b"CE" => Some(CoveredEstimated),
            b"CF" => Some(CoInsuredEstimated),
            b"CG" => Some(CumulativeGasVolume),
            b"CH" => Some(CumulativeEffectOfPriorPeriodAdjustment),
            b"CI" => Some(CumulativeGasInjectionVolume),
            b"CL" => Some(CumulativeLiquidInjectionVolume),
            b"CM" => Some(NumberOfComponents),
            b"CN" => Some(ContinuanceDuration),
            b"CO" => Some(CumulativeOilCondensateVolume),
            b"CP" => Some(CurrentPeriodImbalance),
            b"CR" => Some(CodeCR),
            b"CS" => Some(CurrentServiceLife),
            b"CW" => Some(CumulativeWaterVolume),
            b"CY" => Some(ConvictionsSent),
            b"CZ" => Some(TotalNumberOfConvictions),
            b"D0" => Some(Engineers),
            b"D1" => Some(Billed),
            b"D2" => Some(Executives),
            b"D3" => Some(NumberOfCoInsuranceDays),
            b"D4" => Some(FieldWorkers),
            b"D5" => Some(Installers),
            b"D6" => Some(MembersInGroup),
            b"D7" => Some(NonConsolidatedTotalDomesticSubsidiaries),
            b"D8" => Some(NonConsolidatedTotalForeignSubsidiaries),
            b"D9" => Some(NonUnionEmployees),
            b"DA" => Some(DependentsAge),
            b"DB" => Some(DeductibleBloodUnits),
            b"DC" => Some(DependentCount),
            b"DD" => Some(Distributed),
            b"DE" => Some(Debited),
            b"DF" => Some(Deleted),
            b"DG" => Some(GasUsedForDrilling),
            b"DH" => Some(MaximumBenefitPeriodAccidentToAge),
            b"DI" => Some(Disposed),
            b"DJ" => Some(MaximumBenefitPeriodSicknessToAge),
            b"DK" => Some(AirlineAttendants),
            b"DL" => Some(CompaniesIncludedInConsolidation),
            b"DM" => Some(TotalConsolidatedDomesticSubsidiaries),
            b"DN" => Some(DefaultNotificationResponsePeriod),
            b"DO" => Some(DaysOperated),
            b"DP" => Some(DaysProduced),
            b"DQ" => Some(TotalConsolidatedForeignSubsidiaries),
            b"DR" => Some(DirectWorkers),
            b"DS" => Some(Dose),
            b"DT" => Some(DependentTotal),
            b"DU" => Some(CounterClerks),
            b"DV" => Some(DesignCapacity),
            b"DW" => Some(DomesticAffiliatedCompanies),
            b"DX" => Some(Drivers),
            b"DY" => Some(Days),
            b"DZ" => Some(EmployedAtLocation),
            b"E1" => Some(CourseSegments),
            b"E2" => Some(DegreeSegments),
            b"E3" => Some(EmployedOnThisJob),
            b"E4" => Some(EmployedInThisProfession),
            b"E5" => Some(EmployedByThisCompany),
            b"E8" => Some(NumberOfEntitledExemptions),
            b"E9" => Some(NumberOfWithholdingExemptions),
            b"EA" => Some(ExclusiveUses),
            b"EB" => Some(NonexclusiveUses),
            b"EC" => Some(UseOfExtracorporealCirculation),
            b"ED" => Some(DomesticUses),
            b"EE" => Some(SmallBusinessUses),
            b"EF" => Some(Nurses),
            b"EG" => Some(OfficeWorkers),
            b"EH" => Some(PaidInCommonShares),
            b"EI" => Some(PaidInPreferredShares),
            b"EJ" => Some(Pilots),
            b"EK" => Some(PlantWorkers),
            b"EL" => Some(PrincipalsIncludedAsEmployees),
            b"EM" => Some(EmergencyModifyingUnits),
            b"EN" => Some(Suppliers),
            b"EO" => Some(Teachers),
            b"EP" => Some(ProductExchangeAmount),
            b"EQ" => Some(EquitySecurityHolder),
            b"ER" => Some(EstimatedRemainingEconomicLife),
            b"ES" => Some(EndingStock),
            b"ET" => Some(EmployeeTotal),
            b"EU" => Some(TotalConsolidatedSubsidiaries),
            b"EV" => Some(TotalNonConsolidatedSubsidiaries),
            b"EW" => Some(EvaporatedWater),
            b"EX" => Some(UnionEmployees),
            b"EY" => Some(PortedTelephoneLines),
            b"EZ" => Some(ServiceResale),
            b"F0" => Some(TotalClaimsWithSkinDiseasesOrDisorders),
            b"F1" => Some(OffLeaseFuel),
            b"F3" => Some(TotalDeathsAsAResultOfInjury),
            b"F4" => Some(TotalDeathsAsAResultOfIllness),
            b"F5" => Some(TotalInjuryClaimsWithDaysAwayFromWorkOrRestrictedWorkActivity),
            b"F6" => Some(TotalInjuryClaimsWithDaysAwayFromWork),
            b"F7" => Some(TotalInjuryClaimsWithoutLostWorkDays),
            b"F8" => Some(TotalDaysAwayFromWorkDueToInjury),
            b"F9" => Some(TotalDaysWithRestrictedWorkActivityDueToInjury),
            b"FA" => Some(FullBaths),
            b"FB" => Some(FurnishedBloodUnits),
            b"FC" => Some(FuelConsumedOrBurnedAmount),
            b"FD" => Some(VehicularRadios),
            b"FE" => Some(PortableRadios),
            b"FF" => Some(FlareOrFlash),
            b"FG" => Some(MarineRadios),
            b"FH" => Some(Pagers),
            b"FI" => Some(ConventionalMobiles),
            b"FJ" => Some(TrunkedChannels),
            b"FK" => Some(MobileLoadingAllocation),
            b"FL" => Some(Units),
            b"FM" => Some(AircraftRadios),
            b"FN" => Some(TotalClaimsWithDustDiseasesOfTheLungs),
            b"FO" => Some(TotalClaimsWithRespiratoryConditionsDueToToxicAgents),
            b"FP" => Some(TotalClaimsWithPoisoningIllnesses),
            b"FQ" => Some(TotalClaimsWithDisordersDueToPhysicalAgents),
            b"FS" => Some(GasUsedForFuelSystem),
            b"FT" => Some(ForecastToComplete),
            b"FU" => Some(TotalClaimsAssociatedWithRepeatedTrauma),
            b"FV" => {
                Some(TotalIllnessClaimsWithOccupationalIllnessesNotOtherwiseClassified)
            }
            b"FW" => Some(TotalDaysAwayFromWorkDueToIllness),
            b"FX" => Some(TotalDaysOfRestrictedWorkActivityDueToIllness),
            b"FY" => Some(TotalIllnessWithLostWorkDaysOrRestrictedWorkActivity),
            b"FZ" => Some(TotalIllnessClaimsWithDaysAwayFromWork),
            b"G0" => Some(DischargeQuantity),
            b"G1" => Some(EstimatedDischargeQuantity),
            b"G2" => Some(EstimatedTransferQuantity),
            b"G3" => Some(Excursions),
            b"G4" => Some(NonProductionQuantity),
            b"G5" => Some(NumberOfDeaths),
            b"G6" => Some(NumberOfHospitalizations),
            b"G7" => Some(NumberOfInjuries),
            b"G8" => Some(NumberOfInjuriesRequiringMedicalTreatment),
            b"G9" => Some(NumberOfPeopleEvacuated),
            b"GA" => Some(GrossBuildingArea),
            b"GB" => Some(GrossAnnualIncomeMultiplier),
            b"GC" => Some(GrossLivingArea),
            b"GD" => Some(TotalIllnessClaimsWithoutLostWorkDays),
            b"GE" => Some(OriginalTermInYears),
            b"GF" => Some(YearsRemaining),
            b"GG" => Some(AverageNumberOfEmployees),
            b"GH" => Some(TotalWorkedByAllEmployees),
            b"GI" => Some(GasInjectionVolume),
            b"GL" => Some(GasLiftVolume),
            b"GM" => Some(Episode),
            b"GN" => Some(CodeGN),
            b"GO" => Some(CodeGO),
            b"GP" => Some(GrossProduction),
            b"GQ" => Some(GovernmentReportingQuantity),
            b"GR" => Some(GasReceiptVolume),
            b"GS" => Some(GasSold),
            b"GT" => Some(GradeTransferAmount),
            b"GU" => Some(EmployeeTotalFirstMonthOfQuarter),
            b"GV" => Some(GasVolume),
            b"GW" => Some(EmployeeTotalSecondMonthOfQuarter),
            b"GX" => Some(EmployeeTotalThirdMonthOfQuarter),
            b"GZ" => Some(ActiveListings),
            b"H0" => Some(NumberOfPeopleShelteredInPlace),
            b"H1" => Some(QuantityRecovered),
            b"H2" => Some(QuantityRecycled),
            b"H3" => Some(QuantityReleased),
            b"H4" => Some(QuantityTreated),
            b"H5" => Some(TotalHazardousWasteGenerated),
            b"H6" => Some(OperationalQuantity),
            b"H7" => Some(PenaltyVarianceQuantity),
            b"H8" => Some(AllocatedQuantity),
            b"H9" => Some(ScheduledQuantity),
            b"HA" => Some(MarketPriceChange),
            b"HB" => Some(Unpaid),
            b"HC" => Some(Branches),
            b"HD" => Some(Subsidiaries),
            b"HE" => Some(AgeOfFinancial),
            b"HF" => Some(Invoices),
            b"HG" => Some(FinancialCoveragePeriod),
            b"HH" => Some(MaximumNumberOfEmployeesAtLocation),
            b"HI" => Some(PreviousNumberOfAccounts),
            b"HJ" => Some(CollectionPeriod),
            b"HK" => Some(DisbursementPeriod),
            b"HL" => Some(Seats),
            b"HM" => Some(UseOfHypothermia),
            b"HN" => Some(PreviousNumberOfEmployees),
            b"HO" => Some(UseOfHypotension),
            b"HP" => Some(UseOfHyperbaricPressurization),
            b"HQ" => Some(KindergartenStudents),
            b"HR" => Some(UseOfHypertension),
            b"HS" => Some(Hours),
            b"HT" => Some(EmployeesAge),
            b"HU" => Some(EmployeesNumberOfDaysAwayFromWorkDueToInjury),
            b"HV" => Some(EmployeesNumberOfDaysOfRestrictedWorkActivityDueToInjury),
            b"HW" => Some(EmployeesTotalNumberOfDaysAwayFromWorkDueToIllness),
            b"HY" => Some(TotalDeathClaims),
            b"HZ" => Some(TotalClaimsWithDaysAwayFromWork),
            b"I0" => Some(TenthGradeStudents),
            b"I1" => Some(EleventhGradeStudents),
            b"I2" => Some(TwelfthGradeStudents),
            b"I3" => Some(PriorTeachingExperience),
            b"I4" => Some(PriorFullTimeTeachingExperience),
            b"I5" => Some(PriorPartTimeTeachingExperience),
            b"I6" => Some(PriorExperienceInEducation),
            b"I7" => Some(PriorFullTimeExperienceInEducation),
            b"I8" => Some(PriorPartTimeExperienceInEducation),
            b"I9" => Some(PriorExperienceRelatedToJob),
            b"IA" => Some(LocalCountryEmployees),
            b"IB" => Some(ForeignEmployees),
            b"IC" => Some(PriorFullTimeExperienceRelatedToJob),
            b"ID" => Some(PriorPartTimeExperienceRelatedToJob),
            b"IE" => Some(TotalPriorExperience),
            b"IF" => Some(TotalFullTimePriorExperience),
            b"IG" => Some(TotalPartTimePriorExperience),
            b"IH" => Some(TotalYearsOfEducationalService),
            b"II" => Some(NumberOfIrregularInterestPayments),
            b"IJ" => Some(TotalYearsOfEducationalServiceInThisDistrict),
            b"IK" => Some(YearsOfExperienceAsSchoolPrincipal),
            b"IL" => Some(YearsOfExperienceAsClassroomTeacher),
            b"IM" => Some(YearsWorkedForThisSystem),
            b"IN" => Some(IndirectWorkers),
            b"IP" => Some(NumberOfInterestPayments),
            b"IQ" => Some(InTransitQuantity),
            b"IS" => Some(InformationProviderStandardizedMotorVehiclePenaltyPoints),
            b"IT" => Some(IntertankTransferAmount),
            b"J0" => Some(EndingStorageBalance),
            b"J1" => Some(LocationEndingStorageBalance),
            b"J2" => Some(LocationEndingStorageBalanceFirm),
            b"J3" => Some(LocationEndingStorageBalanceInterruptible),
            b"J4" => Some(MaximumAvailableDailyInjectionQuantity),
            b"J5" => Some(MaximumAvailableDailyWithdrawalQuantity),
            b"J6" => Some(MinimumRequiredDailyInjectionQuantity),
            b"J7" => Some(MinimumRequiredDailyWithdrawalQuantity),
            b"JA" => Some(ActivityCodes),
            b"JB" => Some(Associates),
            b"JC" => Some(AverageEmployees),
            b"JD" => Some(CooperativeShares),
            b"JE" => Some(EstimatedEmployeesAtLocation),
            b"JF" => Some(EstimatedTotalEmployees),
            b"JG" => Some(FinancialInstitutions),
            b"JH" => Some(Judgments),
            b"JI" => Some(LandSize),
            b"JJ" => Some(Liens),
            b"JK" => Some(MinimumEmployeesAtLocation),
            b"JL" => Some(OfficeSize),
            b"JM" => Some(Owner),
            b"JN" => Some(PlantSize),
            b"JO" => Some(PreviousNumberOfBranches),
            b"JP" => Some(ProtestedBills),
            b"JQ" => Some(Suits),
            b"JR" => Some(CodeJR),
            b"JS" => Some(JudicialStayDuration),
            b"JT" => Some(WarehouseSize),
            b"JU" => Some(TotalDaysAwayFromWork),
            b"JV" => Some(TotalDaysOfRestrictedWorkActivity),
            b"JW" => {
                Some(TotalClaimsWithoutDaysAwayFromWorkAndWithoutRestrictedWorkActivity)
            }
            b"JX" => Some(Secretaries),
            b"JY" => Some(Mechanics),
            b"JZ" => Some(Auditors),
            b"K1" => Some(Messengers),
            b"K2" => Some(PrimaryManagers),
            b"K3" => Some(ParticipationShares),
            b"K4" => Some(DetrimentalLegalFilings),
            b"K5" => Some(PetitionsFiled),
            b"K6" => Some(Drafts),
            b"K7" => Some(BusinessFailureNationalAverageIncidence),
            b"K8" => Some(BusinessFailureIndustryIncidence),
            b"K9" => Some(BusinessFailureClassIncidence),
            b"KA" => Some(Estimated),
            b"KB" => Some(NetQuantityIncrease),
            b"KC" => Some(NetQuantityDecrease),
            b"KD" => Some(ExpenditureQuantity),
            b"KE" => Some(Originals),
            b"KF" => Some(Duplicates),
            b"KG" => Some(CompletedLineItems),
            b"KH" => Some(CompletedContracts),
            b"KI" => Some(ActiveContractsDelinquentBuyingPartyCaused),
            b"KJ" => Some(ActiveContractsDelinquent),
            b"KK" => Some(ActiveContractsDelinquentContractorCaused),
            b"KL" => Some(ActiveContractsDelinquentUnknownCauses),
            b"KM" => Some(ActiveLineItemsDelinquent),
            b"KN" => Some(ActiveLineItemsDelinquentBuyingPartyCaused),
            b"KO" => Some(ActiveLineItemsDelinquentContractorCaused),
            b"KP" => Some(ActiveLineItemsDelinquentUnknownCauses),
            b"KQ" => Some(ContractsCompletedDelinquentBuyingPartyCaused),
            b"KR" => Some(ContractCompletedDelinquentContractorCaused),
            b"KS" => Some(ContractsCompletedDelinquentUnknownCauses),
            b"KU" => Some(ReportedDeficiencies),
            b"KV" => Some(LineItemsCompletedDelinquentBuyingPartyCaused),
            b"KW" => Some(LineItemsCompletedDelinquentContractorCaused),
            b"KX" => Some(LineItemsCompletedDelinquentUnknownCauses),
            b"KY" => Some(CorrectiveActionRequestsVerbal),
            b"KZ" => Some(CorrectiveActionRequestsWritten),
            b"L2" => Some(GuaranteeFeeBuyupMaximum),
            b"L3" => Some(ContractBuyup),
            b"L4" => Some(ContractBuydown),
            b"L5" => Some(GuaranteeFeeRateAfterAlternatePaymentMethod),
            b"L6" => Some(GuaranteeFeeRateAfterBuyupOrBuydown),
            b"L7" => Some(BuyupOrBuydownRatePerBasisPoint),
            b"L8" => Some(LocationNetCapacity),
            b"L9" => Some(SubjectToLossOrElimination),
            b"LA" => Some(LifeTimeReserveActual),
            b"LB" => Some(LossAllowance),
            b"LC" => Some(LatePaymentPeriod),
            b"LD" => Some(LimitValue),
            b"LE" => Some(LifeTimeReserveEstimated),
            b"LG" => Some(LossOrGain),
            b"LH" => Some(LostGas),
            b"LI" => Some(LiquidInjectionVolume),
            b"LK" => Some(CorrectiveActionRequestsMethodC),
            b"LL" => Some(CorrectiveActionRequestsMethodD),
            b"LM" => Some(CorrectiveActionRequestsMethodE),
            b"LN" => Some(AgedActiveLineItemsDelinquentContractorCaused),
            b"LO" => Some(LostOil),
            b"LP" => Some(LeasePeriods),
            b"LQ" => Some(AgedLineItemsDelinquent),
            b"LR" => Some(AgedLineItemsCompletedContractorCaused),
            b"LS" => Some(OilCondensateSold),
            b"LT" => Some(TariffLossAllowance),
            b"LU" => Some(LifetimeReserveDaysAppliedToThisClaim),
            b"LV" => Some(OilCondensateVolume),
            b"LW" => Some(LostWorkTimeActual),
            b"LX" => Some(LostWorkTimeEstimated),
            b"LY" => Some(LengthOfResidency),
            b"LZ" => Some(Lanes),
            b"M1" => Some(MatchingEquipment),
            b"M2" => Some(Maximum),
            b"M3" => Some(TotalFederalPoints),
            b"M4" => Some(Contributions),
            b"M5" => Some(Contributors),
            b"M6" => Some(Endorsers),
            b"M7" => Some(Functions),
            b"M8" => Some(Guarantors),
            b"M9" => Some(Points),
            b"MA" => Some(MiscellaneousAllowance),
            b"MB" => Some(NumberOfPublicOfficials),
            b"MC" => Some(TotalNonFederalPoints),
            b"MD" => Some(MillionDollarRoundtableCredits),
            b"ME" => Some(MinimumNumberOfEmployees),
            b"MF" => Some(Manufactured),
            b"MG" => Some(Pledges),
            b"MH" => Some(TotalPoints),
            b"MI" => Some(Miles),
            b"MJ" => Some(Attendees),
            b"MK" => Some(TicketsSold),
            b"ML" => Some(TotalNumberOfManifestLines),
            b"MM" => Some(MaximumMaturityExtension),
            b"MN" => Some(Month),
            b"MO" => Some(MinimumOrderPackageLevel),
            b"MP" => Some(TotalNumberOfMapsInAPack),
            b"MQ" => Some(MaximumShipQuantity),
            b"MR" => Some(QuantityOfNextLowerLevelTradeItem),
            b"NX" => Some(QuantityOfNextLowerLevelTradeItem),
            b"MS" => Some(MeasuredQuantity),
            b"MT" => Some(ResterilizationMaximum),
            b"MU" => Some(RecommendedNumberOfUses),
            b"MV" => Some(TotalUnits),
            b"MX" => Some(MaximumNumberOfEmployees),
            b"MY" => Some(StackingFactor),
            b"MZ" => Some(ComponentQuantity),
            b"N1" => Some(NumberOfAttacksOrOccurrences),
            b"N2" => Some(NumberOfDead),
            b"N3" => Some(NumberOfLiving),
            b"N4" => Some(NumberOfTimes),
            b"N5" => Some(MinimumForecastQuantity),
            b"N6" => Some(MaximumForecastQuantity),
            b"N7" => Some(RequestedReceiptQuantity),
            b"N8" => Some(RequestedDeliveryQuantity),
            b"NA" => Some(NumberOfNonCoveredDays),
            b"NB" => Some(CodeNB),
            b"NC" => Some(NumberOfClaimants),
            b"ND" => Some(NumberOfLateCharges),
            b"NE" => Some(NonCoveredEstimated),
            b"NF" => Some(NumberOfFullTimeEmployees),
            b"NG" => Some(NumberOfNonsufficientFundItems),
            b"NH" => Some(NoncoveredActual),
            b"NL" => Some(NumberOfLevels),
            b"NN" => Some(NumberOfHospitals),
            b"NO" => Some(NumberOfPhysicians),
            b"NP" => Some(NumberOfMembers),
            b"NQ" => Some(NumberOfFranchisees),
            b"NR" => Some(NotReplacedBloodUnits),
            b"NS" => Some(NumberOfStations),
            b"NT" => Some(Reports),
            b"NU" => Some(SinceLastTravel),
            b"NV" => Some(Net),
            b"NW" => Some(UntilNextTravel),
            b"O1" => Some(ScheduledReceipt),
            b"O2" => Some(ScheduledDelivery),
            b"O3" => Some(OperationalReceipt),
            b"O4" => Some(OperationalDelivery),
            b"O5" => Some(AllocatedReceipt),
            b"O6" => Some(AllocatedDelivery),
            b"O7" => Some(DistributedConfirmedReceipt),
            b"O8" => Some(DistributedConfirmedDelivery),
            b"O9" => Some(SchedulingToleranceReceipt),
            b"OA" => Some(SchedulingToleranceDelivery),
            b"OB" => Some(Energy),
            b"OC" => Some(OrderCount),
            b"OD" => Some(OtherMiscellaneousDisposition),
            b"OE" => Some(NumberOfWeeksPerYear),
            b"OF" => Some(OffPremiseSalesQuantity),
            b"OG" => Some(OtherGasDisposition),
            b"OH" => Some(OtherInjectionVolume),
            b"OI" => Some(OpeningStatementBalance),
            b"OJ" => Some(OrderSizingFactor),
            b"OL" => Some(OriginalLoanTerm),
            b"ON" => Some(OnPremiseSalesQuantity),
            b"OO" => Some(OtherOilCondensateDisposition),
            b"OQ" => Some(OptimumOrderQuantity),
            b"OR" => Some(Original),
            b"OT" => Some(NumberOfOperatingPeriodsAtFailure),
            b"OU" => Some(OutlierDays),
            b"OV" => Some(Overage),
            b"OW" => Some(OtherWaterDisposition),
            b"P1" => Some(ProjectPhases),
            b"P3" => Some(PhysicalStatusIii),
            b"P4" => Some(PhysicalStatusIv),
            b"P5" => Some(PhysicalStatusV),
            b"P6" => Some(NumberOfServicesOrProcedures),
            b"P7" => Some(PrescriptionDosage),
            b"P8" => Some(PrescriptionFrequency),
            b"P9" => Some(NumberOfPeopleLivingAtResidence),
            b"PA" => Some(PipelineAdjustmentOrAllowance),
            b"PB" => Some(PressureBase),
            b"PC" => Some(PriorCumulativeImbalance),
            b"PD" => Some(PaymentDurationWeeks),
            b"PE" => Some(PeriodOfEmployment),
            b"PF" => Some(GasUsedForPlantFuel),
            b"PG" => Some(Persistency),
            b"PH" => Some(Promotional),
            b"PK" => Some(ParkingSpaces),
            b"PL" => Some(PartialBaths),
            b"PO" => Some(PercentageOfOrderedQuantity),
            b"PP" => Some(PurchaseOfProduct),
            b"PQ" => Some(CumulativeQuantityRequiredPriorToTheFirstScheduledPeriod),
            b"PR" => Some(RequirementQuantityThatWasPreviouslyReleased),
            b"PS" => Some(Prescription),
            b"PT" => Some(Patients),
            b"PW" => Some(PittedWater),
            b"PX" => Some(PriorUnitsAccepted),
            b"PY" => Some(Paid),
            b"Q1" => Some(MinimumQuantityToWhichTaxRateApplies),
            b"Q2" => Some(MaximumQuantityToWhichTaxRateApplies),
            b"Q3" => Some(QuantityEarned),
            b"Q4" => Some(QuantityCarriedForward),
            b"Q5" => Some(NumberOf3To4YearOlds),
            b"Q6" => Some(AutisticIndividuals),
            b"Q7" => Some(DeafBlindIndividuals),
            b"Q8" => Some(HearingImpairedIndividuals),
            b"Q9" => Some(MentallyRetardedIndividuals),
            b"QA" => Some(QuantityApproved),
            b"QB" => Some(QuantityDispensed),
            b"QC" => Some(QuantityDisapproved),
            b"QD" => Some(QuantityDelivered),
            b"QE" => Some(QuantityDeferred),
            b"QF" => Some(HighFabricationAuthorizationQuantity),
            b"QH" => Some(QuantityOnHold),
            b"QI" => Some(CommunityServiceDuration),
            b"QJ" => Some(NumberOfTimesDeported),
            b"QK" => Some(QuantityOfInnerPacks),
            b"QL" => Some(JailSentenceDuration),
            b"QM" => Some(ProbationDuration),
            b"QN" => Some(RestrictionDuration),
            b"QO" => Some(OperatingQuantity),
            b"QP" => Some(QuantityByPosition),
            b"QQ" => Some(SuspendedDuration),
            b"QR" => Some(HighRawMaterialAuthorizationQuantity),
            b"QS" => Some(QuantityPerSkid),
            b"QT" => Some(PlantThermalReduction),
            b"QU" => Some(QuantityServiced),
            b"QV" => Some(QuantityCancelled),
            b"QW" => Some(QuantityWithdrawn),
            b"QX" => Some(QualifyingWeeks),
            b"QY" => Some(RepaymentPlanTerm),
            b"R1" => Some(CodeR1),
            b"R2" => Some(IndividualsWithOrthopedicImpairment),
            b"R3" => Some(EstimatedRemainingPhysicalLife),
            b"R4" => Some(IndividualsWithSpecificLearningDisability),
            b"R5" => Some(Axles),
            b"R6" => Some(PlatformCount),
            b"R7" => Some(IndividualsWithVisualImpairment),
            b"R8" => Some(IndividualsWithOtherHealthImpairment),
            b"R9" => Some(Fuel),
            b"RA" => Some(RefillsAuthorized),
            b"RB" => Some(ReplacedBloodUnits),
            b"RC" => Some(NumberOfItemsAuthorizedAtStore),
            b"RD" => Some(NumberOfItemsAuthorizedAtWarehouse),
            b"RE" => Some(GasReturnedToEarth),
            b"RF" => Some(NumberOfItemsInStock),
            b"RG" => Some(GasUsedForRepressuringOrPressureMaintenance),
            b"RH" => Some(NumberOfShelfTags),
            b"RJ" => Some(QuantityAvailableOnShelf),
            b"RL" => Some(GasReturnedToPropertyForFuel),
            b"RM" => Some(RoomCount),
            b"RN" => Some(UnitsRented),
            b"RP" => Some(RetailDemandQuantity),
            b"RQ" => Some(Royalty),
            b"RS" => Some(NumberOfShelfFacings),
            b"RT" => Some(RetailSalesQuantity),
            b"RW" => Some(WaterReInjectedOnProperty),
            b"RY" => Some(RequirementQuantity),
            b"S1" => Some(CodeS1),
            b"S2" => Some(CodeS2),
            b"S3" => Some(DwellingArea),
            b"S4" => Some(GarageOrCarportArea),
            b"S5" => Some(UnitsForSale),
            b"S6" => Some(GrossRentMultiplier),
            b"S7" => Some(CodeS7),
            b"S8" => Some(CodeS8),
            b"S9" => Some(CodeS9),
            b"SA" => Some(Shipments),
            b"SB" => Some(Solicited),
            b"SC" => Some(CodeSC),
            b"SD" => Some(CriminalSentenceDuration),
            b"SE" => Some(CodeSE),
            b"SF" => Some(Site),
            b"SG" => Some(SwanGanz),
            b"SH" => Some(Shortage),
            b"SI" => Some(Rooms),
            b"SJ" => Some(AreaOfLevel),
            b"SK" => Some(GasShrinkage),
            b"SL" => Some(PredominateAge),
            b"SM" => Some(MinimumCriminalSentenceDuration),
            b"SN" => Some(Age),
            b"SO" => Some(OilSedimentation),
            b"SP" => Some(DaysSupply),
            b"SQ" => Some(ProductSalesAmount),
            b"SR" => Some(EffectiveAge),
            b"SS" => Some(SharesOfPreferredStock),
            b"ST" => Some(Standard),
            b"SU" => Some(ForecastedScannedQuantity),
            b"SV" => Some(SharesOfCommonStock),
            b"SW" => Some(SampleAmount),
            b"SX" => Some(MaximumCriminalSentenceDuration),
            b"SY" => Some(StateOrProvinceMotorVehiclePenaltyPoints),
            b"SZ" => Some(Seasonal),
            b"T1" => Some(TimeUnitsKnown),
            b"T2" => Some(TimeUnitsSpentOnDuty),
            b"T3" => Some(TotalDaysOnMarket),
            b"T4" => Some(TotalRooms),
            b"T5" => Some(TotalNumberOfUnits),
            b"T6" => Some(TotalNumberOfUnitsForSale),
            b"T7" => Some(Tires),
            b"TA" => Some(TankAllowance),
            b"TB" => Some(OilTheft),
            b"TC" => Some(TotalAtComplete),
            b"TD" => Some(TotalToDate),
            b"TE" => Some(NumberOfTheatres),
            b"TG" => Some(TotalGasInjectionVolume),
            b"TH" => Some(TheoreticalQuantity),
            b"TI" => Some(TotalOilAndOrCondensateInjectionVolume),
            b"TJ" => Some(DurationInCurrentJob),
            b"TK" => Some(TotalOilAndOrCondensateDisposition),
            b"TM" => Some(TotalWaterDisposition),
            b"TN" => Some(TotalBeginningInventory),
            b"TO" => Some(Total),
            b"TP" => Some(TimeInPosition),
            b"TQ" => Some(TotalQuantityOfAllBuys),
            b"TR" => Some(Trips),
            b"TS" => Some(TotalNumberOfParkingSpaces),
            b"TT" => Some(TotalProductionVolume),
            b"TU" => Some(TotalAdjustmentsVolume),
            b"TV" => Some(TotalGasDisposition),
            b"TW" => Some(TotalWaterInjectionVolume),
            b"TX" => Some(TotalEndingInventory),
            b"TY" => Some(TotalSalesVolume),
            b"U1" => Some(FreelanceCollectors),
            b"U2" => Some(BranchLocationsOwned),
            b"U3" => Some(BranchLocationsLeased),
            b"UA" => Some(UnitsCompleted),
            b"UB" => Some(Poultry),
            b"UC" => Some(Livestock),
            b"UD" => Some(Passengers),
            b"UE" => Some(Trainers),
            b"UF" => Some(Operators),
            b"UG" => Some(GasUsedOnProperty),
            b"UH" => Some(Inspectors),
            b"UI" => Some(Collectors),
            b"UJ" => Some(Professionals),
            b"UK" => Some(Supervisors),
            b"UL" => Some(ApproximateNumberOfUnitsForSaleProjected),
            b"UM" => Some(Administrators),
            b"UN" => Some(Promoters),
            b"UO" => Some(OilCondensateUsedOnProperty),
            b"UP" => Some(Divisions),
            b"UQ" => Some(Tables),
            b"UR" => Some(FuelPumps),
            b"US" => Some(InUse),
            b"UT" => Some(Machines),
            b"UU" => Some(Used),
            b"UV" => Some(TrademarksUsed),
            b"UW" => Some(AvailableForCultivation),
            b"UX" => Some(Foremen),
            b"UY" => Some(TravellingEmployees),
            b"UZ" => Some(FreelanceSalespersons),
            b"V1" => Some(RetentionQuantity),
            b"V2" => Some(AvailableQuantity),
            b"V3" => Some(TransferQuantity),
            b"V4" => Some(SurveysInAverageRating),
            b"V5" => Some(Vacancies),
            b"V8" => Some(UnsubscribedCapacity),
            b"V9" => Some(ShippingContainerQuantity),
            b"VA" => Some(VolumeShrinkageAdjustmentOrAllowance),
            b"VB" => Some(BlankVotes),
            b"VC" => Some(CumulativeEarnedValue),
            b"VD" => Some(ScatteredVotes),
            b"VE" => Some(EarnedValue),
            b"VF" => Some(FederalVotes),
            b"VG" => Some(GasVented),
            b"VH" => Some(ScheduleVariance),
            b"VI" => Some(CumulativeScheduleVariance),
            b"VJ" => Some(CumulativeVariance),
            b"VK" => Some(EstimateAtComplete),
            b"VL" => Some(AtCompleteVariance),
            b"VM" => Some(VarianceAdjustment),
            b"VN" => Some(NoVotes),
            b"VP" => Some(PresidentialVotes),
            b"VQ" => Some(UtilizationServiceLife),
            b"VR" => Some(Variance),
            b"VS" => Some(Visits),
            b"VT" => Some(Votes),
            b"VU" => Some(RecommendedServiceLife),
            b"VV" => Some(VoidVotes),
            b"VW" => Some(ShelfLifePeriod),
            b"VY" => Some(YesVotes),
            b"W0" => Some(BankruptcyPetitions),
            b"W1" => Some(Buyers),
            b"W2" => Some(Debentures),
            b"W3" => Some(DebenturesFiledAgainstDirectors),
            b"W4" => Some(DetrimentalLegalFilingsAgainstDirectors),
            b"W5" => Some(FailedBusinessesOfDirectors),
            b"W6" => Some(Professors),
            b"W7" => Some(Sellers),
            b"W8" => Some(SkilledWorkers),
            b"W9" => Some(TrademarksRepresented),
            b"WA" => Some(TotalNumberOfWorkersCompensationFirstReports),
            b"WB" => Some(TotalNumberOfWorkersCompensationSubsequentReports),
            b"WC" => Some(TotalNumberOfWorkersCompensationCombinedReports),
            b"WD" => Some(UnitsWorkedPerDay),
            b"WE" => Some(LimitedQuantity),
            b"WG" => Some(WeightGain),
            b"WL" => Some(WeightLoss),
            b"WO" => Some(OperatorsWorkingInterest),
            b"WP" => Some(NumberOfProducingWellsRemainingOnPropertyOrFacility),
            b"WR" => Some(NumberOfProducingWellsRemainingOnRoyaltyAccount),
            b"WT" => Some(TotalWorkingInterest),
            b"WV" => Some(WaterVolume),
            b"WW" => Some(WeeksWorked),
            b"WX" => Some(LicenseWithdrawalDuration),
            b"WY" => Some(LicenseWithdrawalsSent),
            b"X1" => Some(ProducingWells),
            b"X2" => Some(Gross),
            b"X6" => Some(AssessmentHours),
            b"X7" => Some(DutyDays),
            b"X8" => Some(ContractDays),
            b"X9" => Some(NumberOfDaysEmployed),
            b"XA" => Some(TotalOfIssuableAssets),
            b"XB" => Some(CodeXB),
            b"XC" => Some(CodeXC),
            b"XD" => Some(CodeXD),
            b"XE" => Some(CodeXE),
            b"XG" => Some(OnHandAndDueIn),
            b"XI" => Some(InstallmentPayments),
            b"XJ" => Some(CodeXJ),
            b"XL" => Some(ApproximateNumberOfUnitsProjected),
            b"XN" => Some(ApproximateNumberOfHolders),
            b"XO" => Some(CirculatingOil),
            b"XR" => Some(StockObjectiveAndInsuranceQuantity),
            b"XT" => Some(ProtectedQuantity),
            b"XU" => Some(Reserved),
            b"XV" => Some(RequisitioningObjective),
            b"XX" => Some(AuthorizedRetentionLevel),
            b"XY" => Some(SafetyLevel),
            b"XZ" => Some(BackorderLines),
            b"Y1" => Some(NumberOfLostCards),
            b"Y2" => Some(NumberOfStolenCards),
            b"Y3" => Some(NumberOfCardsNotReceived),
            b"Y4" => Some(NumberOfActiveAccountsThisCycle),
            b"Y5" => Some(NumberOfOpenAccounts),
            b"Y6" => Some(NumberOfAccountsPastDue),
            b"Y7" => Some(NumberOfCardsOutstanding),
            b"Y8" => Some(OnHandPlusPipeline),
            b"YA" => Some(TotalDemandQuantity),
            b"YB" => Some(TotalDemandOrders),
            b"YC" => Some(FirstQuarterRecurringDemand),
            b"YD" => Some(FirstQuarterRecurringOrders),
            b"YE" => Some(FirstQuarterNonRecurringDemand),
            b"YF" => Some(FirstQuarterNonRecurringOrders),
            b"YG" => Some(SecondQuarterRecurringDemand),
            b"YH" => Some(SecondQuarterRecurringOrders),
            b"YJ" => Some(SecondQuarterNonRecurringDemand),
            b"YK" => Some(SecondQuarterNonRecurringOrders),
            b"YL" => Some(ThirdQuarterRecurringDemand),
            b"YM" => Some(ThirdQuarterRecurringOrders),
            b"YN" => Some(ThirdQuarterNonRecurringDemand),
            b"YP" => Some(ThirdQuarterNonRecurringOrders),
            b"YQ" => Some(FourthQuarterRecurringDemand),
            b"YR" => Some(FourthQuarterRecurringOrders),
            b"YS" => Some(FourthQuarterNonRecurringDemand),
            b"YT" => Some(FourthQuarterNonRecurringOrders),
            b"YU" => Some(Trailers),
            b"YW" => Some(ReorderPointQuantity),
            b"YX" => Some(ContractLineItemQuantity),
            b"YY" => Some(Years),
            b"YZ" => Some(MaximumQuantityOfFreeServiceCalls),
            b"Z1" => Some(UnitsWorkedLastDay),
            b"Z2" => Some(UnitsWorkedPerWeek),
            b"Z3" => Some(UnitsWorkedPerQuarter),
            b"Z4" => Some(NumberWeeksPaid),
            b"Z6" => Some(UnusedAccumulatedSickDays),
            b"Z7" => Some(DeliveryPointReductionQuantity),
            b"Z8" => Some(ReceiptPointReductionQuantity),
            b"Z9" => Some(ReductionQuantity),
            b"ZA" => Some(FederalMedicareOrMedicaidClaimMandateCategory1),
            b"ZB" => Some(FederalMedicareOrMedicaidClaimMandateCategory2),
            b"ZC" => Some(FederalMedicareOrMedicaidClaimMandateCategory3),
            b"ZD" => Some(FederalMedicareOrMedicaidClaimMandateCategory4),
            b"ZE" => Some(FederalMedicareOrMedicaidClaimMandateCategory5),
            b"ZF" => Some(FederalPensionMandateCategory1),
            b"ZG" => Some(FederalPensionMandateCategory2),
            b"ZH" => Some(FederalPensionMandateCategory3),
            b"ZI" => Some(HoldingPeriod),
            b"ZJ" => Some(FederalPensionMandateCategory5),
            b"ZK" => Some(FederalMedicareOrMedicaidPaymentMandateCategory1),
            b"ZL" => Some(FederalMedicareOrMedicaidPaymentMandateCategory2),
            b"ZM" => Some(FederalMedicareOrMedicaidPaymentMandateCategory3),
            b"ZN" => Some(FederalMedicareOrMedicaidPaymentMandateCategory4),
            b"ZO" => Some(FederalMedicareOrMedicaidPaymentMandateCategory5),
            b"ZP" => Some(FederalPensionMandateCategory4),
            b"ZQ" => Some(SharesAdded),
            b"ZR" => Some(ExtendedTerm),
            b"ZS" => Some(AmortizationTerm),
            b"ZT" => Some(BeginningShares),
            b"ZU" => Some(SharesDeleted),
            b"ZV" => Some(QuantityOfDealerLicensePlates),
            b"ZW" => Some(CurrentShareBalance),
            b"ZX" => Some(SizeOfHousehold),
            b"ZY" => Some(ProjectUnitsSold),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use QuantityQualifier::*;
        match self {
            HospitalHomeboundIndividuals => "Hospital/Homebound Individuals",
            NumberOfHoursPerDay => "Number of Hours Per Day",
            NumberOfHoursPerWeek => "Number of Hours Per Week",
            NumberOfMonthsPerYear => "Number of Months Per Year",
            NumberOfPeriodsPerWeek => "Number of Periods Per Week",
            ExpectedExpenditureQuantity => "Expected Expenditure Quantity",
            NumberOfHoursPerYear => "Number of Hours Per Year",
            PreKindergartenStudents => "Pre-Kindergarten Students",
            FirstGradeStudents => "First Grade Students",
            SecondGradeStudents => "Second Grade Students",
            ThirdGradeStudents => "Third Grade Students",
            FourthGradeStudents => "Fourth Grade Students",
            FifthGradeStudents => "Fifth Grade Students",
            SixthGradeStudents => "Sixth Grade Students",
            SeventhGradeStudents => "Seventh Grade Students",
            EighthGradeStudents => "Eighth Grade Students",
            NinthGradeStudents => "Ninth Grade Students",
            CarnegieUnits => "Carnegie Units",
            NumberOfDisabilityTypes => "Number of Disability Types",
            NumberOfMales => "Number of Males",
            NumberOfFemales => "Number of Females",
            IndividualsWithMultipleDisabilities => {
                "Individuals with Multiple Disabilities"
            }
            IndividualsWithSeriousEmotionalDisturbance => {
                "Individuals with Serious Emotional Disturbance"
            }
            IndividualsWithSpeechOrLanguageImpairment => {
                "Individuals with Speech or Language Impairment"
            }
            IndividualsWithTraumaticBrainInjury => {
                "Individuals with Traumatic Brain Injury"
            }
            BlindIndividuals => "Blind Individuals",
            DeafIndividuals => "Deaf Individuals",
            DiscreteQuantity => "Discrete Quantity",
            Code1A => "Original Duration (in calendar units)",
            Code1B => "Current Duration (in calendar units)",
            Code1C => "Remaining Duration (in calendar units)",
            Code1D => "Total Float (in calendar units)",
            Code1E => "Free Float (in calendar units)",
            Code1F => "Lag (as in Lag Time - in calendar units)",
            Code1G => "Lead Time (in calendar units)",
            Started => "Started",
            Completed => "Completed",
            Due => "Due",
            TimeUnits => "Time Units",
            Shifts => "Shifts",
            TimeUnitsPerShift => "Time units per shift",
            ScrapAllowed => "Scrap allowed",
            CalendarUnits => "Calendar Units",
            Code1P => "Resource (Quantity) available",
            Code1Q => "Total Resource (Quantity)",
            Code1R => "Level Resource (Quantity)",
            Late => "Late",
            NumberOfDelinquentInstallments => "Number of Delinquent Installments",
            NumberOfLoans => "Number of Loans",
            TotalNumberOfMortgagees => "Total Number of Mortgagees",
            TotalNumberOfLoanDetailRecords => "Total Number of Loan Detail Records",
            PrescriptionEffectivePeriod => "Prescription Effective Period",
            Code1Y => "Rate Per Day (RPD)",
            EndOfMonthInventoryPriorToShip => "End Of Month Inventory Prior To Ship",
            CumulativeQuantity => "Cumulative Quantity",
            CommitmentPeriod => "Commitment Period",
            NumberOfBorrowers => "Number of Borrowers",
            NumberOfAdjustmentPeriods => "Number of Adjustment Periods",
            AgeNearest => "Age Nearest",
            TotalOtherPropertiesOwnedAndFinanced => {
                "Total Other Properties Owned and Financed"
            }
            AgeNext => "Age Next",
            ReconsiderationPeriod => "Reconsideration Period",
            FlatExtraPremium => "Flat Extra Premium",
            Co2InjectionVolume => "CO2 Injection Volume",
            AccountsPlacedForCollection => "Accounts Placed for Collection",
            Changes => "Changes",
            CompaniesInSameActivityForAPeriod => {
                "Companies in Same Activity for a Period"
            }
            ComparisonPeriod => "Comparison Period",
            Departments => "Departments",
            EmployeesShared => "Employees Shared",
            EstimatedAccounts => "Estimated Accounts",
            InstalledCapacity => "Installed Capacity",
            LevelsOccupied => "Levels Occupied",
            RegisteredBrandsDistributed => "Registered Brands Distributed",
            ElectronicSignatures => "Electronic Signatures",
            Bytes => "Bytes",
            EmployedAtThisLocation => "Employed at this Location",
            Segments => "Segments",
            RegisteredBrandsManufactured => "Registered Brands Manufactured",
            FunctionalGroups => "Functional Groups",
            TransactionSets => "Transaction Sets",
            DiscreetQuantityRejectedMaterial => "Discreet Quantity - Rejected Material",
            TotalCreditsAccepted => "Total Credits Accepted",
            TotalCreditsRejected => "Total Credits Rejected",
            TotalDebitsAccepted => "Total Debits Accepted",
            TotalDebitsRejected => "Total Debits Rejected",
            TotalPaymentsRejected => "Total Payments Rejected",
            TotalPreAdvicesAccepted => "Total Pre-advices Accepted",
            TotalPreAdvicesRejected => "Total Pre-advices Rejected",
            TotalPrenotesAccepted => "Total Prenotes Accepted",
            TotalPrenotesRejected => "Total Prenotes Rejected",
            TotalPostAdvicesAccepted => "Total Post-advices Accepted",
            TotalPostAdvicesRejected => "Total Post-advices Rejected",
            TotalUnidentifiedTransactionsRejected => {
                "Total Unidentified Transactions Rejected"
            }
            TotalCreditsReceived => "Total Credits Received",
            TotalDebitsReceived => "Total Debits Received",
            IndividualsWithNoncategoricalPreschoolDisability => {
                "Individuals with Noncategorical Preschool Disability"
            }
            TotalPreAdvicesReceived => "Total Pre-advices Received",
            TotalPrenotesReceived => "Total Prenotes Received",
            TotalPostAdvicesReceived => "Total Post-advices Received",
            TotalDebits => "Total Debits",
            TotalCredits => "Total Credits",
            MinimumTransfer => "Minimum Transfer",
            MaximumTransfer => "Maximum Transfer",
            SpeedCapacity => "Speed Capacity",
            Subcontractors => "Subcontractors",
            Students => "Students",
            DiscreteQuantityRejectedMaterialDispositionReplacement => {
                "Discrete Quantity - Rejected Material: Disposition Replacement"
            }
            Accounts => "Accounts",
            Agents => "Agents",
            AuthorizedShares => "Authorized Shares",
            Clerks => "Clerks",
            DesignEmployees => "Design Employees",
            ForeignRelatedEntities => "Foreign Related Entities",
            GroupEmployees => "Group Employees",
            IssuedShares => "Issued Shares",
            Laborers => "Laborers",
            OtherEmployeeType => "Other Employee Type",
            PartTimeEmployees => "Part Time Employees",
            RelatedEntities => "Related Entities",
            RelativesEmployed => "Relatives Employed",
            Salespersons => "Salespersons",
            SpaceOccupied => "Space Occupied",
            SpecialPartners => "Special Partners",
            SuppliersCredit => "Suppliers' Credit",
            Technicians => "Technicians",
            Trainees => "Trainees",
            WarehouseEmployees => "Warehouse Employees",
            Shareholders => "Shareholders",
            AvailableUnits => "Available Units",
            TotalUnduplicatedHeadcount => "Total Unduplicated Headcount",
            MealsPerWeek => "Meals Per Week",
            ProgramsOffered => "Programs Offered",
            Code4Z => "Typical Credit Hours Taken per Graduate Student (Full Time)",
            DiscreteQuantityRejectedMaterialDispositionCredit => {
                "Discrete Quantity - Rejected Material: Disposition Credit"
            }
            AggregateBenefitPeriod => "Aggregate Benefit Period",
            AnticipatedLengthOfService => "Anticipated Length of Service",
            ApprovalOfferDuration => "Approval/Offer Duration",
            BenefitAmount => "Benefit Amount",
            BenefitPeriod => "Benefit Period",
            BrothersDeceased => "Brothers Deceased",
            BrothersLiving => "Brothers Living",
            Children => "Children",
            Citations => "Citations",
            ClaimPeriod => "Claim Period",
            Coverage => "Coverage",
            EliminationPeriod => "Elimination Period",
            EliminationPeriodAccident => "Elimination Period - Accident",
            EliminationPeriodSickness => "Elimination Period - Sickness",
            EmployeesNonowner => "Employees - Nonowner",
            EmployeesOwner => "Employees - Owner",
            EmployeesPartTime => "Employees - Part Time",
            EmployeesSameDuties => "Employees - Same Duties",
            EmployeesSameOccupation => "Employees - Same Occupation",
            Expense => "Expense",
            Frequency => "Frequency",
            GeneralEliminationPeriod => "General Elimination Period",
            GuaranteePeriod => "Guarantee Period",
            Height => "Height",
            HoursFlownAircraftTypeLife => "Hours Flown - Aircraft Type/Life",
            HoursFlownAircraftTypePeriod => "Hours Flown - Aircraft Type/Period",
            DiscreteQuantityRejectedMaterialDispositionPending => {
                "Discrete Quantity - Rejected Material: Disposition Pending"
            }
            HoursFlownAircraftTypeFlying => "Hours Flown - Aircraft/Type Flying",
            HoursFlownLifetime => "Hours Flown - Lifetime",
            HoursFlownTypeFlying => "Hours Flown - Type Flying",
            ImpairmentDuration => "Impairment Duration",
            ImpairmentFrequency => "Impairment Frequency",
            InstallmentFrequency => "Installment Frequency",
            Installments => "Installments",
            IntendedChangeTimePeriod => "Intended Change Time Period",
            InterimTermPeriod => "Interim Term Period",
            InvolvementPeriod => "Involvement Period",
            LoanRate => "Loan Rate",
            MaximumAge => "Maximum Age",
            MaximumBenefitPeriodAccident => "Maximum Benefit Period - Accident",
            MaximumBenefitPeriodSickness => "Maximum Benefit Period - Sickness",
            MaximumBenefitPeriod => "Maximum Benefit Period",
            MedicationDuration => "Medication Duration",
            MinimumAge => "Minimum Age",
            OwnOccupationQualificationPeriod => "Own Occupation Qualification Period",
            OwnersEquity => "Owner's Equity",
            OwnershipChangeAge => "Ownership Change Age",
            OwnershipDuration => "Ownership Duration",
            OwnershipPercentage => "Ownership Percentage",
            PaymentFrequency => "Payment Frequency",
            PaymentsNumber => "Payments Number",
            Arrests => "Arrests",
            PlacementPeriodExpiration => "Placement Period Expiration",
            CumulativeQuantityRejectedMaterial => {
                "Cumulative Quantity - Rejected Material"
            }
            PreviousBenefits => "Previous Benefits",
            QualificationPeriod => "Qualification Period",
            RangeAverage => "Range Average",
            RangeMaximum => "Range Maximum",
            RangeMinimum => "Range Minimum",
            RelationshipDuration => "Relationship Duration",
            ReplacedAmount => "Replaced Amount",
            ResidenceDuration => "Residence Duration",
            SistersDeceased => "Sisters Deceased",
            SistersLiving => "Sisters Living",
            TimeFrame => "Time Frame",
            TimeInCountry => "Time in Country",
            TimeSinceHospitalization => "Time Since Hospitalization",
            TimeSinceLastApplication => "Time Since Last Application",
            TimeSinceLastCivilianFlight => "Time Since Last Civilian Flight",
            TimeSinceLastInsuranceMedical => "Time Since Last Insurance Medical",
            TimeSinceLastMilitaryFlight => "Time Since Last Military Flight",
            TimeSinceMedicalConsult => "Time Since Medical Consult",
            TimeSinceMedicationEnd => "Time Since Medication End",
            TimeSinceMedicationStart => "Time Since Medication Start",
            TimeSinceOnset => "Time Since Onset",
            TimeSinceSurgery => "Time Since Surgery",
            TimeSinceTrip => "Time Since Trip",
            TravelFrequency => "Travel Frequency",
            TravelPeriod => "Travel Period",
            TripDuration => "Trip Duration",
            CumulativeQuantityRejectedMaterialDispositionReplacement => {
                "Cumulative Quantity - Rejected Material: Disposition Replacement"
            }
            VisitationFrequency => "Visitation Frequency",
            Weight => "Weight",
            WeightChangePeriod => "Weight Change Period",
            WorkPeriod => "Work Period",
            ExistenceLimitPeriod => "Existence Limit Period",
            Shares => "Shares",
            Directors => "Directors",
            Minimum => "Minimum",
            VotingSharesHeld => "Voting Shares Held",
            OutstandingShares => "Outstanding Shares",
            SharesHeldAsTreasuryStock => "Shares Held as Treasury Stock",
            SharesSubscribedButNotIssued => "Shares Subscribed but Not Issued",
            TotalSharesOfStock => "Total Shares of Stock",
            SharesOwnedByInStateResidents => "Shares Owned by In-State Residents",
            SharesOwnedByOutOfStateResidents => "Shares Owned by Out-of-State Residents",
            Partners => "Partners",
            LandHolding => "Land Holding",
            NonDomesticStockholders => "Non-Domestic Stockholders",
            SharesSubscribed => "Shares Subscribed",
            MaximumNumberFreeMiles => "Maximum Number Free Miles",
            Code8U => "Typical Credit Hours Taken per Undergraduate Student (Full Time)",
            Code8V => {
                "Typical Credit Hours Taken per First-Professional Student (Full Time)"
            }
            FullTimeEquivalents => "Full-time Equivalents",
            TotalCreditHours => "Total Credit Hours",
            TotalNonCreditHours => "Total Non-Credit Hours",
            TotalContactHours => "Total Contact Hours",
            CumulativeQuantityRejectedMaterialDispositionCredit => {
                "Cumulative Quantity - Rejected Material: Disposition Credit"
            }
            TimeExpended => "Time Expended",
            PrimaryMeterReadingValue => "Primary Meter Reading Value",
            EngineeredStandard => "Engineered Standard",
            ActiveMaintenanceTime => "Active Maintenance Time",
            ActualDuration => "Actual Duration",
            EstimatedDuration => "Estimated Duration",
            GrossEstimate => "Gross Estimate",
            FinishOffset => "Finish Offset",
            StartOffset => "Start Offset",
            PictureCount => "Picture Count",
            ComponentMeterReadingCount => "Component Meter Reading Count",
            TotalClockHours => "Total Clock Hours",
            Enrollees => "Enrollees",
            TotalDaysSubmitted => "Total Days Submitted",
            TotalDaysApproved => "Total Days Approved",
            CumulativeQuantityRejectedMaterialDispositionPending => {
                "Cumulative Quantity - Rejected Material: Disposition Pending"
            }
            SplitQuantity => "Split Quantity",
            ShipNoticeQuantity => "Ship Notice Quantity",
            CollateralRequirements => "Collateral Requirements",
            QuantityInFloat => "Quantity in Float",
            QuantityInHoldOut => "Quantity in Hold Out",
            LineThreadQuantity => "Line Thread Quantity",
            QuantityOnHand => "Quantity on Hand",
            PreviousWeekQuantity => "Previous Week Quantity",
            UnverifiedReceipts => "Unverified Receipts",
            UnusableQuantity => "Unusable Quantity",
            CumulativeQuantityShippedShortDispositionPending => {
                "Cumulative Quantity Shipped Short - Disposition Pending"
            }
            CumulativeQuantityShippedShortDispositionChallenged => {
                "Cumulative Quantity Shipped Short - Disposition Challenged"
            }
            CumulativeQuantityShippedLongDispositionPending => {
                "Cumulative Quantity Shipped Long - Disposition Pending"
            }
            CumulativeQuantityShippedLongDispositionChallenged => {
                "Cumulative Quantity Shipped Long - Disposition Challenged"
            }
            OemInventory => "OEM Inventory",
            TotalInventory => "Total Inventory",
            CommittedQuantity => "Committed Quantity",
            QuantityAvailableForReturn => "Quantity Available for Return",
            ProjectedAvailableInventory => "Projected Available Inventory",
            QuoteQuantityOnInventory => "Quote Quantity on Inventory",
            AdditionalDemandQuantity => "Additional Demand Quantity",
            QuantitySold => "Quantity Sold",
            Code33 => "Quantity Available for Sale (stock quantity)",
            NoncommittedInventoryOnShelf => "Noncommitted Inventory on Shelf",
            InventoryOnShelfWorkInProgress => "Inventory on Shelf + Work in Progress",
            DistributorInventory => "Distributor Inventory",
            WorkInProcess => "Work In Process",
            OriginalQuantity => "Original Quantity",
            ShippedQuantity => "Shipped Quantity",
            RemainingQuantity => "Remaining Quantity",
            NumberOfBatches => "Number of Batches",
            NumberOfChecks => "Number of Checks",
            TalkPaths => "Talk Paths",
            NumberOfPatientAdmissions => "Number of Patient Admissions",
            CumulativeQuantityOnOrder => "Cumulative quantity on order",
            TotalTransactions => "Total transactions",
            PrimaryNetQuantity => "Primary Net Quantity",
            SecondaryNetQuantity => "Secondary Net Quantity",
            NumberOfSignedBillsOfLading => "Number of Signed Bills of Lading",
            NumberOfCopiesOfBillOfLading => "Number of Copies of Bill of Lading",
            NumberOfUnsignedBillsOfLading => "Number of Unsigned Bills of Lading",
            NumberOfOriginals => "Number of Originals",
            OriginalPaymentItemCount => "Original payment item count.",
            BankRejectItemCount => "Bank reject item count.",
            NetToPayItemCount => "Net to pay item count.",
            MinimumContractQuantity => "Minimum Contract Quantity",
            MinimumOrderQuantity => "Minimum Order Quantity",
            PaymentCancellationItemCount => "Payment Cancellation Item Count",
            IndividualsWithDevelopmentalDelay => "Individuals with Developmental Delay",
            TotalAuthorizedQuantity => "Total Authorized Quantity",
            RemainingAuthorizedQuantity => "Remaining Authorized Quantity",
            NumberOfDaysCoveredByInventory => "Number of Days Covered by Inventory",
            OnOrderQuantity => "On Order Quantity",
            PastDueQuantity => "Past Due Quantity",
            PreviousMonthsUsage => "Previous Month's Usage",
            MinimumFabricationQuantity => "Minimum Fabrication Quantity",
            MinimumShipQuantity => "Minimum Ship Quantity",
            MaximumNumberOfShipmentsAllowed => "Maximum Number of Shipments Allowed",
            IncrementalOrderQuantity => "Incremental Order Quantity",
            MaximumOrderQuantity => "Maximum Order Quantity",
            EducableMentallyRetardedIndividuals => {
                "Educable Mentally Retarded Individuals"
            }
            MinimumStockLevel => "Minimum Stock Level",
            MaximumStockLevel => "Maximum Stock Level",
            DamagedGoods => "Damaged Goods",
            Receipts => "Receipts",
            Returns => "Returns",
            StockTransfersIn => "Stock Transfers In",
            StockTransfersOut => "Stock Transfers Out",
            Code79 => "Billing Unit(s) Per Pricing Unit",
            Code80 => "Pricing Unit(s) Per Billing Unit",
            PrepaidQuantityShipped => "Prepaid Quantity Shipped",
            PrepaidQuantityNotShipped => "Prepaid Quantity Not Shipped",
            SubmittedQuantitySold => "Submitted Quantity Sold",
            SubmittedQuantityReturned => "Submitted Quantity Returned",
            LotSize => "Lot Size",
            NonconformanceQuantity => "Nonconformance Quantity",
            QuantityReceived => "Quantity Received",
            Beds => "Beds",
            OperatingBeds => "Operating Beds",
            AcknowledgedQuantity => "Acknowledged Quantity",
            AdditionalUsageQuantity => "Additional Usage Quantity",
            AllottedUsageQuantity => "Allotted Usage Quantity",
            AttendantHandledQuantity => "Attendant-Handled Quantity",
            BillableQuantity => "Billable Quantity",
            DataStorageQuantity => "Data Storage Quantity",
            NonBillableQuantity => "Non-Billable Quantity",
            NonUrgentDeliveryQuantity => "Non-Urgent Delivery Quantity",
            OverflowQuantity => "Overflow Quantity",
            QuantityUsed => "Quantity Used",
            SeverelyMentallyRetardedIndividuals => {
                "Severely Mentally Retarded Individuals"
            }
            AcceptableUnserviceableQuantity => "Acceptable Unserviceable Quantity",
            OptimisticDuration => "Optimistic Duration",
            MostLikelyDuration => "Most Likely Duration",
            PessimisticDuration => "Pessimistic Duration",
            AdjustedQuantity => "Adjusted Quantity",
            Accidents => "Accidents",
            YearsInSchool => "Years in School",
            NumberOfDependents => "Number of Dependents",
            YearsOnJob => "Years on Job",
            UnacknowledgedQuantity => "Unacknowledged Quantity",
            UrgentDeliveryQuantity => "Urgent Delivery Quantity",
            VoiceStorageQuantity => "Voice Storage Quantity",
            MaintenanceUnits => "Maintenance Units",
            CodeAE => "Minimum Average Time Requirement (MATR) Units",
            CodeAF => "Wide Area Telephone Service (WATS)/800 Service Units",
            NumberOfEndUsers => "Number of End Users",
            NumberOfMessageRecipients => "Number of Message Recipients",
            NumberOfOperatorCredits => "Number of Operator Credits",
            DailyAdjustments => "Daily Adjustments",
            YearsInThisLineOfWorkProfession => "Years in this Line of Work/Profession",
            AreaPerUnits => "Area per Units",
            TrainableMentallyRetardedIndividuals => {
                "Trainable Mentally Retarded Individuals"
            }
            AgeAtDeath => "Age at Death",
            VerifiedReceipts => "Verified Receipts",
            OrderQuantityMultiple => "Order Quantity Multiple",
            ContributionTotal => "Contribution Total",
            LoanRepaymentTotal => "Loan Repayment Total",
            ParticipantTotal => "Participant Total",
            Actual => "Actual",
            CumulativeActual => "Cumulative Actual",
            Budget => "Budget",
            CumulativeBudget => "Cumulative Budget",
            NumberOfInsuredLives => "Number of Insured Lives",
            Forecast => "Forecast",
            ForecastAtComplete => "Forecast at Complete",
            NumberOfMortgagors => "Number of Mortgagors",
            MortgagePoolCount => "Mortgage Pool Count",
            RequestedAmount => "Requested Amount",
            ApprovedAmount => "Approved Amount",
            AdditionalAmount => "Additional Amount",
            PreOpDays => "Pre-op Days",
            PostOpDays => "Post-op Days",
            Average => "Average",
            PeriodBeginningImbalanceQuantity => "Period Beginning Imbalance Quantity",
            DueIn => "Due-In",
            ContractorCumulativeToDate => "Contractor Cumulative to Date",
            BudgetAtComplete => "Budget At Complete",
            ContractorAtComplete => "Contractor at Complete",
            SubcontractorCumulativeToDate => "Subcontractor Cumulative to Date",
            AgeModifyingUnits => "Age Modifying Units",
            SubcontractorAtComplete => "Subcontractor at Complete",
            BookOrderQuantity => "Book Order Quantity",
            BookInventory => "Book Inventory",
            BedroomCount => "Bedroom Count",
            BathroomCount => "Bathroom Count",
            BettermentHours => "Betterment Hours",
            DepreciationHours => "Depreciation Hours",
            SystemAdjustedHours => "System Adjusted Hours",
            UserAdjustedHours => "User Adjusted Hours",
            PeriodEndingImbalanceQuantity => "Period Ending Imbalance Quantity",
            BackorderQuantity => "Backorder Quantity",
            BloodRecord => "Blood Record",
            CumulativeBeginningImbalanceQuantity => {
                "Cumulative Beginning Imbalance Quantity"
            }
            CumulativeCurrentPeriodImbalanceQuantity => {
                "Cumulative Current Period Imbalance Quantity"
            }
            CumulativePriorPeriodAdjustment => "Cumulative Prior Period Adjustment",
            CumulativeEndingImbalanceQuantity => "Cumulative Ending Imbalance Quantity",
            BirthWeight => "Birth Weight",
            CurrentPeriodImbalanceQuantity => "Current Period Imbalance Quantity",
            ProductionDeliveryQuantity => "Production Delivery Quantity",
            EntitlementQuantity => "Entitlement Quantity",
            Creditors => "Creditors",
            PaymentExperiencesInLast12Months => "Payment Experiences in Last 12 Months",
            PaymentExperiencesInLast3Months => "Payment Experiences in Last 3 Months",
            AreaDamaged => "Area Damaged",
            OtherUnlistedStockholders => "Other Unlisted Stockholders",
            OtherUnlistedParticipants => "Other Unlisted Participants",
            CoveredActual => "Covered - Actual",
            ClosingStatementBalance => "Closing Statement Balance",
            CurrentDaysOnMarket => "Current Days on Market",
            CoInsuredActual => "Co-insured - Actual",
            CoveredEstimated => "Covered - Estimated",
            CoInsuredEstimated => "Co-insured - Estimated",
            CumulativeGasVolume => "Cumulative Gas Volume",
            CumulativeEffectOfPriorPeriodAdjustment => {
                "Cumulative Effect of Prior Period Adjustment"
            }
            CumulativeGasInjectionVolume => "Cumulative Gas Injection Volume",
            CumulativeLiquidInjectionVolume => "Cumulative Liquid Injection Volume",
            NumberOfComponents => "Number of Components",
            ContinuanceDuration => "Continuance Duration",
            CumulativeOilCondensateVolume => "Cumulative Oil/Condensate Volume",
            CurrentPeriodImbalance => "Current Period Imbalance",
            CodeCR => {
                "Certified Registered Nurse Anesthetist (CRNA) Number of Concurrent Procedures"
            }
            CurrentServiceLife => "Current Service Life",
            CumulativeWaterVolume => "Cumulative Water Volume",
            ConvictionsSent => "Convictions Sent",
            TotalNumberOfConvictions => "Total Number of Convictions",
            Engineers => "Engineers",
            Billed => "Billed",
            Executives => "Executives",
            NumberOfCoInsuranceDays => "Number of Co-insurance Days",
            FieldWorkers => "Field Workers",
            Installers => "Installers",
            MembersInGroup => "Members in Group",
            NonConsolidatedTotalDomesticSubsidiaries => {
                "Non-Consolidated Total-Domestic Subsidiaries"
            }
            NonConsolidatedTotalForeignSubsidiaries => {
                "Non-Consolidated Total-Foreign Subsidiaries"
            }
            NonUnionEmployees => "Non-Union Employees",
            DependentsAge => "Dependent's Age",
            DeductibleBloodUnits => "Deductible Blood Units",
            DependentCount => "Dependent Count",
            Distributed => "Distributed",
            Debited => "Debited",
            Deleted => "Deleted",
            GasUsedForDrilling => "Gas Used for Drilling",
            MaximumBenefitPeriodAccidentToAge => "Maximum Benefit Period Accident to Age",
            Disposed => "Disposed",
            MaximumBenefitPeriodSicknessToAge => "Maximum Benefit Period Sickness to Age",
            AirlineAttendants => "Airline Attendants",
            CompaniesIncludedInConsolidation => "Companies Included in Consolidation",
            TotalConsolidatedDomesticSubsidiaries => {
                "Total Consolidated Domestic Subsidiaries"
            }
            DefaultNotificationResponsePeriod => "Default Notification Response Period",
            DaysOperated => "Days Operated",
            DaysProduced => "Days Produced",
            TotalConsolidatedForeignSubsidiaries => {
                "Total Consolidated Foreign Subsidiaries"
            }
            DirectWorkers => "Direct Workers",
            Dose => "Dose",
            DependentTotal => "Dependent Total",
            CounterClerks => "Counter Clerks",
            DesignCapacity => "Design Capacity",
            DomesticAffiliatedCompanies => "Domestic Affiliated Companies",
            Drivers => "Drivers",
            Days => "Days",
            EmployedAtLocation => "Employed at Location",
            CourseSegments => "Course Segments",
            DegreeSegments => "Degree Segments",
            EmployedOnThisJob => "Employed on this job",
            EmployedInThisProfession => "Employed in this Profession",
            EmployedByThisCompany => "Employed by this Company",
            NumberOfEntitledExemptions => "Number of Entitled Exemptions",
            NumberOfWithholdingExemptions => "Number of Withholding Exemptions",
            ExclusiveUses => "Exclusive Uses",
            NonexclusiveUses => "Nonexclusive Uses",
            UseOfExtracorporealCirculation => "Use of Extracorporeal Circulation",
            DomesticUses => "Domestic Uses",
            SmallBusinessUses => "Small Business Uses",
            Nurses => "Nurses",
            OfficeWorkers => "Office Workers",
            PaidInCommonShares => "Paid in Common Shares",
            PaidInPreferredShares => "Paid in Preferred Shares",
            Pilots => "Pilots",
            PlantWorkers => "Plant Workers",
            PrincipalsIncludedAsEmployees => "Principals Included as Employees",
            EmergencyModifyingUnits => "Emergency Modifying Units",
            Suppliers => "Suppliers",
            Teachers => "Teachers",
            ProductExchangeAmount => "Product Exchange Amount",
            EquitySecurityHolder => "Equity Security Holder",
            EstimatedRemainingEconomicLife => "Estimated Remaining Economic Life",
            EndingStock => "Ending Stock",
            EmployeeTotal => "Employee Total",
            TotalConsolidatedSubsidiaries => "Total Consolidated Subsidiaries",
            TotalNonConsolidatedSubsidiaries => "Total Non-Consolidated Subsidiaries",
            EvaporatedWater => "Evaporated Water",
            UnionEmployees => "Union Employees",
            PortedTelephoneLines => "Ported Telephone Lines",
            ServiceResale => "Service Resale",
            TotalClaimsWithSkinDiseasesOrDisorders => {
                "Total claims with skin diseases or disorders"
            }
            OffLeaseFuel => "Off Lease Fuel",
            TotalDeathsAsAResultOfInjury => "Total deaths as a Result of Injury",
            TotalDeathsAsAResultOfIllness => "Total deaths as a Result of Illness",
            TotalInjuryClaimsWithDaysAwayFromWorkOrRestrictedWorkActivity => {
                "Total injury Claims with Days Away from Work or Restricted Work Activity"
            }
            TotalInjuryClaimsWithDaysAwayFromWork => {
                "Total injury Claims with Days Away from Work"
            }
            TotalInjuryClaimsWithoutLostWorkDays => {
                "Total injury Claims without Lost Work Days"
            }
            TotalDaysAwayFromWorkDueToInjury => "Total Days Away from Work Due to Injury",
            TotalDaysWithRestrictedWorkActivityDueToInjury => {
                "Total Days with Restricted Work Activity Due to Injury"
            }
            FullBaths => "Full Baths",
            FurnishedBloodUnits => "Furnished Blood Units",
            FuelConsumedOrBurnedAmount => "Fuel Consumed or Burned Amount",
            VehicularRadios => "Vehicular Radios",
            PortableRadios => "Portable Radios",
            FlareOrFlash => "Flare or Flash",
            MarineRadios => "Marine Radios",
            Pagers => "Pagers",
            ConventionalMobiles => "Conventional Mobiles",
            TrunkedChannels => "Trunked Channels",
            MobileLoadingAllocation => "Mobile Loading Allocation",
            Units => "Units",
            AircraftRadios => "Aircraft Radios",
            TotalClaimsWithDustDiseasesOfTheLungs => {
                "Total Claims with Dust Diseases of the Lungs"
            }
            TotalClaimsWithRespiratoryConditionsDueToToxicAgents => {
                "Total Claims with Respiratory Conditions Due to Toxic Agents"
            }
            TotalClaimsWithPoisoningIllnesses => "Total Claims with Poisoning Illnesses",
            TotalClaimsWithDisordersDueToPhysicalAgents => {
                "Total Claims with Disorders Due to Physical Agents"
            }
            GasUsedForFuelSystem => "Gas Used for Fuel System",
            ForecastToComplete => "Forecast to Complete",
            TotalClaimsAssociatedWithRepeatedTrauma => {
                "Total Claims Associated with Repeated Trauma"
            }
            TotalIllnessClaimsWithOccupationalIllnessesNotOtherwiseClassified => {
                "Total illness Claims with occupational illnesses not otherwise classified"
            }
            TotalDaysAwayFromWorkDueToIllness => {
                "Total Days Away from Work Due to Illness"
            }
            TotalDaysOfRestrictedWorkActivityDueToIllness => {
                "Total Days of Restricted Work Activity Due to Illness"
            }
            TotalIllnessWithLostWorkDaysOrRestrictedWorkActivity => {
                "Total illness with Lost Work Days or Restricted Work Activity"
            }
            TotalIllnessClaimsWithDaysAwayFromWork => {
                "Total illness Claims with Days Away from Work"
            }
            DischargeQuantity => "Discharge Quantity",
            EstimatedDischargeQuantity => "Estimated Discharge Quantity",
            EstimatedTransferQuantity => "Estimated Transfer Quantity",
            Excursions => "Excursions",
            NonProductionQuantity => "Non-production Quantity",
            NumberOfDeaths => "Number of Deaths",
            NumberOfHospitalizations => "Number of Hospitalizations",
            NumberOfInjuries => "Number of Injuries",
            NumberOfInjuriesRequiringMedicalTreatment => {
                "Number of Injuries Requiring Medical Treatment"
            }
            NumberOfPeopleEvacuated => "Number of People Evacuated",
            GrossBuildingArea => "Gross Building Area",
            GrossAnnualIncomeMultiplier => "Gross Annual Income Multiplier",
            GrossLivingArea => "Gross Living Area",
            TotalIllnessClaimsWithoutLostWorkDays => {
                "Total illness Claims without Lost Work Days"
            }
            OriginalTermInYears => "Original Term In Years",
            YearsRemaining => "Years Remaining",
            AverageNumberOfEmployees => "Average Number of Employees",
            TotalWorkedByAllEmployees => "Total Worked by All Employees",
            GasInjectionVolume => "Gas Injection Volume",
            GasLiftVolume => "Gas Lift Volume",
            Episode => "Episode",
            CodeGN => "Period(s)",
            CodeGO => "Session(s)",
            GrossProduction => "Gross Production",
            GovernmentReportingQuantity => "Government Reporting Quantity",
            GasReceiptVolume => "Gas Receipt Volume",
            GasSold => "Gas Sold",
            GradeTransferAmount => "Grade Transfer Amount",
            EmployeeTotalFirstMonthOfQuarter => "Employee Total First Month of Quarter",
            GasVolume => "Gas Volume",
            EmployeeTotalSecondMonthOfQuarter => "Employee Total Second Month of Quarter",
            EmployeeTotalThirdMonthOfQuarter => "Employee Total Third Month of Quarter",
            ActiveListings => "Active Listings",
            NumberOfPeopleShelteredInPlace => "Number of People Sheltered-in-Place",
            QuantityRecovered => "Quantity Recovered",
            QuantityRecycled => "Quantity Recycled",
            QuantityReleased => "Quantity Released",
            QuantityTreated => "Quantity Treated",
            TotalHazardousWasteGenerated => "Total Hazardous Waste Generated",
            OperationalQuantity => "Operational Quantity",
            PenaltyVarianceQuantity => "Penalty Variance Quantity",
            AllocatedQuantity => "Allocated Quantity",
            ScheduledQuantity => "Scheduled Quantity",
            MarketPriceChange => "Market Price Change",
            Unpaid => "Unpaid",
            Branches => "Branches",
            Subsidiaries => "Subsidiaries",
            AgeOfFinancial => "Age of Financial Information",
            Invoices => "Invoices",
            FinancialCoveragePeriod => "Financial Coverage Period",
            MaximumNumberOfEmployeesAtLocation => {
                "Maximum Number of Employees at Location"
            }
            PreviousNumberOfAccounts => "Previous Number of Accounts",
            CollectionPeriod => "Collection Period",
            DisbursementPeriod => "Disbursement Period",
            Seats => "Seats",
            UseOfHypothermia => "Use of Hypothermia",
            PreviousNumberOfEmployees => "Previous Number of Employees",
            UseOfHypotension => "Use of Hypotension",
            UseOfHyperbaricPressurization => "Use of Hyperbaric Pressurization",
            KindergartenStudents => "Kindergarten Students",
            UseOfHypertension => "Use of Hypertension",
            Hours => "Hours",
            EmployeesAge => "Employee's Age",
            EmployeesNumberOfDaysAwayFromWorkDueToInjury => {
                "Employee's Number of Days Away from Work Due to Injury"
            }
            EmployeesNumberOfDaysOfRestrictedWorkActivityDueToInjury => {
                "Employee's Number of Days of Restricted Work Activity Due to Injury"
            }
            EmployeesTotalNumberOfDaysAwayFromWorkDueToIllness => {
                "Employee's Total Number of Days Away from Work Due to Illness"
            }
            TotalDeathClaims => "Total Death Claims",
            TotalClaimsWithDaysAwayFromWork => "Total Claims with Days Away from Work",
            TenthGradeStudents => "Tenth Grade Students",
            EleventhGradeStudents => "Eleventh Grade Students",
            TwelfthGradeStudents => "Twelfth Grade Students",
            PriorTeachingExperience => "Prior Teaching Experience",
            PriorFullTimeTeachingExperience => "Prior Full-time Teaching Experience",
            PriorPartTimeTeachingExperience => "Prior Part-time Teaching Experience",
            PriorExperienceInEducation => "Prior Experience in Education",
            PriorFullTimeExperienceInEducation => {
                "Prior Full-time Experience in Education"
            }
            PriorPartTimeExperienceInEducation => {
                "Prior Part-time Experience in Education"
            }
            PriorExperienceRelatedToJob => "Prior Experience Related to Job",
            LocalCountryEmployees => "Local Country Employees",
            ForeignEmployees => "Foreign Employees",
            PriorFullTimeExperienceRelatedToJob => {
                "Prior Full-time Experience Related to Job"
            }
            PriorPartTimeExperienceRelatedToJob => {
                "Prior Part-time Experience Related to Job"
            }
            TotalPriorExperience => "Total Prior Experience",
            TotalFullTimePriorExperience => "Total Full-time Prior Experience",
            TotalPartTimePriorExperience => "Total Part-time Prior Experience",
            TotalYearsOfEducationalService => "Total Years of Educational Service",
            NumberOfIrregularInterestPayments => "Number of Irregular Interest Payments",
            TotalYearsOfEducationalServiceInThisDistrict => {
                "Total Years of Educational Service in this District"
            }
            YearsOfExperienceAsSchoolPrincipal => {
                "Years of Experience as School Principal"
            }
            YearsOfExperienceAsClassroomTeacher => {
                "Years of Experience as Classroom Teacher"
            }
            YearsWorkedForThisSystem => "Years Worked for this System",
            IndirectWorkers => "Indirect Workers",
            NumberOfInterestPayments => "Number of Interest Payments",
            InTransitQuantity => "In-Transit Quantity",
            InformationProviderStandardizedMotorVehiclePenaltyPoints => {
                "Information Provider Standardized Motor Vehicle Penalty Points"
            }
            IntertankTransferAmount => "Intertank Transfer Amount",
            EndingStorageBalance => "Ending Storage Balance",
            LocationEndingStorageBalance => "Location Ending Storage Balance",
            LocationEndingStorageBalanceFirm => "Location Ending Storage Balance - Firm",
            LocationEndingStorageBalanceInterruptible => {
                "Location Ending Storage Balance - Interruptible"
            }
            MaximumAvailableDailyInjectionQuantity => {
                "Maximum Available Daily Injection Quantity"
            }
            MaximumAvailableDailyWithdrawalQuantity => {
                "Maximum Available Daily Withdrawal Quantity"
            }
            MinimumRequiredDailyInjectionQuantity => {
                "Minimum Required Daily Injection Quantity"
            }
            MinimumRequiredDailyWithdrawalQuantity => {
                "Minimum Required Daily Withdrawal Quantity"
            }
            ActivityCodes => "Activity Codes",
            Associates => "Associates",
            AverageEmployees => "Average Employees",
            CooperativeShares => "Cooperative Shares",
            EstimatedEmployeesAtLocation => "Estimated Employees at Location",
            EstimatedTotalEmployees => "Estimated Total Employees",
            FinancialInstitutions => "Financial Institutions",
            Judgments => "Judgments",
            LandSize => "Land Size",
            Liens => "Liens",
            MinimumEmployeesAtLocation => "Minimum Employees at Location",
            OfficeSize => "Office Size",
            Owner => "Owner",
            PlantSize => "Plant Size",
            PreviousNumberOfBranches => "Previous Number of Branches",
            ProtestedBills => "Protested Bills",
            Suits => "Suits",
            CodeJR => "Uniform Commercial Code (UCC) Filings",
            JudicialStayDuration => "Judicial Stay Duration",
            WarehouseSize => "Warehouse Size",
            TotalDaysAwayFromWork => "Total Days Away from Work",
            TotalDaysOfRestrictedWorkActivity => "Total Days of Restricted Work Activity",
            TotalClaimsWithoutDaysAwayFromWorkAndWithoutRestrictedWorkActivity => {
                "Total Claims without Days Away from Work and without Restricted Work Activity"
            }
            Secretaries => "Secretaries",
            Mechanics => "Mechanics",
            Auditors => "Auditors",
            Messengers => "Messengers",
            PrimaryManagers => "Primary Managers",
            ParticipationShares => "Participation Shares",
            DetrimentalLegalFilings => "Detrimental Legal Filings",
            PetitionsFiled => "Petitions Filed",
            Drafts => "Drafts",
            BusinessFailureNationalAverageIncidence => {
                "Business Failure National Average Incidence"
            }
            BusinessFailureIndustryIncidence => "Business Failure Industry Incidence",
            BusinessFailureClassIncidence => "Business Failure Class Incidence",
            Estimated => "Estimated",
            NetQuantityIncrease => "Net Quantity Increase",
            NetQuantityDecrease => "Net Quantity Decrease",
            ExpenditureQuantity => "Expenditure Quantity",
            Originals => "Originals",
            Duplicates => "Duplicates",
            CompletedLineItems => "Completed Line Items",
            CompletedContracts => "Completed Contracts",
            ActiveContractsDelinquentBuyingPartyCaused => {
                "Active Contracts Delinquent-Buying Party Caused"
            }
            ActiveContractsDelinquent => "Active Contracts Delinquent",
            ActiveContractsDelinquentContractorCaused => {
                "Active Contracts Delinquent-Contractor Caused"
            }
            ActiveContractsDelinquentUnknownCauses => {
                "Active Contracts Delinquent-Unknown Causes"
            }
            ActiveLineItemsDelinquent => "Active Line Items Delinquent",
            ActiveLineItemsDelinquentBuyingPartyCaused => {
                "Active Line Items Delinquent-Buying Party Caused"
            }
            ActiveLineItemsDelinquentContractorCaused => {
                "Active Line Items Delinquent-Contractor Caused"
            }
            ActiveLineItemsDelinquentUnknownCauses => {
                "Active Line Items Delinquent-Unknown Causes"
            }
            ContractsCompletedDelinquentBuyingPartyCaused => {
                "Contracts Completed Delinquent-Buying Party Caused"
            }
            ContractCompletedDelinquentContractorCaused => {
                "Contract Completed Delinquent-Contractor Caused"
            }
            ContractsCompletedDelinquentUnknownCauses => {
                "Contracts Completed Delinquent-Unknown Causes"
            }
            ReportedDeficiencies => "Reported Deficiencies",
            LineItemsCompletedDelinquentBuyingPartyCaused => {
                "Line Items Completed Delinquent-Buying Party Caused"
            }
            LineItemsCompletedDelinquentContractorCaused => {
                "Line Items Completed Delinquent-Contractor Caused"
            }
            LineItemsCompletedDelinquentUnknownCauses => {
                "Line Items Completed Delinquent-Unknown Causes"
            }
            CorrectiveActionRequestsVerbal => "Corrective Action Requests-Verbal",
            CorrectiveActionRequestsWritten => "Corrective Action Requests-Written",
            GuaranteeFeeBuyupMaximum => "Guarantee Fee Buyup Maximum",
            ContractBuyup => "Contract Buyup",
            ContractBuydown => "Contract Buydown",
            GuaranteeFeeRateAfterAlternatePaymentMethod => {
                "Guarantee Fee Rate after Alternate Payment Method"
            }
            GuaranteeFeeRateAfterBuyupOrBuydown => {
                "Guarantee Fee Rate after Buyup or Buydown"
            }
            BuyupOrBuydownRatePerBasisPoint => "Buyup or Buydown Rate per Basis Point",
            LocationNetCapacity => "Location Net Capacity",
            SubjectToLossOrElimination => "Subject to loss or elimination",
            LifeTimeReserveActual => "Life-time Reserve - Actual",
            LossAllowance => "Loss Allowance",
            LatePaymentPeriod => "Late Payment Period",
            LimitValue => "Limit Value",
            LifeTimeReserveEstimated => "Life-time Reserve - Estimated",
            LossOrGain => "Loss or Gain",
            LostGas => "Lost Gas",
            LiquidInjectionVolume => "Liquid Injection Volume",
            CorrectiveActionRequestsMethodC => "Corrective Action Requests-Method C",
            CorrectiveActionRequestsMethodD => "Corrective Action Requests-Method D",
            CorrectiveActionRequestsMethodE => "Corrective Action Requests-Method E",
            AgedActiveLineItemsDelinquentContractorCaused => {
                "Aged Active Line Items Delinquent-Contractor Caused"
            }
            LostOil => "Lost Oil",
            LeasePeriods => "Lease Periods",
            AgedLineItemsDelinquent => "Aged Line Items Delinquent",
            AgedLineItemsCompletedContractorCaused => {
                "Aged Line Items Completed-Contractor Caused"
            }
            OilCondensateSold => "Oil Condensate Sold",
            TariffLossAllowance => "Tariff Loss Allowance",
            LifetimeReserveDaysAppliedToThisClaim => {
                "Lifetime Reserve Days - Applied to this Claim"
            }
            OilCondensateVolume => "Oil/Condensate Volume",
            LostWorkTimeActual => "Lost Work Time Actual",
            LostWorkTimeEstimated => "Lost Work Time Estimated",
            LengthOfResidency => "Length of Residency",
            Lanes => "Lanes",
            MatchingEquipment => "Matching Equipment",
            Maximum => "Maximum",
            TotalFederalPoints => "Total Federal Points",
            Contributions => "Contributions",
            Contributors => "Contributors",
            Endorsers => "Endorsers",
            Functions => "Functions",
            Guarantors => "Guarantors",
            Points => "Points",
            MiscellaneousAllowance => "Miscellaneous Allowance",
            NumberOfPublicOfficials => "Number of Public Officials",
            TotalNonFederalPoints => "Total Non-Federal Points",
            MillionDollarRoundtableCredits => "Million Dollar Roundtable Credits",
            MinimumNumberOfEmployees => "Minimum Number of Employees",
            Manufactured => "Manufactured",
            Pledges => "Pledges",
            TotalPoints => "Total Points",
            Miles => "Miles",
            Attendees => "Attendees",
            TicketsSold => "Tickets Sold",
            TotalNumberOfManifestLines => "Total Number of Manifest Lines",
            MaximumMaturityExtension => "Maximum Maturity Extension",
            Month => "Month",
            MinimumOrderPackageLevel => "Minimum Order Package Level",
            TotalNumberOfMapsInAPack => "Total Number of Maps in a Pack",
            MaximumShipQuantity => "Maximum Ship Quantity",
            QuantityOfNextLowerLevelTradeItem => {
                "Quantity of next lower level trade item"
            }
            MeasuredQuantity => "Measured Quantity",
            ResterilizationMaximum => "Resterilization Maximum",
            RecommendedNumberOfUses => "Recommended Number of Uses",
            TotalUnits => "Total Units",
            MaximumNumberOfEmployees => "Maximum Number of Employees",
            StackingFactor => "Stacking Factor",
            ComponentQuantity => "Component Quantity",
            NumberOfAttacksOrOccurrences => "Number of Attacks or Occurrences",
            NumberOfDead => "Number of Dead",
            NumberOfLiving => "Number of Living",
            NumberOfTimes => "Number of Times",
            MinimumForecastQuantity => "Minimum Forecast Quantity",
            MaximumForecastQuantity => "Maximum Forecast Quantity",
            RequestedReceiptQuantity => "Requested Receipt Quantity",
            RequestedDeliveryQuantity => "Requested Delivery Quantity",
            NumberOfNonCoveredDays => "Number of Non-covered Days",
            CodeNB => "Number of Units (Housing)",
            NumberOfClaimants => "Number of Claimants",
            NumberOfLateCharges => "Number of Late Charges",
            NonCoveredEstimated => "Non-Covered - Estimated",
            NumberOfFullTimeEmployees => "Number of Full-Time Employees",
            NumberOfNonsufficientFundItems => "Number of Nonsufficient Fund Items",
            NoncoveredActual => "Noncovered - Actual",
            NumberOfLevels => "Number of Levels",
            NumberOfHospitals => "Number of Hospitals",
            NumberOfPhysicians => "Number of Physicians",
            NumberOfMembers => "Number of Members",
            NumberOfFranchisees => "Number of Franchisees",
            NotReplacedBloodUnits => "Not Replaced Blood Units",
            NumberOfStations => "Number of Stations",
            Reports => "Reports",
            SinceLastTravel => "Since Last Travel",
            Net => "Net",
            UntilNextTravel => "Until Next Travel",
            ScheduledReceipt => "Scheduled Receipt",
            ScheduledDelivery => "Scheduled Delivery",
            OperationalReceipt => "Operational Receipt",
            OperationalDelivery => "Operational Delivery",
            AllocatedReceipt => "Allocated Receipt",
            AllocatedDelivery => "Allocated Delivery",
            DistributedConfirmedReceipt => "Distributed Confirmed Receipt",
            DistributedConfirmedDelivery => "Distributed Confirmed Delivery",
            SchedulingToleranceReceipt => "Scheduling Tolerance Receipt",
            SchedulingToleranceDelivery => "Scheduling Tolerance Delivery",
            Energy => "Energy",
            OrderCount => "Order Count",
            OtherMiscellaneousDisposition => "Other Miscellaneous Disposition",
            NumberOfWeeksPerYear => "Number of Weeks Per Year",
            OffPremiseSalesQuantity => "Off Premise Sales Quantity",
            OtherGasDisposition => "Other Gas Disposition",
            OtherInjectionVolume => "Other Injection Volume",
            OpeningStatementBalance => "Opening Statement Balance",
            OrderSizingFactor => "Order Sizing Factor",
            OriginalLoanTerm => "Original Loan Term",
            OnPremiseSalesQuantity => "On Premise Sales Quantity",
            OtherOilCondensateDisposition => "Other Oil Condensate Disposition",
            OptimumOrderQuantity => "Optimum Order Quantity",
            Original => "Original",
            NumberOfOperatingPeriodsAtFailure => "Number of Operating Periods at Failure",
            OutlierDays => "Outlier Days",
            Overage => "Overage",
            OtherWaterDisposition => "Other Water Disposition",
            ProjectPhases => "Project Phases",
            PhysicalStatusIii => "Physical Status III",
            PhysicalStatusIv => "Physical Status IV",
            PhysicalStatusV => "Physical Status V",
            NumberOfServicesOrProcedures => "Number of Services or Procedures",
            PrescriptionDosage => "Prescription Dosage",
            PrescriptionFrequency => "Prescription Frequency",
            NumberOfPeopleLivingAtResidence => "Number of People Living at Residence",
            PipelineAdjustmentOrAllowance => "Pipeline Adjustment or Allowance",
            PressureBase => "Pressure Base",
            PriorCumulativeImbalance => "Prior Cumulative Imbalance",
            PaymentDurationWeeks => "Payment Duration Weeks",
            PeriodOfEmployment => "Period of Employment",
            GasUsedForPlantFuel => "Gas Used for Plant Fuel",
            Persistency => "Persistency",
            Promotional => "Promotional",
            ParkingSpaces => "Parking Spaces",
            PartialBaths => "Partial Baths",
            PercentageOfOrderedQuantity => "Percentage of Ordered Quantity",
            PurchaseOfProduct => "Purchase of Product",
            CumulativeQuantityRequiredPriorToTheFirstScheduledPeriod => {
                "Cumulative Quantity Required Prior to the First Scheduled Period"
            }
            RequirementQuantityThatWasPreviouslyReleased => {
                "Requirement Quantity that was Previously Released"
            }
            Prescription => "Prescription",
            Patients => "Patients",
            PittedWater => "Pitted Water",
            PriorUnitsAccepted => "Prior Units Accepted",
            Paid => "Paid",
            MinimumQuantityToWhichTaxRateApplies => {
                "Minimum quantity to which tax rate applies"
            }
            MaximumQuantityToWhichTaxRateApplies => {
                "Maximum quantity to which tax rate applies"
            }
            QuantityEarned => "Quantity Earned",
            QuantityCarriedForward => "Quantity Carried Forward",
            NumberOf3To4YearOlds => "Number of 3 to 4 Year Olds",
            AutisticIndividuals => "Autistic Individuals",
            DeafBlindIndividuals => "Deaf-blind Individuals",
            HearingImpairedIndividuals => "Hearing Impaired Individuals",
            MentallyRetardedIndividuals => "Mentally Retarded Individuals",
            QuantityApproved => "Quantity Approved",
            QuantityDispensed => "Quantity Dispensed",
            QuantityDisapproved => "Quantity Disapproved",
            QuantityDelivered => "Quantity Delivered",
            QuantityDeferred => "Quantity Deferred",
            HighFabricationAuthorizationQuantity => {
                "High Fabrication Authorization Quantity"
            }
            QuantityOnHold => "Quantity on Hold",
            CommunityServiceDuration => "Community Service Duration",
            NumberOfTimesDeported => "Number of Times Deported",
            QuantityOfInnerPacks => "Quantity of Inner Packs",
            JailSentenceDuration => "Jail Sentence Duration",
            ProbationDuration => "Probation Duration",
            RestrictionDuration => "Restriction Duration",
            OperatingQuantity => "Operating Quantity",
            QuantityByPosition => "Quantity by Position",
            SuspendedDuration => "Suspended Duration",
            HighRawMaterialAuthorizationQuantity => {
                "High Raw Material Authorization Quantity"
            }
            QuantityPerSkid => "Quantity Per Skid",
            PlantThermalReduction => "Plant Thermal Reduction",
            QuantityServiced => "Quantity Serviced",
            QuantityCancelled => "Quantity Cancelled",
            QuantityWithdrawn => "Quantity Withdrawn",
            QualifyingWeeks => "Qualifying Weeks",
            RepaymentPlanTerm => "Repayment Plan Term",
            CodeR1 => "Replenishment (Fill)",
            IndividualsWithOrthopedicImpairment => {
                "Individuals with Orthopedic Impairment"
            }
            EstimatedRemainingPhysicalLife => "Estimated Remaining Physical Life",
            IndividualsWithSpecificLearningDisability => {
                "Individuals with Specific Learning Disability"
            }
            Axles => "Axles",
            PlatformCount => "Platform Count",
            IndividualsWithVisualImpairment => "Individuals with Visual Impairment",
            IndividualsWithOtherHealthImpairment => {
                "Individuals with Other Health Impairment"
            }
            Fuel => "Fuel",
            RefillsAuthorized => "Refills Authorized",
            ReplacedBloodUnits => "Replaced Blood Units",
            NumberOfItemsAuthorizedAtStore => "Number of Items Authorized at Store",
            NumberOfItemsAuthorizedAtWarehouse => {
                "Number of Items Authorized at Warehouse"
            }
            GasReturnedToEarth => "Gas Returned to Earth",
            NumberOfItemsInStock => "Number of Items in Stock",
            GasUsedForRepressuringOrPressureMaintenance => {
                "Gas Used for Repressuring or Pressure Maintenance"
            }
            NumberOfShelfTags => "Number of Shelf Tags",
            QuantityAvailableOnShelf => "Quantity Available on Shelf",
            GasReturnedToPropertyForFuel => "Gas Returned to Property for fuel",
            RoomCount => "Room Count",
            UnitsRented => "Units Rented",
            RetailDemandQuantity => "Retail Demand Quantity",
            Royalty => "Royalty",
            NumberOfShelfFacings => "Number of Shelf Facings",
            RetailSalesQuantity => "Retail Sales Quantity",
            WaterReInjectedOnProperty => "Water Re-injected on Property",
            RequirementQuantity => "Requirement Quantity",
            CodeS1 => "Planned Unit Development (PUD) Units",
            CodeS2 => "Rooms, Finished Area Above Grade",
            DwellingArea => "Dwelling Area",
            GarageOrCarportArea => "Garage or Carport Area",
            UnitsForSale => "Units for Sale",
            GrossRentMultiplier => "Gross Rent Multiplier",
            CodeS7 => "Age, High Value",
            CodeS8 => "Age, Low Value",
            CodeS9 => "Bedrooms, Finished Area Above Grade",
            Shipments => "Shipments",
            Solicited => "Solicited",
            CodeSC => "Bathrooms, Finished Area Above Grade",
            CriminalSentenceDuration => "Criminal Sentence Duration",
            CodeSE => "Gross Living, Finished Area Above Grade",
            Site => "Site",
            SwanGanz => "Swan-Ganz",
            Shortage => "Shortage",
            Rooms => "Rooms",
            AreaOfLevel => "Area of Level",
            GasShrinkage => "Gas Shrinkage",
            PredominateAge => "Predominate Age",
            MinimumCriminalSentenceDuration => "Minimum Criminal Sentence Duration",
            Age => "Age",
            OilSedimentation => "Oil Sedimentation",
            DaysSupply => "Days Supply",
            ProductSalesAmount => "Product Sales Amount",
            EffectiveAge => "Effective Age",
            SharesOfPreferredStock => "Shares of Preferred Stock",
            Standard => "Standard",
            ForecastedScannedQuantity => "Forecasted Scanned Quantity",
            SharesOfCommonStock => "Shares of Common Stock",
            SampleAmount => "Sample Amount",
            MaximumCriminalSentenceDuration => "Maximum Criminal Sentence Duration",
            StateOrProvinceMotorVehiclePenaltyPoints => {
                "State or Province Motor Vehicle Penalty Points"
            }
            Seasonal => "Seasonal",
            TimeUnitsKnown => "Time Units Known",
            TimeUnitsSpentOnDuty => "Time Units Spent on Duty",
            TotalDaysOnMarket => "Total Days on Market",
            TotalRooms => "Total Rooms",
            TotalNumberOfUnits => "Total Number of Units",
            TotalNumberOfUnitsForSale => "Total Number of Units for Sale",
            Tires => "Tires",
            TankAllowance => "Tank Allowance",
            OilTheft => "Oil Theft",
            TotalAtComplete => "Total at Complete",
            TotalToDate => "Total to Date",
            NumberOfTheatres => "Number of Theatres",
            TotalGasInjectionVolume => "Total Gas Injection Volume",
            TheoreticalQuantity => "Theoretical Quantity",
            TotalOilAndOrCondensateInjectionVolume => {
                "Total Oil and/or Condensate Injection Volume"
            }
            DurationInCurrentJob => "Duration in Current Job",
            TotalOilAndOrCondensateDisposition => {
                "Total Oil and/or Condensate Disposition"
            }
            TotalWaterDisposition => "Total Water Disposition",
            TotalBeginningInventory => "Total Beginning Inventory",
            Total => "Total",
            TimeInPosition => "Time in Position",
            TotalQuantityOfAllBuys => "Total Quantity of All Buys",
            Trips => "Trips",
            TotalNumberOfParkingSpaces => "Total Number of Parking Spaces",
            TotalProductionVolume => "Total Production Volume",
            TotalAdjustmentsVolume => "Total Adjustments Volume",
            TotalGasDisposition => "Total Gas Disposition",
            TotalWaterInjectionVolume => "Total Water Injection Volume",
            TotalEndingInventory => "Total Ending Inventory",
            TotalSalesVolume => "Total Sales Volume",
            FreelanceCollectors => "Freelance Collectors",
            BranchLocationsOwned => "Branch Locations Owned",
            BranchLocationsLeased => "Branch Locations Leased",
            UnitsCompleted => "Units Completed",
            Poultry => "Poultry",
            Livestock => "Livestock",
            Passengers => "Passengers",
            Trainers => "Trainers",
            Operators => "Operators",
            GasUsedOnProperty => "Gas Used on Property",
            Inspectors => "Inspectors",
            Collectors => "Collectors",
            Professionals => "Professionals",
            Supervisors => "Supervisors",
            ApproximateNumberOfUnitsForSaleProjected => {
                "Approximate Number of Units for Sale Projected"
            }
            Administrators => "Administrators",
            Promoters => "Promoters",
            OilCondensateUsedOnProperty => "Oil Condensate Used on Property",
            Divisions => "Divisions",
            Tables => "Tables",
            FuelPumps => "Fuel Pumps",
            InUse => "In-Use",
            Machines => "Machines",
            Used => "Used",
            TrademarksUsed => "Trademarks Used",
            AvailableForCultivation => "Available for Cultivation",
            Foremen => "Foremen",
            TravellingEmployees => "Travelling Employees",
            FreelanceSalespersons => "Freelance Salespersons",
            RetentionQuantity => "Retention Quantity",
            AvailableQuantity => "Available Quantity",
            TransferQuantity => "Transfer Quantity",
            SurveysInAverageRating => "Surveys in Average Rating",
            Vacancies => "Vacancies",
            UnsubscribedCapacity => "Unsubscribed Capacity",
            ShippingContainerQuantity => "Shipping Container Quantity",
            VolumeShrinkageAdjustmentOrAllowance => {
                "Volume Shrinkage Adjustment or Allowance"
            }
            BlankVotes => "Blank Votes",
            CumulativeEarnedValue => "Cumulative Earned Value",
            ScatteredVotes => "Scattered Votes",
            EarnedValue => "Earned Value",
            FederalVotes => "Federal Votes",
            GasVented => "Gas Vented",
            ScheduleVariance => "Schedule Variance",
            CumulativeScheduleVariance => "Cumulative Schedule Variance",
            CumulativeVariance => "Cumulative Variance",
            EstimateAtComplete => "Estimate at Complete",
            AtCompleteVariance => "At Complete Variance",
            VarianceAdjustment => "Variance Adjustment",
            NoVotes => "No Votes",
            PresidentialVotes => "Presidential Votes",
            UtilizationServiceLife => "Utilization Service Life",
            Variance => "Variance",
            Visits => "Visits",
            Votes => "Votes",
            RecommendedServiceLife => "Recommended Service Life",
            VoidVotes => "Void Votes",
            ShelfLifePeriod => "Shelf Life Period",
            YesVotes => "Yes Votes",
            BankruptcyPetitions => "Bankruptcy Petitions",
            Buyers => "Buyers",
            Debentures => "Debentures",
            DebenturesFiledAgainstDirectors => "Debentures Filed against Directors",
            DetrimentalLegalFilingsAgainstDirectors => {
                "Detrimental Legal Filings against Directors"
            }
            FailedBusinessesOfDirectors => "Failed Businesses of Directors",
            Professors => "Professors",
            Sellers => "Sellers",
            SkilledWorkers => "Skilled Workers",
            TrademarksRepresented => "Trademarks Represented",
            TotalNumberOfWorkersCompensationFirstReports => {
                "Total number of Workers' Compensation First Reports"
            }
            TotalNumberOfWorkersCompensationSubsequentReports => {
                "Total number of Workers' Compensation Subsequent Reports"
            }
            TotalNumberOfWorkersCompensationCombinedReports => {
                "Total number of Workers' Compensation Combined Reports"
            }
            UnitsWorkedPerDay => "Units Worked per Day",
            LimitedQuantity => "Limited Quantity",
            WeightGain => "Weight Gain",
            WeightLoss => "Weight Loss",
            OperatorsWorkingInterest => "Operator's Working Interest",
            NumberOfProducingWellsRemainingOnPropertyOrFacility => {
                "Number of Producing Wells Remaining on Property or Facility"
            }
            NumberOfProducingWellsRemainingOnRoyaltyAccount => {
                "Number of Producing Wells Remaining on Royalty Account"
            }
            TotalWorkingInterest => "Total Working Interest",
            WaterVolume => "Water Volume",
            WeeksWorked => "Weeks Worked",
            LicenseWithdrawalDuration => "License Withdrawal Duration",
            LicenseWithdrawalsSent => "License Withdrawals Sent",
            ProducingWells => "Producing Wells",
            Gross => "Gross",
            AssessmentHours => "Assessment Hours",
            DutyDays => "Duty Days",
            ContractDays => "Contract Days",
            NumberOfDaysEmployed => "Number of Days Employed",
            TotalOfIssuableAssets => "Total of Issuable Assets",
            CodeXB => "Total System Backorder Quantity, High Priority",
            CodeXC => "Total Service Backorder Quantity, High Priority",
            CodeXD => "Total System Backorder Quantity, Low Priority",
            CodeXE => "Total Service Backorder Quantity, Low Priority",
            OnHandAndDueIn => "On Hand and Due-In",
            InstallmentPayments => "Installment Payments",
            CodeXJ => {
                "Other War Reserve Material Requirements Protectable (OWRMRP) Quantity"
            }
            ApproximateNumberOfUnitsProjected => "Approximate Number of Units Projected",
            ApproximateNumberOfHolders => "Approximate Number of Holders",
            CirculatingOil => "Circulating Oil",
            StockObjectiveAndInsuranceQuantity => {
                "Stock Objective and Insurance Quantity"
            }
            ProtectedQuantity => "Protected Quantity",
            Reserved => "Reserved",
            RequisitioningObjective => "Requisitioning Objective",
            AuthorizedRetentionLevel => "Authorized Retention Level",
            SafetyLevel => "Safety Level",
            BackorderLines => "Backorder Lines",
            NumberOfLostCards => "Number of Lost Cards",
            NumberOfStolenCards => "Number of Stolen Cards",
            NumberOfCardsNotReceived => "Number of Cards not Received",
            NumberOfActiveAccountsThisCycle => "Number of Active Accounts This Cycle",
            NumberOfOpenAccounts => "Number of Open Accounts",
            NumberOfAccountsPastDue => "Number of Accounts Past Due",
            NumberOfCardsOutstanding => "Number of Cards Outstanding",
            OnHandPlusPipeline => "On Hand plus Pipeline",
            TotalDemandQuantity => "Total Demand Quantity",
            TotalDemandOrders => "Total Demand Orders",
            FirstQuarterRecurringDemand => "First Quarter Recurring Demand",
            FirstQuarterRecurringOrders => "First Quarter Recurring Orders",
            FirstQuarterNonRecurringDemand => "First Quarter Non-recurring Demand",
            FirstQuarterNonRecurringOrders => "First Quarter Non-recurring Orders",
            SecondQuarterRecurringDemand => "Second Quarter Recurring Demand",
            SecondQuarterRecurringOrders => "Second Quarter Recurring Orders",
            SecondQuarterNonRecurringDemand => "Second Quarter Non-recurring Demand",
            SecondQuarterNonRecurringOrders => "Second Quarter Non-recurring Orders",
            ThirdQuarterRecurringDemand => "Third Quarter Recurring Demand",
            ThirdQuarterRecurringOrders => "Third Quarter Recurring Orders",
            ThirdQuarterNonRecurringDemand => "Third Quarter Non-recurring Demand",
            ThirdQuarterNonRecurringOrders => "Third Quarter Non-recurring Orders",
            FourthQuarterRecurringDemand => "Fourth Quarter Recurring Demand",
            FourthQuarterRecurringOrders => "Fourth Quarter Recurring Orders",
            FourthQuarterNonRecurringDemand => "Fourth Quarter Non-recurring Demand",
            FourthQuarterNonRecurringOrders => "Fourth Quarter Non-recurring Orders",
            Trailers => "Trailers",
            ReorderPointQuantity => "Reorder Point Quantity",
            ContractLineItemQuantity => "Contract Line Item Quantity",
            Years => "Years",
            MaximumQuantityOfFreeServiceCalls => "Maximum Quantity of Free Service Calls",
            UnitsWorkedLastDay => "Units Worked Last Day",
            UnitsWorkedPerWeek => "Units Worked per Week",
            UnitsWorkedPerQuarter => "Units Worked per Quarter",
            NumberWeeksPaid => "Number Weeks Paid",
            UnusedAccumulatedSickDays => "Unused Accumulated Sick Days",
            DeliveryPointReductionQuantity => "Delivery Point Reduction Quantity",
            ReceiptPointReductionQuantity => "Receipt Point Reduction Quantity",
            ReductionQuantity => "Reduction Quantity",
            FederalMedicareOrMedicaidClaimMandateCategory1 => {
                "Federal Medicare or Medicaid Claim Mandate - Category 1"
            }
            FederalMedicareOrMedicaidClaimMandateCategory2 => {
                "Federal Medicare or Medicaid Claim Mandate - Category 2"
            }
            FederalMedicareOrMedicaidClaimMandateCategory3 => {
                "Federal Medicare or Medicaid Claim Mandate - Category 3"
            }
            FederalMedicareOrMedicaidClaimMandateCategory4 => {
                "Federal Medicare or Medicaid Claim Mandate - Category 4"
            }
            FederalMedicareOrMedicaidClaimMandateCategory5 => {
                "Federal Medicare or Medicaid Claim Mandate - Category 5"
            }
            FederalPensionMandateCategory1 => "Federal Pension Mandate - Category 1",
            FederalPensionMandateCategory2 => "Federal Pension Mandate - Category 2",
            FederalPensionMandateCategory3 => "Federal Pension Mandate - Category 3",
            HoldingPeriod => "Holding Period",
            FederalPensionMandateCategory5 => "Federal Pension Mandate - Category 5",
            FederalMedicareOrMedicaidPaymentMandateCategory1 => {
                "Federal Medicare or Medicaid Payment Mandate - Category 1"
            }
            FederalMedicareOrMedicaidPaymentMandateCategory2 => {
                "Federal Medicare or Medicaid Payment Mandate - Category 2"
            }
            FederalMedicareOrMedicaidPaymentMandateCategory3 => {
                "Federal Medicare or Medicaid Payment Mandate - Category 3"
            }
            FederalMedicareOrMedicaidPaymentMandateCategory4 => {
                "Federal Medicare or Medicaid Payment Mandate - Category 4"
            }
            FederalMedicareOrMedicaidPaymentMandateCategory5 => {
                "Federal Medicare or Medicaid Payment Mandate - Category 5"
            }
            FederalPensionMandateCategory4 => "Federal Pension Mandate - Category 4",
            SharesAdded => "Shares Added",
            ExtendedTerm => "Extended Term",
            AmortizationTerm => "Amortization Term",
            BeginningShares => "Beginning Shares",
            SharesDeleted => "Shares Deleted",
            QuantityOfDealerLicensePlates => "Quantity of Dealer License Plates",
            CurrentShareBalance => "Current Share Balance",
            SizeOfHousehold => "Size of Household",
            ProjectUnitsSold => "Project Units Sold",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<QuantityQualifier> {
        {
            use QuantityQualifier::*;
            match description {
                "Hospital/Homebound Individuals" => Some(HospitalHomeboundIndividuals),
                "Number of Hours Per Day" => Some(NumberOfHoursPerDay),
                "Number of Hours Per Week" => Some(NumberOfHoursPerWeek),
                "Number of Months Per Year" => Some(NumberOfMonthsPerYear),
                "Number of Periods Per Week" => Some(NumberOfPeriodsPerWeek),
                "Expected Expenditure Quantity" => Some(ExpectedExpenditureQuantity),
                "Number of Hours Per Year" => Some(NumberOfHoursPerYear),
                "Pre-Kindergarten Students" => Some(PreKindergartenStudents),
                "First Grade Students" => Some(FirstGradeStudents),
                "Second Grade Students" => Some(SecondGradeStudents),
                "Third Grade Students" => Some(ThirdGradeStudents),
                "Fourth Grade Students" => Some(FourthGradeStudents),
                "Fifth Grade Students" => Some(FifthGradeStudents),
                "Sixth Grade Students" => Some(SixthGradeStudents),
                "Seventh Grade Students" => Some(SeventhGradeStudents),
                "Eighth Grade Students" => Some(EighthGradeStudents),
                "Ninth Grade Students" => Some(NinthGradeStudents),
                "Carnegie Units" => Some(CarnegieUnits),
                "Number of Disability Types" => Some(NumberOfDisabilityTypes),
                "Number of Males" => Some(NumberOfMales),
                "Number of Females" => Some(NumberOfFemales),
                "Individuals with Multiple Disabilities" => {
                    Some(IndividualsWithMultipleDisabilities)
                }
                "Individuals with Serious Emotional Disturbance" => {
                    Some(IndividualsWithSeriousEmotionalDisturbance)
                }
                "Individuals with Speech or Language Impairment" => {
                    Some(IndividualsWithSpeechOrLanguageImpairment)
                }
                "Individuals with Traumatic Brain Injury" => {
                    Some(IndividualsWithTraumaticBrainInjury)
                }
                "Blind Individuals" => Some(BlindIndividuals),
                "Deaf Individuals" => Some(DeafIndividuals),
                "Discrete Quantity" => Some(DiscreteQuantity),
                "Original Duration (in calendar units)" => Some(Code1A),
                "Current Duration (in calendar units)" => Some(Code1B),
                "Remaining Duration (in calendar units)" => Some(Code1C),
                "Total Float (in calendar units)" => Some(Code1D),
                "Free Float (in calendar units)" => Some(Code1E),
                "Lag (as in Lag Time - in calendar units)" => Some(Code1F),
                "Lead Time (in calendar units)" => Some(Code1G),
                "Started" => Some(Started),
                "Completed" => Some(Completed),
                "Due" => Some(Due),
                "Time Units" => Some(TimeUnits),
                "Shifts" => Some(Shifts),
                "Time units per shift" => Some(TimeUnitsPerShift),
                "Scrap allowed" => Some(ScrapAllowed),
                "Calendar Units" => Some(CalendarUnits),
                "Resource (Quantity) available" => Some(Code1P),
                "Total Resource (Quantity)" => Some(Code1Q),
                "Level Resource (Quantity)" => Some(Code1R),
                "Late" => Some(Late),
                "Number of Delinquent Installments" => {
                    Some(NumberOfDelinquentInstallments)
                }
                "Number of Loans" => Some(NumberOfLoans),
                "Total Number of Mortgagees" => Some(TotalNumberOfMortgagees),
                "Total Number of Loan Detail Records" => {
                    Some(TotalNumberOfLoanDetailRecords)
                }
                "Prescription Effective Period" => Some(PrescriptionEffectivePeriod),
                "Rate Per Day (RPD)" => Some(Code1Y),
                "End Of Month Inventory Prior To Ship" => {
                    Some(EndOfMonthInventoryPriorToShip)
                }
                "Cumulative Quantity" => Some(CumulativeQuantity),
                "Commitment Period" => Some(CommitmentPeriod),
                "Number of Borrowers" => Some(NumberOfBorrowers),
                "Number of Adjustment Periods" => Some(NumberOfAdjustmentPeriods),
                "Age Nearest" => Some(AgeNearest),
                "Total Other Properties Owned and Financed" => {
                    Some(TotalOtherPropertiesOwnedAndFinanced)
                }
                "Age Next" => Some(AgeNext),
                "Reconsideration Period" => Some(ReconsiderationPeriod),
                "Flat Extra Premium" => Some(FlatExtraPremium),
                "CO2 Injection Volume" => Some(Co2InjectionVolume),
                "Accounts Placed for Collection" => Some(AccountsPlacedForCollection),
                "Changes" => Some(Changes),
                "Companies in Same Activity for a Period" => {
                    Some(CompaniesInSameActivityForAPeriod)
                }
                "Comparison Period" => Some(ComparisonPeriod),
                "Departments" => Some(Departments),
                "Employees Shared" => Some(EmployeesShared),
                "Estimated Accounts" => Some(EstimatedAccounts),
                "Installed Capacity" => Some(InstalledCapacity),
                "Levels Occupied" => Some(LevelsOccupied),
                "Registered Brands Distributed" => Some(RegisteredBrandsDistributed),
                "Electronic Signatures" => Some(ElectronicSignatures),
                "Bytes" => Some(Bytes),
                "Employed at this Location" => Some(EmployedAtThisLocation),
                "Segments" => Some(Segments),
                "Registered Brands Manufactured" => Some(RegisteredBrandsManufactured),
                "Functional Groups" => Some(FunctionalGroups),
                "Transaction Sets" => Some(TransactionSets),
                "Discreet Quantity - Rejected Material" => {
                    Some(DiscreetQuantityRejectedMaterial)
                }
                "Total Credits Accepted" => Some(TotalCreditsAccepted),
                "Total Credits Rejected" => Some(TotalCreditsRejected),
                "Total Debits Accepted" => Some(TotalDebitsAccepted),
                "Total Debits Rejected" => Some(TotalDebitsRejected),
                "Total Payments Rejected" => Some(TotalPaymentsRejected),
                "Total Pre-advices Accepted" => Some(TotalPreAdvicesAccepted),
                "Total Pre-advices Rejected" => Some(TotalPreAdvicesRejected),
                "Total Prenotes Accepted" => Some(TotalPrenotesAccepted),
                "Total Prenotes Rejected" => Some(TotalPrenotesRejected),
                "Total Post-advices Accepted" => Some(TotalPostAdvicesAccepted),
                "Total Post-advices Rejected" => Some(TotalPostAdvicesRejected),
                "Total Unidentified Transactions Rejected" => {
                    Some(TotalUnidentifiedTransactionsRejected)
                }
                "Total Credits Received" => Some(TotalCreditsReceived),
                "Total Debits Received" => Some(TotalDebitsReceived),
                "Individuals with Noncategorical Preschool Disability" => {
                    Some(IndividualsWithNoncategoricalPreschoolDisability)
                }
                "Total Pre-advices Received" => Some(TotalPreAdvicesReceived),
                "Total Prenotes Received" => Some(TotalPrenotesReceived),
                "Total Post-advices Received" => Some(TotalPostAdvicesReceived),
                "Total Debits" => Some(TotalDebits),
                "Total Credits" => Some(TotalCredits),
                "Minimum Transfer" => Some(MinimumTransfer),
                "Maximum Transfer" => Some(MaximumTransfer),
                "Speed Capacity" => Some(SpeedCapacity),
                "Subcontractors" => Some(Subcontractors),
                "Students" => Some(Students),
                "Discrete Quantity - Rejected Material: Disposition Replacement" => {
                    Some(DiscreteQuantityRejectedMaterialDispositionReplacement)
                }
                "Accounts" => Some(Accounts),
                "Agents" => Some(Agents),
                "Authorized Shares" => Some(AuthorizedShares),
                "Clerks" => Some(Clerks),
                "Design Employees" => Some(DesignEmployees),
                "Foreign Related Entities" => Some(ForeignRelatedEntities),
                "Group Employees" => Some(GroupEmployees),
                "Issued Shares" => Some(IssuedShares),
                "Laborers" => Some(Laborers),
                "Other Employee Type" => Some(OtherEmployeeType),
                "Part Time Employees" => Some(PartTimeEmployees),
                "Related Entities" => Some(RelatedEntities),
                "Relatives Employed" => Some(RelativesEmployed),
                "Salespersons" => Some(Salespersons),
                "Space Occupied" => Some(SpaceOccupied),
                "Special Partners" => Some(SpecialPartners),
                "Suppliers' Credit" => Some(SuppliersCredit),
                "Technicians" => Some(Technicians),
                "Trainees" => Some(Trainees),
                "Warehouse Employees" => Some(WarehouseEmployees),
                "Shareholders" => Some(Shareholders),
                "Available Units" => Some(AvailableUnits),
                "Total Unduplicated Headcount" => Some(TotalUnduplicatedHeadcount),
                "Meals Per Week" => Some(MealsPerWeek),
                "Programs Offered" => Some(ProgramsOffered),
                "Typical Credit Hours Taken per Graduate Student (Full Time)" => {
                    Some(Code4Z)
                }
                "Discrete Quantity - Rejected Material: Disposition Credit" => {
                    Some(DiscreteQuantityRejectedMaterialDispositionCredit)
                }
                "Aggregate Benefit Period" => Some(AggregateBenefitPeriod),
                "Anticipated Length of Service" => Some(AnticipatedLengthOfService),
                "Approval/Offer Duration" => Some(ApprovalOfferDuration),
                "Benefit Amount" => Some(BenefitAmount),
                "Benefit Period" => Some(BenefitPeriod),
                "Brothers Deceased" => Some(BrothersDeceased),
                "Brothers Living" => Some(BrothersLiving),
                "Children" => Some(Children),
                "Citations" => Some(Citations),
                "Claim Period" => Some(ClaimPeriod),
                "Coverage" => Some(Coverage),
                "Elimination Period" => Some(EliminationPeriod),
                "Elimination Period - Accident" => Some(EliminationPeriodAccident),
                "Elimination Period - Sickness" => Some(EliminationPeriodSickness),
                "Employees - Nonowner" => Some(EmployeesNonowner),
                "Employees - Owner" => Some(EmployeesOwner),
                "Employees - Part Time" => Some(EmployeesPartTime),
                "Employees - Same Duties" => Some(EmployeesSameDuties),
                "Employees - Same Occupation" => Some(EmployeesSameOccupation),
                "Expense" => Some(Expense),
                "Frequency" => Some(Frequency),
                "General Elimination Period" => Some(GeneralEliminationPeriod),
                "Guarantee Period" => Some(GuaranteePeriod),
                "Height" => Some(Height),
                "Hours Flown - Aircraft Type/Life" => Some(HoursFlownAircraftTypeLife),
                "Hours Flown - Aircraft Type/Period" => {
                    Some(HoursFlownAircraftTypePeriod)
                }
                "Discrete Quantity - Rejected Material: Disposition Pending" => {
                    Some(DiscreteQuantityRejectedMaterialDispositionPending)
                }
                "Hours Flown - Aircraft/Type Flying" => {
                    Some(HoursFlownAircraftTypeFlying)
                }
                "Hours Flown - Lifetime" => Some(HoursFlownLifetime),
                "Hours Flown - Type Flying" => Some(HoursFlownTypeFlying),
                "Impairment Duration" => Some(ImpairmentDuration),
                "Impairment Frequency" => Some(ImpairmentFrequency),
                "Installment Frequency" => Some(InstallmentFrequency),
                "Installments" => Some(Installments),
                "Intended Change Time Period" => Some(IntendedChangeTimePeriod),
                "Interim Term Period" => Some(InterimTermPeriod),
                "Involvement Period" => Some(InvolvementPeriod),
                "Loan Rate" => Some(LoanRate),
                "Maximum Age" => Some(MaximumAge),
                "Maximum Benefit Period - Accident" => Some(MaximumBenefitPeriodAccident),
                "Maximum Benefit Period - Sickness" => Some(MaximumBenefitPeriodSickness),
                "Maximum Benefit Period" => Some(MaximumBenefitPeriod),
                "Medication Duration" => Some(MedicationDuration),
                "Minimum Age" => Some(MinimumAge),
                "Own Occupation Qualification Period" => {
                    Some(OwnOccupationQualificationPeriod)
                }
                "Owner's Equity" => Some(OwnersEquity),
                "Ownership Change Age" => Some(OwnershipChangeAge),
                "Ownership Duration" => Some(OwnershipDuration),
                "Ownership Percentage" => Some(OwnershipPercentage),
                "Payment Frequency" => Some(PaymentFrequency),
                "Payments Number" => Some(PaymentsNumber),
                "Arrests" => Some(Arrests),
                "Placement Period Expiration" => Some(PlacementPeriodExpiration),
                "Cumulative Quantity - Rejected Material" => {
                    Some(CumulativeQuantityRejectedMaterial)
                }
                "Previous Benefits" => Some(PreviousBenefits),
                "Qualification Period" => Some(QualificationPeriod),
                "Range Average" => Some(RangeAverage),
                "Range Maximum" => Some(RangeMaximum),
                "Range Minimum" => Some(RangeMinimum),
                "Relationship Duration" => Some(RelationshipDuration),
                "Replaced Amount" => Some(ReplacedAmount),
                "Residence Duration" => Some(ResidenceDuration),
                "Sisters Deceased" => Some(SistersDeceased),
                "Sisters Living" => Some(SistersLiving),
                "Time Frame" => Some(TimeFrame),
                "Time in Country" => Some(TimeInCountry),
                "Time Since Hospitalization" => Some(TimeSinceHospitalization),
                "Time Since Last Application" => Some(TimeSinceLastApplication),
                "Time Since Last Civilian Flight" => Some(TimeSinceLastCivilianFlight),
                "Time Since Last Insurance Medical" => {
                    Some(TimeSinceLastInsuranceMedical)
                }
                "Time Since Last Military Flight" => Some(TimeSinceLastMilitaryFlight),
                "Time Since Medical Consult" => Some(TimeSinceMedicalConsult),
                "Time Since Medication End" => Some(TimeSinceMedicationEnd),
                "Time Since Medication Start" => Some(TimeSinceMedicationStart),
                "Time Since Onset" => Some(TimeSinceOnset),
                "Time Since Surgery" => Some(TimeSinceSurgery),
                "Time Since Trip" => Some(TimeSinceTrip),
                "Travel Frequency" => Some(TravelFrequency),
                "Travel Period" => Some(TravelPeriod),
                "Trip Duration" => Some(TripDuration),
                "Cumulative Quantity - Rejected Material: Disposition Replacement" => {
                    Some(CumulativeQuantityRejectedMaterialDispositionReplacement)
                }
                "Visitation Frequency" => Some(VisitationFrequency),
                "Weight" => Some(Weight),
                "Weight Change Period" => Some(WeightChangePeriod),
                "Work Period" => Some(WorkPeriod),
                "Existence Limit Period" => Some(ExistenceLimitPeriod),
                "Shares" => Some(Shares),
                "Directors" => Some(Directors),
                "Minimum" => Some(Minimum),
                "Voting Shares Held" => Some(VotingSharesHeld),
                "Outstanding Shares" => Some(OutstandingShares),
                "Shares Held as Treasury Stock" => Some(SharesHeldAsTreasuryStock),
                "Shares Subscribed but Not Issued" => Some(SharesSubscribedButNotIssued),
                "Total Shares of Stock" => Some(TotalSharesOfStock),
                "Shares Owned by In-State Residents" => {
                    Some(SharesOwnedByInStateResidents)
                }
                "Shares Owned by Out-of-State Residents" => {
                    Some(SharesOwnedByOutOfStateResidents)
                }
                "Partners" => Some(Partners),
                "Land Holding" => Some(LandHolding),
                "Non-Domestic Stockholders" => Some(NonDomesticStockholders),
                "Shares Subscribed" => Some(SharesSubscribed),
                "Maximum Number Free Miles" => Some(MaximumNumberFreeMiles),
                "Typical Credit Hours Taken per Undergraduate Student (Full Time)" => {
                    Some(Code8U)
                }
                "Typical Credit Hours Taken per First-Professional Student (Full Time)" => {
                    Some(Code8V)
                }
                "Full-time Equivalents" => Some(FullTimeEquivalents),
                "Total Credit Hours" => Some(TotalCreditHours),
                "Total Non-Credit Hours" => Some(TotalNonCreditHours),
                "Total Contact Hours" => Some(TotalContactHours),
                "Cumulative Quantity - Rejected Material: Disposition Credit" => {
                    Some(CumulativeQuantityRejectedMaterialDispositionCredit)
                }
                "Time Expended" => Some(TimeExpended),
                "Primary Meter Reading Value" => Some(PrimaryMeterReadingValue),
                "Engineered Standard" => Some(EngineeredStandard),
                "Active Maintenance Time" => Some(ActiveMaintenanceTime),
                "Actual Duration" => Some(ActualDuration),
                "Estimated Duration" => Some(EstimatedDuration),
                "Gross Estimate" => Some(GrossEstimate),
                "Finish Offset" => Some(FinishOffset),
                "Start Offset" => Some(StartOffset),
                "Picture Count" => Some(PictureCount),
                "Component Meter Reading Count" => Some(ComponentMeterReadingCount),
                "Total Clock Hours" => Some(TotalClockHours),
                "Enrollees" => Some(Enrollees),
                "Total Days Submitted" => Some(TotalDaysSubmitted),
                "Total Days Approved" => Some(TotalDaysApproved),
                "Cumulative Quantity - Rejected Material: Disposition Pending" => {
                    Some(CumulativeQuantityRejectedMaterialDispositionPending)
                }
                "Split Quantity" => Some(SplitQuantity),
                "Ship Notice Quantity" => Some(ShipNoticeQuantity),
                "Collateral Requirements" => Some(CollateralRequirements),
                "Quantity in Float" => Some(QuantityInFloat),
                "Quantity in Hold Out" => Some(QuantityInHoldOut),
                "Line Thread Quantity" => Some(LineThreadQuantity),
                "Quantity on Hand" => Some(QuantityOnHand),
                "Previous Week Quantity" => Some(PreviousWeekQuantity),
                "Unverified Receipts" => Some(UnverifiedReceipts),
                "Unusable Quantity" => Some(UnusableQuantity),
                "Cumulative Quantity Shipped Short - Disposition Pending" => {
                    Some(CumulativeQuantityShippedShortDispositionPending)
                }
                "Cumulative Quantity Shipped Short - Disposition Challenged" => {
                    Some(CumulativeQuantityShippedShortDispositionChallenged)
                }
                "Cumulative Quantity Shipped Long - Disposition Pending" => {
                    Some(CumulativeQuantityShippedLongDispositionPending)
                }
                "Cumulative Quantity Shipped Long - Disposition Challenged" => {
                    Some(CumulativeQuantityShippedLongDispositionChallenged)
                }
                "OEM Inventory" => Some(OemInventory),
                "Total Inventory" => Some(TotalInventory),
                "Committed Quantity" => Some(CommittedQuantity),
                "Quantity Available for Return" => Some(QuantityAvailableForReturn),
                "Projected Available Inventory" => Some(ProjectedAvailableInventory),
                "Quote Quantity on Inventory" => Some(QuoteQuantityOnInventory),
                "Additional Demand Quantity" => Some(AdditionalDemandQuantity),
                "Quantity Sold" => Some(QuantitySold),
                "Quantity Available for Sale (stock quantity)" => Some(Code33),
                "Noncommitted Inventory on Shelf" => Some(NoncommittedInventoryOnShelf),
                "Inventory on Shelf + Work in Progress" => {
                    Some(InventoryOnShelfWorkInProgress)
                }
                "Distributor Inventory" => Some(DistributorInventory),
                "Work In Process" => Some(WorkInProcess),
                "Original Quantity" => Some(OriginalQuantity),
                "Shipped Quantity" => Some(ShippedQuantity),
                "Remaining Quantity" => Some(RemainingQuantity),
                "Number of Batches" => Some(NumberOfBatches),
                "Number of Checks" => Some(NumberOfChecks),
                "Talk Paths" => Some(TalkPaths),
                "Number of Patient Admissions" => Some(NumberOfPatientAdmissions),
                "Cumulative quantity on order" => Some(CumulativeQuantityOnOrder),
                "Total transactions" => Some(TotalTransactions),
                "Primary Net Quantity" => Some(PrimaryNetQuantity),
                "Secondary Net Quantity" => Some(SecondaryNetQuantity),
                "Number of Signed Bills of Lading" => Some(NumberOfSignedBillsOfLading),
                "Number of Copies of Bill of Lading" => {
                    Some(NumberOfCopiesOfBillOfLading)
                }
                "Number of Unsigned Bills of Lading" => {
                    Some(NumberOfUnsignedBillsOfLading)
                }
                "Number of Originals" => Some(NumberOfOriginals),
                "Original payment item count." => Some(OriginalPaymentItemCount),
                "Bank reject item count." => Some(BankRejectItemCount),
                "Net to pay item count." => Some(NetToPayItemCount),
                "Minimum Contract Quantity" => Some(MinimumContractQuantity),
                "Minimum Order Quantity" => Some(MinimumOrderQuantity),
                "Payment Cancellation Item Count" => Some(PaymentCancellationItemCount),
                "Individuals with Developmental Delay" => {
                    Some(IndividualsWithDevelopmentalDelay)
                }
                "Total Authorized Quantity" => Some(TotalAuthorizedQuantity),
                "Remaining Authorized Quantity" => Some(RemainingAuthorizedQuantity),
                "Number of Days Covered by Inventory" => {
                    Some(NumberOfDaysCoveredByInventory)
                }
                "On Order Quantity" => Some(OnOrderQuantity),
                "Past Due Quantity" => Some(PastDueQuantity),
                "Previous Month's Usage" => Some(PreviousMonthsUsage),
                "Minimum Fabrication Quantity" => Some(MinimumFabricationQuantity),
                "Minimum Ship Quantity" => Some(MinimumShipQuantity),
                "Maximum Number of Shipments Allowed" => {
                    Some(MaximumNumberOfShipmentsAllowed)
                }
                "Incremental Order Quantity" => Some(IncrementalOrderQuantity),
                "Maximum Order Quantity" => Some(MaximumOrderQuantity),
                "Educable Mentally Retarded Individuals" => {
                    Some(EducableMentallyRetardedIndividuals)
                }
                "Minimum Stock Level" => Some(MinimumStockLevel),
                "Maximum Stock Level" => Some(MaximumStockLevel),
                "Damaged Goods" => Some(DamagedGoods),
                "Receipts" => Some(Receipts),
                "Returns" => Some(Returns),
                "Stock Transfers In" => Some(StockTransfersIn),
                "Stock Transfers Out" => Some(StockTransfersOut),
                "Billing Unit(s) Per Pricing Unit" => Some(Code79),
                "Pricing Unit(s) Per Billing Unit" => Some(Code80),
                "Prepaid Quantity Shipped" => Some(PrepaidQuantityShipped),
                "Prepaid Quantity Not Shipped" => Some(PrepaidQuantityNotShipped),
                "Submitted Quantity Sold" => Some(SubmittedQuantitySold),
                "Submitted Quantity Returned" => Some(SubmittedQuantityReturned),
                "Lot Size" => Some(LotSize),
                "Nonconformance Quantity" => Some(NonconformanceQuantity),
                "Quantity Received" => Some(QuantityReceived),
                "Beds" => Some(Beds),
                "Operating Beds" => Some(OperatingBeds),
                "Acknowledged Quantity" => Some(AcknowledgedQuantity),
                "Additional Usage Quantity" => Some(AdditionalUsageQuantity),
                "Allotted Usage Quantity" => Some(AllottedUsageQuantity),
                "Attendant-Handled Quantity" => Some(AttendantHandledQuantity),
                "Billable Quantity" => Some(BillableQuantity),
                "Data Storage Quantity" => Some(DataStorageQuantity),
                "Non-Billable Quantity" => Some(NonBillableQuantity),
                "Non-Urgent Delivery Quantity" => Some(NonUrgentDeliveryQuantity),
                "Overflow Quantity" => Some(OverflowQuantity),
                "Quantity Used" => Some(QuantityUsed),
                "Severely Mentally Retarded Individuals" => {
                    Some(SeverelyMentallyRetardedIndividuals)
                }
                "Acceptable Unserviceable Quantity" => {
                    Some(AcceptableUnserviceableQuantity)
                }
                "Optimistic Duration" => Some(OptimisticDuration),
                "Most Likely Duration" => Some(MostLikelyDuration),
                "Pessimistic Duration" => Some(PessimisticDuration),
                "Adjusted Quantity" => Some(AdjustedQuantity),
                "Accidents" => Some(Accidents),
                "Years in School" => Some(YearsInSchool),
                "Number of Dependents" => Some(NumberOfDependents),
                "Years on Job" => Some(YearsOnJob),
                "Unacknowledged Quantity" => Some(UnacknowledgedQuantity),
                "Urgent Delivery Quantity" => Some(UrgentDeliveryQuantity),
                "Voice Storage Quantity" => Some(VoiceStorageQuantity),
                "Maintenance Units" => Some(MaintenanceUnits),
                "Minimum Average Time Requirement (MATR) Units" => Some(CodeAE),
                "Wide Area Telephone Service (WATS)/800 Service Units" => Some(CodeAF),
                "Number of End Users" => Some(NumberOfEndUsers),
                "Number of Message Recipients" => Some(NumberOfMessageRecipients),
                "Number of Operator Credits" => Some(NumberOfOperatorCredits),
                "Daily Adjustments" => Some(DailyAdjustments),
                "Years in this Line of Work/Profession" => {
                    Some(YearsInThisLineOfWorkProfession)
                }
                "Area per Units" => Some(AreaPerUnits),
                "Trainable Mentally Retarded Individuals" => {
                    Some(TrainableMentallyRetardedIndividuals)
                }
                "Age at Death" => Some(AgeAtDeath),
                "Verified Receipts" => Some(VerifiedReceipts),
                "Order Quantity Multiple" => Some(OrderQuantityMultiple),
                "Contribution Total" => Some(ContributionTotal),
                "Loan Repayment Total" => Some(LoanRepaymentTotal),
                "Participant Total" => Some(ParticipantTotal),
                "Actual" => Some(Actual),
                "Cumulative Actual" => Some(CumulativeActual),
                "Budget" => Some(Budget),
                "Cumulative Budget" => Some(CumulativeBudget),
                "Number of Insured Lives" => Some(NumberOfInsuredLives),
                "Forecast" => Some(Forecast),
                "Forecast at Complete" => Some(ForecastAtComplete),
                "Number of Mortgagors" => Some(NumberOfMortgagors),
                "Mortgage Pool Count" => Some(MortgagePoolCount),
                "Requested Amount" => Some(RequestedAmount),
                "Approved Amount" => Some(ApprovedAmount),
                "Additional Amount" => Some(AdditionalAmount),
                "Pre-op Days" => Some(PreOpDays),
                "Post-op Days" => Some(PostOpDays),
                "Average" => Some(Average),
                "Period Beginning Imbalance Quantity" => {
                    Some(PeriodBeginningImbalanceQuantity)
                }
                "Due-In" => Some(DueIn),
                "Contractor Cumulative to Date" => Some(ContractorCumulativeToDate),
                "Budget At Complete" => Some(BudgetAtComplete),
                "Contractor at Complete" => Some(ContractorAtComplete),
                "Subcontractor Cumulative to Date" => Some(SubcontractorCumulativeToDate),
                "Age Modifying Units" => Some(AgeModifyingUnits),
                "Subcontractor at Complete" => Some(SubcontractorAtComplete),
                "Book Order Quantity" => Some(BookOrderQuantity),
                "Book Inventory" => Some(BookInventory),
                "Bedroom Count" => Some(BedroomCount),
                "Bathroom Count" => Some(BathroomCount),
                "Betterment Hours" => Some(BettermentHours),
                "Depreciation Hours" => Some(DepreciationHours),
                "System Adjusted Hours" => Some(SystemAdjustedHours),
                "User Adjusted Hours" => Some(UserAdjustedHours),
                "Period Ending Imbalance Quantity" => Some(PeriodEndingImbalanceQuantity),
                "Backorder Quantity" => Some(BackorderQuantity),
                "Blood Record" => Some(BloodRecord),
                "Cumulative Beginning Imbalance Quantity" => {
                    Some(CumulativeBeginningImbalanceQuantity)
                }
                "Cumulative Current Period Imbalance Quantity" => {
                    Some(CumulativeCurrentPeriodImbalanceQuantity)
                }
                "Cumulative Prior Period Adjustment" => {
                    Some(CumulativePriorPeriodAdjustment)
                }
                "Cumulative Ending Imbalance Quantity" => {
                    Some(CumulativeEndingImbalanceQuantity)
                }
                "Birth Weight" => Some(BirthWeight),
                "Current Period Imbalance Quantity" => {
                    Some(CurrentPeriodImbalanceQuantity)
                }
                "Production Delivery Quantity" => Some(ProductionDeliveryQuantity),
                "Entitlement Quantity" => Some(EntitlementQuantity),
                "Creditors" => Some(Creditors),
                "Payment Experiences in Last 12 Months" => {
                    Some(PaymentExperiencesInLast12Months)
                }
                "Payment Experiences in Last 3 Months" => {
                    Some(PaymentExperiencesInLast3Months)
                }
                "Area Damaged" => Some(AreaDamaged),
                "Other Unlisted Stockholders" => Some(OtherUnlistedStockholders),
                "Other Unlisted Participants" => Some(OtherUnlistedParticipants),
                "Covered - Actual" => Some(CoveredActual),
                "Closing Statement Balance" => Some(ClosingStatementBalance),
                "Current Days on Market" => Some(CurrentDaysOnMarket),
                "Co-insured - Actual" => Some(CoInsuredActual),
                "Covered - Estimated" => Some(CoveredEstimated),
                "Co-insured - Estimated" => Some(CoInsuredEstimated),
                "Cumulative Gas Volume" => Some(CumulativeGasVolume),
                "Cumulative Effect of Prior Period Adjustment" => {
                    Some(CumulativeEffectOfPriorPeriodAdjustment)
                }
                "Cumulative Gas Injection Volume" => Some(CumulativeGasInjectionVolume),
                "Cumulative Liquid Injection Volume" => {
                    Some(CumulativeLiquidInjectionVolume)
                }
                "Number of Components" => Some(NumberOfComponents),
                "Continuance Duration" => Some(ContinuanceDuration),
                "Cumulative Oil/Condensate Volume" => Some(CumulativeOilCondensateVolume),
                "Current Period Imbalance" => Some(CurrentPeriodImbalance),
                "Certified Registered Nurse Anesthetist (CRNA) Number of Concurrent Procedures" => {
                    Some(CodeCR)
                }
                "Current Service Life" => Some(CurrentServiceLife),
                "Cumulative Water Volume" => Some(CumulativeWaterVolume),
                "Convictions Sent" => Some(ConvictionsSent),
                "Total Number of Convictions" => Some(TotalNumberOfConvictions),
                "Engineers" => Some(Engineers),
                "Billed" => Some(Billed),
                "Executives" => Some(Executives),
                "Number of Co-insurance Days" => Some(NumberOfCoInsuranceDays),
                "Field Workers" => Some(FieldWorkers),
                "Installers" => Some(Installers),
                "Members in Group" => Some(MembersInGroup),
                "Non-Consolidated Total-Domestic Subsidiaries" => {
                    Some(NonConsolidatedTotalDomesticSubsidiaries)
                }
                "Non-Consolidated Total-Foreign Subsidiaries" => {
                    Some(NonConsolidatedTotalForeignSubsidiaries)
                }
                "Non-Union Employees" => Some(NonUnionEmployees),
                "Dependent's Age" => Some(DependentsAge),
                "Deductible Blood Units" => Some(DeductibleBloodUnits),
                "Dependent Count" => Some(DependentCount),
                "Distributed" => Some(Distributed),
                "Debited" => Some(Debited),
                "Deleted" => Some(Deleted),
                "Gas Used for Drilling" => Some(GasUsedForDrilling),
                "Maximum Benefit Period Accident to Age" => {
                    Some(MaximumBenefitPeriodAccidentToAge)
                }
                "Disposed" => Some(Disposed),
                "Maximum Benefit Period Sickness to Age" => {
                    Some(MaximumBenefitPeriodSicknessToAge)
                }
                "Airline Attendants" => Some(AirlineAttendants),
                "Companies Included in Consolidation" => {
                    Some(CompaniesIncludedInConsolidation)
                }
                "Total Consolidated Domestic Subsidiaries" => {
                    Some(TotalConsolidatedDomesticSubsidiaries)
                }
                "Default Notification Response Period" => {
                    Some(DefaultNotificationResponsePeriod)
                }
                "Days Operated" => Some(DaysOperated),
                "Days Produced" => Some(DaysProduced),
                "Total Consolidated Foreign Subsidiaries" => {
                    Some(TotalConsolidatedForeignSubsidiaries)
                }
                "Direct Workers" => Some(DirectWorkers),
                "Dose" => Some(Dose),
                "Dependent Total" => Some(DependentTotal),
                "Counter Clerks" => Some(CounterClerks),
                "Design Capacity" => Some(DesignCapacity),
                "Domestic Affiliated Companies" => Some(DomesticAffiliatedCompanies),
                "Drivers" => Some(Drivers),
                "Days" => Some(Days),
                "Employed at Location" => Some(EmployedAtLocation),
                "Course Segments" => Some(CourseSegments),
                "Degree Segments" => Some(DegreeSegments),
                "Employed on this job" => Some(EmployedOnThisJob),
                "Employed in this Profession" => Some(EmployedInThisProfession),
                "Employed by this Company" => Some(EmployedByThisCompany),
                "Number of Entitled Exemptions" => Some(NumberOfEntitledExemptions),
                "Number of Withholding Exemptions" => Some(NumberOfWithholdingExemptions),
                "Exclusive Uses" => Some(ExclusiveUses),
                "Nonexclusive Uses" => Some(NonexclusiveUses),
                "Use of Extracorporeal Circulation" => {
                    Some(UseOfExtracorporealCirculation)
                }
                "Domestic Uses" => Some(DomesticUses),
                "Small Business Uses" => Some(SmallBusinessUses),
                "Nurses" => Some(Nurses),
                "Office Workers" => Some(OfficeWorkers),
                "Paid in Common Shares" => Some(PaidInCommonShares),
                "Paid in Preferred Shares" => Some(PaidInPreferredShares),
                "Pilots" => Some(Pilots),
                "Plant Workers" => Some(PlantWorkers),
                "Principals Included as Employees" => Some(PrincipalsIncludedAsEmployees),
                "Emergency Modifying Units" => Some(EmergencyModifyingUnits),
                "Suppliers" => Some(Suppliers),
                "Teachers" => Some(Teachers),
                "Product Exchange Amount" => Some(ProductExchangeAmount),
                "Equity Security Holder" => Some(EquitySecurityHolder),
                "Estimated Remaining Economic Life" => {
                    Some(EstimatedRemainingEconomicLife)
                }
                "Ending Stock" => Some(EndingStock),
                "Employee Total" => Some(EmployeeTotal),
                "Total Consolidated Subsidiaries" => Some(TotalConsolidatedSubsidiaries),
                "Total Non-Consolidated Subsidiaries" => {
                    Some(TotalNonConsolidatedSubsidiaries)
                }
                "Evaporated Water" => Some(EvaporatedWater),
                "Union Employees" => Some(UnionEmployees),
                "Ported Telephone Lines" => Some(PortedTelephoneLines),
                "Service Resale" => Some(ServiceResale),
                "Total claims with skin diseases or disorders" => {
                    Some(TotalClaimsWithSkinDiseasesOrDisorders)
                }
                "Off Lease Fuel" => Some(OffLeaseFuel),
                "Total deaths as a Result of Injury" => {
                    Some(TotalDeathsAsAResultOfInjury)
                }
                "Total deaths as a Result of Illness" => {
                    Some(TotalDeathsAsAResultOfIllness)
                }
                "Total injury Claims with Days Away from Work or Restricted Work Activity" => {
                    Some(TotalInjuryClaimsWithDaysAwayFromWorkOrRestrictedWorkActivity)
                }
                "Total injury Claims with Days Away from Work" => {
                    Some(TotalInjuryClaimsWithDaysAwayFromWork)
                }
                "Total injury Claims without Lost Work Days" => {
                    Some(TotalInjuryClaimsWithoutLostWorkDays)
                }
                "Total Days Away from Work Due to Injury" => {
                    Some(TotalDaysAwayFromWorkDueToInjury)
                }
                "Total Days with Restricted Work Activity Due to Injury" => {
                    Some(TotalDaysWithRestrictedWorkActivityDueToInjury)
                }
                "Full Baths" => Some(FullBaths),
                "Furnished Blood Units" => Some(FurnishedBloodUnits),
                "Fuel Consumed or Burned Amount" => Some(FuelConsumedOrBurnedAmount),
                "Vehicular Radios" => Some(VehicularRadios),
                "Portable Radios" => Some(PortableRadios),
                "Flare or Flash" => Some(FlareOrFlash),
                "Marine Radios" => Some(MarineRadios),
                "Pagers" => Some(Pagers),
                "Conventional Mobiles" => Some(ConventionalMobiles),
                "Trunked Channels" => Some(TrunkedChannels),
                "Mobile Loading Allocation" => Some(MobileLoadingAllocation),
                "Units" => Some(Units),
                "Aircraft Radios" => Some(AircraftRadios),
                "Total Claims with Dust Diseases of the Lungs" => {
                    Some(TotalClaimsWithDustDiseasesOfTheLungs)
                }
                "Total Claims with Respiratory Conditions Due to Toxic Agents" => {
                    Some(TotalClaimsWithRespiratoryConditionsDueToToxicAgents)
                }
                "Total Claims with Poisoning Illnesses" => {
                    Some(TotalClaimsWithPoisoningIllnesses)
                }
                "Total Claims with Disorders Due to Physical Agents" => {
                    Some(TotalClaimsWithDisordersDueToPhysicalAgents)
                }
                "Gas Used for Fuel System" => Some(GasUsedForFuelSystem),
                "Forecast to Complete" => Some(ForecastToComplete),
                "Total Claims Associated with Repeated Trauma" => {
                    Some(TotalClaimsAssociatedWithRepeatedTrauma)
                }
                "Total illness Claims with occupational illnesses not otherwise classified" => {
                    Some(
                        TotalIllnessClaimsWithOccupationalIllnessesNotOtherwiseClassified,
                    )
                }
                "Total Days Away from Work Due to Illness" => {
                    Some(TotalDaysAwayFromWorkDueToIllness)
                }
                "Total Days of Restricted Work Activity Due to Illness" => {
                    Some(TotalDaysOfRestrictedWorkActivityDueToIllness)
                }
                "Total illness with Lost Work Days or Restricted Work Activity" => {
                    Some(TotalIllnessWithLostWorkDaysOrRestrictedWorkActivity)
                }
                "Total illness Claims with Days Away from Work" => {
                    Some(TotalIllnessClaimsWithDaysAwayFromWork)
                }
                "Discharge Quantity" => Some(DischargeQuantity),
                "Estimated Discharge Quantity" => Some(EstimatedDischargeQuantity),
                "Estimated Transfer Quantity" => Some(EstimatedTransferQuantity),
                "Excursions" => Some(Excursions),
                "Non-production Quantity" => Some(NonProductionQuantity),
                "Number of Deaths" => Some(NumberOfDeaths),
                "Number of Hospitalizations" => Some(NumberOfHospitalizations),
                "Number of Injuries" => Some(NumberOfInjuries),
                "Number of Injuries Requiring Medical Treatment" => {
                    Some(NumberOfInjuriesRequiringMedicalTreatment)
                }
                "Number of People Evacuated" => Some(NumberOfPeopleEvacuated),
                "Gross Building Area" => Some(GrossBuildingArea),
                "Gross Annual Income Multiplier" => Some(GrossAnnualIncomeMultiplier),
                "Gross Living Area" => Some(GrossLivingArea),
                "Total illness Claims without Lost Work Days" => {
                    Some(TotalIllnessClaimsWithoutLostWorkDays)
                }
                "Original Term In Years" => Some(OriginalTermInYears),
                "Years Remaining" => Some(YearsRemaining),
                "Average Number of Employees" => Some(AverageNumberOfEmployees),
                "Total Worked by All Employees" => Some(TotalWorkedByAllEmployees),
                "Gas Injection Volume" => Some(GasInjectionVolume),
                "Gas Lift Volume" => Some(GasLiftVolume),
                "Episode" => Some(Episode),
                "Period(s)" => Some(CodeGN),
                "Session(s)" => Some(CodeGO),
                "Gross Production" => Some(GrossProduction),
                "Government Reporting Quantity" => Some(GovernmentReportingQuantity),
                "Gas Receipt Volume" => Some(GasReceiptVolume),
                "Gas Sold" => Some(GasSold),
                "Grade Transfer Amount" => Some(GradeTransferAmount),
                "Employee Total First Month of Quarter" => {
                    Some(EmployeeTotalFirstMonthOfQuarter)
                }
                "Gas Volume" => Some(GasVolume),
                "Employee Total Second Month of Quarter" => {
                    Some(EmployeeTotalSecondMonthOfQuarter)
                }
                "Employee Total Third Month of Quarter" => {
                    Some(EmployeeTotalThirdMonthOfQuarter)
                }
                "Active Listings" => Some(ActiveListings),
                "Number of People Sheltered-in-Place" => {
                    Some(NumberOfPeopleShelteredInPlace)
                }
                "Quantity Recovered" => Some(QuantityRecovered),
                "Quantity Recycled" => Some(QuantityRecycled),
                "Quantity Released" => Some(QuantityReleased),
                "Quantity Treated" => Some(QuantityTreated),
                "Total Hazardous Waste Generated" => Some(TotalHazardousWasteGenerated),
                "Operational Quantity" => Some(OperationalQuantity),
                "Penalty Variance Quantity" => Some(PenaltyVarianceQuantity),
                "Allocated Quantity" => Some(AllocatedQuantity),
                "Scheduled Quantity" => Some(ScheduledQuantity),
                "Market Price Change" => Some(MarketPriceChange),
                "Unpaid" => Some(Unpaid),
                "Branches" => Some(Branches),
                "Subsidiaries" => Some(Subsidiaries),
                "Age of Financial Information" => Some(AgeOfFinancial),
                "Invoices" => Some(Invoices),
                "Financial Coverage Period" => Some(FinancialCoveragePeriod),
                "Maximum Number of Employees at Location" => {
                    Some(MaximumNumberOfEmployeesAtLocation)
                }
                "Previous Number of Accounts" => Some(PreviousNumberOfAccounts),
                "Collection Period" => Some(CollectionPeriod),
                "Disbursement Period" => Some(DisbursementPeriod),
                "Seats" => Some(Seats),
                "Use of Hypothermia" => Some(UseOfHypothermia),
                "Previous Number of Employees" => Some(PreviousNumberOfEmployees),
                "Use of Hypotension" => Some(UseOfHypotension),
                "Use of Hyperbaric Pressurization" => Some(UseOfHyperbaricPressurization),
                "Kindergarten Students" => Some(KindergartenStudents),
                "Use of Hypertension" => Some(UseOfHypertension),
                "Hours" => Some(Hours),
                "Employee's Age" => Some(EmployeesAge),
                "Employee's Number of Days Away from Work Due to Injury" => {
                    Some(EmployeesNumberOfDaysAwayFromWorkDueToInjury)
                }
                "Employee's Number of Days of Restricted Work Activity Due to Injury" => {
                    Some(EmployeesNumberOfDaysOfRestrictedWorkActivityDueToInjury)
                }
                "Employee's Total Number of Days Away from Work Due to Illness" => {
                    Some(EmployeesTotalNumberOfDaysAwayFromWorkDueToIllness)
                }
                "Total Death Claims" => Some(TotalDeathClaims),
                "Total Claims with Days Away from Work" => {
                    Some(TotalClaimsWithDaysAwayFromWork)
                }
                "Tenth Grade Students" => Some(TenthGradeStudents),
                "Eleventh Grade Students" => Some(EleventhGradeStudents),
                "Twelfth Grade Students" => Some(TwelfthGradeStudents),
                "Prior Teaching Experience" => Some(PriorTeachingExperience),
                "Prior Full-time Teaching Experience" => {
                    Some(PriorFullTimeTeachingExperience)
                }
                "Prior Part-time Teaching Experience" => {
                    Some(PriorPartTimeTeachingExperience)
                }
                "Prior Experience in Education" => Some(PriorExperienceInEducation),
                "Prior Full-time Experience in Education" => {
                    Some(PriorFullTimeExperienceInEducation)
                }
                "Prior Part-time Experience in Education" => {
                    Some(PriorPartTimeExperienceInEducation)
                }
                "Prior Experience Related to Job" => Some(PriorExperienceRelatedToJob),
                "Local Country Employees" => Some(LocalCountryEmployees),
                "Foreign Employees" => Some(ForeignEmployees),
                "Prior Full-time Experience Related to Job" => {
                    Some(PriorFullTimeExperienceRelatedToJob)
                }
                "Prior Part-time Experience Related to Job" => {
                    Some(PriorPartTimeExperienceRelatedToJob)
                }
                "Total Prior Experience" => Some(TotalPriorExperience),
                "Total Full-time Prior Experience" => Some(TotalFullTimePriorExperience),
                "Total Part-time Prior Experience" => Some(TotalPartTimePriorExperience),
                "Total Years of Educational Service" => {
                    Some(TotalYearsOfEducationalService)
                }
                "Number of Irregular Interest Payments" => {
                    Some(NumberOfIrregularInterestPayments)
                }
                "Total Years of Educational Service in this District" => {
                    Some(TotalYearsOfEducationalServiceInThisDistrict)
                }
                "Years of Experience as School Principal" => {
                    Some(YearsOfExperienceAsSchoolPrincipal)
                }
                "Years of Experience as Classroom Teacher" => {
                    Some(YearsOfExperienceAsClassroomTeacher)
                }
                "Years Worked for this System" => Some(YearsWorkedForThisSystem),
                "Indirect Workers" => Some(IndirectWorkers),
                "Number of Interest Payments" => Some(NumberOfInterestPayments),
                "In-Transit Quantity" => Some(InTransitQuantity),
                "Information Provider Standardized Motor Vehicle Penalty Points" => {
                    Some(InformationProviderStandardizedMotorVehiclePenaltyPoints)
                }
                "Intertank Transfer Amount" => Some(IntertankTransferAmount),
                "Ending Storage Balance" => Some(EndingStorageBalance),
                "Location Ending Storage Balance" => Some(LocationEndingStorageBalance),
                "Location Ending Storage Balance - Firm" => {
                    Some(LocationEndingStorageBalanceFirm)
                }
                "Location Ending Storage Balance - Interruptible" => {
                    Some(LocationEndingStorageBalanceInterruptible)
                }
                "Maximum Available Daily Injection Quantity" => {
                    Some(MaximumAvailableDailyInjectionQuantity)
                }
                "Maximum Available Daily Withdrawal Quantity" => {
                    Some(MaximumAvailableDailyWithdrawalQuantity)
                }
                "Minimum Required Daily Injection Quantity" => {
                    Some(MinimumRequiredDailyInjectionQuantity)
                }
                "Minimum Required Daily Withdrawal Quantity" => {
                    Some(MinimumRequiredDailyWithdrawalQuantity)
                }
                "Activity Codes" => Some(ActivityCodes),
                "Associates" => Some(Associates),
                "Average Employees" => Some(AverageEmployees),
                "Cooperative Shares" => Some(CooperativeShares),
                "Estimated Employees at Location" => Some(EstimatedEmployeesAtLocation),
                "Estimated Total Employees" => Some(EstimatedTotalEmployees),
                "Financial Institutions" => Some(FinancialInstitutions),
                "Judgments" => Some(Judgments),
                "Land Size" => Some(LandSize),
                "Liens" => Some(Liens),
                "Minimum Employees at Location" => Some(MinimumEmployeesAtLocation),
                "Office Size" => Some(OfficeSize),
                "Owner" => Some(Owner),
                "Plant Size" => Some(PlantSize),
                "Previous Number of Branches" => Some(PreviousNumberOfBranches),
                "Protested Bills" => Some(ProtestedBills),
                "Suits" => Some(Suits),
                "Uniform Commercial Code (UCC) Filings" => Some(CodeJR),
                "Judicial Stay Duration" => Some(JudicialStayDuration),
                "Warehouse Size" => Some(WarehouseSize),
                "Total Days Away from Work" => Some(TotalDaysAwayFromWork),
                "Total Days of Restricted Work Activity" => {
                    Some(TotalDaysOfRestrictedWorkActivity)
                }
                "Total Claims without Days Away from Work and without Restricted Work Activity" => {
                    Some(
                        TotalClaimsWithoutDaysAwayFromWorkAndWithoutRestrictedWorkActivity,
                    )
                }
                "Secretaries" => Some(Secretaries),
                "Mechanics" => Some(Mechanics),
                "Auditors" => Some(Auditors),
                "Messengers" => Some(Messengers),
                "Primary Managers" => Some(PrimaryManagers),
                "Participation Shares" => Some(ParticipationShares),
                "Detrimental Legal Filings" => Some(DetrimentalLegalFilings),
                "Petitions Filed" => Some(PetitionsFiled),
                "Drafts" => Some(Drafts),
                "Business Failure National Average Incidence" => {
                    Some(BusinessFailureNationalAverageIncidence)
                }
                "Business Failure Industry Incidence" => {
                    Some(BusinessFailureIndustryIncidence)
                }
                "Business Failure Class Incidence" => Some(BusinessFailureClassIncidence),
                "Estimated" => Some(Estimated),
                "Net Quantity Increase" => Some(NetQuantityIncrease),
                "Net Quantity Decrease" => Some(NetQuantityDecrease),
                "Expenditure Quantity" => Some(ExpenditureQuantity),
                "Originals" => Some(Originals),
                "Duplicates" => Some(Duplicates),
                "Completed Line Items" => Some(CompletedLineItems),
                "Completed Contracts" => Some(CompletedContracts),
                "Active Contracts Delinquent-Buying Party Caused" => {
                    Some(ActiveContractsDelinquentBuyingPartyCaused)
                }
                "Active Contracts Delinquent" => Some(ActiveContractsDelinquent),
                "Active Contracts Delinquent-Contractor Caused" => {
                    Some(ActiveContractsDelinquentContractorCaused)
                }
                "Active Contracts Delinquent-Unknown Causes" => {
                    Some(ActiveContractsDelinquentUnknownCauses)
                }
                "Active Line Items Delinquent" => Some(ActiveLineItemsDelinquent),
                "Active Line Items Delinquent-Buying Party Caused" => {
                    Some(ActiveLineItemsDelinquentBuyingPartyCaused)
                }
                "Active Line Items Delinquent-Contractor Caused" => {
                    Some(ActiveLineItemsDelinquentContractorCaused)
                }
                "Active Line Items Delinquent-Unknown Causes" => {
                    Some(ActiveLineItemsDelinquentUnknownCauses)
                }
                "Contracts Completed Delinquent-Buying Party Caused" => {
                    Some(ContractsCompletedDelinquentBuyingPartyCaused)
                }
                "Contract Completed Delinquent-Contractor Caused" => {
                    Some(ContractCompletedDelinquentContractorCaused)
                }
                "Contracts Completed Delinquent-Unknown Causes" => {
                    Some(ContractsCompletedDelinquentUnknownCauses)
                }
                "Reported Deficiencies" => Some(ReportedDeficiencies),
                "Line Items Completed Delinquent-Buying Party Caused" => {
                    Some(LineItemsCompletedDelinquentBuyingPartyCaused)
                }
                "Line Items Completed Delinquent-Contractor Caused" => {
                    Some(LineItemsCompletedDelinquentContractorCaused)
                }
                "Line Items Completed Delinquent-Unknown Causes" => {
                    Some(LineItemsCompletedDelinquentUnknownCauses)
                }
                "Corrective Action Requests-Verbal" => {
                    Some(CorrectiveActionRequestsVerbal)
                }
                "Corrective Action Requests-Written" => {
                    Some(CorrectiveActionRequestsWritten)
                }
                "Guarantee Fee Buyup Maximum" => Some(GuaranteeFeeBuyupMaximum),
                "Contract Buyup" => Some(ContractBuyup),
                "Contract Buydown" => Some(ContractBuydown),
                "Guarantee Fee Rate after Alternate Payment Method" => {
                    Some(GuaranteeFeeRateAfterAlternatePaymentMethod)
                }
                "Guarantee Fee Rate after Buyup or Buydown" => {
                    Some(GuaranteeFeeRateAfterBuyupOrBuydown)
                }
                "Buyup or Buydown Rate per Basis Point" => {
                    Some(BuyupOrBuydownRatePerBasisPoint)
                }
                "Location Net Capacity" => Some(LocationNetCapacity),
                "Subject to loss or elimination" => Some(SubjectToLossOrElimination),
                "Life-time Reserve - Actual" => Some(LifeTimeReserveActual),
                "Loss Allowance" => Some(LossAllowance),
                "Late Payment Period" => Some(LatePaymentPeriod),
                "Limit Value" => Some(LimitValue),
                "Life-time Reserve - Estimated" => Some(LifeTimeReserveEstimated),
                "Loss or Gain" => Some(LossOrGain),
                "Lost Gas" => Some(LostGas),
                "Liquid Injection Volume" => Some(LiquidInjectionVolume),
                "Corrective Action Requests-Method C" => {
                    Some(CorrectiveActionRequestsMethodC)
                }
                "Corrective Action Requests-Method D" => {
                    Some(CorrectiveActionRequestsMethodD)
                }
                "Corrective Action Requests-Method E" => {
                    Some(CorrectiveActionRequestsMethodE)
                }
                "Aged Active Line Items Delinquent-Contractor Caused" => {
                    Some(AgedActiveLineItemsDelinquentContractorCaused)
                }
                "Lost Oil" => Some(LostOil),
                "Lease Periods" => Some(LeasePeriods),
                "Aged Line Items Delinquent" => Some(AgedLineItemsDelinquent),
                "Aged Line Items Completed-Contractor Caused" => {
                    Some(AgedLineItemsCompletedContractorCaused)
                }
                "Oil Condensate Sold" => Some(OilCondensateSold),
                "Tariff Loss Allowance" => Some(TariffLossAllowance),
                "Lifetime Reserve Days - Applied to this Claim" => {
                    Some(LifetimeReserveDaysAppliedToThisClaim)
                }
                "Oil/Condensate Volume" => Some(OilCondensateVolume),
                "Lost Work Time Actual" => Some(LostWorkTimeActual),
                "Lost Work Time Estimated" => Some(LostWorkTimeEstimated),
                "Length of Residency" => Some(LengthOfResidency),
                "Lanes" => Some(Lanes),
                "Matching Equipment" => Some(MatchingEquipment),
                "Maximum" => Some(Maximum),
                "Total Federal Points" => Some(TotalFederalPoints),
                "Contributions" => Some(Contributions),
                "Contributors" => Some(Contributors),
                "Endorsers" => Some(Endorsers),
                "Functions" => Some(Functions),
                "Guarantors" => Some(Guarantors),
                "Points" => Some(Points),
                "Miscellaneous Allowance" => Some(MiscellaneousAllowance),
                "Number of Public Officials" => Some(NumberOfPublicOfficials),
                "Total Non-Federal Points" => Some(TotalNonFederalPoints),
                "Million Dollar Roundtable Credits" => {
                    Some(MillionDollarRoundtableCredits)
                }
                "Minimum Number of Employees" => Some(MinimumNumberOfEmployees),
                "Manufactured" => Some(Manufactured),
                "Pledges" => Some(Pledges),
                "Total Points" => Some(TotalPoints),
                "Miles" => Some(Miles),
                "Attendees" => Some(Attendees),
                "Tickets Sold" => Some(TicketsSold),
                "Total Number of Manifest Lines" => Some(TotalNumberOfManifestLines),
                "Maximum Maturity Extension" => Some(MaximumMaturityExtension),
                "Month" => Some(Month),
                "Minimum Order Package Level" => Some(MinimumOrderPackageLevel),
                "Total Number of Maps in a Pack" => Some(TotalNumberOfMapsInAPack),
                "Maximum Ship Quantity" => Some(MaximumShipQuantity),
                "Quantity of next lower level trade item" => {
                    Some(QuantityOfNextLowerLevelTradeItem)
                }
                "Measured Quantity" => Some(MeasuredQuantity),
                "Resterilization Maximum" => Some(ResterilizationMaximum),
                "Recommended Number of Uses" => Some(RecommendedNumberOfUses),
                "Total Units" => Some(TotalUnits),
                "Maximum Number of Employees" => Some(MaximumNumberOfEmployees),
                "Stacking Factor" => Some(StackingFactor),
                "Component Quantity" => Some(ComponentQuantity),
                "Number of Attacks or Occurrences" => Some(NumberOfAttacksOrOccurrences),
                "Number of Dead" => Some(NumberOfDead),
                "Number of Living" => Some(NumberOfLiving),
                "Number of Times" => Some(NumberOfTimes),
                "Minimum Forecast Quantity" => Some(MinimumForecastQuantity),
                "Maximum Forecast Quantity" => Some(MaximumForecastQuantity),
                "Requested Receipt Quantity" => Some(RequestedReceiptQuantity),
                "Requested Delivery Quantity" => Some(RequestedDeliveryQuantity),
                "Number of Non-covered Days" => Some(NumberOfNonCoveredDays),
                "Number of Units (Housing)" => Some(CodeNB),
                "Number of Claimants" => Some(NumberOfClaimants),
                "Number of Late Charges" => Some(NumberOfLateCharges),
                "Non-Covered - Estimated" => Some(NonCoveredEstimated),
                "Number of Full-Time Employees" => Some(NumberOfFullTimeEmployees),
                "Number of Nonsufficient Fund Items" => {
                    Some(NumberOfNonsufficientFundItems)
                }
                "Noncovered - Actual" => Some(NoncoveredActual),
                "Number of Levels" => Some(NumberOfLevels),
                "Number of Hospitals" => Some(NumberOfHospitals),
                "Number of Physicians" => Some(NumberOfPhysicians),
                "Number of Members" => Some(NumberOfMembers),
                "Number of Franchisees" => Some(NumberOfFranchisees),
                "Not Replaced Blood Units" => Some(NotReplacedBloodUnits),
                "Number of Stations" => Some(NumberOfStations),
                "Reports" => Some(Reports),
                "Since Last Travel" => Some(SinceLastTravel),
                "Net" => Some(Net),
                "Until Next Travel" => Some(UntilNextTravel),
                "Scheduled Receipt" => Some(ScheduledReceipt),
                "Scheduled Delivery" => Some(ScheduledDelivery),
                "Operational Receipt" => Some(OperationalReceipt),
                "Operational Delivery" => Some(OperationalDelivery),
                "Allocated Receipt" => Some(AllocatedReceipt),
                "Allocated Delivery" => Some(AllocatedDelivery),
                "Distributed Confirmed Receipt" => Some(DistributedConfirmedReceipt),
                "Distributed Confirmed Delivery" => Some(DistributedConfirmedDelivery),
                "Scheduling Tolerance Receipt" => Some(SchedulingToleranceReceipt),
                "Scheduling Tolerance Delivery" => Some(SchedulingToleranceDelivery),
                "Energy" => Some(Energy),
                "Order Count" => Some(OrderCount),
                "Other Miscellaneous Disposition" => Some(OtherMiscellaneousDisposition),
                "Number of Weeks Per Year" => Some(NumberOfWeeksPerYear),
                "Off Premise Sales Quantity" => Some(OffPremiseSalesQuantity),
                "Other Gas Disposition" => Some(OtherGasDisposition),
                "Other Injection Volume" => Some(OtherInjectionVolume),
                "Opening Statement Balance" => Some(OpeningStatementBalance),
                "Order Sizing Factor" => Some(OrderSizingFactor),
                "Original Loan Term" => Some(OriginalLoanTerm),
                "On Premise Sales Quantity" => Some(OnPremiseSalesQuantity),
                "Other Oil Condensate Disposition" => Some(OtherOilCondensateDisposition),
                "Optimum Order Quantity" => Some(OptimumOrderQuantity),
                "Original" => Some(Original),
                "Number of Operating Periods at Failure" => {
                    Some(NumberOfOperatingPeriodsAtFailure)
                }
                "Outlier Days" => Some(OutlierDays),
                "Overage" => Some(Overage),
                "Other Water Disposition" => Some(OtherWaterDisposition),
                "Project Phases" => Some(ProjectPhases),
                "Physical Status III" => Some(PhysicalStatusIii),
                "Physical Status IV" => Some(PhysicalStatusIv),
                "Physical Status V" => Some(PhysicalStatusV),
                "Number of Services or Procedures" => Some(NumberOfServicesOrProcedures),
                "Prescription Dosage" => Some(PrescriptionDosage),
                "Prescription Frequency" => Some(PrescriptionFrequency),
                "Number of People Living at Residence" => {
                    Some(NumberOfPeopleLivingAtResidence)
                }
                "Pipeline Adjustment or Allowance" => Some(PipelineAdjustmentOrAllowance),
                "Pressure Base" => Some(PressureBase),
                "Prior Cumulative Imbalance" => Some(PriorCumulativeImbalance),
                "Payment Duration Weeks" => Some(PaymentDurationWeeks),
                "Period of Employment" => Some(PeriodOfEmployment),
                "Gas Used for Plant Fuel" => Some(GasUsedForPlantFuel),
                "Persistency" => Some(Persistency),
                "Promotional" => Some(Promotional),
                "Parking Spaces" => Some(ParkingSpaces),
                "Partial Baths" => Some(PartialBaths),
                "Percentage of Ordered Quantity" => Some(PercentageOfOrderedQuantity),
                "Purchase of Product" => Some(PurchaseOfProduct),
                "Cumulative Quantity Required Prior to the First Scheduled Period" => {
                    Some(CumulativeQuantityRequiredPriorToTheFirstScheduledPeriod)
                }
                "Requirement Quantity that was Previously Released" => {
                    Some(RequirementQuantityThatWasPreviouslyReleased)
                }
                "Prescription" => Some(Prescription),
                "Patients" => Some(Patients),
                "Pitted Water" => Some(PittedWater),
                "Prior Units Accepted" => Some(PriorUnitsAccepted),
                "Paid" => Some(Paid),
                "Minimum quantity to which tax rate applies" => {
                    Some(MinimumQuantityToWhichTaxRateApplies)
                }
                "Maximum quantity to which tax rate applies" => {
                    Some(MaximumQuantityToWhichTaxRateApplies)
                }
                "Quantity Earned" => Some(QuantityEarned),
                "Quantity Carried Forward" => Some(QuantityCarriedForward),
                "Number of 3 to 4 Year Olds" => Some(NumberOf3To4YearOlds),
                "Autistic Individuals" => Some(AutisticIndividuals),
                "Deaf-blind Individuals" => Some(DeafBlindIndividuals),
                "Hearing Impaired Individuals" => Some(HearingImpairedIndividuals),
                "Mentally Retarded Individuals" => Some(MentallyRetardedIndividuals),
                "Quantity Approved" => Some(QuantityApproved),
                "Quantity Dispensed" => Some(QuantityDispensed),
                "Quantity Disapproved" => Some(QuantityDisapproved),
                "Quantity Delivered" => Some(QuantityDelivered),
                "Quantity Deferred" => Some(QuantityDeferred),
                "High Fabrication Authorization Quantity" => {
                    Some(HighFabricationAuthorizationQuantity)
                }
                "Quantity on Hold" => Some(QuantityOnHold),
                "Community Service Duration" => Some(CommunityServiceDuration),
                "Number of Times Deported" => Some(NumberOfTimesDeported),
                "Quantity of Inner Packs" => Some(QuantityOfInnerPacks),
                "Jail Sentence Duration" => Some(JailSentenceDuration),
                "Probation Duration" => Some(ProbationDuration),
                "Restriction Duration" => Some(RestrictionDuration),
                "Operating Quantity" => Some(OperatingQuantity),
                "Quantity by Position" => Some(QuantityByPosition),
                "Suspended Duration" => Some(SuspendedDuration),
                "High Raw Material Authorization Quantity" => {
                    Some(HighRawMaterialAuthorizationQuantity)
                }
                "Quantity Per Skid" => Some(QuantityPerSkid),
                "Plant Thermal Reduction" => Some(PlantThermalReduction),
                "Quantity Serviced" => Some(QuantityServiced),
                "Quantity Cancelled" => Some(QuantityCancelled),
                "Quantity Withdrawn" => Some(QuantityWithdrawn),
                "Qualifying Weeks" => Some(QualifyingWeeks),
                "Repayment Plan Term" => Some(RepaymentPlanTerm),
                "Replenishment (Fill)" => Some(CodeR1),
                "Individuals with Orthopedic Impairment" => {
                    Some(IndividualsWithOrthopedicImpairment)
                }
                "Estimated Remaining Physical Life" => {
                    Some(EstimatedRemainingPhysicalLife)
                }
                "Individuals with Specific Learning Disability" => {
                    Some(IndividualsWithSpecificLearningDisability)
                }
                "Axles" => Some(Axles),
                "Platform Count" => Some(PlatformCount),
                "Individuals with Visual Impairment" => {
                    Some(IndividualsWithVisualImpairment)
                }
                "Individuals with Other Health Impairment" => {
                    Some(IndividualsWithOtherHealthImpairment)
                }
                "Fuel" => Some(Fuel),
                "Refills Authorized" => Some(RefillsAuthorized),
                "Replaced Blood Units" => Some(ReplacedBloodUnits),
                "Number of Items Authorized at Store" => {
                    Some(NumberOfItemsAuthorizedAtStore)
                }
                "Number of Items Authorized at Warehouse" => {
                    Some(NumberOfItemsAuthorizedAtWarehouse)
                }
                "Gas Returned to Earth" => Some(GasReturnedToEarth),
                "Number of Items in Stock" => Some(NumberOfItemsInStock),
                "Gas Used for Repressuring or Pressure Maintenance" => {
                    Some(GasUsedForRepressuringOrPressureMaintenance)
                }
                "Number of Shelf Tags" => Some(NumberOfShelfTags),
                "Quantity Available on Shelf" => Some(QuantityAvailableOnShelf),
                "Gas Returned to Property for fuel" => Some(GasReturnedToPropertyForFuel),
                "Room Count" => Some(RoomCount),
                "Units Rented" => Some(UnitsRented),
                "Retail Demand Quantity" => Some(RetailDemandQuantity),
                "Royalty" => Some(Royalty),
                "Number of Shelf Facings" => Some(NumberOfShelfFacings),
                "Retail Sales Quantity" => Some(RetailSalesQuantity),
                "Water Re-injected on Property" => Some(WaterReInjectedOnProperty),
                "Requirement Quantity" => Some(RequirementQuantity),
                "Planned Unit Development (PUD) Units" => Some(CodeS1),
                "Rooms, Finished Area Above Grade" => Some(CodeS2),
                "Dwelling Area" => Some(DwellingArea),
                "Garage or Carport Area" => Some(GarageOrCarportArea),
                "Units for Sale" => Some(UnitsForSale),
                "Gross Rent Multiplier" => Some(GrossRentMultiplier),
                "Age, High Value" => Some(CodeS7),
                "Age, Low Value" => Some(CodeS8),
                "Bedrooms, Finished Area Above Grade" => Some(CodeS9),
                "Shipments" => Some(Shipments),
                "Solicited" => Some(Solicited),
                "Bathrooms, Finished Area Above Grade" => Some(CodeSC),
                "Criminal Sentence Duration" => Some(CriminalSentenceDuration),
                "Gross Living, Finished Area Above Grade" => Some(CodeSE),
                "Site" => Some(Site),
                "Swan-Ganz" => Some(SwanGanz),
                "Shortage" => Some(Shortage),
                "Rooms" => Some(Rooms),
                "Area of Level" => Some(AreaOfLevel),
                "Gas Shrinkage" => Some(GasShrinkage),
                "Predominate Age" => Some(PredominateAge),
                "Minimum Criminal Sentence Duration" => {
                    Some(MinimumCriminalSentenceDuration)
                }
                "Age" => Some(Age),
                "Oil Sedimentation" => Some(OilSedimentation),
                "Days Supply" => Some(DaysSupply),
                "Product Sales Amount" => Some(ProductSalesAmount),
                "Effective Age" => Some(EffectiveAge),
                "Shares of Preferred Stock" => Some(SharesOfPreferredStock),
                "Standard" => Some(Standard),
                "Forecasted Scanned Quantity" => Some(ForecastedScannedQuantity),
                "Shares of Common Stock" => Some(SharesOfCommonStock),
                "Sample Amount" => Some(SampleAmount),
                "Maximum Criminal Sentence Duration" => {
                    Some(MaximumCriminalSentenceDuration)
                }
                "State or Province Motor Vehicle Penalty Points" => {
                    Some(StateOrProvinceMotorVehiclePenaltyPoints)
                }
                "Seasonal" => Some(Seasonal),
                "Time Units Known" => Some(TimeUnitsKnown),
                "Time Units Spent on Duty" => Some(TimeUnitsSpentOnDuty),
                "Total Days on Market" => Some(TotalDaysOnMarket),
                "Total Rooms" => Some(TotalRooms),
                "Total Number of Units" => Some(TotalNumberOfUnits),
                "Total Number of Units for Sale" => Some(TotalNumberOfUnitsForSale),
                "Tires" => Some(Tires),
                "Tank Allowance" => Some(TankAllowance),
                "Oil Theft" => Some(OilTheft),
                "Total at Complete" => Some(TotalAtComplete),
                "Total to Date" => Some(TotalToDate),
                "Number of Theatres" => Some(NumberOfTheatres),
                "Total Gas Injection Volume" => Some(TotalGasInjectionVolume),
                "Theoretical Quantity" => Some(TheoreticalQuantity),
                "Total Oil and/or Condensate Injection Volume" => {
                    Some(TotalOilAndOrCondensateInjectionVolume)
                }
                "Duration in Current Job" => Some(DurationInCurrentJob),
                "Total Oil and/or Condensate Disposition" => {
                    Some(TotalOilAndOrCondensateDisposition)
                }
                "Total Water Disposition" => Some(TotalWaterDisposition),
                "Total Beginning Inventory" => Some(TotalBeginningInventory),
                "Total" => Some(Total),
                "Time in Position" => Some(TimeInPosition),
                "Total Quantity of All Buys" => Some(TotalQuantityOfAllBuys),
                "Trips" => Some(Trips),
                "Total Number of Parking Spaces" => Some(TotalNumberOfParkingSpaces),
                "Total Production Volume" => Some(TotalProductionVolume),
                "Total Adjustments Volume" => Some(TotalAdjustmentsVolume),
                "Total Gas Disposition" => Some(TotalGasDisposition),
                "Total Water Injection Volume" => Some(TotalWaterInjectionVolume),
                "Total Ending Inventory" => Some(TotalEndingInventory),
                "Total Sales Volume" => Some(TotalSalesVolume),
                "Freelance Collectors" => Some(FreelanceCollectors),
                "Branch Locations Owned" => Some(BranchLocationsOwned),
                "Branch Locations Leased" => Some(BranchLocationsLeased),
                "Units Completed" => Some(UnitsCompleted),
                "Poultry" => Some(Poultry),
                "Livestock" => Some(Livestock),
                "Passengers" => Some(Passengers),
                "Trainers" => Some(Trainers),
                "Operators" => Some(Operators),
                "Gas Used on Property" => Some(GasUsedOnProperty),
                "Inspectors" => Some(Inspectors),
                "Collectors" => Some(Collectors),
                "Professionals" => Some(Professionals),
                "Supervisors" => Some(Supervisors),
                "Approximate Number of Units for Sale Projected" => {
                    Some(ApproximateNumberOfUnitsForSaleProjected)
                }
                "Administrators" => Some(Administrators),
                "Promoters" => Some(Promoters),
                "Oil Condensate Used on Property" => Some(OilCondensateUsedOnProperty),
                "Divisions" => Some(Divisions),
                "Tables" => Some(Tables),
                "Fuel Pumps" => Some(FuelPumps),
                "In-Use" => Some(InUse),
                "Machines" => Some(Machines),
                "Used" => Some(Used),
                "Trademarks Used" => Some(TrademarksUsed),
                "Available for Cultivation" => Some(AvailableForCultivation),
                "Foremen" => Some(Foremen),
                "Travelling Employees" => Some(TravellingEmployees),
                "Freelance Salespersons" => Some(FreelanceSalespersons),
                "Retention Quantity" => Some(RetentionQuantity),
                "Available Quantity" => Some(AvailableQuantity),
                "Transfer Quantity" => Some(TransferQuantity),
                "Surveys in Average Rating" => Some(SurveysInAverageRating),
                "Vacancies" => Some(Vacancies),
                "Unsubscribed Capacity" => Some(UnsubscribedCapacity),
                "Shipping Container Quantity" => Some(ShippingContainerQuantity),
                "Volume Shrinkage Adjustment or Allowance" => {
                    Some(VolumeShrinkageAdjustmentOrAllowance)
                }
                "Blank Votes" => Some(BlankVotes),
                "Cumulative Earned Value" => Some(CumulativeEarnedValue),
                "Scattered Votes" => Some(ScatteredVotes),
                "Earned Value" => Some(EarnedValue),
                "Federal Votes" => Some(FederalVotes),
                "Gas Vented" => Some(GasVented),
                "Schedule Variance" => Some(ScheduleVariance),
                "Cumulative Schedule Variance" => Some(CumulativeScheduleVariance),
                "Cumulative Variance" => Some(CumulativeVariance),
                "Estimate at Complete" => Some(EstimateAtComplete),
                "At Complete Variance" => Some(AtCompleteVariance),
                "Variance Adjustment" => Some(VarianceAdjustment),
                "No Votes" => Some(NoVotes),
                "Presidential Votes" => Some(PresidentialVotes),
                "Utilization Service Life" => Some(UtilizationServiceLife),
                "Variance" => Some(Variance),
                "Visits" => Some(Visits),
                "Votes" => Some(Votes),
                "Recommended Service Life" => Some(RecommendedServiceLife),
                "Void Votes" => Some(VoidVotes),
                "Shelf Life Period" => Some(ShelfLifePeriod),
                "Yes Votes" => Some(YesVotes),
                "Bankruptcy Petitions" => Some(BankruptcyPetitions),
                "Buyers" => Some(Buyers),
                "Debentures" => Some(Debentures),
                "Debentures Filed against Directors" => {
                    Some(DebenturesFiledAgainstDirectors)
                }
                "Detrimental Legal Filings against Directors" => {
                    Some(DetrimentalLegalFilingsAgainstDirectors)
                }
                "Failed Businesses of Directors" => Some(FailedBusinessesOfDirectors),
                "Professors" => Some(Professors),
                "Sellers" => Some(Sellers),
                "Skilled Workers" => Some(SkilledWorkers),
                "Trademarks Represented" => Some(TrademarksRepresented),
                "Total number of Workers' Compensation First Reports" => {
                    Some(TotalNumberOfWorkersCompensationFirstReports)
                }
                "Total number of Workers' Compensation Subsequent Reports" => {
                    Some(TotalNumberOfWorkersCompensationSubsequentReports)
                }
                "Total number of Workers' Compensation Combined Reports" => {
                    Some(TotalNumberOfWorkersCompensationCombinedReports)
                }
                "Units Worked per Day" => Some(UnitsWorkedPerDay),
                "Limited Quantity" => Some(LimitedQuantity),
                "Weight Gain" => Some(WeightGain),
                "Weight Loss" => Some(WeightLoss),
                "Operator's Working Interest" => Some(OperatorsWorkingInterest),
                "Number of Producing Wells Remaining on Property or Facility" => {
                    Some(NumberOfProducingWellsRemainingOnPropertyOrFacility)
                }
                "Number of Producing Wells Remaining on Royalty Account" => {
                    Some(NumberOfProducingWellsRemainingOnRoyaltyAccount)
                }
                "Total Working Interest" => Some(TotalWorkingInterest),
                "Water Volume" => Some(WaterVolume),
                "Weeks Worked" => Some(WeeksWorked),
                "License Withdrawal Duration" => Some(LicenseWithdrawalDuration),
                "License Withdrawals Sent" => Some(LicenseWithdrawalsSent),
                "Producing Wells" => Some(ProducingWells),
                "Gross" => Some(Gross),
                "Assessment Hours" => Some(AssessmentHours),
                "Duty Days" => Some(DutyDays),
                "Contract Days" => Some(ContractDays),
                "Number of Days Employed" => Some(NumberOfDaysEmployed),
                "Total of Issuable Assets" => Some(TotalOfIssuableAssets),
                "Total System Backorder Quantity, High Priority" => Some(CodeXB),
                "Total Service Backorder Quantity, High Priority" => Some(CodeXC),
                "Total System Backorder Quantity, Low Priority" => Some(CodeXD),
                "Total Service Backorder Quantity, Low Priority" => Some(CodeXE),
                "On Hand and Due-In" => Some(OnHandAndDueIn),
                "Installment Payments" => Some(InstallmentPayments),
                "Other War Reserve Material Requirements Protectable (OWRMRP) Quantity" => {
                    Some(CodeXJ)
                }
                "Approximate Number of Units Projected" => {
                    Some(ApproximateNumberOfUnitsProjected)
                }
                "Approximate Number of Holders" => Some(ApproximateNumberOfHolders),
                "Circulating Oil" => Some(CirculatingOil),
                "Stock Objective and Insurance Quantity" => {
                    Some(StockObjectiveAndInsuranceQuantity)
                }
                "Protected Quantity" => Some(ProtectedQuantity),
                "Reserved" => Some(Reserved),
                "Requisitioning Objective" => Some(RequisitioningObjective),
                "Authorized Retention Level" => Some(AuthorizedRetentionLevel),
                "Safety Level" => Some(SafetyLevel),
                "Backorder Lines" => Some(BackorderLines),
                "Number of Lost Cards" => Some(NumberOfLostCards),
                "Number of Stolen Cards" => Some(NumberOfStolenCards),
                "Number of Cards not Received" => Some(NumberOfCardsNotReceived),
                "Number of Active Accounts This Cycle" => {
                    Some(NumberOfActiveAccountsThisCycle)
                }
                "Number of Open Accounts" => Some(NumberOfOpenAccounts),
                "Number of Accounts Past Due" => Some(NumberOfAccountsPastDue),
                "Number of Cards Outstanding" => Some(NumberOfCardsOutstanding),
                "On Hand plus Pipeline" => Some(OnHandPlusPipeline),
                "Total Demand Quantity" => Some(TotalDemandQuantity),
                "Total Demand Orders" => Some(TotalDemandOrders),
                "First Quarter Recurring Demand" => Some(FirstQuarterRecurringDemand),
                "First Quarter Recurring Orders" => Some(FirstQuarterRecurringOrders),
                "First Quarter Non-recurring Demand" => {
                    Some(FirstQuarterNonRecurringDemand)
                }
                "First Quarter Non-recurring Orders" => {
                    Some(FirstQuarterNonRecurringOrders)
                }
                "Second Quarter Recurring Demand" => Some(SecondQuarterRecurringDemand),
                "Second Quarter Recurring Orders" => Some(SecondQuarterRecurringOrders),
                "Second Quarter Non-recurring Demand" => {
                    Some(SecondQuarterNonRecurringDemand)
                }
                "Second Quarter Non-recurring Orders" => {
                    Some(SecondQuarterNonRecurringOrders)
                }
                "Third Quarter Recurring Demand" => Some(ThirdQuarterRecurringDemand),
                "Third Quarter Recurring Orders" => Some(ThirdQuarterRecurringOrders),
                "Third Quarter Non-recurring Demand" => {
                    Some(ThirdQuarterNonRecurringDemand)
                }
                "Third Quarter Non-recurring Orders" => {
                    Some(ThirdQuarterNonRecurringOrders)
                }
                "Fourth Quarter Recurring Demand" => Some(FourthQuarterRecurringDemand),
                "Fourth Quarter Recurring Orders" => Some(FourthQuarterRecurringOrders),
                "Fourth Quarter Non-recurring Demand" => {
                    Some(FourthQuarterNonRecurringDemand)
                }
                "Fourth Quarter Non-recurring Orders" => {
                    Some(FourthQuarterNonRecurringOrders)
                }
                "Trailers" => Some(Trailers),
                "Reorder Point Quantity" => Some(ReorderPointQuantity),
                "Contract Line Item Quantity" => Some(ContractLineItemQuantity),
                "Years" => Some(Years),
                "Maximum Quantity of Free Service Calls" => {
                    Some(MaximumQuantityOfFreeServiceCalls)
                }
                "Units Worked Last Day" => Some(UnitsWorkedLastDay),
                "Units Worked per Week" => Some(UnitsWorkedPerWeek),
                "Units Worked per Quarter" => Some(UnitsWorkedPerQuarter),
                "Number Weeks Paid" => Some(NumberWeeksPaid),
                "Unused Accumulated Sick Days" => Some(UnusedAccumulatedSickDays),
                "Delivery Point Reduction Quantity" => {
                    Some(DeliveryPointReductionQuantity)
                }
                "Receipt Point Reduction Quantity" => Some(ReceiptPointReductionQuantity),
                "Reduction Quantity" => Some(ReductionQuantity),
                "Federal Medicare or Medicaid Claim Mandate - Category 1" => {
                    Some(FederalMedicareOrMedicaidClaimMandateCategory1)
                }
                "Federal Medicare or Medicaid Claim Mandate - Category 2" => {
                    Some(FederalMedicareOrMedicaidClaimMandateCategory2)
                }
                "Federal Medicare or Medicaid Claim Mandate - Category 3" => {
                    Some(FederalMedicareOrMedicaidClaimMandateCategory3)
                }
                "Federal Medicare or Medicaid Claim Mandate - Category 4" => {
                    Some(FederalMedicareOrMedicaidClaimMandateCategory4)
                }
                "Federal Medicare or Medicaid Claim Mandate - Category 5" => {
                    Some(FederalMedicareOrMedicaidClaimMandateCategory5)
                }
                "Federal Pension Mandate - Category 1" => {
                    Some(FederalPensionMandateCategory1)
                }
                "Federal Pension Mandate - Category 2" => {
                    Some(FederalPensionMandateCategory2)
                }
                "Federal Pension Mandate - Category 3" => {
                    Some(FederalPensionMandateCategory3)
                }
                "Holding Period" => Some(HoldingPeriod),
                "Federal Pension Mandate - Category 5" => {
                    Some(FederalPensionMandateCategory5)
                }
                "Federal Medicare or Medicaid Payment Mandate - Category 1" => {
                    Some(FederalMedicareOrMedicaidPaymentMandateCategory1)
                }
                "Federal Medicare or Medicaid Payment Mandate - Category 2" => {
                    Some(FederalMedicareOrMedicaidPaymentMandateCategory2)
                }
                "Federal Medicare or Medicaid Payment Mandate - Category 3" => {
                    Some(FederalMedicareOrMedicaidPaymentMandateCategory3)
                }
                "Federal Medicare or Medicaid Payment Mandate - Category 4" => {
                    Some(FederalMedicareOrMedicaidPaymentMandateCategory4)
                }
                "Federal Medicare or Medicaid Payment Mandate - Category 5" => {
                    Some(FederalMedicareOrMedicaidPaymentMandateCategory5)
                }
                "Federal Pension Mandate - Category 4" => {
                    Some(FederalPensionMandateCategory4)
                }
                "Shares Added" => Some(SharesAdded),
                "Extended Term" => Some(ExtendedTerm),
                "Amortization Term" => Some(AmortizationTerm),
                "Beginning Shares" => Some(BeginningShares),
                "Shares Deleted" => Some(SharesDeleted),
                "Quantity of Dealer License Plates" => {
                    Some(QuantityOfDealerLicensePlates)
                }
                "Current Share Balance" => Some(CurrentShareBalance),
                "Size of Household" => Some(SizeOfHousehold),
                "Project Units Sold" => Some(ProjectUnitsSold),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for QuantityQualifier {
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
    type Value = QuantityQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Quantity Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        QuantityQualifier::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Quantity Qualifier: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        QuantityQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Quantity Qualifier: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for QuantityQualifier {
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