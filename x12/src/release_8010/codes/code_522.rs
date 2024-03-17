use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**522

See docs at <https://www.stedi.com/edi/x12/element/522>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AmountQualifierCode {
    ///00 - Downpayment on the Repayment Plan Amount
    DownpaymentOnTheRepaymentPlanAmount,
    ///0A - Interest Substitution Adjustment
    InterestSubstitutionAdjustment,
    ///0B - Principal Substitution Adjustment
    PrincipalSubstitutionAdjustment,
    ///0C - Prepaid Interest
    PrepaidInterest,
    ///0D - Prepaid Principal
    PrepaidPrincipal,
    ///0E - Delinquent Interest
    DelinquentInterest,
    ///0F - Delinquent Principal
    DelinquentPrincipal,
    ///0G - Curtailment Adjustment
    CurtailmentAdjustment,
    ///0H - Serial Note Principal Available for Distribution
    SerialNotePrincipalAvailableForDistribution,
    ///0I - Servicing Fee
    ServicingFee,
    ///0J - Guarantee Fee Adjustment
    GuaranteeFeeAdjustment,
    ///0K - Amount Under-collateralized
    AmountUnderCollateralized,
    ///0L - Amount Over-collateralized
    AmountOverCollateralized,
    ///0M - Trial Balance Adjustment
    TrialBalanceAdjustment,
    ///0N - Custodial Bank Account Adjustment
    CustodialBankAccountAdjustment,
    ///0P - Item
    Item,
    ///0Q - Schedule
    Schedule,
    ///0R - Regular Plan
    RegularPlan,
    ///0S - Previously Billed
    PreviouslyBilled,
    ///0T - Currently Due
    CurrentlyDue,
    ///0U - Coverage Premium
    CoveragePremium,
    ///1 - Line Item Total
    LineItemTotal,
    ///01 - Fixed Installment Control Accounting Error
    FixedInstallmentControlAccountingError,
    ///1A - Variance
    Variance,
    ///1B - Variance Adjustment Cost
    VarianceAdjustmentCost,
    ///1C - Variance Adjustment Schedule
    VarianceAdjustmentSchedule,
    ///1D - Escalation
    Escalation,
    ///1E - Fixed Price
    FixedPrice,
    ///1F - Lodging
    Lodging,
    ///1G - Meals
    Meals,
    ///1H - Travel Expense
    TravelExpense,
    ///1J - Insurance Expense
    InsuranceExpense,
    ///1K - Union Dues
    UnionDues,
    ///1L - Regular Income
    RegularIncome,
    ///1M - Income on Real Property
    IncomeOnRealProperty,
    ///1N - Income from Social Security and/or other Government Assistance
    IncomeFromSocialSecurityAndOrOtherGovernmentAssistance,
    ///1P - Total Monthly Income
    TotalMonthlyIncome,
    ///1Q - Electric and/or Fuel Payment
    ElectricAndOrFuelPayment,
    ///1R - Water and/or Sewer Payment
    WaterAndOrSewerPayment,
    ///1S - Telephone Payment
    TelephonePayment,
    ///1T - Maintenance Expense
    MaintenanceExpense,
    ///1U - Food Expense
    FoodExpense,
    ///1V - Clothing Expense
    ClothingExpense,
    ///1W - Laundry Expense
    LaundryExpense,
    ///1X - Medical and/or Dental Expense
    MedicalAndOrDentalExpense,
    ///1Y - Recreation Expenses
    RecreationExpenses,
    ///1Z - Charitable Contributions
    CharitableContributions,
    ///2 - Batch Total
    BatchTotal,
    ///02 - Graduated Payment Mortgage Adjustment
    GraduatedPaymentMortgageAdjustment,
    ///2A - Home Insurance Expense
    HomeInsuranceExpense,
    ///2B - Life Insurance Expense
    LifeInsuranceExpense,
    ///2C - Health Insurance Expense
    HealthInsuranceExpense,
    ///2D - Automobile Insurance Expense
    AutomobileInsuranceExpense,
    ///2E - Value of Property Claimed as Exempt
    ValueOfPropertyClaimedAsExempt,
    ///2F - Automobile Payment
    AutomobilePayment,
    ///2G - Other Type of Installment Payment
    OtherTypeOfInstallmentPayment,
    ///2H - Operating Expenses
    OperatingExpenses,
    ///2I - Total Projected Monthly Income
    TotalProjectedMonthlyIncome,
    ///2J - Total Projected Monthly Expenses
    TotalProjectedMonthlyExpenses,
    ///2K - Excess Income
    ExcessIncome,
    ///2L - Value of Personal Property
    ValueOfPersonalProperty,
    ///2M - Monthly Overtime
    MonthlyOvertime,
    ///2N - Total All Repairs
    TotalAllRepairs,
    ///2P - Total Recommended Repairs
    TotalRecommendedRepairs,
    ///2Q - State Quarterly Total Gross Wages
    StateQuarterlyTotalGrossWages,
    ///2R - State Quarterly Unemployment Insurance (UI) Total Wages
    Code2R,
    ///2S - State Quarterly Unemployment Insurance (UI) Excess Wages
    Code2S,
    ///2T - State Quarterly Unemployment Insurance (UI) Taxable Wages
    Code2T,
    ///2U - State Quarterly Disability Insurance Taxable Wages
    StateQuarterlyDisabilityInsuranceTaxableWages,
    ///2V - State Quarterly Tip Wages
    StateQuarterlyTipWages,
    ///2W - Asset-Long Term
    AssetLongTerm,
    ///2X - Asset-Short Term
    AssetShortTerm,
    ///2Y - Base Coverage
    BaseCoverage,
    ///2Z - Commission Retained
    CommissionRetained,
    ///3 - Deposit Total
    DepositTotal,
    ///03 - Growing Equity Mortgage Adjustment
    GrowingEquityMortgageAdjustment,
    ///3A - Accounting
    Accounting,
    ///3B - Accounts Payable
    AccountsPayable,
    ///3C - Accounts Receivable
    AccountsReceivable,
    ///3D - Advanced Dividends
    AdvancedDividends,
    ///3E - Advertising Expenses
    AdvertisingExpenses,
    ///3F - Amortization
    Amortization,
    ///3G - Amortization Costs
    AmortizationCosts,
    ///3H - Amount of Decree
    AmountOfDecree,
    ///3I - Asset Investment
    AssetInvestment,
    ///3J - Authorized Capital
    AuthorizedCapital,
    ///3K - Available Reserves
    AvailableReserves,
    ///3L - Bad Debt Allowance
    BadDebtAllowance,
    ///3M - Bad Debts
    BadDebts,
    ///3N - Bank Account(s)
    Code3N,
    ///3O - Long Term Assets
    LongTermAssets,
    ///3P - Long Term Liabilities
    LongTermLiabilities,
    ///3Q - Long Term Tangible Assets
    LongTermTangibleAssets,
    ///3R - Losses on Capital
    LossesOnCapital,
    ///3S - Machines and Tools
    MachinesAndTools,
    ///3T - Member Risk Capital
    MemberRiskCapital,
    ///3U - Miscellaneous After Tax Exempt
    MiscellaneousAfterTaxExempt,
    ///3V - Mortgage
    Mortgage,
    ///3W - Nominal Capital
    NominalCapital,
    ///3X - Nominal Damages
    NominalDamages,
    ///3Y - Non-operational Fixed Assets
    NonOperationalFixedAssets,
    ///3Z - Excess Amount Requested
    ExcessAmountRequested,
    ///4 - Lock Box Total
    LockBoxTotal,
    ///04 - Adjustable Rate Mortgage Change
    AdjustableRateMortgageChange,
    ///4A - Nonissued Capital
    NonissuedCapital,
    ///4B - Notes Payable
    NotesPayable,
    ///4C - Notes Receivable
    NotesReceivable,
    ///4D - Bank Debentures
    BankDebentures,
    ///4E - Bank Obligations
    BankObligations,
    ///4F - Buildings
    Buildings,
    ///4G - Buildings Under Construction
    BuildingsUnderConstruction,
    ///4H - Capital
    Capital,
    ///4I - Capital Associated with Principal
    CapitalAssociatedWithPrincipal,
    ///4J - Capital of Other Subsidiaries
    CapitalOfOtherSubsidiaries,
    ///4K - Capital Stock
    CapitalStock,
    ///4L - Cash
    Cash,
    ///4M - Capital Subsidies Received
    CapitalSubsidiesReceived,
    ///4N - Commercial Debt
    CommercialDebt,
    ///4O - Commercial Expenses
    CommercialExpenses,
    ///4P - Common Stock
    CommonStock,
    ///4Q - Consequential Damages
    ConsequentialDamages,
    ///4R - Compensatory Damages
    CompensatoryDamages,
    ///4S - Convertible Debentures
    ConvertibleDebentures,
    ///4T - Cost of Goods Sold
    CostOfGoodsSold,
    ///4U - Cost of Sales
    CostOfSales,
    ///4V - Cost(s)
    Code4V,
    ///4W - Current Assets
    CurrentAssets,
    ///4X - Current Liabilities
    CurrentLiabilities,
    ///4Y - Damages
    Damages,
    ///4Z - Deferred Cost
    DeferredCost,
    ///5 - Total Invoice Amount
    TotalInvoiceAmount,
    ///05 - Fixed Installment Control Substitution Adjustment
    FixedInstallmentControlSubstitutionAdjustment,
    ///5A - Deferred Credit or Income
    DeferredCreditOrIncome,
    ///5B - Deferred Taxation
    DeferredTaxation,
    ///5C - Deposits
    Deposits,
    ///5D - Depreciation
    Depreciation,
    ///5E - Depreciation of Fixed Assets
    DepreciationOfFixedAssets,
    ///5F - Depreciation of Revaluation of Fixed Assets
    DepreciationOfRevaluationOfFixedAssets,
    ///5G - Director's Remuneration
    DirectorsRemuneration,
    ///5H - Dividends
    Dividends,
    ///5I - Doubtful Receivables
    DoubtfulReceivables,
    ///5J - Equipment
    Equipment,
    ///5K - Equipment Subsidies
    EquipmentSubsidies,
    ///5L - Equities, Stocks
    Code5L,
    ///5M - Equity
    Equity,
    ///5N - Exceptional Item
    ExceptionalItem,
    ///5O - Exports
    Exports,
    ///5P - External Charge
    ExternalCharge,
    ///5Q - Extraordinary Charge
    ExtraordinaryCharge,
    ///5R - Extraordinary Current Asset Write Downs
    ExtraordinaryCurrentAssetWriteDowns,
    ///5S - Extraordinary Result
    ExtraordinaryResult,
    ///5T - Financial Assets
    FinancialAssets,
    ///5U - Financial Charges
    FinancialCharges,
    ///5V - Financial Debt
    FinancialDebt,
    ///5W - Financial Expenses
    FinancialExpenses,
    ///5X - Financial Income
    FinancialIncome,
    ///5Y - Finished Goods
    FinishedGoods,
    ///5Z - Fixed Asset Debts
    FixedAssetDebts,
    ///6 - Amount Subject to Total Monetary Discount
    AmountSubjectToTotalMonetaryDiscount,
    ///06 - Interest Adjustment
    InterestAdjustment,
    ///6A - Fixed Assets
    FixedAssets,
    ///6B - Fixed Assets for Sale
    FixedAssetsForSale,
    ///6C - Fixtures
    Fixtures,
    ///6D - Fixtures and Equipment
    FixturesAndEquipment,
    ///6E - Franchise
    Franchise,
    ///6F - Franchise Tax Balance
    FranchiseTaxBalance,
    ///6G - Franchise Tax Paid
    FranchiseTaxPaid,
    ///6H - Free Reserves
    FreeReserves,
    ///6I - Furniture
    Furniture,
    ///6J - Future Loan
    FutureLoan,
    ///6K - General Accounts
    GeneralAccounts,
    ///6L - General Expenses
    GeneralExpenses,
    ///6M - Goodwill
    Goodwill,
    ///6N - Grants for Operating Costs
    GrantsForOperatingCosts,
    ///6O - Group Related Financial Income
    GroupRelatedFinancialIncome,
    ///6P - Income Stated in Advance
    IncomeStatedInAdvance,
    ///6Q - Income Tax
    IncomeTax,
    ///6R - Income Tax, Corporate
    Code6R,
    ///6S - Income Tax, Noncorporate
    Code6S,
    ///6T - Injunction
    Injunction,
    ///6U - Intangible Depreciation
    IntangibleDepreciation,
    ///6V - Intangibles
    Intangibles,
    ///6W - Interest of Third Party
    InterestOfThirdParty,
    ///6X - Interest on Loans
    InterestOnLoans,
    ///6Y - Operating Income (Loss)
    Code6Y,
    ///6Z - Optional Reserves
    OptionalReserves,
    ///7 - Discount Amount Due
    DiscountAmountDue,
    ///07 - Deferred Graduated Payment Mortgage Interest Paid
    DeferredGraduatedPaymentMortgageInterestPaid,
    ///7A - Organizational Expenses
    OrganizationalExpenses,
    ///7B - Outside Share in Profit or Loss
    OutsideShareInProfitOrLoss,
    ///7C - Outstanding Debts against Board of Directors/Managers
    OutstandingDebtsAgainstBoardOfDirectorsManagers,
    ///7D - Owing
    Owing,
    ///7E - Owing from Affiliates
    OwingFromAffiliates,
    ///7F - Owing from Participants
    OwingFromParticipants,
    ///7G - Owing to Affiliates
    OwingToAffiliates,
    ///7H - Owing to Fiscal Office
    OwingToFiscalOffice,
    ///7I - Owing to National Social Security Office
    OwingToNationalSocialSecurityOffice,
    ///7J - Owing to Participants
    OwingToParticipants,
    ///7K - Own Work Capitalized
    OwnWorkCapitalized,
    ///7L - Paid in Capital
    PaidInCapital,
    ///7M - Par Value
    ParValue,
    ///7N - Participating Interest
    ParticipatingInterest,
    ///7O - Patents
    Patents,
    ///7P - Pension Debts
    PensionDebts,
    ///7Q - Pensions Provision
    PensionsProvision,
    ///7R - Preferred Stock
    PreferredStock,
    ///7S - Prepaid Orders in Progress
    PrepaidOrdersInProgress,
    ///7T - Prior Results Carried Forward
    PriorResultsCarriedForward,
    ///7U - Profit or Loss
    ProfitOrLoss,
    ///7V - Profit or Loss after Taxes
    ProfitOrLossAfterTaxes,
    ///7W - Profit or Loss before Taxes
    ProfitOrLossBeforeTaxes,
    ///7X - Profit or Loss on Ordinary Activities after Tax
    ProfitOrLossOnOrdinaryActivitiesAfterTax,
    ///7Y - Progress Payments
    ProgressPayments,
    ///7Z - Proposed Dividend
    ProposedDividend,
    ///8 - Total Monetary Discount Amount
    TotalMonetaryDiscountAmount,
    ///08 - Interest Accounting Error
    InterestAccountingError,
    ///8A - Provision for Depreciation of Stock or Inventory
    ProvisionForDepreciationOfStockOrInventory,
    ///8B - Provision for Future Purchases
    ProvisionForFuturePurchases,
    ///8C - Provision for Risks
    ProvisionForRisks,
    ///8D - Punitive Damages
    PunitiveDamages,
    ///8E - Purchase Price
    PurchasePrice,
    ///8F - Purchases
    Purchases,
    ///8G - Raw Materials
    RawMaterials,
    ///8H - Real Estate
    RealEstate,
    ///8I - Receivables
    Receivables,
    ///8J - Regularization Account
    RegularizationAccount,
    ///8K - Research and Development
    ResearchAndDevelopment,
    ///8L - Restructuring Costs
    RestructuringCosts,
    ///8M - Result
    Result,
    ///8N - Retained Earnings
    RetainedEarnings,
    ///8O - Revenues
    Revenues,
    ///8P - Sales
    Sales,
    ///8Q - Sales and Use Tax
    SalesAndUseTax,
    ///8R - Savings
    Savings,
    ///8S - Secured Liability
    SecuredLiability,
    ///8T - Secured Loans
    SecuredLoans,
    ///8U - Selling Expenses
    SellingExpenses,
    ///8V - Services
    Services,
    ///8W - Share Capital
    ShareCapital,
    ///8X - Share in Profit or Loss of Minority Interest
    ShareInProfitOrLossOfMinorityInterest,
    ///8Y - Share Premium Capital
    SharePremiumCapital,
    ///8Z - Shares in Affiliated Companies
    SharesInAffiliatedCompanies,
    ///9 - Total Operational Statement Amount
    TotalOperationalStatementAmount,
    ///09 - Principal Accounting Error
    PrincipalAccountingError,
    ///9A - Social Charges
    SocialCharges,
    ///9B - Social Security (FICA)
    Code9B,
    ///9C - Special Reserves
    SpecialReserves,
    ///9D - Specially Secured Creditors
    SpeciallySecuredCreditors,
    ///9E - Specific Performance
    SpecificPerformance,
    ///9F - Starting Capital
    StartingCapital,
    ///9G - Statutory Reserves
    StatutoryReserves,
    ///9H - Subscribed Capital
    SubscribedCapital,
    ///9I - Suit Amount
    SuitAmount,
    ///9J - Supplies
    Supplies,
    ///9K - Surplus of Revaluation
    SurplusOfRevaluation,
    ///9L - Tangible Net Worth
    TangibleNetWorth,
    ///9M - Tax Adjustments
    TaxAdjustments,
    ///9N - Tax Balance
    TaxBalance,
    ///9O - Tax Capital Amount
    TaxCapitalAmount,
    ///9P - Tax on Extraordinary Items
    TaxOnExtraordinaryItems,
    ///9Q - Tax Recoverable
    TaxRecoverable,
    ///9R - Taxed Reserves
    TaxedReserves,
    ///9S - Trade Creditors
    TradeCreditors,
    ///9T - Inventory (Stock)
    Code9T,
    ///9U - Inventory (Stock) Depreciation
    Code9U,
    ///9V - Inventory (Stock) Purchases
    Code9V,
    ///9W - Investment in Own Shares
    InvestmentInOwnShares,
    ///9X - Investments
    Investments,
    ///9Y - Issued Capital
    IssuedCapital,
    ///9Z - Labor Costs
    LaborCosts,
    ///10 - Shipment Value in U.S. Dollars
    ShipmentValueInUSDollars,
    ///11 - Liabilities at Bankruptcy
    LiabilitiesAtBankruptcy,
    ///12 - Account Average Balance Account
    AccountAverageBalanceAccount,
    ///13 - Outstanding Balance at Foreclosure
    OutstandingBalanceAtForeclosure,
    ///14 - Legal Obligation Debt Amount
    LegalObligationDebtAmount,
    ///15 - Estimated Closing Cost Amount
    EstimatedClosingCostAmount,
    ///16 - Discount Fees Paid by Borrower Amount
    DiscountFeesPaidByBorrowerAmount,
    ///17 - Closing Costs or Concessions Paid by Seller
    ClosingCostsOrConcessionsPaidBySeller,
    ///18 - Prepaid Items Amount
    PrepaidItemsAmount,
    ///19 - Federal Housing Administration, Mortgage Insurance Premium Funding Fee Financed Amount
    Code19,
    ///20 - Federal Housing Administration, Mortgage Insurance Premium or Veteran's Administration Funding Fee Amount
    Code20,
    ///21 - Original Cost of Property Amount
    OriginalCostOfPropertyAmount,
    ///22 - Owner's Estimate of Value Amount
    OwnersEstimateOfValueAmount,
    ///23 - Appraised Value Amount
    AppraisedValueAmount,
    ///24 - Gross Monthly Income Amount
    GrossMonthlyIncomeAmount,
    ///25 - Assets at Bankruptcy
    AssetsAtBankruptcy,
    ///26 - Negotiated Cost
    NegotiatedCost,
    ///27 - Authorized Unpriced Work
    AuthorizedUnpricedWork,
    ///28 - Target Price
    TargetPrice,
    ///29 - Estimated Price
    EstimatedPrice,
    ///30 - Contract Ceiling
    ContractCeiling,
    ///31 - Estimated Contract Ceiling
    EstimatedContractCeiling,
    ///32 - Target Fee or Profit Amount
    TargetFeeOrProfitAmount,
    ///33 - Original Contract Target Cost
    OriginalContractTargetCost,
    ///34 - Negotiated Contract Changes
    NegotiatedContractChanges,
    ///35 - Current Target Cost
    CurrentTargetCost,
    ///36 - Contract Budget Base (CBB)
    Code36,
    ///37 - Current Budgeted Cost for Work Scheduled (BCWS)
    Code37,
    ///38 - Current Budgeted Cost for Work Performed (BCWP)
    Code38,
    ///39 - Current Actual Cost of Work Performed (ACWP)
    Code39,
    ///40 - Current Schedule Variance (SV)
    Code40,
    ///41 - Current Cost Variance (CV)
    Code41,
    ///42 - Cumulative Budgeted Cost for Work Scheduled (BCWS)
    Code42,
    ///43 - Cumulative Budgeted Cost for Work Performed (BCWP)
    Code43,
    ///44 - Cumulative Actual Cost of Work Performed (ACWP)
    Code44,
    ///45 - Cumulative Schedule Variance (SV)
    Code45,
    ///46 - Cumulative Cost Variance (CV)
    Code46,
    ///47 - Reprogram Cost Variance
    ReprogramCostVariance,
    ///48 - Reprogram Budget
    ReprogramBudget,
    ///49 - At Complete Budget (BAC)
    Code49,
    ///50 - At Complete Latest Revised Estimate (LRE)
    Code50,
    ///51 - At Complete Variance
    AtCompleteVariance,
    ///52 - Total Allocated Budget
    TotalAllocatedBudget,
    ///53 - Difference (Contract Budget Base - Total Allocated Budget)
    Code53,
    ///54 - Forecast
    Forecast,
    ///55 - At Complete Forecast
    AtCompleteForecast,
    ///56 - Current Cost Performance Index (CPIe) - Efficiency (BCWP/ACWP)
    Code56,
    ///57 - Current Cost Performance Index (CPIp) - Planned (ACWP/BCWP)
    Code57,
    ///58 - Current Schedule Performance Index (SPI)
    Code58,
    ///59 - Cumulative Cost Performance Index (CPIe) - Efficiency (BCWP/ACWP)
    Code59,
    ///60 - Cumulative Cost Performance Index (CPIp) - Planned (ACWP/BCWP)
    Code60,
    ///61 - Cumulative Schedule Performance Index (SPI)
    Code61,
    ///62 - To Complete Performance Index (TCPI) for Budget at Complete (BAC)
    Code62,
    ///63 - To Complete Performance Index (TCPI) for Estimate At Complete (EAC)
    Code63,
    ///64 - Initial Contract Price Target
    InitialContractPriceTarget,
    ///65 - Initial Contract Price Ceiling
    InitialContractPriceCeiling,
    ///66 - Adjusted Contract Price Target
    AdjustedContractPriceTarget,
    ///67 - Adjusted Contract Price Ceiling
    AdjustedContractPriceCeiling,
    ///68 - Funds Authorized to Date
    FundsAuthorizedToDate,
    ///69 - Accrued Expenditures
    AccruedExpenditures,
    ///70 - Open Commitments
    OpenCommitments,
    ///71 - Forecast of Billings
    ForecastOfBillings,
    ///72 - Estimated Termination Costs
    EstimatedTerminationCosts,
    ///73 - Accrued Expenditures plus Open Commitments
    AccruedExpendituresPlusOpenCommitments,
    ///74 - Contract Work Authorized - Definitized
    ContractWorkAuthorizedDefinitized,
    ///75 - Contract Work Authorized - Not Definitized
    ContractWorkAuthorizedNotDefinitized,
    ///76 - Contract Work Authorized - Total
    ContractWorkAuthorizedTotal,
    ///77 - Forecast of Work - Not Yet Authorized
    ForecastOfWorkNotYetAuthorized,
    ///78 - Forecast of Work - All Other
    ForecastOfWorkAllOther,
    ///79 - Forecast of Work - Total
    ForecastOfWorkTotal,
    ///80 - Funding - Total Requirements
    FundingTotalRequirements,
    ///81 - Funds Carryover
    FundsCarryover,
    ///82 - Net Funds Required
    NetFundsRequired,
    ///83 - Contract Work Authorized (with fee/profit) Actual or Projected
    Code83,
    ///84 - Contract Work Authorized (with fee/profit) Actual or Projected - At Complete
    Code84,
    ///85 - Best Case Estimate
    BestCaseEstimate,
    ///86 - Worst Case Estimate
    WorstCaseEstimate,
    ///87 - Most Likely Estimate
    MostLikelyEstimate,
    ///88 - "As Is" Appraisal Amount
    Code88,
    ///89 - "Subject To" Appraisal Amount
    Code89,
    ///90 - "Completion Per Plans" Appraisal Amount
    Code90,
    ///91 - Site Value Amount
    SiteValueAmount,
    ///92 - Compensation
    Compensation,
    ///93 - Contribution
    Contribution,
    ///94 - Death Benefit
    DeathBenefit,
    ///95 - Death Benefit Decrement
    DeathBenefitDecrement,
    ///96 - Employee Account Balance
    EmployeeAccountBalance,
    ///97 - Loan Repayment
    LoanRepayment,
    ///98 - Prior W2
    PriorW2,
    ///99 - Single Premium
    SinglePremium,
    ///A - Adjusted Chargeback Claim Amount
    AdjustedChargebackClaimAmount,
    ///A0 - Assistantship from Admitting Educational Institution
    AssistantshipFromAdmittingEducationalInstitution,
    ///A1 - Average Negative Ledger Balance
    AverageNegativeLedgerBalance,
    ///A2 - Average Positive Collected Balance
    AveragePositiveCollectedBalance,
    ///A3 - Average Negative Collected Balance
    AverageNegativeCollectedBalance,
    ///A4 - Average Positive Ledger Balance
    AveragePositiveLedgerBalance,
    ///A5 - Disallowed - Estimated
    DisallowedEstimated,
    ///A6 - Disallowed - Actual
    DisallowedActual,
    ///A7 - Noncovered Charges - Estimated
    NoncoveredChargesEstimated,
    ///A8 - Noncovered Charges - Actual
    NoncoveredChargesActual,
    ///A9 - Allowed - Estimated
    AllowedEstimated,
    ///AA - Allocated
    Allocated,
    ///AA1 - Excess Funds
    ExcessFunds,
    ///AA2 - Cumulative Total
    CumulativeTotal,
    ///AA3 - Reimbursable Amount
    ReimbursableAmount,
    ///AA4 - Total Reimbursable Amount
    TotalReimbursableAmount,
    ///AA5 - Direct Citation Amount
    DirectCitationAmount,
    ///AA6 - Total Direct Citation Funds
    TotalDirectCitationFunds,
    ///AA7 - Chargeable Amount
    ChargeableAmount,
    ///AAA - Temporary Term Coverage
    TemporaryTermCoverage,
    ///AAB - Conditional Receipt Coverage
    ConditionalReceiptCoverage,
    ///AAC - Binding Interim Coverage
    BindingInterimCoverage,
    ///AAD - Application Amount
    ApplicationAmount,
    ///AAE - Approved Amount
    ApprovedAmount,
    ///AAF - Ultimate Face Amount
    UltimateFaceAmount,
    ///AAG - Requested Amount from All Reinsurers
    RequestedAmountFromAllReinsurers,
    ///AAH - Replacement Amount
    ReplacementAmount,
    ///AAI - Scheduled Contribution
    ScheduledContribution,
    ///AAJ - Scheduled Disbursement
    ScheduledDisbursement,
    ///AAK - Short Term Investment
    ShortTermInvestment,
    ///AAL - Subsequent Contribution
    SubsequentContribution,
    ///AAM - Subsequent Distribution
    SubsequentDistribution,
    ///AAN - Tax-Federal
    TaxFederal,
    ///AAO - Tax-Local
    TaxLocal,
    ///AAP - Tax-State
    TaxState,
    ///AAQ - Trust Fund
    TrustFund,
    ///AAR - Capital Leases
    CapitalLeases,
    ///AAS - Surplus
    Surplus,
    ///AAT - Restated Assets
    RestatedAssets,
    ///AAU - Owing to Clients
    OwingToClients,
    ///AAV - Shareholder Loans
    ShareholderLoans,
    ///AAW - Accumulated Deficit
    AccumulatedDeficit,
    ///AAX - Loan from Parent Company
    LoanFromParentCompany,
    ///AAY - Contribution Not Subject to Repayment
    ContributionNotSubjectToRepayment,
    ///AAZ - Income Before Depreciation
    IncomeBeforeDepreciation,
    ///AB - Adjusted Collected Balance
    AdjustedCollectedBalance,
    ///ABA - Income After Depreciation
    IncomeAfterDepreciation,
    ///ABB - Profit (Loss) Before Financial Items
    CodeABB,
    ///ABC - Interest Expenses
    InterestExpenses,
    ///ABD - Profit (Loss) Before Extraordinary Items
    CodeABD,
    ///ABE - Profit (Loss) After Financial Items
    CodeABE,
    ///ABF - Income Before Allocations
    IncomeBeforeAllocations,
    ///ABG - Income from Sale of Fixed Assets
    IncomeFromSaleOfFixedAssets,
    ///ABH - Contribution to Group
    ContributionToGroup,
    ///ABI - Deferred Tax Assets
    DeferredTaxAssets,
    ///ABJ - Blocked Accounts
    BlockedAccounts,
    ///ABK - Non-taxed Reserves
    NonTaxedReserves,
    ///ABL - Pledged Assets
    PledgedAssets,
    ///ABM - Restricted Equity
    RestrictedEquity,
    ///ABN - Non-restricted Equity
    NonRestrictedEquity,
    ///ABO - Depreciable Assets
    DepreciableAssets,
    ///ABP - Taxable Assets
    TaxableAssets,
    ///ABQ - Income from Business
    IncomeFromBusiness,
    ///ABR - Income Subject to Taxes
    IncomeSubjectToTaxes,
    ///ABS - Taxable Amount of Real Estate
    TaxableAmountOfRealEstate,
    ///ABT - Ending Principal Balance
    EndingPrincipalBalance,
    ///ABU - Average Daily Principal Balance
    AverageDailyPrincipalBalance,
    ///ABV - Interest Amount
    InterestAmount,
    ///ABW - Adjustments for Difference in Average Daily Principal Balance
    AdjustmentsForDifferenceInAverageDailyPrincipalBalance,
    ///ABX - Beginning Principal Balance
    BeginningPrincipalBalance,
    ///ABY - Loan Principal Disbursements
    LoanPrincipalDisbursements,
    ///ABZ - Principal Increases
    PrincipalIncreases,
    ///AC - Average Collected Balance
    AverageCollectedBalance,
    ///ACA - Principal of Loans Purchased
    PrincipalOfLoansPurchased,
    ///ACB - Principal Cured
    PrincipalCured,
    ///ACC - Principal Sold
    PrincipalSold,
    ///ACD - Principal Insurance Claims
    PrincipalInsuranceClaims,
    ///ACE - Principal Guarantee Voided
    PrincipalGuaranteeVoided,
    ///ACF - Principal Paid by Borrowers
    PrincipalPaidByBorrowers,
    ///ACG - Loans in School and Grace
    LoansInSchoolAndGrace,
    ///ACH - Loans in Authorized Deferment
    LoansInAuthorizedDeferment,
    ///ACI - Loans Repay or Forebearance - Current or Less than 31 Days
    LoansRepayOrForebearanceCurrentOrLessThan31Days,
    ///ACJ - Loans Repay or Forebearance - 31 to 60 Days Past Due
    LoansRepayOrForebearance31To60DaysPastDue,
    ///ACK - Loans Repay or Forebearance - 61 to 90 Days Past Due
    LoansRepayOrForebearance61To90DaysPastDue,
    ///ACL - Loans Repay or Forebearance - 91 to 120 Days Past Due
    LoansRepayOrForebearance91To120DaysPastDue,
    ///ACM - Loans Repay or Forebearance - 121 to 180 Days Past Due
    LoansRepayOrForebearance121To180DaysPastDue,
    ///ACN - Loans Repay or Forebearance - 181 to 270 Days Past Due
    LoansRepayOrForebearance181To270DaysPastDue,
    ///ACO - Loans Repay or Forebearance - 271 or More Days Past Due
    LoansRepayOrForebearance271OrMoreDaysPastDue,
    ///ACP - Loans Repay or Forebearance - Claims Filed, Not Yet Paid
    CodeACP,
    ///ACQ - Agent Sales
    AgentSales,
    ///ACR - Amount Involved
    AmountInvolved,
    ///ACS - Assigned Capital
    AssignedCapital,
    ///ACT - Credit Line Utilized
    CreditLineUtilized,
    ///ACU - Direct Sales
    DirectSales,
    ///ACV - Earnings per Share
    EarningsPerShare,
    ///ACW - Inheritance
    Inheritance,
    ///ACX - Invested Capital
    InvestedCapital,
    ///ACY - Loan from Family Members
    LoanFromFamilyMembers,
    ///ACZ - Non Depreciable Assets
    NonDepreciableAssets,
    ///AD - Adjusted Total
    AdjustedTotal,
    ///ADA - Partially Paid Amount per Share
    PartiallyPaidAmountPerShare,
    ///ADB - Pending Orders
    PendingOrders,
    ///ADC - Personal Loan
    PersonalLoan,
    ///ADD - Plant and Machinery
    PlantAndMachinery,
    ///ADE - Pre-Tax Loss
    PreTaxLoss,
    ///ADF - Pre-Tax Profit
    PreTaxProfit,
    ///ADG - Registered Capital
    RegisteredCapital,
    ///ADH - Revaluation Reserves
    RevaluationReserves,
    ///ADI - Social Capital
    SocialCapital,
    ///ADJ - Statutory Profit
    StatutoryProfit,
    ///ADK - Training Pay
    TrainingPay,
    ///ADL - Retroactive Pay
    RetroactivePay,
    ///ADM - Expected Reimbursement Amount
    ExpectedReimbursementAmount,
    ///ADN - Permit Cost
    PermitCost,
    ///ADO - Minimum
    Minimum,
    ///ADP - Additional Amount to Meet Minimum
    AdditionalAmountToMeetMinimum,
    ///ADQ - Labor Per Hour
    LaborPerHour,
    ///ADR - Non-recoverable Depreciation
    NonRecoverableDepreciation,
    ///ADS - Recoverable Depreciation
    RecoverableDepreciation,
    ///ADT - Overhead
    Overhead,
    ///ADU - Indemnity Benefit
    IndemnityBenefit,
    ///ADW - Replacement Cost of Repairs
    ReplacementCostOfRepairs,
    ///ADX - Actual Cash Value of Repairs
    ActualCashValueOfRepairs,
    ///ADY - Recoverable Depreciation of Repairs
    RecoverableDepreciationOfRepairs,
    ///ADZ - Non-recoverable Depreciation of Repairs
    NonRecoverableDepreciationOfRepairs,
    ///AE - Arrearage
    Arrearage,
    ///AEA - Non-indemnity Benefit
    NonIndemnityBenefit,
    ///AEB - Actual Cash Value of Building
    ActualCashValueOfBuilding,
    ///AEC - Government Share
    GovernmentShare,
    ///AED - Contractor Share
    ContractorShare,
    ///AEE - Award Fee
    AwardFee,
    ///AEF - Base Fee
    BaseFee,
    ///AEG - Target Profit Floor
    TargetProfitFloor,
    ///AEH - Target Profit Ceiling
    TargetProfitCeiling,
    ///AEI - Labor Per Day
    LaborPerDay,
    ///AEJ - Difference in Interest Due
    DifferenceInInterestDue,
    ///AEK - Difference in Prepayment Penalty
    DifferenceInPrepaymentPenalty,
    ///AEL - Difference in Principal Due
    DifferenceInPrincipalDue,
    ///AEM - Appropriation of Retained Earnings Less Reversals
    AppropriationOfRetainedEarningsLessReversals,
    ///AEN - Appropriations
    Appropriations,
    ///AEO - Billings and Costs-Profit Differential
    BillingsAndCostsProfitDifferential,
    ///AEP - Common Stock Par Value
    CommonStockParValue,
    ///AEQ - Cost of Services Rendered
    CostOfServicesRendered,
    ///AER - Creditors
    Creditors,
    ///AES - Declared Profit
    DeclaredProfit,
    ///AET - Discounted Notes
    DiscountedNotes,
    ///AEU - Endorsed Notes
    EndorsedNotes,
    ///AEV - General Reserves
    GeneralReserves,
    ///AEW - Import Volume
    ImportVolume,
    ///AEX - Income Tax Credit
    IncomeTaxCredit,
    ///AEY - Long-Term Deposits
    LongTermDeposits,
    ///AEZ - Long-Term Loans
    LongTermLoans,
    ///AF - Average Float
    AverageFloat,
    ///AFA - Minority Interest
    MinorityInterest,
    ///AFB - Non-Operating Expense
    NonOperatingExpense,
    ///AFC - Non-Operating Income
    NonOperatingIncome,
    ///AFD - Operating Profit or Loss
    OperatingProfitOrLoss,
    ///AFE - Preferred Stock Par Value
    PreferredStockParValue,
    ///AFF - Profit After Tax and Minority Interest
    ProfitAfterTaxAndMinorityInterest,
    ///AFG - Retained Earnings to be Appropriated
    RetainedEarningsToBeAppropriated,
    ///AFH - Revaluation Surplus or Deficit
    RevaluationSurplusOrDeficit,
    ///AFI - Reversal of Voluntary Earned Surplus
    ReversalOfVoluntaryEarnedSurplus,
    ///AFJ - Share Price
    SharePrice,
    ///AFK - Short-Term Deposits
    ShortTermDeposits,
    ///AFL - Short-Term Loans
    ShortTermLoans,
    ///AFM - Tax Provisions
    TaxProvisions,
    ///AFN - Unallocated Profit
    UnallocatedProfit,
    ///AFO - Voluntary Earned Surplus
    VoluntaryEarnedSurplus,
    ///AFP - Calculated Weekly Compensation Amount
    CalculatedWeeklyCompensationAmount,
    ///AFQ - Benefit Type Gross Weekly Amount
    BenefitTypeGrossWeeklyAmount,
    ///AFR - Benefit Type Net Weekly Amount
    BenefitTypeNetWeeklyAmount,
    ///AFS - Employee Gross Wage
    EmployeeGrossWage,
    ///AFT - Garage Gross Wages
    GarageGrossWages,
    ///AFU - Officer Compensation - Actual Flat
    OfficerCompensationActualFlat,
    ///AFV - Officer Compensation - Statutory Maximum
    OfficerCompensationStatutoryMaximum,
    ///AFW - Officer Compensation - Statutory Minimum
    OfficerCompensationStatutoryMinimum,
    ///AFY - Previous Balance
    PreviousBalance,
    ///AFZ - Disputed Amount
    DisputedAmount,
    ///AG - Adjusted Gross Income
    AdjustedGrossIncome,
    ///AGA - Non-Operating Income or Expense
    NonOperatingIncomeOrExpense,
    ///AGB - Operating Income or Expense
    OperatingIncomeOrExpense,
    ///AGC - Income or Expense
    IncomeOrExpense,
    ///AGD - Purchase Authority
    PurchaseAuthority,
    ///AGE - Capital Decrease
    CapitalDecrease,
    ///AGF - Capital Increase
    CapitalIncrease,
    ///AGG - Deed Capital
    DeedCapital,
    ///AGH - Transferred Amount
    TransferredAmount,
    ///AGI - Unadjusted Sales Price
    UnadjustedSalesPrice,
    ///AGJ - Sales Concessions
    SalesConcessions,
    ///AGK - Property Value
    PropertyValue,
    ///AGM - Partial Release Amount
    PartialReleaseAmount,
    ///AGN - Lien Filing Fee
    LienFilingFee,
    ///AGO - Additional Repair Price
    AdditionalRepairPrice,
    ///AGP - Qualified Tuition and Related Expenses
    QualifiedTuitionAndRelatedExpenses,
    ///AGQ - Qualified Financial Assistance
    QualifiedFinancialAssistance,
    ///AGR - Aggregate Reimbursements or Refunds
    AggregateReimbursementsOrRefunds,
    ///AGS - New Loan Balance
    NewLoanBalance,
    ///AGT - Raw Material Purchases
    RawMaterialPurchases,
    ///AGU - Work in Progress Purchases
    WorkInProgressPurchases,
    ///AGV - Operating Cash Flow
    OperatingCashFlow,
    ///AGW - Payments for Outside Work
    PaymentsForOutsideWork,
    ///AGX - Set Aside for Provisions
    SetAsideForProvisions,
    ///AGY - Financial Income or Expense
    FinancialIncomeOrExpense,
    ///AGZ - Extraordinary Income or Expense
    ExtraordinaryIncomeOrExpense,
    ///AH - Loan Balance Difference
    LoanBalanceDifference,
    ///AHA - Unfinished Production Carried Forward
    UnfinishedProductionCarriedForward,
    ///AHB - Installation Materials Cost
    InstallationMaterialsCost,
    ///AHC - New Mortgage Amount
    NewMortgageAmount,
    ///AHD - Capitalized Assets
    CapitalizedAssets,
    ///AHE - Profit Reserves
    ProfitReserves,
    ///AHF - Share in Profit or Loss in Other Companies
    ShareInProfitOrLossInOtherCompanies,
    ///AHG - Monetary Correction
    MonetaryCorrection,
    ///AHI - Scheduled Repayment Amount
    ScheduledRepaymentAmount,
    ///AHJ - Amounts Placed with Other Banks
    AmountsPlacedWithOtherBanks,
    ///AHK - Due from Parent Company
    DueFromParentCompany,
    ///AHL - Owing from Subsidiary Companies
    OwingFromSubsidiaryCompanies,
    ///AHM - Certificates of Deposit Amount
    CertificatesOfDepositAmount,
    ///AHN - Publicly Traded Shares Amount
    PubliclyTradedSharesAmount,
    ///AHO - Non Publicly Traded Shares Amount
    NonPubliclyTradedSharesAmount,
    ///AHP - Trading Securities Amount
    TradingSecuritiesAmount,
    ///AHQ - Investment Securities Amount
    InvestmentSecuritiesAmount,
    ///AHR - Earnings Per Share Minus Dividends
    EarningsPerShareMinusDividends,
    ///AHS - Active Partner Capital
    ActivePartnerCapital,
    ///AI - Sale Amount
    SaleAmount,
    ///AJ - Funds Held by Mortgagee
    FundsHeldByMortgagee,
    ///AJC - Adjusted Claim
    AdjustedClaim,
    ///AK - Attorney Fees
    AttorneyFees,
    ///AL - Average Ledger Balance
    AverageLedgerBalance,
    ///AM - Amount Financed
    AmountFinanced,
    ///AN - Bankruptcy Fee
    BankruptcyFee,
    ///ANC - Accrued Income
    AccruedIncome,
    ///AO - Amount Override
    AmountOverride,
    ///AP - Amount Prior to Fractionalization
    AmountPriorToFractionalization,
    ///APT - Amount of Purchase Exempt From Tax or Fee
    AmountOfPurchaseExemptFromTaxOrFee,
    ///AQ - Average Price Per Call
    AveragePricePerCall,
    ///AQL - Acquisition Cost of Lenses
    AcquisitionCostOfLenses,
    ///AR - Fees to Public Officials for Foreclosure
    FeesToPublicOfficialsForForeclosure,
    ///AS - Average Price Per Minute
    AveragePricePerMinute,
    ///ASP - Annual Sales or Premiums
    AnnualSalesOrPremiums,
    ///AT - Total Received
    TotalReceived,
    ///ATF - Amount of Tax or Fee Exemption
    AmountOfTaxOrFeeExemption,
    ///AU - Coverage Amount
    CoverageAmount,
    ///AV - Actual Cash Value
    ActualCashValue,
    ///AVE - Average
    Average,
    ///AW - Replacement Cost
    ReplacementCost,
    ///AX - Previous Price
    PreviousPrice,
    ///AY - Title Cost
    TitleCost,
    ///AZ - Other Foreclosure and Acquisition Expenses
    OtherForeclosureAndAcquisitionExpenses,
    ///B - Estimated
    Estimated,
    ///B0 - Bond
    Bond,
    ///B1 - Benefit Amount
    BenefitAmount,
    ///B2 - Bonuses and Commissions Divided Over 12 Months
    BonusesAndCommissionsDividedOver12Months,
    ///B3 - Bonuses Divided Over 12 Months
    BonusesDividedOver12Months,
    ///B4 - Bonuses and Commissions
    BonusesAndCommissions,
    ///B5 - Budgeted
    Budgeted,
    ///B6 - Allowed - Actual
    AllowedActual,
    ///B7 - Deductible - Estimated
    DeductibleEstimated,
    ///B8 - Co-insurance - Estimated
    CoInsuranceEstimated,
    ///B9 - Co-insurance - Actual
    CoInsuranceActual,
    ///BA - Bargain
    Bargain,
    ///BAA - Net Taxable Income
    NetTaxableIncome,
    ///BAB - Original Amount of Instrument
    OriginalAmountOfInstrument,
    ///BAC - Addition to Tax
    AdditionToTax,
    ///BAD - Reinstatement Fee
    ReinstatementFee,
    ///BAE - Permit Fee Due
    PermitFeeDue,
    ///BAF - Permit Fee with Extension
    PermitFeeWithExtension,
    ///BAG - Net Annual Permit Fee Due
    NetAnnualPermitFeeDue,
    ///BAH - Permit Fee Penalty Due
    PermitFeePenaltyDue,
    ///BAI - Permit Fee Interest Due
    PermitFeeInterestDue,
    ///BAJ - Total Permit Fee Due
    TotalPermitFeeDue,
    ///BAK - Franchise Tax
    FranchiseTax,
    ///BAL - Unclaimed Franchise Tax Credit
    UnclaimedFranchiseTaxCredit,
    ///BAM - Net Franchise Tax Due
    NetFranchiseTaxDue,
    ///BAN - Franchise Tax Penalty Due
    FranchiseTaxPenaltyDue,
    ///BAO - Total Franchise Tax Due
    TotalFranchiseTaxDue,
    ///BAP - Total Amount Due
    TotalAmountDue,
    ///BAQ - Overpayment
    Overpayment,
    ///BAR - Amount to be Refunded
    AmountToBeRefunded,
    ///BAS - Gross In-State Receipts
    GrossInStateReceipts,
    ///BAT - Gross Receipts
    GrossReceipts,
    ///BAU - Occupation Fee
    OccupationFee,
    ///BAV - Total Assessed Value
    TotalAssessedValue,
    ///BAW - Total Value of All Property
    TotalValueOfAllProperty,
    ///BAX - Value of In-State Property
    ValueOfInStateProperty,
    ///BAY - Total Value of Out-of-State Property
    TotalValueOfOutOfStateProperty,
    ///BAZ - Total Par Value
    TotalParValue,
    ///BB - Mortgage Insurance Premiums
    MortgageInsurancePremiums,
    ///BBA - Total Assessable Capital Stock
    TotalAssessableCapitalStock,
    ///BBB - Apportioned Value
    ApportionedValue,
    ///BBC - Estimated In-State Real Property Value
    EstimatedInStateRealPropertyValue,
    ///BBD - Par Value of In-State Assets
    ParValueOfInStateAssets,
    ///BBE - In-State Business Revenue
    InStateBusinessRevenue,
    ///BBF - Subscription Price
    SubscriptionPrice,
    ///BBG - Value of Authorized Shares
    ValueOfAuthorizedShares,
    ///BBH - Ordinance Amount
    OrdinanceAmount,
    ///BBI - Capital for a Cooperative
    CapitalForACooperative,
    ///BBJ - Directors Legal Obligation Debt
    DirectorsLegalObligationDebt,
    ///BBK - Silent Partner Capital
    SilentPartnerCapital,
    ///BC - Billing Cycle Net Fee Position (Excess/Deficit)
    CodeBC,
    ///BD - Balance Due
    BalanceDue,
    ///BE - Disbursements for Authorized Repair
    DisbursementsForAuthorizedRepair,
    ///BF - Hazard Insurance Premium
    HazardInsurancePremium,
    ///BG - Eviction Attorney Fees
    EvictionAttorneyFees,
    ///BH - Eviction Expenses
    EvictionExpenses,
    ///BI - Property Taxes
    PropertyTaxes,
    ///BJ - Disbursements Not Shown Elsewhere
    DisbursementsNotShownElsewhere,
    ///BK - Disbursements for Protection and Preservation
    DisbursementsForProtectionAndPreservation,
    ///BL - Disbursements for Inspections and Boarding
    DisbursementsForInspectionsAndBoarding,
    ///BLD - Bridge Loan Not Deposited
    BridgeLoanNotDeposited,
    ///BM - Adjustments
    Adjustments,
    ///BN - Rental Income
    RentalIncome,
    ///BO - Rental Expense
    RentalExpense,
    ///BOA - Borrower Advance
    BorrowerAdvance,
    ///BOR - Borrowed Amount
    BorrowedAmount,
    ///BP - Average Net Collected Balance
    AverageNetCollectedBalance,
    ///BQ - Bail
    Bail,
    ///BR - Adjusted Insured Loss Amount
    AdjustedInsuredLossAmount,
    ///BS - Mortgage Note Interest
    MortgageNoteInterest,
    ///BT - Bank Reject Total
    BankRejectTotal,
    ///BTR - Betterment
    Betterment,
    ///BU - Overhead Costs
    OverheadCosts,
    ///BV - Uncollected Interest
    UncollectedInterest,
    ///BW - Amount Due from Buyer at Closing
    AmountDueFromBuyerAtClosing,
    ///BX - Amount Owed to Buyer at Closing
    AmountOwedToBuyerAtClosing,
    ///BXP - Bills of Exchange Payable
    BillsOfExchangePayable,
    ///BY - Additional Closing Expenses
    AdditionalClosingExpenses,
    ///BZ - Deficiency Judgment Expenses
    DeficiencyJudgmentExpenses,
    ///C - City
    City,
    ///C0 - Current Expenditures
    CurrentExpenditures,
    ///C1 - Co-Payment Amount
    CoPaymentAmount,
    ///C2 - Child Rider Coverage
    ChildRiderCoverage,
    ///C3 - Prior Payment - Estimated
    PriorPaymentEstimated,
    ///C4 - Prior Payment - Actual
    PriorPaymentActual,
    ///C5 - Claim Amount Due - Estimated
    ClaimAmountDueEstimated,
    ///C6 - Claim Amount Due - Actual
    ClaimAmountDueActual,
    ///C7 - Payer Responsibility - Estimated
    PayerResponsibilityEstimated,
    ///C8 - Payer Responsibility - Actual
    PayerResponsibilityActual,
    ///C9 - Disallowed Cost Containment - Actual
    DisallowedCostContainmentActual,
    ///CA - Contractor Cumulative to Date
    ContractorCumulativeToDate,
    ///CB - Collected Balance Required
    CollectedBalanceRequired,
    ///CC - Chargeback Claim Amount
    ChargebackClaimAmount,
    ///CD - Overpaid Section 235 Subsidy
    OverpaidSection235Subsidy,
    ///CDC - Clean-up Costs Associated with Deficiency
    CleanUpCostsAssociatedWithDeficiency,
    ///CE - Summary Amount
    SummaryAmount,
    ///CF - Appraisal Fees
    AppraisalFees,
    ///CG - Commission Fees Deducted
    CommissionFeesDeducted,
    ///CH - Change Amount
    ChangeAmount,
    ///CI - Funds Held for Insured
    FundsHeldForInsured,
    ///CJ - Other Deductions
    OtherDeductions,
    ///CJL - Collections, Judgments, and Liens
    CodeCJL,
    ///CK - Back End Load
    BackEndLoad,
    ///CL - Outstanding Balance Current Lender
    OutstandingBalanceCurrentLender,
    ///CM - Claimant Requested Total
    ClaimantRequestedTotal,
    ///CMC - Compression Charge
    CompressionCharge,
    ///CMR - Commodity Refund
    CommodityRefund,
    ///CN - Special Assessments
    SpecialAssessments,
    ///CO - Taxes on Deed
    TaxesOnDeed,
    ///COA - Corporate Assets
    CorporateAssets,
    ///CP - Statutory Disbursements
    StatutoryDisbursements,
    ///CPO - Closing Costs Paid by Any Other Party Other Than Seller or Buyer
    ClosingCostsPaidByAnyOtherPartyOtherThanSellerOrBuyer,
    ///CPS - Closing Costs Paid by Seller
    ClosingCostsPaidBySeller,
    ///CQ - Net Claim Amount
    NetClaimAmount,
    ///CR - Contractor at Complete
    ContractorAtComplete,
    ///CRC - Container Replacement Cost
    ContainerReplacementCost,
    ///CRL - Container Replacement Labor Cost
    ContainerReplacementLaborCost,
    ///CRM - Container Replacement Material Cost
    ContainerReplacementMaterialCost,
    ///CS - Commission Sales
    CommissionSales,
    ///CT - Contract
    Contract,
    ///CU - Subcontractor Cumulative to Date
    SubcontractorCumulativeToDate,
    ///CV - Subcontractor at Complete
    SubcontractorAtComplete,
    ///CW - Earned Value
    EarnedValue,
    ///CX - Actual
    Actual,
    ///CY - Cumulative Budget
    CumulativeBudget,
    ///CZ - Cumulative Earned Value
    CumulativeEarnedValue,
    ///D - Payer Amount Paid
    PayerAmountPaid,
    ///D0 - Administration and Management Costs
    AdministrationAndManagementCosts,
    ///D1 - Deferred Compensation Commissions
    DeferredCompensationCommissions,
    ///D2 - Deductible Amount
    DeductibleAmount,
    ///D3 - Deferred Compensation Commissions and Bonuses
    DeferredCompensationCommissionsAndBonuses,
    ///D4 - Deferred Compensation
    DeferredCompensation,
    ///D4D - Dollar For Dollar Deductions
    DollarForDollarDeductions,
    ///D5 - Dependent Care Contribution
    DependentCareContribution,
    ///D6 - Disallowed Cost Containment - Estimated
    DisallowedCostContainmentEstimated,
    ///D7 - Dispensing Fee
    DispensingFee,
    ///D8 - Discount Amount
    DiscountAmount,
    ///D9 - Cumulative Actual
    CumulativeActual,
    ///DA - Original Mortgage
    OriginalMortgage,
    ///DB - Unapplied Section 235 Funds
    UnappliedSection235Funds,
    ///DC - Unapplied Buydown Fund
    UnappliedBuydownFund,
    ///DD - Direct Deposit
    DirectDeposit,
    ///DE - Estimate of Damage
    EstimateOfDamage,
    ///DEF - Deferral
    Deferral,
    ///DEI - Delayed Interest
    DelayedInterest,
    ///DF - Authorized Bid
    AuthorizedBid,
    ///DFA - Deferred Assets
    DeferredAssets,
    ///DG - Escrow Balance
    EscrowBalance,
    ///DH - Total Disbursements
    TotalDisbursements,
    ///DI - Charge Off
    ChargeOff,
    ///DJ - Liens Amount Original
    LiensAmountOriginal,
    ///DK - Release of Lien
    ReleaseOfLien,
    ///DL - Debit
    Debit,
    ///DLQ - Delinquent Taxes
    DelinquentTaxes,
    ///DM - Asset
    Asset,
    ///DN - Liability
    Liability,
    ///DO - Satisfaction
    Satisfaction,
    ///DP - Exemption
    Exemption,
    ///DPF - Discount Points Financed
    DiscountPointsFinanced,
    ///DPN - Discount Points Not Financed
    DiscountPointsNotFinanced,
    ///DPP - Defaulted Tax Plan Payment
    DefaultedTaxPlanPayment,
    ///DQ - Settlement
    Settlement,
    ///DR - Alimony Expense
    AlimonyExpense,
    ///DS - Alimony Income
    AlimonyIncome,
    ///DT - Child Support Expense
    ChildSupportExpense,
    ///DU - Child Support Income
    ChildSupportIncome,
    ///DV - Separate Maintenance Expense
    SeparateMaintenanceExpense,
    ///DVP - Development Properties
    DevelopmentProperties,
    ///DW - Separate Maintenance Income
    SeparateMaintenanceIncome,
    ///DX - Deductible Waived
    DeductibleWaived,
    ///DY - Per Day Limit
    PerDayLimit,
    ///DZ - Job-related Expense
    JobRelatedExpense,
    ///E - Estimated Credit
    EstimatedCredit,
    ///E0 - Administration and Management Indemnity Charge
    AdministrationAndManagementIndemnityCharge,
    ///E1 - Employer Year to Date Contribution
    EmployerYearToDateContribution,
    ///E2 - Employee Annual Pledge Amount
    EmployeeAnnualPledgeAmount,
    ///E3 - Employee Current Contribution
    EmployeeCurrentContribution,
    ///E4 - Employer Pledge Amount
    EmployerPledgeAmount,
    ///E5 - Employer Current Contribution
    EmployerCurrentContribution,
    ///E6 - Eligible Wage Amount
    EligibleWageAmount,
    ///E7 - Employee Year to Date Contribution
    EmployeeYearToDateContribution,
    ///E8 - Education Contribution
    EducationContribution,
    ///E9 - Initial Fee
    InitialFee,
    ///EA - Earnings Allowance
    EarningsAllowance,
    ///EAA - Administrative Expenses
    AdministrativeExpenses,
    ///EAB - Air Travel Expenses
    AirTravelExpenses,
    ///EAC - Amount Forgiven
    AmountForgiven,
    ///EAD - Amount Guaranteed
    AmountGuaranteed,
    ///EAE - Amount Over Fair Market Value
    AmountOverFairMarketValue,
    ///EAF - Amount Owed
    AmountOwed,
    ///EAG - Amount Payable
    AmountPayable,
    ///EAH - Amount Raised
    AmountRaised,
    ///EAI - Amount Received
    AmountReceived,
    ///EAJ - Amount Refunded
    AmountRefunded,
    ///EAK - Amount Rescinded
    AmountRescinded,
    ///EAL - Anonymous Contribution
    AnonymousContribution,
    ///EAM - Balance Owed
    BalanceOwed,
    ///EAN - Bank Charges
    BankCharges,
    ///EAO - Bank Loan
    BankLoan,
    ///EAP - Brochure Expenses
    BrochureExpenses,
    ///EAQ - Bus Travel Expenses
    BusTravelExpenses,
    ///EAR - Consultant Expenses
    ConsultantExpenses,
    ///EAS - Corrected Amount
    CorrectedAmount,
    ///EAT - Disability Expenses
    DisabilityExpenses,
    ///EAU - Disposed Amount
    DisposedAmount,
    ///EAV - Draw Amount
    DrawAmount,
    ///EAW - Election Expenses
    ElectionExpenses,
    ///EAX - Endorsement Amount
    EndorsementAmount,
    ///EAY - Entertainment Expenses
    EntertainmentExpenses,
    ///EAZ - Excess Expenses
    ExcessExpenses,
    ///EB - Collected Balance (Excess/Deficit)
    CodeEB,
    ///EBA - Expected Expenditure Amount
    ExpectedExpenditureAmount,
    ///EBB - Expenditure Amount
    ExpenditureAmount,
    ///EBC - Family Care Expenses
    FamilyCareExpenses,
    ///EBD - Federal Share Amount
    FederalShareAmount,
    ///EBE - Filing Fee
    FilingFee,
    ///EBG - In-Kind Contribution
    InKindContribution,
    ///EBH - Incurred Amount
    IncurredAmount,
    ///EBI - Loan Amount Plus Interest
    LoanAmountPlusInterest,
    ///EBJ - Loan Balance
    LoanBalance,
    ///EBK - Matching Contribution
    MatchingContribution,
    ///EBL - Meeting Expenses
    MeetingExpenses,
    ///EBM - Miscellaneous Income
    MiscellaneousIncome,
    ///EBN - Miscellaneous Receipts
    MiscellaneousReceipts,
    ///EBO - New Loan Amount
    NewLoanAmount,
    ///EBP - New Unpaid Expenditure
    NewUnpaidExpenditure,
    ///EBQ - Newsletter Expenses
    NewsletterExpenses,
    ///EBR - Newspaper Advertising Expenses
    NewspaperAdvertisingExpenses,
    ///EBS - Nomination Expenses
    NominationExpenses,
    ///EBT - Non-Federal Share
    NonFederalShare,
    ///EBU - Office Expenses
    OfficeExpenses,
    ///EBV - Office Rental
    OfficeRental,
    ///EBW - Original Asset Value
    OriginalAssetValue,
    ///EBX - Original Loan Amount
    OriginalLoanAmount,
    ///EBY - Party Expenses
    PartyExpenses,
    ///EBZ - Payment
    Payment,
    ///EC - Allowance (Excess/Deficit)
    CodeEC,
    ///ECA - Personal Expenses
    PersonalExpenses,
    ///ECB - Personal Funds
    PersonalFunds,
    ///ECC - Pledged Amount
    PledgedAmount,
    ///ECD - Postage Expenses
    PostageExpenses,
    ///ECE - Printing Expenses
    PrintingExpenses,
    ///ECF - Public Funds
    PublicFunds,
    ///ECG - Radio Advertising Expenses
    RadioAdvertisingExpenses,
    ///ECH - Reimbursed Amount
    ReimbursedAmount,
    ///ECI - Reported Amount
    ReportedAmount,
    ///ECJ - Retainer
    Retainer,
    ///ECK - Sign Expenses
    SignExpenses,
    ///ECL - Sub-Contract Value
    SubContractValue,
    ///ECM - Tax Receipts
    TaxReceipts,
    ///ECN - Taxi Travel Expenses
    TaxiTravelExpenses,
    ///ECO - Telecommunication Expenses
    TelecommunicationExpenses,
    ///ECP - Television Advertising Expenses
    TelevisionAdvertisingExpenses,
    ///ECQ - Unpaid Expenditure
    UnpaidExpenditure,
    ///ECR - Utilities Expenses
    UtilitiesExpenses,
    ///ECS - Total
    Total,
    ///ECT - Subtotal
    Subtotal,
    ///ECU - Grand Total
    GrandTotal,
    ///ECV - Incidental Expenses
    IncidentalExpenses,
    ///ECW - Transportation Expenses
    TransportationExpenses,
    ///ECX - Gift Value
    GiftValue,
    ///ECY - Food and Refreshments
    FoodAndRefreshments,
    ///ECZ - Polling Expenses
    PollingExpenses,
    ///ED - Estimated Cost of Attendance
    EstimatedCostOfAttendance,
    ///ED4 - Tuition and Required Fees
    TuitionAndRequiredFees,
    ///EDB - Books and Supplies
    BooksAndSupplies,
    ///EE - Other Expense
    OtherExpense,
    ///EF - Estimated Financial Aid
    EstimatedFinancialAid,
    ///EG - Other Income
    OtherIncome,
    ///EH - Amount of Mortgages and Liens
    AmountOfMortgagesAndLiens,
    ///EI - Mortgage Payment(s)
    CodeEI,
    ///EIP - Maintenance Expense on Income Producing Property
    MaintenanceExpenseOnIncomeProducingProperty,
    ///EJ - Insurance, Maintenance, Taxes and Miscellaneous
    CodeEJ,
    ///EK - Net Rental Income
    NetRentalIncome,
    ///EL - Present Market Value
    PresentMarketValue,
    ///ELT - Electroconvulsive Therapy (ECT) Adjustment
    CodeELT,
    ///EM - Gross Rental Income
    GrossRentalIncome,
    ///EN - Cancellation Fee
    CancellationFee,
    ///ENP - Maintenance Expense on Non-Income Producing Property
    MaintenanceExpenseOnNonIncomeProducingProperty,
    ///ENT - Entitlement Amount
    EntitlementAmount,
    ///EO - Capital Reserves
    CapitalReserves,
    ///EP - Employer Annual Pledge Amount
    EmployerAnnualPledgeAmount,
    ///EQ - Condominium Association Fees
    CondominiumAssociationFees,
    ///ER - Homeowner Association Fees
    HomeownerAssociationFees,
    ///ERN - Earnest Money
    EarnestMoney,
    ///ES - Mortgage Insurance Proceeds
    MortgageInsuranceProceeds,
    ///ET - Net Proceeds from Sale of Real Estate Property
    NetProceedsFromSaleOfRealEstateProperty,
    ///ETD - Ever To Date (ETD) Claim Loss
    CodeETD,
    ///EU - Insurance Proceeds (Primary Settlement)
    CodeEU,
    ///EV - Presale Proceeds
    PresaleProceeds,
    ///EW - Pledged Savings
    PledgedSavings,
    ///EX - As Is Broker's Opinion
    AsIsBrokersOpinion,
    ///EXC - Exploration Costs
    ExplorationCosts,
    ///EY - Subject To Broker's Opinion
    SubjectToBrokersOpinion,
    ///EZ - Uniform Commercial Code Filing Office Fee
    UniformCommercialCodeFilingOfficeFee,
    ///F - Annual Limit
    AnnualLimit,
    ///F0 - Commercial Staff Labor Costs
    CommercialStaffLaborCosts,
    ///F1 - Maximum Allowable Cost (MAC) Penalty Copay
    CodeF1,
    ///F2 - Patient Responsibility - Actual
    PatientResponsibilityActual,
    ///F3 - Patient Responsibility - Estimated
    PatientResponsibilityEstimated,
    ///F4 - Postage Claimed
    PostageClaimed,
    ///F5 - Patient Amount Paid
    PatientAmountPaid,
    ///F6 - Provider Reserves
    ProviderReserves,
    ///F7 - Sales Tax
    SalesTax,
    ///F8 - Usual and Customary Charge - Estimated
    UsualAndCustomaryChargeEstimated,
    ///F9 - Usual and Customary - Actual
    UsualAndCustomaryActual,
    ///FA - Coordination Fee
    CoordinationFee,
    ///FAR - Federal Housing Administration (FHA) Appraiser Required Repairs and Improvements
    CodeFAR,
    ///FB - Calculation Fee
    CalculationFee,
    ///FBA - Final Balance
    FinalBalance,
    ///FC - Expected Family Contribution
    ExpectedFamilyContribution,
    ///FD - Direct Deposit Flipped to Check
    DirectDepositFlippedToCheck,
    ///FE - Fee
    Fee,
    ///FF - Application Fee
    ApplicationFee,
    ///FG - Licensing Fee
    LicensingFee,
    ///FH - Regulatory Fee
    RegulatoryFee,
    ///FI - First Interest Payment Amount
    FirstInterestPaymentAmount,
    ///FJ - Waiver Fee
    WaiverFee,
    ///FK - Other Unlisted Amount
    OtherUnlistedAmount,
    ///FL - Float
    Float,
    ///FLA - First Lien Advance
    FirstLienAdvance,
    ///FM - Fair Market Value
    FairMarketValue,
    ///FN - Fine
    Fine,
    ///FO - Fees Paid
    FeesPaid,
    ///FOA - Foreign Assets
    ForeignAssets,
    ///FP - Fees Paid Year to Date
    FeesPaidYearToDate,
    ///FQ - Firm Contractor Share
    FirmContractorShare,
    ///FR - Estimated Government Share
    EstimatedGovernmentShare,
    ///FRB - Facilities Refund
    FacilitiesRefund,
    ///FS - Expense
    Expense,
    ///FT - Endorsement Premium Amount
    EndorsementPremiumAmount,
    ///FTR - Fuel Tracker Refund
    FuelTrackerRefund,
    ///FTX - Facility tax amount
    FacilityTaxAmount,
    ///FU - Commercial Staff Indemnity Charge
    CommercialStaffIndemnityCharge,
    ///FV - Flat Fee Paid to Date
    FlatFeePaidToDate,
    ///FW - Flat Fee Paid Current Month
    FlatFeePaidCurrentMonth,
    ///FX - Endorsement
    Endorsement,
    ///FY - First Payment
    FirstPayment,
    ///FZ - Earned Income
    EarnedIncome,
    ///G - Collateral
    Collateral,
    ///G0 - Initial Adjustment Total
    InitialAdjustmentTotal,
    ///G1 - Indicated Value by Sales Comparison Approach
    IndicatedValueBySalesComparisonApproach,
    ///G2 - Indicated Value by Income Approach
    IndicatedValueByIncomeApproach,
    ///G3 - Price per Unit Area
    PricePerUnitArea,
    ///G4 - Reconciliation of Final Value Estimate
    ReconciliationOfFinalValueEstimate,
    ///G5 - Estimated Monthly Market Rent
    EstimatedMonthlyMarketRent,
    ///G6 - Adjusted Sales Price
    AdjustedSalesPrice,
    ///G7 - Sales or Financing Concessions
    SalesOrFinancingConcessions,
    ///G8 - Indicated Value by Cost Approach
    IndicatedValueByCostApproach,
    ///G9 - As-is Value of Site Improvements
    AsIsValueOfSiteImprovements,
    ///GA - Depreciated Value of Improvements
    DepreciatedValueOfImprovements,
    ///GAR - Garnishments
    Garnishments,
    ///GAT - Annual Tax
    AnnualTax,
    ///GB - Price, High Value
    CodeGB,
    ///GC - Price, Low Value
    CodeGC,
    ///GCB - Corrected Tax Bill
    CorrectedTaxBill,
    ///GCL - Gross Claim
    GrossClaim,
    ///GD - Physical Depreciation
    PhysicalDepreciation,
    ///GE - Functional Depreciation
    FunctionalDepreciation,
    ///GF - External Depreciation
    ExternalDepreciation,
    ///GFD - Gifts Not Deposited
    GiftsNotDeposited,
    ///GFT - Gift Amount
    GiftAmount,
    ///GG - Adjusted Sales Price of Comparable Sales
    AdjustedSalesPriceOfComparableSales,
    ///GH - Predominate Value
    PredominateValue,
    ///GI - Average Customer Income
    AverageCustomerIncome,
    ///GJ - Average Neighborhood Income
    AverageNeighborhoodIncome,
    ///GK - Average Customer Purchase
    AverageCustomerPurchase,
    ///GL - Weekly Dollar Sales
    WeeklyDollarSales,
    ///GM - Average Case Sales
    AverageCaseSales,
    ///GN - Buy-down
    BuyDown,
    ///GO - Credit Line
    CreditLine,
    ///GP - Appraisal Repair Amount
    AppraisalRepairAmount,
    ///GQ - Brokers Opinion Repair Amount
    BrokersOpinionRepairAmount,
    ///GR - Credit Line Available
    CreditLineAvailable,
    ///GRC - Gross Contribution
    GrossContribution,
    ///GRD - Grants Not Deposited
    GrantsNotDeposited,
    ///GRR - Gross Restoration
    GrossRestoration,
    ///GRT - Grant Amount
    GrantAmount,
    ///GS - Subsequent Adjustments Total
    SubsequentAdjustmentsTotal,
    ///GSP - Good Standing Tax Plan Payment
    GoodStandingTaxPlanPayment,
    ///GT - Goods and Services Tax
    GoodsAndServicesTax,
    ///GTS - Total Supplemental Tax Due
    TotalSupplementalTaxDue,
    ///GU - Taxes Paid
    TaxesPaid,
    ///GUI - Tax Installment Due
    TaxInstallmentDue,
    ///GUP - Tax Installment Paid
    TaxInstallmentPaid,
    ///GUS - Total Supplemental Tax Paid
    TotalSupplementalTaxPaid,
    ///GV - Gross Value
    GrossValue,
    ///GW - Total Charge
    TotalCharge,
    ///GX - Total Credit
    TotalCredit,
    ///GY - Total Debit
    TotalDebit,
    ///GZ - Total Finance Charge
    TotalFinanceCharge,
    ///H - Bid Amount
    BidAmount,
    ///H0 - Host Government (government of the institution) Financing for Education
    CodeH0,
    ///H1 - Legal Reserves
    LegalReserves,
    ///H2 - Cancellation
    Cancellation,
    ///H3 - Deposit Inception to Date
    DepositInceptionToDate,
    ///H4 - Deposit Year to Date
    DepositYearToDate,
    ///H5 - Dump in Remittance
    DumpInRemittance,
    ///H6 - Earnings
    Earnings,
    ///H7 - Life Insurance Cash Value
    LifeInsuranceCashValue,
    ///H8 - Structure Value
    StructureValue,
    ///H9 - Original List Price
    OriginalListPrice,
    ///HA - Coin
    Coin,
    ///HB - Currency
    Currency,
    ///HC - U.S. Treasury Checks
    USTreasuryChecks,
    ///HD - Postal Money Orders
    PostalMoneyOrders,
    ///HDA - Home Equity Line of Credit Draw Amount
    HomeEquityLineOfCreditDrawAmount,
    ///HE - City Checks
    CityChecks,
    ///HEM - Hemophilia Adjustment
    HemophiliaAdjustment,
    ///HF - Other Checks
    OtherChecks,
    ///HG - Home Government Financing for Education
    HomeGovernmentFinancingForEducation,
    ///HH - Annual Social Security Wages
    AnnualSocialSecurityWages,
    ///HI - Annual Social Security Tips
    AnnualSocialSecurityTips,
    ///HJ - Annual Wages, Tips, and Other Compensation
    CodeHJ,
    ///HK - Social Security Employee Tax Withheld
    SocialSecurityEmployeeTaxWithheld,
    ///HL - Federal Income Tax Withheld
    FederalIncomeTaxWithheld,
    ///HM - Advance Earned Income Credit
    AdvanceEarnedIncomeCredit,
    ///HN - Commission
    Commission,
    ///HO - Vacation Pay
    VacationPay,
    ///HOA - Head Office Account
    HeadOfficeAccount,
    ///HP - Gross Pay Submitted
    GrossPaySubmitted,
    ///HQ - Intersell Commission Sales
    IntersellCommissionSales,
    ///HR - Total Payroll Approved
    TotalPayrollApproved,
    ///HS - Holiday Pay
    HolidayPay,
    ///HT - Overtime Pay
    OvertimePay,
    ///HU - Regular Pay
    RegularPay,
    ///HV - Sick Pay
    SickPay,
    ///HW - Special Pay
    SpecialPay,
    ///HX - Contract Price
    ContractPrice,
    ///HY - Commercial Space Income
    CommercialSpaceIncome,
    ///HZ - Utilities Paid by Owner
    UtilitiesPaidByOwner,
    ///HZC - Hazardous Clean-up Cost
    HazardousCleanUpCost,
    ///I - Interest
    Interest,
    ///I0 - Life Insurance Coverage
    LifeInsuranceCoverage,
    ///I1 - Investment Income
    InvestmentIncome,
    ///I2 - Income
    Income,
    ///I3 - Price, Gross Living Area
    CodeI3,
    ///I4 - Total Estimated Rent
    TotalEstimatedRent,
    ///I5 - Gross Annual Income
    GrossAnnualIncome,
    ///I6 - Custodian's Salary
    CustodiansSalary,
    ///I7 - Engineer's Salary
    EngineersSalary,
    ///I8 - Elevator Operator's Salary
    ElevatorOperatorsSalary,
    ///I9 - Indicated Value by Market Approach Estimate of Market Value
    IndicatedValueByMarketApproachEstimateOfMarketValue,
    ///IA - Adjusted Monthly Rent
    AdjustedMonthlyRent,
    ///IB - Investable Balance
    InvestableBalance,
    ///IBL - Inter-Bank Loans
    InterBankLoans,
    ///IC - Accrued Unpaid Interest To Be Capitalized
    AccruedUnpaidInterestToBeCapitalized,
    ///ICR - Imbalance Charges Refund
    ImbalanceChargesRefund,
    ///ID - Import Duty Amount
    ImportDutyAmount,
    ///IE - Excise Tax Amount
    ExciseTaxAmount,
    ///IF - Inspection Fee
    InspectionFee,
    ///IG - Adjustment for Gross Living Area
    AdjustmentForGrossLivingArea,
    ///IGT - Inter-Governmental Transfer (IGT) Payments
    CodeIGT,
    ///IH - Predominant Price High
    PredominantPriceHigh,
    ///II - Irregular Interest Payment Amount
    IrregularInterestPaymentAmount,
    ///IJ - Net Adjusted Monthly Rent
    NetAdjustedMonthlyRent,
    ///IK - Indicated Monthly Market Rent
    IndicatedMonthlyMarketRent,
    ///IL - Predominant Price Low
    PredominantPriceLow,
    ///IM - Adjustment for Rooms
    AdjustmentForRooms,
    ///IN - Installment
    Installment,
    ///INB - Installment Balance After the Current Installment is Applied
    InstallmentBalanceAfterTheCurrentInstallmentIsApplied,
    ///IO - Adjustment for Bedrooms
    AdjustmentForBedrooms,
    ///IP - Interest Payable During Repayment Period
    InterestPayableDuringRepaymentPeriod,
    ///IPD - Interest per Diem
    InterestPerDiem,
    ///IQ - Contingent Debt
    ContingentDebt,
    ///IR - Insurance Recovery
    InsuranceRecovery,
    ///IS - Independent Scholarship
    IndependentScholarship,
    ///ISS - Interest Since Claim Submission
    InterestSinceClaimSubmission,
    ///IT - Incentive Fee
    IncentiveFee,
    ///IU - Accrued Unpaid Interest Not To Be Capitalized
    AccruedUnpaidInterestNotToBeCapitalized,
    ///IV - Utilities Allowance
    UtilitiesAllowance,
    ///IVP - Investment Property
    InvestmentProperty,
    ///IW - Furniture Allowance
    FurnitureAllowance,
    ///IY - Debentures
    Debentures,
    ///IZ - Account High Balance
    AccountHighBalance,
    ///J - Trustee Fees
    TrusteeFees,
    ///J0 - Limited Partnership Capital
    LimitedPartnershipCapital,
    ///J1 - Current Face Amount
    CurrentFaceAmount,
    ///J2 - Original Face Amount
    OriginalFaceAmount,
    ///J3 - Fixed Default Note Holder's Amount
    FixedDefaultNoteHoldersAmount,
    ///J4 - Initial Monthly Payment
    InitialMonthlyPayment,
    ///J5 - Original Principal and Interest Payment
    OriginalPrincipalAndInterestPayment,
    ///J6 - Final Principal and Interest Payment
    FinalPrincipalAndInterestPayment,
    ///J7 - Conversion Fee
    ConversionFee,
    ///J8 - Ending Balance
    EndingBalance,
    ///J9 - Beginning Balance
    BeginningBalance,
    ///JA - Assessment
    Assessment,
    ///JB - Equity Claimed as Exempt
    EquityClaimedAsExempt,
    ///JC - Counter Claim
    CounterClaim,
    ///JD - Weekly Benefit
    WeeklyBenefit,
    ///JE - Lease
    Lease,
    ///JF - Administrative Load
    AdministrativeLoad,
    ///JG - Asset Cost Applicable to Entire Contract
    AssetCostApplicableToEntireContract,
    ///JH - Asset Cost Applicable to Portion of Contract
    AssetCostApplicableToPortionOfContract,
    ///JI - Annual Fee
    AnnualFee,
    ///JJ - Cost Basis
    CostBasis,
    ///JK - Disability Premium
    DisabilityPremium,
    ///JL - Employee Additional Contribution
    EmployeeAdditionalContribution,
    ///JM - Employee Match Contribution
    EmployeeMatchContribution,
    ///JN - Employer Contribution
    EmployerContribution,
    ///JO - Free Look Value
    FreeLookValue,
    ///JP - Free Withdrawal Value
    FreeWithdrawalValue,
    ///JQ - Front End Load
    FrontEndLoad,
    ///JR - Guaranteed Minimum Death Benefit
    GuaranteedMinimumDeathBenefit,
    ///JS - Interim Value
    InterimValue,
    ///JT - Monthly Rent
    MonthlyRent,
    ///JU - Judgment
    Judgment,
    ///JV - Loan Value
    LoanValue,
    ///JW - Market Value
    MarketValue,
    ///JX - Market Value Adjusted Value
    MarketValueAdjustedValue,
    ///JY - Market Value Adjustment
    MarketValueAdjustment,
    ///JZ - Net Contract Value
    NetContractValue,
    ///K - Attorney and Trustee Fees
    AttorneyAndTrusteeFees,
    ///K0 - Discounted Bills not Due
    DiscountedBillsNotDue,
    ///K1 - Unpaid Security Balance
    UnpaidSecurityBalance,
    ///K2 - Total Unpaid Security Balance
    TotalUnpaidSecurityBalance,
    ///K3 - Veterans Affairs Funding Fee
    VeteransAffairsFundingFee,
    ///K4 - Initial Target Fee
    InitialTargetFee,
    ///K5 - Minimum Fee
    MinimumFee,
    ///K6 - Maximum Fee
    MaximumFee,
    ///K7 - Price
    Price,
    ///K8 - Special Accounting Classification Reference Number (ACRN) Amount
    CodeK8,
    ///K9 - New Price
    NewPrice,
    ///KA - Estimated Contract
    EstimatedContract,
    ///KB - Estimated Net Adjustment
    EstimatedNetAdjustment,
    ///KC - Obligated
    Obligated,
    ///KD - Undefinitized
    Undefinitized,
    ///KE - Annual Revenue
    AnnualRevenue,
    ///KF - Net Paid Amount
    NetPaidAmount,
    ///KG - Net Collected Amount
    NetCollectedAmount,
    ///KH - Deduction Amount
    DeductionAmount,
    ///KI - Net Variance Amount
    NetVarianceAmount,
    ///KJ - Minimum Contract Amount
    MinimumContractAmount,
    ///KK - Item Gross Amount
    ItemGrossAmount,
    ///KL - Collected Amount
    CollectedAmount,
    ///KM - Disbursed Amount
    DisbursedAmount,
    ///KN - Gross Amount of Payment
    GrossAmountOfPayment,
    ///KO - Committed Amount
    CommittedAmount,
    ///KP - Principal and Interest
    PrincipalAndInterest,
    ///KQ - Incremental Order Amount
    IncrementalOrderAmount,
    ///KR - Liability-Long Term
    LiabilityLongTerm,
    ///KS - Taxes and Insurance
    TaxesAndInsurance,
    ///KT - Default Principal
    DefaultPrincipal,
    ///KU - Default Interest
    DefaultInterest,
    ///KV - Liability-Short Term
    LiabilityShortTerm,
    ///KW - Default Taxes and Insurance
    DefaultTaxesAndInsurance,
    ///KX - Miscellaneous Fee Collections
    MiscellaneousFeeCollections,
    ///KY - Not-To-Exceed Price
    NotToExceedPrice,
    ///KZ - Mortgagor's Monthly Obligations
    MortgagorsMonthlyObligations,
    ///L - Local
    Local,
    ///L0 - Liquid Assets
    LiquidAssets,
    ///L1 - Legal Contribution
    LegalContribution,
    ///L2 - Leasehold Insurance Amount
    LeaseholdInsuranceAmount,
    ///L3 - Total Unidentified Payments Rejected
    TotalUnidentifiedPaymentsRejected,
    ///L4 - Total Credits Received
    TotalCreditsReceived,
    ///L5 - Total Debits Received
    TotalDebitsReceived,
    ///L6 - Total Pre-advices Received
    TotalPreAdvicesReceived,
    ///L7 - Total Prenotes Received
    TotalPrenotesReceived,
    ///L8 - Total Post-advices Received
    TotalPostAdvicesReceived,
    ///L9 - Total Debit for Settlement
    TotalDebitForSettlement,
    ///LA - Definitized
    Definitized,
    ///LB - Definitized Total
    DefinitizedTotal,
    ///LC - Lessor's Cost
    LessorsCost,
    ///LD - Incremental
    Incremental,
    ///LDR - Land Rights
    LandRights,
    ///LE - Loan Eligibility Amount
    LoanEligibilityAmount,
    ///LF - Loan Remittance or Repayment
    LoanRemittanceOrRepayment,
    ///LG - Laundry Income
    LaundryIncome,
    ///LH - Baseline
    Baseline,
    ///LI - Line Item Unit Price
    LineItemUnitPrice,
    ///LJ - Legal and Audit
    LegalAndAudit,
    ///LK - Loan Amount Requested
    LoanAmountRequested,
    ///LL - Lump Sum
    LumpSum,
    ///LM - Limit
    Limit,
    ///LN - Lien Payoff
    LienPayoff,
    ///LO - Money Purchase
    MoneyPurchase,
    ///LOW - Lower Fund
    LowerFund,
    ///LP - List Price
    ListPrice,
    ///LPC - Total Subject Property Liens Paid by Closing
    TotalSubjectPropertyLiensPaidByClosing,
    ///LPF - Lease Purchase Funds
    LeasePurchaseFunds,
    ///LPY - Lease Payments
    LeasePayments,
    ///LQ - Maximum Potential Liability
    MaximumPotentialLiability,
    ///LR - Total Credit for Settlement
    TotalCreditForSettlement,
    ///LS - Net Settlement
    NetSettlement,
    ///LSP - Total Liabilities to be Paid at Closing Not Including Subject Property Liens
    TotalLiabilitiesToBePaidAtClosingNotIncludingSubjectPropertyLiens,
    ///LST - Loss on Sale Of Property
    LossOnSaleOfProperty,
    ///LT - Total Award
    TotalAward,
    ///LU - Option Amount
    OptionAmount,
    ///LV - Planned Periodic Payment
    PlannedPeriodicPayment,
    ///LW - Tax and Insurance Escrow Fund Balance
    TaxAndInsuranceEscrowFundBalance,
    ///LX - Loan Expense
    LoanExpense,
    ///LY - Total Remaining Principal Balance for the Issuer
    TotalRemainingPrincipalBalanceForTheIssuer,
    ///LZ - Delinquent Payment
    DelinquentPayment,
    ///M - Amount Due from Buyer at Appraisal Notice Date
    AmountDueFromBuyerAtAppraisalNoticeDate,
    ///M0 - Loans from Officers
    LoansFromOfficers,
    ///M1 - Maximum Out of Pocket Amount
    MaximumOutOfPocketAmount,
    ///M2 - Medical Contribution
    MedicalContribution,
    ///M3 - Tax rate expressed as a flat fee
    TaxRateExpressedAsAFlatFee,
    ///M4 - Minimum amount of tax to be paid
    MinimumAmountOfTaxToBePaid,
    ///M5 - Minimum amount to which tax rate is applied
    MinimumAmountToWhichTaxRateIsApplied,
    ///M6 - Maximum amount of tax to be paid
    MaximumAmountOfTaxToBePaid,
    ///M7 - Maximum amount to which tax rate is applied
    MaximumAmountToWhichTaxRateIsApplied,
    ///M8 - Markup Amount
    MarkupAmount,
    ///M9 - Net of Surrender Withdrawal
    NetOfSurrenderWithdrawal,
    ///MA - Maximum Amount
    MaximumAmount,
    ///MAD - Miscellaneous Adjustment
    MiscellaneousAdjustment,
    ///MB - Undistributed Budget
    UndistributedBudget,
    ///MC - Cost of Money
    CostOfMoney,
    ///MD - Minimum Due
    MinimumDue,
    ///ME - Minimum Default Note Holder's Cost
    MinimumDefaultNoteHoldersCost,
    ///MF - Administrative Fees
    AdministrativeFees,
    ///MG - Maximum Late Charge
    MaximumLateCharge,
    ///MH - Minimum Late Charge
    MinimumLateCharge,
    ///MI - Minimum Incentive Fee
    MinimumIncentiveFee,
    ///MJ - Maximum Default Note Holder's Cost
    MaximumDefaultNoteHoldersCost,
    ///MK - Gross to Pay
    GrossToPay,
    ///ML - Prior Net Invoice Total
    PriorNetInvoiceTotal,
    ///MM - Payout
    Payout,
    ///MMA - Modified Mortgage Amount
    ModifiedMortgageAmount,
    ///MN - Monthly Limit
    MonthlyLimit,
    ///MO - Minimum Order Value
    MinimumOrderValue,
    ///MP - Monthly Payment Amount
    MonthlyPaymentAmount,
    ///MQ - Post Tax Equity and Fiscal Responsibility Act (TEFRA) Cost Basis
    CodeMQ,
    ///MR - Management Reserve
    ManagementReserve,
    ///MS - Past-Due Taxes and Assessment Remaining Unpaid
    PastDueTaxesAndAssessmentRemainingUnpaid,
    ///MT - Pre Tax Equity and Fiscal Responsibility Act (TEFRA) Cost Basis
    CodeMT,
    ///MU - Premium Tax Paid on Surrender
    PremiumTaxPaidOnSurrender,
    ///MV - Premium Tax Paid up Front
    PremiumTaxPaidUpFront,
    ///MW - Sales Loads
    SalesLoads,
    ///MX - Maximum Incentive Fee
    MaximumIncentiveFee,
    ///MY - Surrender Value
    SurrenderValue,
    ///MZ - Valuation Price
    ValuationPrice,
    ///N - Net
    Net,
    ///N0 - Loans or Financial Borrowings
    LoansOrFinancialBorrowings,
    ///N1 - Net Worth
    NetWorth,
    ///N2 - Individual Income Taxes and Other
    IndividualIncomeTaxesAndOther,
    ///N3 - Corporate Income and Excess Profits Tax
    CorporateIncomeAndExcessProfitsTax,
    ///N4 - Excise Taxes
    ExciseTaxes,
    ///N5 - Estate and Gift Taxes
    EstateAndGiftTaxes,
    ///N6 - Carrier Tax Act Taxes
    CarrierTaxActTaxes,
    ///N7 - Federal Unemployment Tax Act Taxes
    FederalUnemploymentTaxActTaxes,
    ///N8 - Miscellaneous Taxes
    MiscellaneousTaxes,
    ///N9 - Withheld and Federal Insurance Contribution Act (FICA) Taxes
    CodeN9,
    ///NA - Net Adjustment
    NetAdjustment,
    ///NB - Net Compensation Position
    NetCompensationPosition,
    ///NBF - Net Benefit
    NetBenefit,
    ///NBO - Net Worth of Business Owned
    NetWorthOfBusinessOwned,
    ///NC - Negative Collected Balance
    NegativeCollectedBalance,
    ///NCT - Net Contribution
    NetContribution,
    ///ND - Per Person Monthly Limit
    PerPersonMonthlyLimit,
    ///NE - Net Billed
    NetBilled,
    ///NF - Monthly Net Fee Position (Excess/Deficit)
    CodeNF,
    ///NG - Medicare Copayment
    MedicareCopayment,
    ///NH - Medicare Deductible
    MedicareDeductible,
    ///NI - Medicare Paid
    MedicarePaid,
    ///NJ - Other Insurance Paid Amount
    OtherInsurancePaidAmount,
    ///NK - Total in Force and Applied Coverage
    TotalInForceAndAppliedCoverage,
    ///NL - Negative Ledger Balance
    NegativeLedgerBalance,
    ///NM - Non-collateralized Amount
    NonCollateralizedAmount,
    ///NN - Transaction Fee
    TransactionFee,
    ///NO - Non Commission Sales
    NonCommissionSales,
    ///NP - Net to Pay Total
    NetToPayTotal,
    ///NPP - No Tax Plan Payment
    NoTaxPlanPayment,
    ///NQ - Adjusted Nonrecurring
    AdjustedNonrecurring,
    ///NR - Nonrecurring
    Nonrecurring,
    ///NRE - Net Restoration Expenses
    NetRestorationExpenses,
    ///NS - Net Savings Amount
    NetSavingsAmount,
    ///NT - Unit Value
    UnitValue,
    ///NTA - New Technology Adjustment
    NewTechnologyAdjustment,
    ///NU - Reinsurance Amount
    ReinsuranceAmount,
    ///NV - Renewal Amount
    RenewalAmount,
    ///NW - Retention Per Life
    RetentionPerLife,
    ///NX - Retention Per Policy
    RetentionPerPolicy,
    ///NY - Net Year to Date (Excess/Deficit)
    CodeNY,
    ///NZ - Equalization Account
    EqualizationAccount,
    ///O - Court Cost
    CourtCost,
    ///O0 - Extraordinary Income
    ExtraordinaryIncome,
    ///O1 - Amount of First Mortgage Being Refinanced
    AmountOfFirstMortgageBeingRefinanced,
    ///O2 - Other Family Financing for Education
    OtherFamilyFinancingForEducation,
    ///O3 - Intangible Assets Written Off
    IntangibleAssetsWrittenOff,
    ///O4 - Interest Payable
    InterestPayable,
    ///O5 - Interest Receivable
    InterestReceivable,
    ///O6 - Joint Venture Results
    JointVentureResults,
    ///O7 - Long Term Debt
    LongTermDebt,
    ///O8 - Long Term Provisions
    LongTermProvisions,
    ///O9 - Loss
    Loss,
    ///OA - Principal Balance Amount
    PrincipalBalanceAmount,
    ///OB - Outstanding Loan Balance
    OutstandingLoanBalance,
    ///OC - Opening Bank Charges
    OpeningBankCharges,
    ///OD - Draft Amount
    DraftAmount,
    ///ODC - Odorization Charge
    OdorizationCharge,
    ///OE - Miscellaneous Charges
    MiscellaneousCharges,
    ///OEQ - Office Equipment
    OfficeEquipment,
    ///OF - Contractor's Offer
    ContractorsOffer,
    ///OFC - Operational Flow Order Charge
    OperationalFlowOrderCharge,
    ///OFR - Operational Flow Order Refund
    OperationalFlowOrderRefund,
    ///OG - Cable Charge
    CableCharge,
    ///OH - Handling Charges
    HandlingCharges,
    ///OI - Non-commission Charges
    NonCommissionCharges,
    ///OJ - Merchandise
    Merchandise,
    ///OK - Letter of Credit Amount
    LetterOfCreditAmount,
    ///OL - Outstanding Balance Other Lender
    OutstandingBalanceOtherLender,
    ///OLA - Other Liability Amounts
    OtherLiabilityAmounts,
    ///OM - Other Monthly Income
    OtherMonthlyIncome,
    ///ON - Negotiating Bank Charges
    NegotiatingBankCharges,
    ///ONR - Operational Notice Refund
    OperationalNoticeRefund,
    ///OO - Overdrafts
    Overdrafts,
    ///OP - Original Payment Total
    OriginalPaymentTotal,
    ///OQ - Payroll Costs
    PayrollCosts,
    ///OR - Letter of Credit Remaining Amount
    LetterOfCreditRemainingAmount,
    ///OS - Other Salaries
    OtherSalaries,
    ///OT - Commission Amendment Charges
    CommissionAmendmentCharges,
    ///OU - Profit
    Profit,
    ///OUT - Inpatient Outlier Adjustment
    InpatientOutlierAdjustment,
    ///OV - Profit and Loss Deficit
    ProfitAndLossDeficit,
    ///OW - Profit after Extraordinary Items and before Tax
    ProfitAfterExtraordinaryItemsAndBeforeTax,
    ///OX - Profit after Tax and Before Extraordinary Items
    ProfitAfterTaxAndBeforeExtraordinaryItems,
    ///OY - Payment Commission
    PaymentCommission,
    ///OZ - Profit Distributed to Employees
    ProfitDistributedToEmployees,
    ///P - Penalty
    Penalty,
    ///P0 - Parental Financing for Education
    ParentalFinancingForEducation,
    ///P1 - Partner's Calendar Year Salary
    PartnersCalendarYearSalary,
    ///P2 - Prior Plan Year Gross Salary
    PriorPlanYearGrossSalary,
    ///P3 - Premium Amount
    PremiumAmount,
    ///P4 - Prior Year's Wage
    PriorYearsWage,
    ///P5 - Partner's Tax Year Salary
    PartnersTaxYearSalary,
    ///P6 - Premium Due
    PremiumDue,
    ///P7 - Partner's K1 Tax Year Amount
    PartnersK1TaxYearAmount,
    ///P8 - Partner's K1 Calendar Year Amount
    PartnersK1CalendarYearAmount,
    ///P9 - Current Mortgage Principal Balance
    CurrentMortgagePrincipalBalance,
    ///PA - Payment Cancellation Total
    PaymentCancellationTotal,
    ///PAD - Policy Advance
    PolicyAdvance,
    ///PAM - Minimum Delivery Purchase Amount
    MinimumDeliveryPurchaseAmount,
    ///PAN - Penalty and Interest
    PenaltyAndInterest,
    ///PB - Billed Amount
    BilledAmount,
    ///PBG - Profit (Loss) Before Grants
    CodePBG,
    ///PC - Positive Collected Balance
    PositiveCollectedBalance,
    ///PCA - Processing Allowance
    ProcessingAllowance,
    ///PCC - Prior Contract Cost Basis
    PriorContractCostBasis,
    ///PCP - Previous Claim Payments
    PreviousClaimPayments,
    ///PCS - Prior Contract Surrender Charge
    PriorContractSurrenderCharge,
    ///PCV - Prior Contract Value
    PriorContractValue,
    ///PD - Credit
    Credit,
    ///PE - Plan Period Election
    PlanPeriodElection,
    ///PEX - Profit (Loss) after Extraordinary Items and Tax
    CodePEX,
    ///PF - Principal
    Principal,
    ///PFC - Port Facility Charge
    PortFacilityCharge,
    ///PG - Payoff
    Payoff,
    ///PGR - Proposed Gross Rent for the Subject Property
    ProposedGrossRentForTheSubjectProperty,
    ///PH - Per Occurrence Deductible
    PerOccurrenceDeductible,
    ///PI - Per Occurrence Monthly Limit
    PerOccurrenceMonthlyLimit,
    ///PJ - Past Due
    PastDue,
    ///PK - Photograph Fee
    PhotographFee,
    ///PL - Positive Ledger Balance
    PositiveLedgerBalance,
    ///PM - Last Premium Amount
    LastPremiumAmount,
    ///PN - Prior Gross Invoice Total
    PriorGrossInvoiceTotal,
    ///PO - Percent Override
    PercentOverride,
    ///PP - Payment Prior to Advance
    PaymentPriorToAdvance,
    ///PPN - Pending Net Sale Proceeds from Non-Real Estate Assets
    PendingNetSaleProceedsFromNonRealEstateAssets,
    ///PPR - Pending Net Sale Proceeds from Real Estate Assets
    PendingNetSaleProceedsFromRealEstateAssets,
    ///PQ - Advance Amount
    AdvanceAmount,
    ///PR - Per Occurrence Limit
    PerOccurrenceLimit,
    ///PRD - Property Damage
    PropertyDamage,
    ///PRL - Partial Payroll Payment
    PartialPayrollPayment,
    ///PS - Per Occurrence per Day Limit
    PerOccurrencePerDayLimit,
    ///PT - Per Occurrence Aggregate Limit
    PerOccurrenceAggregateLimit,
    ///PU - Unsecured Priority Claim
    UnsecuredPriorityClaim,
    ///PV - Prepetition Charges
    PrepetitionCharges,
    ///PW - Per Occurrence Maximum per Week Limit
    PerOccurrenceMaximumPerWeekLimit,
    ///PX - Per Person Maximum per Week Limit
    PerPersonMaximumPerWeekLimit,
    ///PY - Per Person per Day Limit
    PerPersonPerDayLimit,
    ///PZ - Original Principal Balance
    OriginalPrincipalBalance,
    ///Q - Amount Owed to Buyer at Appraisal Notice Date
    AmountOwedToBuyerAtAppraisalNoticeDate,
    ///Q0 - Loans to Affiliated Companies
    LoansToAffiliatedCompanies,
    ///Q1 - Proposed
    Proposed,
    ///Q2 - 1035 Exchange
    CodeQ2,
    ///Q3 - 401K Transfer
    CodeQ3,
    ///Q4 - Total Prenotes Accepted
    TotalPrenotesAccepted,
    ///Q5 - Total Prenotes Rejected
    TotalPrenotesRejected,
    ///Q6 - Automatic Premium Deduction
    AutomaticPremiumDeduction,
    ///Q7 - Total Post-advices Accepted
    TotalPostAdvicesAccepted,
    ///Q8 - Total Post-advices Rejected
    TotalPostAdvicesRejected,
    ///Q9 - Cash With Application
    CashWithApplication,
    ///QA - Combined
    Combined,
    ///QB - Credit Card
    CreditCard,
    ///QC - Deposit Fund
    DepositFund,
    ///QD - Direct Billing
    DirectBilling,
    ///QE - Disc Premium
    DiscPremium,
    ///QF - Electronic Funds Transfer (EFT)
    CodeQF,
    ///QG - Government Allotment
    GovernmentAllotment,
    ///QH - Initial Premium
    InitialPremium,
    ///QI - Individual Retirement Account 60 Day Rollover
    IndividualRetirementAccount60DayRollover,
    ///QJ - Individual Retirement Account Direct Transfer
    IndividualRetirementAccountDirectTransfer,
    ///QK - Individual Retirement Account Regular Contribution
    IndividualRetirementAccountRegularContribution,
    ///QL - Keogh/HR 10
    KeoghHr10,
    ///QM - Keogh/HR 10 Transfer
    KeoghHr10Transfer,
    ///QN - Quarterly Net Fee Position (Excess/Deficit)
    CodeQN,
    ///QO - List Billing
    ListBilling,
    ///QP - Modal Premium
    ModalPremium,
    ///QQ - Payroll Taxes
    PayrollTaxes,
    ///QR - Parking Income
    ParkingIncome,
    ///QS - Non-Qualified (1035 Exchange)
    CodeQS,
    ///QT - PAC - Pre-Authorized Check
    PacPreAuthorizedCheck,
    ///QU - Payroll Deduction
    PayrollDeduction,
    ///QV - Pension
    Pension,
    ///QW - Premium Received With Application
    PremiumReceivedWithApplication,
    ///QX - Profit Sharing Trust
    ProfitSharingTrust,
    ///QY - Qualified
    Qualified,
    ///QZ - Payment Amount
    PaymentAmount,
    ///R - Spend Down
    SpendDown,
    ///R0 - Loans to Participants
    LoansToParticipants,
    ///R1 - Fixed, Liquidated Secured Debt
    CodeR1,
    ///R2 - Contingent Secured Debt
    ContingentSecuredDebt,
    ///R3 - Disputed Secured Debt
    DisputedSecuredDebt,
    ///R4 - Unliquidated Secured Debt
    UnliquidatedSecuredDebt,
    ///R5 - Fixed, Liquidated Unsecured Debt
    CodeR5,
    ///R6 - Contingent Unsecured Debt
    ContingentUnsecuredDebt,
    ///R7 - Disputed Unsecured Debt
    DisputedUnsecuredDebt,
    ///R8 - Unliquidated Unsecured Debt
    UnliquidatedUnsecuredDebt,
    ///R9 - At Time of Filing
    AtTimeOfFiling,
    ///RA - Accelerated Royalty
    AcceleratedRoyalty,
    ///RB - Per Person Deductible
    PerPersonDeductible,
    ///RC - Refund Check
    RefundCheck,
    ///RD - Per Person Limit
    PerPersonLimit,
    ///RDS - Reservation/Demand - Storage
    ReservationDemandStorage,
    ///RDT - Reservation/Demand - Transportation
    ReservationDemandTransportation,
    ///RE - Royalty Due
    RoyaltyDue,
    ///RET - Deposit Value
    DepositValue,
    ///RF - Restitution
    Restitution,
    ///RG - Budgeted Redemption
    BudgetedRedemption,
    ///RH - Per Person Aggregate Limit
    PerPersonAggregateLimit,
    ///RI - Residual Value
    ResidualValue,
    ///RJ - Rate Amount
    RateAmount,
    ///RK - Provision for Long Term Depreciation
    ProvisionForLongTermDepreciation,
    ///RL - Regular Remittance
    RegularRemittance,
    ///RM - Remittance Refund
    RemittanceRefund,
    ///RN - Resident Manager's Salary
    ResidentManagersSalary,
    ///RO - Provisions
    Provisions,
    ///RP - Repair
    Repair,
    ///RPC - Repackaging Cost
    RepackagingCost,
    ///RQ - Recommended Amount
    RecommendedAmount,
    ///RR - Reserve Requirement Amount
    ReserveRequirementAmount,
    ///RS - Reserves
    Reserves,
    ///RT - Last Payment
    LastPayment,
    ///RU - Total Debits Rejected
    TotalDebitsRejected,
    ///RV - Total Payments Rejected
    TotalPaymentsRejected,
    ///RW - Total Delinquency
    TotalDelinquency,
    ///RX - Total Pre-advices Accepted
    TotalPreAdvicesAccepted,
    ///RY - Total Pre-advices Rejected
    TotalPreAdvicesRejected,
    ///RZ - Lender's Total Delinquency
    LendersTotalDelinquency,
    ///S - Submitted Chargeback Claim Amount
    SubmittedChargebackClaimAmount,
    ///S0 - Self-Financing for Education
    SelfFinancingForEducation,
    ///S1 - Salary Amount
    SalaryAmount,
    ///S2 - Salary with Bonuses
    SalaryWithBonuses,
    ///S3 - Salary with Commissions
    SalaryWithCommissions,
    ///S4 - Salary with Subchapter S Corporation Income
    SalaryWithSubchapterSCorporationIncome,
    ///S5 - Salary with Partner's Bonuses
    SalaryWithPartnersBonuses,
    ///S6 - Subchapter S Corporation
    SubchapterSCorporation,
    ///S7 - Sole Proprietorship
    SoleProprietorship,
    ///S8 - Period Rental
    PeriodRental,
    ///S9 - Secured Claim Allowed
    SecuredClaimAllowed,
    ///SA - Campaign Summary Amount
    CampaignSummaryAmount,
    ///SAA - System Adjusted Amount
    SystemAdjustedAmount,
    ///SAG - Surplus (Deficit) after Grant
    CodeSAG,
    ///SAL - Salvage
    Salvage,
    ///SB - Stated Amount
    StatedAmount,
    ///SC - Total Service Charge
    TotalServiceCharge,
    ///SCT - State Care Tax
    StateCareTax,
    ///SD - Sales Charge
    SalesCharge,
    ///SE - Service Charges Which Cannot Be Compensated by Balances
    ServiceChargesWhichCannotBeCompensatedByBalances,
    ///SF - Scholarship from Admitting Educational Institution
    ScholarshipFromAdmittingEducationalInstitution,
    ///SFD - Secured Borrowed Funds Not Deposited
    SecuredBorrowedFundsNotDeposited,
    ///SG - Sponsor-Financing for Education
    SponsorFinancingForEducation,
    ///SH - Surrender Charge
    SurrenderCharge,
    ///SI - Subsequent Interest Payment Amount
    SubsequentInterestPaymentAmount,
    ///SJ - Surrender Full
    SurrenderFull,
    ///SK - Surrender Partial
    SurrenderPartial,
    ///SL - Security Personnel's Salary
    SecurityPersonnelsSalary,
    ///SM - Supplemental
    Supplemental,
    ///SM1 - Insurance Value
    InsuranceValue,
    ///SM2 - Declared Value
    DeclaredValue,
    ///SM3 - Shipment Value
    ShipmentValue,
    ///SM4 - Pay on Delivery
    PayOnDelivery,
    ///SM5 - Landed Cost Value
    LandedCostValue,
    ///SN - Sales Administration Expense
    SalesAdministrationExpense,
    ///SO - Special Creditors Amount
    SpecialCreditorsAmount,
    ///SOF - Setoff
    Setoff,
    ///SP - Sales Price
    SalesPrice,
    ///SQ - Special Debtors Amount
    SpecialDebtorsAmount,
    ///SR - Secured Claim
    SecuredClaim,
    ///SRD - Settlement Refund as approved by the Federal Energy Regulatory Commission
    SettlementRefundAsApprovedByTheFederalEnergyRegulatoryCommission,
    ///SS - Campaign Summary Amount to be Shared
    CampaignSummaryAmountToBeShared,
    ///SSC - Shares in Subsidiary Companies
    SharesInSubsidiaryCompanies,
    ///ST - State
    State,
    ///SU - Surcharge
    Surcharge,
    ///SV - Fixed Monthly Principal Payment
    FixedMonthlyPrincipalPayment,
    ///SW - Base Award Fee
    BaseAwardFee,
    ///SX - Severance Tax
    SeveranceTax,
    ///SY - Initial Buydown Balance
    InitialBuydownBalance,
    ///SZ - Certification Fee
    CertificationFee,
    ///T - Tax
    Tax,
    ///T0 - Third-Party Government-Financing for Education
    ThirdPartyGovernmentFinancingForEducation,
    ///T1 - Teacher
    Teacher,
    ///T2 - Total Claim Before Taxes
    TotalClaimBeforeTaxes,
    ///T3 - Total Submitted Charges
    TotalSubmittedCharges,
    ///T4 - Total Current Balance
    TotalCurrentBalance,
    ///T5 - Total Claims
    TotalClaims,
    ///T6 - Claim
    Claim,
    ///T7 - Total Credits Accepted
    TotalCreditsAccepted,
    ///T8 - Total Credits Rejected
    TotalCreditsRejected,
    ///T9 - Total Debits Accepted
    TotalDebitsAccepted,
    ///TA - Total Annual Sales
    TotalAnnualSales,
    ///TB - Total Annual Sales to Customer
    TotalAnnualSalesToCustomer,
    ///TBC - Total Buyer Closing Costs
    TotalBuyerClosingCosts,
    ///TC - Proposed Cost
    ProposedCost,
    ///TCS - Total Commission from Primary and Secondary Sources
    TotalCommissionFromPrimaryAndSecondarySources,
    ///TD - Proposed Profit
    ProposedProfit,
    ///TDA - Total Depository Accounts
    TotalDepositoryAccounts,
    ///TE - Proposed Fee
    ProposedFee,
    ///TEN - Total Maintenance Expense on All Non-Income Producing Properties
    TotalMaintenanceExpenseOnAllNonIncomeProducingProperties,
    ///TEP - Total Maintenance Expense on All Income Producing Properties
    TotalMaintenanceExpenseOnAllIncomeProducingProperties,
    ///TF - Total Proposed Price
    TotalProposedPrice,
    ///TG - Alternate Proposed Price
    AlternateProposedPrice,
    ///TGD - Total Gifts Not Deposited
    TotalGiftsNotDeposited,
    ///TH - Total Claim Allowed
    TotalClaimAllowed,
    ///TI - Title Insurance Amount on Loan
    TitleInsuranceAmountOnLoan,
    ///TIS - Total Self-employed Income from Primary and Secondary Sources
    TotalSelfEmployedIncomeFromPrimaryAndSecondarySources,
    ///TJ - Time and Expense Paid to Date
    TimeAndExpensePaidToDate,
    ///TK - Total Amount of Contract
    TotalAmountOfContract,
    ///TL - Total Prior Loan Amount Owed
    TotalPriorLoanAmountOwed,
    ///TLA - Total Other Liquid Assets
    TotalOtherLiquidAssets,
    ///TLV - Total Life Insurance Net Cash Value
    TotalLifeInsuranceNetCashValue,
    ///TM - Time and Expense Paid Current Month
    TimeAndExpensePaidCurrentMonth,
    ///TMM - Total Monetary Markup Amount
    TotalMonetaryMarkupAmount,
    ///TO - Telephone Operator's Salary
    TelephoneOperatorsSalary,
    ///TOL - Total Omitted Liabilities
    TotalOmittedLiabilities,
    ///TP - Total payment amount
    TotalPaymentAmount,
    ///TPA - Total Previous Adjusted Claim
    TotalPreviousAdjustedClaim,
    ///TPR - Total Net Proceeds from Real Estate Assets
    TotalNetProceedsFromRealEstateAssets,
    ///TPS - Total Paid as Submitted
    TotalPaidAsSubmitted,
    ///TQ - Subsidies for Operating Costs
    SubsidiesForOperatingCosts,
    ///TR - Target Cost
    TargetCost,
    ///TRF - Total Retirement Funds
    TotalRetirementFunds,
    ///TRI - Total Non-rental Income
    TotalNonRentalIncome,
    ///TRL - Total Resubordinated Liabilities
    TotalResubordinatedLiabilities,
    ///TRP - Total Liabilities for Rental Properties
    TotalLiabilitiesForRentalProperties,
    ///TS - Total Sales
    TotalSales,
    ///TSB - Total Stocks and Bonds
    TotalStocksAndBonds,
    ///TT - Total Transaction Amount
    TotalTransactionAmount,
    ///TU - Transportation Cost per Unit of Measure
    TransportationCostPerUnitOfMeasure,
    ///TV - Level of Premium Insurance - Retention
    LevelOfPremiumInsuranceRetention,
    ///TW - Technicians Indemnity Provision
    TechniciansIndemnityProvision,
    ///TX - Total to Date
    TotalToDate,
    ///TY - Total at Complete
    TotalAtComplete,
    ///TZ - Transportation Cost Total
    TransportationCostTotal,
    ///U - Underpayment
    Underpayment,
    ///U0 - U.S. Government-Financing for Education
    USGovernmentFinancingForEducation,
    ///U1 - Unsecured, Priority Claim Allowed
    CodeU1,
    ///U2 - Ingredient Cost Claimed
    IngredientCostClaimed,
    ///U3 - Miscellaneous Expenses
    MiscellaneousExpenses,
    ///U4 - Present Value of Lot
    PresentValueOfLot,
    ///U5 - Cost of Improvements
    CostOfImprovements,
    ///U6 - Alterations, Improvements, Repairs
    CodeU6,
    ///U7 - Land
    Land,
    ///U8 - Refinance
    Refinance,
    ///U8L - Tax Payment Refinanced by Same Lender
    TaxPaymentRefinancedBySameLender,
    ///U9 - Estimated Prepaid Items
    EstimatedPrepaidItems,
    ///UA - Unliquidated Amount
    UnliquidatedAmount,
    ///UAA - User Adjusted Amount
    UserAdjustedAmount,
    ///UAR - Refund of Unauthorized Overrun Charges (UAOR Refund)
    CodeUAR,
    ///UB - Unpaid Principal Balance
    UnpaidPrincipalBalance,
    ///UC - Unspecified Aggregate Limit
    UnspecifiedAggregateLimit,
    ///UD - Unsecured, Nonpriority Claim Allowed
    CodeUD,
    ///UE - Mortgage Insurance
    MortgageInsurance,
    ///UEP - Updated Expenses on Presale
    UpdatedExpensesOnPresale,
    ///UF - Discount (If Borrower Paid)
    CodeUF,
    ///UFD - Unsecured Borrowed Funds Not Deposited
    UnsecuredBorrowedFundsNotDeposited,
    ///UG - Total Unpaid Principal Balance for Stafford Loans
    TotalUnpaidPrincipalBalanceForStaffordLoans,
    ///UH - Subordinate Financing
    SubordinateFinancing,
    ///UHI - Unsecured Home Improvements
    UnsecuredHomeImprovements,
    ///UI - Total Costs
    TotalCosts,
    ///UIP - Updated Interest on Presale
    UpdatedInterestOnPresale,
    ///UJ - Other Credits
    OtherCredits,
    ///UK - Base Loan Amount (w/o financed Mortgage Insurance)
    CodeUK,
    ///UL - Mortgage Insurance Financed
    MortgageInsuranceFinanced,
    ///UM - Total Loan Amount
    TotalLoanAmount,
    ///UN - Unsecured, Nonpriority Claim
    CodeUN,
    ///UNK - Unknown Tax Plan Payment
    UnknownTaxPlanPayment,
    ///UO - Cash from or to Borrower
    CashFromOrToBorrower,
    ///UP - Total Unpaid Principal Balance for Parental Loans for Students
    TotalUnpaidPrincipalBalanceForParentalLoansForStudents,
    ///UPF - Upper Fund
    UpperFund,
    ///UQ - Monthly Income
    MonthlyIncome,
    ///UR - Unearned Income
    UnearnedIncome,
    ///US - Total Unpaid Principal Balance for Supplemental Loans for Students
    TotalUnpaidPrincipalBalanceForSupplementalLoansForStudents,
    ///UT - Value Added Sales
    ValueAddedSales,
    ///UU - Clearing House Settlement
    ClearingHouseSettlement,
    ///UV - Drawback
    Drawback,
    ///UW - Total Monthly Liabilities
    TotalMonthlyLiabilities,
    ///UX - Utilities, Furniture, and Amenities Included in Rent
    CodeUX,
    ///UY - Total Assets
    TotalAssets,
    ///UZ - Total Liquid Assets
    TotalLiquidAssets,
    ///V - Cost of Deficiency
    CostOfDeficiency,
    ///V0 - Value Added
    ValueAdded,
    ///V1 - Tax and Insurance Escrow Fund
    TaxAndInsuranceEscrowFund,
    ///V2 - Interest Due to Investor
    InterestDueToInvestor,
    ///V3 - Total Principal Due to the Investor
    TotalPrincipalDueToTheInvestor,
    ///V4 - Total Interest Due to the Investor
    TotalInterestDueToTheInvestor,
    ///V5 - Total Curtailment Due to the Investor
    TotalCurtailmentDueToTheInvestor,
    ///V6 - Total Principal Payoff and Repurchase Due to the Investor
    TotalPrincipalPayoffAndRepurchaseDueToTheInvestor,
    ///V7 - Total Interest Payoff and Repurchase Due to the Investor
    TotalInterestPayoffAndRepurchaseDueToTheInvestor,
    ///V8 - Actual Outstanding Principal Balance
    ActualOutstandingPrincipalBalance,
    ///V9 - Face Amount
    FaceAmount,
    ///VA - Total Current Rent or Mortgage Payment (Issue)
    CodeVA,
    ///VB - Total Non-liquid Assets
    TotalNonLiquidAssets,
    ///VC - Authorized
    Authorized,
    ///VD - Actual Person Day Rate
    ActualPersonDayRate,
    ///VE - Estimated Person Day Rate
    EstimatedPersonDayRate,
    ///VES - Vested/Earned Upper Fund
    VestedEarnedUpperFund,
    ///VF - Total Monthly Expenses
    TotalMonthlyExpenses,
    ///VG - Current Monthly Principal and Interest
    CurrentMonthlyPrincipalAndInterest,
    ///VH - Levy Amount
    LevyAmount,
    ///VI - Current Support
    CurrentSupport,
    ///VJ - Past Due Support
    PastDueSupport,
    ///VK - Medical Support
    MedicalSupport,
    ///VL - Net Negative Amortization Amount
    NetNegativeAmortizationAmount,
    ///VM - Withhold From Wages
    WithholdFromWages,
    ///VN - Commission Basis
    CommissionBasis,
    ///VO - Commission Earned
    CommissionEarned,
    ///VP - Current Monthly Payment
    CurrentMonthlyPayment,
    ///VQ - Commission Netted
    CommissionNetted,
    ///VR - Total Monthly Debt
    TotalMonthlyDebt,
    ///VRS - Volumetric Reservation
    VolumetricReservation,
    ///VS - Other Financing Payment
    OtherFinancingPayment,
    ///VSI - Value of Securities at Issue Date
    ValueOfSecuritiesAtIssueDate,
    ///VSM - Value of Securities at Maturity
    ValueOfSecuritiesAtMaturity,
    ///VT - Current Value
    CurrentValue,
    ///VU - Closing Cost
    ClosingCost,
    ///VV - Capitalized Mortgage Amount
    CapitalizedMortgageAmount,
    ///VW - First Mortgage Monthly Principal and Interest
    FirstMortgageMonthlyPrincipalAndInterest,
    ///VX - Interest Amount Paid to Date
    InterestAmountPaidToDate,
    ///VY - Minimum Transfer
    MinimumTransfer,
    ///VZ - Maximum Transfer
    MaximumTransfer,
    ///W - Deficiency Judgment Fees
    DeficiencyJudgmentFees,
    ///W0 - Trade Debtors
    TradeDebtors,
    ///W1 - W-2
    W2,
    ///W2 - W-2 with Bonuses
    W2WithBonuses,
    ///W3 - W-2 with Deferred Compensation
    W2WithDeferredCompensation,
    ///W4 - W-2 without Bonuses
    W2WithoutBonuses,
    ///W5 - Deposit Sub Total
    DepositSubTotal,
    ///W6 - Direct Rollover
    DirectRollover,
    ///W7 - Direct Transfer
    DirectTransfer,
    ///W8 - Discounted
    Discounted,
    ///W9 - Secondary Finance
    SecondaryFinance,
    ///WA - Minimum Deposit
    MinimumDeposit,
    ///WB - Sub-Agency Compensation
    SubAgencyCompensation,
    ///WC - Buyers Agency Compensation
    BuyersAgencyCompensation,
    ///WD - Variable Rate Compensation
    VariableRateCompensation,
    ///WE - Compensation Bonus on Sale of Property
    CompensationBonusOnSaleOfProperty,
    ///WF - Veterans Affairs Loan Guarantee
    VeteransAffairsLoanGuarantee,
    ///WG - Security Trade Amount
    SecurityTradeAmount,
    ///WH - Balance Owing All Other Liens, Subject Property
    CodeWH,
    ///WI - Other Financing
    OtherFinancing,
    ///WJ - Dual Agency Compensation
    DualAgencyCompensation,
    ///WK - Per Week Limit
    PerWeekLimit,
    ///WL - Lender's Opinion of Value
    LendersOpinionOfValue,
    ///WM - Total Original Principal Balance
    TotalOriginalPrincipalBalance,
    ///WN - Other Agent Compensation
    OtherAgentCompensation,
    ///WO - Dock Usage Fee
    DockUsageFee,
    ///WP - Pool Usage Fee
    PoolUsageFee,
    ///WQ - Clubhouse Fee
    ClubhouseFee,
    ///WR - Optional Service Fee
    OptionalServiceFee,
    ///WS - Other Association Fees
    OtherAssociationFees,
    ///WT - Principal, Interest, Taxes
    CodeWT,
    ///WU - Principal, Interest, Taxes and Insurance
    CodeWU,
    ///WV - Total Points Paid at Closing
    TotalPointsPaidAtClosing,
    ///WW - Amount that Would Have Been Paid in the Absence of Capitation
    AmountThatWouldHaveBeenPaidInTheAbsenceOfCapitation,
    ///WX - Points Paid by Seller
    PointsPaidBySeller,
    ///WY - Loan Withdrawal
    LoanWithdrawal,
    ///WZ - Severance Pay
    SeverancePay,
    ///X - Deficiency Judgment Expenses and Fees
    DeficiencyJudgmentExpensesAndFees,
    ///X0 - Treble Damages
    TrebleDamages,
    ///X1 - Transfer to Untaxed Reserves
    TransferToUntaxedReserves,
    ///X2 - Reissued
    Reissued,
    ///X3 - Rollover Amount
    RolloverAmount,
    ///X4 - Annual Rental
    AnnualRental,
    ///X5 - Gross Monthly Rent
    GrossMonthlyRent,
    ///X6 - SEP - Self Employee Pension
    SepSelfEmployeePension,
    ///X8 - Funding Amount
    FundingAmount,
    ///X9 - Tax Sheltered Annuity (403B Transfer)
    CodeX9,
    ///XA - Maximum Award Fee
    MaximumAwardFee,
    ///XB - Maturity Value
    MaturityValue,
    ///XC - Earned Wages
    EarnedWages,
    ///XD - Base Period Wage
    BasePeriodWage,
    ///XE - Withdrawal
    Withdrawal,
    ///XF - Withdrawal Inception to Date
    WithdrawalInceptionToDate,
    ///XG - Withdrawal Less Market Value Adjustment
    WithdrawalLessMarketValueAdjustment,
    ///XH - Withdrawal Less Surrender
    WithdrawalLessSurrender,
    ///XI - Withdrawal Less Taxes
    WithdrawalLessTaxes,
    ///XJ - Withdrawal Year to Date
    WithdrawalYearToDate,
    ///XK - Unavailable Reserves
    UnavailableReserves,
    ///XL - Uncalled Capital
    UncalledCapital,
    ///XM - Unemployment Contribution
    UnemploymentContribution,
    ///XN - Unlimited Capital, Minimum Fixed
    CodeXN,
    ///XO - Unpaid Capital
    UnpaidCapital,
    ///XP - Unsecured Liabilities
    UnsecuredLiabilities,
    ///XQ - Value Added Tax
    ValueAddedTax,
    ///XR - Value of Shares
    ValueOfShares,
    ///XS - Vehicles
    Vehicles,
    ///XT - Voluntary Reserves
    VoluntaryReserves,
    ///XTR - Extraction
    Extraction,
    ///XU - Wages
    Wages,
    ///XV - Withholding
    Withholding,
    ///XW - Original Value
    OriginalValue,
    ///XX - Working Capital
    WorkingCapital,
    ///XY - Sales Price Per Dwelling Unit
    SalesPricePerDwellingUnit,
    ///XZ - Sales Price Per Room
    SalesPricePerRoom,
    ///Y - Current List Price
    CurrentListPrice,
    ///Y0 - Self Insurance Amount
    SelfInsuranceAmount,
    ///Y1 - Year to Date Eligible Salary
    YearToDateEligibleSalary,
    ///Y2 - Total Real Estate Owned
    TotalRealEstateOwned,
    ///Y3 - Total Liabilities
    TotalLiabilities,
    ///Y4 - Total Liability Monthly Payments
    TotalLiabilityMonthlyPayments,
    ///Y5 - Total Real Estate Owned Market Value
    TotalRealEstateOwnedMarketValue,
    ///Y6 - Total Real Estate Owned Gross Rental Income
    TotalRealEstateOwnedGrossRentalIncome,
    ///Y7 - Total Real Estate Owned Mortgages and Liens
    TotalRealEstateOwnedMortgagesAndLiens,
    ///Y8 - Total Real Estate Owned Mortgage Payments
    TotalRealEstateOwnedMortgagePayments,
    ///Y9 - Total Real Estate Owned Miscellaneous Expenses
    TotalRealEstateOwnedMiscellaneousExpenses,
    ///YA - Total Real Estate Owned Net Rental Income
    TotalRealEstateOwnedNetRentalIncome,
    ///YB - Actual Unpaid Principal Balance
    ActualUnpaidPrincipalBalance,
    ///YC - Scheduled Unpaid Principal Balance
    ScheduledUnpaidPrincipalBalance,
    ///YD - Principal Due to Investor
    PrincipalDueToInvestor,
    ///YE - Constant Principal and Interest
    ConstantPrincipalAndInterest,
    ///YF - Other Fee Collection
    OtherFeeCollection,
    ///YG - Beginning Scheduled Unpaid Principal Balance
    BeginningScheduledUnpaidPrincipalBalance,
    ///YH - Tax and Insurance Principal Balance
    TaxAndInsurancePrincipalBalance,
    ///YI - New Principal and Interest
    NewPrincipalAndInterest,
    ///YJ - Curtailment
    Curtailment,
    ///YK - Prepayment Penalty
    PrepaymentPenalty,
    ///YL - Partial Annuitization
    PartialAnnuitization,
    ///YM - Partial Withdrawal
    PartialWithdrawal,
    ///YN - Post Tax Equity and Fiscal Responsibility Act (TEFRA) Gain
    CodeYN,
    ///YO - Pre Tax Equity and Fiscal Responsibility Act (TEFRA) Gain
    CodeYO,
    ///YP - Policy Amount
    PolicyAmount,
    ///YQ - Payments in Advance
    PaymentsInAdvance,
    ///YR - Payments in Arrears
    PaymentsInArrears,
    ///YS - Cancelled
    Cancelled,
    ///YT - Denied
    Denied,
    ///YU - In Process
    InProcess,
    ///YV - Requested
    Requested,
    ///YW - Paid
    Paid,
    ///YX - Paid for This Facility
    PaidForThisFacility,
    ///YY - Returned
    Returned,
    ///YZ - Total Aggregate Limit
    TotalAggregateLimit,
    ///Z - List Price When Sold
    ListPriceWhenSold,
    ///Z0 - Insertion Cost
    InsertionCost,
    ///Z1 - Repackaging Labor Cost
    RepackagingLaborCost,
    ///Z2 - Repackaging Material Cost
    RepackagingMaterialCost,
    ///Z3 - Unit Cost of Discrepant Material
    UnitCostOfDiscrepantMaterial,
    ///Z4 - Liquidation Principal
    LiquidationPrincipal,
    ///Z5 - Remaining Pool Balance
    RemainingPoolBalance,
    ///Z6 - Remaining Security Balance
    RemainingSecurityBalance,
    ///Z7 - Program Cost
    ProgramCost,
    ///Z8 - Override to Handling Fee
    OverrideToHandlingFee,
    ///Z9 - Production Cost
    ProductionCost,
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
    ///ZI - Federal Pension Mandate - Category 4
    FederalPensionMandateCategory4,
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
    ///ZP - Coupon Face Value
    CouponFaceValue,
    ///ZQ - Initial Target Cost
    InitialTargetCost,
    ///ZR - Increase
    Increase,
    ///ZS - Decrease
    Decrease,
    ///ZT - Prorated Amount
    ProratedAmount,
    ///ZU - Loan Charge
    LoanCharge,
    ///ZV - Mortgage Recording Fee
    MortgageRecordingFee,
    ///ZW - Deed Recording Fee
    DeedRecordingFee,
    ///ZX - Release Recording Fee
    ReleaseRecordingFee,
    ///ZY - Assumption
    Assumption,
    ///ZZ - Mutually Defined
    MutuallyDefined,
}
impl AmountQualifierCode {
    pub fn code(&self) -> &str {
        {
            use AmountQualifierCode::*;
            match self {
                DownpaymentOnTheRepaymentPlanAmount => "00",
                InterestSubstitutionAdjustment => "0A",
                PrincipalSubstitutionAdjustment => "0B",
                PrepaidInterest => "0C",
                PrepaidPrincipal => "0D",
                DelinquentInterest => "0E",
                DelinquentPrincipal => "0F",
                CurtailmentAdjustment => "0G",
                SerialNotePrincipalAvailableForDistribution => "0H",
                ServicingFee => "0I",
                GuaranteeFeeAdjustment => "0J",
                AmountUnderCollateralized => "0K",
                AmountOverCollateralized => "0L",
                TrialBalanceAdjustment => "0M",
                CustodialBankAccountAdjustment => "0N",
                Item => "0P",
                Schedule => "0Q",
                RegularPlan => "0R",
                PreviouslyBilled => "0S",
                CurrentlyDue => "0T",
                CoveragePremium => "0U",
                LineItemTotal => "1",
                FixedInstallmentControlAccountingError => "01",
                Variance => "1A",
                VarianceAdjustmentCost => "1B",
                VarianceAdjustmentSchedule => "1C",
                Escalation => "1D",
                FixedPrice => "1E",
                Lodging => "1F",
                Meals => "1G",
                TravelExpense => "1H",
                InsuranceExpense => "1J",
                UnionDues => "1K",
                RegularIncome => "1L",
                IncomeOnRealProperty => "1M",
                IncomeFromSocialSecurityAndOrOtherGovernmentAssistance => "1N",
                TotalMonthlyIncome => "1P",
                ElectricAndOrFuelPayment => "1Q",
                WaterAndOrSewerPayment => "1R",
                TelephonePayment => "1S",
                MaintenanceExpense => "1T",
                FoodExpense => "1U",
                ClothingExpense => "1V",
                LaundryExpense => "1W",
                MedicalAndOrDentalExpense => "1X",
                RecreationExpenses => "1Y",
                CharitableContributions => "1Z",
                BatchTotal => "2",
                GraduatedPaymentMortgageAdjustment => "02",
                HomeInsuranceExpense => "2A",
                LifeInsuranceExpense => "2B",
                HealthInsuranceExpense => "2C",
                AutomobileInsuranceExpense => "2D",
                ValueOfPropertyClaimedAsExempt => "2E",
                AutomobilePayment => "2F",
                OtherTypeOfInstallmentPayment => "2G",
                OperatingExpenses => "2H",
                TotalProjectedMonthlyIncome => "2I",
                TotalProjectedMonthlyExpenses => "2J",
                ExcessIncome => "2K",
                ValueOfPersonalProperty => "2L",
                MonthlyOvertime => "2M",
                TotalAllRepairs => "2N",
                TotalRecommendedRepairs => "2P",
                StateQuarterlyTotalGrossWages => "2Q",
                Code2R => "2R",
                Code2S => "2S",
                Code2T => "2T",
                StateQuarterlyDisabilityInsuranceTaxableWages => "2U",
                StateQuarterlyTipWages => "2V",
                AssetLongTerm => "2W",
                AssetShortTerm => "2X",
                BaseCoverage => "2Y",
                CommissionRetained => "2Z",
                DepositTotal => "3",
                GrowingEquityMortgageAdjustment => "03",
                Accounting => "3A",
                AccountsPayable => "3B",
                AccountsReceivable => "3C",
                AdvancedDividends => "3D",
                AdvertisingExpenses => "3E",
                Amortization => "3F",
                AmortizationCosts => "3G",
                AmountOfDecree => "3H",
                AssetInvestment => "3I",
                AuthorizedCapital => "3J",
                AvailableReserves => "3K",
                BadDebtAllowance => "3L",
                BadDebts => "3M",
                Code3N => "3N",
                LongTermAssets => "3O",
                LongTermLiabilities => "3P",
                LongTermTangibleAssets => "3Q",
                LossesOnCapital => "3R",
                MachinesAndTools => "3S",
                MemberRiskCapital => "3T",
                MiscellaneousAfterTaxExempt => "3U",
                Mortgage => "3V",
                NominalCapital => "3W",
                NominalDamages => "3X",
                NonOperationalFixedAssets => "3Y",
                ExcessAmountRequested => "3Z",
                LockBoxTotal => "4",
                AdjustableRateMortgageChange => "04",
                NonissuedCapital => "4A",
                NotesPayable => "4B",
                NotesReceivable => "4C",
                BankDebentures => "4D",
                BankObligations => "4E",
                Buildings => "4F",
                BuildingsUnderConstruction => "4G",
                Capital => "4H",
                CapitalAssociatedWithPrincipal => "4I",
                CapitalOfOtherSubsidiaries => "4J",
                CapitalStock => "4K",
                Cash => "4L",
                CapitalSubsidiesReceived => "4M",
                CommercialDebt => "4N",
                CommercialExpenses => "4O",
                CommonStock => "4P",
                ConsequentialDamages => "4Q",
                CompensatoryDamages => "4R",
                ConvertibleDebentures => "4S",
                CostOfGoodsSold => "4T",
                CostOfSales => "4U",
                Code4V => "4V",
                CurrentAssets => "4W",
                CurrentLiabilities => "4X",
                Damages => "4Y",
                DeferredCost => "4Z",
                TotalInvoiceAmount => "5",
                FixedInstallmentControlSubstitutionAdjustment => "05",
                DeferredCreditOrIncome => "5A",
                DeferredTaxation => "5B",
                Deposits => "5C",
                Depreciation => "5D",
                DepreciationOfFixedAssets => "5E",
                DepreciationOfRevaluationOfFixedAssets => "5F",
                DirectorsRemuneration => "5G",
                Dividends => "5H",
                DoubtfulReceivables => "5I",
                Equipment => "5J",
                EquipmentSubsidies => "5K",
                Code5L => "5L",
                Equity => "5M",
                ExceptionalItem => "5N",
                Exports => "5O",
                ExternalCharge => "5P",
                ExtraordinaryCharge => "5Q",
                ExtraordinaryCurrentAssetWriteDowns => "5R",
                ExtraordinaryResult => "5S",
                FinancialAssets => "5T",
                FinancialCharges => "5U",
                FinancialDebt => "5V",
                FinancialExpenses => "5W",
                FinancialIncome => "5X",
                FinishedGoods => "5Y",
                FixedAssetDebts => "5Z",
                AmountSubjectToTotalMonetaryDiscount => "6",
                InterestAdjustment => "06",
                FixedAssets => "6A",
                FixedAssetsForSale => "6B",
                Fixtures => "6C",
                FixturesAndEquipment => "6D",
                Franchise => "6E",
                FranchiseTaxBalance => "6F",
                FranchiseTaxPaid => "6G",
                FreeReserves => "6H",
                Furniture => "6I",
                FutureLoan => "6J",
                GeneralAccounts => "6K",
                GeneralExpenses => "6L",
                Goodwill => "6M",
                GrantsForOperatingCosts => "6N",
                GroupRelatedFinancialIncome => "6O",
                IncomeStatedInAdvance => "6P",
                IncomeTax => "6Q",
                Code6R => "6R",
                Code6S => "6S",
                Injunction => "6T",
                IntangibleDepreciation => "6U",
                Intangibles => "6V",
                InterestOfThirdParty => "6W",
                InterestOnLoans => "6X",
                Code6Y => "6Y",
                OptionalReserves => "6Z",
                DiscountAmountDue => "7",
                DeferredGraduatedPaymentMortgageInterestPaid => "07",
                OrganizationalExpenses => "7A",
                OutsideShareInProfitOrLoss => "7B",
                OutstandingDebtsAgainstBoardOfDirectorsManagers => "7C",
                Owing => "7D",
                OwingFromAffiliates => "7E",
                OwingFromParticipants => "7F",
                OwingToAffiliates => "7G",
                OwingToFiscalOffice => "7H",
                OwingToNationalSocialSecurityOffice => "7I",
                OwingToParticipants => "7J",
                OwnWorkCapitalized => "7K",
                PaidInCapital => "7L",
                ParValue => "7M",
                ParticipatingInterest => "7N",
                Patents => "7O",
                PensionDebts => "7P",
                PensionsProvision => "7Q",
                PreferredStock => "7R",
                PrepaidOrdersInProgress => "7S",
                PriorResultsCarriedForward => "7T",
                ProfitOrLoss => "7U",
                ProfitOrLossAfterTaxes => "7V",
                ProfitOrLossBeforeTaxes => "7W",
                ProfitOrLossOnOrdinaryActivitiesAfterTax => "7X",
                ProgressPayments => "7Y",
                ProposedDividend => "7Z",
                TotalMonetaryDiscountAmount => "8",
                InterestAccountingError => "08",
                ProvisionForDepreciationOfStockOrInventory => "8A",
                ProvisionForFuturePurchases => "8B",
                ProvisionForRisks => "8C",
                PunitiveDamages => "8D",
                PurchasePrice => "8E",
                Purchases => "8F",
                RawMaterials => "8G",
                RealEstate => "8H",
                Receivables => "8I",
                RegularizationAccount => "8J",
                ResearchAndDevelopment => "8K",
                RestructuringCosts => "8L",
                Result => "8M",
                RetainedEarnings => "8N",
                Revenues => "8O",
                Sales => "8P",
                SalesAndUseTax => "8Q",
                Savings => "8R",
                SecuredLiability => "8S",
                SecuredLoans => "8T",
                SellingExpenses => "8U",
                Services => "8V",
                ShareCapital => "8W",
                ShareInProfitOrLossOfMinorityInterest => "8X",
                SharePremiumCapital => "8Y",
                SharesInAffiliatedCompanies => "8Z",
                TotalOperationalStatementAmount => "9",
                PrincipalAccountingError => "09",
                SocialCharges => "9A",
                Code9B => "9B",
                SpecialReserves => "9C",
                SpeciallySecuredCreditors => "9D",
                SpecificPerformance => "9E",
                StartingCapital => "9F",
                StatutoryReserves => "9G",
                SubscribedCapital => "9H",
                SuitAmount => "9I",
                Supplies => "9J",
                SurplusOfRevaluation => "9K",
                TangibleNetWorth => "9L",
                TaxAdjustments => "9M",
                TaxBalance => "9N",
                TaxCapitalAmount => "9O",
                TaxOnExtraordinaryItems => "9P",
                TaxRecoverable => "9Q",
                TaxedReserves => "9R",
                TradeCreditors => "9S",
                Code9T => "9T",
                Code9U => "9U",
                Code9V => "9V",
                InvestmentInOwnShares => "9W",
                Investments => "9X",
                IssuedCapital => "9Y",
                LaborCosts => "9Z",
                ShipmentValueInUSDollars => "10",
                LiabilitiesAtBankruptcy => "11",
                AccountAverageBalanceAccount => "12",
                OutstandingBalanceAtForeclosure => "13",
                LegalObligationDebtAmount => "14",
                EstimatedClosingCostAmount => "15",
                DiscountFeesPaidByBorrowerAmount => "16",
                ClosingCostsOrConcessionsPaidBySeller => "17",
                PrepaidItemsAmount => "18",
                Code19 => "19",
                Code20 => "20",
                OriginalCostOfPropertyAmount => "21",
                OwnersEstimateOfValueAmount => "22",
                AppraisedValueAmount => "23",
                GrossMonthlyIncomeAmount => "24",
                AssetsAtBankruptcy => "25",
                NegotiatedCost => "26",
                AuthorizedUnpricedWork => "27",
                TargetPrice => "28",
                EstimatedPrice => "29",
                ContractCeiling => "30",
                EstimatedContractCeiling => "31",
                TargetFeeOrProfitAmount => "32",
                OriginalContractTargetCost => "33",
                NegotiatedContractChanges => "34",
                CurrentTargetCost => "35",
                Code36 => "36",
                Code37 => "37",
                Code38 => "38",
                Code39 => "39",
                Code40 => "40",
                Code41 => "41",
                Code42 => "42",
                Code43 => "43",
                Code44 => "44",
                Code45 => "45",
                Code46 => "46",
                ReprogramCostVariance => "47",
                ReprogramBudget => "48",
                Code49 => "49",
                Code50 => "50",
                AtCompleteVariance => "51",
                TotalAllocatedBudget => "52",
                Code53 => "53",
                Forecast => "54",
                AtCompleteForecast => "55",
                Code56 => "56",
                Code57 => "57",
                Code58 => "58",
                Code59 => "59",
                Code60 => "60",
                Code61 => "61",
                Code62 => "62",
                Code63 => "63",
                InitialContractPriceTarget => "64",
                InitialContractPriceCeiling => "65",
                AdjustedContractPriceTarget => "66",
                AdjustedContractPriceCeiling => "67",
                FundsAuthorizedToDate => "68",
                AccruedExpenditures => "69",
                OpenCommitments => "70",
                ForecastOfBillings => "71",
                EstimatedTerminationCosts => "72",
                AccruedExpendituresPlusOpenCommitments => "73",
                ContractWorkAuthorizedDefinitized => "74",
                ContractWorkAuthorizedNotDefinitized => "75",
                ContractWorkAuthorizedTotal => "76",
                ForecastOfWorkNotYetAuthorized => "77",
                ForecastOfWorkAllOther => "78",
                ForecastOfWorkTotal => "79",
                FundingTotalRequirements => "80",
                FundsCarryover => "81",
                NetFundsRequired => "82",
                Code83 => "83",
                Code84 => "84",
                BestCaseEstimate => "85",
                WorstCaseEstimate => "86",
                MostLikelyEstimate => "87",
                Code88 => "88",
                Code89 => "89",
                Code90 => "90",
                SiteValueAmount => "91",
                Compensation => "92",
                Contribution => "93",
                DeathBenefit => "94",
                DeathBenefitDecrement => "95",
                EmployeeAccountBalance => "96",
                LoanRepayment => "97",
                PriorW2 => "98",
                SinglePremium => "99",
                AdjustedChargebackClaimAmount => "A",
                AssistantshipFromAdmittingEducationalInstitution => "A0",
                AverageNegativeLedgerBalance => "A1",
                AveragePositiveCollectedBalance => "A2",
                AverageNegativeCollectedBalance => "A3",
                AveragePositiveLedgerBalance => "A4",
                DisallowedEstimated => "A5",
                DisallowedActual => "A6",
                NoncoveredChargesEstimated => "A7",
                NoncoveredChargesActual => "A8",
                AllowedEstimated => "A9",
                Allocated => "AA",
                ExcessFunds => "AA1",
                CumulativeTotal => "AA2",
                ReimbursableAmount => "AA3",
                TotalReimbursableAmount => "AA4",
                DirectCitationAmount => "AA5",
                TotalDirectCitationFunds => "AA6",
                ChargeableAmount => "AA7",
                TemporaryTermCoverage => "AAA",
                ConditionalReceiptCoverage => "AAB",
                BindingInterimCoverage => "AAC",
                ApplicationAmount => "AAD",
                ApprovedAmount => "AAE",
                UltimateFaceAmount => "AAF",
                RequestedAmountFromAllReinsurers => "AAG",
                ReplacementAmount => "AAH",
                ScheduledContribution => "AAI",
                ScheduledDisbursement => "AAJ",
                ShortTermInvestment => "AAK",
                SubsequentContribution => "AAL",
                SubsequentDistribution => "AAM",
                TaxFederal => "AAN",
                TaxLocal => "AAO",
                TaxState => "AAP",
                TrustFund => "AAQ",
                CapitalLeases => "AAR",
                Surplus => "AAS",
                RestatedAssets => "AAT",
                OwingToClients => "AAU",
                ShareholderLoans => "AAV",
                AccumulatedDeficit => "AAW",
                LoanFromParentCompany => "AAX",
                ContributionNotSubjectToRepayment => "AAY",
                IncomeBeforeDepreciation => "AAZ",
                AdjustedCollectedBalance => "AB",
                IncomeAfterDepreciation => "ABA",
                CodeABB => "ABB",
                InterestExpenses => "ABC",
                CodeABD => "ABD",
                CodeABE => "ABE",
                IncomeBeforeAllocations => "ABF",
                IncomeFromSaleOfFixedAssets => "ABG",
                ContributionToGroup => "ABH",
                DeferredTaxAssets => "ABI",
                BlockedAccounts => "ABJ",
                NonTaxedReserves => "ABK",
                PledgedAssets => "ABL",
                RestrictedEquity => "ABM",
                NonRestrictedEquity => "ABN",
                DepreciableAssets => "ABO",
                TaxableAssets => "ABP",
                IncomeFromBusiness => "ABQ",
                IncomeSubjectToTaxes => "ABR",
                TaxableAmountOfRealEstate => "ABS",
                EndingPrincipalBalance => "ABT",
                AverageDailyPrincipalBalance => "ABU",
                InterestAmount => "ABV",
                AdjustmentsForDifferenceInAverageDailyPrincipalBalance => "ABW",
                BeginningPrincipalBalance => "ABX",
                LoanPrincipalDisbursements => "ABY",
                PrincipalIncreases => "ABZ",
                AverageCollectedBalance => "AC",
                PrincipalOfLoansPurchased => "ACA",
                PrincipalCured => "ACB",
                PrincipalSold => "ACC",
                PrincipalInsuranceClaims => "ACD",
                PrincipalGuaranteeVoided => "ACE",
                PrincipalPaidByBorrowers => "ACF",
                LoansInSchoolAndGrace => "ACG",
                LoansInAuthorizedDeferment => "ACH",
                LoansRepayOrForebearanceCurrentOrLessThan31Days => "ACI",
                LoansRepayOrForebearance31To60DaysPastDue => "ACJ",
                LoansRepayOrForebearance61To90DaysPastDue => "ACK",
                LoansRepayOrForebearance91To120DaysPastDue => "ACL",
                LoansRepayOrForebearance121To180DaysPastDue => "ACM",
                LoansRepayOrForebearance181To270DaysPastDue => "ACN",
                LoansRepayOrForebearance271OrMoreDaysPastDue => "ACO",
                CodeACP => "ACP",
                AgentSales => "ACQ",
                AmountInvolved => "ACR",
                AssignedCapital => "ACS",
                CreditLineUtilized => "ACT",
                DirectSales => "ACU",
                EarningsPerShare => "ACV",
                Inheritance => "ACW",
                InvestedCapital => "ACX",
                LoanFromFamilyMembers => "ACY",
                NonDepreciableAssets => "ACZ",
                AdjustedTotal => "AD",
                PartiallyPaidAmountPerShare => "ADA",
                PendingOrders => "ADB",
                PersonalLoan => "ADC",
                PlantAndMachinery => "ADD",
                PreTaxLoss => "ADE",
                PreTaxProfit => "ADF",
                RegisteredCapital => "ADG",
                RevaluationReserves => "ADH",
                SocialCapital => "ADI",
                StatutoryProfit => "ADJ",
                TrainingPay => "ADK",
                RetroactivePay => "ADL",
                ExpectedReimbursementAmount => "ADM",
                PermitCost => "ADN",
                Minimum => "ADO",
                AdditionalAmountToMeetMinimum => "ADP",
                LaborPerHour => "ADQ",
                NonRecoverableDepreciation => "ADR",
                RecoverableDepreciation => "ADS",
                Overhead => "ADT",
                IndemnityBenefit => "ADU",
                ReplacementCostOfRepairs => "ADW",
                ActualCashValueOfRepairs => "ADX",
                RecoverableDepreciationOfRepairs => "ADY",
                NonRecoverableDepreciationOfRepairs => "ADZ",
                Arrearage => "AE",
                NonIndemnityBenefit => "AEA",
                ActualCashValueOfBuilding => "AEB",
                GovernmentShare => "AEC",
                ContractorShare => "AED",
                AwardFee => "AEE",
                BaseFee => "AEF",
                TargetProfitFloor => "AEG",
                TargetProfitCeiling => "AEH",
                LaborPerDay => "AEI",
                DifferenceInInterestDue => "AEJ",
                DifferenceInPrepaymentPenalty => "AEK",
                DifferenceInPrincipalDue => "AEL",
                AppropriationOfRetainedEarningsLessReversals => "AEM",
                Appropriations => "AEN",
                BillingsAndCostsProfitDifferential => "AEO",
                CommonStockParValue => "AEP",
                CostOfServicesRendered => "AEQ",
                Creditors => "AER",
                DeclaredProfit => "AES",
                DiscountedNotes => "AET",
                EndorsedNotes => "AEU",
                GeneralReserves => "AEV",
                ImportVolume => "AEW",
                IncomeTaxCredit => "AEX",
                LongTermDeposits => "AEY",
                LongTermLoans => "AEZ",
                AverageFloat => "AF",
                MinorityInterest => "AFA",
                NonOperatingExpense => "AFB",
                NonOperatingIncome => "AFC",
                OperatingProfitOrLoss => "AFD",
                PreferredStockParValue => "AFE",
                ProfitAfterTaxAndMinorityInterest => "AFF",
                RetainedEarningsToBeAppropriated => "AFG",
                RevaluationSurplusOrDeficit => "AFH",
                ReversalOfVoluntaryEarnedSurplus => "AFI",
                SharePrice => "AFJ",
                ShortTermDeposits => "AFK",
                ShortTermLoans => "AFL",
                TaxProvisions => "AFM",
                UnallocatedProfit => "AFN",
                VoluntaryEarnedSurplus => "AFO",
                CalculatedWeeklyCompensationAmount => "AFP",
                BenefitTypeGrossWeeklyAmount => "AFQ",
                BenefitTypeNetWeeklyAmount => "AFR",
                EmployeeGrossWage => "AFS",
                GarageGrossWages => "AFT",
                OfficerCompensationActualFlat => "AFU",
                OfficerCompensationStatutoryMaximum => "AFV",
                OfficerCompensationStatutoryMinimum => "AFW",
                PreviousBalance => "AFY",
                DisputedAmount => "AFZ",
                AdjustedGrossIncome => "AG",
                NonOperatingIncomeOrExpense => "AGA",
                OperatingIncomeOrExpense => "AGB",
                IncomeOrExpense => "AGC",
                PurchaseAuthority => "AGD",
                CapitalDecrease => "AGE",
                CapitalIncrease => "AGF",
                DeedCapital => "AGG",
                TransferredAmount => "AGH",
                UnadjustedSalesPrice => "AGI",
                SalesConcessions => "AGJ",
                PropertyValue => "AGK",
                PartialReleaseAmount => "AGM",
                LienFilingFee => "AGN",
                AdditionalRepairPrice => "AGO",
                QualifiedTuitionAndRelatedExpenses => "AGP",
                QualifiedFinancialAssistance => "AGQ",
                AggregateReimbursementsOrRefunds => "AGR",
                NewLoanBalance => "AGS",
                RawMaterialPurchases => "AGT",
                WorkInProgressPurchases => "AGU",
                OperatingCashFlow => "AGV",
                PaymentsForOutsideWork => "AGW",
                SetAsideForProvisions => "AGX",
                FinancialIncomeOrExpense => "AGY",
                ExtraordinaryIncomeOrExpense => "AGZ",
                LoanBalanceDifference => "AH",
                UnfinishedProductionCarriedForward => "AHA",
                InstallationMaterialsCost => "AHB",
                NewMortgageAmount => "AHC",
                CapitalizedAssets => "AHD",
                ProfitReserves => "AHE",
                ShareInProfitOrLossInOtherCompanies => "AHF",
                MonetaryCorrection => "AHG",
                ScheduledRepaymentAmount => "AHI",
                AmountsPlacedWithOtherBanks => "AHJ",
                DueFromParentCompany => "AHK",
                OwingFromSubsidiaryCompanies => "AHL",
                CertificatesOfDepositAmount => "AHM",
                PubliclyTradedSharesAmount => "AHN",
                NonPubliclyTradedSharesAmount => "AHO",
                TradingSecuritiesAmount => "AHP",
                InvestmentSecuritiesAmount => "AHQ",
                EarningsPerShareMinusDividends => "AHR",
                ActivePartnerCapital => "AHS",
                SaleAmount => "AI",
                FundsHeldByMortgagee => "AJ",
                AdjustedClaim => "AJC",
                AttorneyFees => "AK",
                AverageLedgerBalance => "AL",
                AmountFinanced => "AM",
                BankruptcyFee => "AN",
                AccruedIncome => "ANC",
                AmountOverride => "AO",
                AmountPriorToFractionalization => "AP",
                AmountOfPurchaseExemptFromTaxOrFee => "APT",
                AveragePricePerCall => "AQ",
                AcquisitionCostOfLenses => "AQL",
                FeesToPublicOfficialsForForeclosure => "AR",
                AveragePricePerMinute => "AS",
                AnnualSalesOrPremiums => "ASP",
                TotalReceived => "AT",
                AmountOfTaxOrFeeExemption => "ATF",
                CoverageAmount => "AU",
                ActualCashValue => "AV",
                Average => "AVE",
                ReplacementCost => "AW",
                PreviousPrice => "AX",
                TitleCost => "AY",
                OtherForeclosureAndAcquisitionExpenses => "AZ",
                Estimated => "B",
                Bond => "B0",
                BenefitAmount => "B1",
                BonusesAndCommissionsDividedOver12Months => "B2",
                BonusesDividedOver12Months => "B3",
                BonusesAndCommissions => "B4",
                Budgeted => "B5",
                AllowedActual => "B6",
                DeductibleEstimated => "B7",
                CoInsuranceEstimated => "B8",
                CoInsuranceActual => "B9",
                Bargain => "BA",
                NetTaxableIncome => "BAA",
                OriginalAmountOfInstrument => "BAB",
                AdditionToTax => "BAC",
                ReinstatementFee => "BAD",
                PermitFeeDue => "BAE",
                PermitFeeWithExtension => "BAF",
                NetAnnualPermitFeeDue => "BAG",
                PermitFeePenaltyDue => "BAH",
                PermitFeeInterestDue => "BAI",
                TotalPermitFeeDue => "BAJ",
                FranchiseTax => "BAK",
                UnclaimedFranchiseTaxCredit => "BAL",
                NetFranchiseTaxDue => "BAM",
                FranchiseTaxPenaltyDue => "BAN",
                TotalFranchiseTaxDue => "BAO",
                TotalAmountDue => "BAP",
                Overpayment => "BAQ",
                AmountToBeRefunded => "BAR",
                GrossInStateReceipts => "BAS",
                GrossReceipts => "BAT",
                OccupationFee => "BAU",
                TotalAssessedValue => "BAV",
                TotalValueOfAllProperty => "BAW",
                ValueOfInStateProperty => "BAX",
                TotalValueOfOutOfStateProperty => "BAY",
                TotalParValue => "BAZ",
                MortgageInsurancePremiums => "BB",
                TotalAssessableCapitalStock => "BBA",
                ApportionedValue => "BBB",
                EstimatedInStateRealPropertyValue => "BBC",
                ParValueOfInStateAssets => "BBD",
                InStateBusinessRevenue => "BBE",
                SubscriptionPrice => "BBF",
                ValueOfAuthorizedShares => "BBG",
                OrdinanceAmount => "BBH",
                CapitalForACooperative => "BBI",
                DirectorsLegalObligationDebt => "BBJ",
                SilentPartnerCapital => "BBK",
                CodeBC => "BC",
                BalanceDue => "BD",
                DisbursementsForAuthorizedRepair => "BE",
                HazardInsurancePremium => "BF",
                EvictionAttorneyFees => "BG",
                EvictionExpenses => "BH",
                PropertyTaxes => "BI",
                DisbursementsNotShownElsewhere => "BJ",
                DisbursementsForProtectionAndPreservation => "BK",
                DisbursementsForInspectionsAndBoarding => "BL",
                BridgeLoanNotDeposited => "BLD",
                Adjustments => "BM",
                RentalIncome => "BN",
                RentalExpense => "BO",
                BorrowerAdvance => "BOA",
                BorrowedAmount => "BOR",
                AverageNetCollectedBalance => "BP",
                Bail => "BQ",
                AdjustedInsuredLossAmount => "BR",
                MortgageNoteInterest => "BS",
                BankRejectTotal => "BT",
                Betterment => "BTR",
                OverheadCosts => "BU",
                UncollectedInterest => "BV",
                AmountDueFromBuyerAtClosing => "BW",
                AmountOwedToBuyerAtClosing => "BX",
                BillsOfExchangePayable => "BXP",
                AdditionalClosingExpenses => "BY",
                DeficiencyJudgmentExpenses => "BZ",
                City => "C",
                CurrentExpenditures => "C0",
                CoPaymentAmount => "C1",
                ChildRiderCoverage => "C2",
                PriorPaymentEstimated => "C3",
                PriorPaymentActual => "C4",
                ClaimAmountDueEstimated => "C5",
                ClaimAmountDueActual => "C6",
                PayerResponsibilityEstimated => "C7",
                PayerResponsibilityActual => "C8",
                DisallowedCostContainmentActual => "C9",
                ContractorCumulativeToDate => "CA",
                CollectedBalanceRequired => "CB",
                ChargebackClaimAmount => "CC",
                OverpaidSection235Subsidy => "CD",
                CleanUpCostsAssociatedWithDeficiency => "CDC",
                SummaryAmount => "CE",
                AppraisalFees => "CF",
                CommissionFeesDeducted => "CG",
                ChangeAmount => "CH",
                FundsHeldForInsured => "CI",
                OtherDeductions => "CJ",
                CodeCJL => "CJL",
                BackEndLoad => "CK",
                OutstandingBalanceCurrentLender => "CL",
                ClaimantRequestedTotal => "CM",
                CompressionCharge => "CMC",
                CommodityRefund => "CMR",
                SpecialAssessments => "CN",
                TaxesOnDeed => "CO",
                CorporateAssets => "COA",
                StatutoryDisbursements => "CP",
                ClosingCostsPaidByAnyOtherPartyOtherThanSellerOrBuyer => "CPO",
                ClosingCostsPaidBySeller => "CPS",
                NetClaimAmount => "CQ",
                ContractorAtComplete => "CR",
                ContainerReplacementCost => "CRC",
                ContainerReplacementLaborCost => "CRL",
                ContainerReplacementMaterialCost => "CRM",
                CommissionSales => "CS",
                Contract => "CT",
                SubcontractorCumulativeToDate => "CU",
                SubcontractorAtComplete => "CV",
                EarnedValue => "CW",
                Actual => "CX",
                CumulativeBudget => "CY",
                CumulativeEarnedValue => "CZ",
                PayerAmountPaid => "D",
                AdministrationAndManagementCosts => "D0",
                DeferredCompensationCommissions => "D1",
                DeductibleAmount => "D2",
                DeferredCompensationCommissionsAndBonuses => "D3",
                DeferredCompensation => "D4",
                DollarForDollarDeductions => "D4D",
                DependentCareContribution => "D5",
                DisallowedCostContainmentEstimated => "D6",
                DispensingFee => "D7",
                DiscountAmount => "D8",
                CumulativeActual => "D9",
                OriginalMortgage => "DA",
                UnappliedSection235Funds => "DB",
                UnappliedBuydownFund => "DC",
                DirectDeposit => "DD",
                EstimateOfDamage => "DE",
                Deferral => "DEF",
                DelayedInterest => "DEI",
                AuthorizedBid => "DF",
                DeferredAssets => "DFA",
                EscrowBalance => "DG",
                TotalDisbursements => "DH",
                ChargeOff => "DI",
                LiensAmountOriginal => "DJ",
                ReleaseOfLien => "DK",
                Debit => "DL",
                DelinquentTaxes => "DLQ",
                Asset => "DM",
                Liability => "DN",
                Satisfaction => "DO",
                Exemption => "DP",
                DiscountPointsFinanced => "DPF",
                DiscountPointsNotFinanced => "DPN",
                DefaultedTaxPlanPayment => "DPP",
                Settlement => "DQ",
                AlimonyExpense => "DR",
                AlimonyIncome => "DS",
                ChildSupportExpense => "DT",
                ChildSupportIncome => "DU",
                SeparateMaintenanceExpense => "DV",
                DevelopmentProperties => "DVP",
                SeparateMaintenanceIncome => "DW",
                DeductibleWaived => "DX",
                PerDayLimit => "DY",
                JobRelatedExpense => "DZ",
                EstimatedCredit => "E",
                AdministrationAndManagementIndemnityCharge => "E0",
                EmployerYearToDateContribution => "E1",
                EmployeeAnnualPledgeAmount => "E2",
                EmployeeCurrentContribution => "E3",
                EmployerPledgeAmount => "E4",
                EmployerCurrentContribution => "E5",
                EligibleWageAmount => "E6",
                EmployeeYearToDateContribution => "E7",
                EducationContribution => "E8",
                InitialFee => "E9",
                EarningsAllowance => "EA",
                AdministrativeExpenses => "EAA",
                AirTravelExpenses => "EAB",
                AmountForgiven => "EAC",
                AmountGuaranteed => "EAD",
                AmountOverFairMarketValue => "EAE",
                AmountOwed => "EAF",
                AmountPayable => "EAG",
                AmountRaised => "EAH",
                AmountReceived => "EAI",
                AmountRefunded => "EAJ",
                AmountRescinded => "EAK",
                AnonymousContribution => "EAL",
                BalanceOwed => "EAM",
                BankCharges => "EAN",
                BankLoan => "EAO",
                BrochureExpenses => "EAP",
                BusTravelExpenses => "EAQ",
                ConsultantExpenses => "EAR",
                CorrectedAmount => "EAS",
                DisabilityExpenses => "EAT",
                DisposedAmount => "EAU",
                DrawAmount => "EAV",
                ElectionExpenses => "EAW",
                EndorsementAmount => "EAX",
                EntertainmentExpenses => "EAY",
                ExcessExpenses => "EAZ",
                CodeEB => "EB",
                ExpectedExpenditureAmount => "EBA",
                ExpenditureAmount => "EBB",
                FamilyCareExpenses => "EBC",
                FederalShareAmount => "EBD",
                FilingFee => "EBE",
                InKindContribution => "EBG",
                IncurredAmount => "EBH",
                LoanAmountPlusInterest => "EBI",
                LoanBalance => "EBJ",
                MatchingContribution => "EBK",
                MeetingExpenses => "EBL",
                MiscellaneousIncome => "EBM",
                MiscellaneousReceipts => "EBN",
                NewLoanAmount => "EBO",
                NewUnpaidExpenditure => "EBP",
                NewsletterExpenses => "EBQ",
                NewspaperAdvertisingExpenses => "EBR",
                NominationExpenses => "EBS",
                NonFederalShare => "EBT",
                OfficeExpenses => "EBU",
                OfficeRental => "EBV",
                OriginalAssetValue => "EBW",
                OriginalLoanAmount => "EBX",
                PartyExpenses => "EBY",
                Payment => "EBZ",
                CodeEC => "EC",
                PersonalExpenses => "ECA",
                PersonalFunds => "ECB",
                PledgedAmount => "ECC",
                PostageExpenses => "ECD",
                PrintingExpenses => "ECE",
                PublicFunds => "ECF",
                RadioAdvertisingExpenses => "ECG",
                ReimbursedAmount => "ECH",
                ReportedAmount => "ECI",
                Retainer => "ECJ",
                SignExpenses => "ECK",
                SubContractValue => "ECL",
                TaxReceipts => "ECM",
                TaxiTravelExpenses => "ECN",
                TelecommunicationExpenses => "ECO",
                TelevisionAdvertisingExpenses => "ECP",
                UnpaidExpenditure => "ECQ",
                UtilitiesExpenses => "ECR",
                Total => "ECS",
                Subtotal => "ECT",
                GrandTotal => "ECU",
                IncidentalExpenses => "ECV",
                TransportationExpenses => "ECW",
                GiftValue => "ECX",
                FoodAndRefreshments => "ECY",
                PollingExpenses => "ECZ",
                EstimatedCostOfAttendance => "ED",
                TuitionAndRequiredFees => "ED4",
                BooksAndSupplies => "EDB",
                OtherExpense => "EE",
                EstimatedFinancialAid => "EF",
                OtherIncome => "EG",
                AmountOfMortgagesAndLiens => "EH",
                CodeEI => "EI",
                MaintenanceExpenseOnIncomeProducingProperty => "EIP",
                CodeEJ => "EJ",
                NetRentalIncome => "EK",
                PresentMarketValue => "EL",
                CodeELT => "ELT",
                GrossRentalIncome => "EM",
                CancellationFee => "EN",
                MaintenanceExpenseOnNonIncomeProducingProperty => "ENP",
                EntitlementAmount => "ENT",
                CapitalReserves => "EO",
                EmployerAnnualPledgeAmount => "EP",
                CondominiumAssociationFees => "EQ",
                HomeownerAssociationFees => "ER",
                EarnestMoney => "ERN",
                MortgageInsuranceProceeds => "ES",
                NetProceedsFromSaleOfRealEstateProperty => "ET",
                CodeETD => "ETD",
                CodeEU => "EU",
                PresaleProceeds => "EV",
                PledgedSavings => "EW",
                AsIsBrokersOpinion => "EX",
                ExplorationCosts => "EXC",
                SubjectToBrokersOpinion => "EY",
                UniformCommercialCodeFilingOfficeFee => "EZ",
                AnnualLimit => "F",
                CommercialStaffLaborCosts => "F0",
                CodeF1 => "F1",
                PatientResponsibilityActual => "F2",
                PatientResponsibilityEstimated => "F3",
                PostageClaimed => "F4",
                PatientAmountPaid => "F5",
                ProviderReserves => "F6",
                SalesTax => "F7",
                UsualAndCustomaryChargeEstimated => "F8",
                UsualAndCustomaryActual => "F9",
                CoordinationFee => "FA",
                CodeFAR => "FAR",
                CalculationFee => "FB",
                FinalBalance => "FBA",
                ExpectedFamilyContribution => "FC",
                DirectDepositFlippedToCheck => "FD",
                Fee => "FE",
                ApplicationFee => "FF",
                LicensingFee => "FG",
                RegulatoryFee => "FH",
                FirstInterestPaymentAmount => "FI",
                WaiverFee => "FJ",
                OtherUnlistedAmount => "FK",
                Float => "FL",
                FirstLienAdvance => "FLA",
                FairMarketValue => "FM",
                Fine => "FN",
                FeesPaid => "FO",
                ForeignAssets => "FOA",
                FeesPaidYearToDate => "FP",
                FirmContractorShare => "FQ",
                EstimatedGovernmentShare => "FR",
                FacilitiesRefund => "FRB",
                Expense => "FS",
                EndorsementPremiumAmount => "FT",
                FuelTrackerRefund => "FTR",
                FacilityTaxAmount => "FTX",
                CommercialStaffIndemnityCharge => "FU",
                FlatFeePaidToDate => "FV",
                FlatFeePaidCurrentMonth => "FW",
                Endorsement => "FX",
                FirstPayment => "FY",
                EarnedIncome => "FZ",
                Collateral => "G",
                InitialAdjustmentTotal => "G0",
                IndicatedValueBySalesComparisonApproach => "G1",
                IndicatedValueByIncomeApproach => "G2",
                PricePerUnitArea => "G3",
                ReconciliationOfFinalValueEstimate => "G4",
                EstimatedMonthlyMarketRent => "G5",
                AdjustedSalesPrice => "G6",
                SalesOrFinancingConcessions => "G7",
                IndicatedValueByCostApproach => "G8",
                AsIsValueOfSiteImprovements => "G9",
                DepreciatedValueOfImprovements => "GA",
                Garnishments => "GAR",
                AnnualTax => "GAT",
                CodeGB => "GB",
                CodeGC => "GC",
                CorrectedTaxBill => "GCB",
                GrossClaim => "GCL",
                PhysicalDepreciation => "GD",
                FunctionalDepreciation => "GE",
                ExternalDepreciation => "GF",
                GiftsNotDeposited => "GFD",
                GiftAmount => "GFT",
                AdjustedSalesPriceOfComparableSales => "GG",
                PredominateValue => "GH",
                AverageCustomerIncome => "GI",
                AverageNeighborhoodIncome => "GJ",
                AverageCustomerPurchase => "GK",
                WeeklyDollarSales => "GL",
                AverageCaseSales => "GM",
                BuyDown => "GN",
                CreditLine => "GO",
                AppraisalRepairAmount => "GP",
                BrokersOpinionRepairAmount => "GQ",
                CreditLineAvailable => "GR",
                GrossContribution => "GRC",
                GrantsNotDeposited => "GRD",
                GrossRestoration => "GRR",
                GrantAmount => "GRT",
                SubsequentAdjustmentsTotal => "GS",
                GoodStandingTaxPlanPayment => "GSP",
                GoodsAndServicesTax => "GT",
                TotalSupplementalTaxDue => "GTS",
                TaxesPaid => "GU",
                TaxInstallmentDue => "GUI",
                TaxInstallmentPaid => "GUP",
                TotalSupplementalTaxPaid => "GUS",
                GrossValue => "GV",
                TotalCharge => "GW",
                TotalCredit => "GX",
                TotalDebit => "GY",
                TotalFinanceCharge => "GZ",
                BidAmount => "H",
                CodeH0 => "H0",
                LegalReserves => "H1",
                Cancellation => "H2",
                DepositInceptionToDate => "H3",
                DepositYearToDate => "H4",
                DumpInRemittance => "H5",
                Earnings => "H6",
                LifeInsuranceCashValue => "H7",
                StructureValue => "H8",
                OriginalListPrice => "H9",
                Coin => "HA",
                Currency => "HB",
                USTreasuryChecks => "HC",
                PostalMoneyOrders => "HD",
                HomeEquityLineOfCreditDrawAmount => "HDA",
                CityChecks => "HE",
                HemophiliaAdjustment => "HEM",
                OtherChecks => "HF",
                HomeGovernmentFinancingForEducation => "HG",
                AnnualSocialSecurityWages => "HH",
                AnnualSocialSecurityTips => "HI",
                CodeHJ => "HJ",
                SocialSecurityEmployeeTaxWithheld => "HK",
                FederalIncomeTaxWithheld => "HL",
                AdvanceEarnedIncomeCredit => "HM",
                Commission => "HN",
                VacationPay => "HO",
                HeadOfficeAccount => "HOA",
                GrossPaySubmitted => "HP",
                IntersellCommissionSales => "HQ",
                TotalPayrollApproved => "HR",
                HolidayPay => "HS",
                OvertimePay => "HT",
                RegularPay => "HU",
                SickPay => "HV",
                SpecialPay => "HW",
                ContractPrice => "HX",
                CommercialSpaceIncome => "HY",
                UtilitiesPaidByOwner => "HZ",
                HazardousCleanUpCost => "HZC",
                Interest => "I",
                LifeInsuranceCoverage => "I0",
                InvestmentIncome => "I1",
                Income => "I2",
                CodeI3 => "I3",
                TotalEstimatedRent => "I4",
                GrossAnnualIncome => "I5",
                CustodiansSalary => "I6",
                EngineersSalary => "I7",
                ElevatorOperatorsSalary => "I8",
                IndicatedValueByMarketApproachEstimateOfMarketValue => "I9",
                AdjustedMonthlyRent => "IA",
                InvestableBalance => "IB",
                InterBankLoans => "IBL",
                AccruedUnpaidInterestToBeCapitalized => "IC",
                ImbalanceChargesRefund => "ICR",
                ImportDutyAmount => "ID",
                ExciseTaxAmount => "IE",
                InspectionFee => "IF",
                AdjustmentForGrossLivingArea => "IG",
                CodeIGT => "IGT",
                PredominantPriceHigh => "IH",
                IrregularInterestPaymentAmount => "II",
                NetAdjustedMonthlyRent => "IJ",
                IndicatedMonthlyMarketRent => "IK",
                PredominantPriceLow => "IL",
                AdjustmentForRooms => "IM",
                Installment => "IN",
                InstallmentBalanceAfterTheCurrentInstallmentIsApplied => "INB",
                AdjustmentForBedrooms => "IO",
                InterestPayableDuringRepaymentPeriod => "IP",
                InterestPerDiem => "IPD",
                ContingentDebt => "IQ",
                InsuranceRecovery => "IR",
                IndependentScholarship => "IS",
                InterestSinceClaimSubmission => "ISS",
                IncentiveFee => "IT",
                AccruedUnpaidInterestNotToBeCapitalized => "IU",
                UtilitiesAllowance => "IV",
                InvestmentProperty => "IVP",
                FurnitureAllowance => "IW",
                Debentures => "IY",
                AccountHighBalance => "IZ",
                TrusteeFees => "J",
                LimitedPartnershipCapital => "J0",
                CurrentFaceAmount => "J1",
                OriginalFaceAmount => "J2",
                FixedDefaultNoteHoldersAmount => "J3",
                InitialMonthlyPayment => "J4",
                OriginalPrincipalAndInterestPayment => "J5",
                FinalPrincipalAndInterestPayment => "J6",
                ConversionFee => "J7",
                EndingBalance => "J8",
                BeginningBalance => "J9",
                Assessment => "JA",
                EquityClaimedAsExempt => "JB",
                CounterClaim => "JC",
                WeeklyBenefit => "JD",
                Lease => "JE",
                AdministrativeLoad => "JF",
                AssetCostApplicableToEntireContract => "JG",
                AssetCostApplicableToPortionOfContract => "JH",
                AnnualFee => "JI",
                CostBasis => "JJ",
                DisabilityPremium => "JK",
                EmployeeAdditionalContribution => "JL",
                EmployeeMatchContribution => "JM",
                EmployerContribution => "JN",
                FreeLookValue => "JO",
                FreeWithdrawalValue => "JP",
                FrontEndLoad => "JQ",
                GuaranteedMinimumDeathBenefit => "JR",
                InterimValue => "JS",
                MonthlyRent => "JT",
                Judgment => "JU",
                LoanValue => "JV",
                MarketValue => "JW",
                MarketValueAdjustedValue => "JX",
                MarketValueAdjustment => "JY",
                NetContractValue => "JZ",
                AttorneyAndTrusteeFees => "K",
                DiscountedBillsNotDue => "K0",
                UnpaidSecurityBalance => "K1",
                TotalUnpaidSecurityBalance => "K2",
                VeteransAffairsFundingFee => "K3",
                InitialTargetFee => "K4",
                MinimumFee => "K5",
                MaximumFee => "K6",
                Price => "K7",
                CodeK8 => "K8",
                NewPrice => "K9",
                EstimatedContract => "KA",
                EstimatedNetAdjustment => "KB",
                Obligated => "KC",
                Undefinitized => "KD",
                AnnualRevenue => "KE",
                NetPaidAmount => "KF",
                NetCollectedAmount => "KG",
                DeductionAmount => "KH",
                NetVarianceAmount => "KI",
                MinimumContractAmount => "KJ",
                ItemGrossAmount => "KK",
                CollectedAmount => "KL",
                DisbursedAmount => "KM",
                GrossAmountOfPayment => "KN",
                CommittedAmount => "KO",
                PrincipalAndInterest => "KP",
                IncrementalOrderAmount => "KQ",
                LiabilityLongTerm => "KR",
                TaxesAndInsurance => "KS",
                DefaultPrincipal => "KT",
                DefaultInterest => "KU",
                LiabilityShortTerm => "KV",
                DefaultTaxesAndInsurance => "KW",
                MiscellaneousFeeCollections => "KX",
                NotToExceedPrice => "KY",
                MortgagorsMonthlyObligations => "KZ",
                Local => "L",
                LiquidAssets => "L0",
                LegalContribution => "L1",
                LeaseholdInsuranceAmount => "L2",
                TotalUnidentifiedPaymentsRejected => "L3",
                TotalCreditsReceived => "L4",
                TotalDebitsReceived => "L5",
                TotalPreAdvicesReceived => "L6",
                TotalPrenotesReceived => "L7",
                TotalPostAdvicesReceived => "L8",
                TotalDebitForSettlement => "L9",
                Definitized => "LA",
                DefinitizedTotal => "LB",
                LessorsCost => "LC",
                Incremental => "LD",
                LandRights => "LDR",
                LoanEligibilityAmount => "LE",
                LoanRemittanceOrRepayment => "LF",
                LaundryIncome => "LG",
                Baseline => "LH",
                LineItemUnitPrice => "LI",
                LegalAndAudit => "LJ",
                LoanAmountRequested => "LK",
                LumpSum => "LL",
                Limit => "LM",
                LienPayoff => "LN",
                MoneyPurchase => "LO",
                LowerFund => "LOW",
                ListPrice => "LP",
                TotalSubjectPropertyLiensPaidByClosing => "LPC",
                LeasePurchaseFunds => "LPF",
                LeasePayments => "LPY",
                MaximumPotentialLiability => "LQ",
                TotalCreditForSettlement => "LR",
                NetSettlement => "LS",
                TotalLiabilitiesToBePaidAtClosingNotIncludingSubjectPropertyLiens => {
                    "LSP"
                }
                LossOnSaleOfProperty => "LST",
                TotalAward => "LT",
                OptionAmount => "LU",
                PlannedPeriodicPayment => "LV",
                TaxAndInsuranceEscrowFundBalance => "LW",
                LoanExpense => "LX",
                TotalRemainingPrincipalBalanceForTheIssuer => "LY",
                DelinquentPayment => "LZ",
                AmountDueFromBuyerAtAppraisalNoticeDate => "M",
                LoansFromOfficers => "M0",
                MaximumOutOfPocketAmount => "M1",
                MedicalContribution => "M2",
                TaxRateExpressedAsAFlatFee => "M3",
                MinimumAmountOfTaxToBePaid => "M4",
                MinimumAmountToWhichTaxRateIsApplied => "M5",
                MaximumAmountOfTaxToBePaid => "M6",
                MaximumAmountToWhichTaxRateIsApplied => "M7",
                MarkupAmount => "M8",
                NetOfSurrenderWithdrawal => "M9",
                MaximumAmount => "MA",
                MiscellaneousAdjustment => "MAD",
                UndistributedBudget => "MB",
                CostOfMoney => "MC",
                MinimumDue => "MD",
                MinimumDefaultNoteHoldersCost => "ME",
                AdministrativeFees => "MF",
                MaximumLateCharge => "MG",
                MinimumLateCharge => "MH",
                MinimumIncentiveFee => "MI",
                MaximumDefaultNoteHoldersCost => "MJ",
                GrossToPay => "MK",
                PriorNetInvoiceTotal => "ML",
                Payout => "MM",
                ModifiedMortgageAmount => "MMA",
                MonthlyLimit => "MN",
                MinimumOrderValue => "MO",
                MonthlyPaymentAmount => "MP",
                CodeMQ => "MQ",
                ManagementReserve => "MR",
                PastDueTaxesAndAssessmentRemainingUnpaid => "MS",
                CodeMT => "MT",
                PremiumTaxPaidOnSurrender => "MU",
                PremiumTaxPaidUpFront => "MV",
                SalesLoads => "MW",
                MaximumIncentiveFee => "MX",
                SurrenderValue => "MY",
                ValuationPrice => "MZ",
                Net => "N",
                LoansOrFinancialBorrowings => "N0",
                NetWorth => "N1",
                IndividualIncomeTaxesAndOther => "N2",
                CorporateIncomeAndExcessProfitsTax => "N3",
                ExciseTaxes => "N4",
                EstateAndGiftTaxes => "N5",
                CarrierTaxActTaxes => "N6",
                FederalUnemploymentTaxActTaxes => "N7",
                MiscellaneousTaxes => "N8",
                CodeN9 => "N9",
                NetAdjustment => "NA",
                NetCompensationPosition => "NB",
                NetBenefit => "NBF",
                NetWorthOfBusinessOwned => "NBO",
                NegativeCollectedBalance => "NC",
                NetContribution => "NCT",
                PerPersonMonthlyLimit => "ND",
                NetBilled => "NE",
                CodeNF => "NF",
                MedicareCopayment => "NG",
                MedicareDeductible => "NH",
                MedicarePaid => "NI",
                OtherInsurancePaidAmount => "NJ",
                TotalInForceAndAppliedCoverage => "NK",
                NegativeLedgerBalance => "NL",
                NonCollateralizedAmount => "NM",
                TransactionFee => "NN",
                NonCommissionSales => "NO",
                NetToPayTotal => "NP",
                NoTaxPlanPayment => "NPP",
                AdjustedNonrecurring => "NQ",
                Nonrecurring => "NR",
                NetRestorationExpenses => "NRE",
                NetSavingsAmount => "NS",
                UnitValue => "NT",
                NewTechnologyAdjustment => "NTA",
                ReinsuranceAmount => "NU",
                RenewalAmount => "NV",
                RetentionPerLife => "NW",
                RetentionPerPolicy => "NX",
                CodeNY => "NY",
                EqualizationAccount => "NZ",
                CourtCost => "O",
                ExtraordinaryIncome => "O0",
                AmountOfFirstMortgageBeingRefinanced => "O1",
                OtherFamilyFinancingForEducation => "O2",
                IntangibleAssetsWrittenOff => "O3",
                InterestPayable => "O4",
                InterestReceivable => "O5",
                JointVentureResults => "O6",
                LongTermDebt => "O7",
                LongTermProvisions => "O8",
                Loss => "O9",
                PrincipalBalanceAmount => "OA",
                OutstandingLoanBalance => "OB",
                OpeningBankCharges => "OC",
                DraftAmount => "OD",
                OdorizationCharge => "ODC",
                MiscellaneousCharges => "OE",
                OfficeEquipment => "OEQ",
                ContractorsOffer => "OF",
                OperationalFlowOrderCharge => "OFC",
                OperationalFlowOrderRefund => "OFR",
                CableCharge => "OG",
                HandlingCharges => "OH",
                NonCommissionCharges => "OI",
                Merchandise => "OJ",
                LetterOfCreditAmount => "OK",
                OutstandingBalanceOtherLender => "OL",
                OtherLiabilityAmounts => "OLA",
                OtherMonthlyIncome => "OM",
                NegotiatingBankCharges => "ON",
                OperationalNoticeRefund => "ONR",
                Overdrafts => "OO",
                OriginalPaymentTotal => "OP",
                PayrollCosts => "OQ",
                LetterOfCreditRemainingAmount => "OR",
                OtherSalaries => "OS",
                CommissionAmendmentCharges => "OT",
                Profit => "OU",
                InpatientOutlierAdjustment => "OUT",
                ProfitAndLossDeficit => "OV",
                ProfitAfterExtraordinaryItemsAndBeforeTax => "OW",
                ProfitAfterTaxAndBeforeExtraordinaryItems => "OX",
                PaymentCommission => "OY",
                ProfitDistributedToEmployees => "OZ",
                Penalty => "P",
                ParentalFinancingForEducation => "P0",
                PartnersCalendarYearSalary => "P1",
                PriorPlanYearGrossSalary => "P2",
                PremiumAmount => "P3",
                PriorYearsWage => "P4",
                PartnersTaxYearSalary => "P5",
                PremiumDue => "P6",
                PartnersK1TaxYearAmount => "P7",
                PartnersK1CalendarYearAmount => "P8",
                CurrentMortgagePrincipalBalance => "P9",
                PaymentCancellationTotal => "PA",
                PolicyAdvance => "PAD",
                MinimumDeliveryPurchaseAmount => "PAM",
                PenaltyAndInterest => "PAN",
                BilledAmount => "PB",
                CodePBG => "PBG",
                PositiveCollectedBalance => "PC",
                ProcessingAllowance => "PCA",
                PriorContractCostBasis => "PCC",
                PreviousClaimPayments => "PCP",
                PriorContractSurrenderCharge => "PCS",
                PriorContractValue => "PCV",
                Credit => "PD",
                PlanPeriodElection => "PE",
                CodePEX => "PEX",
                Principal => "PF",
                PortFacilityCharge => "PFC",
                Payoff => "PG",
                ProposedGrossRentForTheSubjectProperty => "PGR",
                PerOccurrenceDeductible => "PH",
                PerOccurrenceMonthlyLimit => "PI",
                PastDue => "PJ",
                PhotographFee => "PK",
                PositiveLedgerBalance => "PL",
                LastPremiumAmount => "PM",
                PriorGrossInvoiceTotal => "PN",
                PercentOverride => "PO",
                PaymentPriorToAdvance => "PP",
                PendingNetSaleProceedsFromNonRealEstateAssets => "PPN",
                PendingNetSaleProceedsFromRealEstateAssets => "PPR",
                AdvanceAmount => "PQ",
                PerOccurrenceLimit => "PR",
                PropertyDamage => "PRD",
                PartialPayrollPayment => "PRL",
                PerOccurrencePerDayLimit => "PS",
                PerOccurrenceAggregateLimit => "PT",
                UnsecuredPriorityClaim => "PU",
                PrepetitionCharges => "PV",
                PerOccurrenceMaximumPerWeekLimit => "PW",
                PerPersonMaximumPerWeekLimit => "PX",
                PerPersonPerDayLimit => "PY",
                OriginalPrincipalBalance => "PZ",
                AmountOwedToBuyerAtAppraisalNoticeDate => "Q",
                LoansToAffiliatedCompanies => "Q0",
                Proposed => "Q1",
                CodeQ2 => "Q2",
                CodeQ3 => "Q3",
                TotalPrenotesAccepted => "Q4",
                TotalPrenotesRejected => "Q5",
                AutomaticPremiumDeduction => "Q6",
                TotalPostAdvicesAccepted => "Q7",
                TotalPostAdvicesRejected => "Q8",
                CashWithApplication => "Q9",
                Combined => "QA",
                CreditCard => "QB",
                DepositFund => "QC",
                DirectBilling => "QD",
                DiscPremium => "QE",
                CodeQF => "QF",
                GovernmentAllotment => "QG",
                InitialPremium => "QH",
                IndividualRetirementAccount60DayRollover => "QI",
                IndividualRetirementAccountDirectTransfer => "QJ",
                IndividualRetirementAccountRegularContribution => "QK",
                KeoghHr10 => "QL",
                KeoghHr10Transfer => "QM",
                CodeQN => "QN",
                ListBilling => "QO",
                ModalPremium => "QP",
                PayrollTaxes => "QQ",
                ParkingIncome => "QR",
                CodeQS => "QS",
                PacPreAuthorizedCheck => "QT",
                PayrollDeduction => "QU",
                Pension => "QV",
                PremiumReceivedWithApplication => "QW",
                ProfitSharingTrust => "QX",
                Qualified => "QY",
                PaymentAmount => "QZ",
                SpendDown => "R",
                LoansToParticipants => "R0",
                CodeR1 => "R1",
                ContingentSecuredDebt => "R2",
                DisputedSecuredDebt => "R3",
                UnliquidatedSecuredDebt => "R4",
                CodeR5 => "R5",
                ContingentUnsecuredDebt => "R6",
                DisputedUnsecuredDebt => "R7",
                UnliquidatedUnsecuredDebt => "R8",
                AtTimeOfFiling => "R9",
                AcceleratedRoyalty => "RA",
                PerPersonDeductible => "RB",
                RefundCheck => "RC",
                PerPersonLimit => "RD",
                ReservationDemandStorage => "RDS",
                ReservationDemandTransportation => "RDT",
                RoyaltyDue => "RE",
                DepositValue => "RET",
                Restitution => "RF",
                BudgetedRedemption => "RG",
                PerPersonAggregateLimit => "RH",
                ResidualValue => "RI",
                RateAmount => "RJ",
                ProvisionForLongTermDepreciation => "RK",
                RegularRemittance => "RL",
                RemittanceRefund => "RM",
                ResidentManagersSalary => "RN",
                Provisions => "RO",
                Repair => "RP",
                RepackagingCost => "RPC",
                RecommendedAmount => "RQ",
                ReserveRequirementAmount => "RR",
                Reserves => "RS",
                LastPayment => "RT",
                TotalDebitsRejected => "RU",
                TotalPaymentsRejected => "RV",
                TotalDelinquency => "RW",
                TotalPreAdvicesAccepted => "RX",
                TotalPreAdvicesRejected => "RY",
                LendersTotalDelinquency => "RZ",
                SubmittedChargebackClaimAmount => "S",
                SelfFinancingForEducation => "S0",
                SalaryAmount => "S1",
                SalaryWithBonuses => "S2",
                SalaryWithCommissions => "S3",
                SalaryWithSubchapterSCorporationIncome => "S4",
                SalaryWithPartnersBonuses => "S5",
                SubchapterSCorporation => "S6",
                SoleProprietorship => "S7",
                PeriodRental => "S8",
                SecuredClaimAllowed => "S9",
                CampaignSummaryAmount => "SA",
                SystemAdjustedAmount => "SAA",
                CodeSAG => "SAG",
                Salvage => "SAL",
                StatedAmount => "SB",
                TotalServiceCharge => "SC",
                StateCareTax => "SCT",
                SalesCharge => "SD",
                ServiceChargesWhichCannotBeCompensatedByBalances => "SE",
                ScholarshipFromAdmittingEducationalInstitution => "SF",
                SecuredBorrowedFundsNotDeposited => "SFD",
                SponsorFinancingForEducation => "SG",
                SurrenderCharge => "SH",
                SubsequentInterestPaymentAmount => "SI",
                SurrenderFull => "SJ",
                SurrenderPartial => "SK",
                SecurityPersonnelsSalary => "SL",
                Supplemental => "SM",
                InsuranceValue => "SM1",
                DeclaredValue => "SM2",
                ShipmentValue => "SM3",
                PayOnDelivery => "SM4",
                LandedCostValue => "SM5",
                SalesAdministrationExpense => "SN",
                SpecialCreditorsAmount => "SO",
                Setoff => "SOF",
                SalesPrice => "SP",
                SpecialDebtorsAmount => "SQ",
                SecuredClaim => "SR",
                SettlementRefundAsApprovedByTheFederalEnergyRegulatoryCommission => "SRD",
                CampaignSummaryAmountToBeShared => "SS",
                SharesInSubsidiaryCompanies => "SSC",
                State => "ST",
                Surcharge => "SU",
                FixedMonthlyPrincipalPayment => "SV",
                BaseAwardFee => "SW",
                SeveranceTax => "SX",
                InitialBuydownBalance => "SY",
                CertificationFee => "SZ",
                Tax => "T",
                ThirdPartyGovernmentFinancingForEducation => "T0",
                Teacher => "T1",
                TotalClaimBeforeTaxes => "T2",
                TotalSubmittedCharges => "T3",
                TotalCurrentBalance => "T4",
                TotalClaims => "T5",
                Claim => "T6",
                TotalCreditsAccepted => "T7",
                TotalCreditsRejected => "T8",
                TotalDebitsAccepted => "T9",
                TotalAnnualSales => "TA",
                TotalAnnualSalesToCustomer => "TB",
                TotalBuyerClosingCosts => "TBC",
                ProposedCost => "TC",
                TotalCommissionFromPrimaryAndSecondarySources => "TCS",
                ProposedProfit => "TD",
                TotalDepositoryAccounts => "TDA",
                ProposedFee => "TE",
                TotalMaintenanceExpenseOnAllNonIncomeProducingProperties => "TEN",
                TotalMaintenanceExpenseOnAllIncomeProducingProperties => "TEP",
                TotalProposedPrice => "TF",
                AlternateProposedPrice => "TG",
                TotalGiftsNotDeposited => "TGD",
                TotalClaimAllowed => "TH",
                TitleInsuranceAmountOnLoan => "TI",
                TotalSelfEmployedIncomeFromPrimaryAndSecondarySources => "TIS",
                TimeAndExpensePaidToDate => "TJ",
                TotalAmountOfContract => "TK",
                TotalPriorLoanAmountOwed => "TL",
                TotalOtherLiquidAssets => "TLA",
                TotalLifeInsuranceNetCashValue => "TLV",
                TimeAndExpensePaidCurrentMonth => "TM",
                TotalMonetaryMarkupAmount => "TMM",
                TelephoneOperatorsSalary => "TO",
                TotalOmittedLiabilities => "TOL",
                TotalPaymentAmount => "TP",
                TotalPreviousAdjustedClaim => "TPA",
                TotalNetProceedsFromRealEstateAssets => "TPR",
                TotalPaidAsSubmitted => "TPS",
                SubsidiesForOperatingCosts => "TQ",
                TargetCost => "TR",
                TotalRetirementFunds => "TRF",
                TotalNonRentalIncome => "TRI",
                TotalResubordinatedLiabilities => "TRL",
                TotalLiabilitiesForRentalProperties => "TRP",
                TotalSales => "TS",
                TotalStocksAndBonds => "TSB",
                TotalTransactionAmount => "TT",
                TransportationCostPerUnitOfMeasure => "TU",
                LevelOfPremiumInsuranceRetention => "TV",
                TechniciansIndemnityProvision => "TW",
                TotalToDate => "TX",
                TotalAtComplete => "TY",
                TransportationCostTotal => "TZ",
                Underpayment => "U",
                USGovernmentFinancingForEducation => "U0",
                CodeU1 => "U1",
                IngredientCostClaimed => "U2",
                MiscellaneousExpenses => "U3",
                PresentValueOfLot => "U4",
                CostOfImprovements => "U5",
                CodeU6 => "U6",
                Land => "U7",
                Refinance => "U8",
                TaxPaymentRefinancedBySameLender => "U8L",
                EstimatedPrepaidItems => "U9",
                UnliquidatedAmount => "UA",
                UserAdjustedAmount => "UAA",
                CodeUAR => "UAR",
                UnpaidPrincipalBalance => "UB",
                UnspecifiedAggregateLimit => "UC",
                CodeUD => "UD",
                MortgageInsurance => "UE",
                UpdatedExpensesOnPresale => "UEP",
                CodeUF => "UF",
                UnsecuredBorrowedFundsNotDeposited => "UFD",
                TotalUnpaidPrincipalBalanceForStaffordLoans => "UG",
                SubordinateFinancing => "UH",
                UnsecuredHomeImprovements => "UHI",
                TotalCosts => "UI",
                UpdatedInterestOnPresale => "UIP",
                OtherCredits => "UJ",
                CodeUK => "UK",
                MortgageInsuranceFinanced => "UL",
                TotalLoanAmount => "UM",
                CodeUN => "UN",
                UnknownTaxPlanPayment => "UNK",
                CashFromOrToBorrower => "UO",
                TotalUnpaidPrincipalBalanceForParentalLoansForStudents => "UP",
                UpperFund => "UPF",
                MonthlyIncome => "UQ",
                UnearnedIncome => "UR",
                TotalUnpaidPrincipalBalanceForSupplementalLoansForStudents => "US",
                ValueAddedSales => "UT",
                ClearingHouseSettlement => "UU",
                Drawback => "UV",
                TotalMonthlyLiabilities => "UW",
                CodeUX => "UX",
                TotalAssets => "UY",
                TotalLiquidAssets => "UZ",
                CostOfDeficiency => "V",
                ValueAdded => "V0",
                TaxAndInsuranceEscrowFund => "V1",
                InterestDueToInvestor => "V2",
                TotalPrincipalDueToTheInvestor => "V3",
                TotalInterestDueToTheInvestor => "V4",
                TotalCurtailmentDueToTheInvestor => "V5",
                TotalPrincipalPayoffAndRepurchaseDueToTheInvestor => "V6",
                TotalInterestPayoffAndRepurchaseDueToTheInvestor => "V7",
                ActualOutstandingPrincipalBalance => "V8",
                FaceAmount => "V9",
                CodeVA => "VA",
                TotalNonLiquidAssets => "VB",
                Authorized => "VC",
                ActualPersonDayRate => "VD",
                EstimatedPersonDayRate => "VE",
                VestedEarnedUpperFund => "VES",
                TotalMonthlyExpenses => "VF",
                CurrentMonthlyPrincipalAndInterest => "VG",
                LevyAmount => "VH",
                CurrentSupport => "VI",
                PastDueSupport => "VJ",
                MedicalSupport => "VK",
                NetNegativeAmortizationAmount => "VL",
                WithholdFromWages => "VM",
                CommissionBasis => "VN",
                CommissionEarned => "VO",
                CurrentMonthlyPayment => "VP",
                CommissionNetted => "VQ",
                TotalMonthlyDebt => "VR",
                VolumetricReservation => "VRS",
                OtherFinancingPayment => "VS",
                ValueOfSecuritiesAtIssueDate => "VSI",
                ValueOfSecuritiesAtMaturity => "VSM",
                CurrentValue => "VT",
                ClosingCost => "VU",
                CapitalizedMortgageAmount => "VV",
                FirstMortgageMonthlyPrincipalAndInterest => "VW",
                InterestAmountPaidToDate => "VX",
                MinimumTransfer => "VY",
                MaximumTransfer => "VZ",
                DeficiencyJudgmentFees => "W",
                TradeDebtors => "W0",
                W2 => "W1",
                W2WithBonuses => "W2",
                W2WithDeferredCompensation => "W3",
                W2WithoutBonuses => "W4",
                DepositSubTotal => "W5",
                DirectRollover => "W6",
                DirectTransfer => "W7",
                Discounted => "W8",
                SecondaryFinance => "W9",
                MinimumDeposit => "WA",
                SubAgencyCompensation => "WB",
                BuyersAgencyCompensation => "WC",
                VariableRateCompensation => "WD",
                CompensationBonusOnSaleOfProperty => "WE",
                VeteransAffairsLoanGuarantee => "WF",
                SecurityTradeAmount => "WG",
                CodeWH => "WH",
                OtherFinancing => "WI",
                DualAgencyCompensation => "WJ",
                PerWeekLimit => "WK",
                LendersOpinionOfValue => "WL",
                TotalOriginalPrincipalBalance => "WM",
                OtherAgentCompensation => "WN",
                DockUsageFee => "WO",
                PoolUsageFee => "WP",
                ClubhouseFee => "WQ",
                OptionalServiceFee => "WR",
                OtherAssociationFees => "WS",
                CodeWT => "WT",
                CodeWU => "WU",
                TotalPointsPaidAtClosing => "WV",
                AmountThatWouldHaveBeenPaidInTheAbsenceOfCapitation => "WW",
                PointsPaidBySeller => "WX",
                LoanWithdrawal => "WY",
                SeverancePay => "WZ",
                DeficiencyJudgmentExpensesAndFees => "X",
                TrebleDamages => "X0",
                TransferToUntaxedReserves => "X1",
                Reissued => "X2",
                RolloverAmount => "X3",
                AnnualRental => "X4",
                GrossMonthlyRent => "X5",
                SepSelfEmployeePension => "X6",
                FundingAmount => "X8",
                CodeX9 => "X9",
                MaximumAwardFee => "XA",
                MaturityValue => "XB",
                EarnedWages => "XC",
                BasePeriodWage => "XD",
                Withdrawal => "XE",
                WithdrawalInceptionToDate => "XF",
                WithdrawalLessMarketValueAdjustment => "XG",
                WithdrawalLessSurrender => "XH",
                WithdrawalLessTaxes => "XI",
                WithdrawalYearToDate => "XJ",
                UnavailableReserves => "XK",
                UncalledCapital => "XL",
                UnemploymentContribution => "XM",
                CodeXN => "XN",
                UnpaidCapital => "XO",
                UnsecuredLiabilities => "XP",
                ValueAddedTax => "XQ",
                ValueOfShares => "XR",
                Vehicles => "XS",
                VoluntaryReserves => "XT",
                Extraction => "XTR",
                Wages => "XU",
                Withholding => "XV",
                OriginalValue => "XW",
                WorkingCapital => "XX",
                SalesPricePerDwellingUnit => "XY",
                SalesPricePerRoom => "XZ",
                CurrentListPrice => "Y",
                SelfInsuranceAmount => "Y0",
                YearToDateEligibleSalary => "Y1",
                TotalRealEstateOwned => "Y2",
                TotalLiabilities => "Y3",
                TotalLiabilityMonthlyPayments => "Y4",
                TotalRealEstateOwnedMarketValue => "Y5",
                TotalRealEstateOwnedGrossRentalIncome => "Y6",
                TotalRealEstateOwnedMortgagesAndLiens => "Y7",
                TotalRealEstateOwnedMortgagePayments => "Y8",
                TotalRealEstateOwnedMiscellaneousExpenses => "Y9",
                TotalRealEstateOwnedNetRentalIncome => "YA",
                ActualUnpaidPrincipalBalance => "YB",
                ScheduledUnpaidPrincipalBalance => "YC",
                PrincipalDueToInvestor => "YD",
                ConstantPrincipalAndInterest => "YE",
                OtherFeeCollection => "YF",
                BeginningScheduledUnpaidPrincipalBalance => "YG",
                TaxAndInsurancePrincipalBalance => "YH",
                NewPrincipalAndInterest => "YI",
                Curtailment => "YJ",
                PrepaymentPenalty => "YK",
                PartialAnnuitization => "YL",
                PartialWithdrawal => "YM",
                CodeYN => "YN",
                CodeYO => "YO",
                PolicyAmount => "YP",
                PaymentsInAdvance => "YQ",
                PaymentsInArrears => "YR",
                Cancelled => "YS",
                Denied => "YT",
                InProcess => "YU",
                Requested => "YV",
                Paid => "YW",
                PaidForThisFacility => "YX",
                Returned => "YY",
                TotalAggregateLimit => "YZ",
                ListPriceWhenSold => "Z",
                InsertionCost => "Z0",
                RepackagingLaborCost => "Z1",
                RepackagingMaterialCost => "Z2",
                UnitCostOfDiscrepantMaterial => "Z3",
                LiquidationPrincipal => "Z4",
                RemainingPoolBalance => "Z5",
                RemainingSecurityBalance => "Z6",
                ProgramCost => "Z7",
                OverrideToHandlingFee => "Z8",
                ProductionCost => "Z9",
                FederalMedicareOrMedicaidClaimMandateCategory1 => "ZA",
                FederalMedicareOrMedicaidClaimMandateCategory2 => "ZB",
                FederalMedicareOrMedicaidClaimMandateCategory3 => "ZC",
                FederalMedicareOrMedicaidClaimMandateCategory4 => "ZD",
                FederalMedicareOrMedicaidClaimMandateCategory5 => "ZE",
                FederalPensionMandateCategory1 => "ZF",
                FederalPensionMandateCategory2 => "ZG",
                FederalPensionMandateCategory3 => "ZH",
                FederalPensionMandateCategory4 => "ZI",
                FederalPensionMandateCategory5 => "ZJ",
                FederalMedicareOrMedicaidPaymentMandateCategory1 => "ZK",
                FederalMedicareOrMedicaidPaymentMandateCategory2 => "ZL",
                FederalMedicareOrMedicaidPaymentMandateCategory3 => "ZM",
                FederalMedicareOrMedicaidPaymentMandateCategory4 => "ZN",
                FederalMedicareOrMedicaidPaymentMandateCategory5 => "ZO",
                CouponFaceValue => "ZP",
                InitialTargetCost => "ZQ",
                Increase => "ZR",
                Decrease => "ZS",
                ProratedAmount => "ZT",
                LoanCharge => "ZU",
                MortgageRecordingFee => "ZV",
                DeedRecordingFee => "ZW",
                ReleaseRecordingFee => "ZX",
                Assumption => "ZY",
                MutuallyDefined => "ZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<AmountQualifierCode> {
        use AmountQualifierCode::*;
        match code {
            b"00" => Some(DownpaymentOnTheRepaymentPlanAmount),
            b"0A" => Some(InterestSubstitutionAdjustment),
            b"0B" => Some(PrincipalSubstitutionAdjustment),
            b"0C" => Some(PrepaidInterest),
            b"0D" => Some(PrepaidPrincipal),
            b"0E" => Some(DelinquentInterest),
            b"0F" => Some(DelinquentPrincipal),
            b"0G" => Some(CurtailmentAdjustment),
            b"0H" => Some(SerialNotePrincipalAvailableForDistribution),
            b"0I" => Some(ServicingFee),
            b"0J" => Some(GuaranteeFeeAdjustment),
            b"0K" => Some(AmountUnderCollateralized),
            b"0L" => Some(AmountOverCollateralized),
            b"0M" => Some(TrialBalanceAdjustment),
            b"0N" => Some(CustodialBankAccountAdjustment),
            b"0P" => Some(Item),
            b"0Q" => Some(Schedule),
            b"0R" => Some(RegularPlan),
            b"0S" => Some(PreviouslyBilled),
            b"0T" => Some(CurrentlyDue),
            b"0U" => Some(CoveragePremium),
            b"1" => Some(LineItemTotal),
            b"01" => Some(FixedInstallmentControlAccountingError),
            b"1A" => Some(Variance),
            b"1B" => Some(VarianceAdjustmentCost),
            b"1C" => Some(VarianceAdjustmentSchedule),
            b"1D" => Some(Escalation),
            b"1E" => Some(FixedPrice),
            b"1F" => Some(Lodging),
            b"1G" => Some(Meals),
            b"1H" => Some(TravelExpense),
            b"1J" => Some(InsuranceExpense),
            b"1K" => Some(UnionDues),
            b"1L" => Some(RegularIncome),
            b"1M" => Some(IncomeOnRealProperty),
            b"1N" => Some(IncomeFromSocialSecurityAndOrOtherGovernmentAssistance),
            b"1P" => Some(TotalMonthlyIncome),
            b"1Q" => Some(ElectricAndOrFuelPayment),
            b"1R" => Some(WaterAndOrSewerPayment),
            b"1S" => Some(TelephonePayment),
            b"1T" => Some(MaintenanceExpense),
            b"1U" => Some(FoodExpense),
            b"1V" => Some(ClothingExpense),
            b"1W" => Some(LaundryExpense),
            b"1X" => Some(MedicalAndOrDentalExpense),
            b"1Y" => Some(RecreationExpenses),
            b"1Z" => Some(CharitableContributions),
            b"2" => Some(BatchTotal),
            b"02" => Some(GraduatedPaymentMortgageAdjustment),
            b"2A" => Some(HomeInsuranceExpense),
            b"2B" => Some(LifeInsuranceExpense),
            b"2C" => Some(HealthInsuranceExpense),
            b"2D" => Some(AutomobileInsuranceExpense),
            b"2E" => Some(ValueOfPropertyClaimedAsExempt),
            b"2F" => Some(AutomobilePayment),
            b"2G" => Some(OtherTypeOfInstallmentPayment),
            b"2H" => Some(OperatingExpenses),
            b"2I" => Some(TotalProjectedMonthlyIncome),
            b"2J" => Some(TotalProjectedMonthlyExpenses),
            b"2K" => Some(ExcessIncome),
            b"2L" => Some(ValueOfPersonalProperty),
            b"2M" => Some(MonthlyOvertime),
            b"2N" => Some(TotalAllRepairs),
            b"2P" => Some(TotalRecommendedRepairs),
            b"2Q" => Some(StateQuarterlyTotalGrossWages),
            b"2R" => Some(Code2R),
            b"2S" => Some(Code2S),
            b"2T" => Some(Code2T),
            b"2U" => Some(StateQuarterlyDisabilityInsuranceTaxableWages),
            b"2V" => Some(StateQuarterlyTipWages),
            b"2W" => Some(AssetLongTerm),
            b"2X" => Some(AssetShortTerm),
            b"2Y" => Some(BaseCoverage),
            b"2Z" => Some(CommissionRetained),
            b"3" => Some(DepositTotal),
            b"03" => Some(GrowingEquityMortgageAdjustment),
            b"3A" => Some(Accounting),
            b"3B" => Some(AccountsPayable),
            b"3C" => Some(AccountsReceivable),
            b"3D" => Some(AdvancedDividends),
            b"3E" => Some(AdvertisingExpenses),
            b"3F" => Some(Amortization),
            b"3G" => Some(AmortizationCosts),
            b"3H" => Some(AmountOfDecree),
            b"3I" => Some(AssetInvestment),
            b"3J" => Some(AuthorizedCapital),
            b"3K" => Some(AvailableReserves),
            b"3L" => Some(BadDebtAllowance),
            b"3M" => Some(BadDebts),
            b"3N" => Some(Code3N),
            b"3O" => Some(LongTermAssets),
            b"3P" => Some(LongTermLiabilities),
            b"3Q" => Some(LongTermTangibleAssets),
            b"3R" => Some(LossesOnCapital),
            b"3S" => Some(MachinesAndTools),
            b"3T" => Some(MemberRiskCapital),
            b"3U" => Some(MiscellaneousAfterTaxExempt),
            b"3V" => Some(Mortgage),
            b"3W" => Some(NominalCapital),
            b"3X" => Some(NominalDamages),
            b"3Y" => Some(NonOperationalFixedAssets),
            b"3Z" => Some(ExcessAmountRequested),
            b"4" => Some(LockBoxTotal),
            b"04" => Some(AdjustableRateMortgageChange),
            b"4A" => Some(NonissuedCapital),
            b"4B" => Some(NotesPayable),
            b"4C" => Some(NotesReceivable),
            b"4D" => Some(BankDebentures),
            b"4E" => Some(BankObligations),
            b"4F" => Some(Buildings),
            b"4G" => Some(BuildingsUnderConstruction),
            b"4H" => Some(Capital),
            b"4I" => Some(CapitalAssociatedWithPrincipal),
            b"4J" => Some(CapitalOfOtherSubsidiaries),
            b"4K" => Some(CapitalStock),
            b"4L" => Some(Cash),
            b"4M" => Some(CapitalSubsidiesReceived),
            b"4N" => Some(CommercialDebt),
            b"4O" => Some(CommercialExpenses),
            b"4P" => Some(CommonStock),
            b"4Q" => Some(ConsequentialDamages),
            b"4R" => Some(CompensatoryDamages),
            b"4S" => Some(ConvertibleDebentures),
            b"4T" => Some(CostOfGoodsSold),
            b"4U" => Some(CostOfSales),
            b"4V" => Some(Code4V),
            b"4W" => Some(CurrentAssets),
            b"4X" => Some(CurrentLiabilities),
            b"4Y" => Some(Damages),
            b"4Z" => Some(DeferredCost),
            b"5" => Some(TotalInvoiceAmount),
            b"05" => Some(FixedInstallmentControlSubstitutionAdjustment),
            b"5A" => Some(DeferredCreditOrIncome),
            b"5B" => Some(DeferredTaxation),
            b"5C" => Some(Deposits),
            b"5D" => Some(Depreciation),
            b"5E" => Some(DepreciationOfFixedAssets),
            b"5F" => Some(DepreciationOfRevaluationOfFixedAssets),
            b"5G" => Some(DirectorsRemuneration),
            b"5H" => Some(Dividends),
            b"5I" => Some(DoubtfulReceivables),
            b"5J" => Some(Equipment),
            b"5K" => Some(EquipmentSubsidies),
            b"5L" => Some(Code5L),
            b"5M" => Some(Equity),
            b"5N" => Some(ExceptionalItem),
            b"5O" => Some(Exports),
            b"5P" => Some(ExternalCharge),
            b"5Q" => Some(ExtraordinaryCharge),
            b"5R" => Some(ExtraordinaryCurrentAssetWriteDowns),
            b"5S" => Some(ExtraordinaryResult),
            b"5T" => Some(FinancialAssets),
            b"5U" => Some(FinancialCharges),
            b"5V" => Some(FinancialDebt),
            b"5W" => Some(FinancialExpenses),
            b"5X" => Some(FinancialIncome),
            b"5Y" => Some(FinishedGoods),
            b"5Z" => Some(FixedAssetDebts),
            b"6" => Some(AmountSubjectToTotalMonetaryDiscount),
            b"06" => Some(InterestAdjustment),
            b"6A" => Some(FixedAssets),
            b"6B" => Some(FixedAssetsForSale),
            b"6C" => Some(Fixtures),
            b"6D" => Some(FixturesAndEquipment),
            b"6E" => Some(Franchise),
            b"6F" => Some(FranchiseTaxBalance),
            b"6G" => Some(FranchiseTaxPaid),
            b"6H" => Some(FreeReserves),
            b"6I" => Some(Furniture),
            b"6J" => Some(FutureLoan),
            b"6K" => Some(GeneralAccounts),
            b"6L" => Some(GeneralExpenses),
            b"6M" => Some(Goodwill),
            b"6N" => Some(GrantsForOperatingCosts),
            b"6O" => Some(GroupRelatedFinancialIncome),
            b"6P" => Some(IncomeStatedInAdvance),
            b"6Q" => Some(IncomeTax),
            b"6R" => Some(Code6R),
            b"6S" => Some(Code6S),
            b"6T" => Some(Injunction),
            b"6U" => Some(IntangibleDepreciation),
            b"6V" => Some(Intangibles),
            b"6W" => Some(InterestOfThirdParty),
            b"6X" => Some(InterestOnLoans),
            b"6Y" => Some(Code6Y),
            b"6Z" => Some(OptionalReserves),
            b"7" => Some(DiscountAmountDue),
            b"07" => Some(DeferredGraduatedPaymentMortgageInterestPaid),
            b"7A" => Some(OrganizationalExpenses),
            b"7B" => Some(OutsideShareInProfitOrLoss),
            b"7C" => Some(OutstandingDebtsAgainstBoardOfDirectorsManagers),
            b"7D" => Some(Owing),
            b"7E" => Some(OwingFromAffiliates),
            b"7F" => Some(OwingFromParticipants),
            b"7G" => Some(OwingToAffiliates),
            b"7H" => Some(OwingToFiscalOffice),
            b"7I" => Some(OwingToNationalSocialSecurityOffice),
            b"7J" => Some(OwingToParticipants),
            b"7K" => Some(OwnWorkCapitalized),
            b"7L" => Some(PaidInCapital),
            b"7M" => Some(ParValue),
            b"7N" => Some(ParticipatingInterest),
            b"7O" => Some(Patents),
            b"7P" => Some(PensionDebts),
            b"7Q" => Some(PensionsProvision),
            b"7R" => Some(PreferredStock),
            b"7S" => Some(PrepaidOrdersInProgress),
            b"7T" => Some(PriorResultsCarriedForward),
            b"7U" => Some(ProfitOrLoss),
            b"7V" => Some(ProfitOrLossAfterTaxes),
            b"7W" => Some(ProfitOrLossBeforeTaxes),
            b"7X" => Some(ProfitOrLossOnOrdinaryActivitiesAfterTax),
            b"7Y" => Some(ProgressPayments),
            b"7Z" => Some(ProposedDividend),
            b"8" => Some(TotalMonetaryDiscountAmount),
            b"08" => Some(InterestAccountingError),
            b"8A" => Some(ProvisionForDepreciationOfStockOrInventory),
            b"8B" => Some(ProvisionForFuturePurchases),
            b"8C" => Some(ProvisionForRisks),
            b"8D" => Some(PunitiveDamages),
            b"8E" => Some(PurchasePrice),
            b"8F" => Some(Purchases),
            b"8G" => Some(RawMaterials),
            b"8H" => Some(RealEstate),
            b"8I" => Some(Receivables),
            b"8J" => Some(RegularizationAccount),
            b"8K" => Some(ResearchAndDevelopment),
            b"8L" => Some(RestructuringCosts),
            b"8M" => Some(Result),
            b"8N" => Some(RetainedEarnings),
            b"8O" => Some(Revenues),
            b"8P" => Some(Sales),
            b"8Q" => Some(SalesAndUseTax),
            b"8R" => Some(Savings),
            b"8S" => Some(SecuredLiability),
            b"8T" => Some(SecuredLoans),
            b"8U" => Some(SellingExpenses),
            b"8V" => Some(Services),
            b"8W" => Some(ShareCapital),
            b"8X" => Some(ShareInProfitOrLossOfMinorityInterest),
            b"8Y" => Some(SharePremiumCapital),
            b"8Z" => Some(SharesInAffiliatedCompanies),
            b"9" => Some(TotalOperationalStatementAmount),
            b"09" => Some(PrincipalAccountingError),
            b"9A" => Some(SocialCharges),
            b"9B" => Some(Code9B),
            b"9C" => Some(SpecialReserves),
            b"9D" => Some(SpeciallySecuredCreditors),
            b"9E" => Some(SpecificPerformance),
            b"9F" => Some(StartingCapital),
            b"9G" => Some(StatutoryReserves),
            b"9H" => Some(SubscribedCapital),
            b"9I" => Some(SuitAmount),
            b"9J" => Some(Supplies),
            b"9K" => Some(SurplusOfRevaluation),
            b"9L" => Some(TangibleNetWorth),
            b"9M" => Some(TaxAdjustments),
            b"9N" => Some(TaxBalance),
            b"9O" => Some(TaxCapitalAmount),
            b"9P" => Some(TaxOnExtraordinaryItems),
            b"9Q" => Some(TaxRecoverable),
            b"9R" => Some(TaxedReserves),
            b"9S" => Some(TradeCreditors),
            b"9T" => Some(Code9T),
            b"9U" => Some(Code9U),
            b"9V" => Some(Code9V),
            b"9W" => Some(InvestmentInOwnShares),
            b"9X" => Some(Investments),
            b"9Y" => Some(IssuedCapital),
            b"9Z" => Some(LaborCosts),
            b"10" => Some(ShipmentValueInUSDollars),
            b"11" => Some(LiabilitiesAtBankruptcy),
            b"12" => Some(AccountAverageBalanceAccount),
            b"13" => Some(OutstandingBalanceAtForeclosure),
            b"14" => Some(LegalObligationDebtAmount),
            b"15" => Some(EstimatedClosingCostAmount),
            b"16" => Some(DiscountFeesPaidByBorrowerAmount),
            b"17" => Some(ClosingCostsOrConcessionsPaidBySeller),
            b"18" => Some(PrepaidItemsAmount),
            b"19" => Some(Code19),
            b"20" => Some(Code20),
            b"21" => Some(OriginalCostOfPropertyAmount),
            b"22" => Some(OwnersEstimateOfValueAmount),
            b"23" => Some(AppraisedValueAmount),
            b"24" => Some(GrossMonthlyIncomeAmount),
            b"25" => Some(AssetsAtBankruptcy),
            b"26" => Some(NegotiatedCost),
            b"27" => Some(AuthorizedUnpricedWork),
            b"28" => Some(TargetPrice),
            b"29" => Some(EstimatedPrice),
            b"30" => Some(ContractCeiling),
            b"31" => Some(EstimatedContractCeiling),
            b"32" => Some(TargetFeeOrProfitAmount),
            b"33" => Some(OriginalContractTargetCost),
            b"34" => Some(NegotiatedContractChanges),
            b"35" => Some(CurrentTargetCost),
            b"36" => Some(Code36),
            b"37" => Some(Code37),
            b"38" => Some(Code38),
            b"39" => Some(Code39),
            b"40" => Some(Code40),
            b"41" => Some(Code41),
            b"42" => Some(Code42),
            b"43" => Some(Code43),
            b"44" => Some(Code44),
            b"45" => Some(Code45),
            b"46" => Some(Code46),
            b"47" => Some(ReprogramCostVariance),
            b"48" => Some(ReprogramBudget),
            b"49" => Some(Code49),
            b"50" => Some(Code50),
            b"51" => Some(AtCompleteVariance),
            b"52" => Some(TotalAllocatedBudget),
            b"53" => Some(Code53),
            b"54" => Some(Forecast),
            b"55" => Some(AtCompleteForecast),
            b"56" => Some(Code56),
            b"57" => Some(Code57),
            b"58" => Some(Code58),
            b"59" => Some(Code59),
            b"60" => Some(Code60),
            b"61" => Some(Code61),
            b"62" => Some(Code62),
            b"63" => Some(Code63),
            b"64" => Some(InitialContractPriceTarget),
            b"65" => Some(InitialContractPriceCeiling),
            b"66" => Some(AdjustedContractPriceTarget),
            b"67" => Some(AdjustedContractPriceCeiling),
            b"68" => Some(FundsAuthorizedToDate),
            b"69" => Some(AccruedExpenditures),
            b"70" => Some(OpenCommitments),
            b"71" => Some(ForecastOfBillings),
            b"72" => Some(EstimatedTerminationCosts),
            b"73" => Some(AccruedExpendituresPlusOpenCommitments),
            b"74" => Some(ContractWorkAuthorizedDefinitized),
            b"75" => Some(ContractWorkAuthorizedNotDefinitized),
            b"76" => Some(ContractWorkAuthorizedTotal),
            b"77" => Some(ForecastOfWorkNotYetAuthorized),
            b"78" => Some(ForecastOfWorkAllOther),
            b"79" => Some(ForecastOfWorkTotal),
            b"80" => Some(FundingTotalRequirements),
            b"81" => Some(FundsCarryover),
            b"82" => Some(NetFundsRequired),
            b"83" => Some(Code83),
            b"84" => Some(Code84),
            b"85" => Some(BestCaseEstimate),
            b"86" => Some(WorstCaseEstimate),
            b"87" => Some(MostLikelyEstimate),
            b"88" => Some(Code88),
            b"89" => Some(Code89),
            b"90" => Some(Code90),
            b"91" => Some(SiteValueAmount),
            b"92" => Some(Compensation),
            b"93" => Some(Contribution),
            b"94" => Some(DeathBenefit),
            b"95" => Some(DeathBenefitDecrement),
            b"96" => Some(EmployeeAccountBalance),
            b"97" => Some(LoanRepayment),
            b"98" => Some(PriorW2),
            b"99" => Some(SinglePremium),
            b"A" => Some(AdjustedChargebackClaimAmount),
            b"A0" => Some(AssistantshipFromAdmittingEducationalInstitution),
            b"A1" => Some(AverageNegativeLedgerBalance),
            b"A2" => Some(AveragePositiveCollectedBalance),
            b"A3" => Some(AverageNegativeCollectedBalance),
            b"A4" => Some(AveragePositiveLedgerBalance),
            b"A5" => Some(DisallowedEstimated),
            b"A6" => Some(DisallowedActual),
            b"A7" => Some(NoncoveredChargesEstimated),
            b"A8" => Some(NoncoveredChargesActual),
            b"A9" => Some(AllowedEstimated),
            b"AA" => Some(Allocated),
            b"AA1" => Some(ExcessFunds),
            b"AA2" => Some(CumulativeTotal),
            b"AA3" => Some(ReimbursableAmount),
            b"AA4" => Some(TotalReimbursableAmount),
            b"AA5" => Some(DirectCitationAmount),
            b"AA6" => Some(TotalDirectCitationFunds),
            b"AA7" => Some(ChargeableAmount),
            b"AAA" => Some(TemporaryTermCoverage),
            b"AAB" => Some(ConditionalReceiptCoverage),
            b"AAC" => Some(BindingInterimCoverage),
            b"AAD" => Some(ApplicationAmount),
            b"AAE" => Some(ApprovedAmount),
            b"AAF" => Some(UltimateFaceAmount),
            b"AAG" => Some(RequestedAmountFromAllReinsurers),
            b"AAH" => Some(ReplacementAmount),
            b"AAI" => Some(ScheduledContribution),
            b"AAJ" => Some(ScheduledDisbursement),
            b"AAK" => Some(ShortTermInvestment),
            b"AAL" => Some(SubsequentContribution),
            b"AAM" => Some(SubsequentDistribution),
            b"AAN" => Some(TaxFederal),
            b"AAO" => Some(TaxLocal),
            b"AAP" => Some(TaxState),
            b"AAQ" => Some(TrustFund),
            b"AAR" => Some(CapitalLeases),
            b"AAS" => Some(Surplus),
            b"AAT" => Some(RestatedAssets),
            b"AAU" => Some(OwingToClients),
            b"AAV" => Some(ShareholderLoans),
            b"AAW" => Some(AccumulatedDeficit),
            b"AAX" => Some(LoanFromParentCompany),
            b"AAY" => Some(ContributionNotSubjectToRepayment),
            b"AAZ" => Some(IncomeBeforeDepreciation),
            b"AB" => Some(AdjustedCollectedBalance),
            b"ABA" => Some(IncomeAfterDepreciation),
            b"ABB" => Some(CodeABB),
            b"ABC" => Some(InterestExpenses),
            b"ABD" => Some(CodeABD),
            b"ABE" => Some(CodeABE),
            b"ABF" => Some(IncomeBeforeAllocations),
            b"ABG" => Some(IncomeFromSaleOfFixedAssets),
            b"ABH" => Some(ContributionToGroup),
            b"ABI" => Some(DeferredTaxAssets),
            b"ABJ" => Some(BlockedAccounts),
            b"ABK" => Some(NonTaxedReserves),
            b"ABL" => Some(PledgedAssets),
            b"ABM" => Some(RestrictedEquity),
            b"ABN" => Some(NonRestrictedEquity),
            b"ABO" => Some(DepreciableAssets),
            b"ABP" => Some(TaxableAssets),
            b"ABQ" => Some(IncomeFromBusiness),
            b"ABR" => Some(IncomeSubjectToTaxes),
            b"ABS" => Some(TaxableAmountOfRealEstate),
            b"ABT" => Some(EndingPrincipalBalance),
            b"ABU" => Some(AverageDailyPrincipalBalance),
            b"ABV" => Some(InterestAmount),
            b"ABW" => Some(AdjustmentsForDifferenceInAverageDailyPrincipalBalance),
            b"ABX" => Some(BeginningPrincipalBalance),
            b"ABY" => Some(LoanPrincipalDisbursements),
            b"ABZ" => Some(PrincipalIncreases),
            b"AC" => Some(AverageCollectedBalance),
            b"ACA" => Some(PrincipalOfLoansPurchased),
            b"ACB" => Some(PrincipalCured),
            b"ACC" => Some(PrincipalSold),
            b"ACD" => Some(PrincipalInsuranceClaims),
            b"ACE" => Some(PrincipalGuaranteeVoided),
            b"ACF" => Some(PrincipalPaidByBorrowers),
            b"ACG" => Some(LoansInSchoolAndGrace),
            b"ACH" => Some(LoansInAuthorizedDeferment),
            b"ACI" => Some(LoansRepayOrForebearanceCurrentOrLessThan31Days),
            b"ACJ" => Some(LoansRepayOrForebearance31To60DaysPastDue),
            b"ACK" => Some(LoansRepayOrForebearance61To90DaysPastDue),
            b"ACL" => Some(LoansRepayOrForebearance91To120DaysPastDue),
            b"ACM" => Some(LoansRepayOrForebearance121To180DaysPastDue),
            b"ACN" => Some(LoansRepayOrForebearance181To270DaysPastDue),
            b"ACO" => Some(LoansRepayOrForebearance271OrMoreDaysPastDue),
            b"ACP" => Some(CodeACP),
            b"ACQ" => Some(AgentSales),
            b"ACR" => Some(AmountInvolved),
            b"ACS" => Some(AssignedCapital),
            b"ACT" => Some(CreditLineUtilized),
            b"ACU" => Some(DirectSales),
            b"ACV" => Some(EarningsPerShare),
            b"ACW" => Some(Inheritance),
            b"ACX" => Some(InvestedCapital),
            b"ACY" => Some(LoanFromFamilyMembers),
            b"ACZ" => Some(NonDepreciableAssets),
            b"AD" => Some(AdjustedTotal),
            b"ADA" => Some(PartiallyPaidAmountPerShare),
            b"ADB" => Some(PendingOrders),
            b"ADC" => Some(PersonalLoan),
            b"ADD" => Some(PlantAndMachinery),
            b"ADE" => Some(PreTaxLoss),
            b"ADF" => Some(PreTaxProfit),
            b"ADG" => Some(RegisteredCapital),
            b"ADH" => Some(RevaluationReserves),
            b"ADI" => Some(SocialCapital),
            b"ADJ" => Some(StatutoryProfit),
            b"ADK" => Some(TrainingPay),
            b"ADL" => Some(RetroactivePay),
            b"ADM" => Some(ExpectedReimbursementAmount),
            b"ADN" => Some(PermitCost),
            b"ADO" => Some(Minimum),
            b"ADP" => Some(AdditionalAmountToMeetMinimum),
            b"ADQ" => Some(LaborPerHour),
            b"ADR" => Some(NonRecoverableDepreciation),
            b"ADS" => Some(RecoverableDepreciation),
            b"ADT" => Some(Overhead),
            b"ADU" => Some(IndemnityBenefit),
            b"ADW" => Some(ReplacementCostOfRepairs),
            b"ADX" => Some(ActualCashValueOfRepairs),
            b"ADY" => Some(RecoverableDepreciationOfRepairs),
            b"ADZ" => Some(NonRecoverableDepreciationOfRepairs),
            b"AE" => Some(Arrearage),
            b"AEA" => Some(NonIndemnityBenefit),
            b"AEB" => Some(ActualCashValueOfBuilding),
            b"AEC" => Some(GovernmentShare),
            b"AED" => Some(ContractorShare),
            b"AEE" => Some(AwardFee),
            b"AEF" => Some(BaseFee),
            b"AEG" => Some(TargetProfitFloor),
            b"AEH" => Some(TargetProfitCeiling),
            b"AEI" => Some(LaborPerDay),
            b"AEJ" => Some(DifferenceInInterestDue),
            b"AEK" => Some(DifferenceInPrepaymentPenalty),
            b"AEL" => Some(DifferenceInPrincipalDue),
            b"AEM" => Some(AppropriationOfRetainedEarningsLessReversals),
            b"AEN" => Some(Appropriations),
            b"AEO" => Some(BillingsAndCostsProfitDifferential),
            b"AEP" => Some(CommonStockParValue),
            b"AEQ" => Some(CostOfServicesRendered),
            b"AER" => Some(Creditors),
            b"AES" => Some(DeclaredProfit),
            b"AET" => Some(DiscountedNotes),
            b"AEU" => Some(EndorsedNotes),
            b"AEV" => Some(GeneralReserves),
            b"AEW" => Some(ImportVolume),
            b"AEX" => Some(IncomeTaxCredit),
            b"AEY" => Some(LongTermDeposits),
            b"AEZ" => Some(LongTermLoans),
            b"AF" => Some(AverageFloat),
            b"AFA" => Some(MinorityInterest),
            b"AFB" => Some(NonOperatingExpense),
            b"AFC" => Some(NonOperatingIncome),
            b"AFD" => Some(OperatingProfitOrLoss),
            b"AFE" => Some(PreferredStockParValue),
            b"AFF" => Some(ProfitAfterTaxAndMinorityInterest),
            b"AFG" => Some(RetainedEarningsToBeAppropriated),
            b"AFH" => Some(RevaluationSurplusOrDeficit),
            b"AFI" => Some(ReversalOfVoluntaryEarnedSurplus),
            b"AFJ" => Some(SharePrice),
            b"AFK" => Some(ShortTermDeposits),
            b"AFL" => Some(ShortTermLoans),
            b"AFM" => Some(TaxProvisions),
            b"AFN" => Some(UnallocatedProfit),
            b"AFO" => Some(VoluntaryEarnedSurplus),
            b"AFP" => Some(CalculatedWeeklyCompensationAmount),
            b"AFQ" => Some(BenefitTypeGrossWeeklyAmount),
            b"AFR" => Some(BenefitTypeNetWeeklyAmount),
            b"AFS" => Some(EmployeeGrossWage),
            b"AFT" => Some(GarageGrossWages),
            b"AFU" => Some(OfficerCompensationActualFlat),
            b"AFV" => Some(OfficerCompensationStatutoryMaximum),
            b"AFW" => Some(OfficerCompensationStatutoryMinimum),
            b"AFY" => Some(PreviousBalance),
            b"AFZ" => Some(DisputedAmount),
            b"AG" => Some(AdjustedGrossIncome),
            b"AGA" => Some(NonOperatingIncomeOrExpense),
            b"AGB" => Some(OperatingIncomeOrExpense),
            b"AGC" => Some(IncomeOrExpense),
            b"AGD" => Some(PurchaseAuthority),
            b"AGE" => Some(CapitalDecrease),
            b"AGF" => Some(CapitalIncrease),
            b"AGG" => Some(DeedCapital),
            b"AGH" => Some(TransferredAmount),
            b"AGI" => Some(UnadjustedSalesPrice),
            b"AGJ" => Some(SalesConcessions),
            b"AGK" => Some(PropertyValue),
            b"AGM" => Some(PartialReleaseAmount),
            b"AGN" => Some(LienFilingFee),
            b"AGO" => Some(AdditionalRepairPrice),
            b"AGP" => Some(QualifiedTuitionAndRelatedExpenses),
            b"AGQ" => Some(QualifiedFinancialAssistance),
            b"AGR" => Some(AggregateReimbursementsOrRefunds),
            b"AGS" => Some(NewLoanBalance),
            b"AGT" => Some(RawMaterialPurchases),
            b"AGU" => Some(WorkInProgressPurchases),
            b"AGV" => Some(OperatingCashFlow),
            b"AGW" => Some(PaymentsForOutsideWork),
            b"AGX" => Some(SetAsideForProvisions),
            b"AGY" => Some(FinancialIncomeOrExpense),
            b"AGZ" => Some(ExtraordinaryIncomeOrExpense),
            b"AH" => Some(LoanBalanceDifference),
            b"AHA" => Some(UnfinishedProductionCarriedForward),
            b"AHB" => Some(InstallationMaterialsCost),
            b"AHC" => Some(NewMortgageAmount),
            b"AHD" => Some(CapitalizedAssets),
            b"AHE" => Some(ProfitReserves),
            b"AHF" => Some(ShareInProfitOrLossInOtherCompanies),
            b"AHG" => Some(MonetaryCorrection),
            b"AHI" => Some(ScheduledRepaymentAmount),
            b"AHJ" => Some(AmountsPlacedWithOtherBanks),
            b"AHK" => Some(DueFromParentCompany),
            b"AHL" => Some(OwingFromSubsidiaryCompanies),
            b"AHM" => Some(CertificatesOfDepositAmount),
            b"AHN" => Some(PubliclyTradedSharesAmount),
            b"AHO" => Some(NonPubliclyTradedSharesAmount),
            b"AHP" => Some(TradingSecuritiesAmount),
            b"AHQ" => Some(InvestmentSecuritiesAmount),
            b"AHR" => Some(EarningsPerShareMinusDividends),
            b"AHS" => Some(ActivePartnerCapital),
            b"AI" => Some(SaleAmount),
            b"AJ" => Some(FundsHeldByMortgagee),
            b"AJC" => Some(AdjustedClaim),
            b"AK" => Some(AttorneyFees),
            b"AL" => Some(AverageLedgerBalance),
            b"AM" => Some(AmountFinanced),
            b"AN" => Some(BankruptcyFee),
            b"ANC" => Some(AccruedIncome),
            b"AO" => Some(AmountOverride),
            b"AP" => Some(AmountPriorToFractionalization),
            b"APT" => Some(AmountOfPurchaseExemptFromTaxOrFee),
            b"AQ" => Some(AveragePricePerCall),
            b"AQL" => Some(AcquisitionCostOfLenses),
            b"AR" => Some(FeesToPublicOfficialsForForeclosure),
            b"AS" => Some(AveragePricePerMinute),
            b"ASP" => Some(AnnualSalesOrPremiums),
            b"AT" => Some(TotalReceived),
            b"ATF" => Some(AmountOfTaxOrFeeExemption),
            b"AU" => Some(CoverageAmount),
            b"AV" => Some(ActualCashValue),
            b"AVE" => Some(Average),
            b"AW" => Some(ReplacementCost),
            b"AX" => Some(PreviousPrice),
            b"AY" => Some(TitleCost),
            b"AZ" => Some(OtherForeclosureAndAcquisitionExpenses),
            b"B" => Some(Estimated),
            b"B0" => Some(Bond),
            b"B1" => Some(BenefitAmount),
            b"B2" => Some(BonusesAndCommissionsDividedOver12Months),
            b"B3" => Some(BonusesDividedOver12Months),
            b"B4" => Some(BonusesAndCommissions),
            b"B5" => Some(Budgeted),
            b"B6" => Some(AllowedActual),
            b"B7" => Some(DeductibleEstimated),
            b"B8" => Some(CoInsuranceEstimated),
            b"B9" => Some(CoInsuranceActual),
            b"BA" => Some(Bargain),
            b"BAA" => Some(NetTaxableIncome),
            b"BAB" => Some(OriginalAmountOfInstrument),
            b"BAC" => Some(AdditionToTax),
            b"BAD" => Some(ReinstatementFee),
            b"BAE" => Some(PermitFeeDue),
            b"BAF" => Some(PermitFeeWithExtension),
            b"BAG" => Some(NetAnnualPermitFeeDue),
            b"BAH" => Some(PermitFeePenaltyDue),
            b"BAI" => Some(PermitFeeInterestDue),
            b"BAJ" => Some(TotalPermitFeeDue),
            b"BAK" => Some(FranchiseTax),
            b"BAL" => Some(UnclaimedFranchiseTaxCredit),
            b"BAM" => Some(NetFranchiseTaxDue),
            b"BAN" => Some(FranchiseTaxPenaltyDue),
            b"BAO" => Some(TotalFranchiseTaxDue),
            b"BAP" => Some(TotalAmountDue),
            b"BAQ" => Some(Overpayment),
            b"BAR" => Some(AmountToBeRefunded),
            b"BAS" => Some(GrossInStateReceipts),
            b"BAT" => Some(GrossReceipts),
            b"BAU" => Some(OccupationFee),
            b"BAV" => Some(TotalAssessedValue),
            b"BAW" => Some(TotalValueOfAllProperty),
            b"BAX" => Some(ValueOfInStateProperty),
            b"BAY" => Some(TotalValueOfOutOfStateProperty),
            b"BAZ" => Some(TotalParValue),
            b"BB" => Some(MortgageInsurancePremiums),
            b"BBA" => Some(TotalAssessableCapitalStock),
            b"BBB" => Some(ApportionedValue),
            b"BBC" => Some(EstimatedInStateRealPropertyValue),
            b"BBD" => Some(ParValueOfInStateAssets),
            b"BBE" => Some(InStateBusinessRevenue),
            b"BBF" => Some(SubscriptionPrice),
            b"BBG" => Some(ValueOfAuthorizedShares),
            b"BBH" => Some(OrdinanceAmount),
            b"BBI" => Some(CapitalForACooperative),
            b"BBJ" => Some(DirectorsLegalObligationDebt),
            b"BBK" => Some(SilentPartnerCapital),
            b"BC" => Some(CodeBC),
            b"BD" => Some(BalanceDue),
            b"BE" => Some(DisbursementsForAuthorizedRepair),
            b"BF" => Some(HazardInsurancePremium),
            b"BG" => Some(EvictionAttorneyFees),
            b"BH" => Some(EvictionExpenses),
            b"BI" => Some(PropertyTaxes),
            b"BJ" => Some(DisbursementsNotShownElsewhere),
            b"BK" => Some(DisbursementsForProtectionAndPreservation),
            b"BL" => Some(DisbursementsForInspectionsAndBoarding),
            b"BLD" => Some(BridgeLoanNotDeposited),
            b"BM" => Some(Adjustments),
            b"BN" => Some(RentalIncome),
            b"BO" => Some(RentalExpense),
            b"BOA" => Some(BorrowerAdvance),
            b"BOR" => Some(BorrowedAmount),
            b"BP" => Some(AverageNetCollectedBalance),
            b"BQ" => Some(Bail),
            b"BR" => Some(AdjustedInsuredLossAmount),
            b"BS" => Some(MortgageNoteInterest),
            b"BT" => Some(BankRejectTotal),
            b"BTR" => Some(Betterment),
            b"BU" => Some(OverheadCosts),
            b"BV" => Some(UncollectedInterest),
            b"BW" => Some(AmountDueFromBuyerAtClosing),
            b"BX" => Some(AmountOwedToBuyerAtClosing),
            b"BXP" => Some(BillsOfExchangePayable),
            b"BY" => Some(AdditionalClosingExpenses),
            b"BZ" => Some(DeficiencyJudgmentExpenses),
            b"C" => Some(City),
            b"C0" => Some(CurrentExpenditures),
            b"C1" => Some(CoPaymentAmount),
            b"C2" => Some(ChildRiderCoverage),
            b"C3" => Some(PriorPaymentEstimated),
            b"C4" => Some(PriorPaymentActual),
            b"C5" => Some(ClaimAmountDueEstimated),
            b"C6" => Some(ClaimAmountDueActual),
            b"C7" => Some(PayerResponsibilityEstimated),
            b"C8" => Some(PayerResponsibilityActual),
            b"C9" => Some(DisallowedCostContainmentActual),
            b"CA" => Some(ContractorCumulativeToDate),
            b"CB" => Some(CollectedBalanceRequired),
            b"CC" => Some(ChargebackClaimAmount),
            b"CD" => Some(OverpaidSection235Subsidy),
            b"CDC" => Some(CleanUpCostsAssociatedWithDeficiency),
            b"CE" => Some(SummaryAmount),
            b"CF" => Some(AppraisalFees),
            b"CG" => Some(CommissionFeesDeducted),
            b"CH" => Some(ChangeAmount),
            b"CI" => Some(FundsHeldForInsured),
            b"CJ" => Some(OtherDeductions),
            b"CJL" => Some(CodeCJL),
            b"CK" => Some(BackEndLoad),
            b"CL" => Some(OutstandingBalanceCurrentLender),
            b"CM" => Some(ClaimantRequestedTotal),
            b"CMC" => Some(CompressionCharge),
            b"CMR" => Some(CommodityRefund),
            b"CN" => Some(SpecialAssessments),
            b"CO" => Some(TaxesOnDeed),
            b"COA" => Some(CorporateAssets),
            b"CP" => Some(StatutoryDisbursements),
            b"CPO" => Some(ClosingCostsPaidByAnyOtherPartyOtherThanSellerOrBuyer),
            b"CPS" => Some(ClosingCostsPaidBySeller),
            b"CQ" => Some(NetClaimAmount),
            b"CR" => Some(ContractorAtComplete),
            b"CRC" => Some(ContainerReplacementCost),
            b"CRL" => Some(ContainerReplacementLaborCost),
            b"CRM" => Some(ContainerReplacementMaterialCost),
            b"CS" => Some(CommissionSales),
            b"CT" => Some(Contract),
            b"CU" => Some(SubcontractorCumulativeToDate),
            b"CV" => Some(SubcontractorAtComplete),
            b"CW" => Some(EarnedValue),
            b"CX" => Some(Actual),
            b"CY" => Some(CumulativeBudget),
            b"CZ" => Some(CumulativeEarnedValue),
            b"D" => Some(PayerAmountPaid),
            b"D0" => Some(AdministrationAndManagementCosts),
            b"D1" => Some(DeferredCompensationCommissions),
            b"D2" => Some(DeductibleAmount),
            b"D3" => Some(DeferredCompensationCommissionsAndBonuses),
            b"D4" => Some(DeferredCompensation),
            b"D4D" => Some(DollarForDollarDeductions),
            b"D5" => Some(DependentCareContribution),
            b"D6" => Some(DisallowedCostContainmentEstimated),
            b"D7" => Some(DispensingFee),
            b"D8" => Some(DiscountAmount),
            b"D9" => Some(CumulativeActual),
            b"DA" => Some(OriginalMortgage),
            b"DB" => Some(UnappliedSection235Funds),
            b"DC" => Some(UnappliedBuydownFund),
            b"DD" => Some(DirectDeposit),
            b"DE" => Some(EstimateOfDamage),
            b"DEF" => Some(Deferral),
            b"DEI" => Some(DelayedInterest),
            b"DF" => Some(AuthorizedBid),
            b"DFA" => Some(DeferredAssets),
            b"DG" => Some(EscrowBalance),
            b"DH" => Some(TotalDisbursements),
            b"DI" => Some(ChargeOff),
            b"DJ" => Some(LiensAmountOriginal),
            b"DK" => Some(ReleaseOfLien),
            b"DL" => Some(Debit),
            b"DLQ" => Some(DelinquentTaxes),
            b"DM" => Some(Asset),
            b"DN" => Some(Liability),
            b"DO" => Some(Satisfaction),
            b"DP" => Some(Exemption),
            b"DPF" => Some(DiscountPointsFinanced),
            b"DPN" => Some(DiscountPointsNotFinanced),
            b"DPP" => Some(DefaultedTaxPlanPayment),
            b"DQ" => Some(Settlement),
            b"DR" => Some(AlimonyExpense),
            b"DS" => Some(AlimonyIncome),
            b"DT" => Some(ChildSupportExpense),
            b"DU" => Some(ChildSupportIncome),
            b"DV" => Some(SeparateMaintenanceExpense),
            b"DVP" => Some(DevelopmentProperties),
            b"DW" => Some(SeparateMaintenanceIncome),
            b"DX" => Some(DeductibleWaived),
            b"DY" => Some(PerDayLimit),
            b"DZ" => Some(JobRelatedExpense),
            b"E" => Some(EstimatedCredit),
            b"E0" => Some(AdministrationAndManagementIndemnityCharge),
            b"E1" => Some(EmployerYearToDateContribution),
            b"E2" => Some(EmployeeAnnualPledgeAmount),
            b"E3" => Some(EmployeeCurrentContribution),
            b"E4" => Some(EmployerPledgeAmount),
            b"E5" => Some(EmployerCurrentContribution),
            b"E6" => Some(EligibleWageAmount),
            b"E7" => Some(EmployeeYearToDateContribution),
            b"E8" => Some(EducationContribution),
            b"E9" => Some(InitialFee),
            b"EA" => Some(EarningsAllowance),
            b"EAA" => Some(AdministrativeExpenses),
            b"EAB" => Some(AirTravelExpenses),
            b"EAC" => Some(AmountForgiven),
            b"EAD" => Some(AmountGuaranteed),
            b"EAE" => Some(AmountOverFairMarketValue),
            b"EAF" => Some(AmountOwed),
            b"EAG" => Some(AmountPayable),
            b"EAH" => Some(AmountRaised),
            b"EAI" => Some(AmountReceived),
            b"EAJ" => Some(AmountRefunded),
            b"EAK" => Some(AmountRescinded),
            b"EAL" => Some(AnonymousContribution),
            b"EAM" => Some(BalanceOwed),
            b"EAN" => Some(BankCharges),
            b"EAO" => Some(BankLoan),
            b"EAP" => Some(BrochureExpenses),
            b"EAQ" => Some(BusTravelExpenses),
            b"EAR" => Some(ConsultantExpenses),
            b"EAS" => Some(CorrectedAmount),
            b"EAT" => Some(DisabilityExpenses),
            b"EAU" => Some(DisposedAmount),
            b"EAV" => Some(DrawAmount),
            b"EAW" => Some(ElectionExpenses),
            b"EAX" => Some(EndorsementAmount),
            b"EAY" => Some(EntertainmentExpenses),
            b"EAZ" => Some(ExcessExpenses),
            b"EB" => Some(CodeEB),
            b"EBA" => Some(ExpectedExpenditureAmount),
            b"EBB" => Some(ExpenditureAmount),
            b"EBC" => Some(FamilyCareExpenses),
            b"EBD" => Some(FederalShareAmount),
            b"EBE" => Some(FilingFee),
            b"EBG" => Some(InKindContribution),
            b"EBH" => Some(IncurredAmount),
            b"EBI" => Some(LoanAmountPlusInterest),
            b"EBJ" => Some(LoanBalance),
            b"EBK" => Some(MatchingContribution),
            b"EBL" => Some(MeetingExpenses),
            b"EBM" => Some(MiscellaneousIncome),
            b"EBN" => Some(MiscellaneousReceipts),
            b"EBO" => Some(NewLoanAmount),
            b"EBP" => Some(NewUnpaidExpenditure),
            b"EBQ" => Some(NewsletterExpenses),
            b"EBR" => Some(NewspaperAdvertisingExpenses),
            b"EBS" => Some(NominationExpenses),
            b"EBT" => Some(NonFederalShare),
            b"EBU" => Some(OfficeExpenses),
            b"EBV" => Some(OfficeRental),
            b"EBW" => Some(OriginalAssetValue),
            b"EBX" => Some(OriginalLoanAmount),
            b"EBY" => Some(PartyExpenses),
            b"EBZ" => Some(Payment),
            b"EC" => Some(CodeEC),
            b"ECA" => Some(PersonalExpenses),
            b"ECB" => Some(PersonalFunds),
            b"ECC" => Some(PledgedAmount),
            b"ECD" => Some(PostageExpenses),
            b"ECE" => Some(PrintingExpenses),
            b"ECF" => Some(PublicFunds),
            b"ECG" => Some(RadioAdvertisingExpenses),
            b"ECH" => Some(ReimbursedAmount),
            b"ECI" => Some(ReportedAmount),
            b"ECJ" => Some(Retainer),
            b"ECK" => Some(SignExpenses),
            b"ECL" => Some(SubContractValue),
            b"ECM" => Some(TaxReceipts),
            b"ECN" => Some(TaxiTravelExpenses),
            b"ECO" => Some(TelecommunicationExpenses),
            b"ECP" => Some(TelevisionAdvertisingExpenses),
            b"ECQ" => Some(UnpaidExpenditure),
            b"ECR" => Some(UtilitiesExpenses),
            b"ECS" => Some(Total),
            b"ECT" => Some(Subtotal),
            b"ECU" => Some(GrandTotal),
            b"ECV" => Some(IncidentalExpenses),
            b"ECW" => Some(TransportationExpenses),
            b"ECX" => Some(GiftValue),
            b"ECY" => Some(FoodAndRefreshments),
            b"ECZ" => Some(PollingExpenses),
            b"ED" => Some(EstimatedCostOfAttendance),
            b"ED4" => Some(TuitionAndRequiredFees),
            b"EDB" => Some(BooksAndSupplies),
            b"EE" => Some(OtherExpense),
            b"EF" => Some(EstimatedFinancialAid),
            b"EG" => Some(OtherIncome),
            b"EH" => Some(AmountOfMortgagesAndLiens),
            b"EI" => Some(CodeEI),
            b"EIP" => Some(MaintenanceExpenseOnIncomeProducingProperty),
            b"EJ" => Some(CodeEJ),
            b"EK" => Some(NetRentalIncome),
            b"EL" => Some(PresentMarketValue),
            b"ELT" => Some(CodeELT),
            b"EM" => Some(GrossRentalIncome),
            b"EN" => Some(CancellationFee),
            b"ENP" => Some(MaintenanceExpenseOnNonIncomeProducingProperty),
            b"ENT" => Some(EntitlementAmount),
            b"EO" => Some(CapitalReserves),
            b"EP" => Some(EmployerAnnualPledgeAmount),
            b"EQ" => Some(CondominiumAssociationFees),
            b"ER" => Some(HomeownerAssociationFees),
            b"ERN" => Some(EarnestMoney),
            b"ES" => Some(MortgageInsuranceProceeds),
            b"ET" => Some(NetProceedsFromSaleOfRealEstateProperty),
            b"ETD" => Some(CodeETD),
            b"EU" => Some(CodeEU),
            b"EV" => Some(PresaleProceeds),
            b"EW" => Some(PledgedSavings),
            b"EX" => Some(AsIsBrokersOpinion),
            b"EXC" => Some(ExplorationCosts),
            b"EY" => Some(SubjectToBrokersOpinion),
            b"EZ" => Some(UniformCommercialCodeFilingOfficeFee),
            b"F" => Some(AnnualLimit),
            b"F0" => Some(CommercialStaffLaborCosts),
            b"F1" => Some(CodeF1),
            b"F2" => Some(PatientResponsibilityActual),
            b"F3" => Some(PatientResponsibilityEstimated),
            b"F4" => Some(PostageClaimed),
            b"F5" => Some(PatientAmountPaid),
            b"F6" => Some(ProviderReserves),
            b"F7" => Some(SalesTax),
            b"F8" => Some(UsualAndCustomaryChargeEstimated),
            b"F9" => Some(UsualAndCustomaryActual),
            b"FA" => Some(CoordinationFee),
            b"FAR" => Some(CodeFAR),
            b"FB" => Some(CalculationFee),
            b"FBA" => Some(FinalBalance),
            b"FC" => Some(ExpectedFamilyContribution),
            b"FD" => Some(DirectDepositFlippedToCheck),
            b"FE" => Some(Fee),
            b"FF" => Some(ApplicationFee),
            b"FG" => Some(LicensingFee),
            b"FH" => Some(RegulatoryFee),
            b"FI" => Some(FirstInterestPaymentAmount),
            b"FJ" => Some(WaiverFee),
            b"FK" => Some(OtherUnlistedAmount),
            b"FL" => Some(Float),
            b"FLA" => Some(FirstLienAdvance),
            b"FM" => Some(FairMarketValue),
            b"FN" => Some(Fine),
            b"FO" => Some(FeesPaid),
            b"FOA" => Some(ForeignAssets),
            b"FP" => Some(FeesPaidYearToDate),
            b"FQ" => Some(FirmContractorShare),
            b"FR" => Some(EstimatedGovernmentShare),
            b"FRB" => Some(FacilitiesRefund),
            b"FS" => Some(Expense),
            b"FT" => Some(EndorsementPremiumAmount),
            b"FTR" => Some(FuelTrackerRefund),
            b"FTX" => Some(FacilityTaxAmount),
            b"FU" => Some(CommercialStaffIndemnityCharge),
            b"FV" => Some(FlatFeePaidToDate),
            b"FW" => Some(FlatFeePaidCurrentMonth),
            b"FX" => Some(Endorsement),
            b"FY" => Some(FirstPayment),
            b"FZ" => Some(EarnedIncome),
            b"G" => Some(Collateral),
            b"G0" => Some(InitialAdjustmentTotal),
            b"G1" => Some(IndicatedValueBySalesComparisonApproach),
            b"G2" => Some(IndicatedValueByIncomeApproach),
            b"G3" => Some(PricePerUnitArea),
            b"G4" => Some(ReconciliationOfFinalValueEstimate),
            b"G5" => Some(EstimatedMonthlyMarketRent),
            b"G6" => Some(AdjustedSalesPrice),
            b"G7" => Some(SalesOrFinancingConcessions),
            b"G8" => Some(IndicatedValueByCostApproach),
            b"G9" => Some(AsIsValueOfSiteImprovements),
            b"GA" => Some(DepreciatedValueOfImprovements),
            b"GAR" => Some(Garnishments),
            b"GAT" => Some(AnnualTax),
            b"GB" => Some(CodeGB),
            b"GC" => Some(CodeGC),
            b"GCB" => Some(CorrectedTaxBill),
            b"GCL" => Some(GrossClaim),
            b"GD" => Some(PhysicalDepreciation),
            b"GE" => Some(FunctionalDepreciation),
            b"GF" => Some(ExternalDepreciation),
            b"GFD" => Some(GiftsNotDeposited),
            b"GFT" => Some(GiftAmount),
            b"GG" => Some(AdjustedSalesPriceOfComparableSales),
            b"GH" => Some(PredominateValue),
            b"GI" => Some(AverageCustomerIncome),
            b"GJ" => Some(AverageNeighborhoodIncome),
            b"GK" => Some(AverageCustomerPurchase),
            b"GL" => Some(WeeklyDollarSales),
            b"GM" => Some(AverageCaseSales),
            b"GN" => Some(BuyDown),
            b"GO" => Some(CreditLine),
            b"GP" => Some(AppraisalRepairAmount),
            b"GQ" => Some(BrokersOpinionRepairAmount),
            b"GR" => Some(CreditLineAvailable),
            b"GRC" => Some(GrossContribution),
            b"GRD" => Some(GrantsNotDeposited),
            b"GRR" => Some(GrossRestoration),
            b"GRT" => Some(GrantAmount),
            b"GS" => Some(SubsequentAdjustmentsTotal),
            b"GSP" => Some(GoodStandingTaxPlanPayment),
            b"GT" => Some(GoodsAndServicesTax),
            b"GTS" => Some(TotalSupplementalTaxDue),
            b"GU" => Some(TaxesPaid),
            b"GUI" => Some(TaxInstallmentDue),
            b"GUP" => Some(TaxInstallmentPaid),
            b"GUS" => Some(TotalSupplementalTaxPaid),
            b"GV" => Some(GrossValue),
            b"GW" => Some(TotalCharge),
            b"GX" => Some(TotalCredit),
            b"GY" => Some(TotalDebit),
            b"GZ" => Some(TotalFinanceCharge),
            b"H" => Some(BidAmount),
            b"H0" => Some(CodeH0),
            b"H1" => Some(LegalReserves),
            b"H2" => Some(Cancellation),
            b"H3" => Some(DepositInceptionToDate),
            b"H4" => Some(DepositYearToDate),
            b"H5" => Some(DumpInRemittance),
            b"H6" => Some(Earnings),
            b"H7" => Some(LifeInsuranceCashValue),
            b"H8" => Some(StructureValue),
            b"H9" => Some(OriginalListPrice),
            b"HA" => Some(Coin),
            b"HB" => Some(Currency),
            b"HC" => Some(USTreasuryChecks),
            b"HD" => Some(PostalMoneyOrders),
            b"HDA" => Some(HomeEquityLineOfCreditDrawAmount),
            b"HE" => Some(CityChecks),
            b"HEM" => Some(HemophiliaAdjustment),
            b"HF" => Some(OtherChecks),
            b"HG" => Some(HomeGovernmentFinancingForEducation),
            b"HH" => Some(AnnualSocialSecurityWages),
            b"HI" => Some(AnnualSocialSecurityTips),
            b"HJ" => Some(CodeHJ),
            b"HK" => Some(SocialSecurityEmployeeTaxWithheld),
            b"HL" => Some(FederalIncomeTaxWithheld),
            b"HM" => Some(AdvanceEarnedIncomeCredit),
            b"HN" => Some(Commission),
            b"HO" => Some(VacationPay),
            b"HOA" => Some(HeadOfficeAccount),
            b"HP" => Some(GrossPaySubmitted),
            b"HQ" => Some(IntersellCommissionSales),
            b"HR" => Some(TotalPayrollApproved),
            b"HS" => Some(HolidayPay),
            b"HT" => Some(OvertimePay),
            b"HU" => Some(RegularPay),
            b"HV" => Some(SickPay),
            b"HW" => Some(SpecialPay),
            b"HX" => Some(ContractPrice),
            b"HY" => Some(CommercialSpaceIncome),
            b"HZ" => Some(UtilitiesPaidByOwner),
            b"HZC" => Some(HazardousCleanUpCost),
            b"I" => Some(Interest),
            b"I0" => Some(LifeInsuranceCoverage),
            b"I1" => Some(InvestmentIncome),
            b"I2" => Some(Income),
            b"I3" => Some(CodeI3),
            b"I4" => Some(TotalEstimatedRent),
            b"I5" => Some(GrossAnnualIncome),
            b"I6" => Some(CustodiansSalary),
            b"I7" => Some(EngineersSalary),
            b"I8" => Some(ElevatorOperatorsSalary),
            b"I9" => Some(IndicatedValueByMarketApproachEstimateOfMarketValue),
            b"IA" => Some(AdjustedMonthlyRent),
            b"IB" => Some(InvestableBalance),
            b"IBL" => Some(InterBankLoans),
            b"IC" => Some(AccruedUnpaidInterestToBeCapitalized),
            b"ICR" => Some(ImbalanceChargesRefund),
            b"ID" => Some(ImportDutyAmount),
            b"IE" => Some(ExciseTaxAmount),
            b"IF" => Some(InspectionFee),
            b"IG" => Some(AdjustmentForGrossLivingArea),
            b"IGT" => Some(CodeIGT),
            b"IH" => Some(PredominantPriceHigh),
            b"II" => Some(IrregularInterestPaymentAmount),
            b"IJ" => Some(NetAdjustedMonthlyRent),
            b"IK" => Some(IndicatedMonthlyMarketRent),
            b"IL" => Some(PredominantPriceLow),
            b"IM" => Some(AdjustmentForRooms),
            b"IN" => Some(Installment),
            b"INB" => Some(InstallmentBalanceAfterTheCurrentInstallmentIsApplied),
            b"IO" => Some(AdjustmentForBedrooms),
            b"IP" => Some(InterestPayableDuringRepaymentPeriod),
            b"IPD" => Some(InterestPerDiem),
            b"IQ" => Some(ContingentDebt),
            b"IR" => Some(InsuranceRecovery),
            b"IS" => Some(IndependentScholarship),
            b"ISS" => Some(InterestSinceClaimSubmission),
            b"IT" => Some(IncentiveFee),
            b"IU" => Some(AccruedUnpaidInterestNotToBeCapitalized),
            b"IV" => Some(UtilitiesAllowance),
            b"IVP" => Some(InvestmentProperty),
            b"IW" => Some(FurnitureAllowance),
            b"IY" => Some(Debentures),
            b"IZ" => Some(AccountHighBalance),
            b"J" => Some(TrusteeFees),
            b"J0" => Some(LimitedPartnershipCapital),
            b"J1" => Some(CurrentFaceAmount),
            b"J2" => Some(OriginalFaceAmount),
            b"J3" => Some(FixedDefaultNoteHoldersAmount),
            b"J4" => Some(InitialMonthlyPayment),
            b"J5" => Some(OriginalPrincipalAndInterestPayment),
            b"J6" => Some(FinalPrincipalAndInterestPayment),
            b"J7" => Some(ConversionFee),
            b"J8" => Some(EndingBalance),
            b"J9" => Some(BeginningBalance),
            b"JA" => Some(Assessment),
            b"JB" => Some(EquityClaimedAsExempt),
            b"JC" => Some(CounterClaim),
            b"JD" => Some(WeeklyBenefit),
            b"JE" => Some(Lease),
            b"JF" => Some(AdministrativeLoad),
            b"JG" => Some(AssetCostApplicableToEntireContract),
            b"JH" => Some(AssetCostApplicableToPortionOfContract),
            b"JI" => Some(AnnualFee),
            b"JJ" => Some(CostBasis),
            b"JK" => Some(DisabilityPremium),
            b"JL" => Some(EmployeeAdditionalContribution),
            b"JM" => Some(EmployeeMatchContribution),
            b"JN" => Some(EmployerContribution),
            b"JO" => Some(FreeLookValue),
            b"JP" => Some(FreeWithdrawalValue),
            b"JQ" => Some(FrontEndLoad),
            b"JR" => Some(GuaranteedMinimumDeathBenefit),
            b"JS" => Some(InterimValue),
            b"JT" => Some(MonthlyRent),
            b"JU" => Some(Judgment),
            b"JV" => Some(LoanValue),
            b"JW" => Some(MarketValue),
            b"JX" => Some(MarketValueAdjustedValue),
            b"JY" => Some(MarketValueAdjustment),
            b"JZ" => Some(NetContractValue),
            b"K" => Some(AttorneyAndTrusteeFees),
            b"K0" => Some(DiscountedBillsNotDue),
            b"K1" => Some(UnpaidSecurityBalance),
            b"K2" => Some(TotalUnpaidSecurityBalance),
            b"K3" => Some(VeteransAffairsFundingFee),
            b"K4" => Some(InitialTargetFee),
            b"K5" => Some(MinimumFee),
            b"K6" => Some(MaximumFee),
            b"K7" => Some(Price),
            b"K8" => Some(CodeK8),
            b"K9" => Some(NewPrice),
            b"KA" => Some(EstimatedContract),
            b"KB" => Some(EstimatedNetAdjustment),
            b"KC" => Some(Obligated),
            b"KD" => Some(Undefinitized),
            b"KE" => Some(AnnualRevenue),
            b"KF" => Some(NetPaidAmount),
            b"KG" => Some(NetCollectedAmount),
            b"KH" => Some(DeductionAmount),
            b"KI" => Some(NetVarianceAmount),
            b"KJ" => Some(MinimumContractAmount),
            b"KK" => Some(ItemGrossAmount),
            b"KL" => Some(CollectedAmount),
            b"KM" => Some(DisbursedAmount),
            b"KN" => Some(GrossAmountOfPayment),
            b"KO" => Some(CommittedAmount),
            b"KP" => Some(PrincipalAndInterest),
            b"KQ" => Some(IncrementalOrderAmount),
            b"KR" => Some(LiabilityLongTerm),
            b"KS" => Some(TaxesAndInsurance),
            b"KT" => Some(DefaultPrincipal),
            b"KU" => Some(DefaultInterest),
            b"KV" => Some(LiabilityShortTerm),
            b"KW" => Some(DefaultTaxesAndInsurance),
            b"KX" => Some(MiscellaneousFeeCollections),
            b"KY" => Some(NotToExceedPrice),
            b"KZ" => Some(MortgagorsMonthlyObligations),
            b"L" => Some(Local),
            b"L0" => Some(LiquidAssets),
            b"L1" => Some(LegalContribution),
            b"L2" => Some(LeaseholdInsuranceAmount),
            b"L3" => Some(TotalUnidentifiedPaymentsRejected),
            b"L4" => Some(TotalCreditsReceived),
            b"L5" => Some(TotalDebitsReceived),
            b"L6" => Some(TotalPreAdvicesReceived),
            b"L7" => Some(TotalPrenotesReceived),
            b"L8" => Some(TotalPostAdvicesReceived),
            b"L9" => Some(TotalDebitForSettlement),
            b"LA" => Some(Definitized),
            b"LB" => Some(DefinitizedTotal),
            b"LC" => Some(LessorsCost),
            b"LD" => Some(Incremental),
            b"LDR" => Some(LandRights),
            b"LE" => Some(LoanEligibilityAmount),
            b"LF" => Some(LoanRemittanceOrRepayment),
            b"LG" => Some(LaundryIncome),
            b"LH" => Some(Baseline),
            b"LI" => Some(LineItemUnitPrice),
            b"LJ" => Some(LegalAndAudit),
            b"LK" => Some(LoanAmountRequested),
            b"LL" => Some(LumpSum),
            b"LM" => Some(Limit),
            b"LN" => Some(LienPayoff),
            b"LO" => Some(MoneyPurchase),
            b"LOW" => Some(LowerFund),
            b"LP" => Some(ListPrice),
            b"LPC" => Some(TotalSubjectPropertyLiensPaidByClosing),
            b"LPF" => Some(LeasePurchaseFunds),
            b"LPY" => Some(LeasePayments),
            b"LQ" => Some(MaximumPotentialLiability),
            b"LR" => Some(TotalCreditForSettlement),
            b"LS" => Some(NetSettlement),
            b"LSP" => {
                Some(TotalLiabilitiesToBePaidAtClosingNotIncludingSubjectPropertyLiens)
            }
            b"LST" => Some(LossOnSaleOfProperty),
            b"LT" => Some(TotalAward),
            b"LU" => Some(OptionAmount),
            b"LV" => Some(PlannedPeriodicPayment),
            b"LW" => Some(TaxAndInsuranceEscrowFundBalance),
            b"LX" => Some(LoanExpense),
            b"LY" => Some(TotalRemainingPrincipalBalanceForTheIssuer),
            b"LZ" => Some(DelinquentPayment),
            b"M" => Some(AmountDueFromBuyerAtAppraisalNoticeDate),
            b"M0" => Some(LoansFromOfficers),
            b"M1" => Some(MaximumOutOfPocketAmount),
            b"M2" => Some(MedicalContribution),
            b"M3" => Some(TaxRateExpressedAsAFlatFee),
            b"M4" => Some(MinimumAmountOfTaxToBePaid),
            b"M5" => Some(MinimumAmountToWhichTaxRateIsApplied),
            b"M6" => Some(MaximumAmountOfTaxToBePaid),
            b"M7" => Some(MaximumAmountToWhichTaxRateIsApplied),
            b"M8" => Some(MarkupAmount),
            b"M9" => Some(NetOfSurrenderWithdrawal),
            b"MA" => Some(MaximumAmount),
            b"MAD" => Some(MiscellaneousAdjustment),
            b"MB" => Some(UndistributedBudget),
            b"MC" => Some(CostOfMoney),
            b"MD" => Some(MinimumDue),
            b"ME" => Some(MinimumDefaultNoteHoldersCost),
            b"MF" => Some(AdministrativeFees),
            b"MG" => Some(MaximumLateCharge),
            b"MH" => Some(MinimumLateCharge),
            b"MI" => Some(MinimumIncentiveFee),
            b"MJ" => Some(MaximumDefaultNoteHoldersCost),
            b"MK" => Some(GrossToPay),
            b"ML" => Some(PriorNetInvoiceTotal),
            b"MM" => Some(Payout),
            b"MMA" => Some(ModifiedMortgageAmount),
            b"MN" => Some(MonthlyLimit),
            b"MO" => Some(MinimumOrderValue),
            b"MP" => Some(MonthlyPaymentAmount),
            b"MQ" => Some(CodeMQ),
            b"MR" => Some(ManagementReserve),
            b"MS" => Some(PastDueTaxesAndAssessmentRemainingUnpaid),
            b"MT" => Some(CodeMT),
            b"MU" => Some(PremiumTaxPaidOnSurrender),
            b"MV" => Some(PremiumTaxPaidUpFront),
            b"MW" => Some(SalesLoads),
            b"MX" => Some(MaximumIncentiveFee),
            b"MY" => Some(SurrenderValue),
            b"MZ" => Some(ValuationPrice),
            b"N" => Some(Net),
            b"N0" => Some(LoansOrFinancialBorrowings),
            b"N1" => Some(NetWorth),
            b"N2" => Some(IndividualIncomeTaxesAndOther),
            b"N3" => Some(CorporateIncomeAndExcessProfitsTax),
            b"N4" => Some(ExciseTaxes),
            b"N5" => Some(EstateAndGiftTaxes),
            b"N6" => Some(CarrierTaxActTaxes),
            b"N7" => Some(FederalUnemploymentTaxActTaxes),
            b"N8" => Some(MiscellaneousTaxes),
            b"N9" => Some(CodeN9),
            b"NA" => Some(NetAdjustment),
            b"NB" => Some(NetCompensationPosition),
            b"NBF" => Some(NetBenefit),
            b"NBO" => Some(NetWorthOfBusinessOwned),
            b"NC" => Some(NegativeCollectedBalance),
            b"NCT" => Some(NetContribution),
            b"ND" => Some(PerPersonMonthlyLimit),
            b"NE" => Some(NetBilled),
            b"NF" => Some(CodeNF),
            b"NG" => Some(MedicareCopayment),
            b"NH" => Some(MedicareDeductible),
            b"NI" => Some(MedicarePaid),
            b"NJ" => Some(OtherInsurancePaidAmount),
            b"NK" => Some(TotalInForceAndAppliedCoverage),
            b"NL" => Some(NegativeLedgerBalance),
            b"NM" => Some(NonCollateralizedAmount),
            b"NN" => Some(TransactionFee),
            b"NO" => Some(NonCommissionSales),
            b"NP" => Some(NetToPayTotal),
            b"NPP" => Some(NoTaxPlanPayment),
            b"NQ" => Some(AdjustedNonrecurring),
            b"NR" => Some(Nonrecurring),
            b"NRE" => Some(NetRestorationExpenses),
            b"NS" => Some(NetSavingsAmount),
            b"NT" => Some(UnitValue),
            b"NTA" => Some(NewTechnologyAdjustment),
            b"NU" => Some(ReinsuranceAmount),
            b"NV" => Some(RenewalAmount),
            b"NW" => Some(RetentionPerLife),
            b"NX" => Some(RetentionPerPolicy),
            b"NY" => Some(CodeNY),
            b"NZ" => Some(EqualizationAccount),
            b"O" => Some(CourtCost),
            b"O0" => Some(ExtraordinaryIncome),
            b"O1" => Some(AmountOfFirstMortgageBeingRefinanced),
            b"O2" => Some(OtherFamilyFinancingForEducation),
            b"O3" => Some(IntangibleAssetsWrittenOff),
            b"O4" => Some(InterestPayable),
            b"O5" => Some(InterestReceivable),
            b"O6" => Some(JointVentureResults),
            b"O7" => Some(LongTermDebt),
            b"O8" => Some(LongTermProvisions),
            b"O9" => Some(Loss),
            b"OA" => Some(PrincipalBalanceAmount),
            b"OB" => Some(OutstandingLoanBalance),
            b"OC" => Some(OpeningBankCharges),
            b"OD" => Some(DraftAmount),
            b"ODC" => Some(OdorizationCharge),
            b"OE" => Some(MiscellaneousCharges),
            b"OEQ" => Some(OfficeEquipment),
            b"OF" => Some(ContractorsOffer),
            b"OFC" => Some(OperationalFlowOrderCharge),
            b"OFR" => Some(OperationalFlowOrderRefund),
            b"OG" => Some(CableCharge),
            b"OH" => Some(HandlingCharges),
            b"OI" => Some(NonCommissionCharges),
            b"OJ" => Some(Merchandise),
            b"OK" => Some(LetterOfCreditAmount),
            b"OL" => Some(OutstandingBalanceOtherLender),
            b"OLA" => Some(OtherLiabilityAmounts),
            b"OM" => Some(OtherMonthlyIncome),
            b"ON" => Some(NegotiatingBankCharges),
            b"ONR" => Some(OperationalNoticeRefund),
            b"OO" => Some(Overdrafts),
            b"OP" => Some(OriginalPaymentTotal),
            b"OQ" => Some(PayrollCosts),
            b"OR" => Some(LetterOfCreditRemainingAmount),
            b"OS" => Some(OtherSalaries),
            b"OT" => Some(CommissionAmendmentCharges),
            b"OU" => Some(Profit),
            b"OUT" => Some(InpatientOutlierAdjustment),
            b"OV" => Some(ProfitAndLossDeficit),
            b"OW" => Some(ProfitAfterExtraordinaryItemsAndBeforeTax),
            b"OX" => Some(ProfitAfterTaxAndBeforeExtraordinaryItems),
            b"OY" => Some(PaymentCommission),
            b"OZ" => Some(ProfitDistributedToEmployees),
            b"P" => Some(Penalty),
            b"P0" => Some(ParentalFinancingForEducation),
            b"P1" => Some(PartnersCalendarYearSalary),
            b"P2" => Some(PriorPlanYearGrossSalary),
            b"P3" => Some(PremiumAmount),
            b"P4" => Some(PriorYearsWage),
            b"P5" => Some(PartnersTaxYearSalary),
            b"P6" => Some(PremiumDue),
            b"P7" => Some(PartnersK1TaxYearAmount),
            b"P8" => Some(PartnersK1CalendarYearAmount),
            b"P9" => Some(CurrentMortgagePrincipalBalance),
            b"PA" => Some(PaymentCancellationTotal),
            b"PAD" => Some(PolicyAdvance),
            b"PAM" => Some(MinimumDeliveryPurchaseAmount),
            b"PAN" => Some(PenaltyAndInterest),
            b"PB" => Some(BilledAmount),
            b"PBG" => Some(CodePBG),
            b"PC" => Some(PositiveCollectedBalance),
            b"PCA" => Some(ProcessingAllowance),
            b"PCC" => Some(PriorContractCostBasis),
            b"PCP" => Some(PreviousClaimPayments),
            b"PCS" => Some(PriorContractSurrenderCharge),
            b"PCV" => Some(PriorContractValue),
            b"PD" => Some(Credit),
            b"PE" => Some(PlanPeriodElection),
            b"PEX" => Some(CodePEX),
            b"PF" => Some(Principal),
            b"PFC" => Some(PortFacilityCharge),
            b"PG" => Some(Payoff),
            b"PGR" => Some(ProposedGrossRentForTheSubjectProperty),
            b"PH" => Some(PerOccurrenceDeductible),
            b"PI" => Some(PerOccurrenceMonthlyLimit),
            b"PJ" => Some(PastDue),
            b"PK" => Some(PhotographFee),
            b"PL" => Some(PositiveLedgerBalance),
            b"PM" => Some(LastPremiumAmount),
            b"PN" => Some(PriorGrossInvoiceTotal),
            b"PO" => Some(PercentOverride),
            b"PP" => Some(PaymentPriorToAdvance),
            b"PPN" => Some(PendingNetSaleProceedsFromNonRealEstateAssets),
            b"PPR" => Some(PendingNetSaleProceedsFromRealEstateAssets),
            b"PQ" => Some(AdvanceAmount),
            b"PR" => Some(PerOccurrenceLimit),
            b"PRD" => Some(PropertyDamage),
            b"PRL" => Some(PartialPayrollPayment),
            b"PS" => Some(PerOccurrencePerDayLimit),
            b"PT" => Some(PerOccurrenceAggregateLimit),
            b"PU" => Some(UnsecuredPriorityClaim),
            b"PV" => Some(PrepetitionCharges),
            b"PW" => Some(PerOccurrenceMaximumPerWeekLimit),
            b"PX" => Some(PerPersonMaximumPerWeekLimit),
            b"PY" => Some(PerPersonPerDayLimit),
            b"PZ" => Some(OriginalPrincipalBalance),
            b"Q" => Some(AmountOwedToBuyerAtAppraisalNoticeDate),
            b"Q0" => Some(LoansToAffiliatedCompanies),
            b"Q1" => Some(Proposed),
            b"Q2" => Some(CodeQ2),
            b"Q3" => Some(CodeQ3),
            b"Q4" => Some(TotalPrenotesAccepted),
            b"Q5" => Some(TotalPrenotesRejected),
            b"Q6" => Some(AutomaticPremiumDeduction),
            b"Q7" => Some(TotalPostAdvicesAccepted),
            b"Q8" => Some(TotalPostAdvicesRejected),
            b"Q9" => Some(CashWithApplication),
            b"QA" => Some(Combined),
            b"QB" => Some(CreditCard),
            b"QC" => Some(DepositFund),
            b"QD" => Some(DirectBilling),
            b"QE" => Some(DiscPremium),
            b"QF" => Some(CodeQF),
            b"QG" => Some(GovernmentAllotment),
            b"QH" => Some(InitialPremium),
            b"QI" => Some(IndividualRetirementAccount60DayRollover),
            b"QJ" => Some(IndividualRetirementAccountDirectTransfer),
            b"QK" => Some(IndividualRetirementAccountRegularContribution),
            b"QL" => Some(KeoghHr10),
            b"QM" => Some(KeoghHr10Transfer),
            b"QN" => Some(CodeQN),
            b"QO" => Some(ListBilling),
            b"QP" => Some(ModalPremium),
            b"QQ" => Some(PayrollTaxes),
            b"QR" => Some(ParkingIncome),
            b"QS" => Some(CodeQS),
            b"QT" => Some(PacPreAuthorizedCheck),
            b"QU" => Some(PayrollDeduction),
            b"QV" => Some(Pension),
            b"QW" => Some(PremiumReceivedWithApplication),
            b"QX" => Some(ProfitSharingTrust),
            b"QY" => Some(Qualified),
            b"QZ" => Some(PaymentAmount),
            b"R" => Some(SpendDown),
            b"R0" => Some(LoansToParticipants),
            b"R1" => Some(CodeR1),
            b"R2" => Some(ContingentSecuredDebt),
            b"R3" => Some(DisputedSecuredDebt),
            b"R4" => Some(UnliquidatedSecuredDebt),
            b"R5" => Some(CodeR5),
            b"R6" => Some(ContingentUnsecuredDebt),
            b"R7" => Some(DisputedUnsecuredDebt),
            b"R8" => Some(UnliquidatedUnsecuredDebt),
            b"R9" => Some(AtTimeOfFiling),
            b"RA" => Some(AcceleratedRoyalty),
            b"RB" => Some(PerPersonDeductible),
            b"RC" => Some(RefundCheck),
            b"RD" => Some(PerPersonLimit),
            b"RDS" => Some(ReservationDemandStorage),
            b"RDT" => Some(ReservationDemandTransportation),
            b"RE" => Some(RoyaltyDue),
            b"RET" => Some(DepositValue),
            b"RF" => Some(Restitution),
            b"RG" => Some(BudgetedRedemption),
            b"RH" => Some(PerPersonAggregateLimit),
            b"RI" => Some(ResidualValue),
            b"RJ" => Some(RateAmount),
            b"RK" => Some(ProvisionForLongTermDepreciation),
            b"RL" => Some(RegularRemittance),
            b"RM" => Some(RemittanceRefund),
            b"RN" => Some(ResidentManagersSalary),
            b"RO" => Some(Provisions),
            b"RP" => Some(Repair),
            b"RPC" => Some(RepackagingCost),
            b"RQ" => Some(RecommendedAmount),
            b"RR" => Some(ReserveRequirementAmount),
            b"RS" => Some(Reserves),
            b"RT" => Some(LastPayment),
            b"RU" => Some(TotalDebitsRejected),
            b"RV" => Some(TotalPaymentsRejected),
            b"RW" => Some(TotalDelinquency),
            b"RX" => Some(TotalPreAdvicesAccepted),
            b"RY" => Some(TotalPreAdvicesRejected),
            b"RZ" => Some(LendersTotalDelinquency),
            b"S" => Some(SubmittedChargebackClaimAmount),
            b"S0" => Some(SelfFinancingForEducation),
            b"S1" => Some(SalaryAmount),
            b"S2" => Some(SalaryWithBonuses),
            b"S3" => Some(SalaryWithCommissions),
            b"S4" => Some(SalaryWithSubchapterSCorporationIncome),
            b"S5" => Some(SalaryWithPartnersBonuses),
            b"S6" => Some(SubchapterSCorporation),
            b"S7" => Some(SoleProprietorship),
            b"S8" => Some(PeriodRental),
            b"S9" => Some(SecuredClaimAllowed),
            b"SA" => Some(CampaignSummaryAmount),
            b"SAA" => Some(SystemAdjustedAmount),
            b"SAG" => Some(CodeSAG),
            b"SAL" => Some(Salvage),
            b"SB" => Some(StatedAmount),
            b"SC" => Some(TotalServiceCharge),
            b"SCT" => Some(StateCareTax),
            b"SD" => Some(SalesCharge),
            b"SE" => Some(ServiceChargesWhichCannotBeCompensatedByBalances),
            b"SF" => Some(ScholarshipFromAdmittingEducationalInstitution),
            b"SFD" => Some(SecuredBorrowedFundsNotDeposited),
            b"SG" => Some(SponsorFinancingForEducation),
            b"SH" => Some(SurrenderCharge),
            b"SI" => Some(SubsequentInterestPaymentAmount),
            b"SJ" => Some(SurrenderFull),
            b"SK" => Some(SurrenderPartial),
            b"SL" => Some(SecurityPersonnelsSalary),
            b"SM" => Some(Supplemental),
            b"SM1" => Some(InsuranceValue),
            b"SM2" => Some(DeclaredValue),
            b"SM3" => Some(ShipmentValue),
            b"SM4" => Some(PayOnDelivery),
            b"SM5" => Some(LandedCostValue),
            b"SN" => Some(SalesAdministrationExpense),
            b"SO" => Some(SpecialCreditorsAmount),
            b"SOF" => Some(Setoff),
            b"SP" => Some(SalesPrice),
            b"SQ" => Some(SpecialDebtorsAmount),
            b"SR" => Some(SecuredClaim),
            b"SRD" => {
                Some(SettlementRefundAsApprovedByTheFederalEnergyRegulatoryCommission)
            }
            b"SS" => Some(CampaignSummaryAmountToBeShared),
            b"SSC" => Some(SharesInSubsidiaryCompanies),
            b"ST" => Some(State),
            b"SU" => Some(Surcharge),
            b"SV" => Some(FixedMonthlyPrincipalPayment),
            b"SW" => Some(BaseAwardFee),
            b"SX" => Some(SeveranceTax),
            b"SY" => Some(InitialBuydownBalance),
            b"SZ" => Some(CertificationFee),
            b"T" => Some(Tax),
            b"T0" => Some(ThirdPartyGovernmentFinancingForEducation),
            b"T1" => Some(Teacher),
            b"T2" => Some(TotalClaimBeforeTaxes),
            b"T3" => Some(TotalSubmittedCharges),
            b"T4" => Some(TotalCurrentBalance),
            b"T5" => Some(TotalClaims),
            b"T6" => Some(Claim),
            b"T7" => Some(TotalCreditsAccepted),
            b"T8" => Some(TotalCreditsRejected),
            b"T9" => Some(TotalDebitsAccepted),
            b"TA" => Some(TotalAnnualSales),
            b"TB" => Some(TotalAnnualSalesToCustomer),
            b"TBC" => Some(TotalBuyerClosingCosts),
            b"TC" => Some(ProposedCost),
            b"TCS" => Some(TotalCommissionFromPrimaryAndSecondarySources),
            b"TD" => Some(ProposedProfit),
            b"TDA" => Some(TotalDepositoryAccounts),
            b"TE" => Some(ProposedFee),
            b"TEN" => Some(TotalMaintenanceExpenseOnAllNonIncomeProducingProperties),
            b"TEP" => Some(TotalMaintenanceExpenseOnAllIncomeProducingProperties),
            b"TF" => Some(TotalProposedPrice),
            b"TG" => Some(AlternateProposedPrice),
            b"TGD" => Some(TotalGiftsNotDeposited),
            b"TH" => Some(TotalClaimAllowed),
            b"TI" => Some(TitleInsuranceAmountOnLoan),
            b"TIS" => Some(TotalSelfEmployedIncomeFromPrimaryAndSecondarySources),
            b"TJ" => Some(TimeAndExpensePaidToDate),
            b"TK" => Some(TotalAmountOfContract),
            b"TL" => Some(TotalPriorLoanAmountOwed),
            b"TLA" => Some(TotalOtherLiquidAssets),
            b"TLV" => Some(TotalLifeInsuranceNetCashValue),
            b"TM" => Some(TimeAndExpensePaidCurrentMonth),
            b"TMM" => Some(TotalMonetaryMarkupAmount),
            b"TO" => Some(TelephoneOperatorsSalary),
            b"TOL" => Some(TotalOmittedLiabilities),
            b"TP" => Some(TotalPaymentAmount),
            b"TPA" => Some(TotalPreviousAdjustedClaim),
            b"TPR" => Some(TotalNetProceedsFromRealEstateAssets),
            b"TPS" => Some(TotalPaidAsSubmitted),
            b"TQ" => Some(SubsidiesForOperatingCosts),
            b"TR" => Some(TargetCost),
            b"TRF" => Some(TotalRetirementFunds),
            b"TRI" => Some(TotalNonRentalIncome),
            b"TRL" => Some(TotalResubordinatedLiabilities),
            b"TRP" => Some(TotalLiabilitiesForRentalProperties),
            b"TS" => Some(TotalSales),
            b"TSB" => Some(TotalStocksAndBonds),
            b"TT" => Some(TotalTransactionAmount),
            b"TU" => Some(TransportationCostPerUnitOfMeasure),
            b"TV" => Some(LevelOfPremiumInsuranceRetention),
            b"TW" => Some(TechniciansIndemnityProvision),
            b"TX" => Some(TotalToDate),
            b"TY" => Some(TotalAtComplete),
            b"TZ" => Some(TransportationCostTotal),
            b"U" => Some(Underpayment),
            b"U0" => Some(USGovernmentFinancingForEducation),
            b"U1" => Some(CodeU1),
            b"U2" => Some(IngredientCostClaimed),
            b"U3" => Some(MiscellaneousExpenses),
            b"U4" => Some(PresentValueOfLot),
            b"U5" => Some(CostOfImprovements),
            b"U6" => Some(CodeU6),
            b"U7" => Some(Land),
            b"U8" => Some(Refinance),
            b"U8L" => Some(TaxPaymentRefinancedBySameLender),
            b"U9" => Some(EstimatedPrepaidItems),
            b"UA" => Some(UnliquidatedAmount),
            b"UAA" => Some(UserAdjustedAmount),
            b"UAR" => Some(CodeUAR),
            b"UB" => Some(UnpaidPrincipalBalance),
            b"UC" => Some(UnspecifiedAggregateLimit),
            b"UD" => Some(CodeUD),
            b"UE" => Some(MortgageInsurance),
            b"UEP" => Some(UpdatedExpensesOnPresale),
            b"UF" => Some(CodeUF),
            b"UFD" => Some(UnsecuredBorrowedFundsNotDeposited),
            b"UG" => Some(TotalUnpaidPrincipalBalanceForStaffordLoans),
            b"UH" => Some(SubordinateFinancing),
            b"UHI" => Some(UnsecuredHomeImprovements),
            b"UI" => Some(TotalCosts),
            b"UIP" => Some(UpdatedInterestOnPresale),
            b"UJ" => Some(OtherCredits),
            b"UK" => Some(CodeUK),
            b"UL" => Some(MortgageInsuranceFinanced),
            b"UM" => Some(TotalLoanAmount),
            b"UN" => Some(CodeUN),
            b"UNK" => Some(UnknownTaxPlanPayment),
            b"UO" => Some(CashFromOrToBorrower),
            b"UP" => Some(TotalUnpaidPrincipalBalanceForParentalLoansForStudents),
            b"UPF" => Some(UpperFund),
            b"UQ" => Some(MonthlyIncome),
            b"UR" => Some(UnearnedIncome),
            b"US" => Some(TotalUnpaidPrincipalBalanceForSupplementalLoansForStudents),
            b"UT" => Some(ValueAddedSales),
            b"UU" => Some(ClearingHouseSettlement),
            b"UV" => Some(Drawback),
            b"UW" => Some(TotalMonthlyLiabilities),
            b"UX" => Some(CodeUX),
            b"UY" => Some(TotalAssets),
            b"UZ" => Some(TotalLiquidAssets),
            b"V" => Some(CostOfDeficiency),
            b"V0" => Some(ValueAdded),
            b"V1" => Some(TaxAndInsuranceEscrowFund),
            b"V2" => Some(InterestDueToInvestor),
            b"V3" => Some(TotalPrincipalDueToTheInvestor),
            b"V4" => Some(TotalInterestDueToTheInvestor),
            b"V5" => Some(TotalCurtailmentDueToTheInvestor),
            b"V6" => Some(TotalPrincipalPayoffAndRepurchaseDueToTheInvestor),
            b"V7" => Some(TotalInterestPayoffAndRepurchaseDueToTheInvestor),
            b"V8" => Some(ActualOutstandingPrincipalBalance),
            b"V9" => Some(FaceAmount),
            b"VA" => Some(CodeVA),
            b"VB" => Some(TotalNonLiquidAssets),
            b"VC" => Some(Authorized),
            b"VD" => Some(ActualPersonDayRate),
            b"VE" => Some(EstimatedPersonDayRate),
            b"VES" => Some(VestedEarnedUpperFund),
            b"VF" => Some(TotalMonthlyExpenses),
            b"VG" => Some(CurrentMonthlyPrincipalAndInterest),
            b"VH" => Some(LevyAmount),
            b"VI" => Some(CurrentSupport),
            b"VJ" => Some(PastDueSupport),
            b"VK" => Some(MedicalSupport),
            b"VL" => Some(NetNegativeAmortizationAmount),
            b"VM" => Some(WithholdFromWages),
            b"VN" => Some(CommissionBasis),
            b"VO" => Some(CommissionEarned),
            b"VP" => Some(CurrentMonthlyPayment),
            b"VQ" => Some(CommissionNetted),
            b"VR" => Some(TotalMonthlyDebt),
            b"VRS" => Some(VolumetricReservation),
            b"VS" => Some(OtherFinancingPayment),
            b"VSI" => Some(ValueOfSecuritiesAtIssueDate),
            b"VSM" => Some(ValueOfSecuritiesAtMaturity),
            b"VT" => Some(CurrentValue),
            b"VU" => Some(ClosingCost),
            b"VV" => Some(CapitalizedMortgageAmount),
            b"VW" => Some(FirstMortgageMonthlyPrincipalAndInterest),
            b"VX" => Some(InterestAmountPaidToDate),
            b"VY" => Some(MinimumTransfer),
            b"VZ" => Some(MaximumTransfer),
            b"W" => Some(DeficiencyJudgmentFees),
            b"W0" => Some(TradeDebtors),
            b"W1" => Some(W2),
            b"W2" => Some(W2WithBonuses),
            b"W3" => Some(W2WithDeferredCompensation),
            b"W4" => Some(W2WithoutBonuses),
            b"W5" => Some(DepositSubTotal),
            b"W6" => Some(DirectRollover),
            b"W7" => Some(DirectTransfer),
            b"W8" => Some(Discounted),
            b"W9" => Some(SecondaryFinance),
            b"WA" => Some(MinimumDeposit),
            b"WB" => Some(SubAgencyCompensation),
            b"WC" => Some(BuyersAgencyCompensation),
            b"WD" => Some(VariableRateCompensation),
            b"WE" => Some(CompensationBonusOnSaleOfProperty),
            b"WF" => Some(VeteransAffairsLoanGuarantee),
            b"WG" => Some(SecurityTradeAmount),
            b"WH" => Some(CodeWH),
            b"WI" => Some(OtherFinancing),
            b"WJ" => Some(DualAgencyCompensation),
            b"WK" => Some(PerWeekLimit),
            b"WL" => Some(LendersOpinionOfValue),
            b"WM" => Some(TotalOriginalPrincipalBalance),
            b"WN" => Some(OtherAgentCompensation),
            b"WO" => Some(DockUsageFee),
            b"WP" => Some(PoolUsageFee),
            b"WQ" => Some(ClubhouseFee),
            b"WR" => Some(OptionalServiceFee),
            b"WS" => Some(OtherAssociationFees),
            b"WT" => Some(CodeWT),
            b"WU" => Some(CodeWU),
            b"WV" => Some(TotalPointsPaidAtClosing),
            b"WW" => Some(AmountThatWouldHaveBeenPaidInTheAbsenceOfCapitation),
            b"WX" => Some(PointsPaidBySeller),
            b"WY" => Some(LoanWithdrawal),
            b"WZ" => Some(SeverancePay),
            b"X" => Some(DeficiencyJudgmentExpensesAndFees),
            b"X0" => Some(TrebleDamages),
            b"X1" => Some(TransferToUntaxedReserves),
            b"X2" => Some(Reissued),
            b"X3" => Some(RolloverAmount),
            b"X4" => Some(AnnualRental),
            b"X5" => Some(GrossMonthlyRent),
            b"X6" => Some(SepSelfEmployeePension),
            b"X8" => Some(FundingAmount),
            b"X9" => Some(CodeX9),
            b"XA" => Some(MaximumAwardFee),
            b"XB" => Some(MaturityValue),
            b"XC" => Some(EarnedWages),
            b"XD" => Some(BasePeriodWage),
            b"XE" => Some(Withdrawal),
            b"XF" => Some(WithdrawalInceptionToDate),
            b"XG" => Some(WithdrawalLessMarketValueAdjustment),
            b"XH" => Some(WithdrawalLessSurrender),
            b"XI" => Some(WithdrawalLessTaxes),
            b"XJ" => Some(WithdrawalYearToDate),
            b"XK" => Some(UnavailableReserves),
            b"XL" => Some(UncalledCapital),
            b"XM" => Some(UnemploymentContribution),
            b"XN" => Some(CodeXN),
            b"XO" => Some(UnpaidCapital),
            b"XP" => Some(UnsecuredLiabilities),
            b"XQ" => Some(ValueAddedTax),
            b"XR" => Some(ValueOfShares),
            b"XS" => Some(Vehicles),
            b"XT" => Some(VoluntaryReserves),
            b"XTR" => Some(Extraction),
            b"XU" => Some(Wages),
            b"XV" => Some(Withholding),
            b"XW" => Some(OriginalValue),
            b"XX" => Some(WorkingCapital),
            b"XY" => Some(SalesPricePerDwellingUnit),
            b"XZ" => Some(SalesPricePerRoom),
            b"Y" => Some(CurrentListPrice),
            b"Y0" => Some(SelfInsuranceAmount),
            b"Y1" => Some(YearToDateEligibleSalary),
            b"Y2" => Some(TotalRealEstateOwned),
            b"Y3" => Some(TotalLiabilities),
            b"Y4" => Some(TotalLiabilityMonthlyPayments),
            b"Y5" => Some(TotalRealEstateOwnedMarketValue),
            b"Y6" => Some(TotalRealEstateOwnedGrossRentalIncome),
            b"Y7" => Some(TotalRealEstateOwnedMortgagesAndLiens),
            b"Y8" => Some(TotalRealEstateOwnedMortgagePayments),
            b"Y9" => Some(TotalRealEstateOwnedMiscellaneousExpenses),
            b"YA" => Some(TotalRealEstateOwnedNetRentalIncome),
            b"YB" => Some(ActualUnpaidPrincipalBalance),
            b"YC" => Some(ScheduledUnpaidPrincipalBalance),
            b"YD" => Some(PrincipalDueToInvestor),
            b"YE" => Some(ConstantPrincipalAndInterest),
            b"YF" => Some(OtherFeeCollection),
            b"YG" => Some(BeginningScheduledUnpaidPrincipalBalance),
            b"YH" => Some(TaxAndInsurancePrincipalBalance),
            b"YI" => Some(NewPrincipalAndInterest),
            b"YJ" => Some(Curtailment),
            b"YK" => Some(PrepaymentPenalty),
            b"YL" => Some(PartialAnnuitization),
            b"YM" => Some(PartialWithdrawal),
            b"YN" => Some(CodeYN),
            b"YO" => Some(CodeYO),
            b"YP" => Some(PolicyAmount),
            b"YQ" => Some(PaymentsInAdvance),
            b"YR" => Some(PaymentsInArrears),
            b"YS" => Some(Cancelled),
            b"YT" => Some(Denied),
            b"YU" => Some(InProcess),
            b"YV" => Some(Requested),
            b"YW" => Some(Paid),
            b"YX" => Some(PaidForThisFacility),
            b"YY" => Some(Returned),
            b"YZ" => Some(TotalAggregateLimit),
            b"Z" => Some(ListPriceWhenSold),
            b"Z0" => Some(InsertionCost),
            b"Z1" => Some(RepackagingLaborCost),
            b"Z2" => Some(RepackagingMaterialCost),
            b"Z3" => Some(UnitCostOfDiscrepantMaterial),
            b"Z4" => Some(LiquidationPrincipal),
            b"Z5" => Some(RemainingPoolBalance),
            b"Z6" => Some(RemainingSecurityBalance),
            b"Z7" => Some(ProgramCost),
            b"Z8" => Some(OverrideToHandlingFee),
            b"Z9" => Some(ProductionCost),
            b"ZA" => Some(FederalMedicareOrMedicaidClaimMandateCategory1),
            b"ZB" => Some(FederalMedicareOrMedicaidClaimMandateCategory2),
            b"ZC" => Some(FederalMedicareOrMedicaidClaimMandateCategory3),
            b"ZD" => Some(FederalMedicareOrMedicaidClaimMandateCategory4),
            b"ZE" => Some(FederalMedicareOrMedicaidClaimMandateCategory5),
            b"ZF" => Some(FederalPensionMandateCategory1),
            b"ZG" => Some(FederalPensionMandateCategory2),
            b"ZH" => Some(FederalPensionMandateCategory3),
            b"ZI" => Some(FederalPensionMandateCategory4),
            b"ZJ" => Some(FederalPensionMandateCategory5),
            b"ZK" => Some(FederalMedicareOrMedicaidPaymentMandateCategory1),
            b"ZL" => Some(FederalMedicareOrMedicaidPaymentMandateCategory2),
            b"ZM" => Some(FederalMedicareOrMedicaidPaymentMandateCategory3),
            b"ZN" => Some(FederalMedicareOrMedicaidPaymentMandateCategory4),
            b"ZO" => Some(FederalMedicareOrMedicaidPaymentMandateCategory5),
            b"ZP" => Some(CouponFaceValue),
            b"ZQ" => Some(InitialTargetCost),
            b"ZR" => Some(Increase),
            b"ZS" => Some(Decrease),
            b"ZT" => Some(ProratedAmount),
            b"ZU" => Some(LoanCharge),
            b"ZV" => Some(MortgageRecordingFee),
            b"ZW" => Some(DeedRecordingFee),
            b"ZX" => Some(ReleaseRecordingFee),
            b"ZY" => Some(Assumption),
            b"ZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use AmountQualifierCode::*;
        match self {
            DownpaymentOnTheRepaymentPlanAmount => {
                "Downpayment on the Repayment Plan Amount"
            }
            InterestSubstitutionAdjustment => "Interest Substitution Adjustment",
            PrincipalSubstitutionAdjustment => "Principal Substitution Adjustment",
            PrepaidInterest => "Prepaid Interest",
            PrepaidPrincipal => "Prepaid Principal",
            DelinquentInterest => "Delinquent Interest",
            DelinquentPrincipal => "Delinquent Principal",
            CurtailmentAdjustment => "Curtailment Adjustment",
            SerialNotePrincipalAvailableForDistribution => {
                "Serial Note Principal Available for Distribution"
            }
            ServicingFee => "Servicing Fee",
            GuaranteeFeeAdjustment => "Guarantee Fee Adjustment",
            AmountUnderCollateralized => "Amount Under-collateralized",
            AmountOverCollateralized => "Amount Over-collateralized",
            TrialBalanceAdjustment => "Trial Balance Adjustment",
            CustodialBankAccountAdjustment => "Custodial Bank Account Adjustment",
            Item => "Item",
            Schedule => "Schedule",
            RegularPlan => "Regular Plan",
            PreviouslyBilled => "Previously Billed",
            CurrentlyDue => "Currently Due",
            CoveragePremium => "Coverage Premium",
            LineItemTotal => "Line Item Total",
            FixedInstallmentControlAccountingError => {
                "Fixed Installment Control Accounting Error"
            }
            Variance => "Variance",
            VarianceAdjustmentCost => "Variance Adjustment Cost",
            VarianceAdjustmentSchedule => "Variance Adjustment Schedule",
            Escalation => "Escalation",
            FixedPrice => "Fixed Price",
            Lodging => "Lodging",
            Meals => "Meals",
            TravelExpense => "Travel Expense",
            InsuranceExpense => "Insurance Expense",
            UnionDues => "Union Dues",
            RegularIncome => "Regular Income",
            IncomeOnRealProperty => "Income on Real Property",
            IncomeFromSocialSecurityAndOrOtherGovernmentAssistance => {
                "Income from Social Security and/or other Government Assistance"
            }
            TotalMonthlyIncome => "Total Monthly Income",
            ElectricAndOrFuelPayment => "Electric and/or Fuel Payment",
            WaterAndOrSewerPayment => "Water and/or Sewer Payment",
            TelephonePayment => "Telephone Payment",
            MaintenanceExpense => "Maintenance Expense",
            FoodExpense => "Food Expense",
            ClothingExpense => "Clothing Expense",
            LaundryExpense => "Laundry Expense",
            MedicalAndOrDentalExpense => "Medical and/or Dental Expense",
            RecreationExpenses => "Recreation Expenses",
            CharitableContributions => "Charitable Contributions",
            BatchTotal => "Batch Total",
            GraduatedPaymentMortgageAdjustment => "Graduated Payment Mortgage Adjustment",
            HomeInsuranceExpense => "Home Insurance Expense",
            LifeInsuranceExpense => "Life Insurance Expense",
            HealthInsuranceExpense => "Health Insurance Expense",
            AutomobileInsuranceExpense => "Automobile Insurance Expense",
            ValueOfPropertyClaimedAsExempt => "Value of Property Claimed as Exempt",
            AutomobilePayment => "Automobile Payment",
            OtherTypeOfInstallmentPayment => "Other Type of Installment Payment",
            OperatingExpenses => "Operating Expenses",
            TotalProjectedMonthlyIncome => "Total Projected Monthly Income",
            TotalProjectedMonthlyExpenses => "Total Projected Monthly Expenses",
            ExcessIncome => "Excess Income",
            ValueOfPersonalProperty => "Value of Personal Property",
            MonthlyOvertime => "Monthly Overtime",
            TotalAllRepairs => "Total All Repairs",
            TotalRecommendedRepairs => "Total Recommended Repairs",
            StateQuarterlyTotalGrossWages => "State Quarterly Total Gross Wages",
            Code2R => "State Quarterly Unemployment Insurance (UI) Total Wages",
            Code2S => "State Quarterly Unemployment Insurance (UI) Excess Wages",
            Code2T => "State Quarterly Unemployment Insurance (UI) Taxable Wages",
            StateQuarterlyDisabilityInsuranceTaxableWages => {
                "State Quarterly Disability Insurance Taxable Wages"
            }
            StateQuarterlyTipWages => "State Quarterly Tip Wages",
            AssetLongTerm => "Asset-Long Term",
            AssetShortTerm => "Asset-Short Term",
            BaseCoverage => "Base Coverage",
            CommissionRetained => "Commission Retained",
            DepositTotal => "Deposit Total",
            GrowingEquityMortgageAdjustment => "Growing Equity Mortgage Adjustment",
            Accounting => "Accounting",
            AccountsPayable => "Accounts Payable",
            AccountsReceivable => "Accounts Receivable",
            AdvancedDividends => "Advanced Dividends",
            AdvertisingExpenses => "Advertising Expenses",
            Amortization => "Amortization",
            AmortizationCosts => "Amortization Costs",
            AmountOfDecree => "Amount of Decree",
            AssetInvestment => "Asset Investment",
            AuthorizedCapital => "Authorized Capital",
            AvailableReserves => "Available Reserves",
            BadDebtAllowance => "Bad Debt Allowance",
            BadDebts => "Bad Debts",
            Code3N => "Bank Account(s)",
            LongTermAssets => "Long Term Assets",
            LongTermLiabilities => "Long Term Liabilities",
            LongTermTangibleAssets => "Long Term Tangible Assets",
            LossesOnCapital => "Losses on Capital",
            MachinesAndTools => "Machines and Tools",
            MemberRiskCapital => "Member Risk Capital",
            MiscellaneousAfterTaxExempt => "Miscellaneous After Tax Exempt",
            Mortgage => "Mortgage",
            NominalCapital => "Nominal Capital",
            NominalDamages => "Nominal Damages",
            NonOperationalFixedAssets => "Non-operational Fixed Assets",
            ExcessAmountRequested => "Excess Amount Requested",
            LockBoxTotal => "Lock Box Total",
            AdjustableRateMortgageChange => "Adjustable Rate Mortgage Change",
            NonissuedCapital => "Nonissued Capital",
            NotesPayable => "Notes Payable",
            NotesReceivable => "Notes Receivable",
            BankDebentures => "Bank Debentures",
            BankObligations => "Bank Obligations",
            Buildings => "Buildings",
            BuildingsUnderConstruction => "Buildings Under Construction",
            Capital => "Capital",
            CapitalAssociatedWithPrincipal => "Capital Associated with Principal",
            CapitalOfOtherSubsidiaries => "Capital of Other Subsidiaries",
            CapitalStock => "Capital Stock",
            Cash => "Cash",
            CapitalSubsidiesReceived => "Capital Subsidies Received",
            CommercialDebt => "Commercial Debt",
            CommercialExpenses => "Commercial Expenses",
            CommonStock => "Common Stock",
            ConsequentialDamages => "Consequential Damages",
            CompensatoryDamages => "Compensatory Damages",
            ConvertibleDebentures => "Convertible Debentures",
            CostOfGoodsSold => "Cost of Goods Sold",
            CostOfSales => "Cost of Sales",
            Code4V => "Cost(s)",
            CurrentAssets => "Current Assets",
            CurrentLiabilities => "Current Liabilities",
            Damages => "Damages",
            DeferredCost => "Deferred Cost",
            TotalInvoiceAmount => "Total Invoice Amount",
            FixedInstallmentControlSubstitutionAdjustment => {
                "Fixed Installment Control Substitution Adjustment"
            }
            DeferredCreditOrIncome => "Deferred Credit or Income",
            DeferredTaxation => "Deferred Taxation",
            Deposits => "Deposits",
            Depreciation => "Depreciation",
            DepreciationOfFixedAssets => "Depreciation of Fixed Assets",
            DepreciationOfRevaluationOfFixedAssets => {
                "Depreciation of Revaluation of Fixed Assets"
            }
            DirectorsRemuneration => "Director's Remuneration",
            Dividends => "Dividends",
            DoubtfulReceivables => "Doubtful Receivables",
            Equipment => "Equipment",
            EquipmentSubsidies => "Equipment Subsidies",
            Code5L => "Equities, Stocks",
            Equity => "Equity",
            ExceptionalItem => "Exceptional Item",
            Exports => "Exports",
            ExternalCharge => "External Charge",
            ExtraordinaryCharge => "Extraordinary Charge",
            ExtraordinaryCurrentAssetWriteDowns => {
                "Extraordinary Current Asset Write Downs"
            }
            ExtraordinaryResult => "Extraordinary Result",
            FinancialAssets => "Financial Assets",
            FinancialCharges => "Financial Charges",
            FinancialDebt => "Financial Debt",
            FinancialExpenses => "Financial Expenses",
            FinancialIncome => "Financial Income",
            FinishedGoods => "Finished Goods",
            FixedAssetDebts => "Fixed Asset Debts",
            AmountSubjectToTotalMonetaryDiscount => {
                "Amount Subject to Total Monetary Discount"
            }
            InterestAdjustment => "Interest Adjustment",
            FixedAssets => "Fixed Assets",
            FixedAssetsForSale => "Fixed Assets for Sale",
            Fixtures => "Fixtures",
            FixturesAndEquipment => "Fixtures and Equipment",
            Franchise => "Franchise",
            FranchiseTaxBalance => "Franchise Tax Balance",
            FranchiseTaxPaid => "Franchise Tax Paid",
            FreeReserves => "Free Reserves",
            Furniture => "Furniture",
            FutureLoan => "Future Loan",
            GeneralAccounts => "General Accounts",
            GeneralExpenses => "General Expenses",
            Goodwill => "Goodwill",
            GrantsForOperatingCosts => "Grants for Operating Costs",
            GroupRelatedFinancialIncome => "Group Related Financial Income",
            IncomeStatedInAdvance => "Income Stated in Advance",
            IncomeTax => "Income Tax",
            Code6R => "Income Tax, Corporate",
            Code6S => "Income Tax, Noncorporate",
            Injunction => "Injunction",
            IntangibleDepreciation => "Intangible Depreciation",
            Intangibles => "Intangibles",
            InterestOfThirdParty => "Interest of Third Party",
            InterestOnLoans => "Interest on Loans",
            Code6Y => "Operating Income (Loss)",
            OptionalReserves => "Optional Reserves",
            DiscountAmountDue => "Discount Amount Due",
            DeferredGraduatedPaymentMortgageInterestPaid => {
                "Deferred Graduated Payment Mortgage Interest Paid"
            }
            OrganizationalExpenses => "Organizational Expenses",
            OutsideShareInProfitOrLoss => "Outside Share in Profit or Loss",
            OutstandingDebtsAgainstBoardOfDirectorsManagers => {
                "Outstanding Debts against Board of Directors/Managers"
            }
            Owing => "Owing",
            OwingFromAffiliates => "Owing from Affiliates",
            OwingFromParticipants => "Owing from Participants",
            OwingToAffiliates => "Owing to Affiliates",
            OwingToFiscalOffice => "Owing to Fiscal Office",
            OwingToNationalSocialSecurityOffice => {
                "Owing to National Social Security Office"
            }
            OwingToParticipants => "Owing to Participants",
            OwnWorkCapitalized => "Own Work Capitalized",
            PaidInCapital => "Paid in Capital",
            ParValue => "Par Value",
            ParticipatingInterest => "Participating Interest",
            Patents => "Patents",
            PensionDebts => "Pension Debts",
            PensionsProvision => "Pensions Provision",
            PreferredStock => "Preferred Stock",
            PrepaidOrdersInProgress => "Prepaid Orders in Progress",
            PriorResultsCarriedForward => "Prior Results Carried Forward",
            ProfitOrLoss => "Profit or Loss",
            ProfitOrLossAfterTaxes => "Profit or Loss after Taxes",
            ProfitOrLossBeforeTaxes => "Profit or Loss before Taxes",
            ProfitOrLossOnOrdinaryActivitiesAfterTax => {
                "Profit or Loss on Ordinary Activities after Tax"
            }
            ProgressPayments => "Progress Payments",
            ProposedDividend => "Proposed Dividend",
            TotalMonetaryDiscountAmount => "Total Monetary Discount Amount",
            InterestAccountingError => "Interest Accounting Error",
            ProvisionForDepreciationOfStockOrInventory => {
                "Provision for Depreciation of Stock or Inventory"
            }
            ProvisionForFuturePurchases => "Provision for Future Purchases",
            ProvisionForRisks => "Provision for Risks",
            PunitiveDamages => "Punitive Damages",
            PurchasePrice => "Purchase Price",
            Purchases => "Purchases",
            RawMaterials => "Raw Materials",
            RealEstate => "Real Estate",
            Receivables => "Receivables",
            RegularizationAccount => "Regularization Account",
            ResearchAndDevelopment => "Research and Development",
            RestructuringCosts => "Restructuring Costs",
            Result => "Result",
            RetainedEarnings => "Retained Earnings",
            Revenues => "Revenues",
            Sales => "Sales",
            SalesAndUseTax => "Sales and Use Tax",
            Savings => "Savings",
            SecuredLiability => "Secured Liability",
            SecuredLoans => "Secured Loans",
            SellingExpenses => "Selling Expenses",
            Services => "Services",
            ShareCapital => "Share Capital",
            ShareInProfitOrLossOfMinorityInterest => {
                "Share in Profit or Loss of Minority Interest"
            }
            SharePremiumCapital => "Share Premium Capital",
            SharesInAffiliatedCompanies => "Shares in Affiliated Companies",
            TotalOperationalStatementAmount => "Total Operational Statement Amount",
            PrincipalAccountingError => "Principal Accounting Error",
            SocialCharges => "Social Charges",
            Code9B => "Social Security (FICA)",
            SpecialReserves => "Special Reserves",
            SpeciallySecuredCreditors => "Specially Secured Creditors",
            SpecificPerformance => "Specific Performance",
            StartingCapital => "Starting Capital",
            StatutoryReserves => "Statutory Reserves",
            SubscribedCapital => "Subscribed Capital",
            SuitAmount => "Suit Amount",
            Supplies => "Supplies",
            SurplusOfRevaluation => "Surplus of Revaluation",
            TangibleNetWorth => "Tangible Net Worth",
            TaxAdjustments => "Tax Adjustments",
            TaxBalance => "Tax Balance",
            TaxCapitalAmount => "Tax Capital Amount",
            TaxOnExtraordinaryItems => "Tax on Extraordinary Items",
            TaxRecoverable => "Tax Recoverable",
            TaxedReserves => "Taxed Reserves",
            TradeCreditors => "Trade Creditors",
            Code9T => "Inventory (Stock)",
            Code9U => "Inventory (Stock) Depreciation",
            Code9V => "Inventory (Stock) Purchases",
            InvestmentInOwnShares => "Investment in Own Shares",
            Investments => "Investments",
            IssuedCapital => "Issued Capital",
            LaborCosts => "Labor Costs",
            ShipmentValueInUSDollars => "Shipment Value in U.S. Dollars",
            LiabilitiesAtBankruptcy => "Liabilities at Bankruptcy",
            AccountAverageBalanceAccount => "Account Average Balance Account",
            OutstandingBalanceAtForeclosure => "Outstanding Balance at Foreclosure",
            LegalObligationDebtAmount => "Legal Obligation Debt Amount",
            EstimatedClosingCostAmount => "Estimated Closing Cost Amount",
            DiscountFeesPaidByBorrowerAmount => "Discount Fees Paid by Borrower Amount",
            ClosingCostsOrConcessionsPaidBySeller => {
                "Closing Costs or Concessions Paid by Seller"
            }
            PrepaidItemsAmount => "Prepaid Items Amount",
            Code19 => {
                "Federal Housing Administration, Mortgage Insurance Premium Funding Fee Financed Amount"
            }
            Code20 => {
                "Federal Housing Administration, Mortgage Insurance Premium or Veteran's Administration Funding Fee Amount"
            }
            OriginalCostOfPropertyAmount => "Original Cost of Property Amount",
            OwnersEstimateOfValueAmount => "Owner's Estimate of Value Amount",
            AppraisedValueAmount => "Appraised Value Amount",
            GrossMonthlyIncomeAmount => "Gross Monthly Income Amount",
            AssetsAtBankruptcy => "Assets at Bankruptcy",
            NegotiatedCost => "Negotiated Cost",
            AuthorizedUnpricedWork => "Authorized Unpriced Work",
            TargetPrice => "Target Price",
            EstimatedPrice => "Estimated Price",
            ContractCeiling => "Contract Ceiling",
            EstimatedContractCeiling => "Estimated Contract Ceiling",
            TargetFeeOrProfitAmount => "Target Fee or Profit Amount",
            OriginalContractTargetCost => "Original Contract Target Cost",
            NegotiatedContractChanges => "Negotiated Contract Changes",
            CurrentTargetCost => "Current Target Cost",
            Code36 => "Contract Budget Base (CBB)",
            Code37 => "Current Budgeted Cost for Work Scheduled (BCWS)",
            Code38 => "Current Budgeted Cost for Work Performed (BCWP)",
            Code39 => "Current Actual Cost of Work Performed (ACWP)",
            Code40 => "Current Schedule Variance (SV)",
            Code41 => "Current Cost Variance (CV)",
            Code42 => "Cumulative Budgeted Cost for Work Scheduled (BCWS)",
            Code43 => "Cumulative Budgeted Cost for Work Performed (BCWP)",
            Code44 => "Cumulative Actual Cost of Work Performed (ACWP)",
            Code45 => "Cumulative Schedule Variance (SV)",
            Code46 => "Cumulative Cost Variance (CV)",
            ReprogramCostVariance => "Reprogram Cost Variance",
            ReprogramBudget => "Reprogram Budget",
            Code49 => "At Complete Budget (BAC)",
            Code50 => "At Complete Latest Revised Estimate (LRE)",
            AtCompleteVariance => "At Complete Variance",
            TotalAllocatedBudget => "Total Allocated Budget",
            Code53 => "Difference (Contract Budget Base - Total Allocated Budget)",
            Forecast => "Forecast",
            AtCompleteForecast => "At Complete Forecast",
            Code56 => "Current Cost Performance Index (CPIe) - Efficiency (BCWP/ACWP)",
            Code57 => "Current Cost Performance Index (CPIp) - Planned (ACWP/BCWP)",
            Code58 => "Current Schedule Performance Index (SPI)",
            Code59 => "Cumulative Cost Performance Index (CPIe) - Efficiency (BCWP/ACWP)",
            Code60 => "Cumulative Cost Performance Index (CPIp) - Planned (ACWP/BCWP)",
            Code61 => "Cumulative Schedule Performance Index (SPI)",
            Code62 => "To Complete Performance Index (TCPI) for Budget at Complete (BAC)",
            Code63 => {
                "To Complete Performance Index (TCPI) for Estimate At Complete (EAC)"
            }
            InitialContractPriceTarget => "Initial Contract Price Target",
            InitialContractPriceCeiling => "Initial Contract Price Ceiling",
            AdjustedContractPriceTarget => "Adjusted Contract Price Target",
            AdjustedContractPriceCeiling => "Adjusted Contract Price Ceiling",
            FundsAuthorizedToDate => "Funds Authorized to Date",
            AccruedExpenditures => "Accrued Expenditures",
            OpenCommitments => "Open Commitments",
            ForecastOfBillings => "Forecast of Billings",
            EstimatedTerminationCosts => "Estimated Termination Costs",
            AccruedExpendituresPlusOpenCommitments => {
                "Accrued Expenditures plus Open Commitments"
            }
            ContractWorkAuthorizedDefinitized => "Contract Work Authorized - Definitized",
            ContractWorkAuthorizedNotDefinitized => {
                "Contract Work Authorized - Not Definitized"
            }
            ContractWorkAuthorizedTotal => "Contract Work Authorized - Total",
            ForecastOfWorkNotYetAuthorized => "Forecast of Work - Not Yet Authorized",
            ForecastOfWorkAllOther => "Forecast of Work - All Other",
            ForecastOfWorkTotal => "Forecast of Work - Total",
            FundingTotalRequirements => "Funding - Total Requirements",
            FundsCarryover => "Funds Carryover",
            NetFundsRequired => "Net Funds Required",
            Code83 => "Contract Work Authorized (with fee/profit) Actual or Projected",
            Code84 => {
                "Contract Work Authorized (with fee/profit) Actual or Projected - At Complete"
            }
            BestCaseEstimate => "Best Case Estimate",
            WorstCaseEstimate => "Worst Case Estimate",
            MostLikelyEstimate => "Most Likely Estimate",
            Code88 => "\"As Is\" Appraisal Amount",
            Code89 => "\"Subject To\" Appraisal Amount",
            Code90 => "\"Completion Per Plans\" Appraisal Amount",
            SiteValueAmount => "Site Value Amount",
            Compensation => "Compensation",
            Contribution => "Contribution",
            DeathBenefit => "Death Benefit",
            DeathBenefitDecrement => "Death Benefit Decrement",
            EmployeeAccountBalance => "Employee Account Balance",
            LoanRepayment => "Loan Repayment",
            PriorW2 => "Prior W2",
            SinglePremium => "Single Premium",
            AdjustedChargebackClaimAmount => "Adjusted Chargeback Claim Amount",
            AssistantshipFromAdmittingEducationalInstitution => {
                "Assistantship from Admitting Educational Institution"
            }
            AverageNegativeLedgerBalance => "Average Negative Ledger Balance",
            AveragePositiveCollectedBalance => "Average Positive Collected Balance",
            AverageNegativeCollectedBalance => "Average Negative Collected Balance",
            AveragePositiveLedgerBalance => "Average Positive Ledger Balance",
            DisallowedEstimated => "Disallowed - Estimated",
            DisallowedActual => "Disallowed - Actual",
            NoncoveredChargesEstimated => "Noncovered Charges - Estimated",
            NoncoveredChargesActual => "Noncovered Charges - Actual",
            AllowedEstimated => "Allowed - Estimated",
            Allocated => "Allocated",
            ExcessFunds => "Excess Funds",
            CumulativeTotal => "Cumulative Total",
            ReimbursableAmount => "Reimbursable Amount",
            TotalReimbursableAmount => "Total Reimbursable Amount",
            DirectCitationAmount => "Direct Citation Amount",
            TotalDirectCitationFunds => "Total Direct Citation Funds",
            ChargeableAmount => "Chargeable Amount",
            TemporaryTermCoverage => "Temporary Term Coverage",
            ConditionalReceiptCoverage => "Conditional Receipt Coverage",
            BindingInterimCoverage => "Binding Interim Coverage",
            ApplicationAmount => "Application Amount",
            ApprovedAmount => "Approved Amount",
            UltimateFaceAmount => "Ultimate Face Amount",
            RequestedAmountFromAllReinsurers => "Requested Amount from All Reinsurers",
            ReplacementAmount => "Replacement Amount",
            ScheduledContribution => "Scheduled Contribution",
            ScheduledDisbursement => "Scheduled Disbursement",
            ShortTermInvestment => "Short Term Investment",
            SubsequentContribution => "Subsequent Contribution",
            SubsequentDistribution => "Subsequent Distribution",
            TaxFederal => "Tax-Federal",
            TaxLocal => "Tax-Local",
            TaxState => "Tax-State",
            TrustFund => "Trust Fund",
            CapitalLeases => "Capital Leases",
            Surplus => "Surplus",
            RestatedAssets => "Restated Assets",
            OwingToClients => "Owing to Clients",
            ShareholderLoans => "Shareholder Loans",
            AccumulatedDeficit => "Accumulated Deficit",
            LoanFromParentCompany => "Loan from Parent Company",
            ContributionNotSubjectToRepayment => "Contribution Not Subject to Repayment",
            IncomeBeforeDepreciation => "Income Before Depreciation",
            AdjustedCollectedBalance => "Adjusted Collected Balance",
            IncomeAfterDepreciation => "Income After Depreciation",
            CodeABB => "Profit (Loss) Before Financial Items",
            InterestExpenses => "Interest Expenses",
            CodeABD => "Profit (Loss) Before Extraordinary Items",
            CodeABE => "Profit (Loss) After Financial Items",
            IncomeBeforeAllocations => "Income Before Allocations",
            IncomeFromSaleOfFixedAssets => "Income from Sale of Fixed Assets",
            ContributionToGroup => "Contribution to Group",
            DeferredTaxAssets => "Deferred Tax Assets",
            BlockedAccounts => "Blocked Accounts",
            NonTaxedReserves => "Non-taxed Reserves",
            PledgedAssets => "Pledged Assets",
            RestrictedEquity => "Restricted Equity",
            NonRestrictedEquity => "Non-restricted Equity",
            DepreciableAssets => "Depreciable Assets",
            TaxableAssets => "Taxable Assets",
            IncomeFromBusiness => "Income from Business",
            IncomeSubjectToTaxes => "Income Subject to Taxes",
            TaxableAmountOfRealEstate => "Taxable Amount of Real Estate",
            EndingPrincipalBalance => "Ending Principal Balance",
            AverageDailyPrincipalBalance => "Average Daily Principal Balance",
            InterestAmount => "Interest Amount",
            AdjustmentsForDifferenceInAverageDailyPrincipalBalance => {
                "Adjustments for Difference in Average Daily Principal Balance"
            }
            BeginningPrincipalBalance => "Beginning Principal Balance",
            LoanPrincipalDisbursements => "Loan Principal Disbursements",
            PrincipalIncreases => "Principal Increases",
            AverageCollectedBalance => "Average Collected Balance",
            PrincipalOfLoansPurchased => "Principal of Loans Purchased",
            PrincipalCured => "Principal Cured",
            PrincipalSold => "Principal Sold",
            PrincipalInsuranceClaims => "Principal Insurance Claims",
            PrincipalGuaranteeVoided => "Principal Guarantee Voided",
            PrincipalPaidByBorrowers => "Principal Paid by Borrowers",
            LoansInSchoolAndGrace => "Loans in School and Grace",
            LoansInAuthorizedDeferment => "Loans in Authorized Deferment",
            LoansRepayOrForebearanceCurrentOrLessThan31Days => {
                "Loans Repay or Forebearance - Current or Less than 31 Days"
            }
            LoansRepayOrForebearance31To60DaysPastDue => {
                "Loans Repay or Forebearance - 31 to 60 Days Past Due"
            }
            LoansRepayOrForebearance61To90DaysPastDue => {
                "Loans Repay or Forebearance - 61 to 90 Days Past Due"
            }
            LoansRepayOrForebearance91To120DaysPastDue => {
                "Loans Repay or Forebearance - 91 to 120 Days Past Due"
            }
            LoansRepayOrForebearance121To180DaysPastDue => {
                "Loans Repay or Forebearance - 121 to 180 Days Past Due"
            }
            LoansRepayOrForebearance181To270DaysPastDue => {
                "Loans Repay or Forebearance - 181 to 270 Days Past Due"
            }
            LoansRepayOrForebearance271OrMoreDaysPastDue => {
                "Loans Repay or Forebearance - 271 or More Days Past Due"
            }
            CodeACP => "Loans Repay or Forebearance - Claims Filed, Not Yet Paid",
            AgentSales => "Agent Sales",
            AmountInvolved => "Amount Involved",
            AssignedCapital => "Assigned Capital",
            CreditLineUtilized => "Credit Line Utilized",
            DirectSales => "Direct Sales",
            EarningsPerShare => "Earnings per Share",
            Inheritance => "Inheritance",
            InvestedCapital => "Invested Capital",
            LoanFromFamilyMembers => "Loan from Family Members",
            NonDepreciableAssets => "Non Depreciable Assets",
            AdjustedTotal => "Adjusted Total",
            PartiallyPaidAmountPerShare => "Partially Paid Amount per Share",
            PendingOrders => "Pending Orders",
            PersonalLoan => "Personal Loan",
            PlantAndMachinery => "Plant and Machinery",
            PreTaxLoss => "Pre-Tax Loss",
            PreTaxProfit => "Pre-Tax Profit",
            RegisteredCapital => "Registered Capital",
            RevaluationReserves => "Revaluation Reserves",
            SocialCapital => "Social Capital",
            StatutoryProfit => "Statutory Profit",
            TrainingPay => "Training Pay",
            RetroactivePay => "Retroactive Pay",
            ExpectedReimbursementAmount => "Expected Reimbursement Amount",
            PermitCost => "Permit Cost",
            Minimum => "Minimum",
            AdditionalAmountToMeetMinimum => "Additional Amount to Meet Minimum",
            LaborPerHour => "Labor Per Hour",
            NonRecoverableDepreciation => "Non-recoverable Depreciation",
            RecoverableDepreciation => "Recoverable Depreciation",
            Overhead => "Overhead",
            IndemnityBenefit => "Indemnity Benefit",
            ReplacementCostOfRepairs => "Replacement Cost of Repairs",
            ActualCashValueOfRepairs => "Actual Cash Value of Repairs",
            RecoverableDepreciationOfRepairs => "Recoverable Depreciation of Repairs",
            NonRecoverableDepreciationOfRepairs => {
                "Non-recoverable Depreciation of Repairs"
            }
            Arrearage => "Arrearage",
            NonIndemnityBenefit => "Non-indemnity Benefit",
            ActualCashValueOfBuilding => "Actual Cash Value of Building",
            GovernmentShare => "Government Share",
            ContractorShare => "Contractor Share",
            AwardFee => "Award Fee",
            BaseFee => "Base Fee",
            TargetProfitFloor => "Target Profit Floor",
            TargetProfitCeiling => "Target Profit Ceiling",
            LaborPerDay => "Labor Per Day",
            DifferenceInInterestDue => "Difference in Interest Due",
            DifferenceInPrepaymentPenalty => "Difference in Prepayment Penalty",
            DifferenceInPrincipalDue => "Difference in Principal Due",
            AppropriationOfRetainedEarningsLessReversals => {
                "Appropriation of Retained Earnings Less Reversals"
            }
            Appropriations => "Appropriations",
            BillingsAndCostsProfitDifferential => {
                "Billings and Costs-Profit Differential"
            }
            CommonStockParValue => "Common Stock Par Value",
            CostOfServicesRendered => "Cost of Services Rendered",
            Creditors => "Creditors",
            DeclaredProfit => "Declared Profit",
            DiscountedNotes => "Discounted Notes",
            EndorsedNotes => "Endorsed Notes",
            GeneralReserves => "General Reserves",
            ImportVolume => "Import Volume",
            IncomeTaxCredit => "Income Tax Credit",
            LongTermDeposits => "Long-Term Deposits",
            LongTermLoans => "Long-Term Loans",
            AverageFloat => "Average Float",
            MinorityInterest => "Minority Interest",
            NonOperatingExpense => "Non-Operating Expense",
            NonOperatingIncome => "Non-Operating Income",
            OperatingProfitOrLoss => "Operating Profit or Loss",
            PreferredStockParValue => "Preferred Stock Par Value",
            ProfitAfterTaxAndMinorityInterest => "Profit After Tax and Minority Interest",
            RetainedEarningsToBeAppropriated => "Retained Earnings to be Appropriated",
            RevaluationSurplusOrDeficit => "Revaluation Surplus or Deficit",
            ReversalOfVoluntaryEarnedSurplus => "Reversal of Voluntary Earned Surplus",
            SharePrice => "Share Price",
            ShortTermDeposits => "Short-Term Deposits",
            ShortTermLoans => "Short-Term Loans",
            TaxProvisions => "Tax Provisions",
            UnallocatedProfit => "Unallocated Profit",
            VoluntaryEarnedSurplus => "Voluntary Earned Surplus",
            CalculatedWeeklyCompensationAmount => "Calculated Weekly Compensation Amount",
            BenefitTypeGrossWeeklyAmount => "Benefit Type Gross Weekly Amount",
            BenefitTypeNetWeeklyAmount => "Benefit Type Net Weekly Amount",
            EmployeeGrossWage => "Employee Gross Wage",
            GarageGrossWages => "Garage Gross Wages",
            OfficerCompensationActualFlat => "Officer Compensation - Actual Flat",
            OfficerCompensationStatutoryMaximum => {
                "Officer Compensation - Statutory Maximum"
            }
            OfficerCompensationStatutoryMinimum => {
                "Officer Compensation - Statutory Minimum"
            }
            PreviousBalance => "Previous Balance",
            DisputedAmount => "Disputed Amount",
            AdjustedGrossIncome => "Adjusted Gross Income",
            NonOperatingIncomeOrExpense => "Non-Operating Income or Expense",
            OperatingIncomeOrExpense => "Operating Income or Expense",
            IncomeOrExpense => "Income or Expense",
            PurchaseAuthority => "Purchase Authority",
            CapitalDecrease => "Capital Decrease",
            CapitalIncrease => "Capital Increase",
            DeedCapital => "Deed Capital",
            TransferredAmount => "Transferred Amount",
            UnadjustedSalesPrice => "Unadjusted Sales Price",
            SalesConcessions => "Sales Concessions",
            PropertyValue => "Property Value",
            PartialReleaseAmount => "Partial Release Amount",
            LienFilingFee => "Lien Filing Fee",
            AdditionalRepairPrice => "Additional Repair Price",
            QualifiedTuitionAndRelatedExpenses => {
                "Qualified Tuition and Related Expenses"
            }
            QualifiedFinancialAssistance => "Qualified Financial Assistance",
            AggregateReimbursementsOrRefunds => "Aggregate Reimbursements or Refunds",
            NewLoanBalance => "New Loan Balance",
            RawMaterialPurchases => "Raw Material Purchases",
            WorkInProgressPurchases => "Work in Progress Purchases",
            OperatingCashFlow => "Operating Cash Flow",
            PaymentsForOutsideWork => "Payments for Outside Work",
            SetAsideForProvisions => "Set Aside for Provisions",
            FinancialIncomeOrExpense => "Financial Income or Expense",
            ExtraordinaryIncomeOrExpense => "Extraordinary Income or Expense",
            LoanBalanceDifference => "Loan Balance Difference",
            UnfinishedProductionCarriedForward => "Unfinished Production Carried Forward",
            InstallationMaterialsCost => "Installation Materials Cost",
            NewMortgageAmount => "New Mortgage Amount",
            CapitalizedAssets => "Capitalized Assets",
            ProfitReserves => "Profit Reserves",
            ShareInProfitOrLossInOtherCompanies => {
                "Share in Profit or Loss in Other Companies"
            }
            MonetaryCorrection => "Monetary Correction",
            ScheduledRepaymentAmount => "Scheduled Repayment Amount",
            AmountsPlacedWithOtherBanks => "Amounts Placed with Other Banks",
            DueFromParentCompany => "Due from Parent Company",
            OwingFromSubsidiaryCompanies => "Owing from Subsidiary Companies",
            CertificatesOfDepositAmount => "Certificates of Deposit Amount",
            PubliclyTradedSharesAmount => "Publicly Traded Shares Amount",
            NonPubliclyTradedSharesAmount => "Non Publicly Traded Shares Amount",
            TradingSecuritiesAmount => "Trading Securities Amount",
            InvestmentSecuritiesAmount => "Investment Securities Amount",
            EarningsPerShareMinusDividends => "Earnings Per Share Minus Dividends",
            ActivePartnerCapital => "Active Partner Capital",
            SaleAmount => "Sale Amount",
            FundsHeldByMortgagee => "Funds Held by Mortgagee",
            AdjustedClaim => "Adjusted Claim",
            AttorneyFees => "Attorney Fees",
            AverageLedgerBalance => "Average Ledger Balance",
            AmountFinanced => "Amount Financed",
            BankruptcyFee => "Bankruptcy Fee",
            AccruedIncome => "Accrued Income",
            AmountOverride => "Amount Override",
            AmountPriorToFractionalization => "Amount Prior to Fractionalization",
            AmountOfPurchaseExemptFromTaxOrFee => {
                "Amount of Purchase Exempt From Tax or Fee"
            }
            AveragePricePerCall => "Average Price Per Call",
            AcquisitionCostOfLenses => "Acquisition Cost of Lenses",
            FeesToPublicOfficialsForForeclosure => {
                "Fees to Public Officials for Foreclosure"
            }
            AveragePricePerMinute => "Average Price Per Minute",
            AnnualSalesOrPremiums => "Annual Sales or Premiums",
            TotalReceived => "Total Received",
            AmountOfTaxOrFeeExemption => "Amount of Tax or Fee Exemption",
            CoverageAmount => "Coverage Amount",
            ActualCashValue => "Actual Cash Value",
            Average => "Average",
            ReplacementCost => "Replacement Cost",
            PreviousPrice => "Previous Price",
            TitleCost => "Title Cost",
            OtherForeclosureAndAcquisitionExpenses => {
                "Other Foreclosure and Acquisition Expenses"
            }
            Estimated => "Estimated",
            Bond => "Bond",
            BenefitAmount => "Benefit Amount",
            BonusesAndCommissionsDividedOver12Months => {
                "Bonuses and Commissions Divided Over 12 Months"
            }
            BonusesDividedOver12Months => "Bonuses Divided Over 12 Months",
            BonusesAndCommissions => "Bonuses and Commissions",
            Budgeted => "Budgeted",
            AllowedActual => "Allowed - Actual",
            DeductibleEstimated => "Deductible - Estimated",
            CoInsuranceEstimated => "Co-insurance - Estimated",
            CoInsuranceActual => "Co-insurance - Actual",
            Bargain => "Bargain",
            NetTaxableIncome => "Net Taxable Income",
            OriginalAmountOfInstrument => "Original Amount of Instrument",
            AdditionToTax => "Addition to Tax",
            ReinstatementFee => "Reinstatement Fee",
            PermitFeeDue => "Permit Fee Due",
            PermitFeeWithExtension => "Permit Fee with Extension",
            NetAnnualPermitFeeDue => "Net Annual Permit Fee Due",
            PermitFeePenaltyDue => "Permit Fee Penalty Due",
            PermitFeeInterestDue => "Permit Fee Interest Due",
            TotalPermitFeeDue => "Total Permit Fee Due",
            FranchiseTax => "Franchise Tax",
            UnclaimedFranchiseTaxCredit => "Unclaimed Franchise Tax Credit",
            NetFranchiseTaxDue => "Net Franchise Tax Due",
            FranchiseTaxPenaltyDue => "Franchise Tax Penalty Due",
            TotalFranchiseTaxDue => "Total Franchise Tax Due",
            TotalAmountDue => "Total Amount Due",
            Overpayment => "Overpayment",
            AmountToBeRefunded => "Amount to be Refunded",
            GrossInStateReceipts => "Gross In-State Receipts",
            GrossReceipts => "Gross Receipts",
            OccupationFee => "Occupation Fee",
            TotalAssessedValue => "Total Assessed Value",
            TotalValueOfAllProperty => "Total Value of All Property",
            ValueOfInStateProperty => "Value of In-State Property",
            TotalValueOfOutOfStateProperty => "Total Value of Out-of-State Property",
            TotalParValue => "Total Par Value",
            MortgageInsurancePremiums => "Mortgage Insurance Premiums",
            TotalAssessableCapitalStock => "Total Assessable Capital Stock",
            ApportionedValue => "Apportioned Value",
            EstimatedInStateRealPropertyValue => "Estimated In-State Real Property Value",
            ParValueOfInStateAssets => "Par Value of In-State Assets",
            InStateBusinessRevenue => "In-State Business Revenue",
            SubscriptionPrice => "Subscription Price",
            ValueOfAuthorizedShares => "Value of Authorized Shares",
            OrdinanceAmount => "Ordinance Amount",
            CapitalForACooperative => "Capital for a Cooperative",
            DirectorsLegalObligationDebt => "Directors Legal Obligation Debt",
            SilentPartnerCapital => "Silent Partner Capital",
            CodeBC => "Billing Cycle Net Fee Position (Excess/Deficit)",
            BalanceDue => "Balance Due",
            DisbursementsForAuthorizedRepair => "Disbursements for Authorized Repair",
            HazardInsurancePremium => "Hazard Insurance Premium",
            EvictionAttorneyFees => "Eviction Attorney Fees",
            EvictionExpenses => "Eviction Expenses",
            PropertyTaxes => "Property Taxes",
            DisbursementsNotShownElsewhere => "Disbursements Not Shown Elsewhere",
            DisbursementsForProtectionAndPreservation => {
                "Disbursements for Protection and Preservation"
            }
            DisbursementsForInspectionsAndBoarding => {
                "Disbursements for Inspections and Boarding"
            }
            BridgeLoanNotDeposited => "Bridge Loan Not Deposited",
            Adjustments => "Adjustments",
            RentalIncome => "Rental Income",
            RentalExpense => "Rental Expense",
            BorrowerAdvance => "Borrower Advance",
            BorrowedAmount => "Borrowed Amount",
            AverageNetCollectedBalance => "Average Net Collected Balance",
            Bail => "Bail",
            AdjustedInsuredLossAmount => "Adjusted Insured Loss Amount",
            MortgageNoteInterest => "Mortgage Note Interest",
            BankRejectTotal => "Bank Reject Total",
            Betterment => "Betterment",
            OverheadCosts => "Overhead Costs",
            UncollectedInterest => "Uncollected Interest",
            AmountDueFromBuyerAtClosing => "Amount Due from Buyer at Closing",
            AmountOwedToBuyerAtClosing => "Amount Owed to Buyer at Closing",
            BillsOfExchangePayable => "Bills of Exchange Payable",
            AdditionalClosingExpenses => "Additional Closing Expenses",
            DeficiencyJudgmentExpenses => "Deficiency Judgment Expenses",
            City => "City",
            CurrentExpenditures => "Current Expenditures",
            CoPaymentAmount => "Co-Payment Amount",
            ChildRiderCoverage => "Child Rider Coverage",
            PriorPaymentEstimated => "Prior Payment - Estimated",
            PriorPaymentActual => "Prior Payment - Actual",
            ClaimAmountDueEstimated => "Claim Amount Due - Estimated",
            ClaimAmountDueActual => "Claim Amount Due - Actual",
            PayerResponsibilityEstimated => "Payer Responsibility - Estimated",
            PayerResponsibilityActual => "Payer Responsibility - Actual",
            DisallowedCostContainmentActual => "Disallowed Cost Containment - Actual",
            ContractorCumulativeToDate => "Contractor Cumulative to Date",
            CollectedBalanceRequired => "Collected Balance Required",
            ChargebackClaimAmount => "Chargeback Claim Amount",
            OverpaidSection235Subsidy => "Overpaid Section 235 Subsidy",
            CleanUpCostsAssociatedWithDeficiency => {
                "Clean-up Costs Associated with Deficiency"
            }
            SummaryAmount => "Summary Amount",
            AppraisalFees => "Appraisal Fees",
            CommissionFeesDeducted => "Commission Fees Deducted",
            ChangeAmount => "Change Amount",
            FundsHeldForInsured => "Funds Held for Insured",
            OtherDeductions => "Other Deductions",
            CodeCJL => "Collections, Judgments, and Liens",
            BackEndLoad => "Back End Load",
            OutstandingBalanceCurrentLender => "Outstanding Balance Current Lender",
            ClaimantRequestedTotal => "Claimant Requested Total",
            CompressionCharge => "Compression Charge",
            CommodityRefund => "Commodity Refund",
            SpecialAssessments => "Special Assessments",
            TaxesOnDeed => "Taxes on Deed",
            CorporateAssets => "Corporate Assets",
            StatutoryDisbursements => "Statutory Disbursements",
            ClosingCostsPaidByAnyOtherPartyOtherThanSellerOrBuyer => {
                "Closing Costs Paid by Any Other Party Other Than Seller or Buyer"
            }
            ClosingCostsPaidBySeller => "Closing Costs Paid by Seller",
            NetClaimAmount => "Net Claim Amount",
            ContractorAtComplete => "Contractor at Complete",
            ContainerReplacementCost => "Container Replacement Cost",
            ContainerReplacementLaborCost => "Container Replacement Labor Cost",
            ContainerReplacementMaterialCost => "Container Replacement Material Cost",
            CommissionSales => "Commission Sales",
            Contract => "Contract",
            SubcontractorCumulativeToDate => "Subcontractor Cumulative to Date",
            SubcontractorAtComplete => "Subcontractor at Complete",
            EarnedValue => "Earned Value",
            Actual => "Actual",
            CumulativeBudget => "Cumulative Budget",
            CumulativeEarnedValue => "Cumulative Earned Value",
            PayerAmountPaid => "Payer Amount Paid",
            AdministrationAndManagementCosts => "Administration and Management Costs",
            DeferredCompensationCommissions => "Deferred Compensation Commissions",
            DeductibleAmount => "Deductible Amount",
            DeferredCompensationCommissionsAndBonuses => {
                "Deferred Compensation Commissions and Bonuses"
            }
            DeferredCompensation => "Deferred Compensation",
            DollarForDollarDeductions => "Dollar For Dollar Deductions",
            DependentCareContribution => "Dependent Care Contribution",
            DisallowedCostContainmentEstimated => {
                "Disallowed Cost Containment - Estimated"
            }
            DispensingFee => "Dispensing Fee",
            DiscountAmount => "Discount Amount",
            CumulativeActual => "Cumulative Actual",
            OriginalMortgage => "Original Mortgage",
            UnappliedSection235Funds => "Unapplied Section 235 Funds",
            UnappliedBuydownFund => "Unapplied Buydown Fund",
            DirectDeposit => "Direct Deposit",
            EstimateOfDamage => "Estimate of Damage",
            Deferral => "Deferral",
            DelayedInterest => "Delayed Interest",
            AuthorizedBid => "Authorized Bid",
            DeferredAssets => "Deferred Assets",
            EscrowBalance => "Escrow Balance",
            TotalDisbursements => "Total Disbursements",
            ChargeOff => "Charge Off",
            LiensAmountOriginal => "Liens Amount Original",
            ReleaseOfLien => "Release of Lien",
            Debit => "Debit",
            DelinquentTaxes => "Delinquent Taxes",
            Asset => "Asset",
            Liability => "Liability",
            Satisfaction => "Satisfaction",
            Exemption => "Exemption",
            DiscountPointsFinanced => "Discount Points Financed",
            DiscountPointsNotFinanced => "Discount Points Not Financed",
            DefaultedTaxPlanPayment => "Defaulted Tax Plan Payment",
            Settlement => "Settlement",
            AlimonyExpense => "Alimony Expense",
            AlimonyIncome => "Alimony Income",
            ChildSupportExpense => "Child Support Expense",
            ChildSupportIncome => "Child Support Income",
            SeparateMaintenanceExpense => "Separate Maintenance Expense",
            DevelopmentProperties => "Development Properties",
            SeparateMaintenanceIncome => "Separate Maintenance Income",
            DeductibleWaived => "Deductible Waived",
            PerDayLimit => "Per Day Limit",
            JobRelatedExpense => "Job-related Expense",
            EstimatedCredit => "Estimated Credit",
            AdministrationAndManagementIndemnityCharge => {
                "Administration and Management Indemnity Charge"
            }
            EmployerYearToDateContribution => "Employer Year to Date Contribution",
            EmployeeAnnualPledgeAmount => "Employee Annual Pledge Amount",
            EmployeeCurrentContribution => "Employee Current Contribution",
            EmployerPledgeAmount => "Employer Pledge Amount",
            EmployerCurrentContribution => "Employer Current Contribution",
            EligibleWageAmount => "Eligible Wage Amount",
            EmployeeYearToDateContribution => "Employee Year to Date Contribution",
            EducationContribution => "Education Contribution",
            InitialFee => "Initial Fee",
            EarningsAllowance => "Earnings Allowance",
            AdministrativeExpenses => "Administrative Expenses",
            AirTravelExpenses => "Air Travel Expenses",
            AmountForgiven => "Amount Forgiven",
            AmountGuaranteed => "Amount Guaranteed",
            AmountOverFairMarketValue => "Amount Over Fair Market Value",
            AmountOwed => "Amount Owed",
            AmountPayable => "Amount Payable",
            AmountRaised => "Amount Raised",
            AmountReceived => "Amount Received",
            AmountRefunded => "Amount Refunded",
            AmountRescinded => "Amount Rescinded",
            AnonymousContribution => "Anonymous Contribution",
            BalanceOwed => "Balance Owed",
            BankCharges => "Bank Charges",
            BankLoan => "Bank Loan",
            BrochureExpenses => "Brochure Expenses",
            BusTravelExpenses => "Bus Travel Expenses",
            ConsultantExpenses => "Consultant Expenses",
            CorrectedAmount => "Corrected Amount",
            DisabilityExpenses => "Disability Expenses",
            DisposedAmount => "Disposed Amount",
            DrawAmount => "Draw Amount",
            ElectionExpenses => "Election Expenses",
            EndorsementAmount => "Endorsement Amount",
            EntertainmentExpenses => "Entertainment Expenses",
            ExcessExpenses => "Excess Expenses",
            CodeEB => "Collected Balance (Excess/Deficit)",
            ExpectedExpenditureAmount => "Expected Expenditure Amount",
            ExpenditureAmount => "Expenditure Amount",
            FamilyCareExpenses => "Family Care Expenses",
            FederalShareAmount => "Federal Share Amount",
            FilingFee => "Filing Fee",
            InKindContribution => "In-Kind Contribution",
            IncurredAmount => "Incurred Amount",
            LoanAmountPlusInterest => "Loan Amount Plus Interest",
            LoanBalance => "Loan Balance",
            MatchingContribution => "Matching Contribution",
            MeetingExpenses => "Meeting Expenses",
            MiscellaneousIncome => "Miscellaneous Income",
            MiscellaneousReceipts => "Miscellaneous Receipts",
            NewLoanAmount => "New Loan Amount",
            NewUnpaidExpenditure => "New Unpaid Expenditure",
            NewsletterExpenses => "Newsletter Expenses",
            NewspaperAdvertisingExpenses => "Newspaper Advertising Expenses",
            NominationExpenses => "Nomination Expenses",
            NonFederalShare => "Non-Federal Share",
            OfficeExpenses => "Office Expenses",
            OfficeRental => "Office Rental",
            OriginalAssetValue => "Original Asset Value",
            OriginalLoanAmount => "Original Loan Amount",
            PartyExpenses => "Party Expenses",
            Payment => "Payment",
            CodeEC => "Allowance (Excess/Deficit)",
            PersonalExpenses => "Personal Expenses",
            PersonalFunds => "Personal Funds",
            PledgedAmount => "Pledged Amount",
            PostageExpenses => "Postage Expenses",
            PrintingExpenses => "Printing Expenses",
            PublicFunds => "Public Funds",
            RadioAdvertisingExpenses => "Radio Advertising Expenses",
            ReimbursedAmount => "Reimbursed Amount",
            ReportedAmount => "Reported Amount",
            Retainer => "Retainer",
            SignExpenses => "Sign Expenses",
            SubContractValue => "Sub-Contract Value",
            TaxReceipts => "Tax Receipts",
            TaxiTravelExpenses => "Taxi Travel Expenses",
            TelecommunicationExpenses => "Telecommunication Expenses",
            TelevisionAdvertisingExpenses => "Television Advertising Expenses",
            UnpaidExpenditure => "Unpaid Expenditure",
            UtilitiesExpenses => "Utilities Expenses",
            Total => "Total",
            Subtotal => "Subtotal",
            GrandTotal => "Grand Total",
            IncidentalExpenses => "Incidental Expenses",
            TransportationExpenses => "Transportation Expenses",
            GiftValue => "Gift Value",
            FoodAndRefreshments => "Food and Refreshments",
            PollingExpenses => "Polling Expenses",
            EstimatedCostOfAttendance => "Estimated Cost of Attendance",
            TuitionAndRequiredFees => "Tuition and Required Fees",
            BooksAndSupplies => "Books and Supplies",
            OtherExpense => "Other Expense",
            EstimatedFinancialAid => "Estimated Financial Aid",
            OtherIncome => "Other Income",
            AmountOfMortgagesAndLiens => "Amount of Mortgages and Liens",
            CodeEI => "Mortgage Payment(s)",
            MaintenanceExpenseOnIncomeProducingProperty => {
                "Maintenance Expense on Income Producing Property"
            }
            CodeEJ => "Insurance, Maintenance, Taxes and Miscellaneous",
            NetRentalIncome => "Net Rental Income",
            PresentMarketValue => "Present Market Value",
            CodeELT => "Electroconvulsive Therapy (ECT) Adjustment",
            GrossRentalIncome => "Gross Rental Income",
            CancellationFee => "Cancellation Fee",
            MaintenanceExpenseOnNonIncomeProducingProperty => {
                "Maintenance Expense on Non-Income Producing Property"
            }
            EntitlementAmount => "Entitlement Amount",
            CapitalReserves => "Capital Reserves",
            EmployerAnnualPledgeAmount => "Employer Annual Pledge Amount",
            CondominiumAssociationFees => "Condominium Association Fees",
            HomeownerAssociationFees => "Homeowner Association Fees",
            EarnestMoney => "Earnest Money",
            MortgageInsuranceProceeds => "Mortgage Insurance Proceeds",
            NetProceedsFromSaleOfRealEstateProperty => {
                "Net Proceeds from Sale of Real Estate Property"
            }
            CodeETD => "Ever To Date (ETD) Claim Loss",
            CodeEU => "Insurance Proceeds (Primary Settlement)",
            PresaleProceeds => "Presale Proceeds",
            PledgedSavings => "Pledged Savings",
            AsIsBrokersOpinion => "As Is Broker's Opinion",
            ExplorationCosts => "Exploration Costs",
            SubjectToBrokersOpinion => "Subject To Broker's Opinion",
            UniformCommercialCodeFilingOfficeFee => {
                "Uniform Commercial Code Filing Office Fee"
            }
            AnnualLimit => "Annual Limit",
            CommercialStaffLaborCosts => "Commercial Staff Labor Costs",
            CodeF1 => "Maximum Allowable Cost (MAC) Penalty Copay",
            PatientResponsibilityActual => "Patient Responsibility - Actual",
            PatientResponsibilityEstimated => "Patient Responsibility - Estimated",
            PostageClaimed => "Postage Claimed",
            PatientAmountPaid => "Patient Amount Paid",
            ProviderReserves => "Provider Reserves",
            SalesTax => "Sales Tax",
            UsualAndCustomaryChargeEstimated => "Usual and Customary Charge - Estimated",
            UsualAndCustomaryActual => "Usual and Customary - Actual",
            CoordinationFee => "Coordination Fee",
            CodeFAR => {
                "Federal Housing Administration (FHA) Appraiser Required Repairs and Improvements"
            }
            CalculationFee => "Calculation Fee",
            FinalBalance => "Final Balance",
            ExpectedFamilyContribution => "Expected Family Contribution",
            DirectDepositFlippedToCheck => "Direct Deposit Flipped to Check",
            Fee => "Fee",
            ApplicationFee => "Application Fee",
            LicensingFee => "Licensing Fee",
            RegulatoryFee => "Regulatory Fee",
            FirstInterestPaymentAmount => "First Interest Payment Amount",
            WaiverFee => "Waiver Fee",
            OtherUnlistedAmount => "Other Unlisted Amount",
            Float => "Float",
            FirstLienAdvance => "First Lien Advance",
            FairMarketValue => "Fair Market Value",
            Fine => "Fine",
            FeesPaid => "Fees Paid",
            ForeignAssets => "Foreign Assets",
            FeesPaidYearToDate => "Fees Paid Year to Date",
            FirmContractorShare => "Firm Contractor Share",
            EstimatedGovernmentShare => "Estimated Government Share",
            FacilitiesRefund => "Facilities Refund",
            Expense => "Expense",
            EndorsementPremiumAmount => "Endorsement Premium Amount",
            FuelTrackerRefund => "Fuel Tracker Refund",
            FacilityTaxAmount => "Facility tax amount",
            CommercialStaffIndemnityCharge => "Commercial Staff Indemnity Charge",
            FlatFeePaidToDate => "Flat Fee Paid to Date",
            FlatFeePaidCurrentMonth => "Flat Fee Paid Current Month",
            Endorsement => "Endorsement",
            FirstPayment => "First Payment",
            EarnedIncome => "Earned Income",
            Collateral => "Collateral",
            InitialAdjustmentTotal => "Initial Adjustment Total",
            IndicatedValueBySalesComparisonApproach => {
                "Indicated Value by Sales Comparison Approach"
            }
            IndicatedValueByIncomeApproach => "Indicated Value by Income Approach",
            PricePerUnitArea => "Price per Unit Area",
            ReconciliationOfFinalValueEstimate => {
                "Reconciliation of Final Value Estimate"
            }
            EstimatedMonthlyMarketRent => "Estimated Monthly Market Rent",
            AdjustedSalesPrice => "Adjusted Sales Price",
            SalesOrFinancingConcessions => "Sales or Financing Concessions",
            IndicatedValueByCostApproach => "Indicated Value by Cost Approach",
            AsIsValueOfSiteImprovements => "As-is Value of Site Improvements",
            DepreciatedValueOfImprovements => "Depreciated Value of Improvements",
            Garnishments => "Garnishments",
            AnnualTax => "Annual Tax",
            CodeGB => "Price, High Value",
            CodeGC => "Price, Low Value",
            CorrectedTaxBill => "Corrected Tax Bill",
            GrossClaim => "Gross Claim",
            PhysicalDepreciation => "Physical Depreciation",
            FunctionalDepreciation => "Functional Depreciation",
            ExternalDepreciation => "External Depreciation",
            GiftsNotDeposited => "Gifts Not Deposited",
            GiftAmount => "Gift Amount",
            AdjustedSalesPriceOfComparableSales => {
                "Adjusted Sales Price of Comparable Sales"
            }
            PredominateValue => "Predominate Value",
            AverageCustomerIncome => "Average Customer Income",
            AverageNeighborhoodIncome => "Average Neighborhood Income",
            AverageCustomerPurchase => "Average Customer Purchase",
            WeeklyDollarSales => "Weekly Dollar Sales",
            AverageCaseSales => "Average Case Sales",
            BuyDown => "Buy-down",
            CreditLine => "Credit Line",
            AppraisalRepairAmount => "Appraisal Repair Amount",
            BrokersOpinionRepairAmount => "Brokers Opinion Repair Amount",
            CreditLineAvailable => "Credit Line Available",
            GrossContribution => "Gross Contribution",
            GrantsNotDeposited => "Grants Not Deposited",
            GrossRestoration => "Gross Restoration",
            GrantAmount => "Grant Amount",
            SubsequentAdjustmentsTotal => "Subsequent Adjustments Total",
            GoodStandingTaxPlanPayment => "Good Standing Tax Plan Payment",
            GoodsAndServicesTax => "Goods and Services Tax",
            TotalSupplementalTaxDue => "Total Supplemental Tax Due",
            TaxesPaid => "Taxes Paid",
            TaxInstallmentDue => "Tax Installment Due",
            TaxInstallmentPaid => "Tax Installment Paid",
            TotalSupplementalTaxPaid => "Total Supplemental Tax Paid",
            GrossValue => "Gross Value",
            TotalCharge => "Total Charge",
            TotalCredit => "Total Credit",
            TotalDebit => "Total Debit",
            TotalFinanceCharge => "Total Finance Charge",
            BidAmount => "Bid Amount",
            CodeH0 => {
                "Host Government (government of the institution) Financing for Education"
            }
            LegalReserves => "Legal Reserves",
            Cancellation => "Cancellation",
            DepositInceptionToDate => "Deposit Inception to Date",
            DepositYearToDate => "Deposit Year to Date",
            DumpInRemittance => "Dump in Remittance",
            Earnings => "Earnings",
            LifeInsuranceCashValue => "Life Insurance Cash Value",
            StructureValue => "Structure Value",
            OriginalListPrice => "Original List Price",
            Coin => "Coin",
            Currency => "Currency",
            USTreasuryChecks => "U.S. Treasury Checks",
            PostalMoneyOrders => "Postal Money Orders",
            HomeEquityLineOfCreditDrawAmount => "Home Equity Line of Credit Draw Amount",
            CityChecks => "City Checks",
            HemophiliaAdjustment => "Hemophilia Adjustment",
            OtherChecks => "Other Checks",
            HomeGovernmentFinancingForEducation => {
                "Home Government Financing for Education"
            }
            AnnualSocialSecurityWages => "Annual Social Security Wages",
            AnnualSocialSecurityTips => "Annual Social Security Tips",
            CodeHJ => "Annual Wages, Tips, and Other Compensation",
            SocialSecurityEmployeeTaxWithheld => "Social Security Employee Tax Withheld",
            FederalIncomeTaxWithheld => "Federal Income Tax Withheld",
            AdvanceEarnedIncomeCredit => "Advance Earned Income Credit",
            Commission => "Commission",
            VacationPay => "Vacation Pay",
            HeadOfficeAccount => "Head Office Account",
            GrossPaySubmitted => "Gross Pay Submitted",
            IntersellCommissionSales => "Intersell Commission Sales",
            TotalPayrollApproved => "Total Payroll Approved",
            HolidayPay => "Holiday Pay",
            OvertimePay => "Overtime Pay",
            RegularPay => "Regular Pay",
            SickPay => "Sick Pay",
            SpecialPay => "Special Pay",
            ContractPrice => "Contract Price",
            CommercialSpaceIncome => "Commercial Space Income",
            UtilitiesPaidByOwner => "Utilities Paid by Owner",
            HazardousCleanUpCost => "Hazardous Clean-up Cost",
            Interest => "Interest",
            LifeInsuranceCoverage => "Life Insurance Coverage",
            InvestmentIncome => "Investment Income",
            Income => "Income",
            CodeI3 => "Price, Gross Living Area",
            TotalEstimatedRent => "Total Estimated Rent",
            GrossAnnualIncome => "Gross Annual Income",
            CustodiansSalary => "Custodian's Salary",
            EngineersSalary => "Engineer's Salary",
            ElevatorOperatorsSalary => "Elevator Operator's Salary",
            IndicatedValueByMarketApproachEstimateOfMarketValue => {
                "Indicated Value by Market Approach Estimate of Market Value"
            }
            AdjustedMonthlyRent => "Adjusted Monthly Rent",
            InvestableBalance => "Investable Balance",
            InterBankLoans => "Inter-Bank Loans",
            AccruedUnpaidInterestToBeCapitalized => {
                "Accrued Unpaid Interest To Be Capitalized"
            }
            ImbalanceChargesRefund => "Imbalance Charges Refund",
            ImportDutyAmount => "Import Duty Amount",
            ExciseTaxAmount => "Excise Tax Amount",
            InspectionFee => "Inspection Fee",
            AdjustmentForGrossLivingArea => "Adjustment for Gross Living Area",
            CodeIGT => "Inter-Governmental Transfer (IGT) Payments",
            PredominantPriceHigh => "Predominant Price High",
            IrregularInterestPaymentAmount => "Irregular Interest Payment Amount",
            NetAdjustedMonthlyRent => "Net Adjusted Monthly Rent",
            IndicatedMonthlyMarketRent => "Indicated Monthly Market Rent",
            PredominantPriceLow => "Predominant Price Low",
            AdjustmentForRooms => "Adjustment for Rooms",
            Installment => "Installment",
            InstallmentBalanceAfterTheCurrentInstallmentIsApplied => {
                "Installment Balance After the Current Installment is Applied"
            }
            AdjustmentForBedrooms => "Adjustment for Bedrooms",
            InterestPayableDuringRepaymentPeriod => {
                "Interest Payable During Repayment Period"
            }
            InterestPerDiem => "Interest per Diem",
            ContingentDebt => "Contingent Debt",
            InsuranceRecovery => "Insurance Recovery",
            IndependentScholarship => "Independent Scholarship",
            InterestSinceClaimSubmission => "Interest Since Claim Submission",
            IncentiveFee => "Incentive Fee",
            AccruedUnpaidInterestNotToBeCapitalized => {
                "Accrued Unpaid Interest Not To Be Capitalized"
            }
            UtilitiesAllowance => "Utilities Allowance",
            InvestmentProperty => "Investment Property",
            FurnitureAllowance => "Furniture Allowance",
            Debentures => "Debentures",
            AccountHighBalance => "Account High Balance",
            TrusteeFees => "Trustee Fees",
            LimitedPartnershipCapital => "Limited Partnership Capital",
            CurrentFaceAmount => "Current Face Amount",
            OriginalFaceAmount => "Original Face Amount",
            FixedDefaultNoteHoldersAmount => "Fixed Default Note Holder's Amount",
            InitialMonthlyPayment => "Initial Monthly Payment",
            OriginalPrincipalAndInterestPayment => {
                "Original Principal and Interest Payment"
            }
            FinalPrincipalAndInterestPayment => "Final Principal and Interest Payment",
            ConversionFee => "Conversion Fee",
            EndingBalance => "Ending Balance",
            BeginningBalance => "Beginning Balance",
            Assessment => "Assessment",
            EquityClaimedAsExempt => "Equity Claimed as Exempt",
            CounterClaim => "Counter Claim",
            WeeklyBenefit => "Weekly Benefit",
            Lease => "Lease",
            AdministrativeLoad => "Administrative Load",
            AssetCostApplicableToEntireContract => {
                "Asset Cost Applicable to Entire Contract"
            }
            AssetCostApplicableToPortionOfContract => {
                "Asset Cost Applicable to Portion of Contract"
            }
            AnnualFee => "Annual Fee",
            CostBasis => "Cost Basis",
            DisabilityPremium => "Disability Premium",
            EmployeeAdditionalContribution => "Employee Additional Contribution",
            EmployeeMatchContribution => "Employee Match Contribution",
            EmployerContribution => "Employer Contribution",
            FreeLookValue => "Free Look Value",
            FreeWithdrawalValue => "Free Withdrawal Value",
            FrontEndLoad => "Front End Load",
            GuaranteedMinimumDeathBenefit => "Guaranteed Minimum Death Benefit",
            InterimValue => "Interim Value",
            MonthlyRent => "Monthly Rent",
            Judgment => "Judgment",
            LoanValue => "Loan Value",
            MarketValue => "Market Value",
            MarketValueAdjustedValue => "Market Value Adjusted Value",
            MarketValueAdjustment => "Market Value Adjustment",
            NetContractValue => "Net Contract Value",
            AttorneyAndTrusteeFees => "Attorney and Trustee Fees",
            DiscountedBillsNotDue => "Discounted Bills not Due",
            UnpaidSecurityBalance => "Unpaid Security Balance",
            TotalUnpaidSecurityBalance => "Total Unpaid Security Balance",
            VeteransAffairsFundingFee => "Veterans Affairs Funding Fee",
            InitialTargetFee => "Initial Target Fee",
            MinimumFee => "Minimum Fee",
            MaximumFee => "Maximum Fee",
            Price => "Price",
            CodeK8 => "Special Accounting Classification Reference Number (ACRN) Amount",
            NewPrice => "New Price",
            EstimatedContract => "Estimated Contract",
            EstimatedNetAdjustment => "Estimated Net Adjustment",
            Obligated => "Obligated",
            Undefinitized => "Undefinitized",
            AnnualRevenue => "Annual Revenue",
            NetPaidAmount => "Net Paid Amount",
            NetCollectedAmount => "Net Collected Amount",
            DeductionAmount => "Deduction Amount",
            NetVarianceAmount => "Net Variance Amount",
            MinimumContractAmount => "Minimum Contract Amount",
            ItemGrossAmount => "Item Gross Amount",
            CollectedAmount => "Collected Amount",
            DisbursedAmount => "Disbursed Amount",
            GrossAmountOfPayment => "Gross Amount of Payment",
            CommittedAmount => "Committed Amount",
            PrincipalAndInterest => "Principal and Interest",
            IncrementalOrderAmount => "Incremental Order Amount",
            LiabilityLongTerm => "Liability-Long Term",
            TaxesAndInsurance => "Taxes and Insurance",
            DefaultPrincipal => "Default Principal",
            DefaultInterest => "Default Interest",
            LiabilityShortTerm => "Liability-Short Term",
            DefaultTaxesAndInsurance => "Default Taxes and Insurance",
            MiscellaneousFeeCollections => "Miscellaneous Fee Collections",
            NotToExceedPrice => "Not-To-Exceed Price",
            MortgagorsMonthlyObligations => "Mortgagor's Monthly Obligations",
            Local => "Local",
            LiquidAssets => "Liquid Assets",
            LegalContribution => "Legal Contribution",
            LeaseholdInsuranceAmount => "Leasehold Insurance Amount",
            TotalUnidentifiedPaymentsRejected => "Total Unidentified Payments Rejected",
            TotalCreditsReceived => "Total Credits Received",
            TotalDebitsReceived => "Total Debits Received",
            TotalPreAdvicesReceived => "Total Pre-advices Received",
            TotalPrenotesReceived => "Total Prenotes Received",
            TotalPostAdvicesReceived => "Total Post-advices Received",
            TotalDebitForSettlement => "Total Debit for Settlement",
            Definitized => "Definitized",
            DefinitizedTotal => "Definitized Total",
            LessorsCost => "Lessor's Cost",
            Incremental => "Incremental",
            LandRights => "Land Rights",
            LoanEligibilityAmount => "Loan Eligibility Amount",
            LoanRemittanceOrRepayment => "Loan Remittance or Repayment",
            LaundryIncome => "Laundry Income",
            Baseline => "Baseline",
            LineItemUnitPrice => "Line Item Unit Price",
            LegalAndAudit => "Legal and Audit",
            LoanAmountRequested => "Loan Amount Requested",
            LumpSum => "Lump Sum",
            Limit => "Limit",
            LienPayoff => "Lien Payoff",
            MoneyPurchase => "Money Purchase",
            LowerFund => "Lower Fund",
            ListPrice => "List Price",
            TotalSubjectPropertyLiensPaidByClosing => {
                "Total Subject Property Liens Paid by Closing"
            }
            LeasePurchaseFunds => "Lease Purchase Funds",
            LeasePayments => "Lease Payments",
            MaximumPotentialLiability => "Maximum Potential Liability",
            TotalCreditForSettlement => "Total Credit for Settlement",
            NetSettlement => "Net Settlement",
            TotalLiabilitiesToBePaidAtClosingNotIncludingSubjectPropertyLiens => {
                "Total Liabilities to be Paid at Closing Not Including Subject Property Liens"
            }
            LossOnSaleOfProperty => "Loss on Sale Of Property",
            TotalAward => "Total Award",
            OptionAmount => "Option Amount",
            PlannedPeriodicPayment => "Planned Periodic Payment",
            TaxAndInsuranceEscrowFundBalance => "Tax and Insurance Escrow Fund Balance",
            LoanExpense => "Loan Expense",
            TotalRemainingPrincipalBalanceForTheIssuer => {
                "Total Remaining Principal Balance for the Issuer"
            }
            DelinquentPayment => "Delinquent Payment",
            AmountDueFromBuyerAtAppraisalNoticeDate => {
                "Amount Due from Buyer at Appraisal Notice Date"
            }
            LoansFromOfficers => "Loans from Officers",
            MaximumOutOfPocketAmount => "Maximum Out of Pocket Amount",
            MedicalContribution => "Medical Contribution",
            TaxRateExpressedAsAFlatFee => "Tax rate expressed as a flat fee",
            MinimumAmountOfTaxToBePaid => "Minimum amount of tax to be paid",
            MinimumAmountToWhichTaxRateIsApplied => {
                "Minimum amount to which tax rate is applied"
            }
            MaximumAmountOfTaxToBePaid => "Maximum amount of tax to be paid",
            MaximumAmountToWhichTaxRateIsApplied => {
                "Maximum amount to which tax rate is applied"
            }
            MarkupAmount => "Markup Amount",
            NetOfSurrenderWithdrawal => "Net of Surrender Withdrawal",
            MaximumAmount => "Maximum Amount",
            MiscellaneousAdjustment => "Miscellaneous Adjustment",
            UndistributedBudget => "Undistributed Budget",
            CostOfMoney => "Cost of Money",
            MinimumDue => "Minimum Due",
            MinimumDefaultNoteHoldersCost => "Minimum Default Note Holder's Cost",
            AdministrativeFees => "Administrative Fees",
            MaximumLateCharge => "Maximum Late Charge",
            MinimumLateCharge => "Minimum Late Charge",
            MinimumIncentiveFee => "Minimum Incentive Fee",
            MaximumDefaultNoteHoldersCost => "Maximum Default Note Holder's Cost",
            GrossToPay => "Gross to Pay",
            PriorNetInvoiceTotal => "Prior Net Invoice Total",
            Payout => "Payout",
            ModifiedMortgageAmount => "Modified Mortgage Amount",
            MonthlyLimit => "Monthly Limit",
            MinimumOrderValue => "Minimum Order Value",
            MonthlyPaymentAmount => "Monthly Payment Amount",
            CodeMQ => "Post Tax Equity and Fiscal Responsibility Act (TEFRA) Cost Basis",
            ManagementReserve => "Management Reserve",
            PastDueTaxesAndAssessmentRemainingUnpaid => {
                "Past-Due Taxes and Assessment Remaining Unpaid"
            }
            CodeMT => "Pre Tax Equity and Fiscal Responsibility Act (TEFRA) Cost Basis",
            PremiumTaxPaidOnSurrender => "Premium Tax Paid on Surrender",
            PremiumTaxPaidUpFront => "Premium Tax Paid up Front",
            SalesLoads => "Sales Loads",
            MaximumIncentiveFee => "Maximum Incentive Fee",
            SurrenderValue => "Surrender Value",
            ValuationPrice => "Valuation Price",
            Net => "Net",
            LoansOrFinancialBorrowings => "Loans or Financial Borrowings",
            NetWorth => "Net Worth",
            IndividualIncomeTaxesAndOther => "Individual Income Taxes and Other",
            CorporateIncomeAndExcessProfitsTax => {
                "Corporate Income and Excess Profits Tax"
            }
            ExciseTaxes => "Excise Taxes",
            EstateAndGiftTaxes => "Estate and Gift Taxes",
            CarrierTaxActTaxes => "Carrier Tax Act Taxes",
            FederalUnemploymentTaxActTaxes => "Federal Unemployment Tax Act Taxes",
            MiscellaneousTaxes => "Miscellaneous Taxes",
            CodeN9 => "Withheld and Federal Insurance Contribution Act (FICA) Taxes",
            NetAdjustment => "Net Adjustment",
            NetCompensationPosition => "Net Compensation Position",
            NetBenefit => "Net Benefit",
            NetWorthOfBusinessOwned => "Net Worth of Business Owned",
            NegativeCollectedBalance => "Negative Collected Balance",
            NetContribution => "Net Contribution",
            PerPersonMonthlyLimit => "Per Person Monthly Limit",
            NetBilled => "Net Billed",
            CodeNF => "Monthly Net Fee Position (Excess/Deficit)",
            MedicareCopayment => "Medicare Copayment",
            MedicareDeductible => "Medicare Deductible",
            MedicarePaid => "Medicare Paid",
            OtherInsurancePaidAmount => "Other Insurance Paid Amount",
            TotalInForceAndAppliedCoverage => "Total in Force and Applied Coverage",
            NegativeLedgerBalance => "Negative Ledger Balance",
            NonCollateralizedAmount => "Non-collateralized Amount",
            TransactionFee => "Transaction Fee",
            NonCommissionSales => "Non Commission Sales",
            NetToPayTotal => "Net to Pay Total",
            NoTaxPlanPayment => "No Tax Plan Payment",
            AdjustedNonrecurring => "Adjusted Nonrecurring",
            Nonrecurring => "Nonrecurring",
            NetRestorationExpenses => "Net Restoration Expenses",
            NetSavingsAmount => "Net Savings Amount",
            UnitValue => "Unit Value",
            NewTechnologyAdjustment => "New Technology Adjustment",
            ReinsuranceAmount => "Reinsurance Amount",
            RenewalAmount => "Renewal Amount",
            RetentionPerLife => "Retention Per Life",
            RetentionPerPolicy => "Retention Per Policy",
            CodeNY => "Net Year to Date (Excess/Deficit)",
            EqualizationAccount => "Equalization Account",
            CourtCost => "Court Cost",
            ExtraordinaryIncome => "Extraordinary Income",
            AmountOfFirstMortgageBeingRefinanced => {
                "Amount of First Mortgage Being Refinanced"
            }
            OtherFamilyFinancingForEducation => "Other Family Financing for Education",
            IntangibleAssetsWrittenOff => "Intangible Assets Written Off",
            InterestPayable => "Interest Payable",
            InterestReceivable => "Interest Receivable",
            JointVentureResults => "Joint Venture Results",
            LongTermDebt => "Long Term Debt",
            LongTermProvisions => "Long Term Provisions",
            Loss => "Loss",
            PrincipalBalanceAmount => "Principal Balance Amount",
            OutstandingLoanBalance => "Outstanding Loan Balance",
            OpeningBankCharges => "Opening Bank Charges",
            DraftAmount => "Draft Amount",
            OdorizationCharge => "Odorization Charge",
            MiscellaneousCharges => "Miscellaneous Charges",
            OfficeEquipment => "Office Equipment",
            ContractorsOffer => "Contractor's Offer",
            OperationalFlowOrderCharge => "Operational Flow Order Charge",
            OperationalFlowOrderRefund => "Operational Flow Order Refund",
            CableCharge => "Cable Charge",
            HandlingCharges => "Handling Charges",
            NonCommissionCharges => "Non-commission Charges",
            Merchandise => "Merchandise",
            LetterOfCreditAmount => "Letter of Credit Amount",
            OutstandingBalanceOtherLender => "Outstanding Balance Other Lender",
            OtherLiabilityAmounts => "Other Liability Amounts",
            OtherMonthlyIncome => "Other Monthly Income",
            NegotiatingBankCharges => "Negotiating Bank Charges",
            OperationalNoticeRefund => "Operational Notice Refund",
            Overdrafts => "Overdrafts",
            OriginalPaymentTotal => "Original Payment Total",
            PayrollCosts => "Payroll Costs",
            LetterOfCreditRemainingAmount => "Letter of Credit Remaining Amount",
            OtherSalaries => "Other Salaries",
            CommissionAmendmentCharges => "Commission Amendment Charges",
            Profit => "Profit",
            InpatientOutlierAdjustment => "Inpatient Outlier Adjustment",
            ProfitAndLossDeficit => "Profit and Loss Deficit",
            ProfitAfterExtraordinaryItemsAndBeforeTax => {
                "Profit after Extraordinary Items and before Tax"
            }
            ProfitAfterTaxAndBeforeExtraordinaryItems => {
                "Profit after Tax and Before Extraordinary Items"
            }
            PaymentCommission => "Payment Commission",
            ProfitDistributedToEmployees => "Profit Distributed to Employees",
            Penalty => "Penalty",
            ParentalFinancingForEducation => "Parental Financing for Education",
            PartnersCalendarYearSalary => "Partner's Calendar Year Salary",
            PriorPlanYearGrossSalary => "Prior Plan Year Gross Salary",
            PremiumAmount => "Premium Amount",
            PriorYearsWage => "Prior Year's Wage",
            PartnersTaxYearSalary => "Partner's Tax Year Salary",
            PremiumDue => "Premium Due",
            PartnersK1TaxYearAmount => "Partner's K1 Tax Year Amount",
            PartnersK1CalendarYearAmount => "Partner's K1 Calendar Year Amount",
            CurrentMortgagePrincipalBalance => "Current Mortgage Principal Balance",
            PaymentCancellationTotal => "Payment Cancellation Total",
            PolicyAdvance => "Policy Advance",
            MinimumDeliveryPurchaseAmount => "Minimum Delivery Purchase Amount",
            PenaltyAndInterest => "Penalty and Interest",
            BilledAmount => "Billed Amount",
            CodePBG => "Profit (Loss) Before Grants",
            PositiveCollectedBalance => "Positive Collected Balance",
            ProcessingAllowance => "Processing Allowance",
            PriorContractCostBasis => "Prior Contract Cost Basis",
            PreviousClaimPayments => "Previous Claim Payments",
            PriorContractSurrenderCharge => "Prior Contract Surrender Charge",
            PriorContractValue => "Prior Contract Value",
            Credit => "Credit",
            PlanPeriodElection => "Plan Period Election",
            CodePEX => "Profit (Loss) after Extraordinary Items and Tax",
            Principal => "Principal",
            PortFacilityCharge => "Port Facility Charge",
            Payoff => "Payoff",
            ProposedGrossRentForTheSubjectProperty => {
                "Proposed Gross Rent for the Subject Property"
            }
            PerOccurrenceDeductible => "Per Occurrence Deductible",
            PerOccurrenceMonthlyLimit => "Per Occurrence Monthly Limit",
            PastDue => "Past Due",
            PhotographFee => "Photograph Fee",
            PositiveLedgerBalance => "Positive Ledger Balance",
            LastPremiumAmount => "Last Premium Amount",
            PriorGrossInvoiceTotal => "Prior Gross Invoice Total",
            PercentOverride => "Percent Override",
            PaymentPriorToAdvance => "Payment Prior to Advance",
            PendingNetSaleProceedsFromNonRealEstateAssets => {
                "Pending Net Sale Proceeds from Non-Real Estate Assets"
            }
            PendingNetSaleProceedsFromRealEstateAssets => {
                "Pending Net Sale Proceeds from Real Estate Assets"
            }
            AdvanceAmount => "Advance Amount",
            PerOccurrenceLimit => "Per Occurrence Limit",
            PropertyDamage => "Property Damage",
            PartialPayrollPayment => "Partial Payroll Payment",
            PerOccurrencePerDayLimit => "Per Occurrence per Day Limit",
            PerOccurrenceAggregateLimit => "Per Occurrence Aggregate Limit",
            UnsecuredPriorityClaim => "Unsecured Priority Claim",
            PrepetitionCharges => "Prepetition Charges",
            PerOccurrenceMaximumPerWeekLimit => "Per Occurrence Maximum per Week Limit",
            PerPersonMaximumPerWeekLimit => "Per Person Maximum per Week Limit",
            PerPersonPerDayLimit => "Per Person per Day Limit",
            OriginalPrincipalBalance => "Original Principal Balance",
            AmountOwedToBuyerAtAppraisalNoticeDate => {
                "Amount Owed to Buyer at Appraisal Notice Date"
            }
            LoansToAffiliatedCompanies => "Loans to Affiliated Companies",
            Proposed => "Proposed",
            CodeQ2 => "1035 Exchange",
            CodeQ3 => "401K Transfer",
            TotalPrenotesAccepted => "Total Prenotes Accepted",
            TotalPrenotesRejected => "Total Prenotes Rejected",
            AutomaticPremiumDeduction => "Automatic Premium Deduction",
            TotalPostAdvicesAccepted => "Total Post-advices Accepted",
            TotalPostAdvicesRejected => "Total Post-advices Rejected",
            CashWithApplication => "Cash With Application",
            Combined => "Combined",
            CreditCard => "Credit Card",
            DepositFund => "Deposit Fund",
            DirectBilling => "Direct Billing",
            DiscPremium => "Disc Premium",
            CodeQF => "Electronic Funds Transfer (EFT)",
            GovernmentAllotment => "Government Allotment",
            InitialPremium => "Initial Premium",
            IndividualRetirementAccount60DayRollover => {
                "Individual Retirement Account 60 Day Rollover"
            }
            IndividualRetirementAccountDirectTransfer => {
                "Individual Retirement Account Direct Transfer"
            }
            IndividualRetirementAccountRegularContribution => {
                "Individual Retirement Account Regular Contribution"
            }
            KeoghHr10 => "Keogh/HR 10",
            KeoghHr10Transfer => "Keogh/HR 10 Transfer",
            CodeQN => "Quarterly Net Fee Position (Excess/Deficit)",
            ListBilling => "List Billing",
            ModalPremium => "Modal Premium",
            PayrollTaxes => "Payroll Taxes",
            ParkingIncome => "Parking Income",
            CodeQS => "Non-Qualified (1035 Exchange)",
            PacPreAuthorizedCheck => "PAC - Pre-Authorized Check",
            PayrollDeduction => "Payroll Deduction",
            Pension => "Pension",
            PremiumReceivedWithApplication => "Premium Received With Application",
            ProfitSharingTrust => "Profit Sharing Trust",
            Qualified => "Qualified",
            PaymentAmount => "Payment Amount",
            SpendDown => "Spend Down",
            LoansToParticipants => "Loans to Participants",
            CodeR1 => "Fixed, Liquidated Secured Debt",
            ContingentSecuredDebt => "Contingent Secured Debt",
            DisputedSecuredDebt => "Disputed Secured Debt",
            UnliquidatedSecuredDebt => "Unliquidated Secured Debt",
            CodeR5 => "Fixed, Liquidated Unsecured Debt",
            ContingentUnsecuredDebt => "Contingent Unsecured Debt",
            DisputedUnsecuredDebt => "Disputed Unsecured Debt",
            UnliquidatedUnsecuredDebt => "Unliquidated Unsecured Debt",
            AtTimeOfFiling => "At Time of Filing",
            AcceleratedRoyalty => "Accelerated Royalty",
            PerPersonDeductible => "Per Person Deductible",
            RefundCheck => "Refund Check",
            PerPersonLimit => "Per Person Limit",
            ReservationDemandStorage => "Reservation/Demand - Storage",
            ReservationDemandTransportation => "Reservation/Demand - Transportation",
            RoyaltyDue => "Royalty Due",
            DepositValue => "Deposit Value",
            Restitution => "Restitution",
            BudgetedRedemption => "Budgeted Redemption",
            PerPersonAggregateLimit => "Per Person Aggregate Limit",
            ResidualValue => "Residual Value",
            RateAmount => "Rate Amount",
            ProvisionForLongTermDepreciation => "Provision for Long Term Depreciation",
            RegularRemittance => "Regular Remittance",
            RemittanceRefund => "Remittance Refund",
            ResidentManagersSalary => "Resident Manager's Salary",
            Provisions => "Provisions",
            Repair => "Repair",
            RepackagingCost => "Repackaging Cost",
            RecommendedAmount => "Recommended Amount",
            ReserveRequirementAmount => "Reserve Requirement Amount",
            Reserves => "Reserves",
            LastPayment => "Last Payment",
            TotalDebitsRejected => "Total Debits Rejected",
            TotalPaymentsRejected => "Total Payments Rejected",
            TotalDelinquency => "Total Delinquency",
            TotalPreAdvicesAccepted => "Total Pre-advices Accepted",
            TotalPreAdvicesRejected => "Total Pre-advices Rejected",
            LendersTotalDelinquency => "Lender's Total Delinquency",
            SubmittedChargebackClaimAmount => "Submitted Chargeback Claim Amount",
            SelfFinancingForEducation => "Self-Financing for Education",
            SalaryAmount => "Salary Amount",
            SalaryWithBonuses => "Salary with Bonuses",
            SalaryWithCommissions => "Salary with Commissions",
            SalaryWithSubchapterSCorporationIncome => {
                "Salary with Subchapter S Corporation Income"
            }
            SalaryWithPartnersBonuses => "Salary with Partner's Bonuses",
            SubchapterSCorporation => "Subchapter S Corporation",
            SoleProprietorship => "Sole Proprietorship",
            PeriodRental => "Period Rental",
            SecuredClaimAllowed => "Secured Claim Allowed",
            CampaignSummaryAmount => "Campaign Summary Amount",
            SystemAdjustedAmount => "System Adjusted Amount",
            CodeSAG => "Surplus (Deficit) after Grant",
            Salvage => "Salvage",
            StatedAmount => "Stated Amount",
            TotalServiceCharge => "Total Service Charge",
            StateCareTax => "State Care Tax",
            SalesCharge => "Sales Charge",
            ServiceChargesWhichCannotBeCompensatedByBalances => {
                "Service Charges Which Cannot Be Compensated by Balances"
            }
            ScholarshipFromAdmittingEducationalInstitution => {
                "Scholarship from Admitting Educational Institution"
            }
            SecuredBorrowedFundsNotDeposited => "Secured Borrowed Funds Not Deposited",
            SponsorFinancingForEducation => "Sponsor-Financing for Education",
            SurrenderCharge => "Surrender Charge",
            SubsequentInterestPaymentAmount => "Subsequent Interest Payment Amount",
            SurrenderFull => "Surrender Full",
            SurrenderPartial => "Surrender Partial",
            SecurityPersonnelsSalary => "Security Personnel's Salary",
            Supplemental => "Supplemental",
            InsuranceValue => "Insurance Value",
            DeclaredValue => "Declared Value",
            ShipmentValue => "Shipment Value",
            PayOnDelivery => "Pay on Delivery",
            LandedCostValue => "Landed Cost Value",
            SalesAdministrationExpense => "Sales Administration Expense",
            SpecialCreditorsAmount => "Special Creditors Amount",
            Setoff => "Setoff",
            SalesPrice => "Sales Price",
            SpecialDebtorsAmount => "Special Debtors Amount",
            SecuredClaim => "Secured Claim",
            SettlementRefundAsApprovedByTheFederalEnergyRegulatoryCommission => {
                "Settlement Refund as approved by the Federal Energy Regulatory Commission"
            }
            CampaignSummaryAmountToBeShared => "Campaign Summary Amount to be Shared",
            SharesInSubsidiaryCompanies => "Shares in Subsidiary Companies",
            State => "State",
            Surcharge => "Surcharge",
            FixedMonthlyPrincipalPayment => "Fixed Monthly Principal Payment",
            BaseAwardFee => "Base Award Fee",
            SeveranceTax => "Severance Tax",
            InitialBuydownBalance => "Initial Buydown Balance",
            CertificationFee => "Certification Fee",
            Tax => "Tax",
            ThirdPartyGovernmentFinancingForEducation => {
                "Third-Party Government-Financing for Education"
            }
            Teacher => "Teacher",
            TotalClaimBeforeTaxes => "Total Claim Before Taxes",
            TotalSubmittedCharges => "Total Submitted Charges",
            TotalCurrentBalance => "Total Current Balance",
            TotalClaims => "Total Claims",
            Claim => "Claim",
            TotalCreditsAccepted => "Total Credits Accepted",
            TotalCreditsRejected => "Total Credits Rejected",
            TotalDebitsAccepted => "Total Debits Accepted",
            TotalAnnualSales => "Total Annual Sales",
            TotalAnnualSalesToCustomer => "Total Annual Sales to Customer",
            TotalBuyerClosingCosts => "Total Buyer Closing Costs",
            ProposedCost => "Proposed Cost",
            TotalCommissionFromPrimaryAndSecondarySources => {
                "Total Commission from Primary and Secondary Sources"
            }
            ProposedProfit => "Proposed Profit",
            TotalDepositoryAccounts => "Total Depository Accounts",
            ProposedFee => "Proposed Fee",
            TotalMaintenanceExpenseOnAllNonIncomeProducingProperties => {
                "Total Maintenance Expense on All Non-Income Producing Properties"
            }
            TotalMaintenanceExpenseOnAllIncomeProducingProperties => {
                "Total Maintenance Expense on All Income Producing Properties"
            }
            TotalProposedPrice => "Total Proposed Price",
            AlternateProposedPrice => "Alternate Proposed Price",
            TotalGiftsNotDeposited => "Total Gifts Not Deposited",
            TotalClaimAllowed => "Total Claim Allowed",
            TitleInsuranceAmountOnLoan => "Title Insurance Amount on Loan",
            TotalSelfEmployedIncomeFromPrimaryAndSecondarySources => {
                "Total Self-employed Income from Primary and Secondary Sources"
            }
            TimeAndExpensePaidToDate => "Time and Expense Paid to Date",
            TotalAmountOfContract => "Total Amount of Contract",
            TotalPriorLoanAmountOwed => "Total Prior Loan Amount Owed",
            TotalOtherLiquidAssets => "Total Other Liquid Assets",
            TotalLifeInsuranceNetCashValue => "Total Life Insurance Net Cash Value",
            TimeAndExpensePaidCurrentMonth => "Time and Expense Paid Current Month",
            TotalMonetaryMarkupAmount => "Total Monetary Markup Amount",
            TelephoneOperatorsSalary => "Telephone Operator's Salary",
            TotalOmittedLiabilities => "Total Omitted Liabilities",
            TotalPaymentAmount => "Total payment amount",
            TotalPreviousAdjustedClaim => "Total Previous Adjusted Claim",
            TotalNetProceedsFromRealEstateAssets => {
                "Total Net Proceeds from Real Estate Assets"
            }
            TotalPaidAsSubmitted => "Total Paid as Submitted",
            SubsidiesForOperatingCosts => "Subsidies for Operating Costs",
            TargetCost => "Target Cost",
            TotalRetirementFunds => "Total Retirement Funds",
            TotalNonRentalIncome => "Total Non-rental Income",
            TotalResubordinatedLiabilities => "Total Resubordinated Liabilities",
            TotalLiabilitiesForRentalProperties => {
                "Total Liabilities for Rental Properties"
            }
            TotalSales => "Total Sales",
            TotalStocksAndBonds => "Total Stocks and Bonds",
            TotalTransactionAmount => "Total Transaction Amount",
            TransportationCostPerUnitOfMeasure => {
                "Transportation Cost per Unit of Measure"
            }
            LevelOfPremiumInsuranceRetention => "Level of Premium Insurance - Retention",
            TechniciansIndemnityProvision => "Technicians Indemnity Provision",
            TotalToDate => "Total to Date",
            TotalAtComplete => "Total at Complete",
            TransportationCostTotal => "Transportation Cost Total",
            Underpayment => "Underpayment",
            USGovernmentFinancingForEducation => {
                "U.S. Government-Financing for Education"
            }
            CodeU1 => "Unsecured, Priority Claim Allowed",
            IngredientCostClaimed => "Ingredient Cost Claimed",
            MiscellaneousExpenses => "Miscellaneous Expenses",
            PresentValueOfLot => "Present Value of Lot",
            CostOfImprovements => "Cost of Improvements",
            CodeU6 => "Alterations, Improvements, Repairs",
            Land => "Land",
            Refinance => "Refinance",
            TaxPaymentRefinancedBySameLender => "Tax Payment Refinanced by Same Lender",
            EstimatedPrepaidItems => "Estimated Prepaid Items",
            UnliquidatedAmount => "Unliquidated Amount",
            UserAdjustedAmount => "User Adjusted Amount",
            CodeUAR => "Refund of Unauthorized Overrun Charges (UAOR Refund)",
            UnpaidPrincipalBalance => "Unpaid Principal Balance",
            UnspecifiedAggregateLimit => "Unspecified Aggregate Limit",
            CodeUD => "Unsecured, Nonpriority Claim Allowed",
            MortgageInsurance => "Mortgage Insurance",
            UpdatedExpensesOnPresale => "Updated Expenses on Presale",
            CodeUF => "Discount (If Borrower Paid)",
            UnsecuredBorrowedFundsNotDeposited => {
                "Unsecured Borrowed Funds Not Deposited"
            }
            TotalUnpaidPrincipalBalanceForStaffordLoans => {
                "Total Unpaid Principal Balance for Stafford Loans"
            }
            SubordinateFinancing => "Subordinate Financing",
            UnsecuredHomeImprovements => "Unsecured Home Improvements",
            TotalCosts => "Total Costs",
            UpdatedInterestOnPresale => "Updated Interest on Presale",
            OtherCredits => "Other Credits",
            CodeUK => "Base Loan Amount (w/o financed Mortgage Insurance)",
            MortgageInsuranceFinanced => "Mortgage Insurance Financed",
            TotalLoanAmount => "Total Loan Amount",
            CodeUN => "Unsecured, Nonpriority Claim",
            UnknownTaxPlanPayment => "Unknown Tax Plan Payment",
            CashFromOrToBorrower => "Cash from or to Borrower",
            TotalUnpaidPrincipalBalanceForParentalLoansForStudents => {
                "Total Unpaid Principal Balance for Parental Loans for Students"
            }
            UpperFund => "Upper Fund",
            MonthlyIncome => "Monthly Income",
            UnearnedIncome => "Unearned Income",
            TotalUnpaidPrincipalBalanceForSupplementalLoansForStudents => {
                "Total Unpaid Principal Balance for Supplemental Loans for Students"
            }
            ValueAddedSales => "Value Added Sales",
            ClearingHouseSettlement => "Clearing House Settlement",
            Drawback => "Drawback",
            TotalMonthlyLiabilities => "Total Monthly Liabilities",
            CodeUX => "Utilities, Furniture, and Amenities Included in Rent",
            TotalAssets => "Total Assets",
            TotalLiquidAssets => "Total Liquid Assets",
            CostOfDeficiency => "Cost of Deficiency",
            ValueAdded => "Value Added",
            TaxAndInsuranceEscrowFund => "Tax and Insurance Escrow Fund",
            InterestDueToInvestor => "Interest Due to Investor",
            TotalPrincipalDueToTheInvestor => "Total Principal Due to the Investor",
            TotalInterestDueToTheInvestor => "Total Interest Due to the Investor",
            TotalCurtailmentDueToTheInvestor => "Total Curtailment Due to the Investor",
            TotalPrincipalPayoffAndRepurchaseDueToTheInvestor => {
                "Total Principal Payoff and Repurchase Due to the Investor"
            }
            TotalInterestPayoffAndRepurchaseDueToTheInvestor => {
                "Total Interest Payoff and Repurchase Due to the Investor"
            }
            ActualOutstandingPrincipalBalance => "Actual Outstanding Principal Balance",
            FaceAmount => "Face Amount",
            CodeVA => "Total Current Rent or Mortgage Payment (Issue)",
            TotalNonLiquidAssets => "Total Non-liquid Assets",
            Authorized => "Authorized",
            ActualPersonDayRate => "Actual Person Day Rate",
            EstimatedPersonDayRate => "Estimated Person Day Rate",
            VestedEarnedUpperFund => "Vested/Earned Upper Fund",
            TotalMonthlyExpenses => "Total Monthly Expenses",
            CurrentMonthlyPrincipalAndInterest => {
                "Current Monthly Principal and Interest"
            }
            LevyAmount => "Levy Amount",
            CurrentSupport => "Current Support",
            PastDueSupport => "Past Due Support",
            MedicalSupport => "Medical Support",
            NetNegativeAmortizationAmount => "Net Negative Amortization Amount",
            WithholdFromWages => "Withhold From Wages",
            CommissionBasis => "Commission Basis",
            CommissionEarned => "Commission Earned",
            CurrentMonthlyPayment => "Current Monthly Payment",
            CommissionNetted => "Commission Netted",
            TotalMonthlyDebt => "Total Monthly Debt",
            VolumetricReservation => "Volumetric Reservation",
            OtherFinancingPayment => "Other Financing Payment",
            ValueOfSecuritiesAtIssueDate => "Value of Securities at Issue Date",
            ValueOfSecuritiesAtMaturity => "Value of Securities at Maturity",
            CurrentValue => "Current Value",
            ClosingCost => "Closing Cost",
            CapitalizedMortgageAmount => "Capitalized Mortgage Amount",
            FirstMortgageMonthlyPrincipalAndInterest => {
                "First Mortgage Monthly Principal and Interest"
            }
            InterestAmountPaidToDate => "Interest Amount Paid to Date",
            MinimumTransfer => "Minimum Transfer",
            MaximumTransfer => "Maximum Transfer",
            DeficiencyJudgmentFees => "Deficiency Judgment Fees",
            TradeDebtors => "Trade Debtors",
            W2 => "W-2",
            W2WithBonuses => "W-2 with Bonuses",
            W2WithDeferredCompensation => "W-2 with Deferred Compensation",
            W2WithoutBonuses => "W-2 without Bonuses",
            DepositSubTotal => "Deposit Sub Total",
            DirectRollover => "Direct Rollover",
            DirectTransfer => "Direct Transfer",
            Discounted => "Discounted",
            SecondaryFinance => "Secondary Finance",
            MinimumDeposit => "Minimum Deposit",
            SubAgencyCompensation => "Sub-Agency Compensation",
            BuyersAgencyCompensation => "Buyers Agency Compensation",
            VariableRateCompensation => "Variable Rate Compensation",
            CompensationBonusOnSaleOfProperty => "Compensation Bonus on Sale of Property",
            VeteransAffairsLoanGuarantee => "Veterans Affairs Loan Guarantee",
            SecurityTradeAmount => "Security Trade Amount",
            CodeWH => "Balance Owing All Other Liens, Subject Property",
            OtherFinancing => "Other Financing",
            DualAgencyCompensation => "Dual Agency Compensation",
            PerWeekLimit => "Per Week Limit",
            LendersOpinionOfValue => "Lender's Opinion of Value",
            TotalOriginalPrincipalBalance => "Total Original Principal Balance",
            OtherAgentCompensation => "Other Agent Compensation",
            DockUsageFee => "Dock Usage Fee",
            PoolUsageFee => "Pool Usage Fee",
            ClubhouseFee => "Clubhouse Fee",
            OptionalServiceFee => "Optional Service Fee",
            OtherAssociationFees => "Other Association Fees",
            CodeWT => "Principal, Interest, Taxes",
            CodeWU => "Principal, Interest, Taxes and Insurance",
            TotalPointsPaidAtClosing => "Total Points Paid at Closing",
            AmountThatWouldHaveBeenPaidInTheAbsenceOfCapitation => {
                "Amount that Would Have Been Paid in the Absence of Capitation"
            }
            PointsPaidBySeller => "Points Paid by Seller",
            LoanWithdrawal => "Loan Withdrawal",
            SeverancePay => "Severance Pay",
            DeficiencyJudgmentExpensesAndFees => "Deficiency Judgment Expenses and Fees",
            TrebleDamages => "Treble Damages",
            TransferToUntaxedReserves => "Transfer to Untaxed Reserves",
            Reissued => "Reissued",
            RolloverAmount => "Rollover Amount",
            AnnualRental => "Annual Rental",
            GrossMonthlyRent => "Gross Monthly Rent",
            SepSelfEmployeePension => "SEP - Self Employee Pension",
            FundingAmount => "Funding Amount",
            CodeX9 => "Tax Sheltered Annuity (403B Transfer)",
            MaximumAwardFee => "Maximum Award Fee",
            MaturityValue => "Maturity Value",
            EarnedWages => "Earned Wages",
            BasePeriodWage => "Base Period Wage",
            Withdrawal => "Withdrawal",
            WithdrawalInceptionToDate => "Withdrawal Inception to Date",
            WithdrawalLessMarketValueAdjustment => {
                "Withdrawal Less Market Value Adjustment"
            }
            WithdrawalLessSurrender => "Withdrawal Less Surrender",
            WithdrawalLessTaxes => "Withdrawal Less Taxes",
            WithdrawalYearToDate => "Withdrawal Year to Date",
            UnavailableReserves => "Unavailable Reserves",
            UncalledCapital => "Uncalled Capital",
            UnemploymentContribution => "Unemployment Contribution",
            CodeXN => "Unlimited Capital, Minimum Fixed",
            UnpaidCapital => "Unpaid Capital",
            UnsecuredLiabilities => "Unsecured Liabilities",
            ValueAddedTax => "Value Added Tax",
            ValueOfShares => "Value of Shares",
            Vehicles => "Vehicles",
            VoluntaryReserves => "Voluntary Reserves",
            Extraction => "Extraction",
            Wages => "Wages",
            Withholding => "Withholding",
            OriginalValue => "Original Value",
            WorkingCapital => "Working Capital",
            SalesPricePerDwellingUnit => "Sales Price Per Dwelling Unit",
            SalesPricePerRoom => "Sales Price Per Room",
            CurrentListPrice => "Current List Price",
            SelfInsuranceAmount => "Self Insurance Amount",
            YearToDateEligibleSalary => "Year to Date Eligible Salary",
            TotalRealEstateOwned => "Total Real Estate Owned",
            TotalLiabilities => "Total Liabilities",
            TotalLiabilityMonthlyPayments => "Total Liability Monthly Payments",
            TotalRealEstateOwnedMarketValue => "Total Real Estate Owned Market Value",
            TotalRealEstateOwnedGrossRentalIncome => {
                "Total Real Estate Owned Gross Rental Income"
            }
            TotalRealEstateOwnedMortgagesAndLiens => {
                "Total Real Estate Owned Mortgages and Liens"
            }
            TotalRealEstateOwnedMortgagePayments => {
                "Total Real Estate Owned Mortgage Payments"
            }
            TotalRealEstateOwnedMiscellaneousExpenses => {
                "Total Real Estate Owned Miscellaneous Expenses"
            }
            TotalRealEstateOwnedNetRentalIncome => {
                "Total Real Estate Owned Net Rental Income"
            }
            ActualUnpaidPrincipalBalance => "Actual Unpaid Principal Balance",
            ScheduledUnpaidPrincipalBalance => "Scheduled Unpaid Principal Balance",
            PrincipalDueToInvestor => "Principal Due to Investor",
            ConstantPrincipalAndInterest => "Constant Principal and Interest",
            OtherFeeCollection => "Other Fee Collection",
            BeginningScheduledUnpaidPrincipalBalance => {
                "Beginning Scheduled Unpaid Principal Balance"
            }
            TaxAndInsurancePrincipalBalance => "Tax and Insurance Principal Balance",
            NewPrincipalAndInterest => "New Principal and Interest",
            Curtailment => "Curtailment",
            PrepaymentPenalty => "Prepayment Penalty",
            PartialAnnuitization => "Partial Annuitization",
            PartialWithdrawal => "Partial Withdrawal",
            CodeYN => "Post Tax Equity and Fiscal Responsibility Act (TEFRA) Gain",
            CodeYO => "Pre Tax Equity and Fiscal Responsibility Act (TEFRA) Gain",
            PolicyAmount => "Policy Amount",
            PaymentsInAdvance => "Payments in Advance",
            PaymentsInArrears => "Payments in Arrears",
            Cancelled => "Cancelled",
            Denied => "Denied",
            InProcess => "In Process",
            Requested => "Requested",
            Paid => "Paid",
            PaidForThisFacility => "Paid for This Facility",
            Returned => "Returned",
            TotalAggregateLimit => "Total Aggregate Limit",
            ListPriceWhenSold => "List Price When Sold",
            InsertionCost => "Insertion Cost",
            RepackagingLaborCost => "Repackaging Labor Cost",
            RepackagingMaterialCost => "Repackaging Material Cost",
            UnitCostOfDiscrepantMaterial => "Unit Cost of Discrepant Material",
            LiquidationPrincipal => "Liquidation Principal",
            RemainingPoolBalance => "Remaining Pool Balance",
            RemainingSecurityBalance => "Remaining Security Balance",
            ProgramCost => "Program Cost",
            OverrideToHandlingFee => "Override to Handling Fee",
            ProductionCost => "Production Cost",
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
            FederalPensionMandateCategory4 => "Federal Pension Mandate - Category 4",
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
            CouponFaceValue => "Coupon Face Value",
            InitialTargetCost => "Initial Target Cost",
            Increase => "Increase",
            Decrease => "Decrease",
            ProratedAmount => "Prorated Amount",
            LoanCharge => "Loan Charge",
            MortgageRecordingFee => "Mortgage Recording Fee",
            DeedRecordingFee => "Deed Recording Fee",
            ReleaseRecordingFee => "Release Recording Fee",
            Assumption => "Assumption",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<AmountQualifierCode> {
        {
            use AmountQualifierCode::*;
            match description {
                "Downpayment on the Repayment Plan Amount" => {
                    Some(DownpaymentOnTheRepaymentPlanAmount)
                }
                "Interest Substitution Adjustment" => {
                    Some(InterestSubstitutionAdjustment)
                }
                "Principal Substitution Adjustment" => {
                    Some(PrincipalSubstitutionAdjustment)
                }
                "Prepaid Interest" => Some(PrepaidInterest),
                "Prepaid Principal" => Some(PrepaidPrincipal),
                "Delinquent Interest" => Some(DelinquentInterest),
                "Delinquent Principal" => Some(DelinquentPrincipal),
                "Curtailment Adjustment" => Some(CurtailmentAdjustment),
                "Serial Note Principal Available for Distribution" => {
                    Some(SerialNotePrincipalAvailableForDistribution)
                }
                "Servicing Fee" => Some(ServicingFee),
                "Guarantee Fee Adjustment" => Some(GuaranteeFeeAdjustment),
                "Amount Under-collateralized" => Some(AmountUnderCollateralized),
                "Amount Over-collateralized" => Some(AmountOverCollateralized),
                "Trial Balance Adjustment" => Some(TrialBalanceAdjustment),
                "Custodial Bank Account Adjustment" => {
                    Some(CustodialBankAccountAdjustment)
                }
                "Item" => Some(Item),
                "Schedule" => Some(Schedule),
                "Regular Plan" => Some(RegularPlan),
                "Previously Billed" => Some(PreviouslyBilled),
                "Currently Due" => Some(CurrentlyDue),
                "Coverage Premium" => Some(CoveragePremium),
                "Line Item Total" => Some(LineItemTotal),
                "Fixed Installment Control Accounting Error" => {
                    Some(FixedInstallmentControlAccountingError)
                }
                "Variance" => Some(Variance),
                "Variance Adjustment Cost" => Some(VarianceAdjustmentCost),
                "Variance Adjustment Schedule" => Some(VarianceAdjustmentSchedule),
                "Escalation" => Some(Escalation),
                "Fixed Price" => Some(FixedPrice),
                "Lodging" => Some(Lodging),
                "Meals" => Some(Meals),
                "Travel Expense" => Some(TravelExpense),
                "Insurance Expense" => Some(InsuranceExpense),
                "Union Dues" => Some(UnionDues),
                "Regular Income" => Some(RegularIncome),
                "Income on Real Property" => Some(IncomeOnRealProperty),
                "Income from Social Security and/or other Government Assistance" => {
                    Some(IncomeFromSocialSecurityAndOrOtherGovernmentAssistance)
                }
                "Total Monthly Income" => Some(TotalMonthlyIncome),
                "Electric and/or Fuel Payment" => Some(ElectricAndOrFuelPayment),
                "Water and/or Sewer Payment" => Some(WaterAndOrSewerPayment),
                "Telephone Payment" => Some(TelephonePayment),
                "Maintenance Expense" => Some(MaintenanceExpense),
                "Food Expense" => Some(FoodExpense),
                "Clothing Expense" => Some(ClothingExpense),
                "Laundry Expense" => Some(LaundryExpense),
                "Medical and/or Dental Expense" => Some(MedicalAndOrDentalExpense),
                "Recreation Expenses" => Some(RecreationExpenses),
                "Charitable Contributions" => Some(CharitableContributions),
                "Batch Total" => Some(BatchTotal),
                "Graduated Payment Mortgage Adjustment" => {
                    Some(GraduatedPaymentMortgageAdjustment)
                }
                "Home Insurance Expense" => Some(HomeInsuranceExpense),
                "Life Insurance Expense" => Some(LifeInsuranceExpense),
                "Health Insurance Expense" => Some(HealthInsuranceExpense),
                "Automobile Insurance Expense" => Some(AutomobileInsuranceExpense),
                "Value of Property Claimed as Exempt" => {
                    Some(ValueOfPropertyClaimedAsExempt)
                }
                "Automobile Payment" => Some(AutomobilePayment),
                "Other Type of Installment Payment" => {
                    Some(OtherTypeOfInstallmentPayment)
                }
                "Operating Expenses" => Some(OperatingExpenses),
                "Total Projected Monthly Income" => Some(TotalProjectedMonthlyIncome),
                "Total Projected Monthly Expenses" => Some(TotalProjectedMonthlyExpenses),
                "Excess Income" => Some(ExcessIncome),
                "Value of Personal Property" => Some(ValueOfPersonalProperty),
                "Monthly Overtime" => Some(MonthlyOvertime),
                "Total All Repairs" => Some(TotalAllRepairs),
                "Total Recommended Repairs" => Some(TotalRecommendedRepairs),
                "State Quarterly Total Gross Wages" => {
                    Some(StateQuarterlyTotalGrossWages)
                }
                "State Quarterly Unemployment Insurance (UI) Total Wages" => Some(Code2R),
                "State Quarterly Unemployment Insurance (UI) Excess Wages" => {
                    Some(Code2S)
                }
                "State Quarterly Unemployment Insurance (UI) Taxable Wages" => {
                    Some(Code2T)
                }
                "State Quarterly Disability Insurance Taxable Wages" => {
                    Some(StateQuarterlyDisabilityInsuranceTaxableWages)
                }
                "State Quarterly Tip Wages" => Some(StateQuarterlyTipWages),
                "Asset-Long Term" => Some(AssetLongTerm),
                "Asset-Short Term" => Some(AssetShortTerm),
                "Base Coverage" => Some(BaseCoverage),
                "Commission Retained" => Some(CommissionRetained),
                "Deposit Total" => Some(DepositTotal),
                "Growing Equity Mortgage Adjustment" => {
                    Some(GrowingEquityMortgageAdjustment)
                }
                "Accounting" => Some(Accounting),
                "Accounts Payable" => Some(AccountsPayable),
                "Accounts Receivable" => Some(AccountsReceivable),
                "Advanced Dividends" => Some(AdvancedDividends),
                "Advertising Expenses" => Some(AdvertisingExpenses),
                "Amortization" => Some(Amortization),
                "Amortization Costs" => Some(AmortizationCosts),
                "Amount of Decree" => Some(AmountOfDecree),
                "Asset Investment" => Some(AssetInvestment),
                "Authorized Capital" => Some(AuthorizedCapital),
                "Available Reserves" => Some(AvailableReserves),
                "Bad Debt Allowance" => Some(BadDebtAllowance),
                "Bad Debts" => Some(BadDebts),
                "Bank Account(s)" => Some(Code3N),
                "Long Term Assets" => Some(LongTermAssets),
                "Long Term Liabilities" => Some(LongTermLiabilities),
                "Long Term Tangible Assets" => Some(LongTermTangibleAssets),
                "Losses on Capital" => Some(LossesOnCapital),
                "Machines and Tools" => Some(MachinesAndTools),
                "Member Risk Capital" => Some(MemberRiskCapital),
                "Miscellaneous After Tax Exempt" => Some(MiscellaneousAfterTaxExempt),
                "Mortgage" => Some(Mortgage),
                "Nominal Capital" => Some(NominalCapital),
                "Nominal Damages" => Some(NominalDamages),
                "Non-operational Fixed Assets" => Some(NonOperationalFixedAssets),
                "Excess Amount Requested" => Some(ExcessAmountRequested),
                "Lock Box Total" => Some(LockBoxTotal),
                "Adjustable Rate Mortgage Change" => Some(AdjustableRateMortgageChange),
                "Nonissued Capital" => Some(NonissuedCapital),
                "Notes Payable" => Some(NotesPayable),
                "Notes Receivable" => Some(NotesReceivable),
                "Bank Debentures" => Some(BankDebentures),
                "Bank Obligations" => Some(BankObligations),
                "Buildings" => Some(Buildings),
                "Buildings Under Construction" => Some(BuildingsUnderConstruction),
                "Capital" => Some(Capital),
                "Capital Associated with Principal" => {
                    Some(CapitalAssociatedWithPrincipal)
                }
                "Capital of Other Subsidiaries" => Some(CapitalOfOtherSubsidiaries),
                "Capital Stock" => Some(CapitalStock),
                "Cash" => Some(Cash),
                "Capital Subsidies Received" => Some(CapitalSubsidiesReceived),
                "Commercial Debt" => Some(CommercialDebt),
                "Commercial Expenses" => Some(CommercialExpenses),
                "Common Stock" => Some(CommonStock),
                "Consequential Damages" => Some(ConsequentialDamages),
                "Compensatory Damages" => Some(CompensatoryDamages),
                "Convertible Debentures" => Some(ConvertibleDebentures),
                "Cost of Goods Sold" => Some(CostOfGoodsSold),
                "Cost of Sales" => Some(CostOfSales),
                "Cost(s)" => Some(Code4V),
                "Current Assets" => Some(CurrentAssets),
                "Current Liabilities" => Some(CurrentLiabilities),
                "Damages" => Some(Damages),
                "Deferred Cost" => Some(DeferredCost),
                "Total Invoice Amount" => Some(TotalInvoiceAmount),
                "Fixed Installment Control Substitution Adjustment" => {
                    Some(FixedInstallmentControlSubstitutionAdjustment)
                }
                "Deferred Credit or Income" => Some(DeferredCreditOrIncome),
                "Deferred Taxation" => Some(DeferredTaxation),
                "Deposits" => Some(Deposits),
                "Depreciation" => Some(Depreciation),
                "Depreciation of Fixed Assets" => Some(DepreciationOfFixedAssets),
                "Depreciation of Revaluation of Fixed Assets" => {
                    Some(DepreciationOfRevaluationOfFixedAssets)
                }
                "Director's Remuneration" => Some(DirectorsRemuneration),
                "Dividends" => Some(Dividends),
                "Doubtful Receivables" => Some(DoubtfulReceivables),
                "Equipment" => Some(Equipment),
                "Equipment Subsidies" => Some(EquipmentSubsidies),
                "Equities, Stocks" => Some(Code5L),
                "Equity" => Some(Equity),
                "Exceptional Item" => Some(ExceptionalItem),
                "Exports" => Some(Exports),
                "External Charge" => Some(ExternalCharge),
                "Extraordinary Charge" => Some(ExtraordinaryCharge),
                "Extraordinary Current Asset Write Downs" => {
                    Some(ExtraordinaryCurrentAssetWriteDowns)
                }
                "Extraordinary Result" => Some(ExtraordinaryResult),
                "Financial Assets" => Some(FinancialAssets),
                "Financial Charges" => Some(FinancialCharges),
                "Financial Debt" => Some(FinancialDebt),
                "Financial Expenses" => Some(FinancialExpenses),
                "Financial Income" => Some(FinancialIncome),
                "Finished Goods" => Some(FinishedGoods),
                "Fixed Asset Debts" => Some(FixedAssetDebts),
                "Amount Subject to Total Monetary Discount" => {
                    Some(AmountSubjectToTotalMonetaryDiscount)
                }
                "Interest Adjustment" => Some(InterestAdjustment),
                "Fixed Assets" => Some(FixedAssets),
                "Fixed Assets for Sale" => Some(FixedAssetsForSale),
                "Fixtures" => Some(Fixtures),
                "Fixtures and Equipment" => Some(FixturesAndEquipment),
                "Franchise" => Some(Franchise),
                "Franchise Tax Balance" => Some(FranchiseTaxBalance),
                "Franchise Tax Paid" => Some(FranchiseTaxPaid),
                "Free Reserves" => Some(FreeReserves),
                "Furniture" => Some(Furniture),
                "Future Loan" => Some(FutureLoan),
                "General Accounts" => Some(GeneralAccounts),
                "General Expenses" => Some(GeneralExpenses),
                "Goodwill" => Some(Goodwill),
                "Grants for Operating Costs" => Some(GrantsForOperatingCosts),
                "Group Related Financial Income" => Some(GroupRelatedFinancialIncome),
                "Income Stated in Advance" => Some(IncomeStatedInAdvance),
                "Income Tax" => Some(IncomeTax),
                "Income Tax, Corporate" => Some(Code6R),
                "Income Tax, Noncorporate" => Some(Code6S),
                "Injunction" => Some(Injunction),
                "Intangible Depreciation" => Some(IntangibleDepreciation),
                "Intangibles" => Some(Intangibles),
                "Interest of Third Party" => Some(InterestOfThirdParty),
                "Interest on Loans" => Some(InterestOnLoans),
                "Operating Income (Loss)" => Some(Code6Y),
                "Optional Reserves" => Some(OptionalReserves),
                "Discount Amount Due" => Some(DiscountAmountDue),
                "Deferred Graduated Payment Mortgage Interest Paid" => {
                    Some(DeferredGraduatedPaymentMortgageInterestPaid)
                }
                "Organizational Expenses" => Some(OrganizationalExpenses),
                "Outside Share in Profit or Loss" => Some(OutsideShareInProfitOrLoss),
                "Outstanding Debts against Board of Directors/Managers" => {
                    Some(OutstandingDebtsAgainstBoardOfDirectorsManagers)
                }
                "Owing" => Some(Owing),
                "Owing from Affiliates" => Some(OwingFromAffiliates),
                "Owing from Participants" => Some(OwingFromParticipants),
                "Owing to Affiliates" => Some(OwingToAffiliates),
                "Owing to Fiscal Office" => Some(OwingToFiscalOffice),
                "Owing to National Social Security Office" => {
                    Some(OwingToNationalSocialSecurityOffice)
                }
                "Owing to Participants" => Some(OwingToParticipants),
                "Own Work Capitalized" => Some(OwnWorkCapitalized),
                "Paid in Capital" => Some(PaidInCapital),
                "Par Value" => Some(ParValue),
                "Participating Interest" => Some(ParticipatingInterest),
                "Patents" => Some(Patents),
                "Pension Debts" => Some(PensionDebts),
                "Pensions Provision" => Some(PensionsProvision),
                "Preferred Stock" => Some(PreferredStock),
                "Prepaid Orders in Progress" => Some(PrepaidOrdersInProgress),
                "Prior Results Carried Forward" => Some(PriorResultsCarriedForward),
                "Profit or Loss" => Some(ProfitOrLoss),
                "Profit or Loss after Taxes" => Some(ProfitOrLossAfterTaxes),
                "Profit or Loss before Taxes" => Some(ProfitOrLossBeforeTaxes),
                "Profit or Loss on Ordinary Activities after Tax" => {
                    Some(ProfitOrLossOnOrdinaryActivitiesAfterTax)
                }
                "Progress Payments" => Some(ProgressPayments),
                "Proposed Dividend" => Some(ProposedDividend),
                "Total Monetary Discount Amount" => Some(TotalMonetaryDiscountAmount),
                "Interest Accounting Error" => Some(InterestAccountingError),
                "Provision for Depreciation of Stock or Inventory" => {
                    Some(ProvisionForDepreciationOfStockOrInventory)
                }
                "Provision for Future Purchases" => Some(ProvisionForFuturePurchases),
                "Provision for Risks" => Some(ProvisionForRisks),
                "Punitive Damages" => Some(PunitiveDamages),
                "Purchase Price" => Some(PurchasePrice),
                "Purchases" => Some(Purchases),
                "Raw Materials" => Some(RawMaterials),
                "Real Estate" => Some(RealEstate),
                "Receivables" => Some(Receivables),
                "Regularization Account" => Some(RegularizationAccount),
                "Research and Development" => Some(ResearchAndDevelopment),
                "Restructuring Costs" => Some(RestructuringCosts),
                "Result" => Some(Result),
                "Retained Earnings" => Some(RetainedEarnings),
                "Revenues" => Some(Revenues),
                "Sales" => Some(Sales),
                "Sales and Use Tax" => Some(SalesAndUseTax),
                "Savings" => Some(Savings),
                "Secured Liability" => Some(SecuredLiability),
                "Secured Loans" => Some(SecuredLoans),
                "Selling Expenses" => Some(SellingExpenses),
                "Services" => Some(Services),
                "Share Capital" => Some(ShareCapital),
                "Share in Profit or Loss of Minority Interest" => {
                    Some(ShareInProfitOrLossOfMinorityInterest)
                }
                "Share Premium Capital" => Some(SharePremiumCapital),
                "Shares in Affiliated Companies" => Some(SharesInAffiliatedCompanies),
                "Total Operational Statement Amount" => {
                    Some(TotalOperationalStatementAmount)
                }
                "Principal Accounting Error" => Some(PrincipalAccountingError),
                "Social Charges" => Some(SocialCharges),
                "Social Security (FICA)" => Some(Code9B),
                "Special Reserves" => Some(SpecialReserves),
                "Specially Secured Creditors" => Some(SpeciallySecuredCreditors),
                "Specific Performance" => Some(SpecificPerformance),
                "Starting Capital" => Some(StartingCapital),
                "Statutory Reserves" => Some(StatutoryReserves),
                "Subscribed Capital" => Some(SubscribedCapital),
                "Suit Amount" => Some(SuitAmount),
                "Supplies" => Some(Supplies),
                "Surplus of Revaluation" => Some(SurplusOfRevaluation),
                "Tangible Net Worth" => Some(TangibleNetWorth),
                "Tax Adjustments" => Some(TaxAdjustments),
                "Tax Balance" => Some(TaxBalance),
                "Tax Capital Amount" => Some(TaxCapitalAmount),
                "Tax on Extraordinary Items" => Some(TaxOnExtraordinaryItems),
                "Tax Recoverable" => Some(TaxRecoverable),
                "Taxed Reserves" => Some(TaxedReserves),
                "Trade Creditors" => Some(TradeCreditors),
                "Inventory (Stock)" => Some(Code9T),
                "Inventory (Stock) Depreciation" => Some(Code9U),
                "Inventory (Stock) Purchases" => Some(Code9V),
                "Investment in Own Shares" => Some(InvestmentInOwnShares),
                "Investments" => Some(Investments),
                "Issued Capital" => Some(IssuedCapital),
                "Labor Costs" => Some(LaborCosts),
                "Shipment Value in U.S. Dollars" => Some(ShipmentValueInUSDollars),
                "Liabilities at Bankruptcy" => Some(LiabilitiesAtBankruptcy),
                "Account Average Balance Account" => Some(AccountAverageBalanceAccount),
                "Outstanding Balance at Foreclosure" => {
                    Some(OutstandingBalanceAtForeclosure)
                }
                "Legal Obligation Debt Amount" => Some(LegalObligationDebtAmount),
                "Estimated Closing Cost Amount" => Some(EstimatedClosingCostAmount),
                "Discount Fees Paid by Borrower Amount" => {
                    Some(DiscountFeesPaidByBorrowerAmount)
                }
                "Closing Costs or Concessions Paid by Seller" => {
                    Some(ClosingCostsOrConcessionsPaidBySeller)
                }
                "Prepaid Items Amount" => Some(PrepaidItemsAmount),
                "Federal Housing Administration, Mortgage Insurance Premium Funding Fee Financed Amount" => {
                    Some(Code19)
                }
                "Federal Housing Administration, Mortgage Insurance Premium or Veteran's Administration Funding Fee Amount" => {
                    Some(Code20)
                }
                "Original Cost of Property Amount" => Some(OriginalCostOfPropertyAmount),
                "Owner's Estimate of Value Amount" => Some(OwnersEstimateOfValueAmount),
                "Appraised Value Amount" => Some(AppraisedValueAmount),
                "Gross Monthly Income Amount" => Some(GrossMonthlyIncomeAmount),
                "Assets at Bankruptcy" => Some(AssetsAtBankruptcy),
                "Negotiated Cost" => Some(NegotiatedCost),
                "Authorized Unpriced Work" => Some(AuthorizedUnpricedWork),
                "Target Price" => Some(TargetPrice),
                "Estimated Price" => Some(EstimatedPrice),
                "Contract Ceiling" => Some(ContractCeiling),
                "Estimated Contract Ceiling" => Some(EstimatedContractCeiling),
                "Target Fee or Profit Amount" => Some(TargetFeeOrProfitAmount),
                "Original Contract Target Cost" => Some(OriginalContractTargetCost),
                "Negotiated Contract Changes" => Some(NegotiatedContractChanges),
                "Current Target Cost" => Some(CurrentTargetCost),
                "Contract Budget Base (CBB)" => Some(Code36),
                "Current Budgeted Cost for Work Scheduled (BCWS)" => Some(Code37),
                "Current Budgeted Cost for Work Performed (BCWP)" => Some(Code38),
                "Current Actual Cost of Work Performed (ACWP)" => Some(Code39),
                "Current Schedule Variance (SV)" => Some(Code40),
                "Current Cost Variance (CV)" => Some(Code41),
                "Cumulative Budgeted Cost for Work Scheduled (BCWS)" => Some(Code42),
                "Cumulative Budgeted Cost for Work Performed (BCWP)" => Some(Code43),
                "Cumulative Actual Cost of Work Performed (ACWP)" => Some(Code44),
                "Cumulative Schedule Variance (SV)" => Some(Code45),
                "Cumulative Cost Variance (CV)" => Some(Code46),
                "Reprogram Cost Variance" => Some(ReprogramCostVariance),
                "Reprogram Budget" => Some(ReprogramBudget),
                "At Complete Budget (BAC)" => Some(Code49),
                "At Complete Latest Revised Estimate (LRE)" => Some(Code50),
                "At Complete Variance" => Some(AtCompleteVariance),
                "Total Allocated Budget" => Some(TotalAllocatedBudget),
                "Difference (Contract Budget Base - Total Allocated Budget)" => {
                    Some(Code53)
                }
                "Forecast" => Some(Forecast),
                "At Complete Forecast" => Some(AtCompleteForecast),
                "Current Cost Performance Index (CPIe) - Efficiency (BCWP/ACWP)" => {
                    Some(Code56)
                }
                "Current Cost Performance Index (CPIp) - Planned (ACWP/BCWP)" => {
                    Some(Code57)
                }
                "Current Schedule Performance Index (SPI)" => Some(Code58),
                "Cumulative Cost Performance Index (CPIe) - Efficiency (BCWP/ACWP)" => {
                    Some(Code59)
                }
                "Cumulative Cost Performance Index (CPIp) - Planned (ACWP/BCWP)" => {
                    Some(Code60)
                }
                "Cumulative Schedule Performance Index (SPI)" => Some(Code61),
                "To Complete Performance Index (TCPI) for Budget at Complete (BAC)" => {
                    Some(Code62)
                }
                "To Complete Performance Index (TCPI) for Estimate At Complete (EAC)" => {
                    Some(Code63)
                }
                "Initial Contract Price Target" => Some(InitialContractPriceTarget),
                "Initial Contract Price Ceiling" => Some(InitialContractPriceCeiling),
                "Adjusted Contract Price Target" => Some(AdjustedContractPriceTarget),
                "Adjusted Contract Price Ceiling" => Some(AdjustedContractPriceCeiling),
                "Funds Authorized to Date" => Some(FundsAuthorizedToDate),
                "Accrued Expenditures" => Some(AccruedExpenditures),
                "Open Commitments" => Some(OpenCommitments),
                "Forecast of Billings" => Some(ForecastOfBillings),
                "Estimated Termination Costs" => Some(EstimatedTerminationCosts),
                "Accrued Expenditures plus Open Commitments" => {
                    Some(AccruedExpendituresPlusOpenCommitments)
                }
                "Contract Work Authorized - Definitized" => {
                    Some(ContractWorkAuthorizedDefinitized)
                }
                "Contract Work Authorized - Not Definitized" => {
                    Some(ContractWorkAuthorizedNotDefinitized)
                }
                "Contract Work Authorized - Total" => Some(ContractWorkAuthorizedTotal),
                "Forecast of Work - Not Yet Authorized" => {
                    Some(ForecastOfWorkNotYetAuthorized)
                }
                "Forecast of Work - All Other" => Some(ForecastOfWorkAllOther),
                "Forecast of Work - Total" => Some(ForecastOfWorkTotal),
                "Funding - Total Requirements" => Some(FundingTotalRequirements),
                "Funds Carryover" => Some(FundsCarryover),
                "Net Funds Required" => Some(NetFundsRequired),
                "Contract Work Authorized (with fee/profit) Actual or Projected" => {
                    Some(Code83)
                }
                "Contract Work Authorized (with fee/profit) Actual or Projected - At Complete" => {
                    Some(Code84)
                }
                "Best Case Estimate" => Some(BestCaseEstimate),
                "Worst Case Estimate" => Some(WorstCaseEstimate),
                "Most Likely Estimate" => Some(MostLikelyEstimate),
                "\"As Is\" Appraisal Amount" => Some(Code88),
                "\"Subject To\" Appraisal Amount" => Some(Code89),
                "\"Completion Per Plans\" Appraisal Amount" => Some(Code90),
                "Site Value Amount" => Some(SiteValueAmount),
                "Compensation" => Some(Compensation),
                "Contribution" => Some(Contribution),
                "Death Benefit" => Some(DeathBenefit),
                "Death Benefit Decrement" => Some(DeathBenefitDecrement),
                "Employee Account Balance" => Some(EmployeeAccountBalance),
                "Loan Repayment" => Some(LoanRepayment),
                "Prior W2" => Some(PriorW2),
                "Single Premium" => Some(SinglePremium),
                "Adjusted Chargeback Claim Amount" => Some(AdjustedChargebackClaimAmount),
                "Assistantship from Admitting Educational Institution" => {
                    Some(AssistantshipFromAdmittingEducationalInstitution)
                }
                "Average Negative Ledger Balance" => Some(AverageNegativeLedgerBalance),
                "Average Positive Collected Balance" => {
                    Some(AveragePositiveCollectedBalance)
                }
                "Average Negative Collected Balance" => {
                    Some(AverageNegativeCollectedBalance)
                }
                "Average Positive Ledger Balance" => Some(AveragePositiveLedgerBalance),
                "Disallowed - Estimated" => Some(DisallowedEstimated),
                "Disallowed - Actual" => Some(DisallowedActual),
                "Noncovered Charges - Estimated" => Some(NoncoveredChargesEstimated),
                "Noncovered Charges - Actual" => Some(NoncoveredChargesActual),
                "Allowed - Estimated" => Some(AllowedEstimated),
                "Allocated" => Some(Allocated),
                "Excess Funds" => Some(ExcessFunds),
                "Cumulative Total" => Some(CumulativeTotal),
                "Reimbursable Amount" => Some(ReimbursableAmount),
                "Total Reimbursable Amount" => Some(TotalReimbursableAmount),
                "Direct Citation Amount" => Some(DirectCitationAmount),
                "Total Direct Citation Funds" => Some(TotalDirectCitationFunds),
                "Chargeable Amount" => Some(ChargeableAmount),
                "Temporary Term Coverage" => Some(TemporaryTermCoverage),
                "Conditional Receipt Coverage" => Some(ConditionalReceiptCoverage),
                "Binding Interim Coverage" => Some(BindingInterimCoverage),
                "Application Amount" => Some(ApplicationAmount),
                "Approved Amount" => Some(ApprovedAmount),
                "Ultimate Face Amount" => Some(UltimateFaceAmount),
                "Requested Amount from All Reinsurers" => {
                    Some(RequestedAmountFromAllReinsurers)
                }
                "Replacement Amount" => Some(ReplacementAmount),
                "Scheduled Contribution" => Some(ScheduledContribution),
                "Scheduled Disbursement" => Some(ScheduledDisbursement),
                "Short Term Investment" => Some(ShortTermInvestment),
                "Subsequent Contribution" => Some(SubsequentContribution),
                "Subsequent Distribution" => Some(SubsequentDistribution),
                "Tax-Federal" => Some(TaxFederal),
                "Tax-Local" => Some(TaxLocal),
                "Tax-State" => Some(TaxState),
                "Trust Fund" => Some(TrustFund),
                "Capital Leases" => Some(CapitalLeases),
                "Surplus" => Some(Surplus),
                "Restated Assets" => Some(RestatedAssets),
                "Owing to Clients" => Some(OwingToClients),
                "Shareholder Loans" => Some(ShareholderLoans),
                "Accumulated Deficit" => Some(AccumulatedDeficit),
                "Loan from Parent Company" => Some(LoanFromParentCompany),
                "Contribution Not Subject to Repayment" => {
                    Some(ContributionNotSubjectToRepayment)
                }
                "Income Before Depreciation" => Some(IncomeBeforeDepreciation),
                "Adjusted Collected Balance" => Some(AdjustedCollectedBalance),
                "Income After Depreciation" => Some(IncomeAfterDepreciation),
                "Profit (Loss) Before Financial Items" => Some(CodeABB),
                "Interest Expenses" => Some(InterestExpenses),
                "Profit (Loss) Before Extraordinary Items" => Some(CodeABD),
                "Profit (Loss) After Financial Items" => Some(CodeABE),
                "Income Before Allocations" => Some(IncomeBeforeAllocations),
                "Income from Sale of Fixed Assets" => Some(IncomeFromSaleOfFixedAssets),
                "Contribution to Group" => Some(ContributionToGroup),
                "Deferred Tax Assets" => Some(DeferredTaxAssets),
                "Blocked Accounts" => Some(BlockedAccounts),
                "Non-taxed Reserves" => Some(NonTaxedReserves),
                "Pledged Assets" => Some(PledgedAssets),
                "Restricted Equity" => Some(RestrictedEquity),
                "Non-restricted Equity" => Some(NonRestrictedEquity),
                "Depreciable Assets" => Some(DepreciableAssets),
                "Taxable Assets" => Some(TaxableAssets),
                "Income from Business" => Some(IncomeFromBusiness),
                "Income Subject to Taxes" => Some(IncomeSubjectToTaxes),
                "Taxable Amount of Real Estate" => Some(TaxableAmountOfRealEstate),
                "Ending Principal Balance" => Some(EndingPrincipalBalance),
                "Average Daily Principal Balance" => Some(AverageDailyPrincipalBalance),
                "Interest Amount" => Some(InterestAmount),
                "Adjustments for Difference in Average Daily Principal Balance" => {
                    Some(AdjustmentsForDifferenceInAverageDailyPrincipalBalance)
                }
                "Beginning Principal Balance" => Some(BeginningPrincipalBalance),
                "Loan Principal Disbursements" => Some(LoanPrincipalDisbursements),
                "Principal Increases" => Some(PrincipalIncreases),
                "Average Collected Balance" => Some(AverageCollectedBalance),
                "Principal of Loans Purchased" => Some(PrincipalOfLoansPurchased),
                "Principal Cured" => Some(PrincipalCured),
                "Principal Sold" => Some(PrincipalSold),
                "Principal Insurance Claims" => Some(PrincipalInsuranceClaims),
                "Principal Guarantee Voided" => Some(PrincipalGuaranteeVoided),
                "Principal Paid by Borrowers" => Some(PrincipalPaidByBorrowers),
                "Loans in School and Grace" => Some(LoansInSchoolAndGrace),
                "Loans in Authorized Deferment" => Some(LoansInAuthorizedDeferment),
                "Loans Repay or Forebearance - Current or Less than 31 Days" => {
                    Some(LoansRepayOrForebearanceCurrentOrLessThan31Days)
                }
                "Loans Repay or Forebearance - 31 to 60 Days Past Due" => {
                    Some(LoansRepayOrForebearance31To60DaysPastDue)
                }
                "Loans Repay or Forebearance - 61 to 90 Days Past Due" => {
                    Some(LoansRepayOrForebearance61To90DaysPastDue)
                }
                "Loans Repay or Forebearance - 91 to 120 Days Past Due" => {
                    Some(LoansRepayOrForebearance91To120DaysPastDue)
                }
                "Loans Repay or Forebearance - 121 to 180 Days Past Due" => {
                    Some(LoansRepayOrForebearance121To180DaysPastDue)
                }
                "Loans Repay or Forebearance - 181 to 270 Days Past Due" => {
                    Some(LoansRepayOrForebearance181To270DaysPastDue)
                }
                "Loans Repay or Forebearance - 271 or More Days Past Due" => {
                    Some(LoansRepayOrForebearance271OrMoreDaysPastDue)
                }
                "Loans Repay or Forebearance - Claims Filed, Not Yet Paid" => {
                    Some(CodeACP)
                }
                "Agent Sales" => Some(AgentSales),
                "Amount Involved" => Some(AmountInvolved),
                "Assigned Capital" => Some(AssignedCapital),
                "Credit Line Utilized" => Some(CreditLineUtilized),
                "Direct Sales" => Some(DirectSales),
                "Earnings per Share" => Some(EarningsPerShare),
                "Inheritance" => Some(Inheritance),
                "Invested Capital" => Some(InvestedCapital),
                "Loan from Family Members" => Some(LoanFromFamilyMembers),
                "Non Depreciable Assets" => Some(NonDepreciableAssets),
                "Adjusted Total" => Some(AdjustedTotal),
                "Partially Paid Amount per Share" => Some(PartiallyPaidAmountPerShare),
                "Pending Orders" => Some(PendingOrders),
                "Personal Loan" => Some(PersonalLoan),
                "Plant and Machinery" => Some(PlantAndMachinery),
                "Pre-Tax Loss" => Some(PreTaxLoss),
                "Pre-Tax Profit" => Some(PreTaxProfit),
                "Registered Capital" => Some(RegisteredCapital),
                "Revaluation Reserves" => Some(RevaluationReserves),
                "Social Capital" => Some(SocialCapital),
                "Statutory Profit" => Some(StatutoryProfit),
                "Training Pay" => Some(TrainingPay),
                "Retroactive Pay" => Some(RetroactivePay),
                "Expected Reimbursement Amount" => Some(ExpectedReimbursementAmount),
                "Permit Cost" => Some(PermitCost),
                "Minimum" => Some(Minimum),
                "Additional Amount to Meet Minimum" => {
                    Some(AdditionalAmountToMeetMinimum)
                }
                "Labor Per Hour" => Some(LaborPerHour),
                "Non-recoverable Depreciation" => Some(NonRecoverableDepreciation),
                "Recoverable Depreciation" => Some(RecoverableDepreciation),
                "Overhead" => Some(Overhead),
                "Indemnity Benefit" => Some(IndemnityBenefit),
                "Replacement Cost of Repairs" => Some(ReplacementCostOfRepairs),
                "Actual Cash Value of Repairs" => Some(ActualCashValueOfRepairs),
                "Recoverable Depreciation of Repairs" => {
                    Some(RecoverableDepreciationOfRepairs)
                }
                "Non-recoverable Depreciation of Repairs" => {
                    Some(NonRecoverableDepreciationOfRepairs)
                }
                "Arrearage" => Some(Arrearage),
                "Non-indemnity Benefit" => Some(NonIndemnityBenefit),
                "Actual Cash Value of Building" => Some(ActualCashValueOfBuilding),
                "Government Share" => Some(GovernmentShare),
                "Contractor Share" => Some(ContractorShare),
                "Award Fee" => Some(AwardFee),
                "Base Fee" => Some(BaseFee),
                "Target Profit Floor" => Some(TargetProfitFloor),
                "Target Profit Ceiling" => Some(TargetProfitCeiling),
                "Labor Per Day" => Some(LaborPerDay),
                "Difference in Interest Due" => Some(DifferenceInInterestDue),
                "Difference in Prepayment Penalty" => Some(DifferenceInPrepaymentPenalty),
                "Difference in Principal Due" => Some(DifferenceInPrincipalDue),
                "Appropriation of Retained Earnings Less Reversals" => {
                    Some(AppropriationOfRetainedEarningsLessReversals)
                }
                "Appropriations" => Some(Appropriations),
                "Billings and Costs-Profit Differential" => {
                    Some(BillingsAndCostsProfitDifferential)
                }
                "Common Stock Par Value" => Some(CommonStockParValue),
                "Cost of Services Rendered" => Some(CostOfServicesRendered),
                "Creditors" => Some(Creditors),
                "Declared Profit" => Some(DeclaredProfit),
                "Discounted Notes" => Some(DiscountedNotes),
                "Endorsed Notes" => Some(EndorsedNotes),
                "General Reserves" => Some(GeneralReserves),
                "Import Volume" => Some(ImportVolume),
                "Income Tax Credit" => Some(IncomeTaxCredit),
                "Long-Term Deposits" => Some(LongTermDeposits),
                "Long-Term Loans" => Some(LongTermLoans),
                "Average Float" => Some(AverageFloat),
                "Minority Interest" => Some(MinorityInterest),
                "Non-Operating Expense" => Some(NonOperatingExpense),
                "Non-Operating Income" => Some(NonOperatingIncome),
                "Operating Profit or Loss" => Some(OperatingProfitOrLoss),
                "Preferred Stock Par Value" => Some(PreferredStockParValue),
                "Profit After Tax and Minority Interest" => {
                    Some(ProfitAfterTaxAndMinorityInterest)
                }
                "Retained Earnings to be Appropriated" => {
                    Some(RetainedEarningsToBeAppropriated)
                }
                "Revaluation Surplus or Deficit" => Some(RevaluationSurplusOrDeficit),
                "Reversal of Voluntary Earned Surplus" => {
                    Some(ReversalOfVoluntaryEarnedSurplus)
                }
                "Share Price" => Some(SharePrice),
                "Short-Term Deposits" => Some(ShortTermDeposits),
                "Short-Term Loans" => Some(ShortTermLoans),
                "Tax Provisions" => Some(TaxProvisions),
                "Unallocated Profit" => Some(UnallocatedProfit),
                "Voluntary Earned Surplus" => Some(VoluntaryEarnedSurplus),
                "Calculated Weekly Compensation Amount" => {
                    Some(CalculatedWeeklyCompensationAmount)
                }
                "Benefit Type Gross Weekly Amount" => Some(BenefitTypeGrossWeeklyAmount),
                "Benefit Type Net Weekly Amount" => Some(BenefitTypeNetWeeklyAmount),
                "Employee Gross Wage" => Some(EmployeeGrossWage),
                "Garage Gross Wages" => Some(GarageGrossWages),
                "Officer Compensation - Actual Flat" => {
                    Some(OfficerCompensationActualFlat)
                }
                "Officer Compensation - Statutory Maximum" => {
                    Some(OfficerCompensationStatutoryMaximum)
                }
                "Officer Compensation - Statutory Minimum" => {
                    Some(OfficerCompensationStatutoryMinimum)
                }
                "Previous Balance" => Some(PreviousBalance),
                "Disputed Amount" => Some(DisputedAmount),
                "Adjusted Gross Income" => Some(AdjustedGrossIncome),
                "Non-Operating Income or Expense" => Some(NonOperatingIncomeOrExpense),
                "Operating Income or Expense" => Some(OperatingIncomeOrExpense),
                "Income or Expense" => Some(IncomeOrExpense),
                "Purchase Authority" => Some(PurchaseAuthority),
                "Capital Decrease" => Some(CapitalDecrease),
                "Capital Increase" => Some(CapitalIncrease),
                "Deed Capital" => Some(DeedCapital),
                "Transferred Amount" => Some(TransferredAmount),
                "Unadjusted Sales Price" => Some(UnadjustedSalesPrice),
                "Sales Concessions" => Some(SalesConcessions),
                "Property Value" => Some(PropertyValue),
                "Partial Release Amount" => Some(PartialReleaseAmount),
                "Lien Filing Fee" => Some(LienFilingFee),
                "Additional Repair Price" => Some(AdditionalRepairPrice),
                "Qualified Tuition and Related Expenses" => {
                    Some(QualifiedTuitionAndRelatedExpenses)
                }
                "Qualified Financial Assistance" => Some(QualifiedFinancialAssistance),
                "Aggregate Reimbursements or Refunds" => {
                    Some(AggregateReimbursementsOrRefunds)
                }
                "New Loan Balance" => Some(NewLoanBalance),
                "Raw Material Purchases" => Some(RawMaterialPurchases),
                "Work in Progress Purchases" => Some(WorkInProgressPurchases),
                "Operating Cash Flow" => Some(OperatingCashFlow),
                "Payments for Outside Work" => Some(PaymentsForOutsideWork),
                "Set Aside for Provisions" => Some(SetAsideForProvisions),
                "Financial Income or Expense" => Some(FinancialIncomeOrExpense),
                "Extraordinary Income or Expense" => Some(ExtraordinaryIncomeOrExpense),
                "Loan Balance Difference" => Some(LoanBalanceDifference),
                "Unfinished Production Carried Forward" => {
                    Some(UnfinishedProductionCarriedForward)
                }
                "Installation Materials Cost" => Some(InstallationMaterialsCost),
                "New Mortgage Amount" => Some(NewMortgageAmount),
                "Capitalized Assets" => Some(CapitalizedAssets),
                "Profit Reserves" => Some(ProfitReserves),
                "Share in Profit or Loss in Other Companies" => {
                    Some(ShareInProfitOrLossInOtherCompanies)
                }
                "Monetary Correction" => Some(MonetaryCorrection),
                "Scheduled Repayment Amount" => Some(ScheduledRepaymentAmount),
                "Amounts Placed with Other Banks" => Some(AmountsPlacedWithOtherBanks),
                "Due from Parent Company" => Some(DueFromParentCompany),
                "Owing from Subsidiary Companies" => Some(OwingFromSubsidiaryCompanies),
                "Certificates of Deposit Amount" => Some(CertificatesOfDepositAmount),
                "Publicly Traded Shares Amount" => Some(PubliclyTradedSharesAmount),
                "Non Publicly Traded Shares Amount" => {
                    Some(NonPubliclyTradedSharesAmount)
                }
                "Trading Securities Amount" => Some(TradingSecuritiesAmount),
                "Investment Securities Amount" => Some(InvestmentSecuritiesAmount),
                "Earnings Per Share Minus Dividends" => {
                    Some(EarningsPerShareMinusDividends)
                }
                "Active Partner Capital" => Some(ActivePartnerCapital),
                "Sale Amount" => Some(SaleAmount),
                "Funds Held by Mortgagee" => Some(FundsHeldByMortgagee),
                "Adjusted Claim" => Some(AdjustedClaim),
                "Attorney Fees" => Some(AttorneyFees),
                "Average Ledger Balance" => Some(AverageLedgerBalance),
                "Amount Financed" => Some(AmountFinanced),
                "Bankruptcy Fee" => Some(BankruptcyFee),
                "Accrued Income" => Some(AccruedIncome),
                "Amount Override" => Some(AmountOverride),
                "Amount Prior to Fractionalization" => {
                    Some(AmountPriorToFractionalization)
                }
                "Amount of Purchase Exempt From Tax or Fee" => {
                    Some(AmountOfPurchaseExemptFromTaxOrFee)
                }
                "Average Price Per Call" => Some(AveragePricePerCall),
                "Acquisition Cost of Lenses" => Some(AcquisitionCostOfLenses),
                "Fees to Public Officials for Foreclosure" => {
                    Some(FeesToPublicOfficialsForForeclosure)
                }
                "Average Price Per Minute" => Some(AveragePricePerMinute),
                "Annual Sales or Premiums" => Some(AnnualSalesOrPremiums),
                "Total Received" => Some(TotalReceived),
                "Amount of Tax or Fee Exemption" => Some(AmountOfTaxOrFeeExemption),
                "Coverage Amount" => Some(CoverageAmount),
                "Actual Cash Value" => Some(ActualCashValue),
                "Average" => Some(Average),
                "Replacement Cost" => Some(ReplacementCost),
                "Previous Price" => Some(PreviousPrice),
                "Title Cost" => Some(TitleCost),
                "Other Foreclosure and Acquisition Expenses" => {
                    Some(OtherForeclosureAndAcquisitionExpenses)
                }
                "Estimated" => Some(Estimated),
                "Bond" => Some(Bond),
                "Benefit Amount" => Some(BenefitAmount),
                "Bonuses and Commissions Divided Over 12 Months" => {
                    Some(BonusesAndCommissionsDividedOver12Months)
                }
                "Bonuses Divided Over 12 Months" => Some(BonusesDividedOver12Months),
                "Bonuses and Commissions" => Some(BonusesAndCommissions),
                "Budgeted" => Some(Budgeted),
                "Allowed - Actual" => Some(AllowedActual),
                "Deductible - Estimated" => Some(DeductibleEstimated),
                "Co-insurance - Estimated" => Some(CoInsuranceEstimated),
                "Co-insurance - Actual" => Some(CoInsuranceActual),
                "Bargain" => Some(Bargain),
                "Net Taxable Income" => Some(NetTaxableIncome),
                "Original Amount of Instrument" => Some(OriginalAmountOfInstrument),
                "Addition to Tax" => Some(AdditionToTax),
                "Reinstatement Fee" => Some(ReinstatementFee),
                "Permit Fee Due" => Some(PermitFeeDue),
                "Permit Fee with Extension" => Some(PermitFeeWithExtension),
                "Net Annual Permit Fee Due" => Some(NetAnnualPermitFeeDue),
                "Permit Fee Penalty Due" => Some(PermitFeePenaltyDue),
                "Permit Fee Interest Due" => Some(PermitFeeInterestDue),
                "Total Permit Fee Due" => Some(TotalPermitFeeDue),
                "Franchise Tax" => Some(FranchiseTax),
                "Unclaimed Franchise Tax Credit" => Some(UnclaimedFranchiseTaxCredit),
                "Net Franchise Tax Due" => Some(NetFranchiseTaxDue),
                "Franchise Tax Penalty Due" => Some(FranchiseTaxPenaltyDue),
                "Total Franchise Tax Due" => Some(TotalFranchiseTaxDue),
                "Total Amount Due" => Some(TotalAmountDue),
                "Overpayment" => Some(Overpayment),
                "Amount to be Refunded" => Some(AmountToBeRefunded),
                "Gross In-State Receipts" => Some(GrossInStateReceipts),
                "Gross Receipts" => Some(GrossReceipts),
                "Occupation Fee" => Some(OccupationFee),
                "Total Assessed Value" => Some(TotalAssessedValue),
                "Total Value of All Property" => Some(TotalValueOfAllProperty),
                "Value of In-State Property" => Some(ValueOfInStateProperty),
                "Total Value of Out-of-State Property" => {
                    Some(TotalValueOfOutOfStateProperty)
                }
                "Total Par Value" => Some(TotalParValue),
                "Mortgage Insurance Premiums" => Some(MortgageInsurancePremiums),
                "Total Assessable Capital Stock" => Some(TotalAssessableCapitalStock),
                "Apportioned Value" => Some(ApportionedValue),
                "Estimated In-State Real Property Value" => {
                    Some(EstimatedInStateRealPropertyValue)
                }
                "Par Value of In-State Assets" => Some(ParValueOfInStateAssets),
                "In-State Business Revenue" => Some(InStateBusinessRevenue),
                "Subscription Price" => Some(SubscriptionPrice),
                "Value of Authorized Shares" => Some(ValueOfAuthorizedShares),
                "Ordinance Amount" => Some(OrdinanceAmount),
                "Capital for a Cooperative" => Some(CapitalForACooperative),
                "Directors Legal Obligation Debt" => Some(DirectorsLegalObligationDebt),
                "Silent Partner Capital" => Some(SilentPartnerCapital),
                "Billing Cycle Net Fee Position (Excess/Deficit)" => Some(CodeBC),
                "Balance Due" => Some(BalanceDue),
                "Disbursements for Authorized Repair" => {
                    Some(DisbursementsForAuthorizedRepair)
                }
                "Hazard Insurance Premium" => Some(HazardInsurancePremium),
                "Eviction Attorney Fees" => Some(EvictionAttorneyFees),
                "Eviction Expenses" => Some(EvictionExpenses),
                "Property Taxes" => Some(PropertyTaxes),
                "Disbursements Not Shown Elsewhere" => {
                    Some(DisbursementsNotShownElsewhere)
                }
                "Disbursements for Protection and Preservation" => {
                    Some(DisbursementsForProtectionAndPreservation)
                }
                "Disbursements for Inspections and Boarding" => {
                    Some(DisbursementsForInspectionsAndBoarding)
                }
                "Bridge Loan Not Deposited" => Some(BridgeLoanNotDeposited),
                "Adjustments" => Some(Adjustments),
                "Rental Income" => Some(RentalIncome),
                "Rental Expense" => Some(RentalExpense),
                "Borrower Advance" => Some(BorrowerAdvance),
                "Borrowed Amount" => Some(BorrowedAmount),
                "Average Net Collected Balance" => Some(AverageNetCollectedBalance),
                "Bail" => Some(Bail),
                "Adjusted Insured Loss Amount" => Some(AdjustedInsuredLossAmount),
                "Mortgage Note Interest" => Some(MortgageNoteInterest),
                "Bank Reject Total" => Some(BankRejectTotal),
                "Betterment" => Some(Betterment),
                "Overhead Costs" => Some(OverheadCosts),
                "Uncollected Interest" => Some(UncollectedInterest),
                "Amount Due from Buyer at Closing" => Some(AmountDueFromBuyerAtClosing),
                "Amount Owed to Buyer at Closing" => Some(AmountOwedToBuyerAtClosing),
                "Bills of Exchange Payable" => Some(BillsOfExchangePayable),
                "Additional Closing Expenses" => Some(AdditionalClosingExpenses),
                "Deficiency Judgment Expenses" => Some(DeficiencyJudgmentExpenses),
                "City" => Some(City),
                "Current Expenditures" => Some(CurrentExpenditures),
                "Co-Payment Amount" => Some(CoPaymentAmount),
                "Child Rider Coverage" => Some(ChildRiderCoverage),
                "Prior Payment - Estimated" => Some(PriorPaymentEstimated),
                "Prior Payment - Actual" => Some(PriorPaymentActual),
                "Claim Amount Due - Estimated" => Some(ClaimAmountDueEstimated),
                "Claim Amount Due - Actual" => Some(ClaimAmountDueActual),
                "Payer Responsibility - Estimated" => Some(PayerResponsibilityEstimated),
                "Payer Responsibility - Actual" => Some(PayerResponsibilityActual),
                "Disallowed Cost Containment - Actual" => {
                    Some(DisallowedCostContainmentActual)
                }
                "Contractor Cumulative to Date" => Some(ContractorCumulativeToDate),
                "Collected Balance Required" => Some(CollectedBalanceRequired),
                "Chargeback Claim Amount" => Some(ChargebackClaimAmount),
                "Overpaid Section 235 Subsidy" => Some(OverpaidSection235Subsidy),
                "Clean-up Costs Associated with Deficiency" => {
                    Some(CleanUpCostsAssociatedWithDeficiency)
                }
                "Summary Amount" => Some(SummaryAmount),
                "Appraisal Fees" => Some(AppraisalFees),
                "Commission Fees Deducted" => Some(CommissionFeesDeducted),
                "Change Amount" => Some(ChangeAmount),
                "Funds Held for Insured" => Some(FundsHeldForInsured),
                "Other Deductions" => Some(OtherDeductions),
                "Collections, Judgments, and Liens" => Some(CodeCJL),
                "Back End Load" => Some(BackEndLoad),
                "Outstanding Balance Current Lender" => {
                    Some(OutstandingBalanceCurrentLender)
                }
                "Claimant Requested Total" => Some(ClaimantRequestedTotal),
                "Compression Charge" => Some(CompressionCharge),
                "Commodity Refund" => Some(CommodityRefund),
                "Special Assessments" => Some(SpecialAssessments),
                "Taxes on Deed" => Some(TaxesOnDeed),
                "Corporate Assets" => Some(CorporateAssets),
                "Statutory Disbursements" => Some(StatutoryDisbursements),
                "Closing Costs Paid by Any Other Party Other Than Seller or Buyer" => {
                    Some(ClosingCostsPaidByAnyOtherPartyOtherThanSellerOrBuyer)
                }
                "Closing Costs Paid by Seller" => Some(ClosingCostsPaidBySeller),
                "Net Claim Amount" => Some(NetClaimAmount),
                "Contractor at Complete" => Some(ContractorAtComplete),
                "Container Replacement Cost" => Some(ContainerReplacementCost),
                "Container Replacement Labor Cost" => Some(ContainerReplacementLaborCost),
                "Container Replacement Material Cost" => {
                    Some(ContainerReplacementMaterialCost)
                }
                "Commission Sales" => Some(CommissionSales),
                "Contract" => Some(Contract),
                "Subcontractor Cumulative to Date" => Some(SubcontractorCumulativeToDate),
                "Subcontractor at Complete" => Some(SubcontractorAtComplete),
                "Earned Value" => Some(EarnedValue),
                "Actual" => Some(Actual),
                "Cumulative Budget" => Some(CumulativeBudget),
                "Cumulative Earned Value" => Some(CumulativeEarnedValue),
                "Payer Amount Paid" => Some(PayerAmountPaid),
                "Administration and Management Costs" => {
                    Some(AdministrationAndManagementCosts)
                }
                "Deferred Compensation Commissions" => {
                    Some(DeferredCompensationCommissions)
                }
                "Deductible Amount" => Some(DeductibleAmount),
                "Deferred Compensation Commissions and Bonuses" => {
                    Some(DeferredCompensationCommissionsAndBonuses)
                }
                "Deferred Compensation" => Some(DeferredCompensation),
                "Dollar For Dollar Deductions" => Some(DollarForDollarDeductions),
                "Dependent Care Contribution" => Some(DependentCareContribution),
                "Disallowed Cost Containment - Estimated" => {
                    Some(DisallowedCostContainmentEstimated)
                }
                "Dispensing Fee" => Some(DispensingFee),
                "Discount Amount" => Some(DiscountAmount),
                "Cumulative Actual" => Some(CumulativeActual),
                "Original Mortgage" => Some(OriginalMortgage),
                "Unapplied Section 235 Funds" => Some(UnappliedSection235Funds),
                "Unapplied Buydown Fund" => Some(UnappliedBuydownFund),
                "Direct Deposit" => Some(DirectDeposit),
                "Estimate of Damage" => Some(EstimateOfDamage),
                "Deferral" => Some(Deferral),
                "Delayed Interest" => Some(DelayedInterest),
                "Authorized Bid" => Some(AuthorizedBid),
                "Deferred Assets" => Some(DeferredAssets),
                "Escrow Balance" => Some(EscrowBalance),
                "Total Disbursements" => Some(TotalDisbursements),
                "Charge Off" => Some(ChargeOff),
                "Liens Amount Original" => Some(LiensAmountOriginal),
                "Release of Lien" => Some(ReleaseOfLien),
                "Debit" => Some(Debit),
                "Delinquent Taxes" => Some(DelinquentTaxes),
                "Asset" => Some(Asset),
                "Liability" => Some(Liability),
                "Satisfaction" => Some(Satisfaction),
                "Exemption" => Some(Exemption),
                "Discount Points Financed" => Some(DiscountPointsFinanced),
                "Discount Points Not Financed" => Some(DiscountPointsNotFinanced),
                "Defaulted Tax Plan Payment" => Some(DefaultedTaxPlanPayment),
                "Settlement" => Some(Settlement),
                "Alimony Expense" => Some(AlimonyExpense),
                "Alimony Income" => Some(AlimonyIncome),
                "Child Support Expense" => Some(ChildSupportExpense),
                "Child Support Income" => Some(ChildSupportIncome),
                "Separate Maintenance Expense" => Some(SeparateMaintenanceExpense),
                "Development Properties" => Some(DevelopmentProperties),
                "Separate Maintenance Income" => Some(SeparateMaintenanceIncome),
                "Deductible Waived" => Some(DeductibleWaived),
                "Per Day Limit" => Some(PerDayLimit),
                "Job-related Expense" => Some(JobRelatedExpense),
                "Estimated Credit" => Some(EstimatedCredit),
                "Administration and Management Indemnity Charge" => {
                    Some(AdministrationAndManagementIndemnityCharge)
                }
                "Employer Year to Date Contribution" => {
                    Some(EmployerYearToDateContribution)
                }
                "Employee Annual Pledge Amount" => Some(EmployeeAnnualPledgeAmount),
                "Employee Current Contribution" => Some(EmployeeCurrentContribution),
                "Employer Pledge Amount" => Some(EmployerPledgeAmount),
                "Employer Current Contribution" => Some(EmployerCurrentContribution),
                "Eligible Wage Amount" => Some(EligibleWageAmount),
                "Employee Year to Date Contribution" => {
                    Some(EmployeeYearToDateContribution)
                }
                "Education Contribution" => Some(EducationContribution),
                "Initial Fee" => Some(InitialFee),
                "Earnings Allowance" => Some(EarningsAllowance),
                "Administrative Expenses" => Some(AdministrativeExpenses),
                "Air Travel Expenses" => Some(AirTravelExpenses),
                "Amount Forgiven" => Some(AmountForgiven),
                "Amount Guaranteed" => Some(AmountGuaranteed),
                "Amount Over Fair Market Value" => Some(AmountOverFairMarketValue),
                "Amount Owed" => Some(AmountOwed),
                "Amount Payable" => Some(AmountPayable),
                "Amount Raised" => Some(AmountRaised),
                "Amount Received" => Some(AmountReceived),
                "Amount Refunded" => Some(AmountRefunded),
                "Amount Rescinded" => Some(AmountRescinded),
                "Anonymous Contribution" => Some(AnonymousContribution),
                "Balance Owed" => Some(BalanceOwed),
                "Bank Charges" => Some(BankCharges),
                "Bank Loan" => Some(BankLoan),
                "Brochure Expenses" => Some(BrochureExpenses),
                "Bus Travel Expenses" => Some(BusTravelExpenses),
                "Consultant Expenses" => Some(ConsultantExpenses),
                "Corrected Amount" => Some(CorrectedAmount),
                "Disability Expenses" => Some(DisabilityExpenses),
                "Disposed Amount" => Some(DisposedAmount),
                "Draw Amount" => Some(DrawAmount),
                "Election Expenses" => Some(ElectionExpenses),
                "Endorsement Amount" => Some(EndorsementAmount),
                "Entertainment Expenses" => Some(EntertainmentExpenses),
                "Excess Expenses" => Some(ExcessExpenses),
                "Collected Balance (Excess/Deficit)" => Some(CodeEB),
                "Expected Expenditure Amount" => Some(ExpectedExpenditureAmount),
                "Expenditure Amount" => Some(ExpenditureAmount),
                "Family Care Expenses" => Some(FamilyCareExpenses),
                "Federal Share Amount" => Some(FederalShareAmount),
                "Filing Fee" => Some(FilingFee),
                "In-Kind Contribution" => Some(InKindContribution),
                "Incurred Amount" => Some(IncurredAmount),
                "Loan Amount Plus Interest" => Some(LoanAmountPlusInterest),
                "Loan Balance" => Some(LoanBalance),
                "Matching Contribution" => Some(MatchingContribution),
                "Meeting Expenses" => Some(MeetingExpenses),
                "Miscellaneous Income" => Some(MiscellaneousIncome),
                "Miscellaneous Receipts" => Some(MiscellaneousReceipts),
                "New Loan Amount" => Some(NewLoanAmount),
                "New Unpaid Expenditure" => Some(NewUnpaidExpenditure),
                "Newsletter Expenses" => Some(NewsletterExpenses),
                "Newspaper Advertising Expenses" => Some(NewspaperAdvertisingExpenses),
                "Nomination Expenses" => Some(NominationExpenses),
                "Non-Federal Share" => Some(NonFederalShare),
                "Office Expenses" => Some(OfficeExpenses),
                "Office Rental" => Some(OfficeRental),
                "Original Asset Value" => Some(OriginalAssetValue),
                "Original Loan Amount" => Some(OriginalLoanAmount),
                "Party Expenses" => Some(PartyExpenses),
                "Payment" => Some(Payment),
                "Allowance (Excess/Deficit)" => Some(CodeEC),
                "Personal Expenses" => Some(PersonalExpenses),
                "Personal Funds" => Some(PersonalFunds),
                "Pledged Amount" => Some(PledgedAmount),
                "Postage Expenses" => Some(PostageExpenses),
                "Printing Expenses" => Some(PrintingExpenses),
                "Public Funds" => Some(PublicFunds),
                "Radio Advertising Expenses" => Some(RadioAdvertisingExpenses),
                "Reimbursed Amount" => Some(ReimbursedAmount),
                "Reported Amount" => Some(ReportedAmount),
                "Retainer" => Some(Retainer),
                "Sign Expenses" => Some(SignExpenses),
                "Sub-Contract Value" => Some(SubContractValue),
                "Tax Receipts" => Some(TaxReceipts),
                "Taxi Travel Expenses" => Some(TaxiTravelExpenses),
                "Telecommunication Expenses" => Some(TelecommunicationExpenses),
                "Television Advertising Expenses" => Some(TelevisionAdvertisingExpenses),
                "Unpaid Expenditure" => Some(UnpaidExpenditure),
                "Utilities Expenses" => Some(UtilitiesExpenses),
                "Total" => Some(Total),
                "Subtotal" => Some(Subtotal),
                "Grand Total" => Some(GrandTotal),
                "Incidental Expenses" => Some(IncidentalExpenses),
                "Transportation Expenses" => Some(TransportationExpenses),
                "Gift Value" => Some(GiftValue),
                "Food and Refreshments" => Some(FoodAndRefreshments),
                "Polling Expenses" => Some(PollingExpenses),
                "Estimated Cost of Attendance" => Some(EstimatedCostOfAttendance),
                "Tuition and Required Fees" => Some(TuitionAndRequiredFees),
                "Books and Supplies" => Some(BooksAndSupplies),
                "Other Expense" => Some(OtherExpense),
                "Estimated Financial Aid" => Some(EstimatedFinancialAid),
                "Other Income" => Some(OtherIncome),
                "Amount of Mortgages and Liens" => Some(AmountOfMortgagesAndLiens),
                "Mortgage Payment(s)" => Some(CodeEI),
                "Maintenance Expense on Income Producing Property" => {
                    Some(MaintenanceExpenseOnIncomeProducingProperty)
                }
                "Insurance, Maintenance, Taxes and Miscellaneous" => Some(CodeEJ),
                "Net Rental Income" => Some(NetRentalIncome),
                "Present Market Value" => Some(PresentMarketValue),
                "Electroconvulsive Therapy (ECT) Adjustment" => Some(CodeELT),
                "Gross Rental Income" => Some(GrossRentalIncome),
                "Cancellation Fee" => Some(CancellationFee),
                "Maintenance Expense on Non-Income Producing Property" => {
                    Some(MaintenanceExpenseOnNonIncomeProducingProperty)
                }
                "Entitlement Amount" => Some(EntitlementAmount),
                "Capital Reserves" => Some(CapitalReserves),
                "Employer Annual Pledge Amount" => Some(EmployerAnnualPledgeAmount),
                "Condominium Association Fees" => Some(CondominiumAssociationFees),
                "Homeowner Association Fees" => Some(HomeownerAssociationFees),
                "Earnest Money" => Some(EarnestMoney),
                "Mortgage Insurance Proceeds" => Some(MortgageInsuranceProceeds),
                "Net Proceeds from Sale of Real Estate Property" => {
                    Some(NetProceedsFromSaleOfRealEstateProperty)
                }
                "Ever To Date (ETD) Claim Loss" => Some(CodeETD),
                "Insurance Proceeds (Primary Settlement)" => Some(CodeEU),
                "Presale Proceeds" => Some(PresaleProceeds),
                "Pledged Savings" => Some(PledgedSavings),
                "As Is Broker's Opinion" => Some(AsIsBrokersOpinion),
                "Exploration Costs" => Some(ExplorationCosts),
                "Subject To Broker's Opinion" => Some(SubjectToBrokersOpinion),
                "Uniform Commercial Code Filing Office Fee" => {
                    Some(UniformCommercialCodeFilingOfficeFee)
                }
                "Annual Limit" => Some(AnnualLimit),
                "Commercial Staff Labor Costs" => Some(CommercialStaffLaborCosts),
                "Maximum Allowable Cost (MAC) Penalty Copay" => Some(CodeF1),
                "Patient Responsibility - Actual" => Some(PatientResponsibilityActual),
                "Patient Responsibility - Estimated" => {
                    Some(PatientResponsibilityEstimated)
                }
                "Postage Claimed" => Some(PostageClaimed),
                "Patient Amount Paid" => Some(PatientAmountPaid),
                "Provider Reserves" => Some(ProviderReserves),
                "Sales Tax" => Some(SalesTax),
                "Usual and Customary Charge - Estimated" => {
                    Some(UsualAndCustomaryChargeEstimated)
                }
                "Usual and Customary - Actual" => Some(UsualAndCustomaryActual),
                "Coordination Fee" => Some(CoordinationFee),
                "Federal Housing Administration (FHA) Appraiser Required Repairs and Improvements" => {
                    Some(CodeFAR)
                }
                "Calculation Fee" => Some(CalculationFee),
                "Final Balance" => Some(FinalBalance),
                "Expected Family Contribution" => Some(ExpectedFamilyContribution),
                "Direct Deposit Flipped to Check" => Some(DirectDepositFlippedToCheck),
                "Fee" => Some(Fee),
                "Application Fee" => Some(ApplicationFee),
                "Licensing Fee" => Some(LicensingFee),
                "Regulatory Fee" => Some(RegulatoryFee),
                "First Interest Payment Amount" => Some(FirstInterestPaymentAmount),
                "Waiver Fee" => Some(WaiverFee),
                "Other Unlisted Amount" => Some(OtherUnlistedAmount),
                "Float" => Some(Float),
                "First Lien Advance" => Some(FirstLienAdvance),
                "Fair Market Value" => Some(FairMarketValue),
                "Fine" => Some(Fine),
                "Fees Paid" => Some(FeesPaid),
                "Foreign Assets" => Some(ForeignAssets),
                "Fees Paid Year to Date" => Some(FeesPaidYearToDate),
                "Firm Contractor Share" => Some(FirmContractorShare),
                "Estimated Government Share" => Some(EstimatedGovernmentShare),
                "Facilities Refund" => Some(FacilitiesRefund),
                "Expense" => Some(Expense),
                "Endorsement Premium Amount" => Some(EndorsementPremiumAmount),
                "Fuel Tracker Refund" => Some(FuelTrackerRefund),
                "Facility tax amount" => Some(FacilityTaxAmount),
                "Commercial Staff Indemnity Charge" => {
                    Some(CommercialStaffIndemnityCharge)
                }
                "Flat Fee Paid to Date" => Some(FlatFeePaidToDate),
                "Flat Fee Paid Current Month" => Some(FlatFeePaidCurrentMonth),
                "Endorsement" => Some(Endorsement),
                "First Payment" => Some(FirstPayment),
                "Earned Income" => Some(EarnedIncome),
                "Collateral" => Some(Collateral),
                "Initial Adjustment Total" => Some(InitialAdjustmentTotal),
                "Indicated Value by Sales Comparison Approach" => {
                    Some(IndicatedValueBySalesComparisonApproach)
                }
                "Indicated Value by Income Approach" => {
                    Some(IndicatedValueByIncomeApproach)
                }
                "Price per Unit Area" => Some(PricePerUnitArea),
                "Reconciliation of Final Value Estimate" => {
                    Some(ReconciliationOfFinalValueEstimate)
                }
                "Estimated Monthly Market Rent" => Some(EstimatedMonthlyMarketRent),
                "Adjusted Sales Price" => Some(AdjustedSalesPrice),
                "Sales or Financing Concessions" => Some(SalesOrFinancingConcessions),
                "Indicated Value by Cost Approach" => Some(IndicatedValueByCostApproach),
                "As-is Value of Site Improvements" => Some(AsIsValueOfSiteImprovements),
                "Depreciated Value of Improvements" => {
                    Some(DepreciatedValueOfImprovements)
                }
                "Garnishments" => Some(Garnishments),
                "Annual Tax" => Some(AnnualTax),
                "Price, High Value" => Some(CodeGB),
                "Price, Low Value" => Some(CodeGC),
                "Corrected Tax Bill" => Some(CorrectedTaxBill),
                "Gross Claim" => Some(GrossClaim),
                "Physical Depreciation" => Some(PhysicalDepreciation),
                "Functional Depreciation" => Some(FunctionalDepreciation),
                "External Depreciation" => Some(ExternalDepreciation),
                "Gifts Not Deposited" => Some(GiftsNotDeposited),
                "Gift Amount" => Some(GiftAmount),
                "Adjusted Sales Price of Comparable Sales" => {
                    Some(AdjustedSalesPriceOfComparableSales)
                }
                "Predominate Value" => Some(PredominateValue),
                "Average Customer Income" => Some(AverageCustomerIncome),
                "Average Neighborhood Income" => Some(AverageNeighborhoodIncome),
                "Average Customer Purchase" => Some(AverageCustomerPurchase),
                "Weekly Dollar Sales" => Some(WeeklyDollarSales),
                "Average Case Sales" => Some(AverageCaseSales),
                "Buy-down" => Some(BuyDown),
                "Credit Line" => Some(CreditLine),
                "Appraisal Repair Amount" => Some(AppraisalRepairAmount),
                "Brokers Opinion Repair Amount" => Some(BrokersOpinionRepairAmount),
                "Credit Line Available" => Some(CreditLineAvailable),
                "Gross Contribution" => Some(GrossContribution),
                "Grants Not Deposited" => Some(GrantsNotDeposited),
                "Gross Restoration" => Some(GrossRestoration),
                "Grant Amount" => Some(GrantAmount),
                "Subsequent Adjustments Total" => Some(SubsequentAdjustmentsTotal),
                "Good Standing Tax Plan Payment" => Some(GoodStandingTaxPlanPayment),
                "Goods and Services Tax" => Some(GoodsAndServicesTax),
                "Total Supplemental Tax Due" => Some(TotalSupplementalTaxDue),
                "Taxes Paid" => Some(TaxesPaid),
                "Tax Installment Due" => Some(TaxInstallmentDue),
                "Tax Installment Paid" => Some(TaxInstallmentPaid),
                "Total Supplemental Tax Paid" => Some(TotalSupplementalTaxPaid),
                "Gross Value" => Some(GrossValue),
                "Total Charge" => Some(TotalCharge),
                "Total Credit" => Some(TotalCredit),
                "Total Debit" => Some(TotalDebit),
                "Total Finance Charge" => Some(TotalFinanceCharge),
                "Bid Amount" => Some(BidAmount),
                "Host Government (government of the institution) Financing for Education" => {
                    Some(CodeH0)
                }
                "Legal Reserves" => Some(LegalReserves),
                "Cancellation" => Some(Cancellation),
                "Deposit Inception to Date" => Some(DepositInceptionToDate),
                "Deposit Year to Date" => Some(DepositYearToDate),
                "Dump in Remittance" => Some(DumpInRemittance),
                "Earnings" => Some(Earnings),
                "Life Insurance Cash Value" => Some(LifeInsuranceCashValue),
                "Structure Value" => Some(StructureValue),
                "Original List Price" => Some(OriginalListPrice),
                "Coin" => Some(Coin),
                "Currency" => Some(Currency),
                "U.S. Treasury Checks" => Some(USTreasuryChecks),
                "Postal Money Orders" => Some(PostalMoneyOrders),
                "Home Equity Line of Credit Draw Amount" => {
                    Some(HomeEquityLineOfCreditDrawAmount)
                }
                "City Checks" => Some(CityChecks),
                "Hemophilia Adjustment" => Some(HemophiliaAdjustment),
                "Other Checks" => Some(OtherChecks),
                "Home Government Financing for Education" => {
                    Some(HomeGovernmentFinancingForEducation)
                }
                "Annual Social Security Wages" => Some(AnnualSocialSecurityWages),
                "Annual Social Security Tips" => Some(AnnualSocialSecurityTips),
                "Annual Wages, Tips, and Other Compensation" => Some(CodeHJ),
                "Social Security Employee Tax Withheld" => {
                    Some(SocialSecurityEmployeeTaxWithheld)
                }
                "Federal Income Tax Withheld" => Some(FederalIncomeTaxWithheld),
                "Advance Earned Income Credit" => Some(AdvanceEarnedIncomeCredit),
                "Commission" => Some(Commission),
                "Vacation Pay" => Some(VacationPay),
                "Head Office Account" => Some(HeadOfficeAccount),
                "Gross Pay Submitted" => Some(GrossPaySubmitted),
                "Intersell Commission Sales" => Some(IntersellCommissionSales),
                "Total Payroll Approved" => Some(TotalPayrollApproved),
                "Holiday Pay" => Some(HolidayPay),
                "Overtime Pay" => Some(OvertimePay),
                "Regular Pay" => Some(RegularPay),
                "Sick Pay" => Some(SickPay),
                "Special Pay" => Some(SpecialPay),
                "Contract Price" => Some(ContractPrice),
                "Commercial Space Income" => Some(CommercialSpaceIncome),
                "Utilities Paid by Owner" => Some(UtilitiesPaidByOwner),
                "Hazardous Clean-up Cost" => Some(HazardousCleanUpCost),
                "Interest" => Some(Interest),
                "Life Insurance Coverage" => Some(LifeInsuranceCoverage),
                "Investment Income" => Some(InvestmentIncome),
                "Income" => Some(Income),
                "Price, Gross Living Area" => Some(CodeI3),
                "Total Estimated Rent" => Some(TotalEstimatedRent),
                "Gross Annual Income" => Some(GrossAnnualIncome),
                "Custodian's Salary" => Some(CustodiansSalary),
                "Engineer's Salary" => Some(EngineersSalary),
                "Elevator Operator's Salary" => Some(ElevatorOperatorsSalary),
                "Indicated Value by Market Approach Estimate of Market Value" => {
                    Some(IndicatedValueByMarketApproachEstimateOfMarketValue)
                }
                "Adjusted Monthly Rent" => Some(AdjustedMonthlyRent),
                "Investable Balance" => Some(InvestableBalance),
                "Inter-Bank Loans" => Some(InterBankLoans),
                "Accrued Unpaid Interest To Be Capitalized" => {
                    Some(AccruedUnpaidInterestToBeCapitalized)
                }
                "Imbalance Charges Refund" => Some(ImbalanceChargesRefund),
                "Import Duty Amount" => Some(ImportDutyAmount),
                "Excise Tax Amount" => Some(ExciseTaxAmount),
                "Inspection Fee" => Some(InspectionFee),
                "Adjustment for Gross Living Area" => Some(AdjustmentForGrossLivingArea),
                "Inter-Governmental Transfer (IGT) Payments" => Some(CodeIGT),
                "Predominant Price High" => Some(PredominantPriceHigh),
                "Irregular Interest Payment Amount" => {
                    Some(IrregularInterestPaymentAmount)
                }
                "Net Adjusted Monthly Rent" => Some(NetAdjustedMonthlyRent),
                "Indicated Monthly Market Rent" => Some(IndicatedMonthlyMarketRent),
                "Predominant Price Low" => Some(PredominantPriceLow),
                "Adjustment for Rooms" => Some(AdjustmentForRooms),
                "Installment" => Some(Installment),
                "Installment Balance After the Current Installment is Applied" => {
                    Some(InstallmentBalanceAfterTheCurrentInstallmentIsApplied)
                }
                "Adjustment for Bedrooms" => Some(AdjustmentForBedrooms),
                "Interest Payable During Repayment Period" => {
                    Some(InterestPayableDuringRepaymentPeriod)
                }
                "Interest per Diem" => Some(InterestPerDiem),
                "Contingent Debt" => Some(ContingentDebt),
                "Insurance Recovery" => Some(InsuranceRecovery),
                "Independent Scholarship" => Some(IndependentScholarship),
                "Interest Since Claim Submission" => Some(InterestSinceClaimSubmission),
                "Incentive Fee" => Some(IncentiveFee),
                "Accrued Unpaid Interest Not To Be Capitalized" => {
                    Some(AccruedUnpaidInterestNotToBeCapitalized)
                }
                "Utilities Allowance" => Some(UtilitiesAllowance),
                "Investment Property" => Some(InvestmentProperty),
                "Furniture Allowance" => Some(FurnitureAllowance),
                "Debentures" => Some(Debentures),
                "Account High Balance" => Some(AccountHighBalance),
                "Trustee Fees" => Some(TrusteeFees),
                "Limited Partnership Capital" => Some(LimitedPartnershipCapital),
                "Current Face Amount" => Some(CurrentFaceAmount),
                "Original Face Amount" => Some(OriginalFaceAmount),
                "Fixed Default Note Holder's Amount" => {
                    Some(FixedDefaultNoteHoldersAmount)
                }
                "Initial Monthly Payment" => Some(InitialMonthlyPayment),
                "Original Principal and Interest Payment" => {
                    Some(OriginalPrincipalAndInterestPayment)
                }
                "Final Principal and Interest Payment" => {
                    Some(FinalPrincipalAndInterestPayment)
                }
                "Conversion Fee" => Some(ConversionFee),
                "Ending Balance" => Some(EndingBalance),
                "Beginning Balance" => Some(BeginningBalance),
                "Assessment" => Some(Assessment),
                "Equity Claimed as Exempt" => Some(EquityClaimedAsExempt),
                "Counter Claim" => Some(CounterClaim),
                "Weekly Benefit" => Some(WeeklyBenefit),
                "Lease" => Some(Lease),
                "Administrative Load" => Some(AdministrativeLoad),
                "Asset Cost Applicable to Entire Contract" => {
                    Some(AssetCostApplicableToEntireContract)
                }
                "Asset Cost Applicable to Portion of Contract" => {
                    Some(AssetCostApplicableToPortionOfContract)
                }
                "Annual Fee" => Some(AnnualFee),
                "Cost Basis" => Some(CostBasis),
                "Disability Premium" => Some(DisabilityPremium),
                "Employee Additional Contribution" => {
                    Some(EmployeeAdditionalContribution)
                }
                "Employee Match Contribution" => Some(EmployeeMatchContribution),
                "Employer Contribution" => Some(EmployerContribution),
                "Free Look Value" => Some(FreeLookValue),
                "Free Withdrawal Value" => Some(FreeWithdrawalValue),
                "Front End Load" => Some(FrontEndLoad),
                "Guaranteed Minimum Death Benefit" => Some(GuaranteedMinimumDeathBenefit),
                "Interim Value" => Some(InterimValue),
                "Monthly Rent" => Some(MonthlyRent),
                "Judgment" => Some(Judgment),
                "Loan Value" => Some(LoanValue),
                "Market Value" => Some(MarketValue),
                "Market Value Adjusted Value" => Some(MarketValueAdjustedValue),
                "Market Value Adjustment" => Some(MarketValueAdjustment),
                "Net Contract Value" => Some(NetContractValue),
                "Attorney and Trustee Fees" => Some(AttorneyAndTrusteeFees),
                "Discounted Bills not Due" => Some(DiscountedBillsNotDue),
                "Unpaid Security Balance" => Some(UnpaidSecurityBalance),
                "Total Unpaid Security Balance" => Some(TotalUnpaidSecurityBalance),
                "Veterans Affairs Funding Fee" => Some(VeteransAffairsFundingFee),
                "Initial Target Fee" => Some(InitialTargetFee),
                "Minimum Fee" => Some(MinimumFee),
                "Maximum Fee" => Some(MaximumFee),
                "Price" => Some(Price),
                "Special Accounting Classification Reference Number (ACRN) Amount" => {
                    Some(CodeK8)
                }
                "New Price" => Some(NewPrice),
                "Estimated Contract" => Some(EstimatedContract),
                "Estimated Net Adjustment" => Some(EstimatedNetAdjustment),
                "Obligated" => Some(Obligated),
                "Undefinitized" => Some(Undefinitized),
                "Annual Revenue" => Some(AnnualRevenue),
                "Net Paid Amount" => Some(NetPaidAmount),
                "Net Collected Amount" => Some(NetCollectedAmount),
                "Deduction Amount" => Some(DeductionAmount),
                "Net Variance Amount" => Some(NetVarianceAmount),
                "Minimum Contract Amount" => Some(MinimumContractAmount),
                "Item Gross Amount" => Some(ItemGrossAmount),
                "Collected Amount" => Some(CollectedAmount),
                "Disbursed Amount" => Some(DisbursedAmount),
                "Gross Amount of Payment" => Some(GrossAmountOfPayment),
                "Committed Amount" => Some(CommittedAmount),
                "Principal and Interest" => Some(PrincipalAndInterest),
                "Incremental Order Amount" => Some(IncrementalOrderAmount),
                "Liability-Long Term" => Some(LiabilityLongTerm),
                "Taxes and Insurance" => Some(TaxesAndInsurance),
                "Default Principal" => Some(DefaultPrincipal),
                "Default Interest" => Some(DefaultInterest),
                "Liability-Short Term" => Some(LiabilityShortTerm),
                "Default Taxes and Insurance" => Some(DefaultTaxesAndInsurance),
                "Miscellaneous Fee Collections" => Some(MiscellaneousFeeCollections),
                "Not-To-Exceed Price" => Some(NotToExceedPrice),
                "Mortgagor's Monthly Obligations" => Some(MortgagorsMonthlyObligations),
                "Local" => Some(Local),
                "Liquid Assets" => Some(LiquidAssets),
                "Legal Contribution" => Some(LegalContribution),
                "Leasehold Insurance Amount" => Some(LeaseholdInsuranceAmount),
                "Total Unidentified Payments Rejected" => {
                    Some(TotalUnidentifiedPaymentsRejected)
                }
                "Total Credits Received" => Some(TotalCreditsReceived),
                "Total Debits Received" => Some(TotalDebitsReceived),
                "Total Pre-advices Received" => Some(TotalPreAdvicesReceived),
                "Total Prenotes Received" => Some(TotalPrenotesReceived),
                "Total Post-advices Received" => Some(TotalPostAdvicesReceived),
                "Total Debit for Settlement" => Some(TotalDebitForSettlement),
                "Definitized" => Some(Definitized),
                "Definitized Total" => Some(DefinitizedTotal),
                "Lessor's Cost" => Some(LessorsCost),
                "Incremental" => Some(Incremental),
                "Land Rights" => Some(LandRights),
                "Loan Eligibility Amount" => Some(LoanEligibilityAmount),
                "Loan Remittance or Repayment" => Some(LoanRemittanceOrRepayment),
                "Laundry Income" => Some(LaundryIncome),
                "Baseline" => Some(Baseline),
                "Line Item Unit Price" => Some(LineItemUnitPrice),
                "Legal and Audit" => Some(LegalAndAudit),
                "Loan Amount Requested" => Some(LoanAmountRequested),
                "Lump Sum" => Some(LumpSum),
                "Limit" => Some(Limit),
                "Lien Payoff" => Some(LienPayoff),
                "Money Purchase" => Some(MoneyPurchase),
                "Lower Fund" => Some(LowerFund),
                "List Price" => Some(ListPrice),
                "Total Subject Property Liens Paid by Closing" => {
                    Some(TotalSubjectPropertyLiensPaidByClosing)
                }
                "Lease Purchase Funds" => Some(LeasePurchaseFunds),
                "Lease Payments" => Some(LeasePayments),
                "Maximum Potential Liability" => Some(MaximumPotentialLiability),
                "Total Credit for Settlement" => Some(TotalCreditForSettlement),
                "Net Settlement" => Some(NetSettlement),
                "Total Liabilities to be Paid at Closing Not Including Subject Property Liens" => {
                    Some(
                        TotalLiabilitiesToBePaidAtClosingNotIncludingSubjectPropertyLiens,
                    )
                }
                "Loss on Sale Of Property" => Some(LossOnSaleOfProperty),
                "Total Award" => Some(TotalAward),
                "Option Amount" => Some(OptionAmount),
                "Planned Periodic Payment" => Some(PlannedPeriodicPayment),
                "Tax and Insurance Escrow Fund Balance" => {
                    Some(TaxAndInsuranceEscrowFundBalance)
                }
                "Loan Expense" => Some(LoanExpense),
                "Total Remaining Principal Balance for the Issuer" => {
                    Some(TotalRemainingPrincipalBalanceForTheIssuer)
                }
                "Delinquent Payment" => Some(DelinquentPayment),
                "Amount Due from Buyer at Appraisal Notice Date" => {
                    Some(AmountDueFromBuyerAtAppraisalNoticeDate)
                }
                "Loans from Officers" => Some(LoansFromOfficers),
                "Maximum Out of Pocket Amount" => Some(MaximumOutOfPocketAmount),
                "Medical Contribution" => Some(MedicalContribution),
                "Tax rate expressed as a flat fee" => Some(TaxRateExpressedAsAFlatFee),
                "Minimum amount of tax to be paid" => Some(MinimumAmountOfTaxToBePaid),
                "Minimum amount to which tax rate is applied" => {
                    Some(MinimumAmountToWhichTaxRateIsApplied)
                }
                "Maximum amount of tax to be paid" => Some(MaximumAmountOfTaxToBePaid),
                "Maximum amount to which tax rate is applied" => {
                    Some(MaximumAmountToWhichTaxRateIsApplied)
                }
                "Markup Amount" => Some(MarkupAmount),
                "Net of Surrender Withdrawal" => Some(NetOfSurrenderWithdrawal),
                "Maximum Amount" => Some(MaximumAmount),
                "Miscellaneous Adjustment" => Some(MiscellaneousAdjustment),
                "Undistributed Budget" => Some(UndistributedBudget),
                "Cost of Money" => Some(CostOfMoney),
                "Minimum Due" => Some(MinimumDue),
                "Minimum Default Note Holder's Cost" => {
                    Some(MinimumDefaultNoteHoldersCost)
                }
                "Administrative Fees" => Some(AdministrativeFees),
                "Maximum Late Charge" => Some(MaximumLateCharge),
                "Minimum Late Charge" => Some(MinimumLateCharge),
                "Minimum Incentive Fee" => Some(MinimumIncentiveFee),
                "Maximum Default Note Holder's Cost" => {
                    Some(MaximumDefaultNoteHoldersCost)
                }
                "Gross to Pay" => Some(GrossToPay),
                "Prior Net Invoice Total" => Some(PriorNetInvoiceTotal),
                "Payout" => Some(Payout),
                "Modified Mortgage Amount" => Some(ModifiedMortgageAmount),
                "Monthly Limit" => Some(MonthlyLimit),
                "Minimum Order Value" => Some(MinimumOrderValue),
                "Monthly Payment Amount" => Some(MonthlyPaymentAmount),
                "Post Tax Equity and Fiscal Responsibility Act (TEFRA) Cost Basis" => {
                    Some(CodeMQ)
                }
                "Management Reserve" => Some(ManagementReserve),
                "Past-Due Taxes and Assessment Remaining Unpaid" => {
                    Some(PastDueTaxesAndAssessmentRemainingUnpaid)
                }
                "Pre Tax Equity and Fiscal Responsibility Act (TEFRA) Cost Basis" => {
                    Some(CodeMT)
                }
                "Premium Tax Paid on Surrender" => Some(PremiumTaxPaidOnSurrender),
                "Premium Tax Paid up Front" => Some(PremiumTaxPaidUpFront),
                "Sales Loads" => Some(SalesLoads),
                "Maximum Incentive Fee" => Some(MaximumIncentiveFee),
                "Surrender Value" => Some(SurrenderValue),
                "Valuation Price" => Some(ValuationPrice),
                "Net" => Some(Net),
                "Loans or Financial Borrowings" => Some(LoansOrFinancialBorrowings),
                "Net Worth" => Some(NetWorth),
                "Individual Income Taxes and Other" => {
                    Some(IndividualIncomeTaxesAndOther)
                }
                "Corporate Income and Excess Profits Tax" => {
                    Some(CorporateIncomeAndExcessProfitsTax)
                }
                "Excise Taxes" => Some(ExciseTaxes),
                "Estate and Gift Taxes" => Some(EstateAndGiftTaxes),
                "Carrier Tax Act Taxes" => Some(CarrierTaxActTaxes),
                "Federal Unemployment Tax Act Taxes" => {
                    Some(FederalUnemploymentTaxActTaxes)
                }
                "Miscellaneous Taxes" => Some(MiscellaneousTaxes),
                "Withheld and Federal Insurance Contribution Act (FICA) Taxes" => {
                    Some(CodeN9)
                }
                "Net Adjustment" => Some(NetAdjustment),
                "Net Compensation Position" => Some(NetCompensationPosition),
                "Net Benefit" => Some(NetBenefit),
                "Net Worth of Business Owned" => Some(NetWorthOfBusinessOwned),
                "Negative Collected Balance" => Some(NegativeCollectedBalance),
                "Net Contribution" => Some(NetContribution),
                "Per Person Monthly Limit" => Some(PerPersonMonthlyLimit),
                "Net Billed" => Some(NetBilled),
                "Monthly Net Fee Position (Excess/Deficit)" => Some(CodeNF),
                "Medicare Copayment" => Some(MedicareCopayment),
                "Medicare Deductible" => Some(MedicareDeductible),
                "Medicare Paid" => Some(MedicarePaid),
                "Other Insurance Paid Amount" => Some(OtherInsurancePaidAmount),
                "Total in Force and Applied Coverage" => {
                    Some(TotalInForceAndAppliedCoverage)
                }
                "Negative Ledger Balance" => Some(NegativeLedgerBalance),
                "Non-collateralized Amount" => Some(NonCollateralizedAmount),
                "Transaction Fee" => Some(TransactionFee),
                "Non Commission Sales" => Some(NonCommissionSales),
                "Net to Pay Total" => Some(NetToPayTotal),
                "No Tax Plan Payment" => Some(NoTaxPlanPayment),
                "Adjusted Nonrecurring" => Some(AdjustedNonrecurring),
                "Nonrecurring" => Some(Nonrecurring),
                "Net Restoration Expenses" => Some(NetRestorationExpenses),
                "Net Savings Amount" => Some(NetSavingsAmount),
                "Unit Value" => Some(UnitValue),
                "New Technology Adjustment" => Some(NewTechnologyAdjustment),
                "Reinsurance Amount" => Some(ReinsuranceAmount),
                "Renewal Amount" => Some(RenewalAmount),
                "Retention Per Life" => Some(RetentionPerLife),
                "Retention Per Policy" => Some(RetentionPerPolicy),
                "Net Year to Date (Excess/Deficit)" => Some(CodeNY),
                "Equalization Account" => Some(EqualizationAccount),
                "Court Cost" => Some(CourtCost),
                "Extraordinary Income" => Some(ExtraordinaryIncome),
                "Amount of First Mortgage Being Refinanced" => {
                    Some(AmountOfFirstMortgageBeingRefinanced)
                }
                "Other Family Financing for Education" => {
                    Some(OtherFamilyFinancingForEducation)
                }
                "Intangible Assets Written Off" => Some(IntangibleAssetsWrittenOff),
                "Interest Payable" => Some(InterestPayable),
                "Interest Receivable" => Some(InterestReceivable),
                "Joint Venture Results" => Some(JointVentureResults),
                "Long Term Debt" => Some(LongTermDebt),
                "Long Term Provisions" => Some(LongTermProvisions),
                "Loss" => Some(Loss),
                "Principal Balance Amount" => Some(PrincipalBalanceAmount),
                "Outstanding Loan Balance" => Some(OutstandingLoanBalance),
                "Opening Bank Charges" => Some(OpeningBankCharges),
                "Draft Amount" => Some(DraftAmount),
                "Odorization Charge" => Some(OdorizationCharge),
                "Miscellaneous Charges" => Some(MiscellaneousCharges),
                "Office Equipment" => Some(OfficeEquipment),
                "Contractor's Offer" => Some(ContractorsOffer),
                "Operational Flow Order Charge" => Some(OperationalFlowOrderCharge),
                "Operational Flow Order Refund" => Some(OperationalFlowOrderRefund),
                "Cable Charge" => Some(CableCharge),
                "Handling Charges" => Some(HandlingCharges),
                "Non-commission Charges" => Some(NonCommissionCharges),
                "Merchandise" => Some(Merchandise),
                "Letter of Credit Amount" => Some(LetterOfCreditAmount),
                "Outstanding Balance Other Lender" => Some(OutstandingBalanceOtherLender),
                "Other Liability Amounts" => Some(OtherLiabilityAmounts),
                "Other Monthly Income" => Some(OtherMonthlyIncome),
                "Negotiating Bank Charges" => Some(NegotiatingBankCharges),
                "Operational Notice Refund" => Some(OperationalNoticeRefund),
                "Overdrafts" => Some(Overdrafts),
                "Original Payment Total" => Some(OriginalPaymentTotal),
                "Payroll Costs" => Some(PayrollCosts),
                "Letter of Credit Remaining Amount" => {
                    Some(LetterOfCreditRemainingAmount)
                }
                "Other Salaries" => Some(OtherSalaries),
                "Commission Amendment Charges" => Some(CommissionAmendmentCharges),
                "Profit" => Some(Profit),
                "Inpatient Outlier Adjustment" => Some(InpatientOutlierAdjustment),
                "Profit and Loss Deficit" => Some(ProfitAndLossDeficit),
                "Profit after Extraordinary Items and before Tax" => {
                    Some(ProfitAfterExtraordinaryItemsAndBeforeTax)
                }
                "Profit after Tax and Before Extraordinary Items" => {
                    Some(ProfitAfterTaxAndBeforeExtraordinaryItems)
                }
                "Payment Commission" => Some(PaymentCommission),
                "Profit Distributed to Employees" => Some(ProfitDistributedToEmployees),
                "Penalty" => Some(Penalty),
                "Parental Financing for Education" => Some(ParentalFinancingForEducation),
                "Partner's Calendar Year Salary" => Some(PartnersCalendarYearSalary),
                "Prior Plan Year Gross Salary" => Some(PriorPlanYearGrossSalary),
                "Premium Amount" => Some(PremiumAmount),
                "Prior Year's Wage" => Some(PriorYearsWage),
                "Partner's Tax Year Salary" => Some(PartnersTaxYearSalary),
                "Premium Due" => Some(PremiumDue),
                "Partner's K1 Tax Year Amount" => Some(PartnersK1TaxYearAmount),
                "Partner's K1 Calendar Year Amount" => Some(PartnersK1CalendarYearAmount),
                "Current Mortgage Principal Balance" => {
                    Some(CurrentMortgagePrincipalBalance)
                }
                "Payment Cancellation Total" => Some(PaymentCancellationTotal),
                "Policy Advance" => Some(PolicyAdvance),
                "Minimum Delivery Purchase Amount" => Some(MinimumDeliveryPurchaseAmount),
                "Penalty and Interest" => Some(PenaltyAndInterest),
                "Billed Amount" => Some(BilledAmount),
                "Profit (Loss) Before Grants" => Some(CodePBG),
                "Positive Collected Balance" => Some(PositiveCollectedBalance),
                "Processing Allowance" => Some(ProcessingAllowance),
                "Prior Contract Cost Basis" => Some(PriorContractCostBasis),
                "Previous Claim Payments" => Some(PreviousClaimPayments),
                "Prior Contract Surrender Charge" => Some(PriorContractSurrenderCharge),
                "Prior Contract Value" => Some(PriorContractValue),
                "Credit" => Some(Credit),
                "Plan Period Election" => Some(PlanPeriodElection),
                "Profit (Loss) after Extraordinary Items and Tax" => Some(CodePEX),
                "Principal" => Some(Principal),
                "Port Facility Charge" => Some(PortFacilityCharge),
                "Payoff" => Some(Payoff),
                "Proposed Gross Rent for the Subject Property" => {
                    Some(ProposedGrossRentForTheSubjectProperty)
                }
                "Per Occurrence Deductible" => Some(PerOccurrenceDeductible),
                "Per Occurrence Monthly Limit" => Some(PerOccurrenceMonthlyLimit),
                "Past Due" => Some(PastDue),
                "Photograph Fee" => Some(PhotographFee),
                "Positive Ledger Balance" => Some(PositiveLedgerBalance),
                "Last Premium Amount" => Some(LastPremiumAmount),
                "Prior Gross Invoice Total" => Some(PriorGrossInvoiceTotal),
                "Percent Override" => Some(PercentOverride),
                "Payment Prior to Advance" => Some(PaymentPriorToAdvance),
                "Pending Net Sale Proceeds from Non-Real Estate Assets" => {
                    Some(PendingNetSaleProceedsFromNonRealEstateAssets)
                }
                "Pending Net Sale Proceeds from Real Estate Assets" => {
                    Some(PendingNetSaleProceedsFromRealEstateAssets)
                }
                "Advance Amount" => Some(AdvanceAmount),
                "Per Occurrence Limit" => Some(PerOccurrenceLimit),
                "Property Damage" => Some(PropertyDamage),
                "Partial Payroll Payment" => Some(PartialPayrollPayment),
                "Per Occurrence per Day Limit" => Some(PerOccurrencePerDayLimit),
                "Per Occurrence Aggregate Limit" => Some(PerOccurrenceAggregateLimit),
                "Unsecured Priority Claim" => Some(UnsecuredPriorityClaim),
                "Prepetition Charges" => Some(PrepetitionCharges),
                "Per Occurrence Maximum per Week Limit" => {
                    Some(PerOccurrenceMaximumPerWeekLimit)
                }
                "Per Person Maximum per Week Limit" => Some(PerPersonMaximumPerWeekLimit),
                "Per Person per Day Limit" => Some(PerPersonPerDayLimit),
                "Original Principal Balance" => Some(OriginalPrincipalBalance),
                "Amount Owed to Buyer at Appraisal Notice Date" => {
                    Some(AmountOwedToBuyerAtAppraisalNoticeDate)
                }
                "Loans to Affiliated Companies" => Some(LoansToAffiliatedCompanies),
                "Proposed" => Some(Proposed),
                "1035 Exchange" => Some(CodeQ2),
                "401K Transfer" => Some(CodeQ3),
                "Total Prenotes Accepted" => Some(TotalPrenotesAccepted),
                "Total Prenotes Rejected" => Some(TotalPrenotesRejected),
                "Automatic Premium Deduction" => Some(AutomaticPremiumDeduction),
                "Total Post-advices Accepted" => Some(TotalPostAdvicesAccepted),
                "Total Post-advices Rejected" => Some(TotalPostAdvicesRejected),
                "Cash With Application" => Some(CashWithApplication),
                "Combined" => Some(Combined),
                "Credit Card" => Some(CreditCard),
                "Deposit Fund" => Some(DepositFund),
                "Direct Billing" => Some(DirectBilling),
                "Disc Premium" => Some(DiscPremium),
                "Electronic Funds Transfer (EFT)" => Some(CodeQF),
                "Government Allotment" => Some(GovernmentAllotment),
                "Initial Premium" => Some(InitialPremium),
                "Individual Retirement Account 60 Day Rollover" => {
                    Some(IndividualRetirementAccount60DayRollover)
                }
                "Individual Retirement Account Direct Transfer" => {
                    Some(IndividualRetirementAccountDirectTransfer)
                }
                "Individual Retirement Account Regular Contribution" => {
                    Some(IndividualRetirementAccountRegularContribution)
                }
                "Keogh/HR 10" => Some(KeoghHr10),
                "Keogh/HR 10 Transfer" => Some(KeoghHr10Transfer),
                "Quarterly Net Fee Position (Excess/Deficit)" => Some(CodeQN),
                "List Billing" => Some(ListBilling),
                "Modal Premium" => Some(ModalPremium),
                "Payroll Taxes" => Some(PayrollTaxes),
                "Parking Income" => Some(ParkingIncome),
                "Non-Qualified (1035 Exchange)" => Some(CodeQS),
                "PAC - Pre-Authorized Check" => Some(PacPreAuthorizedCheck),
                "Payroll Deduction" => Some(PayrollDeduction),
                "Pension" => Some(Pension),
                "Premium Received With Application" => {
                    Some(PremiumReceivedWithApplication)
                }
                "Profit Sharing Trust" => Some(ProfitSharingTrust),
                "Qualified" => Some(Qualified),
                "Payment Amount" => Some(PaymentAmount),
                "Spend Down" => Some(SpendDown),
                "Loans to Participants" => Some(LoansToParticipants),
                "Fixed, Liquidated Secured Debt" => Some(CodeR1),
                "Contingent Secured Debt" => Some(ContingentSecuredDebt),
                "Disputed Secured Debt" => Some(DisputedSecuredDebt),
                "Unliquidated Secured Debt" => Some(UnliquidatedSecuredDebt),
                "Fixed, Liquidated Unsecured Debt" => Some(CodeR5),
                "Contingent Unsecured Debt" => Some(ContingentUnsecuredDebt),
                "Disputed Unsecured Debt" => Some(DisputedUnsecuredDebt),
                "Unliquidated Unsecured Debt" => Some(UnliquidatedUnsecuredDebt),
                "At Time of Filing" => Some(AtTimeOfFiling),
                "Accelerated Royalty" => Some(AcceleratedRoyalty),
                "Per Person Deductible" => Some(PerPersonDeductible),
                "Refund Check" => Some(RefundCheck),
                "Per Person Limit" => Some(PerPersonLimit),
                "Reservation/Demand - Storage" => Some(ReservationDemandStorage),
                "Reservation/Demand - Transportation" => {
                    Some(ReservationDemandTransportation)
                }
                "Royalty Due" => Some(RoyaltyDue),
                "Deposit Value" => Some(DepositValue),
                "Restitution" => Some(Restitution),
                "Budgeted Redemption" => Some(BudgetedRedemption),
                "Per Person Aggregate Limit" => Some(PerPersonAggregateLimit),
                "Residual Value" => Some(ResidualValue),
                "Rate Amount" => Some(RateAmount),
                "Provision for Long Term Depreciation" => {
                    Some(ProvisionForLongTermDepreciation)
                }
                "Regular Remittance" => Some(RegularRemittance),
                "Remittance Refund" => Some(RemittanceRefund),
                "Resident Manager's Salary" => Some(ResidentManagersSalary),
                "Provisions" => Some(Provisions),
                "Repair" => Some(Repair),
                "Repackaging Cost" => Some(RepackagingCost),
                "Recommended Amount" => Some(RecommendedAmount),
                "Reserve Requirement Amount" => Some(ReserveRequirementAmount),
                "Reserves" => Some(Reserves),
                "Last Payment" => Some(LastPayment),
                "Total Debits Rejected" => Some(TotalDebitsRejected),
                "Total Payments Rejected" => Some(TotalPaymentsRejected),
                "Total Delinquency" => Some(TotalDelinquency),
                "Total Pre-advices Accepted" => Some(TotalPreAdvicesAccepted),
                "Total Pre-advices Rejected" => Some(TotalPreAdvicesRejected),
                "Lender's Total Delinquency" => Some(LendersTotalDelinquency),
                "Submitted Chargeback Claim Amount" => {
                    Some(SubmittedChargebackClaimAmount)
                }
                "Self-Financing for Education" => Some(SelfFinancingForEducation),
                "Salary Amount" => Some(SalaryAmount),
                "Salary with Bonuses" => Some(SalaryWithBonuses),
                "Salary with Commissions" => Some(SalaryWithCommissions),
                "Salary with Subchapter S Corporation Income" => {
                    Some(SalaryWithSubchapterSCorporationIncome)
                }
                "Salary with Partner's Bonuses" => Some(SalaryWithPartnersBonuses),
                "Subchapter S Corporation" => Some(SubchapterSCorporation),
                "Sole Proprietorship" => Some(SoleProprietorship),
                "Period Rental" => Some(PeriodRental),
                "Secured Claim Allowed" => Some(SecuredClaimAllowed),
                "Campaign Summary Amount" => Some(CampaignSummaryAmount),
                "System Adjusted Amount" => Some(SystemAdjustedAmount),
                "Surplus (Deficit) after Grant" => Some(CodeSAG),
                "Salvage" => Some(Salvage),
                "Stated Amount" => Some(StatedAmount),
                "Total Service Charge" => Some(TotalServiceCharge),
                "State Care Tax" => Some(StateCareTax),
                "Sales Charge" => Some(SalesCharge),
                "Service Charges Which Cannot Be Compensated by Balances" => {
                    Some(ServiceChargesWhichCannotBeCompensatedByBalances)
                }
                "Scholarship from Admitting Educational Institution" => {
                    Some(ScholarshipFromAdmittingEducationalInstitution)
                }
                "Secured Borrowed Funds Not Deposited" => {
                    Some(SecuredBorrowedFundsNotDeposited)
                }
                "Sponsor-Financing for Education" => Some(SponsorFinancingForEducation),
                "Surrender Charge" => Some(SurrenderCharge),
                "Subsequent Interest Payment Amount" => {
                    Some(SubsequentInterestPaymentAmount)
                }
                "Surrender Full" => Some(SurrenderFull),
                "Surrender Partial" => Some(SurrenderPartial),
                "Security Personnel's Salary" => Some(SecurityPersonnelsSalary),
                "Supplemental" => Some(Supplemental),
                "Insurance Value" => Some(InsuranceValue),
                "Declared Value" => Some(DeclaredValue),
                "Shipment Value" => Some(ShipmentValue),
                "Pay on Delivery" => Some(PayOnDelivery),
                "Landed Cost Value" => Some(LandedCostValue),
                "Sales Administration Expense" => Some(SalesAdministrationExpense),
                "Special Creditors Amount" => Some(SpecialCreditorsAmount),
                "Setoff" => Some(Setoff),
                "Sales Price" => Some(SalesPrice),
                "Special Debtors Amount" => Some(SpecialDebtorsAmount),
                "Secured Claim" => Some(SecuredClaim),
                "Settlement Refund as approved by the Federal Energy Regulatory Commission" => {
                    Some(
                        SettlementRefundAsApprovedByTheFederalEnergyRegulatoryCommission,
                    )
                }
                "Campaign Summary Amount to be Shared" => {
                    Some(CampaignSummaryAmountToBeShared)
                }
                "Shares in Subsidiary Companies" => Some(SharesInSubsidiaryCompanies),
                "State" => Some(State),
                "Surcharge" => Some(Surcharge),
                "Fixed Monthly Principal Payment" => Some(FixedMonthlyPrincipalPayment),
                "Base Award Fee" => Some(BaseAwardFee),
                "Severance Tax" => Some(SeveranceTax),
                "Initial Buydown Balance" => Some(InitialBuydownBalance),
                "Certification Fee" => Some(CertificationFee),
                "Tax" => Some(Tax),
                "Third-Party Government-Financing for Education" => {
                    Some(ThirdPartyGovernmentFinancingForEducation)
                }
                "Teacher" => Some(Teacher),
                "Total Claim Before Taxes" => Some(TotalClaimBeforeTaxes),
                "Total Submitted Charges" => Some(TotalSubmittedCharges),
                "Total Current Balance" => Some(TotalCurrentBalance),
                "Total Claims" => Some(TotalClaims),
                "Claim" => Some(Claim),
                "Total Credits Accepted" => Some(TotalCreditsAccepted),
                "Total Credits Rejected" => Some(TotalCreditsRejected),
                "Total Debits Accepted" => Some(TotalDebitsAccepted),
                "Total Annual Sales" => Some(TotalAnnualSales),
                "Total Annual Sales to Customer" => Some(TotalAnnualSalesToCustomer),
                "Total Buyer Closing Costs" => Some(TotalBuyerClosingCosts),
                "Proposed Cost" => Some(ProposedCost),
                "Total Commission from Primary and Secondary Sources" => {
                    Some(TotalCommissionFromPrimaryAndSecondarySources)
                }
                "Proposed Profit" => Some(ProposedProfit),
                "Total Depository Accounts" => Some(TotalDepositoryAccounts),
                "Proposed Fee" => Some(ProposedFee),
                "Total Maintenance Expense on All Non-Income Producing Properties" => {
                    Some(TotalMaintenanceExpenseOnAllNonIncomeProducingProperties)
                }
                "Total Maintenance Expense on All Income Producing Properties" => {
                    Some(TotalMaintenanceExpenseOnAllIncomeProducingProperties)
                }
                "Total Proposed Price" => Some(TotalProposedPrice),
                "Alternate Proposed Price" => Some(AlternateProposedPrice),
                "Total Gifts Not Deposited" => Some(TotalGiftsNotDeposited),
                "Total Claim Allowed" => Some(TotalClaimAllowed),
                "Title Insurance Amount on Loan" => Some(TitleInsuranceAmountOnLoan),
                "Total Self-employed Income from Primary and Secondary Sources" => {
                    Some(TotalSelfEmployedIncomeFromPrimaryAndSecondarySources)
                }
                "Time and Expense Paid to Date" => Some(TimeAndExpensePaidToDate),
                "Total Amount of Contract" => Some(TotalAmountOfContract),
                "Total Prior Loan Amount Owed" => Some(TotalPriorLoanAmountOwed),
                "Total Other Liquid Assets" => Some(TotalOtherLiquidAssets),
                "Total Life Insurance Net Cash Value" => {
                    Some(TotalLifeInsuranceNetCashValue)
                }
                "Time and Expense Paid Current Month" => {
                    Some(TimeAndExpensePaidCurrentMonth)
                }
                "Total Monetary Markup Amount" => Some(TotalMonetaryMarkupAmount),
                "Telephone Operator's Salary" => Some(TelephoneOperatorsSalary),
                "Total Omitted Liabilities" => Some(TotalOmittedLiabilities),
                "Total payment amount" => Some(TotalPaymentAmount),
                "Total Previous Adjusted Claim" => Some(TotalPreviousAdjustedClaim),
                "Total Net Proceeds from Real Estate Assets" => {
                    Some(TotalNetProceedsFromRealEstateAssets)
                }
                "Total Paid as Submitted" => Some(TotalPaidAsSubmitted),
                "Subsidies for Operating Costs" => Some(SubsidiesForOperatingCosts),
                "Target Cost" => Some(TargetCost),
                "Total Retirement Funds" => Some(TotalRetirementFunds),
                "Total Non-rental Income" => Some(TotalNonRentalIncome),
                "Total Resubordinated Liabilities" => {
                    Some(TotalResubordinatedLiabilities)
                }
                "Total Liabilities for Rental Properties" => {
                    Some(TotalLiabilitiesForRentalProperties)
                }
                "Total Sales" => Some(TotalSales),
                "Total Stocks and Bonds" => Some(TotalStocksAndBonds),
                "Total Transaction Amount" => Some(TotalTransactionAmount),
                "Transportation Cost per Unit of Measure" => {
                    Some(TransportationCostPerUnitOfMeasure)
                }
                "Level of Premium Insurance - Retention" => {
                    Some(LevelOfPremiumInsuranceRetention)
                }
                "Technicians Indemnity Provision" => Some(TechniciansIndemnityProvision),
                "Total to Date" => Some(TotalToDate),
                "Total at Complete" => Some(TotalAtComplete),
                "Transportation Cost Total" => Some(TransportationCostTotal),
                "Underpayment" => Some(Underpayment),
                "U.S. Government-Financing for Education" => {
                    Some(USGovernmentFinancingForEducation)
                }
                "Unsecured, Priority Claim Allowed" => Some(CodeU1),
                "Ingredient Cost Claimed" => Some(IngredientCostClaimed),
                "Miscellaneous Expenses" => Some(MiscellaneousExpenses),
                "Present Value of Lot" => Some(PresentValueOfLot),
                "Cost of Improvements" => Some(CostOfImprovements),
                "Alterations, Improvements, Repairs" => Some(CodeU6),
                "Land" => Some(Land),
                "Refinance" => Some(Refinance),
                "Tax Payment Refinanced by Same Lender" => {
                    Some(TaxPaymentRefinancedBySameLender)
                }
                "Estimated Prepaid Items" => Some(EstimatedPrepaidItems),
                "Unliquidated Amount" => Some(UnliquidatedAmount),
                "User Adjusted Amount" => Some(UserAdjustedAmount),
                "Refund of Unauthorized Overrun Charges (UAOR Refund)" => Some(CodeUAR),
                "Unpaid Principal Balance" => Some(UnpaidPrincipalBalance),
                "Unspecified Aggregate Limit" => Some(UnspecifiedAggregateLimit),
                "Unsecured, Nonpriority Claim Allowed" => Some(CodeUD),
                "Mortgage Insurance" => Some(MortgageInsurance),
                "Updated Expenses on Presale" => Some(UpdatedExpensesOnPresale),
                "Discount (If Borrower Paid)" => Some(CodeUF),
                "Unsecured Borrowed Funds Not Deposited" => {
                    Some(UnsecuredBorrowedFundsNotDeposited)
                }
                "Total Unpaid Principal Balance for Stafford Loans" => {
                    Some(TotalUnpaidPrincipalBalanceForStaffordLoans)
                }
                "Subordinate Financing" => Some(SubordinateFinancing),
                "Unsecured Home Improvements" => Some(UnsecuredHomeImprovements),
                "Total Costs" => Some(TotalCosts),
                "Updated Interest on Presale" => Some(UpdatedInterestOnPresale),
                "Other Credits" => Some(OtherCredits),
                "Base Loan Amount (w/o financed Mortgage Insurance)" => Some(CodeUK),
                "Mortgage Insurance Financed" => Some(MortgageInsuranceFinanced),
                "Total Loan Amount" => Some(TotalLoanAmount),
                "Unsecured, Nonpriority Claim" => Some(CodeUN),
                "Unknown Tax Plan Payment" => Some(UnknownTaxPlanPayment),
                "Cash from or to Borrower" => Some(CashFromOrToBorrower),
                "Total Unpaid Principal Balance for Parental Loans for Students" => {
                    Some(TotalUnpaidPrincipalBalanceForParentalLoansForStudents)
                }
                "Upper Fund" => Some(UpperFund),
                "Monthly Income" => Some(MonthlyIncome),
                "Unearned Income" => Some(UnearnedIncome),
                "Total Unpaid Principal Balance for Supplemental Loans for Students" => {
                    Some(TotalUnpaidPrincipalBalanceForSupplementalLoansForStudents)
                }
                "Value Added Sales" => Some(ValueAddedSales),
                "Clearing House Settlement" => Some(ClearingHouseSettlement),
                "Drawback" => Some(Drawback),
                "Total Monthly Liabilities" => Some(TotalMonthlyLiabilities),
                "Utilities, Furniture, and Amenities Included in Rent" => Some(CodeUX),
                "Total Assets" => Some(TotalAssets),
                "Total Liquid Assets" => Some(TotalLiquidAssets),
                "Cost of Deficiency" => Some(CostOfDeficiency),
                "Value Added" => Some(ValueAdded),
                "Tax and Insurance Escrow Fund" => Some(TaxAndInsuranceEscrowFund),
                "Interest Due to Investor" => Some(InterestDueToInvestor),
                "Total Principal Due to the Investor" => {
                    Some(TotalPrincipalDueToTheInvestor)
                }
                "Total Interest Due to the Investor" => {
                    Some(TotalInterestDueToTheInvestor)
                }
                "Total Curtailment Due to the Investor" => {
                    Some(TotalCurtailmentDueToTheInvestor)
                }
                "Total Principal Payoff and Repurchase Due to the Investor" => {
                    Some(TotalPrincipalPayoffAndRepurchaseDueToTheInvestor)
                }
                "Total Interest Payoff and Repurchase Due to the Investor" => {
                    Some(TotalInterestPayoffAndRepurchaseDueToTheInvestor)
                }
                "Actual Outstanding Principal Balance" => {
                    Some(ActualOutstandingPrincipalBalance)
                }
                "Face Amount" => Some(FaceAmount),
                "Total Current Rent or Mortgage Payment (Issue)" => Some(CodeVA),
                "Total Non-liquid Assets" => Some(TotalNonLiquidAssets),
                "Authorized" => Some(Authorized),
                "Actual Person Day Rate" => Some(ActualPersonDayRate),
                "Estimated Person Day Rate" => Some(EstimatedPersonDayRate),
                "Vested/Earned Upper Fund" => Some(VestedEarnedUpperFund),
                "Total Monthly Expenses" => Some(TotalMonthlyExpenses),
                "Current Monthly Principal and Interest" => {
                    Some(CurrentMonthlyPrincipalAndInterest)
                }
                "Levy Amount" => Some(LevyAmount),
                "Current Support" => Some(CurrentSupport),
                "Past Due Support" => Some(PastDueSupport),
                "Medical Support" => Some(MedicalSupport),
                "Net Negative Amortization Amount" => Some(NetNegativeAmortizationAmount),
                "Withhold From Wages" => Some(WithholdFromWages),
                "Commission Basis" => Some(CommissionBasis),
                "Commission Earned" => Some(CommissionEarned),
                "Current Monthly Payment" => Some(CurrentMonthlyPayment),
                "Commission Netted" => Some(CommissionNetted),
                "Total Monthly Debt" => Some(TotalMonthlyDebt),
                "Volumetric Reservation" => Some(VolumetricReservation),
                "Other Financing Payment" => Some(OtherFinancingPayment),
                "Value of Securities at Issue Date" => Some(ValueOfSecuritiesAtIssueDate),
                "Value of Securities at Maturity" => Some(ValueOfSecuritiesAtMaturity),
                "Current Value" => Some(CurrentValue),
                "Closing Cost" => Some(ClosingCost),
                "Capitalized Mortgage Amount" => Some(CapitalizedMortgageAmount),
                "First Mortgage Monthly Principal and Interest" => {
                    Some(FirstMortgageMonthlyPrincipalAndInterest)
                }
                "Interest Amount Paid to Date" => Some(InterestAmountPaidToDate),
                "Minimum Transfer" => Some(MinimumTransfer),
                "Maximum Transfer" => Some(MaximumTransfer),
                "Deficiency Judgment Fees" => Some(DeficiencyJudgmentFees),
                "Trade Debtors" => Some(TradeDebtors),
                "W-2" => Some(W2),
                "W-2 with Bonuses" => Some(W2WithBonuses),
                "W-2 with Deferred Compensation" => Some(W2WithDeferredCompensation),
                "W-2 without Bonuses" => Some(W2WithoutBonuses),
                "Deposit Sub Total" => Some(DepositSubTotal),
                "Direct Rollover" => Some(DirectRollover),
                "Direct Transfer" => Some(DirectTransfer),
                "Discounted" => Some(Discounted),
                "Secondary Finance" => Some(SecondaryFinance),
                "Minimum Deposit" => Some(MinimumDeposit),
                "Sub-Agency Compensation" => Some(SubAgencyCompensation),
                "Buyers Agency Compensation" => Some(BuyersAgencyCompensation),
                "Variable Rate Compensation" => Some(VariableRateCompensation),
                "Compensation Bonus on Sale of Property" => {
                    Some(CompensationBonusOnSaleOfProperty)
                }
                "Veterans Affairs Loan Guarantee" => Some(VeteransAffairsLoanGuarantee),
                "Security Trade Amount" => Some(SecurityTradeAmount),
                "Balance Owing All Other Liens, Subject Property" => Some(CodeWH),
                "Other Financing" => Some(OtherFinancing),
                "Dual Agency Compensation" => Some(DualAgencyCompensation),
                "Per Week Limit" => Some(PerWeekLimit),
                "Lender's Opinion of Value" => Some(LendersOpinionOfValue),
                "Total Original Principal Balance" => Some(TotalOriginalPrincipalBalance),
                "Other Agent Compensation" => Some(OtherAgentCompensation),
                "Dock Usage Fee" => Some(DockUsageFee),
                "Pool Usage Fee" => Some(PoolUsageFee),
                "Clubhouse Fee" => Some(ClubhouseFee),
                "Optional Service Fee" => Some(OptionalServiceFee),
                "Other Association Fees" => Some(OtherAssociationFees),
                "Principal, Interest, Taxes" => Some(CodeWT),
                "Principal, Interest, Taxes and Insurance" => Some(CodeWU),
                "Total Points Paid at Closing" => Some(TotalPointsPaidAtClosing),
                "Amount that Would Have Been Paid in the Absence of Capitation" => {
                    Some(AmountThatWouldHaveBeenPaidInTheAbsenceOfCapitation)
                }
                "Points Paid by Seller" => Some(PointsPaidBySeller),
                "Loan Withdrawal" => Some(LoanWithdrawal),
                "Severance Pay" => Some(SeverancePay),
                "Deficiency Judgment Expenses and Fees" => {
                    Some(DeficiencyJudgmentExpensesAndFees)
                }
                "Treble Damages" => Some(TrebleDamages),
                "Transfer to Untaxed Reserves" => Some(TransferToUntaxedReserves),
                "Reissued" => Some(Reissued),
                "Rollover Amount" => Some(RolloverAmount),
                "Annual Rental" => Some(AnnualRental),
                "Gross Monthly Rent" => Some(GrossMonthlyRent),
                "SEP - Self Employee Pension" => Some(SepSelfEmployeePension),
                "Funding Amount" => Some(FundingAmount),
                "Tax Sheltered Annuity (403B Transfer)" => Some(CodeX9),
                "Maximum Award Fee" => Some(MaximumAwardFee),
                "Maturity Value" => Some(MaturityValue),
                "Earned Wages" => Some(EarnedWages),
                "Base Period Wage" => Some(BasePeriodWage),
                "Withdrawal" => Some(Withdrawal),
                "Withdrawal Inception to Date" => Some(WithdrawalInceptionToDate),
                "Withdrawal Less Market Value Adjustment" => {
                    Some(WithdrawalLessMarketValueAdjustment)
                }
                "Withdrawal Less Surrender" => Some(WithdrawalLessSurrender),
                "Withdrawal Less Taxes" => Some(WithdrawalLessTaxes),
                "Withdrawal Year to Date" => Some(WithdrawalYearToDate),
                "Unavailable Reserves" => Some(UnavailableReserves),
                "Uncalled Capital" => Some(UncalledCapital),
                "Unemployment Contribution" => Some(UnemploymentContribution),
                "Unlimited Capital, Minimum Fixed" => Some(CodeXN),
                "Unpaid Capital" => Some(UnpaidCapital),
                "Unsecured Liabilities" => Some(UnsecuredLiabilities),
                "Value Added Tax" => Some(ValueAddedTax),
                "Value of Shares" => Some(ValueOfShares),
                "Vehicles" => Some(Vehicles),
                "Voluntary Reserves" => Some(VoluntaryReserves),
                "Extraction" => Some(Extraction),
                "Wages" => Some(Wages),
                "Withholding" => Some(Withholding),
                "Original Value" => Some(OriginalValue),
                "Working Capital" => Some(WorkingCapital),
                "Sales Price Per Dwelling Unit" => Some(SalesPricePerDwellingUnit),
                "Sales Price Per Room" => Some(SalesPricePerRoom),
                "Current List Price" => Some(CurrentListPrice),
                "Self Insurance Amount" => Some(SelfInsuranceAmount),
                "Year to Date Eligible Salary" => Some(YearToDateEligibleSalary),
                "Total Real Estate Owned" => Some(TotalRealEstateOwned),
                "Total Liabilities" => Some(TotalLiabilities),
                "Total Liability Monthly Payments" => Some(TotalLiabilityMonthlyPayments),
                "Total Real Estate Owned Market Value" => {
                    Some(TotalRealEstateOwnedMarketValue)
                }
                "Total Real Estate Owned Gross Rental Income" => {
                    Some(TotalRealEstateOwnedGrossRentalIncome)
                }
                "Total Real Estate Owned Mortgages and Liens" => {
                    Some(TotalRealEstateOwnedMortgagesAndLiens)
                }
                "Total Real Estate Owned Mortgage Payments" => {
                    Some(TotalRealEstateOwnedMortgagePayments)
                }
                "Total Real Estate Owned Miscellaneous Expenses" => {
                    Some(TotalRealEstateOwnedMiscellaneousExpenses)
                }
                "Total Real Estate Owned Net Rental Income" => {
                    Some(TotalRealEstateOwnedNetRentalIncome)
                }
                "Actual Unpaid Principal Balance" => Some(ActualUnpaidPrincipalBalance),
                "Scheduled Unpaid Principal Balance" => {
                    Some(ScheduledUnpaidPrincipalBalance)
                }
                "Principal Due to Investor" => Some(PrincipalDueToInvestor),
                "Constant Principal and Interest" => Some(ConstantPrincipalAndInterest),
                "Other Fee Collection" => Some(OtherFeeCollection),
                "Beginning Scheduled Unpaid Principal Balance" => {
                    Some(BeginningScheduledUnpaidPrincipalBalance)
                }
                "Tax and Insurance Principal Balance" => {
                    Some(TaxAndInsurancePrincipalBalance)
                }
                "New Principal and Interest" => Some(NewPrincipalAndInterest),
                "Curtailment" => Some(Curtailment),
                "Prepayment Penalty" => Some(PrepaymentPenalty),
                "Partial Annuitization" => Some(PartialAnnuitization),
                "Partial Withdrawal" => Some(PartialWithdrawal),
                "Post Tax Equity and Fiscal Responsibility Act (TEFRA) Gain" => {
                    Some(CodeYN)
                }
                "Pre Tax Equity and Fiscal Responsibility Act (TEFRA) Gain" => {
                    Some(CodeYO)
                }
                "Policy Amount" => Some(PolicyAmount),
                "Payments in Advance" => Some(PaymentsInAdvance),
                "Payments in Arrears" => Some(PaymentsInArrears),
                "Cancelled" => Some(Cancelled),
                "Denied" => Some(Denied),
                "In Process" => Some(InProcess),
                "Requested" => Some(Requested),
                "Paid" => Some(Paid),
                "Paid for This Facility" => Some(PaidForThisFacility),
                "Returned" => Some(Returned),
                "Total Aggregate Limit" => Some(TotalAggregateLimit),
                "List Price When Sold" => Some(ListPriceWhenSold),
                "Insertion Cost" => Some(InsertionCost),
                "Repackaging Labor Cost" => Some(RepackagingLaborCost),
                "Repackaging Material Cost" => Some(RepackagingMaterialCost),
                "Unit Cost of Discrepant Material" => Some(UnitCostOfDiscrepantMaterial),
                "Liquidation Principal" => Some(LiquidationPrincipal),
                "Remaining Pool Balance" => Some(RemainingPoolBalance),
                "Remaining Security Balance" => Some(RemainingSecurityBalance),
                "Program Cost" => Some(ProgramCost),
                "Override to Handling Fee" => Some(OverrideToHandlingFee),
                "Production Cost" => Some(ProductionCost),
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
                "Federal Pension Mandate - Category 4" => {
                    Some(FederalPensionMandateCategory4)
                }
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
                "Coupon Face Value" => Some(CouponFaceValue),
                "Initial Target Cost" => Some(InitialTargetCost),
                "Increase" => Some(Increase),
                "Decrease" => Some(Decrease),
                "Prorated Amount" => Some(ProratedAmount),
                "Loan Charge" => Some(LoanCharge),
                "Mortgage Recording Fee" => Some(MortgageRecordingFee),
                "Deed Recording Fee" => Some(DeedRecordingFee),
                "Release Recording Fee" => Some(ReleaseRecordingFee),
                "Assumption" => Some(Assumption),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for AmountQualifierCode {
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
    type Value = AmountQualifierCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Amount Qualifier Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AmountQualifierCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Amount Qualifier Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AmountQualifierCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Amount Qualifier Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for AmountQualifierCode {
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