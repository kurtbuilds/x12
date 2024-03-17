use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**738

See docs at <https://www.stedi.com/edi/x12-005010/element/738>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementQualifier {
    ///1 - Control Efficiency
    ControlEfficiency,
    ///1F - Radio Frequency
    RadioFrequency,
    ///2 - Capture Efficiency
    CaptureEfficiency,
    ///2F - Alternate Radio Frequency
    AlternateRadioFrequency,
    ///3 - Photonflux Density
    PhotonfluxDensity,
    ///3A - Target Depth
    TargetDepth,
    ///3B - Current Depth
    CurrentDepth,
    ///3C - Total Depth
    TotalDepth,
    ///3D - Well Test Before Oil
    WellTestBeforeOil,
    ///3E - Well Test Before Gas
    WellTestBeforeGas,
    ///3F - Well Test Before Water
    WellTestBeforeWater,
    ///3G - Well Test After Oil
    WellTestAfterOil,
    ///3H - Well Test After Gas
    WellTestAfterGas,
    ///3I - Well Test After Water
    WellTestAfterWater,
    ///3J - Estimated Depth of Operations
    EstimatedDepthOfOperations,
    ///4 - Throughput Rate
    ThroughputRate,
    ///4F - Squelch Tone
    SquelchTone,
    ///5 - Cloud Cover
    CloudCover,
    ///5F - Height above Ground
    HeightAboveGround,
    ///6 - Velocity
    Velocity,
    ///6F - Gain
    Gain,
    ///7 - Plume Height
    PlumeHeight,
    ///8 - Individuals
    Individuals,
    ///8F - Directional Height above Average Terrain
    DirectionalHeightAboveAverageTerrain,
    ///9 - Storage Limits
    StorageLimits,
    ///10 - Painting Costs
    PaintingCosts,
    ///11 - Structural Costs
    StructuralCosts,
    ///12 - Appliances
    Appliances,
    ///13 - Utilities
    Utilities,
    ///14 - Carpet or Floors
    CarpetOrFloors,
    ///15 - Other Repairs
    OtherRepairs,
    ///16 - Landscaping
    Landscaping,
    ///18 - Roof
    Roof,
    ///19 - Windows
    Windows,
    ///20 - Cleaning or Trash Removal
    CleaningOrTrashRemoval,
    ///21 - Probable Sales Price
    ProbableSalesPrice,
    ///22 - Proximity
    Proximity,
    ///23 - Repairs and Improvements
    RepairsAndImprovements,
    ///24 - Contributory Value of Repairs and Improvements
    ContributoryValueOfRepairsAndImprovements,
    ///25 - Marketing Time
    MarketingTime,
    ///26 - Closed Comparable Sales
    ClosedComparableSales,
    ///27 - Competitive Listings in Price Range
    CompetitiveListingsInPriceRange,
    ///28 - Financing Concessions
    FinancingConcessions,
    ///29 - Marketing Concessions
    MarketingConcessions,
    ///30 - Probable Net Price
    ProbableNetPrice,
    ///31 - Suggested Initial List Price
    SuggestedInitialListPrice,
    ///32 - Value Change
    ValueChange,
    ///33 - Probable Final Value
    ProbableFinalValue,
    ///34 - Occupancy Rate
    OccupancyRate,
    ///35 - Number of Living Units
    NumberOfLivingUnits,
    ///36 - Number of Phases
    NumberOfPhases,
    ///37 - Number of Active Listings
    NumberOfActiveListings,
    ///38 - Price Active Listings
    PriceActiveListings,
    ///40 - Price Per Gross Living Area
    PricePerGrossLivingArea,
    ///41 - Built-up Rate
    BuiltUpRate,
    ///42 - Vacant Rate
    VacantRate,
    ///43 - Typical Rents
    TypicalRents,
    ///44 - Neighborhood Apartment Vacancy
    NeighborhoodApartmentVacancy,
    ///45 - Number of Admissions
    NumberOfAdmissions,
    ///48 - Cost of Hire
    CostOfHire,
    ///49 - Frontage
    Frontage,
    ///50 - Gross Sales
    GrossSales,
    ///51 - Number of Employees
    NumberOfEmployees,
    ///52 - Payroll
    Payroll,
    ///53 - Per Capita or Each
    PerCapitaOrEach,
    ///54 - Remuneration
    Remuneration,
    ///56 - Total Cost
    TotalCost,
    ///57 - Total Mileage
    TotalMileage,
    ///58 - Number of Rating Units
    NumberOfRatingUnits,
    ///62 - Garage Employee Payroll Maximum
    GarageEmployeePayrollMaximum,
    ///63 - Employee Gross Wage Less Allowable Deductions
    EmployeeGrossWageLessAllowableDeductions,
    ///65 - Garage Employee Average Hours Worked Per Week
    GarageEmployeeAverageHoursWorkedPerWeek,
    ///66 - Garage (Dealers) Employee Weeks Worked
    Code66,
    ///68 - Gross Wage
    GrossWage,
    ///78 - Subcontractor - Labor and Materials
    SubcontractorLaborAndMaterials,
    ///79 - Subcontractor - Labor Only
    SubcontractorLaborOnly,
    ///A - Consolidated Weight
    ConsolidatedWeight,
    ///A1 - Acids
    Acids,
    ///A2 - Adsorption
    Adsorption,
    ///A3 - Aging Time
    AgingTime,
    ///A4 - Aromatics
    Aromatics,
    ///A5 - Average Differential Pressure
    AverageDifferentialPressure,
    ///A6 - Average Static Pressure
    AverageStaticPressure,
    ///A7 - Flame Projection Distance
    FlameProjectionDistance,
    ///A9 - Exposure
    Exposure,
    ///AA - Alternating Current
    AlternatingCurrent,
    ///AAP - AC-apparent Power
    AcApparentPower,
    ///AB - Activation Energy
    ActivationEnergy,
    ///ABO - Absorbance
    Absorbance,
    ///ABR - Abrasion
    Abrasion,
    ///ABS - Absorbency
    Absorbency,
    ///AC - Actinium
    Actinium,
    ///ACN - Acid Number
    AcidNumber,
    ///AD - Ambient Temperature
    AmbientTemperature,
    ///ADH - Adhesion
    Adhesion,
    ///ADM - Dye Manufacturing Units
    DyeManufacturingUnits,
    ///AE - Argon
    Argon,
    ///AF - Angle of Bend
    AngleOfBend,
    ///AG - Americium
    Americium,
    ///AGE - Inventory Age
    InventoryAge,
    ///AGI - Aggressive Index
    AggressiveIndex,
    ///AH - Astatine
    Astatine,
    ///AI - Acidity
    Acidity,
    ///AJ - Aim Gage
    AimGage,
    ///AK - Volatile Organic Compounds (VOCs)
    CodeAK,
    ///AL - Spine Show
    SpineShow,
    ///ALK - Alkalinity
    Alkalinity,
    ///ALN - Alkalinity Number
    AlkalinityNumber,
    ///ALP - Alpha-Cellulose
    AlphaCellulose,
    ///AM - Average Speed
    AverageSpeed,
    ///AMI - Amines
    Amines,
    ///AMW - Average Molecular Weight
    AverageMolecularWeight,
    ///AN - Flute Test
    FluteTest,
    ///AOX - Antioxidant
    Antioxidant,
    ///AP - Average Pressure
    AveragePressure,
    ///API - API Gravity
    ApiGravity,
    ///APP - Appearance
    Appearance,
    ///AS - Ash Fusion Temperature
    AshFusionTemperature,
    ///ASH - Ash
    Ash,
    ///ASY - Assay
    Assay,
    ///AT - Additive
    Additive,
    ///AU - Number of Units Projected
    NumberOfUnitsProjected,
    ///AV - Age
    Age,
    ///AVT - Average Temperature
    AverageTemperature,
    ///AW - Remaining Economic Life
    RemainingEconomicLife,
    ///AX - Remaining Physical Life
    RemainingPhysicalLife,
    ///AY - Number of Comparable Sales
    NumberOfComparableSales,
    ///AZ - Arbor Size
    ArborSize,
    ///B - Billed Weight
    BilledWeight,
    ///B1 - Base Number
    BaseNumber,
    ///B2 - Number of Comparable Listings
    NumberOfComparableListings,
    ///B3 - Present Land Use
    PresentLandUse,
    ///B4 - Subject Phase Dwelling Units
    SubjectPhaseDwellingUnits,
    ///B5 - Octanol/Water Partition Coefficient
    OctanolWaterPartitionCoefficient,
    ///B6 - Total Project Dwelling Units
    TotalProjectDwellingUnits,
    ///BA - Barium
    Barium,
    ///BB - Beryllium
    Beryllium,
    ///BC - Billet Size
    BilletSize,
    ///BD - Bias
    Bias,
    ///BDP - Perforation Bottom Depth
    PerforationBottomDepth,
    ///BE - Boron Factor
    BoronFactor,
    ///BF - Brinell
    Brinell,
    ///BG - Berkelium
    Berkelium,
    ///BH - Bromine
    Bromine,
    ///BHF - Bottomhole Pressure - Flowing
    BottomholePressureFlowing,
    ///BHS - Bottomhole Pressure - Shutin
    BottomholePressureShutin,
    ///BIC - Bark in Chips
    BarkInChips,
    ///BJ - Burst Index
    BurstIndex,
    ///BK - Bulk
    Bulk,
    ///BL - Blisters
    Blisters,
    ///BN - Bend
    Bend,
    ///BND - Amount Bound in Material
    AmountBoundInMaterial,
    ///BO - Lateral Bow (Camber)
    CodeBO,
    ///BOR - Boiling Range
    BoilingRange,
    ///BP - Boiling Point
    BoilingPoint,
    ///BQ - Breaks
    Breaks,
    ///BR - Brightness
    Brightness,
    ///BRS - Breaking Strength
    BreakingStrength,
    ///BSW - Percent Bottom Sediment and Water
    PercentBottomSedimentAndWater,
    ///BT - Bursts
    Bursts,
    ///BU - Buckles
    Buckles,
    ///BUD - Bulk Density
    BulkDensity,
    ///BW - Basis Weight
    BasisWeight,
    ///BX - Blood Alcohol
    BloodAlcohol,
    ///C - Actual New Repeated for Combination
    ActualNewRepeatedForCombination,
    ///C0 - Color Grade
    ColorGrade,
    ///C1 - Carbonyl
    Carbonyl,
    ///C2 - Catalyst
    Catalyst,
    ///C3 - Maximum Contraction
    MaximumContraction,
    ///C4 - Color Quadrant
    ColorQuadrant,
    ///CA - Caliper
    Caliper,
    ///CAU - Caustic Reaction Severity
    CausticReactionSeverity,
    ///CC - Celsius
    Celsius,
    ///CCF - Composite Corrected Factor
    CompositeCorrectedFactor,
    ///CCG - Combined Center of Gravity
    CombinedCenterOfGravity,
    ///CD - Compression
    Compression,
    ///CE - Capacitance In
    CapacitanceIn,
    ///CF - Capacitance Out
    CapacitanceOut,
    ///CG - Cadmium
    Cadmium,
    ///CGR - Color Grayness RD
    ColorGraynessRd,
    ///CH - Cesium
    Cesium,
    ///CHA - Chemical Addition Rate
    ChemicalAdditionRate,
    ///CHB - Chlorophyll-a
    ChlorophyllA,
    ///CHC - Concentration of Hazardous Component
    ConcentrationOfHazardousComponent,
    ///CHG - Rate of Change
    RateOfChange,
    ///CHL - Chlorides
    Chlorides,
    ///CI - Curium
    Curium,
    ///CIV - Cuene Intrinsic Viscosity
    CueneIntrinsicViscosity,
    ///CJ - Cycle Time
    CycleTime,
    ///CK - Californium
    Californium,
    ///CL - Cladding Thickness(% of Composite Thickness)
    CodeCL,
    ///CLA - Clarity
    Clarity,
    ///CLB - Calibration
    Calibration,
    ///CLN - Cleanliness
    Cleanliness,
    ///CM - Cures
    Cures,
    ///CN - Chlorine
    Chlorine,
    ///CO - Core Loss
    CoreLoss,
    ///COF - Coefficient Factor
    CoefficientFactor,
    ///COH - Coercive Force
    CoerciveForce,
    ///COL - Color
    Color,
    ///CON - Concentration
    Concentration,
    ///COR - Corrosiveness
    Corrosiveness,
    ///COS - Cost
    Cost,
    ///COT - Content
    Content,
    ///CP - Crimp
    Crimp,
    ///CPF - Casing Pressure - Flowing
    CasingPressureFlowing,
    ///CPS - Casing Pressure - Shutin
    CasingPressureShutin,
    ///CPT - Cup Test Adhesion
    CupTestAdhesion,
    ///CQ - Cuts
    Cuts,
    ///CR - Crosswise Spacing
    CrosswiseSpacing,
    ///CRF - Free Chlorine Residual
    FreeChlorineResidual,
    ///CRL - Compression Relaxation
    CompressionRelaxation,
    ///CRN - Crown
    Crown,
    ///CRT - Total Chlorine Residual
    TotalChlorineResidual,
    ///CS - Cross Section
    CrossSection,
    ///CSC - Choke Size - Casing
    ChokeSizeCasing,
    ///CSR - Cost Realism
    CostRealism,
    ///CST - Choke Size - Tubing
    ChokeSizeTubing,
    ///CT - Center-to-center
    CenterToCenter,
    ///CTG - Coating
    Coating,
    ///CTT - Contact Time
    ContactTime,
    ///CU - Coil Curvature
    CoilCurvature,
    ///CUT - Cure Time
    CureTime,
    ///CW - Cuttable Width
    CuttableWidth,
    ///CWT - Charge Weight
    ChargeWeight,
    ///CX - Calculated Value
    CalculatedValue,
    ///CY - Contamination
    Contamination,
    ///CYB - Color Yellowness (+B)
    CodeCYB,
    ///D - Destination Weight Agreement
    DestinationWeightAgreement,
    ///D1 - Maximum Dilatation
    MaximumDilatation,
    ///D2 - Dispersing Agent
    DispersingAgent,
    ///D3 - Drying Agent
    DryingAgent,
    ///D4 - Dry Point
    DryPoint,
    ///D5 - Wear
    Wear,
    ///D6 - Horizontal
    Horizontal,
    ///D7 - Distillation Fraction
    DistillationFraction,
    ///D8 - Vertical
    Vertical,
    ///D9 - Dots per Inch
    DotsPerInch,
    ///DA - Delta Value A
    DeltaValueA,
    ///DAT - Datum Depth
    DatumDepth,
    ///DB - Delta Value B
    DeltaValueB,
    ///DC - Ductile Class
    DuctileClass,
    ///DCT - Dirt Count
    DirtCount,
    ///DD - Depth of Dents
    DepthOfDents,
    ///DE - Defects
    Defects,
    ///DEM - De Minimis Level
    DeMinimisLevel,
    ///DF - Distance Across Flats
    DistanceAcrossFlats,
    ///DG - Direct Current
    DirectCurrent,
    ///DH - Dysprosium
    Dysprosium,
    ///DI - Diameter
    Diameter,
    ///DIL - Dilution Factor; An amount by which a quantity is diluted in order to be read on an instrument scale
    CodeDIL,
    ///DIR - Distillation Range
    DistillationRange,
    ///DIS - Dispersion
    Dispersion,
    ///DJ - Dominant Wave Length
    DominantWaveLength,
    ///DK - Color Bits in Palette
    ColorBitsInPalette,
    ///DL - Delta Value L
    DeltaValueL,
    ///DLC - Dial Count
    DialCount,
    ///DM - Dual Amplitude
    DualAmplitude,
    ///DME - Dichloromethane Extract
    DichloromethaneExtract,
    ///DMF - Distributed Meter Factor
    DistributedMeterFactor,
    ///DN - Density
    Density,
    ///DO - Compressed File Size
    CompressedFileSize,
    ///DOC - Double Olsen Cup
    DoubleOlsenCup,
    ///DP - Depth
    Depth,
    ///DPM - Degree of Polymerization
    DegreeOfPolymerization,
    ///DR - Delta R
    DeltaR,
    ///DRY - Dryness
    Dryness,
    ///DS - Distance Between Points
    DistanceBetweenPoints,
    ///DT - Distance From Base Point
    DistanceFromBasePoint,
    ///DU - Draw Tension
    DrawTension,
    ///DW - Width, Boxcar Door
    CodeDW,
    ///DWP - Dew Point
    DewPoint,
    ///DY - Dyeability
    Dyeability,
    ///E - Estimated New Weight
    EstimatedNewWeight,
    ///E0 - Extraneous Matter
    ExtraneousMatter,
    ///E1 - End
    End,
    ///EA - Elongation
    Elongation,
    ///EB - Edge Burr
    EdgeBurr,
    ///EC - English Coil Dimensions
    EnglishCoilDimensions,
    ///ED - Eddy
    Eddy,
    ///EE - Einsteinium
    Einsteinium,
    ///EF - Europium
    Europium,
    ///EG - Equivalent Temperature
    EquivalentTemperature,
    ///EH - Erbium
    Erbium,
    ///EI - Expansion
    Expansion,
    ///EJ - Electrical Conductivity
    ElectricalConductivity,
    ///EL - Elasticity
    Elasticity,
    ///ELC - Exposure Ceiling: Threshold Limit Value
    ExposureCeilingThresholdLimitValue,
    ///ELE - Elevation
    Elevation,
    ///ELI - Immediately Dangerous to Life and Health
    ImmediatelyDangerousToLifeAndHealth,
    ///ELL - Explosion Limit
    ExplosionLimit,
    ///ELO - Occupational Safety and Health Administration (OSHA) Permissible Exposure Limit
    CodeELO,
    ///ELP - Occupational Safety and Health Administration (OSHA) Permissible Exposure Limit Ceiling
    CodeELP,
    ///ELS - American Conference of Government Industrial Hygienists (ACGIH) Threshold Limit Value: Short-Term Exposure
    CodeELS,
    ///ELT - American Conference of Governmental Industrial Hygienists (ACGIH) Threshold Limit Value: Time Weighted Average
    CodeELT,
    ///ELV - Unshielded Exposure Rate
    UnshieldedExposureRate,
    ///ELW - American Industrial Hygienists Association (AIHA) Work Environment Exposure Level (WEEL)
    CodeELW,
    ///ELX - American Industrial Hygienists Association (AIHA) Work Environment Exposure Level (WEEL): Time Weighted Average
    CodeELX,
    ///EM - Elmendorf Tear
    ElmendorfTear,
    ///EN - Entanglement
    Entanglement,
    ///EP - Exciting Power
    ExcitingPower,
    ///EPL - Expected Product Life
    ExpectedProductLife,
    ///ES - Edge
    Edge,
    ///ET - End Point
    EndPoint,
    ///EVL - Evaporation Loss
    EvaporationLoss,
    ///EVR - Evaporation Rate
    EvaporationRate,
    ///EW - Empty Weight
    EmptyWeight,
    ///EX - Eccentricity
    Eccentricity,
    ///EXH - Exhaust Benzene Emissions
    ExhaustBenzeneEmissions,
    ///EXT - Extractables
    Extractables,
    ///F - Deficit Weight
    DeficitWeight,
    ///F1 - Fire Point
    FirePoint,
    ///F2 - Radiated Power
    RadiatedPower,
    ///F3 - Output Power (Peak Envelope)
    CodeF3,
    ///F4 - Height above Average Terrain
    HeightAboveAverageTerrain,
    ///F5 - Ground Elevation
    GroundElevation,
    ///F6 - Height to Tip
    HeightToTip,
    ///F7 - Radius from a Location
    RadiusFromALocation,
    ///F8 - Radius from Coordinates
    RadiusFromCoordinates,
    ///F9 - Operating Hours
    OperatingHours,
    ///FA - Fluorine
    Fluorine,
    ///FB - Flare
    Flare,
    ///FBP - Acetate Break Point
    AcetateBreakPoint,
    ///FC - Short Cycle Flatness
    ShortCycleFlatness,
    ///FD - Frequency of Operation
    FrequencyOfOperation,
    ///FE - Fermium
    Fermium,
    ///FF - Francium
    Francium,
    ///FG - Freezing Point
    FreezingPoint,
    ///FH - Finish
    Finish,
    ///FI - Filament Count
    FilamentCount,
    ///FIL - Filter Number
    FilterNumber,
    ///FIN - Fineness
    Fineness,
    ///FIT - Filterability
    Filterability,
    ///FJ - Face Width
    FaceWidth,
    ///FK - Fluid Consistency
    FluidConsistency,
    ///FL - Longitudinal Flatness
    LongitudinalFlatness,
    ///FLD - Fluid Point
    FluidPoint,
    ///FLN - Fiber Length
    FiberLength,
    ///FLP - Fluid Level Above Pump
    FluidLevelAbovePump,
    ///FLT - Fluting
    Fluting,
    ///FLV - Flavor Threshold
    FlavorThreshold,
    ///FML - Flammability Limits
    FlammabilityLimits,
    ///FMZ - Flammability
    Flammability,
    ///FN - Flatness
    Flatness,
    ///FNL - Fines Retained on Screen
    FinesRetainedOnScreen,
    ///FNS - Fines
    Fines,
    ///FOA - Foam
    Foam,
    ///FOH - Front Over-Hang of Vehicle
    FrontOverHangOfVehicle,
    ///FOI - Fouling Index
    FoulingIndex,
    ///FOR - Foreign Matter
    ForeignMatter,
    ///FP - Flashpoint
    Flashpoint,
    ///FPV - Acetate Plugging Value
    AcetatePluggingValue,
    ///FQ - Frequency
    Frequency,
    ///FR - Flow Rate
    FlowRate,
    ///FS - Fold Strength
    FoldStrength,
    ///FSI - Free Swelling Index
    FreeSwellingIndex,
    ///FT - Flange Thickness
    FlangeThickness,
    ///FU - Azimuth
    Azimuth,
    ///FUD - Full Load
    FullLoad,
    ///FV - Transverse Flatness
    TransverseFlatness,
    ///FW - Flange Width
    FlangeWidth,
    ///FX - Filler
    Filler,
    ///FY - Beam
    Beam,
    ///FZ - Output Power (Mean RF)
    CodeFZ,
    ///G - Gross Weight
    GrossWeight,
    ///G1 - Gutter
    Gutter,
    ///G2 - Grain Size
    GrainSize,
    ///G3 - Tilt
    Tilt,
    ///G4 - G-Force
    GForce,
    ///GA - Grain
    Grain,
    ///GB - Guided Bends Root
    GuidedBendsRoot,
    ///GC - Gadolinium
    Gadolinium,
    ///GD - Gold
    Gold,
    ///GE - Guided Bends Face
    GuidedBendsFace,
    ///GEL - Gel
    Gel,
    ///GF - Guided Bends Side
    GuidedBendsSide,
    ///GG - Gauge
    Gauge,
    ///GGR - Gas Gravity
    GasGravity,
    ///GH - Gallium
    Gallium,
    ///GI - Grit, Brushed
    CodeGI,
    ///GIR - Gas Injection Test Rate
    GasInjectionTestRate,
    ///GJ - Grit, Unbrushed
    CodeGJ,
    ///GK - Tinting Strength
    TintingStrength,
    ///GL - Gloss
    Gloss,
    ///GLE - Ground Level Elevation
    GroundLevelElevation,
    ///GM - Emulsion Gloss
    EmulsionGloss,
    ///GN - Gross to Net Conversion Factor
    GrossToNetConversionFactor,
    ///GO - Particle End Point
    ParticleEndPoint,
    ///GOR - Gas-to-Oil Ratio
    GasToOilRatio,
    ///GP - Group Package Separation
    GroupPackageSeparation,
    ///GQ - Scattered Particles
    ScatteredParticles,
    ///GR - Gravity
    Gravity,
    ///GRA - Gas Test Rate
    GasTestRate,
    ///GRI - Graininess
    Graininess,
    ///GS - Carbon Black Undertone
    CarbonBlackUndertone,
    ///GT - Glass Transition Temperature
    GlassTransitionTemperature,
    ///GW - Gross Weight, Maximum
    CodeGW,
    ///H - Ground Water Reference Point
    GroundWaterReferencePoint,
    ///H1 - Heavies
    Heavies,
    ///H2O - Water Volume
    WaterVolume,
    ///H8 - Net Explosive Weight
    NetExplosiveWeight,
    ///H9 - Recommended Exposure Limit
    RecommendedExposureLimit,
    ///HA - Hydro Pressure
    HydroPressure,
    ///HAR - Hardening Rate
    HardeningRate,
    ///HAZ - Haze
    Haze,
    ///HB - Heavy Aluminas
    HeavyAluminas,
    ///HC - Helium
    Helium,
    ///HCG - Horizontal Center of Gravity
    HorizontalCenterOfGravity,
    ///HCH - Distance to Endpoint
    DistanceToEndpoint,
    ///HCI - Emission Factor
    EmissionFactor,
    ///HCJ - Heat Content
    HeatContent,
    ///HCK - Pollutant Emission
    PollutantEmission,
    ///HCL - Population
    Population,
    ///HCM - Release Duration
    ReleaseDuration,
    ///HCN - Released Quantity
    ReleasedQuantity,
    ///HCO - Release Rate
    ReleaseRate,
    ///HCP - Sulfur Content
    SulfurContent,
    ///HCQ - Wind Speed
    WindSpeed,
    ///HCR - Acute Toxicity
    AcuteToxicity,
    ///HCS - Chronic Toxicity
    ChronicToxicity,
    ///HCT - Discharges
    Discharges,
    ///HCU - Effectiveness
    Effectiveness,
    ///HCV - Fertilization
    Fertilization,
    ///HCW - Samples in Compliance
    SamplesInCompliance,
    ///HCX - Toxicity
    Toxicity,
    ///HD - High Propagation Delay Time
    HighPropagationDelayTime,
    ///HE - Heavy Silicates
    HeavySilicates,
    ///HF - Hardness
    Hardness,
    ///HG - Heavy Sulfides
    HeavySulfides,
    ///HH - Heavy Globular Oxides
    HeavyGlobularOxides,
    ///HHW - Hemispherical Point
    HemisphericalPoint,
    ///HI - Hafnium
    Hafnium,
    ///HIB - High Boilers
    HighBoilers,
    ///HJ - Holmium
    Holmium,
    ///HK - Human Factors
    HumanFactors,
    ///HL - Heat Loss
    HeatLoss,
    ///HM - Height, Maximum
    CodeHM,
    ///HO - Holes
    Holes,
    ///HOC - Heat of Combustion
    HeatOfCombustion,
    ///HP - Height of Tread Plate Pattern
    HeightOfTreadPlatePattern,
    ///HR - Height of Runners
    HeightOfRunners,
    ///HT - Height
    Height,
    ///HTE - Heat Equivalency
    HeatEquivalency,
    ///HVM - Heavy Metals
    HeavyMetals,
    ///HWS - Softening Point
    SofteningPoint,
    ///HYD - Hydroxyl Number
    HydroxylNumber,
    ///HZ - Hazepoint
    Hazepoint,
    ///HZC - Hazardous Component
    HazardousComponent,
    ///I - Heat Input
    HeatInput,
    ///IA - Imperfections - Thicks
    ImperfectionsThicks,
    ///IB - Impact Energy
    ImpactEnergy,
    ///IC - Incremental Distance
    IncrementalDistance,
    ///ID - Inside Diameter
    InsideDiameter,
    ///IDE - Identification
    Identification,
    ///IE - Imperfections - Thins
    ImperfectionsThins,
    ///IF - Input Low Voltage
    InputLowVoltage,
    ///IG - Granulated Ingot Size
    GranulatedIngotSize,
    ///IGA - Autoignition Temperature
    AutoignitionTemperature,
    ///IGR - Input Gas Rate
    InputGasRate,
    ///IH - Input Current
    InputCurrent,
    ///IHV - Inherent Viscosity
    InherentViscosity,
    ///II - Input Setup Time
    InputSetupTime,
    ///IJ - Input Hold Time
    InputHoldTime,
    ///IK - Indium
    Indium,
    ///IL - Imperfections - Neps
    ImperfectionsNeps,
    ///IM - Iridium
    Iridium,
    ///IMA - Impact Adhesion
    ImpactAdhesion,
    ///IMP - Impurities
    Impurities,
    ///IN - Input Low Current
    InputLowCurrent,
    ///IND - Initial Deformation Point
    InitialDeformationPoint,
    ///INS - Insolubles
    Insolubles,
    ///IO - Input High Current
    InputHighCurrent,
    ///IP - Input Leakage Current
    InputLeakageCurrent,
    ///IPI - Primary Irritation Index
    PrimaryIrritationIndex,
    ///IQ - Input Clamp Diode Voltage
    InputClampDiodeVoltage,
    ///IR - Ideal Diameter
    IdealDiameter,
    ///IRA - Inclusion Rating
    InclusionRating,
    ///IS - Input High Voltage
    InputHighVoltage,
    ///IT - Interrupt Pulse Period
    InterruptPulsePeriod,
    ///ITD - Integrated Differential
    IntegratedDifferential,
    ///IU - Interrupt Setup Time
    InterruptSetupTime,
    ///IV - Iodine
    Iodine,
    ///IW - Incrustation Factor
    IncrustationFactor,
    ///IX - Inside Diameter, Minimum
    CodeIX,
    ///IXD - Index Differential
    IndexDifferential,
    ///IY - Induction
    Induction,
    ///IZ - Ingot Pound Size
    IngotPoundSize,
    ///JA - Junction Temperature
    JunctionTemperature,
    ///JOM - Jominy Hardenability
    JominyHardenability,
    ///KA - Knoop
    Knoop,
    ///KB - Krypton
    Krypton,
    ///KN - K&N Holdout
    CodeKN,
    ///KVL - Strength Coefficient
    StrengthCoefficient,
    ///L - Legal Weight
    LegalWeight,
    ///L0 - Leaf Grade
    LeafGrade,
    ///L1 - Lights
    Lights,
    ///LA - Lanthanum
    Lanthanum,
    ///LAI - Langlier Index
    LanglierIndex,
    ///LB - Lithium
    Lithium,
    ///LC - Low Propagation Delay Time
    LowPropagationDelayTime,
    ///LC5 - Lethal Concentration, 50% ("LC-50")
    CodeLC5,
    ///LCG - Longitudinal Center of Gravity
    LongitudinalCenterOfGravity,
    ///LD - Lawrencium
    Lawrencium,
    ///LD5 - Lethal Dose, 50% ("LD-50")
    CodeLD5,
    ///LDH - Limited Dome Height (LDH)
    CodeLDH,
    ///LE - Lutetium
    Lutetium,
    ///LEF - Leaf
    Leaf,
    ///LF - Long Fibers
    LongFibers,
    ///LG - Leg
    Leg,
    ///LIR - Liquid Injection Test Rate
    LiquidInjectionTestRate,
    ///LIV - Limit Intrinsic Viscosity
    LimitIntrinsicViscosity,
    ///LL - Long Leg
    LongLeg,
    ///LLD - Light Load
    LightLoad,
    ///LM - Length, Maximum
    CodeLM,
    ///LN - Length
    Length,
    ///LO - Long Length
    LongLength,
    ///LOI - Loss on Ignition
    LossOnIgnition,
    ///LOS - Loss on Drying
    LossOnDrying,
    ///LOW - Low Boilers
    LowBoilers,
    ///LP - Camber of Pattern Line
    CamberOfPatternLine,
    ///LPG - Liquefied Petroleum Gas Factor
    LiquefiedPetroleumGasFactor,
    ///LPL - Labeled Product Life
    LabeledProductLife,
    ///LPR - Line Pressure
    LinePressure,
    ///LS - Short Leg
    ShortLeg,
    ///LSA - Lock Seam Adhesion
    LockSeamAdhesion,
    ///LSK - Length Shrinkage
    LengthShrinkage,
    ///LSS - Linear Sheet Swelling
    LinearSheetSwelling,
    ///LT - Lengthwise Spacing
    LengthwiseSpacing,
    ///LTD - Liner Top Depth
    LinerTopDepth,
    ///LW - Long Width
    LongWidth,
    ///M - Minimum Weight (for Weight)
    CodeM,
    ///M1 - Melt Range
    MeltRange,
    ///M2 - Maximum Differential Pressure
    MaximumDifferentialPressure,
    ///M3 - Maximum Static Pressure
    MaximumStaticPressure,
    ///M4 - Area
    Area,
    ///M5 - Minimum Speed
    MinimumSpeed,
    ///M6 - Maximum Speed
    MaximumSpeed,
    ///MA - Mean Average
    MeanAverage,
    ///MAT - Maturity
    Maturity,
    ///MB - Mendelevium
    Mendelevium,
    ///MC - Min./Max Cuttable Width
    MinMaxCuttableWidth,
    ///MCN - Magnetic Contamination
    MagneticContamination,
    ///MD - Measurement Voltage
    MeasurementVoltage,
    ///MDL - Method Detection Limit; Minimum concentration of a substance that can be measured and reported with 99% confidence that analyte concentration is greater than zero
    CodeMDL,
    ///ME - Maximum Input Low Voltage
    MaximumInputLowVoltage,
    ///MEA - Media Depth
    MediaDepth,
    ///MEF - Meter Factor
    MeterFactor,
    ///MEL - Melt Time
    MeltTime,
    ///MEP - Meat Protein
    MeatProtein,
    ///MER - Efficient Rate-Reservoir
    EfficientRateReservoir,
    ///MF - Minimum Input High Voltage
    MinimumInputHighVoltage,
    ///MG - Mercury
    Mercury,
    ///MH - Melting Point
    MeltingPoint,
    ///MHI - Highest Torque
    HighestTorque,
    ///MI - Minimum
    Minimum,
    ///MIC - Micronaire
    Micronaire,
    ///MIL - Milk Fat
    MilkFat,
    ///MJ - Major Section (Stepped)
    CodeMJ,
    ///MK - Microseperometer (MSEP)
    CodeMK,
    ///MM - Management
    Management,
    ///MN - Minimum Average
    MinimumAverage,
    ///MO - Mottles
    Mottles,
    ///MOI - Moisture
    Moisture,
    ///MOR - Mortality
    Mortality,
    ///MP - Mullen Pop
    MullenPop,
    ///MPR - Production Rate-Well
    ProductionRateWell,
    ///MPT - Many Press Test
    ManyPressTest,
    ///MQ - MCQuaid
    McQuaid,
    ///MR - Module R (R Bar)
    CodeMR,
    ///MS - Minor Section (Stepped)
    CodeMS,
    ///MT - Moisture Content
    MoistureContent,
    ///MTD - Maximum Total Depth
    MaximumTotalDepth,
    ///MU - Multiplier
    Multiplier,
    ///MUL - Mullen
    Mullen,
    ///MV - Maximum Average
    MaximumAverage,
    ///MW - Molecular Weight
    MolecularWeight,
    ///MX - Maximum
    Maximum,
    ///MY - Magnetizing Field
    MagnetizingField,
    ///N - Actual Net Weight
    ActualNetWeight,
    ///NA - Number per Package
    NumberPerPackage,
    ///NB - Number per Bundle
    NumberPerBundle,
    ///NC - Number per Coil Group
    NumberPerCoilGroup,
    ///ND - Neodymium
    Neodymium,
    ///NEU - Neutralization Number
    NeutralizationNumber,
    ///NF - Neon
    Neon,
    ///NG - Nobelium
    Nobelium,
    ///NH - Number of Items per Package Label
    NumberOfItemsPerPackageLabel,
    ///NI - Number of Splices per Package Label
    NumberOfSplicesPerPackageLabel,
    ///NIL - Nil Ductility Test
    NilDuctilityTest,
    ///NJ - Number of Sheets per Package Label
    NumberOfSheetsPerPackageLabel,
    ///NK - Nesting Factor
    NestingFactor,
    ///NL - Number per Lift
    NumberPerLift,
    ///NM - Number Pkgs. per Master Pack
    NumberPkgsPerMasterPack,
    ///NNW - Net Net Weight
    NetNetWeight,
    ///NO - Nominal (Target, Aim)
    CodeNO,
    ///NOC - Number of Cosigners
    NumberOfCosigners,
    ///NON - Non-Volatile Matter
    NonVolatileMatter,
    ///NOR - Number of References
    NumberOfReferences,
    ///NOX - NOx Emissions Performance
    NOxEmissionsPerformance,
    ///NP - Percent of Specified
    PercentOfSpecified,
    ///NS - Number per Skid
    NumberPerSkid,
    ///NU - Number per Unit
    NumberPerUnit,
    ///NV - N Value
    NValue,
    ///O - Excess Weight Over Maximum
    ExcessWeightOverMaximum,
    ///O1 - Orifice - Inside Diameter
    OrificeInsideDiameter,
    ///OA - Offset
    Offset,
    ///OAP - Observed American Petroleum Institute Gravity
    ObservedAmericanPetroleumInstituteGravity,
    ///OB - Osmium
    Osmium,
    ///OBT - Observed Temperature
    ObservedTemperature,
    ///OC - Output Low Voltage
    OutputLowVoltage,
    ///OCG - Oil/Condensate Gravity
    OilCondensateGravity,
    ///OCR - Oil/Condensate Test Rate
    OilCondensateTestRate,
    ///OD - Outside Diameter
    OutsideDiameter,
    ///ODR - Odor
    Odor,
    ///OE - Output Low Current
    OutputLowCurrent,
    ///OF - Output High Voltage
    OutputHighVoltage,
    ///OG - Output High Current
    OutputHighCurrent,
    ///OH - Overhead Height, Receiving Door
    CodeOH,
    ///OI - Output Off Current Low
    OutputOffCurrentLow,
    ///OIL - Oil
    Oil,
    ///OJ - Output Off Current High
    OutputOffCurrentHigh,
    ///OK - Output Short-Circuit Current
    OutputShortCircuitCurrent,
    ///OL - Output Disable Time from Low Level of a 3-State Output
    OutputDisableTimeFromLowLevelOfA3StateOutput,
    ///OLE - Olefins
    Olefins,
    ///OM - Outside Diameter, Maximum
    CodeOM,
    ///ON - Output Disable Time from High Level of a 3-State Output
    OutputDisableTimeFromHighLevelOfA3StateOutput,
    ///OO - Output Enable Time from Low Level of a 3-State Output
    OutputEnableTimeFromLowLevelOfA3StateOutput,
    ///OP - Openness
    Openness,
    ///OQ - Output Enable Time from High Level of a 3-State Output
    OutputEnableTimeFromHighLevelOfA3StateOutput,
    ///OR - Distance Between Outside Runners
    DistanceBetweenOutsideRunners,
    ///ORC - Organic Carbon
    OrganicCarbon,
    ///OS - Open Circuits
    OpenCircuits,
    ///OT - Output Delay Time
    OutputDelayTime,
    ///OTE - Others Each
    OthersEach,
    ///OTH - Odor Threshold
    OdorThreshold,
    ///OTT - Others Total
    OthersTotal,
    ///OV - Opacity
    Opacity,
    ///OW - Overall Width
    OverallWidth,
    ///OX - Ownership Share
    OwnershipShare,
    ///OXI - Oxidizable Substance
    OxidizableSubstance,
    ///OXS - Oxidizing Substance
    OxidizingSubstance,
    ///OY - Operating Weight
    OperatingWeight,
    ///P1 - Price
    Price,
    ///PA - Package Separation
    PackageSeparation,
    ///PAR - Particle Size
    ParticleSize,
    ///PB - Pressure
    Pressure,
    ///PBD - Plug Back Total Depth
    PlugBackTotalDepth,
    ///PC - Per Hundred Linear Yards
    PerHundredLinearYards,
    ///PD - Platinum
    Platinum,
    ///PDE - Casing/Liner Tubing Depth
    CasingLinerTubingDepth,
    ///PDG - Pump Depth from Ground
    PumpDepthFromGround,
    ///PE - Potassium
    Potassium,
    ///PER - Magnetic Permeability
    MagneticPermeability,
    ///PF - Promethium
    Promethium,
    ///PFO - Perforation Feet Open
    PerforationFeetOpen,
    ///PG - Polonium
    Polonium,
    ///PH - Pulse Setup Time
    PulseSetupTime,
    ///PHA - pH
    PH,
    ///PHW - Hardwood Fiber
    HardwoodFiber,
    ///PI - Pulse Hold Time
    PulseHoldTime,
    ///PIC - Pick Off
    PickOff,
    ///PJ - Pulse Width
    PulseWidth,
    ///PK - Pulse Recovery Time
    PulseRecoveryTime,
    ///PL - Percent Defective
    PercentDefective,
    ///PM - Practice
    Practice,
    ///PN - Palladium
    Palladium,
    ///PO - Percent of Order (-, +)
    CodePO,
    ///POC - Completion
    Completion,
    ///POD - Physical Description - Outer Diameter
    PhysicalDescriptionOuterDiameter,
    ///POP - Pour Point
    PourPoint,
    ///PP - Powder/Paste Package Size
    PowderPastePackageSize,
    ///PPS - Proprietary Shade
    ProprietaryShade,
    ///PQ - Plutonium
    Plutonium,
    ///PQL - Practical Quantitation Limit; Lowest concentration of a substance which can be consistently determined within +/- 20% of the true concentration by 75% of the laboratories tested in a performance evaluation study
    CodePQL,
    ///PR - Praseodymium
    Praseodymium,
    ///PRA - Proportion Alive
    ProportionAlive,
    ///PRE - Prior Experience
    PriorExperience,
    ///PRF - Pressure Factor
    PressureFactor,
    ///PRI - Product Index
    ProductIndex,
    ///PRL - Product Level
    ProductLevel,
    ///PRN - Proportion Normal
    ProportionNormal,
    ///PRO - Processability
    Processability,
    ///PRQ - Product Reportable Quantity
    ProductReportableQuantity,
    ///PRY - Porosity
    Porosity,
    ///PRZ - Proportion Fertilized
    ProportionFertilized,
    ///PS - Protactinium
    Protactinium,
    ///PSA - Percent Solution Actual
    PercentSolutionActual,
    ///PSP - Past Performance
    PastPerformance,
    ///PSW - Softwood Fiber
    SoftwoodFiber,
    ///PT - Pits
    Pits,
    ///PU - Pressure Base
    PressureBase,
    ///PV - Picks
    Picks,
    ///PW - Purchased Width
    PurchasedWidth,
    ///PWA - Processed Waste
    ProcessedWaste,
    ///PWE - Physical Description - Weight
    PhysicalDescriptionWeight,
    ///PWF - Power Factor
    PowerFactor,
    ///PX - Purity
    Purity,
    ///PY - Percent of Water
    PercentOfWater,
    ///PZ - Pipe Size Nominal
    PipeSizeNominal,
    ///Q - Volatile Organic Compounds Plus Water
    VolatileOrganicCompoundsPlusWater,
    ///QA - Quality Index
    QualityIndex,
    ///QB - Quantity or Loading Average
    QuantityOrLoadingAverage,
    ///QC - Quantity or Loading Maximum
    QuantityOrLoadingMaximum,
    ///QD - Quality or Concentration Average
    QualityOrConcentrationAverage,
    ///QE - Quality or Concentration Minimum
    QualityOrConcentrationMinimum,
    ///QF - Quality or Concentration Maximum
    QualityOrConcentrationMaximum,
    ///QG - Duration
    Duration,
    ///QI - Abundance
    Abundance,
    ///QJ - Biomass
    Biomass,
    ///QK - Size Class
    SizeClass,
    ///QL - Quality
    Quality,
    ///QUR - Reportable Quantity
    ReportableQuantity,
    ///R - Per Unit Dunnage
    PerUnitDunnage,
    ///R1 - Hemoglobin
    Hemoglobin,
    ///R2 - Hematocrit
    Hematocrit,
    ///R3 - Epoetin Starting Dosage
    EpoetinStartingDosage,
    ///R4 - Creatinine
    Creatinine,
    ///R7 - Speed
    Speed,
    ///R8 - Speed Limit
    SpeedLimit,
    ///R10 - Relative Fraction of Pure Long-Chain Cellulose
    RelativeFractionOfPureLongChainCellulose,
    ///R18 - Relative Fraction of Total Cellulose
    RelativeFractionOfTotalCellulose,
    ///RA - Relative Humidity
    RelativeHumidity,
    ///RAD - Radius
    Radius,
    ///RAF - Roof Adjustment Factor
    RoofAdjustmentFactor,
    ///RB - Range Value
    RangeValue,
    ///RC - Radius of Corner
    RadiusOfCorner,
    ///RD - Readpoint
    Readpoint,
    ///RE - Ream Weight
    ReamWeight,
    ///REA - Reactivity
    Reactivity,
    ///RED - Reducing Substance
    ReducingSubstance,
    ///REF - Refining
    Refining,
    ///REI - Refractive Index
    RefractiveIndex,
    ///REL - Reflectance
    Reflectance,
    ///RES - Resistance
    Resistance,
    ///RF - Resistivity
    Resistivity,
    ///RG - Radium
    Radium,
    ///RH - Rhenium
    Rhenium,
    ///RI - Rubidium
    Rubidium,
    ///RJ - Rockwell-C
    RockwellC,
    ///RK - Rockwell-B
    RockwellB,
    ///RL - Reduction Ration
    ReductionRation,
    ///RM - RMS Range (Side 1)
    CodeRM,
    ///RN - Required Interrupt Release
    RequiredInterruptRelease,
    ///RO - Reset Pulse Width
    ResetPulseWidth,
    ///ROH - Rear Over-Hang of Vehicle
    RearOverHangOfVehicle,
    ///ROX - Oxygen from a Renewable Oxygenate
    OxygenFromARenewableOxygenate,
    ///RP - Reduction of Area
    ReductionOfArea,
    ///RQ - Radon
    Radon,
    ///RR - Reduction Ratio
    ReductionRatio,
    ///RS - RMS Range (Side 2)
    CodeRS,
    ///RSZ - Roll Size
    RollSize,
    ///RT - Rounds Ammunition/Military
    RoundsAmmunitionMilitary,
    ///RTB - Reporting Temperature Base
    ReportingTemperatureBase,
    ///RU - Rhodium
    Rhodium,
    ///RUD - Usage Deviation (Applies to Kilowatt Hours, Kilowatt Demand and Reactive Demand)
    CodeRUD,
    ///RV - Ruthenium
    Ruthenium,
    ///RVP - Reid Vapor Pressure
    ReidVaporPressure,
    ///RW - Rolling Width
    RollingWidth,
    ///RX - Ridges
    Ridges,
    ///RY - Ratio
    Ratio,
    ///S - State Weight
    StateWeight,
    ///S1 - Smoothness
    Smoothness,
    ///S2 - Selvedge on Beam
    SelvedgeOnBeam,
    ///S3 - Sheffield Smoothness
    SheffieldSmoothness,
    ///S4 - Surface Strength
    SurfaceStrength,
    ///S5 - Stiffness
    Stiffness,
    ///S6 - Saturation
    Saturation,
    ///S7 - Sediment
    Sediment,
    ///S8 - Solubility
    Solubility,
    ///S9 - Site Atmospheric Pressure
    SiteAtmosphericPressure,
    ///S10 - Pulp Impurities
    PulpImpurities,
    ///S12 - Start
    Start,
    ///S18 - Hemicellulose
    Hemicellulose,
    ///SA - Sort Code CIE LAB
    SortCodeCieLab,
    ///SAL - Salinity; Salt level in a sample of seawater
    CodeSAL,
    ///SAP - Saponification Number
    SaponificationNumber,
    ///SB - Sort Code CMC
    SortCodeCmc,
    ///SC - Schedule Number (Pipe Size)
    CodeSC,
    ///SCH - Schedule
    Schedule,
    ///SD - Strength
    Strength,
    ///SE - Selvage Left
    SelvageLeft,
    ///SEV - Severity
    Severity,
    ///SF - Samarium
    Samarium,
    ///SFC - Short Fiber Content
    ShortFiberContent,
    ///SG - Slit Width
    SlitWidth,
    ///SH - Strontium
    Strontium,
    ///SHA - Shelf Life
    ShelfLife,
    ///SI - Supply Current
    SupplyCurrent,
    ///SIL - Silica (Silicon Dioxide)
    CodeSIL,
    ///SJ - Short Circuits
    ShortCircuits,
    ///SK - Shrinkage
    Shrinkage,
    ///SL - Short Length
    ShortLength,
    ///SLD - Solderability
    Solderability,
    ///SLI - Slagging Index
    SlaggingIndex,
    ///SM - Shear
    Shear,
    ///SMB - SAM-B Rating
    SamBRating,
    ///SMD - SAM-D Rating
    SamDRating,
    ///SN - Stain
    Stain,
    ///SO - Sort Code CIE LCH
    SortCodeCieLch,
    ///SOD - Solids
    Solids,
    ///SOF - Softening Range
    SofteningRange,
    ///SP - Splinter Count
    SplinterCount,
    ///SPG - Specific Gravity
    SpecificGravity,
    ///SPH - Sphere
    Sphere,
    ///SPR - Separator Pressure
    SeparatorPressure,
    ///SPS - Static Pressure
    StaticPressure,
    ///SQ - Shipped Quantity
    ShippedQuantity,
    ///SR - Selvage Right
    SelvageRight,
    ///SS - Silver
    Silver,
    ///ST - Stop Recovery Startup Time
    StopRecoveryStartupTime,
    ///STA - Stability
    Stability,
    ///STL - Short Term Exposure Limit
    ShortTermExposureLimit,
    ///STP - Staple
    Staple,
    ///SU - Shipped Units
    ShippedUnits,
    ///SUM - Suspended Matter
    SuspendedMatter,
    ///SUR - Surface Roughness
    SurfaceRoughness,
    ///SUT - Surface Tension
    SurfaceTension,
    ///SV - Scandium
    Scandium,
    ///SVL - Survival
    Survival,
    ///SW - Short Width
    ShortWidth,
    ///SX - Sodium
    Sodium,
    ///SXX - S10 Minus S18 Value
    S10MinusS18Value,
    ///SY - Service Interrupt Duration
    ServiceInterruptDuration,
    ///SZ - Skid Height
    SkidHeight,
    ///T - Tare Weight
    TareWeight,
    ///T1 - Tire Pressure
    TirePressure,
    ///T2 - Tube - Inside Diameter
    TubeInsideDiameter,
    ///T3 - Technical
    Technical,
    ///T4 - Single End Break
    SingleEndBreak,
    ///T5 - Skein Break
    SkeinBreak,
    ///T50 - T50
    T50,
    ///T90 - T90
    T90,
    ///TA - Thickness Heavy End (Tapered/Stepped)
    CodeTA,
    ///TAS - Taste
    Taste,
    ///TB - Thickness Small End (Tapered/Stepped)
    CodeTB,
    ///TC - Temperature
    Temperature,
    ///TCL - Tire Tread Contact Length
    TireTreadContactLength,
    ///TD - Thin Aluminas
    ThinAluminas,
    ///TDP - Perforation Top Depth
    PerforationTopDepth,
    ///TE - Tenacity
    Tenacity,
    ///TEE - Autodecomposition Temperature
    AutodecompositionTemperature,
    ///TES - Storage Temperature
    StorageTemperature,
    ///TEX - Texture
    Texture,
    ///TF - Tensile
    Tensile,
    ///TG - Thin Sulfides
    ThinSulfides,
    ///TH - Thickness
    Thickness,
    ///TI - Thin Silicates
    ThinSilicates,
    ///TJ - Total Supply Current
    TotalSupplyCurrent,
    ///TK - Timer Pulse Width
    TimerPulseWidth,
    ///TL - Tapered/Stepped Length Type
    TaperedSteppedLengthType,
    ///TM - Length Type: Multiples
    LengthTypeMultiples,
    ///TN - Timer Period
    TimerPeriod,
    ///TO - Terbium
    Terbium,
    ///TOA - Aquatic Toxicity
    AquaticToxicity,
    ///TOR - Torque
    Torque,
    ///TOX - Toxic Emissions Performance
    ToxicEmissionsPerformance,
    ///TP - Thorium
    Thorium,
    ///TPF - Temperature Factor
    TemperatureFactor,
    ///TPL - Tubing Pressure - Flowing
    TubingPressureFlowing,
    ///TPQ - Threshold Planning Quantity
    ThresholdPlanningQuantity,
    ///TPS - Tubing Pressure - Shutin
    TubingPressureShutin,
    ///TQ - Thin Globular Oxides
    ThinGlobularOxides,
    ///TR - Length Type: Random
    LengthTypeRandom,
    ///TRA - Trash Area
    TrashArea,
    ///TRC - Trash Count
    TrashCount,
    ///TRD - Tire Diameter
    TireDiameter,
    ///TRN - Transmittance
    Transmittance,
    ///TRS - Transmissivity; Measure of the quantity of light that passes through a given volume of seawater; also used to measure turbidity and to estimate plant growing zones in the ocean
    CodeTRS,
    ///TRT - Transmissivity Pathlength; The length of the path taken to arrive at transmissivity measurements
    CodeTRT,
    ///TS - Length Type: Specific
    LengthTypeSpecific,
    ///TSZ - Trim Size
    TrimSize,
    ///TT - Time
    Time,
    ///TTL - Trailer Tongue Length
    TrailerTongueLength,
    ///TU - Technetium
    Technetium,
    ///TUR - Turbidity
    Turbidity,
    ///TV - Thallium
    Thallium,
    ///TVD - Maximum True Vertical Depth
    MaximumTrueVerticalDepth,
    ///TW - Top
    Top,
    ///TWD - Tire Width
    TireWidth,
    ///TX - Thulium
    Thulium,
    ///TY - Tear Strength
    TearStrength,
    ///U - Weight per Unit
    WeightPerUnit,
    ///UA - Uranium
    Uranium,
    ///UCB - Cube
    Cube,
    ///UG - Usage
    Usage,
    ///UNA - Unipunch Adhesion
    UnipunchAdhesion,
    ///UNI - Uniformity
    Uniformity,
    ///UNK - Unknowns
    Unknowns,
    ///V - Oxygenation Level
    OxygenationLevel,
    ///VAD - Vapor Density
    VaporDensity,
    ///VAP - Vapor Pressure
    VaporPressure,
    ///VBA - V-Bend Adhesion
    VBendAdhesion,
    ///VCG - Vertical Center of Gravity
    VerticalCenterOfGravity,
    ///VH - Height, Van Door
    CodeVH,
    ///VIN - Vinyl
    Vinyl,
    ///VIS - Viscosity
    Viscosity,
    ///VO - Voltage
    Voltage,
    ///VOC - VOC Emissions Performance
    VocEmissionsPerformance,
    ///VOL - Volume
    Volume,
    ///VOT - Volatiles
    Volatiles,
    ///VOV - Volatiles by Volume
    VolatilesByVolume,
    ///VOW - Volatiles by Weight
    VolatilesByWeight,
    ///VSO - Volume Split to Others
    VolumeSplitToOthers,
    ///VW - Width, Van Door
    CodeVW,
    ///VWT - Volume Weight
    VolumeWeight,
    ///W - Reformulated Fuel Level
    ReformulatedFuelLevel,
    ///WA - Weight per Unit of Area
    WeightPerUnitOfArea,
    ///WB - Web
    Web,
    ///WC - Web Depth/Height
    WebDepthHeight,
    ///WD - Width
    Width,
    ///WDE - Water Depth
    WaterDepth,
    ///WE - Wolfram
    Wolfram,
    ///WEL - Weight Loss
    WeightLoss,
    ///WF - Wait Recovery Startup Time
    WaitRecoveryStartupTime,
    ///WH - Whiteness
    Whiteness,
    ///WI - Winding Loss
    WindingLoss,
    ///WL - Wall Thickness
    WallThickness,
    ///WM - Width, Maximum
    CodeWM,
    ///WOD - Water-Oil Distribution Coefficient
    WaterOilDistributionCoefficient,
    ///WPF - Wellhead Pressure-Flowing
    WellheadPressureFlowing,
    ///WPL - Water/Product Level
    WaterProductLevel,
    ///WPS - Wellhead Pressure Shutin
    WellheadPressureShutin,
    ///WR - Wrinkles
    Wrinkles,
    ///WRA - Water Test Rate
    WaterTestRate,
    ///WSK - Width Shrinkage
    WidthShrinkage,
    ///WT - Weight
    Weight,
    ///WTB - Water/Tank Bottom Level
    WaterTankBottomLevel,
    ///WU - Weight per Unit of Length
    WeightPerUnitOfLength,
    ///WX - Wax Pick
    WaxPick,
    ///X - Maximum Weight (for Rate)
    CodeX,
    ///XA - Xenon
    Xenon,
    ///XH - Side Height, Flat Bed With Removable Sides
    CodeXH,
    ///XP - Specified
    Specified,
    ///XQ - Squareness
    Squareness,
    ///XZ - Spool Size
    SpoolSize,
    ///YA - Yttrium
    Yttrium,
    ///YB - Yield
    Yield,
    ///YC - Ytterbium
    Ytterbium,
    ///YD - Yarn Count
    YarnCount,
    ///YPE - Yield Point Elongation
    YieldPointElongation,
    ///ZAL - Aluminum
    Aluminum,
    ///ZAS - Arsenic
    Arsenic,
    ///ZB - Boron
    Boron,
    ///ZBI - Bismuth
    Bismuth,
    ///ZBT - N-Butane
    NButane,
    ///ZBZ - Benzene
    Benzene,
    ///ZC - Carbon
    Carbon,
    ///ZCA - Calcium
    Calcium,
    ///ZCB - Columbium
    Columbium,
    ///ZCD - Carbon Dioxide
    CarbonDioxide,
    ///ZCE - Cerium
    Cerium,
    ///ZCM - Carbon Monoxide
    CarbonMonoxide,
    ///ZCO - Cobalt
    Cobalt,
    ///ZCR - Chromium
    Chromium,
    ///ZCU - Copper
    Copper,
    ///ZD - Load Factor
    LoadFactor,
    ///ZET - Ethane
    Ethane,
    ///ZF - Sulfate Sulfur
    SulfateSulfur,
    ///ZFE - Iron
    Iron,
    ///ZFL - Newspaper--Full Page
    NewspaperFullPage,
    ///ZFS - Iron plus Silicon
    IronPlusSilicon,
    ///ZG - Organic Sulfur
    OrganicSulfur,
    ///ZGE - Germanium
    Germanium,
    ///ZH - Hydrogen
    Hydrogen,
    ///ZHP - Heptane
    Heptane,
    ///ZHS - Hydrogen Sulfide
    HydrogenSulfide,
    ///ZHX - Hexane
    Hexane,
    ///ZIB - I-Butane
    IButane,
    ///ZIP - I-Pentane
    IPentane,
    ///ZMG - Magnesium
    Magnesium,
    ///ZMN - Manganese
    Manganese,
    ///ZMO - Molybdenum
    Molybdenum,
    ///ZMT - Methane
    Methane,
    ///ZN - Nitrogen
    Nitrogen,
    ///ZNB - Niobium
    Niobium,
    ///ZNI - Nickel
    Nickel,
    ///ZNP - Neo-Pentane
    NeoPentane,
    ///ZO - Oxygen
    Oxygen,
    ///ZOC - Octane
    Octane,
    ///ZP - Phosphorous
    Phosphorous,
    ///ZPB - Lead
    Lead,
    ///ZPP - Propane
    Propane,
    ///ZPT - N-Pentane
    NPentane,
    ///ZR - Pyritic Sulfur
    PyriticSulfur,
    ///ZS - Sulfur
    Sulfur,
    ///ZSB - Antimony
    Antimony,
    ///ZSE - Selenium
    Selenium,
    ///ZSI - Silicon
    Silicon,
    ///ZSN - Tin
    Tin,
    ///ZTA - Tantalum
    Tantalum,
    ///ZTB - Newspaper--Tabloid Page
    NewspaperTabloidPage,
    ///ZTE - Tellurium
    Tellurium,
    ///ZTI - Titanium
    Titanium,
    ///ZV - Vanadium
    Vanadium,
    ///ZW - Tungsten
    Tungsten,
    ///ZZN - Zinc
    Zinc,
    ///ZZR - Zirconium
    Zirconium,
    ///ZZZ - Mutually Defined
    MutuallyDefined,
}
impl MeasurementQualifier {
    pub fn code(&self) -> &str {
        {
            use MeasurementQualifier::*;
            match self {
                ControlEfficiency => "1",
                RadioFrequency => "1F",
                CaptureEfficiency => "2",
                AlternateRadioFrequency => "2F",
                PhotonfluxDensity => "3",
                TargetDepth => "3A",
                CurrentDepth => "3B",
                TotalDepth => "3C",
                WellTestBeforeOil => "3D",
                WellTestBeforeGas => "3E",
                WellTestBeforeWater => "3F",
                WellTestAfterOil => "3G",
                WellTestAfterGas => "3H",
                WellTestAfterWater => "3I",
                EstimatedDepthOfOperations => "3J",
                ThroughputRate => "4",
                SquelchTone => "4F",
                CloudCover => "5",
                HeightAboveGround => "5F",
                Velocity => "6",
                Gain => "6F",
                PlumeHeight => "7",
                Individuals => "8",
                DirectionalHeightAboveAverageTerrain => "8F",
                StorageLimits => "9",
                PaintingCosts => "10",
                StructuralCosts => "11",
                Appliances => "12",
                Utilities => "13",
                CarpetOrFloors => "14",
                OtherRepairs => "15",
                Landscaping => "16",
                Roof => "18",
                Windows => "19",
                CleaningOrTrashRemoval => "20",
                ProbableSalesPrice => "21",
                Proximity => "22",
                RepairsAndImprovements => "23",
                ContributoryValueOfRepairsAndImprovements => "24",
                MarketingTime => "25",
                ClosedComparableSales => "26",
                CompetitiveListingsInPriceRange => "27",
                FinancingConcessions => "28",
                MarketingConcessions => "29",
                ProbableNetPrice => "30",
                SuggestedInitialListPrice => "31",
                ValueChange => "32",
                ProbableFinalValue => "33",
                OccupancyRate => "34",
                NumberOfLivingUnits => "35",
                NumberOfPhases => "36",
                NumberOfActiveListings => "37",
                PriceActiveListings => "38",
                PricePerGrossLivingArea => "40",
                BuiltUpRate => "41",
                VacantRate => "42",
                TypicalRents => "43",
                NeighborhoodApartmentVacancy => "44",
                NumberOfAdmissions => "45",
                CostOfHire => "48",
                Frontage => "49",
                GrossSales => "50",
                NumberOfEmployees => "51",
                Payroll => "52",
                PerCapitaOrEach => "53",
                Remuneration => "54",
                TotalCost => "56",
                TotalMileage => "57",
                NumberOfRatingUnits => "58",
                GarageEmployeePayrollMaximum => "62",
                EmployeeGrossWageLessAllowableDeductions => "63",
                GarageEmployeeAverageHoursWorkedPerWeek => "65",
                Code66 => "66",
                GrossWage => "68",
                SubcontractorLaborAndMaterials => "78",
                SubcontractorLaborOnly => "79",
                ConsolidatedWeight => "A",
                Acids => "A1",
                Adsorption => "A2",
                AgingTime => "A3",
                Aromatics => "A4",
                AverageDifferentialPressure => "A5",
                AverageStaticPressure => "A6",
                FlameProjectionDistance => "A7",
                Exposure => "A9",
                AlternatingCurrent => "AA",
                AcApparentPower => "AAP",
                ActivationEnergy => "AB",
                Absorbance => "ABO",
                Abrasion => "ABR",
                Absorbency => "ABS",
                Actinium => "AC",
                AcidNumber => "ACN",
                AmbientTemperature => "AD",
                Adhesion => "ADH",
                DyeManufacturingUnits => "ADM",
                Argon => "AE",
                AngleOfBend => "AF",
                Americium => "AG",
                InventoryAge => "AGE",
                AggressiveIndex => "AGI",
                Astatine => "AH",
                Acidity => "AI",
                AimGage => "AJ",
                CodeAK => "AK",
                SpineShow => "AL",
                Alkalinity => "ALK",
                AlkalinityNumber => "ALN",
                AlphaCellulose => "ALP",
                AverageSpeed => "AM",
                Amines => "AMI",
                AverageMolecularWeight => "AMW",
                FluteTest => "AN",
                Antioxidant => "AOX",
                AveragePressure => "AP",
                ApiGravity => "API",
                Appearance => "APP",
                AshFusionTemperature => "AS",
                Ash => "ASH",
                Assay => "ASY",
                Additive => "AT",
                NumberOfUnitsProjected => "AU",
                Age => "AV",
                AverageTemperature => "AVT",
                RemainingEconomicLife => "AW",
                RemainingPhysicalLife => "AX",
                NumberOfComparableSales => "AY",
                ArborSize => "AZ",
                BilledWeight => "B",
                BaseNumber => "B1",
                NumberOfComparableListings => "B2",
                PresentLandUse => "B3",
                SubjectPhaseDwellingUnits => "B4",
                OctanolWaterPartitionCoefficient => "B5",
                TotalProjectDwellingUnits => "B6",
                Barium => "BA",
                Beryllium => "BB",
                BilletSize => "BC",
                Bias => "BD",
                PerforationBottomDepth => "BDP",
                BoronFactor => "BE",
                Brinell => "BF",
                Berkelium => "BG",
                Bromine => "BH",
                BottomholePressureFlowing => "BHF",
                BottomholePressureShutin => "BHS",
                BarkInChips => "BIC",
                BurstIndex => "BJ",
                Bulk => "BK",
                Blisters => "BL",
                Bend => "BN",
                AmountBoundInMaterial => "BND",
                CodeBO => "BO",
                BoilingRange => "BOR",
                BoilingPoint => "BP",
                Breaks => "BQ",
                Brightness => "BR",
                BreakingStrength => "BRS",
                PercentBottomSedimentAndWater => "BSW",
                Bursts => "BT",
                Buckles => "BU",
                BulkDensity => "BUD",
                BasisWeight => "BW",
                BloodAlcohol => "BX",
                ActualNewRepeatedForCombination => "C",
                ColorGrade => "C0",
                Carbonyl => "C1",
                Catalyst => "C2",
                MaximumContraction => "C3",
                ColorQuadrant => "C4",
                Caliper => "CA",
                CausticReactionSeverity => "CAU",
                Celsius => "CC",
                CompositeCorrectedFactor => "CCF",
                CombinedCenterOfGravity => "CCG",
                Compression => "CD",
                CapacitanceIn => "CE",
                CapacitanceOut => "CF",
                Cadmium => "CG",
                ColorGraynessRd => "CGR",
                Cesium => "CH",
                ChemicalAdditionRate => "CHA",
                ChlorophyllA => "CHB",
                ConcentrationOfHazardousComponent => "CHC",
                RateOfChange => "CHG",
                Chlorides => "CHL",
                Curium => "CI",
                CueneIntrinsicViscosity => "CIV",
                CycleTime => "CJ",
                Californium => "CK",
                CodeCL => "CL",
                Clarity => "CLA",
                Calibration => "CLB",
                Cleanliness => "CLN",
                Cures => "CM",
                Chlorine => "CN",
                CoreLoss => "CO",
                CoefficientFactor => "COF",
                CoerciveForce => "COH",
                Color => "COL",
                Concentration => "CON",
                Corrosiveness => "COR",
                Cost => "COS",
                Content => "COT",
                Crimp => "CP",
                CasingPressureFlowing => "CPF",
                CasingPressureShutin => "CPS",
                CupTestAdhesion => "CPT",
                Cuts => "CQ",
                CrosswiseSpacing => "CR",
                FreeChlorineResidual => "CRF",
                CompressionRelaxation => "CRL",
                Crown => "CRN",
                TotalChlorineResidual => "CRT",
                CrossSection => "CS",
                ChokeSizeCasing => "CSC",
                CostRealism => "CSR",
                ChokeSizeTubing => "CST",
                CenterToCenter => "CT",
                Coating => "CTG",
                ContactTime => "CTT",
                CoilCurvature => "CU",
                CureTime => "CUT",
                CuttableWidth => "CW",
                ChargeWeight => "CWT",
                CalculatedValue => "CX",
                Contamination => "CY",
                CodeCYB => "CYB",
                DestinationWeightAgreement => "D",
                MaximumDilatation => "D1",
                DispersingAgent => "D2",
                DryingAgent => "D3",
                DryPoint => "D4",
                Wear => "D5",
                Horizontal => "D6",
                DistillationFraction => "D7",
                Vertical => "D8",
                DotsPerInch => "D9",
                DeltaValueA => "DA",
                DatumDepth => "DAT",
                DeltaValueB => "DB",
                DuctileClass => "DC",
                DirtCount => "DCT",
                DepthOfDents => "DD",
                Defects => "DE",
                DeMinimisLevel => "DEM",
                DistanceAcrossFlats => "DF",
                DirectCurrent => "DG",
                Dysprosium => "DH",
                Diameter => "DI",
                CodeDIL => "DIL",
                DistillationRange => "DIR",
                Dispersion => "DIS",
                DominantWaveLength => "DJ",
                ColorBitsInPalette => "DK",
                DeltaValueL => "DL",
                DialCount => "DLC",
                DualAmplitude => "DM",
                DichloromethaneExtract => "DME",
                DistributedMeterFactor => "DMF",
                Density => "DN",
                CompressedFileSize => "DO",
                DoubleOlsenCup => "DOC",
                Depth => "DP",
                DegreeOfPolymerization => "DPM",
                DeltaR => "DR",
                Dryness => "DRY",
                DistanceBetweenPoints => "DS",
                DistanceFromBasePoint => "DT",
                DrawTension => "DU",
                CodeDW => "DW",
                DewPoint => "DWP",
                Dyeability => "DY",
                EstimatedNewWeight => "E",
                ExtraneousMatter => "E0",
                End => "E1",
                Elongation => "EA",
                EdgeBurr => "EB",
                EnglishCoilDimensions => "EC",
                Eddy => "ED",
                Einsteinium => "EE",
                Europium => "EF",
                EquivalentTemperature => "EG",
                Erbium => "EH",
                Expansion => "EI",
                ElectricalConductivity => "EJ",
                Elasticity => "EL",
                ExposureCeilingThresholdLimitValue => "ELC",
                Elevation => "ELE",
                ImmediatelyDangerousToLifeAndHealth => "ELI",
                ExplosionLimit => "ELL",
                CodeELO => "ELO",
                CodeELP => "ELP",
                CodeELS => "ELS",
                CodeELT => "ELT",
                UnshieldedExposureRate => "ELV",
                CodeELW => "ELW",
                CodeELX => "ELX",
                ElmendorfTear => "EM",
                Entanglement => "EN",
                ExcitingPower => "EP",
                ExpectedProductLife => "EPL",
                Edge => "ES",
                EndPoint => "ET",
                EvaporationLoss => "EVL",
                EvaporationRate => "EVR",
                EmptyWeight => "EW",
                Eccentricity => "EX",
                ExhaustBenzeneEmissions => "EXH",
                Extractables => "EXT",
                DeficitWeight => "F",
                FirePoint => "F1",
                RadiatedPower => "F2",
                CodeF3 => "F3",
                HeightAboveAverageTerrain => "F4",
                GroundElevation => "F5",
                HeightToTip => "F6",
                RadiusFromALocation => "F7",
                RadiusFromCoordinates => "F8",
                OperatingHours => "F9",
                Fluorine => "FA",
                Flare => "FB",
                AcetateBreakPoint => "FBP",
                ShortCycleFlatness => "FC",
                FrequencyOfOperation => "FD",
                Fermium => "FE",
                Francium => "FF",
                FreezingPoint => "FG",
                Finish => "FH",
                FilamentCount => "FI",
                FilterNumber => "FIL",
                Fineness => "FIN",
                Filterability => "FIT",
                FaceWidth => "FJ",
                FluidConsistency => "FK",
                LongitudinalFlatness => "FL",
                FluidPoint => "FLD",
                FiberLength => "FLN",
                FluidLevelAbovePump => "FLP",
                Fluting => "FLT",
                FlavorThreshold => "FLV",
                FlammabilityLimits => "FML",
                Flammability => "FMZ",
                Flatness => "FN",
                FinesRetainedOnScreen => "FNL",
                Fines => "FNS",
                Foam => "FOA",
                FrontOverHangOfVehicle => "FOH",
                FoulingIndex => "FOI",
                ForeignMatter => "FOR",
                Flashpoint => "FP",
                AcetatePluggingValue => "FPV",
                Frequency => "FQ",
                FlowRate => "FR",
                FoldStrength => "FS",
                FreeSwellingIndex => "FSI",
                FlangeThickness => "FT",
                Azimuth => "FU",
                FullLoad => "FUD",
                TransverseFlatness => "FV",
                FlangeWidth => "FW",
                Filler => "FX",
                Beam => "FY",
                CodeFZ => "FZ",
                GrossWeight => "G",
                Gutter => "G1",
                GrainSize => "G2",
                Tilt => "G3",
                GForce => "G4",
                Grain => "GA",
                GuidedBendsRoot => "GB",
                Gadolinium => "GC",
                Gold => "GD",
                GuidedBendsFace => "GE",
                Gel => "GEL",
                GuidedBendsSide => "GF",
                Gauge => "GG",
                GasGravity => "GGR",
                Gallium => "GH",
                CodeGI => "GI",
                GasInjectionTestRate => "GIR",
                CodeGJ => "GJ",
                TintingStrength => "GK",
                Gloss => "GL",
                GroundLevelElevation => "GLE",
                EmulsionGloss => "GM",
                GrossToNetConversionFactor => "GN",
                ParticleEndPoint => "GO",
                GasToOilRatio => "GOR",
                GroupPackageSeparation => "GP",
                ScatteredParticles => "GQ",
                Gravity => "GR",
                GasTestRate => "GRA",
                Graininess => "GRI",
                CarbonBlackUndertone => "GS",
                GlassTransitionTemperature => "GT",
                CodeGW => "GW",
                GroundWaterReferencePoint => "H",
                Heavies => "H1",
                WaterVolume => "H2O",
                NetExplosiveWeight => "H8",
                RecommendedExposureLimit => "H9",
                HydroPressure => "HA",
                HardeningRate => "HAR",
                Haze => "HAZ",
                HeavyAluminas => "HB",
                Helium => "HC",
                HorizontalCenterOfGravity => "HCG",
                DistanceToEndpoint => "HCH",
                EmissionFactor => "HCI",
                HeatContent => "HCJ",
                PollutantEmission => "HCK",
                Population => "HCL",
                ReleaseDuration => "HCM",
                ReleasedQuantity => "HCN",
                ReleaseRate => "HCO",
                SulfurContent => "HCP",
                WindSpeed => "HCQ",
                AcuteToxicity => "HCR",
                ChronicToxicity => "HCS",
                Discharges => "HCT",
                Effectiveness => "HCU",
                Fertilization => "HCV",
                SamplesInCompliance => "HCW",
                Toxicity => "HCX",
                HighPropagationDelayTime => "HD",
                HeavySilicates => "HE",
                Hardness => "HF",
                HeavySulfides => "HG",
                HeavyGlobularOxides => "HH",
                HemisphericalPoint => "HHW",
                Hafnium => "HI",
                HighBoilers => "HIB",
                Holmium => "HJ",
                HumanFactors => "HK",
                HeatLoss => "HL",
                CodeHM => "HM",
                Holes => "HO",
                HeatOfCombustion => "HOC",
                HeightOfTreadPlatePattern => "HP",
                HeightOfRunners => "HR",
                Height => "HT",
                HeatEquivalency => "HTE",
                HeavyMetals => "HVM",
                SofteningPoint => "HWS",
                HydroxylNumber => "HYD",
                Hazepoint => "HZ",
                HazardousComponent => "HZC",
                HeatInput => "I",
                ImperfectionsThicks => "IA",
                ImpactEnergy => "IB",
                IncrementalDistance => "IC",
                InsideDiameter => "ID",
                Identification => "IDE",
                ImperfectionsThins => "IE",
                InputLowVoltage => "IF",
                GranulatedIngotSize => "IG",
                AutoignitionTemperature => "IGA",
                InputGasRate => "IGR",
                InputCurrent => "IH",
                InherentViscosity => "IHV",
                InputSetupTime => "II",
                InputHoldTime => "IJ",
                Indium => "IK",
                ImperfectionsNeps => "IL",
                Iridium => "IM",
                ImpactAdhesion => "IMA",
                Impurities => "IMP",
                InputLowCurrent => "IN",
                InitialDeformationPoint => "IND",
                Insolubles => "INS",
                InputHighCurrent => "IO",
                InputLeakageCurrent => "IP",
                PrimaryIrritationIndex => "IPI",
                InputClampDiodeVoltage => "IQ",
                IdealDiameter => "IR",
                InclusionRating => "IRA",
                InputHighVoltage => "IS",
                InterruptPulsePeriod => "IT",
                IntegratedDifferential => "ITD",
                InterruptSetupTime => "IU",
                Iodine => "IV",
                IncrustationFactor => "IW",
                CodeIX => "IX",
                IndexDifferential => "IXD",
                Induction => "IY",
                IngotPoundSize => "IZ",
                JunctionTemperature => "JA",
                JominyHardenability => "JOM",
                Knoop => "KA",
                Krypton => "KB",
                CodeKN => "KN",
                StrengthCoefficient => "KVL",
                LegalWeight => "L",
                LeafGrade => "L0",
                Lights => "L1",
                Lanthanum => "LA",
                LanglierIndex => "LAI",
                Lithium => "LB",
                LowPropagationDelayTime => "LC",
                CodeLC5 => "LC5",
                LongitudinalCenterOfGravity => "LCG",
                Lawrencium => "LD",
                CodeLD5 => "LD5",
                CodeLDH => "LDH",
                Lutetium => "LE",
                Leaf => "LEF",
                LongFibers => "LF",
                Leg => "LG",
                LiquidInjectionTestRate => "LIR",
                LimitIntrinsicViscosity => "LIV",
                LongLeg => "LL",
                LightLoad => "LLD",
                CodeLM => "LM",
                Length => "LN",
                LongLength => "LO",
                LossOnIgnition => "LOI",
                LossOnDrying => "LOS",
                LowBoilers => "LOW",
                CamberOfPatternLine => "LP",
                LiquefiedPetroleumGasFactor => "LPG",
                LabeledProductLife => "LPL",
                LinePressure => "LPR",
                ShortLeg => "LS",
                LockSeamAdhesion => "LSA",
                LengthShrinkage => "LSK",
                LinearSheetSwelling => "LSS",
                LengthwiseSpacing => "LT",
                LinerTopDepth => "LTD",
                LongWidth => "LW",
                CodeM => "M",
                MeltRange => "M1",
                MaximumDifferentialPressure => "M2",
                MaximumStaticPressure => "M3",
                Area => "M4",
                MinimumSpeed => "M5",
                MaximumSpeed => "M6",
                MeanAverage => "MA",
                Maturity => "MAT",
                Mendelevium => "MB",
                MinMaxCuttableWidth => "MC",
                MagneticContamination => "MCN",
                MeasurementVoltage => "MD",
                CodeMDL => "MDL",
                MaximumInputLowVoltage => "ME",
                MediaDepth => "MEA",
                MeterFactor => "MEF",
                MeltTime => "MEL",
                MeatProtein => "MEP",
                EfficientRateReservoir => "MER",
                MinimumInputHighVoltage => "MF",
                Mercury => "MG",
                MeltingPoint => "MH",
                HighestTorque => "MHI",
                Minimum => "MI",
                Micronaire => "MIC",
                MilkFat => "MIL",
                CodeMJ => "MJ",
                CodeMK => "MK",
                Management => "MM",
                MinimumAverage => "MN",
                Mottles => "MO",
                Moisture => "MOI",
                Mortality => "MOR",
                MullenPop => "MP",
                ProductionRateWell => "MPR",
                ManyPressTest => "MPT",
                McQuaid => "MQ",
                CodeMR => "MR",
                CodeMS => "MS",
                MoistureContent => "MT",
                MaximumTotalDepth => "MTD",
                Multiplier => "MU",
                Mullen => "MUL",
                MaximumAverage => "MV",
                MolecularWeight => "MW",
                Maximum => "MX",
                MagnetizingField => "MY",
                ActualNetWeight => "N",
                NumberPerPackage => "NA",
                NumberPerBundle => "NB",
                NumberPerCoilGroup => "NC",
                Neodymium => "ND",
                NeutralizationNumber => "NEU",
                Neon => "NF",
                Nobelium => "NG",
                NumberOfItemsPerPackageLabel => "NH",
                NumberOfSplicesPerPackageLabel => "NI",
                NilDuctilityTest => "NIL",
                NumberOfSheetsPerPackageLabel => "NJ",
                NestingFactor => "NK",
                NumberPerLift => "NL",
                NumberPkgsPerMasterPack => "NM",
                NetNetWeight => "NNW",
                CodeNO => "NO",
                NumberOfCosigners => "NOC",
                NonVolatileMatter => "NON",
                NumberOfReferences => "NOR",
                NOxEmissionsPerformance => "NOX",
                PercentOfSpecified => "NP",
                NumberPerSkid => "NS",
                NumberPerUnit => "NU",
                NValue => "NV",
                ExcessWeightOverMaximum => "O",
                OrificeInsideDiameter => "O1",
                Offset => "OA",
                ObservedAmericanPetroleumInstituteGravity => "OAP",
                Osmium => "OB",
                ObservedTemperature => "OBT",
                OutputLowVoltage => "OC",
                OilCondensateGravity => "OCG",
                OilCondensateTestRate => "OCR",
                OutsideDiameter => "OD",
                Odor => "ODR",
                OutputLowCurrent => "OE",
                OutputHighVoltage => "OF",
                OutputHighCurrent => "OG",
                CodeOH => "OH",
                OutputOffCurrentLow => "OI",
                Oil => "OIL",
                OutputOffCurrentHigh => "OJ",
                OutputShortCircuitCurrent => "OK",
                OutputDisableTimeFromLowLevelOfA3StateOutput => "OL",
                Olefins => "OLE",
                CodeOM => "OM",
                OutputDisableTimeFromHighLevelOfA3StateOutput => "ON",
                OutputEnableTimeFromLowLevelOfA3StateOutput => "OO",
                Openness => "OP",
                OutputEnableTimeFromHighLevelOfA3StateOutput => "OQ",
                DistanceBetweenOutsideRunners => "OR",
                OrganicCarbon => "ORC",
                OpenCircuits => "OS",
                OutputDelayTime => "OT",
                OthersEach => "OTE",
                OdorThreshold => "OTH",
                OthersTotal => "OTT",
                Opacity => "OV",
                OverallWidth => "OW",
                OwnershipShare => "OX",
                OxidizableSubstance => "OXI",
                OxidizingSubstance => "OXS",
                OperatingWeight => "OY",
                Price => "P1",
                PackageSeparation => "PA",
                ParticleSize => "PAR",
                Pressure => "PB",
                PlugBackTotalDepth => "PBD",
                PerHundredLinearYards => "PC",
                Platinum => "PD",
                CasingLinerTubingDepth => "PDE",
                PumpDepthFromGround => "PDG",
                Potassium => "PE",
                MagneticPermeability => "PER",
                Promethium => "PF",
                PerforationFeetOpen => "PFO",
                Polonium => "PG",
                PulseSetupTime => "PH",
                PH => "PHA",
                HardwoodFiber => "PHW",
                PulseHoldTime => "PI",
                PickOff => "PIC",
                PulseWidth => "PJ",
                PulseRecoveryTime => "PK",
                PercentDefective => "PL",
                Practice => "PM",
                Palladium => "PN",
                CodePO => "PO",
                Completion => "POC",
                PhysicalDescriptionOuterDiameter => "POD",
                PourPoint => "POP",
                PowderPastePackageSize => "PP",
                ProprietaryShade => "PPS",
                Plutonium => "PQ",
                CodePQL => "PQL",
                Praseodymium => "PR",
                ProportionAlive => "PRA",
                PriorExperience => "PRE",
                PressureFactor => "PRF",
                ProductIndex => "PRI",
                ProductLevel => "PRL",
                ProportionNormal => "PRN",
                Processability => "PRO",
                ProductReportableQuantity => "PRQ",
                Porosity => "PRY",
                ProportionFertilized => "PRZ",
                Protactinium => "PS",
                PercentSolutionActual => "PSA",
                PastPerformance => "PSP",
                SoftwoodFiber => "PSW",
                Pits => "PT",
                PressureBase => "PU",
                Picks => "PV",
                PurchasedWidth => "PW",
                ProcessedWaste => "PWA",
                PhysicalDescriptionWeight => "PWE",
                PowerFactor => "PWF",
                Purity => "PX",
                PercentOfWater => "PY",
                PipeSizeNominal => "PZ",
                VolatileOrganicCompoundsPlusWater => "Q",
                QualityIndex => "QA",
                QuantityOrLoadingAverage => "QB",
                QuantityOrLoadingMaximum => "QC",
                QualityOrConcentrationAverage => "QD",
                QualityOrConcentrationMinimum => "QE",
                QualityOrConcentrationMaximum => "QF",
                Duration => "QG",
                Abundance => "QI",
                Biomass => "QJ",
                SizeClass => "QK",
                Quality => "QL",
                ReportableQuantity => "QUR",
                PerUnitDunnage => "R",
                Hemoglobin => "R1",
                Hematocrit => "R2",
                EpoetinStartingDosage => "R3",
                Creatinine => "R4",
                Speed => "R7",
                SpeedLimit => "R8",
                RelativeFractionOfPureLongChainCellulose => "R10",
                RelativeFractionOfTotalCellulose => "R18",
                RelativeHumidity => "RA",
                Radius => "RAD",
                RoofAdjustmentFactor => "RAF",
                RangeValue => "RB",
                RadiusOfCorner => "RC",
                Readpoint => "RD",
                ReamWeight => "RE",
                Reactivity => "REA",
                ReducingSubstance => "RED",
                Refining => "REF",
                RefractiveIndex => "REI",
                Reflectance => "REL",
                Resistance => "RES",
                Resistivity => "RF",
                Radium => "RG",
                Rhenium => "RH",
                Rubidium => "RI",
                RockwellC => "RJ",
                RockwellB => "RK",
                ReductionRation => "RL",
                CodeRM => "RM",
                RequiredInterruptRelease => "RN",
                ResetPulseWidth => "RO",
                RearOverHangOfVehicle => "ROH",
                OxygenFromARenewableOxygenate => "ROX",
                ReductionOfArea => "RP",
                Radon => "RQ",
                ReductionRatio => "RR",
                CodeRS => "RS",
                RollSize => "RSZ",
                RoundsAmmunitionMilitary => "RT",
                ReportingTemperatureBase => "RTB",
                Rhodium => "RU",
                CodeRUD => "RUD",
                Ruthenium => "RV",
                ReidVaporPressure => "RVP",
                RollingWidth => "RW",
                Ridges => "RX",
                Ratio => "RY",
                StateWeight => "S",
                Smoothness => "S1",
                SelvedgeOnBeam => "S2",
                SheffieldSmoothness => "S3",
                SurfaceStrength => "S4",
                Stiffness => "S5",
                Saturation => "S6",
                Sediment => "S7",
                Solubility => "S8",
                SiteAtmosphericPressure => "S9",
                PulpImpurities => "S10",
                Start => "S12",
                Hemicellulose => "S18",
                SortCodeCieLab => "SA",
                CodeSAL => "SAL",
                SaponificationNumber => "SAP",
                SortCodeCmc => "SB",
                CodeSC => "SC",
                Schedule => "SCH",
                Strength => "SD",
                SelvageLeft => "SE",
                Severity => "SEV",
                Samarium => "SF",
                ShortFiberContent => "SFC",
                SlitWidth => "SG",
                Strontium => "SH",
                ShelfLife => "SHA",
                SupplyCurrent => "SI",
                CodeSIL => "SIL",
                ShortCircuits => "SJ",
                Shrinkage => "SK",
                ShortLength => "SL",
                Solderability => "SLD",
                SlaggingIndex => "SLI",
                Shear => "SM",
                SamBRating => "SMB",
                SamDRating => "SMD",
                Stain => "SN",
                SortCodeCieLch => "SO",
                Solids => "SOD",
                SofteningRange => "SOF",
                SplinterCount => "SP",
                SpecificGravity => "SPG",
                Sphere => "SPH",
                SeparatorPressure => "SPR",
                StaticPressure => "SPS",
                ShippedQuantity => "SQ",
                SelvageRight => "SR",
                Silver => "SS",
                StopRecoveryStartupTime => "ST",
                Stability => "STA",
                ShortTermExposureLimit => "STL",
                Staple => "STP",
                ShippedUnits => "SU",
                SuspendedMatter => "SUM",
                SurfaceRoughness => "SUR",
                SurfaceTension => "SUT",
                Scandium => "SV",
                Survival => "SVL",
                ShortWidth => "SW",
                Sodium => "SX",
                S10MinusS18Value => "SXX",
                ServiceInterruptDuration => "SY",
                SkidHeight => "SZ",
                TareWeight => "T",
                TirePressure => "T1",
                TubeInsideDiameter => "T2",
                Technical => "T3",
                SingleEndBreak => "T4",
                SkeinBreak => "T5",
                T50 => "T50",
                T90 => "T90",
                CodeTA => "TA",
                Taste => "TAS",
                CodeTB => "TB",
                Temperature => "TC",
                TireTreadContactLength => "TCL",
                ThinAluminas => "TD",
                PerforationTopDepth => "TDP",
                Tenacity => "TE",
                AutodecompositionTemperature => "TEE",
                StorageTemperature => "TES",
                Texture => "TEX",
                Tensile => "TF",
                ThinSulfides => "TG",
                Thickness => "TH",
                ThinSilicates => "TI",
                TotalSupplyCurrent => "TJ",
                TimerPulseWidth => "TK",
                TaperedSteppedLengthType => "TL",
                LengthTypeMultiples => "TM",
                TimerPeriod => "TN",
                Terbium => "TO",
                AquaticToxicity => "TOA",
                Torque => "TOR",
                ToxicEmissionsPerformance => "TOX",
                Thorium => "TP",
                TemperatureFactor => "TPF",
                TubingPressureFlowing => "TPL",
                ThresholdPlanningQuantity => "TPQ",
                TubingPressureShutin => "TPS",
                ThinGlobularOxides => "TQ",
                LengthTypeRandom => "TR",
                TrashArea => "TRA",
                TrashCount => "TRC",
                TireDiameter => "TRD",
                Transmittance => "TRN",
                CodeTRS => "TRS",
                CodeTRT => "TRT",
                LengthTypeSpecific => "TS",
                TrimSize => "TSZ",
                Time => "TT",
                TrailerTongueLength => "TTL",
                Technetium => "TU",
                Turbidity => "TUR",
                Thallium => "TV",
                MaximumTrueVerticalDepth => "TVD",
                Top => "TW",
                TireWidth => "TWD",
                Thulium => "TX",
                TearStrength => "TY",
                WeightPerUnit => "U",
                Uranium => "UA",
                Cube => "UCB",
                Usage => "UG",
                UnipunchAdhesion => "UNA",
                Uniformity => "UNI",
                Unknowns => "UNK",
                OxygenationLevel => "V",
                VaporDensity => "VAD",
                VaporPressure => "VAP",
                VBendAdhesion => "VBA",
                VerticalCenterOfGravity => "VCG",
                CodeVH => "VH",
                Vinyl => "VIN",
                Viscosity => "VIS",
                Voltage => "VO",
                VocEmissionsPerformance => "VOC",
                Volume => "VOL",
                Volatiles => "VOT",
                VolatilesByVolume => "VOV",
                VolatilesByWeight => "VOW",
                VolumeSplitToOthers => "VSO",
                CodeVW => "VW",
                VolumeWeight => "VWT",
                ReformulatedFuelLevel => "W",
                WeightPerUnitOfArea => "WA",
                Web => "WB",
                WebDepthHeight => "WC",
                Width => "WD",
                WaterDepth => "WDE",
                Wolfram => "WE",
                WeightLoss => "WEL",
                WaitRecoveryStartupTime => "WF",
                Whiteness => "WH",
                WindingLoss => "WI",
                WallThickness => "WL",
                CodeWM => "WM",
                WaterOilDistributionCoefficient => "WOD",
                WellheadPressureFlowing => "WPF",
                WaterProductLevel => "WPL",
                WellheadPressureShutin => "WPS",
                Wrinkles => "WR",
                WaterTestRate => "WRA",
                WidthShrinkage => "WSK",
                Weight => "WT",
                WaterTankBottomLevel => "WTB",
                WeightPerUnitOfLength => "WU",
                WaxPick => "WX",
                CodeX => "X",
                Xenon => "XA",
                CodeXH => "XH",
                Specified => "XP",
                Squareness => "XQ",
                SpoolSize => "XZ",
                Yttrium => "YA",
                Yield => "YB",
                Ytterbium => "YC",
                YarnCount => "YD",
                YieldPointElongation => "YPE",
                Aluminum => "ZAL",
                Arsenic => "ZAS",
                Boron => "ZB",
                Bismuth => "ZBI",
                NButane => "ZBT",
                Benzene => "ZBZ",
                Carbon => "ZC",
                Calcium => "ZCA",
                Columbium => "ZCB",
                CarbonDioxide => "ZCD",
                Cerium => "ZCE",
                CarbonMonoxide => "ZCM",
                Cobalt => "ZCO",
                Chromium => "ZCR",
                Copper => "ZCU",
                LoadFactor => "ZD",
                Ethane => "ZET",
                SulfateSulfur => "ZF",
                Iron => "ZFE",
                NewspaperFullPage => "ZFL",
                IronPlusSilicon => "ZFS",
                OrganicSulfur => "ZG",
                Germanium => "ZGE",
                Hydrogen => "ZH",
                Heptane => "ZHP",
                HydrogenSulfide => "ZHS",
                Hexane => "ZHX",
                IButane => "ZIB",
                IPentane => "ZIP",
                Magnesium => "ZMG",
                Manganese => "ZMN",
                Molybdenum => "ZMO",
                Methane => "ZMT",
                Nitrogen => "ZN",
                Niobium => "ZNB",
                Nickel => "ZNI",
                NeoPentane => "ZNP",
                Oxygen => "ZO",
                Octane => "ZOC",
                Phosphorous => "ZP",
                Lead => "ZPB",
                Propane => "ZPP",
                NPentane => "ZPT",
                PyriticSulfur => "ZR",
                Sulfur => "ZS",
                Antimony => "ZSB",
                Selenium => "ZSE",
                Silicon => "ZSI",
                Tin => "ZSN",
                Tantalum => "ZTA",
                NewspaperTabloidPage => "ZTB",
                Tellurium => "ZTE",
                Titanium => "ZTI",
                Vanadium => "ZV",
                Tungsten => "ZW",
                Zinc => "ZZN",
                Zirconium => "ZZR",
                MutuallyDefined => "ZZZ",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<MeasurementQualifier> {
        use MeasurementQualifier::*;
        match code {
            b"1" => Some(ControlEfficiency),
            b"1F" => Some(RadioFrequency),
            b"2" => Some(CaptureEfficiency),
            b"2F" => Some(AlternateRadioFrequency),
            b"3" => Some(PhotonfluxDensity),
            b"3A" => Some(TargetDepth),
            b"3B" => Some(CurrentDepth),
            b"3C" => Some(TotalDepth),
            b"3D" => Some(WellTestBeforeOil),
            b"3E" => Some(WellTestBeforeGas),
            b"3F" => Some(WellTestBeforeWater),
            b"3G" => Some(WellTestAfterOil),
            b"3H" => Some(WellTestAfterGas),
            b"3I" => Some(WellTestAfterWater),
            b"3J" => Some(EstimatedDepthOfOperations),
            b"4" => Some(ThroughputRate),
            b"4F" => Some(SquelchTone),
            b"5" => Some(CloudCover),
            b"5F" => Some(HeightAboveGround),
            b"6" => Some(Velocity),
            b"6F" => Some(Gain),
            b"7" => Some(PlumeHeight),
            b"8" => Some(Individuals),
            b"8F" => Some(DirectionalHeightAboveAverageTerrain),
            b"9" => Some(StorageLimits),
            b"10" => Some(PaintingCosts),
            b"11" => Some(StructuralCosts),
            b"12" => Some(Appliances),
            b"13" => Some(Utilities),
            b"14" => Some(CarpetOrFloors),
            b"15" => Some(OtherRepairs),
            b"16" => Some(Landscaping),
            b"18" => Some(Roof),
            b"19" => Some(Windows),
            b"20" => Some(CleaningOrTrashRemoval),
            b"21" => Some(ProbableSalesPrice),
            b"22" => Some(Proximity),
            b"23" => Some(RepairsAndImprovements),
            b"24" => Some(ContributoryValueOfRepairsAndImprovements),
            b"25" => Some(MarketingTime),
            b"26" => Some(ClosedComparableSales),
            b"27" => Some(CompetitiveListingsInPriceRange),
            b"28" => Some(FinancingConcessions),
            b"29" => Some(MarketingConcessions),
            b"30" => Some(ProbableNetPrice),
            b"31" => Some(SuggestedInitialListPrice),
            b"32" => Some(ValueChange),
            b"33" => Some(ProbableFinalValue),
            b"34" => Some(OccupancyRate),
            b"35" => Some(NumberOfLivingUnits),
            b"36" => Some(NumberOfPhases),
            b"37" => Some(NumberOfActiveListings),
            b"38" => Some(PriceActiveListings),
            b"40" => Some(PricePerGrossLivingArea),
            b"41" => Some(BuiltUpRate),
            b"42" => Some(VacantRate),
            b"43" => Some(TypicalRents),
            b"44" => Some(NeighborhoodApartmentVacancy),
            b"45" => Some(NumberOfAdmissions),
            b"48" => Some(CostOfHire),
            b"49" => Some(Frontage),
            b"50" => Some(GrossSales),
            b"51" => Some(NumberOfEmployees),
            b"52" => Some(Payroll),
            b"53" => Some(PerCapitaOrEach),
            b"54" => Some(Remuneration),
            b"56" => Some(TotalCost),
            b"57" => Some(TotalMileage),
            b"58" => Some(NumberOfRatingUnits),
            b"62" => Some(GarageEmployeePayrollMaximum),
            b"63" => Some(EmployeeGrossWageLessAllowableDeductions),
            b"65" => Some(GarageEmployeeAverageHoursWorkedPerWeek),
            b"66" => Some(Code66),
            b"68" => Some(GrossWage),
            b"78" => Some(SubcontractorLaborAndMaterials),
            b"79" => Some(SubcontractorLaborOnly),
            b"A" => Some(ConsolidatedWeight),
            b"A1" => Some(Acids),
            b"A2" => Some(Adsorption),
            b"A3" => Some(AgingTime),
            b"A4" => Some(Aromatics),
            b"A5" => Some(AverageDifferentialPressure),
            b"A6" => Some(AverageStaticPressure),
            b"A7" => Some(FlameProjectionDistance),
            b"A9" => Some(Exposure),
            b"AA" => Some(AlternatingCurrent),
            b"AAP" => Some(AcApparentPower),
            b"AB" => Some(ActivationEnergy),
            b"ABO" => Some(Absorbance),
            b"ABR" => Some(Abrasion),
            b"ABS" => Some(Absorbency),
            b"AC" => Some(Actinium),
            b"ACN" => Some(AcidNumber),
            b"AD" => Some(AmbientTemperature),
            b"ADH" => Some(Adhesion),
            b"ADM" => Some(DyeManufacturingUnits),
            b"AE" => Some(Argon),
            b"AF" => Some(AngleOfBend),
            b"AG" => Some(Americium),
            b"AGE" => Some(InventoryAge),
            b"AGI" => Some(AggressiveIndex),
            b"AH" => Some(Astatine),
            b"AI" => Some(Acidity),
            b"AJ" => Some(AimGage),
            b"AK" => Some(CodeAK),
            b"AL" => Some(SpineShow),
            b"ALK" => Some(Alkalinity),
            b"ALN" => Some(AlkalinityNumber),
            b"ALP" => Some(AlphaCellulose),
            b"AM" => Some(AverageSpeed),
            b"AMI" => Some(Amines),
            b"AMW" => Some(AverageMolecularWeight),
            b"AN" => Some(FluteTest),
            b"AOX" => Some(Antioxidant),
            b"AP" => Some(AveragePressure),
            b"API" => Some(ApiGravity),
            b"APP" => Some(Appearance),
            b"AS" => Some(AshFusionTemperature),
            b"ASH" => Some(Ash),
            b"ASY" => Some(Assay),
            b"AT" => Some(Additive),
            b"AU" => Some(NumberOfUnitsProjected),
            b"AV" => Some(Age),
            b"AVT" => Some(AverageTemperature),
            b"AW" => Some(RemainingEconomicLife),
            b"AX" => Some(RemainingPhysicalLife),
            b"AY" => Some(NumberOfComparableSales),
            b"AZ" => Some(ArborSize),
            b"B" => Some(BilledWeight),
            b"B1" => Some(BaseNumber),
            b"B2" => Some(NumberOfComparableListings),
            b"B3" => Some(PresentLandUse),
            b"B4" => Some(SubjectPhaseDwellingUnits),
            b"B5" => Some(OctanolWaterPartitionCoefficient),
            b"B6" => Some(TotalProjectDwellingUnits),
            b"BA" => Some(Barium),
            b"BB" => Some(Beryllium),
            b"BC" => Some(BilletSize),
            b"BD" => Some(Bias),
            b"BDP" => Some(PerforationBottomDepth),
            b"BE" => Some(BoronFactor),
            b"BF" => Some(Brinell),
            b"BG" => Some(Berkelium),
            b"BH" => Some(Bromine),
            b"BHF" => Some(BottomholePressureFlowing),
            b"BHS" => Some(BottomholePressureShutin),
            b"BIC" => Some(BarkInChips),
            b"BJ" => Some(BurstIndex),
            b"BK" => Some(Bulk),
            b"BL" => Some(Blisters),
            b"BN" => Some(Bend),
            b"BND" => Some(AmountBoundInMaterial),
            b"BO" => Some(CodeBO),
            b"BOR" => Some(BoilingRange),
            b"BP" => Some(BoilingPoint),
            b"BQ" => Some(Breaks),
            b"BR" => Some(Brightness),
            b"BRS" => Some(BreakingStrength),
            b"BSW" => Some(PercentBottomSedimentAndWater),
            b"BT" => Some(Bursts),
            b"BU" => Some(Buckles),
            b"BUD" => Some(BulkDensity),
            b"BW" => Some(BasisWeight),
            b"BX" => Some(BloodAlcohol),
            b"C" => Some(ActualNewRepeatedForCombination),
            b"C0" => Some(ColorGrade),
            b"C1" => Some(Carbonyl),
            b"C2" => Some(Catalyst),
            b"C3" => Some(MaximumContraction),
            b"C4" => Some(ColorQuadrant),
            b"CA" => Some(Caliper),
            b"CAU" => Some(CausticReactionSeverity),
            b"CC" => Some(Celsius),
            b"CCF" => Some(CompositeCorrectedFactor),
            b"CCG" => Some(CombinedCenterOfGravity),
            b"CD" => Some(Compression),
            b"CE" => Some(CapacitanceIn),
            b"CF" => Some(CapacitanceOut),
            b"CG" => Some(Cadmium),
            b"CGR" => Some(ColorGraynessRd),
            b"CH" => Some(Cesium),
            b"CHA" => Some(ChemicalAdditionRate),
            b"CHB" => Some(ChlorophyllA),
            b"CHC" => Some(ConcentrationOfHazardousComponent),
            b"CHG" => Some(RateOfChange),
            b"CHL" => Some(Chlorides),
            b"CI" => Some(Curium),
            b"CIV" => Some(CueneIntrinsicViscosity),
            b"CJ" => Some(CycleTime),
            b"CK" => Some(Californium),
            b"CL" => Some(CodeCL),
            b"CLA" => Some(Clarity),
            b"CLB" => Some(Calibration),
            b"CLN" => Some(Cleanliness),
            b"CM" => Some(Cures),
            b"CN" => Some(Chlorine),
            b"CO" => Some(CoreLoss),
            b"COF" => Some(CoefficientFactor),
            b"COH" => Some(CoerciveForce),
            b"COL" => Some(Color),
            b"CON" => Some(Concentration),
            b"COR" => Some(Corrosiveness),
            b"COS" => Some(Cost),
            b"COT" => Some(Content),
            b"CP" => Some(Crimp),
            b"CPF" => Some(CasingPressureFlowing),
            b"CPS" => Some(CasingPressureShutin),
            b"CPT" => Some(CupTestAdhesion),
            b"CQ" => Some(Cuts),
            b"CR" => Some(CrosswiseSpacing),
            b"CRF" => Some(FreeChlorineResidual),
            b"CRL" => Some(CompressionRelaxation),
            b"CRN" => Some(Crown),
            b"CRT" => Some(TotalChlorineResidual),
            b"CS" => Some(CrossSection),
            b"CSC" => Some(ChokeSizeCasing),
            b"CSR" => Some(CostRealism),
            b"CST" => Some(ChokeSizeTubing),
            b"CT" => Some(CenterToCenter),
            b"CTG" => Some(Coating),
            b"CTT" => Some(ContactTime),
            b"CU" => Some(CoilCurvature),
            b"CUT" => Some(CureTime),
            b"CW" => Some(CuttableWidth),
            b"CWT" => Some(ChargeWeight),
            b"CX" => Some(CalculatedValue),
            b"CY" => Some(Contamination),
            b"CYB" => Some(CodeCYB),
            b"D" => Some(DestinationWeightAgreement),
            b"D1" => Some(MaximumDilatation),
            b"D2" => Some(DispersingAgent),
            b"D3" => Some(DryingAgent),
            b"D4" => Some(DryPoint),
            b"D5" => Some(Wear),
            b"D6" => Some(Horizontal),
            b"D7" => Some(DistillationFraction),
            b"D8" => Some(Vertical),
            b"D9" => Some(DotsPerInch),
            b"DA" => Some(DeltaValueA),
            b"DAT" => Some(DatumDepth),
            b"DB" => Some(DeltaValueB),
            b"DC" => Some(DuctileClass),
            b"DCT" => Some(DirtCount),
            b"DD" => Some(DepthOfDents),
            b"DE" => Some(Defects),
            b"DEM" => Some(DeMinimisLevel),
            b"DF" => Some(DistanceAcrossFlats),
            b"DG" => Some(DirectCurrent),
            b"DH" => Some(Dysprosium),
            b"DI" => Some(Diameter),
            b"DIL" => Some(CodeDIL),
            b"DIR" => Some(DistillationRange),
            b"DIS" => Some(Dispersion),
            b"DJ" => Some(DominantWaveLength),
            b"DK" => Some(ColorBitsInPalette),
            b"DL" => Some(DeltaValueL),
            b"DLC" => Some(DialCount),
            b"DM" => Some(DualAmplitude),
            b"DME" => Some(DichloromethaneExtract),
            b"DMF" => Some(DistributedMeterFactor),
            b"DN" => Some(Density),
            b"DO" => Some(CompressedFileSize),
            b"DOC" => Some(DoubleOlsenCup),
            b"DP" => Some(Depth),
            b"DPM" => Some(DegreeOfPolymerization),
            b"DR" => Some(DeltaR),
            b"DRY" => Some(Dryness),
            b"DS" => Some(DistanceBetweenPoints),
            b"DT" => Some(DistanceFromBasePoint),
            b"DU" => Some(DrawTension),
            b"DW" => Some(CodeDW),
            b"DWP" => Some(DewPoint),
            b"DY" => Some(Dyeability),
            b"E" => Some(EstimatedNewWeight),
            b"E0" => Some(ExtraneousMatter),
            b"E1" => Some(End),
            b"EA" => Some(Elongation),
            b"EB" => Some(EdgeBurr),
            b"EC" => Some(EnglishCoilDimensions),
            b"ED" => Some(Eddy),
            b"EE" => Some(Einsteinium),
            b"EF" => Some(Europium),
            b"EG" => Some(EquivalentTemperature),
            b"EH" => Some(Erbium),
            b"EI" => Some(Expansion),
            b"EJ" => Some(ElectricalConductivity),
            b"EL" => Some(Elasticity),
            b"ELC" => Some(ExposureCeilingThresholdLimitValue),
            b"ELE" => Some(Elevation),
            b"ELI" => Some(ImmediatelyDangerousToLifeAndHealth),
            b"ELL" => Some(ExplosionLimit),
            b"ELO" => Some(CodeELO),
            b"ELP" => Some(CodeELP),
            b"ELS" => Some(CodeELS),
            b"ELT" => Some(CodeELT),
            b"ELV" => Some(UnshieldedExposureRate),
            b"ELW" => Some(CodeELW),
            b"ELX" => Some(CodeELX),
            b"EM" => Some(ElmendorfTear),
            b"EN" => Some(Entanglement),
            b"EP" => Some(ExcitingPower),
            b"EPL" => Some(ExpectedProductLife),
            b"ES" => Some(Edge),
            b"ET" => Some(EndPoint),
            b"EVL" => Some(EvaporationLoss),
            b"EVR" => Some(EvaporationRate),
            b"EW" => Some(EmptyWeight),
            b"EX" => Some(Eccentricity),
            b"EXH" => Some(ExhaustBenzeneEmissions),
            b"EXT" => Some(Extractables),
            b"F" => Some(DeficitWeight),
            b"F1" => Some(FirePoint),
            b"F2" => Some(RadiatedPower),
            b"F3" => Some(CodeF3),
            b"F4" => Some(HeightAboveAverageTerrain),
            b"F5" => Some(GroundElevation),
            b"F6" => Some(HeightToTip),
            b"F7" => Some(RadiusFromALocation),
            b"F8" => Some(RadiusFromCoordinates),
            b"F9" => Some(OperatingHours),
            b"FA" => Some(Fluorine),
            b"FB" => Some(Flare),
            b"FBP" => Some(AcetateBreakPoint),
            b"FC" => Some(ShortCycleFlatness),
            b"FD" => Some(FrequencyOfOperation),
            b"FE" => Some(Fermium),
            b"FF" => Some(Francium),
            b"FG" => Some(FreezingPoint),
            b"FH" => Some(Finish),
            b"FI" => Some(FilamentCount),
            b"FIL" => Some(FilterNumber),
            b"FIN" => Some(Fineness),
            b"FIT" => Some(Filterability),
            b"FJ" => Some(FaceWidth),
            b"FK" => Some(FluidConsistency),
            b"FL" => Some(LongitudinalFlatness),
            b"FLD" => Some(FluidPoint),
            b"FLN" => Some(FiberLength),
            b"FLP" => Some(FluidLevelAbovePump),
            b"FLT" => Some(Fluting),
            b"FLV" => Some(FlavorThreshold),
            b"FML" => Some(FlammabilityLimits),
            b"FMZ" => Some(Flammability),
            b"FN" => Some(Flatness),
            b"FNL" => Some(FinesRetainedOnScreen),
            b"FNS" => Some(Fines),
            b"FOA" => Some(Foam),
            b"FOH" => Some(FrontOverHangOfVehicle),
            b"FOI" => Some(FoulingIndex),
            b"FOR" => Some(ForeignMatter),
            b"FP" => Some(Flashpoint),
            b"FPV" => Some(AcetatePluggingValue),
            b"FQ" => Some(Frequency),
            b"FR" => Some(FlowRate),
            b"FS" => Some(FoldStrength),
            b"FSI" => Some(FreeSwellingIndex),
            b"FT" => Some(FlangeThickness),
            b"FU" => Some(Azimuth),
            b"FUD" => Some(FullLoad),
            b"FV" => Some(TransverseFlatness),
            b"FW" => Some(FlangeWidth),
            b"FX" => Some(Filler),
            b"FY" => Some(Beam),
            b"FZ" => Some(CodeFZ),
            b"G" => Some(GrossWeight),
            b"G1" => Some(Gutter),
            b"G2" => Some(GrainSize),
            b"G3" => Some(Tilt),
            b"G4" => Some(GForce),
            b"GA" => Some(Grain),
            b"GB" => Some(GuidedBendsRoot),
            b"GC" => Some(Gadolinium),
            b"GD" => Some(Gold),
            b"GE" => Some(GuidedBendsFace),
            b"GEL" => Some(Gel),
            b"GF" => Some(GuidedBendsSide),
            b"GG" => Some(Gauge),
            b"GGR" => Some(GasGravity),
            b"GH" => Some(Gallium),
            b"GI" => Some(CodeGI),
            b"GIR" => Some(GasInjectionTestRate),
            b"GJ" => Some(CodeGJ),
            b"GK" => Some(TintingStrength),
            b"GL" => Some(Gloss),
            b"GLE" => Some(GroundLevelElevation),
            b"GM" => Some(EmulsionGloss),
            b"GN" => Some(GrossToNetConversionFactor),
            b"GO" => Some(ParticleEndPoint),
            b"GOR" => Some(GasToOilRatio),
            b"GP" => Some(GroupPackageSeparation),
            b"GQ" => Some(ScatteredParticles),
            b"GR" => Some(Gravity),
            b"GRA" => Some(GasTestRate),
            b"GRI" => Some(Graininess),
            b"GS" => Some(CarbonBlackUndertone),
            b"GT" => Some(GlassTransitionTemperature),
            b"GW" => Some(CodeGW),
            b"H" => Some(GroundWaterReferencePoint),
            b"H1" => Some(Heavies),
            b"H2O" => Some(WaterVolume),
            b"H8" => Some(NetExplosiveWeight),
            b"H9" => Some(RecommendedExposureLimit),
            b"HA" => Some(HydroPressure),
            b"HAR" => Some(HardeningRate),
            b"HAZ" => Some(Haze),
            b"HB" => Some(HeavyAluminas),
            b"HC" => Some(Helium),
            b"HCG" => Some(HorizontalCenterOfGravity),
            b"HCH" => Some(DistanceToEndpoint),
            b"HCI" => Some(EmissionFactor),
            b"HCJ" => Some(HeatContent),
            b"HCK" => Some(PollutantEmission),
            b"HCL" => Some(Population),
            b"HCM" => Some(ReleaseDuration),
            b"HCN" => Some(ReleasedQuantity),
            b"HCO" => Some(ReleaseRate),
            b"HCP" => Some(SulfurContent),
            b"HCQ" => Some(WindSpeed),
            b"HCR" => Some(AcuteToxicity),
            b"HCS" => Some(ChronicToxicity),
            b"HCT" => Some(Discharges),
            b"HCU" => Some(Effectiveness),
            b"HCV" => Some(Fertilization),
            b"HCW" => Some(SamplesInCompliance),
            b"HCX" => Some(Toxicity),
            b"HD" => Some(HighPropagationDelayTime),
            b"HE" => Some(HeavySilicates),
            b"HF" => Some(Hardness),
            b"HG" => Some(HeavySulfides),
            b"HH" => Some(HeavyGlobularOxides),
            b"HHW" => Some(HemisphericalPoint),
            b"HI" => Some(Hafnium),
            b"HIB" => Some(HighBoilers),
            b"HJ" => Some(Holmium),
            b"HK" => Some(HumanFactors),
            b"HL" => Some(HeatLoss),
            b"HM" => Some(CodeHM),
            b"HO" => Some(Holes),
            b"HOC" => Some(HeatOfCombustion),
            b"HP" => Some(HeightOfTreadPlatePattern),
            b"HR" => Some(HeightOfRunners),
            b"HT" => Some(Height),
            b"HTE" => Some(HeatEquivalency),
            b"HVM" => Some(HeavyMetals),
            b"HWS" => Some(SofteningPoint),
            b"HYD" => Some(HydroxylNumber),
            b"HZ" => Some(Hazepoint),
            b"HZC" => Some(HazardousComponent),
            b"I" => Some(HeatInput),
            b"IA" => Some(ImperfectionsThicks),
            b"IB" => Some(ImpactEnergy),
            b"IC" => Some(IncrementalDistance),
            b"ID" => Some(InsideDiameter),
            b"IDE" => Some(Identification),
            b"IE" => Some(ImperfectionsThins),
            b"IF" => Some(InputLowVoltage),
            b"IG" => Some(GranulatedIngotSize),
            b"IGA" => Some(AutoignitionTemperature),
            b"IGR" => Some(InputGasRate),
            b"IH" => Some(InputCurrent),
            b"IHV" => Some(InherentViscosity),
            b"II" => Some(InputSetupTime),
            b"IJ" => Some(InputHoldTime),
            b"IK" => Some(Indium),
            b"IL" => Some(ImperfectionsNeps),
            b"IM" => Some(Iridium),
            b"IMA" => Some(ImpactAdhesion),
            b"IMP" => Some(Impurities),
            b"IN" => Some(InputLowCurrent),
            b"IND" => Some(InitialDeformationPoint),
            b"INS" => Some(Insolubles),
            b"IO" => Some(InputHighCurrent),
            b"IP" => Some(InputLeakageCurrent),
            b"IPI" => Some(PrimaryIrritationIndex),
            b"IQ" => Some(InputClampDiodeVoltage),
            b"IR" => Some(IdealDiameter),
            b"IRA" => Some(InclusionRating),
            b"IS" => Some(InputHighVoltage),
            b"IT" => Some(InterruptPulsePeriod),
            b"ITD" => Some(IntegratedDifferential),
            b"IU" => Some(InterruptSetupTime),
            b"IV" => Some(Iodine),
            b"IW" => Some(IncrustationFactor),
            b"IX" => Some(CodeIX),
            b"IXD" => Some(IndexDifferential),
            b"IY" => Some(Induction),
            b"IZ" => Some(IngotPoundSize),
            b"JA" => Some(JunctionTemperature),
            b"JOM" => Some(JominyHardenability),
            b"KA" => Some(Knoop),
            b"KB" => Some(Krypton),
            b"KN" => Some(CodeKN),
            b"KVL" => Some(StrengthCoefficient),
            b"L" => Some(LegalWeight),
            b"L0" => Some(LeafGrade),
            b"L1" => Some(Lights),
            b"LA" => Some(Lanthanum),
            b"LAI" => Some(LanglierIndex),
            b"LB" => Some(Lithium),
            b"LC" => Some(LowPropagationDelayTime),
            b"LC5" => Some(CodeLC5),
            b"LCG" => Some(LongitudinalCenterOfGravity),
            b"LD" => Some(Lawrencium),
            b"LD5" => Some(CodeLD5),
            b"LDH" => Some(CodeLDH),
            b"LE" => Some(Lutetium),
            b"LEF" => Some(Leaf),
            b"LF" => Some(LongFibers),
            b"LG" => Some(Leg),
            b"LIR" => Some(LiquidInjectionTestRate),
            b"LIV" => Some(LimitIntrinsicViscosity),
            b"LL" => Some(LongLeg),
            b"LLD" => Some(LightLoad),
            b"LM" => Some(CodeLM),
            b"LN" => Some(Length),
            b"LO" => Some(LongLength),
            b"LOI" => Some(LossOnIgnition),
            b"LOS" => Some(LossOnDrying),
            b"LOW" => Some(LowBoilers),
            b"LP" => Some(CamberOfPatternLine),
            b"LPG" => Some(LiquefiedPetroleumGasFactor),
            b"LPL" => Some(LabeledProductLife),
            b"LPR" => Some(LinePressure),
            b"LS" => Some(ShortLeg),
            b"LSA" => Some(LockSeamAdhesion),
            b"LSK" => Some(LengthShrinkage),
            b"LSS" => Some(LinearSheetSwelling),
            b"LT" => Some(LengthwiseSpacing),
            b"LTD" => Some(LinerTopDepth),
            b"LW" => Some(LongWidth),
            b"M" => Some(CodeM),
            b"M1" => Some(MeltRange),
            b"M2" => Some(MaximumDifferentialPressure),
            b"M3" => Some(MaximumStaticPressure),
            b"M4" => Some(Area),
            b"M5" => Some(MinimumSpeed),
            b"M6" => Some(MaximumSpeed),
            b"MA" => Some(MeanAverage),
            b"MAT" => Some(Maturity),
            b"MB" => Some(Mendelevium),
            b"MC" => Some(MinMaxCuttableWidth),
            b"MCN" => Some(MagneticContamination),
            b"MD" => Some(MeasurementVoltage),
            b"MDL" => Some(CodeMDL),
            b"ME" => Some(MaximumInputLowVoltage),
            b"MEA" => Some(MediaDepth),
            b"MEF" => Some(MeterFactor),
            b"MEL" => Some(MeltTime),
            b"MEP" => Some(MeatProtein),
            b"MER" => Some(EfficientRateReservoir),
            b"MF" => Some(MinimumInputHighVoltage),
            b"MG" => Some(Mercury),
            b"MH" => Some(MeltingPoint),
            b"MHI" => Some(HighestTorque),
            b"MI" => Some(Minimum),
            b"MIC" => Some(Micronaire),
            b"MIL" => Some(MilkFat),
            b"MJ" => Some(CodeMJ),
            b"MK" => Some(CodeMK),
            b"MM" => Some(Management),
            b"MN" => Some(MinimumAverage),
            b"MO" => Some(Mottles),
            b"MOI" => Some(Moisture),
            b"MOR" => Some(Mortality),
            b"MP" => Some(MullenPop),
            b"MPR" => Some(ProductionRateWell),
            b"MPT" => Some(ManyPressTest),
            b"MQ" => Some(McQuaid),
            b"MR" => Some(CodeMR),
            b"MS" => Some(CodeMS),
            b"MT" => Some(MoistureContent),
            b"MTD" => Some(MaximumTotalDepth),
            b"MU" => Some(Multiplier),
            b"MUL" => Some(Mullen),
            b"MV" => Some(MaximumAverage),
            b"MW" => Some(MolecularWeight),
            b"MX" => Some(Maximum),
            b"MY" => Some(MagnetizingField),
            b"N" => Some(ActualNetWeight),
            b"NA" => Some(NumberPerPackage),
            b"NB" => Some(NumberPerBundle),
            b"NC" => Some(NumberPerCoilGroup),
            b"ND" => Some(Neodymium),
            b"NEU" => Some(NeutralizationNumber),
            b"NF" => Some(Neon),
            b"NG" => Some(Nobelium),
            b"NH" => Some(NumberOfItemsPerPackageLabel),
            b"NI" => Some(NumberOfSplicesPerPackageLabel),
            b"NIL" => Some(NilDuctilityTest),
            b"NJ" => Some(NumberOfSheetsPerPackageLabel),
            b"NK" => Some(NestingFactor),
            b"NL" => Some(NumberPerLift),
            b"NM" => Some(NumberPkgsPerMasterPack),
            b"NNW" => Some(NetNetWeight),
            b"NO" => Some(CodeNO),
            b"NOC" => Some(NumberOfCosigners),
            b"NON" => Some(NonVolatileMatter),
            b"NOR" => Some(NumberOfReferences),
            b"NOX" => Some(NOxEmissionsPerformance),
            b"NP" => Some(PercentOfSpecified),
            b"NS" => Some(NumberPerSkid),
            b"NU" => Some(NumberPerUnit),
            b"NV" => Some(NValue),
            b"O" => Some(ExcessWeightOverMaximum),
            b"O1" => Some(OrificeInsideDiameter),
            b"OA" => Some(Offset),
            b"OAP" => Some(ObservedAmericanPetroleumInstituteGravity),
            b"OB" => Some(Osmium),
            b"OBT" => Some(ObservedTemperature),
            b"OC" => Some(OutputLowVoltage),
            b"OCG" => Some(OilCondensateGravity),
            b"OCR" => Some(OilCondensateTestRate),
            b"OD" => Some(OutsideDiameter),
            b"ODR" => Some(Odor),
            b"OE" => Some(OutputLowCurrent),
            b"OF" => Some(OutputHighVoltage),
            b"OG" => Some(OutputHighCurrent),
            b"OH" => Some(CodeOH),
            b"OI" => Some(OutputOffCurrentLow),
            b"OIL" => Some(Oil),
            b"OJ" => Some(OutputOffCurrentHigh),
            b"OK" => Some(OutputShortCircuitCurrent),
            b"OL" => Some(OutputDisableTimeFromLowLevelOfA3StateOutput),
            b"OLE" => Some(Olefins),
            b"OM" => Some(CodeOM),
            b"ON" => Some(OutputDisableTimeFromHighLevelOfA3StateOutput),
            b"OO" => Some(OutputEnableTimeFromLowLevelOfA3StateOutput),
            b"OP" => Some(Openness),
            b"OQ" => Some(OutputEnableTimeFromHighLevelOfA3StateOutput),
            b"OR" => Some(DistanceBetweenOutsideRunners),
            b"ORC" => Some(OrganicCarbon),
            b"OS" => Some(OpenCircuits),
            b"OT" => Some(OutputDelayTime),
            b"OTE" => Some(OthersEach),
            b"OTH" => Some(OdorThreshold),
            b"OTT" => Some(OthersTotal),
            b"OV" => Some(Opacity),
            b"OW" => Some(OverallWidth),
            b"OX" => Some(OwnershipShare),
            b"OXI" => Some(OxidizableSubstance),
            b"OXS" => Some(OxidizingSubstance),
            b"OY" => Some(OperatingWeight),
            b"P1" => Some(Price),
            b"PA" => Some(PackageSeparation),
            b"PAR" => Some(ParticleSize),
            b"PB" => Some(Pressure),
            b"PBD" => Some(PlugBackTotalDepth),
            b"PC" => Some(PerHundredLinearYards),
            b"PD" => Some(Platinum),
            b"PDE" => Some(CasingLinerTubingDepth),
            b"PDG" => Some(PumpDepthFromGround),
            b"PE" => Some(Potassium),
            b"PER" => Some(MagneticPermeability),
            b"PF" => Some(Promethium),
            b"PFO" => Some(PerforationFeetOpen),
            b"PG" => Some(Polonium),
            b"PH" => Some(PulseSetupTime),
            b"PHA" => Some(PH),
            b"PHW" => Some(HardwoodFiber),
            b"PI" => Some(PulseHoldTime),
            b"PIC" => Some(PickOff),
            b"PJ" => Some(PulseWidth),
            b"PK" => Some(PulseRecoveryTime),
            b"PL" => Some(PercentDefective),
            b"PM" => Some(Practice),
            b"PN" => Some(Palladium),
            b"PO" => Some(CodePO),
            b"POC" => Some(Completion),
            b"POD" => Some(PhysicalDescriptionOuterDiameter),
            b"POP" => Some(PourPoint),
            b"PP" => Some(PowderPastePackageSize),
            b"PPS" => Some(ProprietaryShade),
            b"PQ" => Some(Plutonium),
            b"PQL" => Some(CodePQL),
            b"PR" => Some(Praseodymium),
            b"PRA" => Some(ProportionAlive),
            b"PRE" => Some(PriorExperience),
            b"PRF" => Some(PressureFactor),
            b"PRI" => Some(ProductIndex),
            b"PRL" => Some(ProductLevel),
            b"PRN" => Some(ProportionNormal),
            b"PRO" => Some(Processability),
            b"PRQ" => Some(ProductReportableQuantity),
            b"PRY" => Some(Porosity),
            b"PRZ" => Some(ProportionFertilized),
            b"PS" => Some(Protactinium),
            b"PSA" => Some(PercentSolutionActual),
            b"PSP" => Some(PastPerformance),
            b"PSW" => Some(SoftwoodFiber),
            b"PT" => Some(Pits),
            b"PU" => Some(PressureBase),
            b"PV" => Some(Picks),
            b"PW" => Some(PurchasedWidth),
            b"PWA" => Some(ProcessedWaste),
            b"PWE" => Some(PhysicalDescriptionWeight),
            b"PWF" => Some(PowerFactor),
            b"PX" => Some(Purity),
            b"PY" => Some(PercentOfWater),
            b"PZ" => Some(PipeSizeNominal),
            b"Q" => Some(VolatileOrganicCompoundsPlusWater),
            b"QA" => Some(QualityIndex),
            b"QB" => Some(QuantityOrLoadingAverage),
            b"QC" => Some(QuantityOrLoadingMaximum),
            b"QD" => Some(QualityOrConcentrationAverage),
            b"QE" => Some(QualityOrConcentrationMinimum),
            b"QF" => Some(QualityOrConcentrationMaximum),
            b"QG" => Some(Duration),
            b"QI" => Some(Abundance),
            b"QJ" => Some(Biomass),
            b"QK" => Some(SizeClass),
            b"QL" => Some(Quality),
            b"QUR" => Some(ReportableQuantity),
            b"R" => Some(PerUnitDunnage),
            b"R1" => Some(Hemoglobin),
            b"R2" => Some(Hematocrit),
            b"R3" => Some(EpoetinStartingDosage),
            b"R4" => Some(Creatinine),
            b"R7" => Some(Speed),
            b"R8" => Some(SpeedLimit),
            b"R10" => Some(RelativeFractionOfPureLongChainCellulose),
            b"R18" => Some(RelativeFractionOfTotalCellulose),
            b"RA" => Some(RelativeHumidity),
            b"RAD" => Some(Radius),
            b"RAF" => Some(RoofAdjustmentFactor),
            b"RB" => Some(RangeValue),
            b"RC" => Some(RadiusOfCorner),
            b"RD" => Some(Readpoint),
            b"RE" => Some(ReamWeight),
            b"REA" => Some(Reactivity),
            b"RED" => Some(ReducingSubstance),
            b"REF" => Some(Refining),
            b"REI" => Some(RefractiveIndex),
            b"REL" => Some(Reflectance),
            b"RES" => Some(Resistance),
            b"RF" => Some(Resistivity),
            b"RG" => Some(Radium),
            b"RH" => Some(Rhenium),
            b"RI" => Some(Rubidium),
            b"RJ" => Some(RockwellC),
            b"RK" => Some(RockwellB),
            b"RL" => Some(ReductionRation),
            b"RM" => Some(CodeRM),
            b"RN" => Some(RequiredInterruptRelease),
            b"RO" => Some(ResetPulseWidth),
            b"ROH" => Some(RearOverHangOfVehicle),
            b"ROX" => Some(OxygenFromARenewableOxygenate),
            b"RP" => Some(ReductionOfArea),
            b"RQ" => Some(Radon),
            b"RR" => Some(ReductionRatio),
            b"RS" => Some(CodeRS),
            b"RSZ" => Some(RollSize),
            b"RT" => Some(RoundsAmmunitionMilitary),
            b"RTB" => Some(ReportingTemperatureBase),
            b"RU" => Some(Rhodium),
            b"RUD" => Some(CodeRUD),
            b"RV" => Some(Ruthenium),
            b"RVP" => Some(ReidVaporPressure),
            b"RW" => Some(RollingWidth),
            b"RX" => Some(Ridges),
            b"RY" => Some(Ratio),
            b"S" => Some(StateWeight),
            b"S1" => Some(Smoothness),
            b"S2" => Some(SelvedgeOnBeam),
            b"S3" => Some(SheffieldSmoothness),
            b"S4" => Some(SurfaceStrength),
            b"S5" => Some(Stiffness),
            b"S6" => Some(Saturation),
            b"S7" => Some(Sediment),
            b"S8" => Some(Solubility),
            b"S9" => Some(SiteAtmosphericPressure),
            b"S10" => Some(PulpImpurities),
            b"S12" => Some(Start),
            b"S18" => Some(Hemicellulose),
            b"SA" => Some(SortCodeCieLab),
            b"SAL" => Some(CodeSAL),
            b"SAP" => Some(SaponificationNumber),
            b"SB" => Some(SortCodeCmc),
            b"SC" => Some(CodeSC),
            b"SCH" => Some(Schedule),
            b"SD" => Some(Strength),
            b"SE" => Some(SelvageLeft),
            b"SEV" => Some(Severity),
            b"SF" => Some(Samarium),
            b"SFC" => Some(ShortFiberContent),
            b"SG" => Some(SlitWidth),
            b"SH" => Some(Strontium),
            b"SHA" => Some(ShelfLife),
            b"SI" => Some(SupplyCurrent),
            b"SIL" => Some(CodeSIL),
            b"SJ" => Some(ShortCircuits),
            b"SK" => Some(Shrinkage),
            b"SL" => Some(ShortLength),
            b"SLD" => Some(Solderability),
            b"SLI" => Some(SlaggingIndex),
            b"SM" => Some(Shear),
            b"SMB" => Some(SamBRating),
            b"SMD" => Some(SamDRating),
            b"SN" => Some(Stain),
            b"SO" => Some(SortCodeCieLch),
            b"SOD" => Some(Solids),
            b"SOF" => Some(SofteningRange),
            b"SP" => Some(SplinterCount),
            b"SPG" => Some(SpecificGravity),
            b"SPH" => Some(Sphere),
            b"SPR" => Some(SeparatorPressure),
            b"SPS" => Some(StaticPressure),
            b"SQ" => Some(ShippedQuantity),
            b"SR" => Some(SelvageRight),
            b"SS" => Some(Silver),
            b"ST" => Some(StopRecoveryStartupTime),
            b"STA" => Some(Stability),
            b"STL" => Some(ShortTermExposureLimit),
            b"STP" => Some(Staple),
            b"SU" => Some(ShippedUnits),
            b"SUM" => Some(SuspendedMatter),
            b"SUR" => Some(SurfaceRoughness),
            b"SUT" => Some(SurfaceTension),
            b"SV" => Some(Scandium),
            b"SVL" => Some(Survival),
            b"SW" => Some(ShortWidth),
            b"SX" => Some(Sodium),
            b"SXX" => Some(S10MinusS18Value),
            b"SY" => Some(ServiceInterruptDuration),
            b"SZ" => Some(SkidHeight),
            b"T" => Some(TareWeight),
            b"T1" => Some(TirePressure),
            b"T2" => Some(TubeInsideDiameter),
            b"T3" => Some(Technical),
            b"T4" => Some(SingleEndBreak),
            b"T5" => Some(SkeinBreak),
            b"T50" => Some(T50),
            b"T90" => Some(T90),
            b"TA" => Some(CodeTA),
            b"TAS" => Some(Taste),
            b"TB" => Some(CodeTB),
            b"TC" => Some(Temperature),
            b"TCL" => Some(TireTreadContactLength),
            b"TD" => Some(ThinAluminas),
            b"TDP" => Some(PerforationTopDepth),
            b"TE" => Some(Tenacity),
            b"TEE" => Some(AutodecompositionTemperature),
            b"TES" => Some(StorageTemperature),
            b"TEX" => Some(Texture),
            b"TF" => Some(Tensile),
            b"TG" => Some(ThinSulfides),
            b"TH" => Some(Thickness),
            b"TI" => Some(ThinSilicates),
            b"TJ" => Some(TotalSupplyCurrent),
            b"TK" => Some(TimerPulseWidth),
            b"TL" => Some(TaperedSteppedLengthType),
            b"TM" => Some(LengthTypeMultiples),
            b"TN" => Some(TimerPeriod),
            b"TO" => Some(Terbium),
            b"TOA" => Some(AquaticToxicity),
            b"TOR" => Some(Torque),
            b"TOX" => Some(ToxicEmissionsPerformance),
            b"TP" => Some(Thorium),
            b"TPF" => Some(TemperatureFactor),
            b"TPL" => Some(TubingPressureFlowing),
            b"TPQ" => Some(ThresholdPlanningQuantity),
            b"TPS" => Some(TubingPressureShutin),
            b"TQ" => Some(ThinGlobularOxides),
            b"TR" => Some(LengthTypeRandom),
            b"TRA" => Some(TrashArea),
            b"TRC" => Some(TrashCount),
            b"TRD" => Some(TireDiameter),
            b"TRN" => Some(Transmittance),
            b"TRS" => Some(CodeTRS),
            b"TRT" => Some(CodeTRT),
            b"TS" => Some(LengthTypeSpecific),
            b"TSZ" => Some(TrimSize),
            b"TT" => Some(Time),
            b"TTL" => Some(TrailerTongueLength),
            b"TU" => Some(Technetium),
            b"TUR" => Some(Turbidity),
            b"TV" => Some(Thallium),
            b"TVD" => Some(MaximumTrueVerticalDepth),
            b"TW" => Some(Top),
            b"TWD" => Some(TireWidth),
            b"TX" => Some(Thulium),
            b"TY" => Some(TearStrength),
            b"U" => Some(WeightPerUnit),
            b"UA" => Some(Uranium),
            b"UCB" => Some(Cube),
            b"UG" => Some(Usage),
            b"UNA" => Some(UnipunchAdhesion),
            b"UNI" => Some(Uniformity),
            b"UNK" => Some(Unknowns),
            b"V" => Some(OxygenationLevel),
            b"VAD" => Some(VaporDensity),
            b"VAP" => Some(VaporPressure),
            b"VBA" => Some(VBendAdhesion),
            b"VCG" => Some(VerticalCenterOfGravity),
            b"VH" => Some(CodeVH),
            b"VIN" => Some(Vinyl),
            b"VIS" => Some(Viscosity),
            b"VO" => Some(Voltage),
            b"VOC" => Some(VocEmissionsPerformance),
            b"VOL" => Some(Volume),
            b"VOT" => Some(Volatiles),
            b"VOV" => Some(VolatilesByVolume),
            b"VOW" => Some(VolatilesByWeight),
            b"VSO" => Some(VolumeSplitToOthers),
            b"VW" => Some(CodeVW),
            b"VWT" => Some(VolumeWeight),
            b"W" => Some(ReformulatedFuelLevel),
            b"WA" => Some(WeightPerUnitOfArea),
            b"WB" => Some(Web),
            b"WC" => Some(WebDepthHeight),
            b"WD" => Some(Width),
            b"WDE" => Some(WaterDepth),
            b"WE" => Some(Wolfram),
            b"WEL" => Some(WeightLoss),
            b"WF" => Some(WaitRecoveryStartupTime),
            b"WH" => Some(Whiteness),
            b"WI" => Some(WindingLoss),
            b"WL" => Some(WallThickness),
            b"WM" => Some(CodeWM),
            b"WOD" => Some(WaterOilDistributionCoefficient),
            b"WPF" => Some(WellheadPressureFlowing),
            b"WPL" => Some(WaterProductLevel),
            b"WPS" => Some(WellheadPressureShutin),
            b"WR" => Some(Wrinkles),
            b"WRA" => Some(WaterTestRate),
            b"WSK" => Some(WidthShrinkage),
            b"WT" => Some(Weight),
            b"WTB" => Some(WaterTankBottomLevel),
            b"WU" => Some(WeightPerUnitOfLength),
            b"WX" => Some(WaxPick),
            b"X" => Some(CodeX),
            b"XA" => Some(Xenon),
            b"XH" => Some(CodeXH),
            b"XP" => Some(Specified),
            b"XQ" => Some(Squareness),
            b"XZ" => Some(SpoolSize),
            b"YA" => Some(Yttrium),
            b"YB" => Some(Yield),
            b"YC" => Some(Ytterbium),
            b"YD" => Some(YarnCount),
            b"YPE" => Some(YieldPointElongation),
            b"ZAL" => Some(Aluminum),
            b"ZAS" => Some(Arsenic),
            b"ZB" => Some(Boron),
            b"ZBI" => Some(Bismuth),
            b"ZBT" => Some(NButane),
            b"ZBZ" => Some(Benzene),
            b"ZC" => Some(Carbon),
            b"ZCA" => Some(Calcium),
            b"ZCB" => Some(Columbium),
            b"ZCD" => Some(CarbonDioxide),
            b"ZCE" => Some(Cerium),
            b"ZCM" => Some(CarbonMonoxide),
            b"ZCO" => Some(Cobalt),
            b"ZCR" => Some(Chromium),
            b"ZCU" => Some(Copper),
            b"ZD" => Some(LoadFactor),
            b"ZET" => Some(Ethane),
            b"ZF" => Some(SulfateSulfur),
            b"ZFE" => Some(Iron),
            b"ZFL" => Some(NewspaperFullPage),
            b"ZFS" => Some(IronPlusSilicon),
            b"ZG" => Some(OrganicSulfur),
            b"ZGE" => Some(Germanium),
            b"ZH" => Some(Hydrogen),
            b"ZHP" => Some(Heptane),
            b"ZHS" => Some(HydrogenSulfide),
            b"ZHX" => Some(Hexane),
            b"ZIB" => Some(IButane),
            b"ZIP" => Some(IPentane),
            b"ZMG" => Some(Magnesium),
            b"ZMN" => Some(Manganese),
            b"ZMO" => Some(Molybdenum),
            b"ZMT" => Some(Methane),
            b"ZN" => Some(Nitrogen),
            b"ZNB" => Some(Niobium),
            b"ZNI" => Some(Nickel),
            b"ZNP" => Some(NeoPentane),
            b"ZO" => Some(Oxygen),
            b"ZOC" => Some(Octane),
            b"ZP" => Some(Phosphorous),
            b"ZPB" => Some(Lead),
            b"ZPP" => Some(Propane),
            b"ZPT" => Some(NPentane),
            b"ZR" => Some(PyriticSulfur),
            b"ZS" => Some(Sulfur),
            b"ZSB" => Some(Antimony),
            b"ZSE" => Some(Selenium),
            b"ZSI" => Some(Silicon),
            b"ZSN" => Some(Tin),
            b"ZTA" => Some(Tantalum),
            b"ZTB" => Some(NewspaperTabloidPage),
            b"ZTE" => Some(Tellurium),
            b"ZTI" => Some(Titanium),
            b"ZV" => Some(Vanadium),
            b"ZW" => Some(Tungsten),
            b"ZZN" => Some(Zinc),
            b"ZZR" => Some(Zirconium),
            b"ZZZ" => Some(MutuallyDefined),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use MeasurementQualifier::*;
        match self {
            ControlEfficiency => "Control Efficiency",
            RadioFrequency => "Radio Frequency",
            CaptureEfficiency => "Capture Efficiency",
            AlternateRadioFrequency => "Alternate Radio Frequency",
            PhotonfluxDensity => "Photonflux Density",
            TargetDepth => "Target Depth",
            CurrentDepth => "Current Depth",
            TotalDepth => "Total Depth",
            WellTestBeforeOil => "Well Test Before Oil",
            WellTestBeforeGas => "Well Test Before Gas",
            WellTestBeforeWater => "Well Test Before Water",
            WellTestAfterOil => "Well Test After Oil",
            WellTestAfterGas => "Well Test After Gas",
            WellTestAfterWater => "Well Test After Water",
            EstimatedDepthOfOperations => "Estimated Depth of Operations",
            ThroughputRate => "Throughput Rate",
            SquelchTone => "Squelch Tone",
            CloudCover => "Cloud Cover",
            HeightAboveGround => "Height above Ground",
            Velocity => "Velocity",
            Gain => "Gain",
            PlumeHeight => "Plume Height",
            Individuals => "Individuals",
            DirectionalHeightAboveAverageTerrain => {
                "Directional Height above Average Terrain"
            }
            StorageLimits => "Storage Limits",
            PaintingCosts => "Painting Costs",
            StructuralCosts => "Structural Costs",
            Appliances => "Appliances",
            Utilities => "Utilities",
            CarpetOrFloors => "Carpet or Floors",
            OtherRepairs => "Other Repairs",
            Landscaping => "Landscaping",
            Roof => "Roof",
            Windows => "Windows",
            CleaningOrTrashRemoval => "Cleaning or Trash Removal",
            ProbableSalesPrice => "Probable Sales Price",
            Proximity => "Proximity",
            RepairsAndImprovements => "Repairs and Improvements",
            ContributoryValueOfRepairsAndImprovements => {
                "Contributory Value of Repairs and Improvements"
            }
            MarketingTime => "Marketing Time",
            ClosedComparableSales => "Closed Comparable Sales",
            CompetitiveListingsInPriceRange => "Competitive Listings in Price Range",
            FinancingConcessions => "Financing Concessions",
            MarketingConcessions => "Marketing Concessions",
            ProbableNetPrice => "Probable Net Price",
            SuggestedInitialListPrice => "Suggested Initial List Price",
            ValueChange => "Value Change",
            ProbableFinalValue => "Probable Final Value",
            OccupancyRate => "Occupancy Rate",
            NumberOfLivingUnits => "Number of Living Units",
            NumberOfPhases => "Number of Phases",
            NumberOfActiveListings => "Number of Active Listings",
            PriceActiveListings => "Price Active Listings",
            PricePerGrossLivingArea => "Price Per Gross Living Area",
            BuiltUpRate => "Built-up Rate",
            VacantRate => "Vacant Rate",
            TypicalRents => "Typical Rents",
            NeighborhoodApartmentVacancy => "Neighborhood Apartment Vacancy",
            NumberOfAdmissions => "Number of Admissions",
            CostOfHire => "Cost of Hire",
            Frontage => "Frontage",
            GrossSales => "Gross Sales",
            NumberOfEmployees => "Number of Employees",
            Payroll => "Payroll",
            PerCapitaOrEach => "Per Capita or Each",
            Remuneration => "Remuneration",
            TotalCost => "Total Cost",
            TotalMileage => "Total Mileage",
            NumberOfRatingUnits => "Number of Rating Units",
            GarageEmployeePayrollMaximum => "Garage Employee Payroll Maximum",
            EmployeeGrossWageLessAllowableDeductions => {
                "Employee Gross Wage Less Allowable Deductions"
            }
            GarageEmployeeAverageHoursWorkedPerWeek => {
                "Garage Employee Average Hours Worked Per Week"
            }
            Code66 => "Garage (Dealers) Employee Weeks Worked",
            GrossWage => "Gross Wage",
            SubcontractorLaborAndMaterials => "Subcontractor - Labor and Materials",
            SubcontractorLaborOnly => "Subcontractor - Labor Only",
            ConsolidatedWeight => "Consolidated Weight",
            Acids => "Acids",
            Adsorption => "Adsorption",
            AgingTime => "Aging Time",
            Aromatics => "Aromatics",
            AverageDifferentialPressure => "Average Differential Pressure",
            AverageStaticPressure => "Average Static Pressure",
            FlameProjectionDistance => "Flame Projection Distance",
            Exposure => "Exposure",
            AlternatingCurrent => "Alternating Current",
            AcApparentPower => "AC-apparent Power",
            ActivationEnergy => "Activation Energy",
            Absorbance => "Absorbance",
            Abrasion => "Abrasion",
            Absorbency => "Absorbency",
            Actinium => "Actinium",
            AcidNumber => "Acid Number",
            AmbientTemperature => "Ambient Temperature",
            Adhesion => "Adhesion",
            DyeManufacturingUnits => "Dye Manufacturing Units",
            Argon => "Argon",
            AngleOfBend => "Angle of Bend",
            Americium => "Americium",
            InventoryAge => "Inventory Age",
            AggressiveIndex => "Aggressive Index",
            Astatine => "Astatine",
            Acidity => "Acidity",
            AimGage => "Aim Gage",
            CodeAK => "Volatile Organic Compounds (VOCs)",
            SpineShow => "Spine Show",
            Alkalinity => "Alkalinity",
            AlkalinityNumber => "Alkalinity Number",
            AlphaCellulose => "Alpha-Cellulose",
            AverageSpeed => "Average Speed",
            Amines => "Amines",
            AverageMolecularWeight => "Average Molecular Weight",
            FluteTest => "Flute Test",
            Antioxidant => "Antioxidant",
            AveragePressure => "Average Pressure",
            ApiGravity => "API Gravity",
            Appearance => "Appearance",
            AshFusionTemperature => "Ash Fusion Temperature",
            Ash => "Ash",
            Assay => "Assay",
            Additive => "Additive",
            NumberOfUnitsProjected => "Number of Units Projected",
            Age => "Age",
            AverageTemperature => "Average Temperature",
            RemainingEconomicLife => "Remaining Economic Life",
            RemainingPhysicalLife => "Remaining Physical Life",
            NumberOfComparableSales => "Number of Comparable Sales",
            ArborSize => "Arbor Size",
            BilledWeight => "Billed Weight",
            BaseNumber => "Base Number",
            NumberOfComparableListings => "Number of Comparable Listings",
            PresentLandUse => "Present Land Use",
            SubjectPhaseDwellingUnits => "Subject Phase Dwelling Units",
            OctanolWaterPartitionCoefficient => "Octanol/Water Partition Coefficient",
            TotalProjectDwellingUnits => "Total Project Dwelling Units",
            Barium => "Barium",
            Beryllium => "Beryllium",
            BilletSize => "Billet Size",
            Bias => "Bias",
            PerforationBottomDepth => "Perforation Bottom Depth",
            BoronFactor => "Boron Factor",
            Brinell => "Brinell",
            Berkelium => "Berkelium",
            Bromine => "Bromine",
            BottomholePressureFlowing => "Bottomhole Pressure - Flowing",
            BottomholePressureShutin => "Bottomhole Pressure - Shutin",
            BarkInChips => "Bark in Chips",
            BurstIndex => "Burst Index",
            Bulk => "Bulk",
            Blisters => "Blisters",
            Bend => "Bend",
            AmountBoundInMaterial => "Amount Bound in Material",
            CodeBO => "Lateral Bow (Camber)",
            BoilingRange => "Boiling Range",
            BoilingPoint => "Boiling Point",
            Breaks => "Breaks",
            Brightness => "Brightness",
            BreakingStrength => "Breaking Strength",
            PercentBottomSedimentAndWater => "Percent Bottom Sediment and Water",
            Bursts => "Bursts",
            Buckles => "Buckles",
            BulkDensity => "Bulk Density",
            BasisWeight => "Basis Weight",
            BloodAlcohol => "Blood Alcohol",
            ActualNewRepeatedForCombination => "Actual New Repeated for Combination",
            ColorGrade => "Color Grade",
            Carbonyl => "Carbonyl",
            Catalyst => "Catalyst",
            MaximumContraction => "Maximum Contraction",
            ColorQuadrant => "Color Quadrant",
            Caliper => "Caliper",
            CausticReactionSeverity => "Caustic Reaction Severity",
            Celsius => "Celsius",
            CompositeCorrectedFactor => "Composite Corrected Factor",
            CombinedCenterOfGravity => "Combined Center of Gravity",
            Compression => "Compression",
            CapacitanceIn => "Capacitance In",
            CapacitanceOut => "Capacitance Out",
            Cadmium => "Cadmium",
            ColorGraynessRd => "Color Grayness RD",
            Cesium => "Cesium",
            ChemicalAdditionRate => "Chemical Addition Rate",
            ChlorophyllA => "Chlorophyll-a",
            ConcentrationOfHazardousComponent => "Concentration of Hazardous Component",
            RateOfChange => "Rate of Change",
            Chlorides => "Chlorides",
            Curium => "Curium",
            CueneIntrinsicViscosity => "Cuene Intrinsic Viscosity",
            CycleTime => "Cycle Time",
            Californium => "Californium",
            CodeCL => "Cladding Thickness(% of Composite Thickness)",
            Clarity => "Clarity",
            Calibration => "Calibration",
            Cleanliness => "Cleanliness",
            Cures => "Cures",
            Chlorine => "Chlorine",
            CoreLoss => "Core Loss",
            CoefficientFactor => "Coefficient Factor",
            CoerciveForce => "Coercive Force",
            Color => "Color",
            Concentration => "Concentration",
            Corrosiveness => "Corrosiveness",
            Cost => "Cost",
            Content => "Content",
            Crimp => "Crimp",
            CasingPressureFlowing => "Casing Pressure - Flowing",
            CasingPressureShutin => "Casing Pressure - Shutin",
            CupTestAdhesion => "Cup Test Adhesion",
            Cuts => "Cuts",
            CrosswiseSpacing => "Crosswise Spacing",
            FreeChlorineResidual => "Free Chlorine Residual",
            CompressionRelaxation => "Compression Relaxation",
            Crown => "Crown",
            TotalChlorineResidual => "Total Chlorine Residual",
            CrossSection => "Cross Section",
            ChokeSizeCasing => "Choke Size - Casing",
            CostRealism => "Cost Realism",
            ChokeSizeTubing => "Choke Size - Tubing",
            CenterToCenter => "Center-to-center",
            Coating => "Coating",
            ContactTime => "Contact Time",
            CoilCurvature => "Coil Curvature",
            CureTime => "Cure Time",
            CuttableWidth => "Cuttable Width",
            ChargeWeight => "Charge Weight",
            CalculatedValue => "Calculated Value",
            Contamination => "Contamination",
            CodeCYB => "Color Yellowness (+B)",
            DestinationWeightAgreement => "Destination Weight Agreement",
            MaximumDilatation => "Maximum Dilatation",
            DispersingAgent => "Dispersing Agent",
            DryingAgent => "Drying Agent",
            DryPoint => "Dry Point",
            Wear => "Wear",
            Horizontal => "Horizontal",
            DistillationFraction => "Distillation Fraction",
            Vertical => "Vertical",
            DotsPerInch => "Dots per Inch",
            DeltaValueA => "Delta Value A",
            DatumDepth => "Datum Depth",
            DeltaValueB => "Delta Value B",
            DuctileClass => "Ductile Class",
            DirtCount => "Dirt Count",
            DepthOfDents => "Depth of Dents",
            Defects => "Defects",
            DeMinimisLevel => "De Minimis Level",
            DistanceAcrossFlats => "Distance Across Flats",
            DirectCurrent => "Direct Current",
            Dysprosium => "Dysprosium",
            Diameter => "Diameter",
            CodeDIL => {
                "Dilution Factor; An amount by which a quantity is diluted in order to be read on an instrument scale"
            }
            DistillationRange => "Distillation Range",
            Dispersion => "Dispersion",
            DominantWaveLength => "Dominant Wave Length",
            ColorBitsInPalette => "Color Bits in Palette",
            DeltaValueL => "Delta Value L",
            DialCount => "Dial Count",
            DualAmplitude => "Dual Amplitude",
            DichloromethaneExtract => "Dichloromethane Extract",
            DistributedMeterFactor => "Distributed Meter Factor",
            Density => "Density",
            CompressedFileSize => "Compressed File Size",
            DoubleOlsenCup => "Double Olsen Cup",
            Depth => "Depth",
            DegreeOfPolymerization => "Degree of Polymerization",
            DeltaR => "Delta R",
            Dryness => "Dryness",
            DistanceBetweenPoints => "Distance Between Points",
            DistanceFromBasePoint => "Distance From Base Point",
            DrawTension => "Draw Tension",
            CodeDW => "Width, Boxcar Door",
            DewPoint => "Dew Point",
            Dyeability => "Dyeability",
            EstimatedNewWeight => "Estimated New Weight",
            ExtraneousMatter => "Extraneous Matter",
            End => "End",
            Elongation => "Elongation",
            EdgeBurr => "Edge Burr",
            EnglishCoilDimensions => "English Coil Dimensions",
            Eddy => "Eddy",
            Einsteinium => "Einsteinium",
            Europium => "Europium",
            EquivalentTemperature => "Equivalent Temperature",
            Erbium => "Erbium",
            Expansion => "Expansion",
            ElectricalConductivity => "Electrical Conductivity",
            Elasticity => "Elasticity",
            ExposureCeilingThresholdLimitValue => {
                "Exposure Ceiling: Threshold Limit Value"
            }
            Elevation => "Elevation",
            ImmediatelyDangerousToLifeAndHealth => {
                "Immediately Dangerous to Life and Health"
            }
            ExplosionLimit => "Explosion Limit",
            CodeELO => {
                "Occupational Safety and Health Administration (OSHA) Permissible Exposure Limit"
            }
            CodeELP => {
                "Occupational Safety and Health Administration (OSHA) Permissible Exposure Limit Ceiling"
            }
            CodeELS => {
                "American Conference of Government Industrial Hygienists (ACGIH) Threshold Limit Value: Short-Term Exposure"
            }
            CodeELT => {
                "American Conference of Governmental Industrial Hygienists (ACGIH) Threshold Limit Value: Time Weighted Average"
            }
            UnshieldedExposureRate => "Unshielded Exposure Rate",
            CodeELW => {
                "American Industrial Hygienists Association (AIHA) Work Environment Exposure Level (WEEL)"
            }
            CodeELX => {
                "American Industrial Hygienists Association (AIHA) Work Environment Exposure Level (WEEL): Time Weighted Average"
            }
            ElmendorfTear => "Elmendorf Tear",
            Entanglement => "Entanglement",
            ExcitingPower => "Exciting Power",
            ExpectedProductLife => "Expected Product Life",
            Edge => "Edge",
            EndPoint => "End Point",
            EvaporationLoss => "Evaporation Loss",
            EvaporationRate => "Evaporation Rate",
            EmptyWeight => "Empty Weight",
            Eccentricity => "Eccentricity",
            ExhaustBenzeneEmissions => "Exhaust Benzene Emissions",
            Extractables => "Extractables",
            DeficitWeight => "Deficit Weight",
            FirePoint => "Fire Point",
            RadiatedPower => "Radiated Power",
            CodeF3 => "Output Power (Peak Envelope)",
            HeightAboveAverageTerrain => "Height above Average Terrain",
            GroundElevation => "Ground Elevation",
            HeightToTip => "Height to Tip",
            RadiusFromALocation => "Radius from a Location",
            RadiusFromCoordinates => "Radius from Coordinates",
            OperatingHours => "Operating Hours",
            Fluorine => "Fluorine",
            Flare => "Flare",
            AcetateBreakPoint => "Acetate Break Point",
            ShortCycleFlatness => "Short Cycle Flatness",
            FrequencyOfOperation => "Frequency of Operation",
            Fermium => "Fermium",
            Francium => "Francium",
            FreezingPoint => "Freezing Point",
            Finish => "Finish",
            FilamentCount => "Filament Count",
            FilterNumber => "Filter Number",
            Fineness => "Fineness",
            Filterability => "Filterability",
            FaceWidth => "Face Width",
            FluidConsistency => "Fluid Consistency",
            LongitudinalFlatness => "Longitudinal Flatness",
            FluidPoint => "Fluid Point",
            FiberLength => "Fiber Length",
            FluidLevelAbovePump => "Fluid Level Above Pump",
            Fluting => "Fluting",
            FlavorThreshold => "Flavor Threshold",
            FlammabilityLimits => "Flammability Limits",
            Flammability => "Flammability",
            Flatness => "Flatness",
            FinesRetainedOnScreen => "Fines Retained on Screen",
            Fines => "Fines",
            Foam => "Foam",
            FrontOverHangOfVehicle => "Front Over-Hang of Vehicle",
            FoulingIndex => "Fouling Index",
            ForeignMatter => "Foreign Matter",
            Flashpoint => "Flashpoint",
            AcetatePluggingValue => "Acetate Plugging Value",
            Frequency => "Frequency",
            FlowRate => "Flow Rate",
            FoldStrength => "Fold Strength",
            FreeSwellingIndex => "Free Swelling Index",
            FlangeThickness => "Flange Thickness",
            Azimuth => "Azimuth",
            FullLoad => "Full Load",
            TransverseFlatness => "Transverse Flatness",
            FlangeWidth => "Flange Width",
            Filler => "Filler",
            Beam => "Beam",
            CodeFZ => "Output Power (Mean RF)",
            GrossWeight => "Gross Weight",
            Gutter => "Gutter",
            GrainSize => "Grain Size",
            Tilt => "Tilt",
            GForce => "G-Force",
            Grain => "Grain",
            GuidedBendsRoot => "Guided Bends Root",
            Gadolinium => "Gadolinium",
            Gold => "Gold",
            GuidedBendsFace => "Guided Bends Face",
            Gel => "Gel",
            GuidedBendsSide => "Guided Bends Side",
            Gauge => "Gauge",
            GasGravity => "Gas Gravity",
            Gallium => "Gallium",
            CodeGI => "Grit, Brushed",
            GasInjectionTestRate => "Gas Injection Test Rate",
            CodeGJ => "Grit, Unbrushed",
            TintingStrength => "Tinting Strength",
            Gloss => "Gloss",
            GroundLevelElevation => "Ground Level Elevation",
            EmulsionGloss => "Emulsion Gloss",
            GrossToNetConversionFactor => "Gross to Net Conversion Factor",
            ParticleEndPoint => "Particle End Point",
            GasToOilRatio => "Gas-to-Oil Ratio",
            GroupPackageSeparation => "Group Package Separation",
            ScatteredParticles => "Scattered Particles",
            Gravity => "Gravity",
            GasTestRate => "Gas Test Rate",
            Graininess => "Graininess",
            CarbonBlackUndertone => "Carbon Black Undertone",
            GlassTransitionTemperature => "Glass Transition Temperature",
            CodeGW => "Gross Weight, Maximum",
            GroundWaterReferencePoint => "Ground Water Reference Point",
            Heavies => "Heavies",
            WaterVolume => "Water Volume",
            NetExplosiveWeight => "Net Explosive Weight",
            RecommendedExposureLimit => "Recommended Exposure Limit",
            HydroPressure => "Hydro Pressure",
            HardeningRate => "Hardening Rate",
            Haze => "Haze",
            HeavyAluminas => "Heavy Aluminas",
            Helium => "Helium",
            HorizontalCenterOfGravity => "Horizontal Center of Gravity",
            DistanceToEndpoint => "Distance to Endpoint",
            EmissionFactor => "Emission Factor",
            HeatContent => "Heat Content",
            PollutantEmission => "Pollutant Emission",
            Population => "Population",
            ReleaseDuration => "Release Duration",
            ReleasedQuantity => "Released Quantity",
            ReleaseRate => "Release Rate",
            SulfurContent => "Sulfur Content",
            WindSpeed => "Wind Speed",
            AcuteToxicity => "Acute Toxicity",
            ChronicToxicity => "Chronic Toxicity",
            Discharges => "Discharges",
            Effectiveness => "Effectiveness",
            Fertilization => "Fertilization",
            SamplesInCompliance => "Samples in Compliance",
            Toxicity => "Toxicity",
            HighPropagationDelayTime => "High Propagation Delay Time",
            HeavySilicates => "Heavy Silicates",
            Hardness => "Hardness",
            HeavySulfides => "Heavy Sulfides",
            HeavyGlobularOxides => "Heavy Globular Oxides",
            HemisphericalPoint => "Hemispherical Point",
            Hafnium => "Hafnium",
            HighBoilers => "High Boilers",
            Holmium => "Holmium",
            HumanFactors => "Human Factors",
            HeatLoss => "Heat Loss",
            CodeHM => "Height, Maximum",
            Holes => "Holes",
            HeatOfCombustion => "Heat of Combustion",
            HeightOfTreadPlatePattern => "Height of Tread Plate Pattern",
            HeightOfRunners => "Height of Runners",
            Height => "Height",
            HeatEquivalency => "Heat Equivalency",
            HeavyMetals => "Heavy Metals",
            SofteningPoint => "Softening Point",
            HydroxylNumber => "Hydroxyl Number",
            Hazepoint => "Hazepoint",
            HazardousComponent => "Hazardous Component",
            HeatInput => "Heat Input",
            ImperfectionsThicks => "Imperfections - Thicks",
            ImpactEnergy => "Impact Energy",
            IncrementalDistance => "Incremental Distance",
            InsideDiameter => "Inside Diameter",
            Identification => "Identification",
            ImperfectionsThins => "Imperfections - Thins",
            InputLowVoltage => "Input Low Voltage",
            GranulatedIngotSize => "Granulated Ingot Size",
            AutoignitionTemperature => "Autoignition Temperature",
            InputGasRate => "Input Gas Rate",
            InputCurrent => "Input Current",
            InherentViscosity => "Inherent Viscosity",
            InputSetupTime => "Input Setup Time",
            InputHoldTime => "Input Hold Time",
            Indium => "Indium",
            ImperfectionsNeps => "Imperfections - Neps",
            Iridium => "Iridium",
            ImpactAdhesion => "Impact Adhesion",
            Impurities => "Impurities",
            InputLowCurrent => "Input Low Current",
            InitialDeformationPoint => "Initial Deformation Point",
            Insolubles => "Insolubles",
            InputHighCurrent => "Input High Current",
            InputLeakageCurrent => "Input Leakage Current",
            PrimaryIrritationIndex => "Primary Irritation Index",
            InputClampDiodeVoltage => "Input Clamp Diode Voltage",
            IdealDiameter => "Ideal Diameter",
            InclusionRating => "Inclusion Rating",
            InputHighVoltage => "Input High Voltage",
            InterruptPulsePeriod => "Interrupt Pulse Period",
            IntegratedDifferential => "Integrated Differential",
            InterruptSetupTime => "Interrupt Setup Time",
            Iodine => "Iodine",
            IncrustationFactor => "Incrustation Factor",
            CodeIX => "Inside Diameter, Minimum",
            IndexDifferential => "Index Differential",
            Induction => "Induction",
            IngotPoundSize => "Ingot Pound Size",
            JunctionTemperature => "Junction Temperature",
            JominyHardenability => "Jominy Hardenability",
            Knoop => "Knoop",
            Krypton => "Krypton",
            CodeKN => "K&N Holdout",
            StrengthCoefficient => "Strength Coefficient",
            LegalWeight => "Legal Weight",
            LeafGrade => "Leaf Grade",
            Lights => "Lights",
            Lanthanum => "Lanthanum",
            LanglierIndex => "Langlier Index",
            Lithium => "Lithium",
            LowPropagationDelayTime => "Low Propagation Delay Time",
            CodeLC5 => "Lethal Concentration, 50% (\"LC-50\")",
            LongitudinalCenterOfGravity => "Longitudinal Center of Gravity",
            Lawrencium => "Lawrencium",
            CodeLD5 => "Lethal Dose, 50% (\"LD-50\")",
            CodeLDH => "Limited Dome Height (LDH)",
            Lutetium => "Lutetium",
            Leaf => "Leaf",
            LongFibers => "Long Fibers",
            Leg => "Leg",
            LiquidInjectionTestRate => "Liquid Injection Test Rate",
            LimitIntrinsicViscosity => "Limit Intrinsic Viscosity",
            LongLeg => "Long Leg",
            LightLoad => "Light Load",
            CodeLM => "Length, Maximum",
            Length => "Length",
            LongLength => "Long Length",
            LossOnIgnition => "Loss on Ignition",
            LossOnDrying => "Loss on Drying",
            LowBoilers => "Low Boilers",
            CamberOfPatternLine => "Camber of Pattern Line",
            LiquefiedPetroleumGasFactor => "Liquefied Petroleum Gas Factor",
            LabeledProductLife => "Labeled Product Life",
            LinePressure => "Line Pressure",
            ShortLeg => "Short Leg",
            LockSeamAdhesion => "Lock Seam Adhesion",
            LengthShrinkage => "Length Shrinkage",
            LinearSheetSwelling => "Linear Sheet Swelling",
            LengthwiseSpacing => "Lengthwise Spacing",
            LinerTopDepth => "Liner Top Depth",
            LongWidth => "Long Width",
            CodeM => "Minimum Weight (for Weight)",
            MeltRange => "Melt Range",
            MaximumDifferentialPressure => "Maximum Differential Pressure",
            MaximumStaticPressure => "Maximum Static Pressure",
            Area => "Area",
            MinimumSpeed => "Minimum Speed",
            MaximumSpeed => "Maximum Speed",
            MeanAverage => "Mean Average",
            Maturity => "Maturity",
            Mendelevium => "Mendelevium",
            MinMaxCuttableWidth => "Min./Max Cuttable Width",
            MagneticContamination => "Magnetic Contamination",
            MeasurementVoltage => "Measurement Voltage",
            CodeMDL => {
                "Method Detection Limit; Minimum concentration of a substance that can be measured and reported with 99% confidence that analyte concentration is greater than zero"
            }
            MaximumInputLowVoltage => "Maximum Input Low Voltage",
            MediaDepth => "Media Depth",
            MeterFactor => "Meter Factor",
            MeltTime => "Melt Time",
            MeatProtein => "Meat Protein",
            EfficientRateReservoir => "Efficient Rate-Reservoir",
            MinimumInputHighVoltage => "Minimum Input High Voltage",
            Mercury => "Mercury",
            MeltingPoint => "Melting Point",
            HighestTorque => "Highest Torque",
            Minimum => "Minimum",
            Micronaire => "Micronaire",
            MilkFat => "Milk Fat",
            CodeMJ => "Major Section (Stepped)",
            CodeMK => "Microseperometer (MSEP)",
            Management => "Management",
            MinimumAverage => "Minimum Average",
            Mottles => "Mottles",
            Moisture => "Moisture",
            Mortality => "Mortality",
            MullenPop => "Mullen Pop",
            ProductionRateWell => "Production Rate-Well",
            ManyPressTest => "Many Press Test",
            McQuaid => "MCQuaid",
            CodeMR => "Module R (R Bar)",
            CodeMS => "Minor Section (Stepped)",
            MoistureContent => "Moisture Content",
            MaximumTotalDepth => "Maximum Total Depth",
            Multiplier => "Multiplier",
            Mullen => "Mullen",
            MaximumAverage => "Maximum Average",
            MolecularWeight => "Molecular Weight",
            Maximum => "Maximum",
            MagnetizingField => "Magnetizing Field",
            ActualNetWeight => "Actual Net Weight",
            NumberPerPackage => "Number per Package",
            NumberPerBundle => "Number per Bundle",
            NumberPerCoilGroup => "Number per Coil Group",
            Neodymium => "Neodymium",
            NeutralizationNumber => "Neutralization Number",
            Neon => "Neon",
            Nobelium => "Nobelium",
            NumberOfItemsPerPackageLabel => "Number of Items per Package Label",
            NumberOfSplicesPerPackageLabel => "Number of Splices per Package Label",
            NilDuctilityTest => "Nil Ductility Test",
            NumberOfSheetsPerPackageLabel => "Number of Sheets per Package Label",
            NestingFactor => "Nesting Factor",
            NumberPerLift => "Number per Lift",
            NumberPkgsPerMasterPack => "Number Pkgs. per Master Pack",
            NetNetWeight => "Net Net Weight",
            CodeNO => "Nominal (Target, Aim)",
            NumberOfCosigners => "Number of Cosigners",
            NonVolatileMatter => "Non-Volatile Matter",
            NumberOfReferences => "Number of References",
            NOxEmissionsPerformance => "NOx Emissions Performance",
            PercentOfSpecified => "Percent of Specified",
            NumberPerSkid => "Number per Skid",
            NumberPerUnit => "Number per Unit",
            NValue => "N Value",
            ExcessWeightOverMaximum => "Excess Weight Over Maximum",
            OrificeInsideDiameter => "Orifice - Inside Diameter",
            Offset => "Offset",
            ObservedAmericanPetroleumInstituteGravity => {
                "Observed American Petroleum Institute Gravity"
            }
            Osmium => "Osmium",
            ObservedTemperature => "Observed Temperature",
            OutputLowVoltage => "Output Low Voltage",
            OilCondensateGravity => "Oil/Condensate Gravity",
            OilCondensateTestRate => "Oil/Condensate Test Rate",
            OutsideDiameter => "Outside Diameter",
            Odor => "Odor",
            OutputLowCurrent => "Output Low Current",
            OutputHighVoltage => "Output High Voltage",
            OutputHighCurrent => "Output High Current",
            CodeOH => "Overhead Height, Receiving Door",
            OutputOffCurrentLow => "Output Off Current Low",
            Oil => "Oil",
            OutputOffCurrentHigh => "Output Off Current High",
            OutputShortCircuitCurrent => "Output Short-Circuit Current",
            OutputDisableTimeFromLowLevelOfA3StateOutput => {
                "Output Disable Time from Low Level of a 3-State Output"
            }
            Olefins => "Olefins",
            CodeOM => "Outside Diameter, Maximum",
            OutputDisableTimeFromHighLevelOfA3StateOutput => {
                "Output Disable Time from High Level of a 3-State Output"
            }
            OutputEnableTimeFromLowLevelOfA3StateOutput => {
                "Output Enable Time from Low Level of a 3-State Output"
            }
            Openness => "Openness",
            OutputEnableTimeFromHighLevelOfA3StateOutput => {
                "Output Enable Time from High Level of a 3-State Output"
            }
            DistanceBetweenOutsideRunners => "Distance Between Outside Runners",
            OrganicCarbon => "Organic Carbon",
            OpenCircuits => "Open Circuits",
            OutputDelayTime => "Output Delay Time",
            OthersEach => "Others Each",
            OdorThreshold => "Odor Threshold",
            OthersTotal => "Others Total",
            Opacity => "Opacity",
            OverallWidth => "Overall Width",
            OwnershipShare => "Ownership Share",
            OxidizableSubstance => "Oxidizable Substance",
            OxidizingSubstance => "Oxidizing Substance",
            OperatingWeight => "Operating Weight",
            Price => "Price",
            PackageSeparation => "Package Separation",
            ParticleSize => "Particle Size",
            Pressure => "Pressure",
            PlugBackTotalDepth => "Plug Back Total Depth",
            PerHundredLinearYards => "Per Hundred Linear Yards",
            Platinum => "Platinum",
            CasingLinerTubingDepth => "Casing/Liner Tubing Depth",
            PumpDepthFromGround => "Pump Depth from Ground",
            Potassium => "Potassium",
            MagneticPermeability => "Magnetic Permeability",
            Promethium => "Promethium",
            PerforationFeetOpen => "Perforation Feet Open",
            Polonium => "Polonium",
            PulseSetupTime => "Pulse Setup Time",
            PH => "pH",
            HardwoodFiber => "Hardwood Fiber",
            PulseHoldTime => "Pulse Hold Time",
            PickOff => "Pick Off",
            PulseWidth => "Pulse Width",
            PulseRecoveryTime => "Pulse Recovery Time",
            PercentDefective => "Percent Defective",
            Practice => "Practice",
            Palladium => "Palladium",
            CodePO => "Percent of Order (-, +)",
            Completion => "Completion",
            PhysicalDescriptionOuterDiameter => "Physical Description - Outer Diameter",
            PourPoint => "Pour Point",
            PowderPastePackageSize => "Powder/Paste Package Size",
            ProprietaryShade => "Proprietary Shade",
            Plutonium => "Plutonium",
            CodePQL => {
                "Practical Quantitation Limit; Lowest concentration of a substance which can be consistently determined within +/- 20% of the true concentration by 75% of the laboratories tested in a performance evaluation study"
            }
            Praseodymium => "Praseodymium",
            ProportionAlive => "Proportion Alive",
            PriorExperience => "Prior Experience",
            PressureFactor => "Pressure Factor",
            ProductIndex => "Product Index",
            ProductLevel => "Product Level",
            ProportionNormal => "Proportion Normal",
            Processability => "Processability",
            ProductReportableQuantity => "Product Reportable Quantity",
            Porosity => "Porosity",
            ProportionFertilized => "Proportion Fertilized",
            Protactinium => "Protactinium",
            PercentSolutionActual => "Percent Solution Actual",
            PastPerformance => "Past Performance",
            SoftwoodFiber => "Softwood Fiber",
            Pits => "Pits",
            PressureBase => "Pressure Base",
            Picks => "Picks",
            PurchasedWidth => "Purchased Width",
            ProcessedWaste => "Processed Waste",
            PhysicalDescriptionWeight => "Physical Description - Weight",
            PowerFactor => "Power Factor",
            Purity => "Purity",
            PercentOfWater => "Percent of Water",
            PipeSizeNominal => "Pipe Size Nominal",
            VolatileOrganicCompoundsPlusWater => "Volatile Organic Compounds Plus Water",
            QualityIndex => "Quality Index",
            QuantityOrLoadingAverage => "Quantity or Loading Average",
            QuantityOrLoadingMaximum => "Quantity or Loading Maximum",
            QualityOrConcentrationAverage => "Quality or Concentration Average",
            QualityOrConcentrationMinimum => "Quality or Concentration Minimum",
            QualityOrConcentrationMaximum => "Quality or Concentration Maximum",
            Duration => "Duration",
            Abundance => "Abundance",
            Biomass => "Biomass",
            SizeClass => "Size Class",
            Quality => "Quality",
            ReportableQuantity => "Reportable Quantity",
            PerUnitDunnage => "Per Unit Dunnage",
            Hemoglobin => "Hemoglobin",
            Hematocrit => "Hematocrit",
            EpoetinStartingDosage => "Epoetin Starting Dosage",
            Creatinine => "Creatinine",
            Speed => "Speed",
            SpeedLimit => "Speed Limit",
            RelativeFractionOfPureLongChainCellulose => {
                "Relative Fraction of Pure Long-Chain Cellulose"
            }
            RelativeFractionOfTotalCellulose => "Relative Fraction of Total Cellulose",
            RelativeHumidity => "Relative Humidity",
            Radius => "Radius",
            RoofAdjustmentFactor => "Roof Adjustment Factor",
            RangeValue => "Range Value",
            RadiusOfCorner => "Radius of Corner",
            Readpoint => "Readpoint",
            ReamWeight => "Ream Weight",
            Reactivity => "Reactivity",
            ReducingSubstance => "Reducing Substance",
            Refining => "Refining",
            RefractiveIndex => "Refractive Index",
            Reflectance => "Reflectance",
            Resistance => "Resistance",
            Resistivity => "Resistivity",
            Radium => "Radium",
            Rhenium => "Rhenium",
            Rubidium => "Rubidium",
            RockwellC => "Rockwell-C",
            RockwellB => "Rockwell-B",
            ReductionRation => "Reduction Ration",
            CodeRM => "RMS Range (Side 1)",
            RequiredInterruptRelease => "Required Interrupt Release",
            ResetPulseWidth => "Reset Pulse Width",
            RearOverHangOfVehicle => "Rear Over-Hang of Vehicle",
            OxygenFromARenewableOxygenate => "Oxygen from a Renewable Oxygenate",
            ReductionOfArea => "Reduction of Area",
            Radon => "Radon",
            ReductionRatio => "Reduction Ratio",
            CodeRS => "RMS Range (Side 2)",
            RollSize => "Roll Size",
            RoundsAmmunitionMilitary => "Rounds Ammunition/Military",
            ReportingTemperatureBase => "Reporting Temperature Base",
            Rhodium => "Rhodium",
            CodeRUD => {
                "Usage Deviation (Applies to Kilowatt Hours, Kilowatt Demand and Reactive Demand)"
            }
            Ruthenium => "Ruthenium",
            ReidVaporPressure => "Reid Vapor Pressure",
            RollingWidth => "Rolling Width",
            Ridges => "Ridges",
            Ratio => "Ratio",
            StateWeight => "State Weight",
            Smoothness => "Smoothness",
            SelvedgeOnBeam => "Selvedge on Beam",
            SheffieldSmoothness => "Sheffield Smoothness",
            SurfaceStrength => "Surface Strength",
            Stiffness => "Stiffness",
            Saturation => "Saturation",
            Sediment => "Sediment",
            Solubility => "Solubility",
            SiteAtmosphericPressure => "Site Atmospheric Pressure",
            PulpImpurities => "Pulp Impurities",
            Start => "Start",
            Hemicellulose => "Hemicellulose",
            SortCodeCieLab => "Sort Code CIE LAB",
            CodeSAL => "Salinity; Salt level in a sample of seawater",
            SaponificationNumber => "Saponification Number",
            SortCodeCmc => "Sort Code CMC",
            CodeSC => "Schedule Number (Pipe Size)",
            Schedule => "Schedule",
            Strength => "Strength",
            SelvageLeft => "Selvage Left",
            Severity => "Severity",
            Samarium => "Samarium",
            ShortFiberContent => "Short Fiber Content",
            SlitWidth => "Slit Width",
            Strontium => "Strontium",
            ShelfLife => "Shelf Life",
            SupplyCurrent => "Supply Current",
            CodeSIL => "Silica (Silicon Dioxide)",
            ShortCircuits => "Short Circuits",
            Shrinkage => "Shrinkage",
            ShortLength => "Short Length",
            Solderability => "Solderability",
            SlaggingIndex => "Slagging Index",
            Shear => "Shear",
            SamBRating => "SAM-B Rating",
            SamDRating => "SAM-D Rating",
            Stain => "Stain",
            SortCodeCieLch => "Sort Code CIE LCH",
            Solids => "Solids",
            SofteningRange => "Softening Range",
            SplinterCount => "Splinter Count",
            SpecificGravity => "Specific Gravity",
            Sphere => "Sphere",
            SeparatorPressure => "Separator Pressure",
            StaticPressure => "Static Pressure",
            ShippedQuantity => "Shipped Quantity",
            SelvageRight => "Selvage Right",
            Silver => "Silver",
            StopRecoveryStartupTime => "Stop Recovery Startup Time",
            Stability => "Stability",
            ShortTermExposureLimit => "Short Term Exposure Limit",
            Staple => "Staple",
            ShippedUnits => "Shipped Units",
            SuspendedMatter => "Suspended Matter",
            SurfaceRoughness => "Surface Roughness",
            SurfaceTension => "Surface Tension",
            Scandium => "Scandium",
            Survival => "Survival",
            ShortWidth => "Short Width",
            Sodium => "Sodium",
            S10MinusS18Value => "S10 Minus S18 Value",
            ServiceInterruptDuration => "Service Interrupt Duration",
            SkidHeight => "Skid Height",
            TareWeight => "Tare Weight",
            TirePressure => "Tire Pressure",
            TubeInsideDiameter => "Tube - Inside Diameter",
            Technical => "Technical",
            SingleEndBreak => "Single End Break",
            SkeinBreak => "Skein Break",
            T50 => "T50",
            T90 => "T90",
            CodeTA => "Thickness Heavy End (Tapered/Stepped)",
            Taste => "Taste",
            CodeTB => "Thickness Small End (Tapered/Stepped)",
            Temperature => "Temperature",
            TireTreadContactLength => "Tire Tread Contact Length",
            ThinAluminas => "Thin Aluminas",
            PerforationTopDepth => "Perforation Top Depth",
            Tenacity => "Tenacity",
            AutodecompositionTemperature => "Autodecomposition Temperature",
            StorageTemperature => "Storage Temperature",
            Texture => "Texture",
            Tensile => "Tensile",
            ThinSulfides => "Thin Sulfides",
            Thickness => "Thickness",
            ThinSilicates => "Thin Silicates",
            TotalSupplyCurrent => "Total Supply Current",
            TimerPulseWidth => "Timer Pulse Width",
            TaperedSteppedLengthType => "Tapered/Stepped Length Type",
            LengthTypeMultiples => "Length Type: Multiples",
            TimerPeriod => "Timer Period",
            Terbium => "Terbium",
            AquaticToxicity => "Aquatic Toxicity",
            Torque => "Torque",
            ToxicEmissionsPerformance => "Toxic Emissions Performance",
            Thorium => "Thorium",
            TemperatureFactor => "Temperature Factor",
            TubingPressureFlowing => "Tubing Pressure - Flowing",
            ThresholdPlanningQuantity => "Threshold Planning Quantity",
            TubingPressureShutin => "Tubing Pressure - Shutin",
            ThinGlobularOxides => "Thin Globular Oxides",
            LengthTypeRandom => "Length Type: Random",
            TrashArea => "Trash Area",
            TrashCount => "Trash Count",
            TireDiameter => "Tire Diameter",
            Transmittance => "Transmittance",
            CodeTRS => {
                "Transmissivity; Measure of the quantity of light that passes through a given volume of seawater; also used to measure turbidity and to estimate plant growing zones in the ocean"
            }
            CodeTRT => {
                "Transmissivity Pathlength; The length of the path taken to arrive at transmissivity measurements"
            }
            LengthTypeSpecific => "Length Type: Specific",
            TrimSize => "Trim Size",
            Time => "Time",
            TrailerTongueLength => "Trailer Tongue Length",
            Technetium => "Technetium",
            Turbidity => "Turbidity",
            Thallium => "Thallium",
            MaximumTrueVerticalDepth => "Maximum True Vertical Depth",
            Top => "Top",
            TireWidth => "Tire Width",
            Thulium => "Thulium",
            TearStrength => "Tear Strength",
            WeightPerUnit => "Weight per Unit",
            Uranium => "Uranium",
            Cube => "Cube",
            Usage => "Usage",
            UnipunchAdhesion => "Unipunch Adhesion",
            Uniformity => "Uniformity",
            Unknowns => "Unknowns",
            OxygenationLevel => "Oxygenation Level",
            VaporDensity => "Vapor Density",
            VaporPressure => "Vapor Pressure",
            VBendAdhesion => "V-Bend Adhesion",
            VerticalCenterOfGravity => "Vertical Center of Gravity",
            CodeVH => "Height, Van Door",
            Vinyl => "Vinyl",
            Viscosity => "Viscosity",
            Voltage => "Voltage",
            VocEmissionsPerformance => "VOC Emissions Performance",
            Volume => "Volume",
            Volatiles => "Volatiles",
            VolatilesByVolume => "Volatiles by Volume",
            VolatilesByWeight => "Volatiles by Weight",
            VolumeSplitToOthers => "Volume Split to Others",
            CodeVW => "Width, Van Door",
            VolumeWeight => "Volume Weight",
            ReformulatedFuelLevel => "Reformulated Fuel Level",
            WeightPerUnitOfArea => "Weight per Unit of Area",
            Web => "Web",
            WebDepthHeight => "Web Depth/Height",
            Width => "Width",
            WaterDepth => "Water Depth",
            Wolfram => "Wolfram",
            WeightLoss => "Weight Loss",
            WaitRecoveryStartupTime => "Wait Recovery Startup Time",
            Whiteness => "Whiteness",
            WindingLoss => "Winding Loss",
            WallThickness => "Wall Thickness",
            CodeWM => "Width, Maximum",
            WaterOilDistributionCoefficient => "Water-Oil Distribution Coefficient",
            WellheadPressureFlowing => "Wellhead Pressure-Flowing",
            WaterProductLevel => "Water/Product Level",
            WellheadPressureShutin => "Wellhead Pressure Shutin",
            Wrinkles => "Wrinkles",
            WaterTestRate => "Water Test Rate",
            WidthShrinkage => "Width Shrinkage",
            Weight => "Weight",
            WaterTankBottomLevel => "Water/Tank Bottom Level",
            WeightPerUnitOfLength => "Weight per Unit of Length",
            WaxPick => "Wax Pick",
            CodeX => "Maximum Weight (for Rate)",
            Xenon => "Xenon",
            CodeXH => "Side Height, Flat Bed With Removable Sides",
            Specified => "Specified",
            Squareness => "Squareness",
            SpoolSize => "Spool Size",
            Yttrium => "Yttrium",
            Yield => "Yield",
            Ytterbium => "Ytterbium",
            YarnCount => "Yarn Count",
            YieldPointElongation => "Yield Point Elongation",
            Aluminum => "Aluminum",
            Arsenic => "Arsenic",
            Boron => "Boron",
            Bismuth => "Bismuth",
            NButane => "N-Butane",
            Benzene => "Benzene",
            Carbon => "Carbon",
            Calcium => "Calcium",
            Columbium => "Columbium",
            CarbonDioxide => "Carbon Dioxide",
            Cerium => "Cerium",
            CarbonMonoxide => "Carbon Monoxide",
            Cobalt => "Cobalt",
            Chromium => "Chromium",
            Copper => "Copper",
            LoadFactor => "Load Factor",
            Ethane => "Ethane",
            SulfateSulfur => "Sulfate Sulfur",
            Iron => "Iron",
            NewspaperFullPage => "Newspaper--Full Page",
            IronPlusSilicon => "Iron plus Silicon",
            OrganicSulfur => "Organic Sulfur",
            Germanium => "Germanium",
            Hydrogen => "Hydrogen",
            Heptane => "Heptane",
            HydrogenSulfide => "Hydrogen Sulfide",
            Hexane => "Hexane",
            IButane => "I-Butane",
            IPentane => "I-Pentane",
            Magnesium => "Magnesium",
            Manganese => "Manganese",
            Molybdenum => "Molybdenum",
            Methane => "Methane",
            Nitrogen => "Nitrogen",
            Niobium => "Niobium",
            Nickel => "Nickel",
            NeoPentane => "Neo-Pentane",
            Oxygen => "Oxygen",
            Octane => "Octane",
            Phosphorous => "Phosphorous",
            Lead => "Lead",
            Propane => "Propane",
            NPentane => "N-Pentane",
            PyriticSulfur => "Pyritic Sulfur",
            Sulfur => "Sulfur",
            Antimony => "Antimony",
            Selenium => "Selenium",
            Silicon => "Silicon",
            Tin => "Tin",
            Tantalum => "Tantalum",
            NewspaperTabloidPage => "Newspaper--Tabloid Page",
            Tellurium => "Tellurium",
            Titanium => "Titanium",
            Vanadium => "Vanadium",
            Tungsten => "Tungsten",
            Zinc => "Zinc",
            Zirconium => "Zirconium",
            MutuallyDefined => "Mutually Defined",
        }
    }
    fn from_description(description: &str) -> Option<MeasurementQualifier> {
        {
            use MeasurementQualifier::*;
            match description {
                "Control Efficiency" => Some(ControlEfficiency),
                "Radio Frequency" => Some(RadioFrequency),
                "Capture Efficiency" => Some(CaptureEfficiency),
                "Alternate Radio Frequency" => Some(AlternateRadioFrequency),
                "Photonflux Density" => Some(PhotonfluxDensity),
                "Target Depth" => Some(TargetDepth),
                "Current Depth" => Some(CurrentDepth),
                "Total Depth" => Some(TotalDepth),
                "Well Test Before Oil" => Some(WellTestBeforeOil),
                "Well Test Before Gas" => Some(WellTestBeforeGas),
                "Well Test Before Water" => Some(WellTestBeforeWater),
                "Well Test After Oil" => Some(WellTestAfterOil),
                "Well Test After Gas" => Some(WellTestAfterGas),
                "Well Test After Water" => Some(WellTestAfterWater),
                "Estimated Depth of Operations" => Some(EstimatedDepthOfOperations),
                "Throughput Rate" => Some(ThroughputRate),
                "Squelch Tone" => Some(SquelchTone),
                "Cloud Cover" => Some(CloudCover),
                "Height above Ground" => Some(HeightAboveGround),
                "Velocity" => Some(Velocity),
                "Gain" => Some(Gain),
                "Plume Height" => Some(PlumeHeight),
                "Individuals" => Some(Individuals),
                "Directional Height above Average Terrain" => {
                    Some(DirectionalHeightAboveAverageTerrain)
                }
                "Storage Limits" => Some(StorageLimits),
                "Painting Costs" => Some(PaintingCosts),
                "Structural Costs" => Some(StructuralCosts),
                "Appliances" => Some(Appliances),
                "Utilities" => Some(Utilities),
                "Carpet or Floors" => Some(CarpetOrFloors),
                "Other Repairs" => Some(OtherRepairs),
                "Landscaping" => Some(Landscaping),
                "Roof" => Some(Roof),
                "Windows" => Some(Windows),
                "Cleaning or Trash Removal" => Some(CleaningOrTrashRemoval),
                "Probable Sales Price" => Some(ProbableSalesPrice),
                "Proximity" => Some(Proximity),
                "Repairs and Improvements" => Some(RepairsAndImprovements),
                "Contributory Value of Repairs and Improvements" => {
                    Some(ContributoryValueOfRepairsAndImprovements)
                }
                "Marketing Time" => Some(MarketingTime),
                "Closed Comparable Sales" => Some(ClosedComparableSales),
                "Competitive Listings in Price Range" => {
                    Some(CompetitiveListingsInPriceRange)
                }
                "Financing Concessions" => Some(FinancingConcessions),
                "Marketing Concessions" => Some(MarketingConcessions),
                "Probable Net Price" => Some(ProbableNetPrice),
                "Suggested Initial List Price" => Some(SuggestedInitialListPrice),
                "Value Change" => Some(ValueChange),
                "Probable Final Value" => Some(ProbableFinalValue),
                "Occupancy Rate" => Some(OccupancyRate),
                "Number of Living Units" => Some(NumberOfLivingUnits),
                "Number of Phases" => Some(NumberOfPhases),
                "Number of Active Listings" => Some(NumberOfActiveListings),
                "Price Active Listings" => Some(PriceActiveListings),
                "Price Per Gross Living Area" => Some(PricePerGrossLivingArea),
                "Built-up Rate" => Some(BuiltUpRate),
                "Vacant Rate" => Some(VacantRate),
                "Typical Rents" => Some(TypicalRents),
                "Neighborhood Apartment Vacancy" => Some(NeighborhoodApartmentVacancy),
                "Number of Admissions" => Some(NumberOfAdmissions),
                "Cost of Hire" => Some(CostOfHire),
                "Frontage" => Some(Frontage),
                "Gross Sales" => Some(GrossSales),
                "Number of Employees" => Some(NumberOfEmployees),
                "Payroll" => Some(Payroll),
                "Per Capita or Each" => Some(PerCapitaOrEach),
                "Remuneration" => Some(Remuneration),
                "Total Cost" => Some(TotalCost),
                "Total Mileage" => Some(TotalMileage),
                "Number of Rating Units" => Some(NumberOfRatingUnits),
                "Garage Employee Payroll Maximum" => Some(GarageEmployeePayrollMaximum),
                "Employee Gross Wage Less Allowable Deductions" => {
                    Some(EmployeeGrossWageLessAllowableDeductions)
                }
                "Garage Employee Average Hours Worked Per Week" => {
                    Some(GarageEmployeeAverageHoursWorkedPerWeek)
                }
                "Garage (Dealers) Employee Weeks Worked" => Some(Code66),
                "Gross Wage" => Some(GrossWage),
                "Subcontractor - Labor and Materials" => {
                    Some(SubcontractorLaborAndMaterials)
                }
                "Subcontractor - Labor Only" => Some(SubcontractorLaborOnly),
                "Consolidated Weight" => Some(ConsolidatedWeight),
                "Acids" => Some(Acids),
                "Adsorption" => Some(Adsorption),
                "Aging Time" => Some(AgingTime),
                "Aromatics" => Some(Aromatics),
                "Average Differential Pressure" => Some(AverageDifferentialPressure),
                "Average Static Pressure" => Some(AverageStaticPressure),
                "Flame Projection Distance" => Some(FlameProjectionDistance),
                "Exposure" => Some(Exposure),
                "Alternating Current" => Some(AlternatingCurrent),
                "AC-apparent Power" => Some(AcApparentPower),
                "Activation Energy" => Some(ActivationEnergy),
                "Absorbance" => Some(Absorbance),
                "Abrasion" => Some(Abrasion),
                "Absorbency" => Some(Absorbency),
                "Actinium" => Some(Actinium),
                "Acid Number" => Some(AcidNumber),
                "Ambient Temperature" => Some(AmbientTemperature),
                "Adhesion" => Some(Adhesion),
                "Dye Manufacturing Units" => Some(DyeManufacturingUnits),
                "Argon" => Some(Argon),
                "Angle of Bend" => Some(AngleOfBend),
                "Americium" => Some(Americium),
                "Inventory Age" => Some(InventoryAge),
                "Aggressive Index" => Some(AggressiveIndex),
                "Astatine" => Some(Astatine),
                "Acidity" => Some(Acidity),
                "Aim Gage" => Some(AimGage),
                "Volatile Organic Compounds (VOCs)" => Some(CodeAK),
                "Spine Show" => Some(SpineShow),
                "Alkalinity" => Some(Alkalinity),
                "Alkalinity Number" => Some(AlkalinityNumber),
                "Alpha-Cellulose" => Some(AlphaCellulose),
                "Average Speed" => Some(AverageSpeed),
                "Amines" => Some(Amines),
                "Average Molecular Weight" => Some(AverageMolecularWeight),
                "Flute Test" => Some(FluteTest),
                "Antioxidant" => Some(Antioxidant),
                "Average Pressure" => Some(AveragePressure),
                "API Gravity" => Some(ApiGravity),
                "Appearance" => Some(Appearance),
                "Ash Fusion Temperature" => Some(AshFusionTemperature),
                "Ash" => Some(Ash),
                "Assay" => Some(Assay),
                "Additive" => Some(Additive),
                "Number of Units Projected" => Some(NumberOfUnitsProjected),
                "Age" => Some(Age),
                "Average Temperature" => Some(AverageTemperature),
                "Remaining Economic Life" => Some(RemainingEconomicLife),
                "Remaining Physical Life" => Some(RemainingPhysicalLife),
                "Number of Comparable Sales" => Some(NumberOfComparableSales),
                "Arbor Size" => Some(ArborSize),
                "Billed Weight" => Some(BilledWeight),
                "Base Number" => Some(BaseNumber),
                "Number of Comparable Listings" => Some(NumberOfComparableListings),
                "Present Land Use" => Some(PresentLandUse),
                "Subject Phase Dwelling Units" => Some(SubjectPhaseDwellingUnits),
                "Octanol/Water Partition Coefficient" => {
                    Some(OctanolWaterPartitionCoefficient)
                }
                "Total Project Dwelling Units" => Some(TotalProjectDwellingUnits),
                "Barium" => Some(Barium),
                "Beryllium" => Some(Beryllium),
                "Billet Size" => Some(BilletSize),
                "Bias" => Some(Bias),
                "Perforation Bottom Depth" => Some(PerforationBottomDepth),
                "Boron Factor" => Some(BoronFactor),
                "Brinell" => Some(Brinell),
                "Berkelium" => Some(Berkelium),
                "Bromine" => Some(Bromine),
                "Bottomhole Pressure - Flowing" => Some(BottomholePressureFlowing),
                "Bottomhole Pressure - Shutin" => Some(BottomholePressureShutin),
                "Bark in Chips" => Some(BarkInChips),
                "Burst Index" => Some(BurstIndex),
                "Bulk" => Some(Bulk),
                "Blisters" => Some(Blisters),
                "Bend" => Some(Bend),
                "Amount Bound in Material" => Some(AmountBoundInMaterial),
                "Lateral Bow (Camber)" => Some(CodeBO),
                "Boiling Range" => Some(BoilingRange),
                "Boiling Point" => Some(BoilingPoint),
                "Breaks" => Some(Breaks),
                "Brightness" => Some(Brightness),
                "Breaking Strength" => Some(BreakingStrength),
                "Percent Bottom Sediment and Water" => {
                    Some(PercentBottomSedimentAndWater)
                }
                "Bursts" => Some(Bursts),
                "Buckles" => Some(Buckles),
                "Bulk Density" => Some(BulkDensity),
                "Basis Weight" => Some(BasisWeight),
                "Blood Alcohol" => Some(BloodAlcohol),
                "Actual New Repeated for Combination" => {
                    Some(ActualNewRepeatedForCombination)
                }
                "Color Grade" => Some(ColorGrade),
                "Carbonyl" => Some(Carbonyl),
                "Catalyst" => Some(Catalyst),
                "Maximum Contraction" => Some(MaximumContraction),
                "Color Quadrant" => Some(ColorQuadrant),
                "Caliper" => Some(Caliper),
                "Caustic Reaction Severity" => Some(CausticReactionSeverity),
                "Celsius" => Some(Celsius),
                "Composite Corrected Factor" => Some(CompositeCorrectedFactor),
                "Combined Center of Gravity" => Some(CombinedCenterOfGravity),
                "Compression" => Some(Compression),
                "Capacitance In" => Some(CapacitanceIn),
                "Capacitance Out" => Some(CapacitanceOut),
                "Cadmium" => Some(Cadmium),
                "Color Grayness RD" => Some(ColorGraynessRd),
                "Cesium" => Some(Cesium),
                "Chemical Addition Rate" => Some(ChemicalAdditionRate),
                "Chlorophyll-a" => Some(ChlorophyllA),
                "Concentration of Hazardous Component" => {
                    Some(ConcentrationOfHazardousComponent)
                }
                "Rate of Change" => Some(RateOfChange),
                "Chlorides" => Some(Chlorides),
                "Curium" => Some(Curium),
                "Cuene Intrinsic Viscosity" => Some(CueneIntrinsicViscosity),
                "Cycle Time" => Some(CycleTime),
                "Californium" => Some(Californium),
                "Cladding Thickness(% of Composite Thickness)" => Some(CodeCL),
                "Clarity" => Some(Clarity),
                "Calibration" => Some(Calibration),
                "Cleanliness" => Some(Cleanliness),
                "Cures" => Some(Cures),
                "Chlorine" => Some(Chlorine),
                "Core Loss" => Some(CoreLoss),
                "Coefficient Factor" => Some(CoefficientFactor),
                "Coercive Force" => Some(CoerciveForce),
                "Color" => Some(Color),
                "Concentration" => Some(Concentration),
                "Corrosiveness" => Some(Corrosiveness),
                "Cost" => Some(Cost),
                "Content" => Some(Content),
                "Crimp" => Some(Crimp),
                "Casing Pressure - Flowing" => Some(CasingPressureFlowing),
                "Casing Pressure - Shutin" => Some(CasingPressureShutin),
                "Cup Test Adhesion" => Some(CupTestAdhesion),
                "Cuts" => Some(Cuts),
                "Crosswise Spacing" => Some(CrosswiseSpacing),
                "Free Chlorine Residual" => Some(FreeChlorineResidual),
                "Compression Relaxation" => Some(CompressionRelaxation),
                "Crown" => Some(Crown),
                "Total Chlorine Residual" => Some(TotalChlorineResidual),
                "Cross Section" => Some(CrossSection),
                "Choke Size - Casing" => Some(ChokeSizeCasing),
                "Cost Realism" => Some(CostRealism),
                "Choke Size - Tubing" => Some(ChokeSizeTubing),
                "Center-to-center" => Some(CenterToCenter),
                "Coating" => Some(Coating),
                "Contact Time" => Some(ContactTime),
                "Coil Curvature" => Some(CoilCurvature),
                "Cure Time" => Some(CureTime),
                "Cuttable Width" => Some(CuttableWidth),
                "Charge Weight" => Some(ChargeWeight),
                "Calculated Value" => Some(CalculatedValue),
                "Contamination" => Some(Contamination),
                "Color Yellowness (+B)" => Some(CodeCYB),
                "Destination Weight Agreement" => Some(DestinationWeightAgreement),
                "Maximum Dilatation" => Some(MaximumDilatation),
                "Dispersing Agent" => Some(DispersingAgent),
                "Drying Agent" => Some(DryingAgent),
                "Dry Point" => Some(DryPoint),
                "Wear" => Some(Wear),
                "Horizontal" => Some(Horizontal),
                "Distillation Fraction" => Some(DistillationFraction),
                "Vertical" => Some(Vertical),
                "Dots per Inch" => Some(DotsPerInch),
                "Delta Value A" => Some(DeltaValueA),
                "Datum Depth" => Some(DatumDepth),
                "Delta Value B" => Some(DeltaValueB),
                "Ductile Class" => Some(DuctileClass),
                "Dirt Count" => Some(DirtCount),
                "Depth of Dents" => Some(DepthOfDents),
                "Defects" => Some(Defects),
                "De Minimis Level" => Some(DeMinimisLevel),
                "Distance Across Flats" => Some(DistanceAcrossFlats),
                "Direct Current" => Some(DirectCurrent),
                "Dysprosium" => Some(Dysprosium),
                "Diameter" => Some(Diameter),
                "Dilution Factor; An amount by which a quantity is diluted in order to be read on an instrument scale" => {
                    Some(CodeDIL)
                }
                "Distillation Range" => Some(DistillationRange),
                "Dispersion" => Some(Dispersion),
                "Dominant Wave Length" => Some(DominantWaveLength),
                "Color Bits in Palette" => Some(ColorBitsInPalette),
                "Delta Value L" => Some(DeltaValueL),
                "Dial Count" => Some(DialCount),
                "Dual Amplitude" => Some(DualAmplitude),
                "Dichloromethane Extract" => Some(DichloromethaneExtract),
                "Distributed Meter Factor" => Some(DistributedMeterFactor),
                "Density" => Some(Density),
                "Compressed File Size" => Some(CompressedFileSize),
                "Double Olsen Cup" => Some(DoubleOlsenCup),
                "Depth" => Some(Depth),
                "Degree of Polymerization" => Some(DegreeOfPolymerization),
                "Delta R" => Some(DeltaR),
                "Dryness" => Some(Dryness),
                "Distance Between Points" => Some(DistanceBetweenPoints),
                "Distance From Base Point" => Some(DistanceFromBasePoint),
                "Draw Tension" => Some(DrawTension),
                "Width, Boxcar Door" => Some(CodeDW),
                "Dew Point" => Some(DewPoint),
                "Dyeability" => Some(Dyeability),
                "Estimated New Weight" => Some(EstimatedNewWeight),
                "Extraneous Matter" => Some(ExtraneousMatter),
                "End" => Some(End),
                "Elongation" => Some(Elongation),
                "Edge Burr" => Some(EdgeBurr),
                "English Coil Dimensions" => Some(EnglishCoilDimensions),
                "Eddy" => Some(Eddy),
                "Einsteinium" => Some(Einsteinium),
                "Europium" => Some(Europium),
                "Equivalent Temperature" => Some(EquivalentTemperature),
                "Erbium" => Some(Erbium),
                "Expansion" => Some(Expansion),
                "Electrical Conductivity" => Some(ElectricalConductivity),
                "Elasticity" => Some(Elasticity),
                "Exposure Ceiling: Threshold Limit Value" => {
                    Some(ExposureCeilingThresholdLimitValue)
                }
                "Elevation" => Some(Elevation),
                "Immediately Dangerous to Life and Health" => {
                    Some(ImmediatelyDangerousToLifeAndHealth)
                }
                "Explosion Limit" => Some(ExplosionLimit),
                "Occupational Safety and Health Administration (OSHA) Permissible Exposure Limit" => {
                    Some(CodeELO)
                }
                "Occupational Safety and Health Administration (OSHA) Permissible Exposure Limit Ceiling" => {
                    Some(CodeELP)
                }
                "American Conference of Government Industrial Hygienists (ACGIH) Threshold Limit Value: Short-Term Exposure" => {
                    Some(CodeELS)
                }
                "American Conference of Governmental Industrial Hygienists (ACGIH) Threshold Limit Value: Time Weighted Average" => {
                    Some(CodeELT)
                }
                "Unshielded Exposure Rate" => Some(UnshieldedExposureRate),
                "American Industrial Hygienists Association (AIHA) Work Environment Exposure Level (WEEL)" => {
                    Some(CodeELW)
                }
                "American Industrial Hygienists Association (AIHA) Work Environment Exposure Level (WEEL): Time Weighted Average" => {
                    Some(CodeELX)
                }
                "Elmendorf Tear" => Some(ElmendorfTear),
                "Entanglement" => Some(Entanglement),
                "Exciting Power" => Some(ExcitingPower),
                "Expected Product Life" => Some(ExpectedProductLife),
                "Edge" => Some(Edge),
                "End Point" => Some(EndPoint),
                "Evaporation Loss" => Some(EvaporationLoss),
                "Evaporation Rate" => Some(EvaporationRate),
                "Empty Weight" => Some(EmptyWeight),
                "Eccentricity" => Some(Eccentricity),
                "Exhaust Benzene Emissions" => Some(ExhaustBenzeneEmissions),
                "Extractables" => Some(Extractables),
                "Deficit Weight" => Some(DeficitWeight),
                "Fire Point" => Some(FirePoint),
                "Radiated Power" => Some(RadiatedPower),
                "Output Power (Peak Envelope)" => Some(CodeF3),
                "Height above Average Terrain" => Some(HeightAboveAverageTerrain),
                "Ground Elevation" => Some(GroundElevation),
                "Height to Tip" => Some(HeightToTip),
                "Radius from a Location" => Some(RadiusFromALocation),
                "Radius from Coordinates" => Some(RadiusFromCoordinates),
                "Operating Hours" => Some(OperatingHours),
                "Fluorine" => Some(Fluorine),
                "Flare" => Some(Flare),
                "Acetate Break Point" => Some(AcetateBreakPoint),
                "Short Cycle Flatness" => Some(ShortCycleFlatness),
                "Frequency of Operation" => Some(FrequencyOfOperation),
                "Fermium" => Some(Fermium),
                "Francium" => Some(Francium),
                "Freezing Point" => Some(FreezingPoint),
                "Finish" => Some(Finish),
                "Filament Count" => Some(FilamentCount),
                "Filter Number" => Some(FilterNumber),
                "Fineness" => Some(Fineness),
                "Filterability" => Some(Filterability),
                "Face Width" => Some(FaceWidth),
                "Fluid Consistency" => Some(FluidConsistency),
                "Longitudinal Flatness" => Some(LongitudinalFlatness),
                "Fluid Point" => Some(FluidPoint),
                "Fiber Length" => Some(FiberLength),
                "Fluid Level Above Pump" => Some(FluidLevelAbovePump),
                "Fluting" => Some(Fluting),
                "Flavor Threshold" => Some(FlavorThreshold),
                "Flammability Limits" => Some(FlammabilityLimits),
                "Flammability" => Some(Flammability),
                "Flatness" => Some(Flatness),
                "Fines Retained on Screen" => Some(FinesRetainedOnScreen),
                "Fines" => Some(Fines),
                "Foam" => Some(Foam),
                "Front Over-Hang of Vehicle" => Some(FrontOverHangOfVehicle),
                "Fouling Index" => Some(FoulingIndex),
                "Foreign Matter" => Some(ForeignMatter),
                "Flashpoint" => Some(Flashpoint),
                "Acetate Plugging Value" => Some(AcetatePluggingValue),
                "Frequency" => Some(Frequency),
                "Flow Rate" => Some(FlowRate),
                "Fold Strength" => Some(FoldStrength),
                "Free Swelling Index" => Some(FreeSwellingIndex),
                "Flange Thickness" => Some(FlangeThickness),
                "Azimuth" => Some(Azimuth),
                "Full Load" => Some(FullLoad),
                "Transverse Flatness" => Some(TransverseFlatness),
                "Flange Width" => Some(FlangeWidth),
                "Filler" => Some(Filler),
                "Beam" => Some(Beam),
                "Output Power (Mean RF)" => Some(CodeFZ),
                "Gross Weight" => Some(GrossWeight),
                "Gutter" => Some(Gutter),
                "Grain Size" => Some(GrainSize),
                "Tilt" => Some(Tilt),
                "G-Force" => Some(GForce),
                "Grain" => Some(Grain),
                "Guided Bends Root" => Some(GuidedBendsRoot),
                "Gadolinium" => Some(Gadolinium),
                "Gold" => Some(Gold),
                "Guided Bends Face" => Some(GuidedBendsFace),
                "Gel" => Some(Gel),
                "Guided Bends Side" => Some(GuidedBendsSide),
                "Gauge" => Some(Gauge),
                "Gas Gravity" => Some(GasGravity),
                "Gallium" => Some(Gallium),
                "Grit, Brushed" => Some(CodeGI),
                "Gas Injection Test Rate" => Some(GasInjectionTestRate),
                "Grit, Unbrushed" => Some(CodeGJ),
                "Tinting Strength" => Some(TintingStrength),
                "Gloss" => Some(Gloss),
                "Ground Level Elevation" => Some(GroundLevelElevation),
                "Emulsion Gloss" => Some(EmulsionGloss),
                "Gross to Net Conversion Factor" => Some(GrossToNetConversionFactor),
                "Particle End Point" => Some(ParticleEndPoint),
                "Gas-to-Oil Ratio" => Some(GasToOilRatio),
                "Group Package Separation" => Some(GroupPackageSeparation),
                "Scattered Particles" => Some(ScatteredParticles),
                "Gravity" => Some(Gravity),
                "Gas Test Rate" => Some(GasTestRate),
                "Graininess" => Some(Graininess),
                "Carbon Black Undertone" => Some(CarbonBlackUndertone),
                "Glass Transition Temperature" => Some(GlassTransitionTemperature),
                "Gross Weight, Maximum" => Some(CodeGW),
                "Ground Water Reference Point" => Some(GroundWaterReferencePoint),
                "Heavies" => Some(Heavies),
                "Water Volume" => Some(WaterVolume),
                "Net Explosive Weight" => Some(NetExplosiveWeight),
                "Recommended Exposure Limit" => Some(RecommendedExposureLimit),
                "Hydro Pressure" => Some(HydroPressure),
                "Hardening Rate" => Some(HardeningRate),
                "Haze" => Some(Haze),
                "Heavy Aluminas" => Some(HeavyAluminas),
                "Helium" => Some(Helium),
                "Horizontal Center of Gravity" => Some(HorizontalCenterOfGravity),
                "Distance to Endpoint" => Some(DistanceToEndpoint),
                "Emission Factor" => Some(EmissionFactor),
                "Heat Content" => Some(HeatContent),
                "Pollutant Emission" => Some(PollutantEmission),
                "Population" => Some(Population),
                "Release Duration" => Some(ReleaseDuration),
                "Released Quantity" => Some(ReleasedQuantity),
                "Release Rate" => Some(ReleaseRate),
                "Sulfur Content" => Some(SulfurContent),
                "Wind Speed" => Some(WindSpeed),
                "Acute Toxicity" => Some(AcuteToxicity),
                "Chronic Toxicity" => Some(ChronicToxicity),
                "Discharges" => Some(Discharges),
                "Effectiveness" => Some(Effectiveness),
                "Fertilization" => Some(Fertilization),
                "Samples in Compliance" => Some(SamplesInCompliance),
                "Toxicity" => Some(Toxicity),
                "High Propagation Delay Time" => Some(HighPropagationDelayTime),
                "Heavy Silicates" => Some(HeavySilicates),
                "Hardness" => Some(Hardness),
                "Heavy Sulfides" => Some(HeavySulfides),
                "Heavy Globular Oxides" => Some(HeavyGlobularOxides),
                "Hemispherical Point" => Some(HemisphericalPoint),
                "Hafnium" => Some(Hafnium),
                "High Boilers" => Some(HighBoilers),
                "Holmium" => Some(Holmium),
                "Human Factors" => Some(HumanFactors),
                "Heat Loss" => Some(HeatLoss),
                "Height, Maximum" => Some(CodeHM),
                "Holes" => Some(Holes),
                "Heat of Combustion" => Some(HeatOfCombustion),
                "Height of Tread Plate Pattern" => Some(HeightOfTreadPlatePattern),
                "Height of Runners" => Some(HeightOfRunners),
                "Height" => Some(Height),
                "Heat Equivalency" => Some(HeatEquivalency),
                "Heavy Metals" => Some(HeavyMetals),
                "Softening Point" => Some(SofteningPoint),
                "Hydroxyl Number" => Some(HydroxylNumber),
                "Hazepoint" => Some(Hazepoint),
                "Hazardous Component" => Some(HazardousComponent),
                "Heat Input" => Some(HeatInput),
                "Imperfections - Thicks" => Some(ImperfectionsThicks),
                "Impact Energy" => Some(ImpactEnergy),
                "Incremental Distance" => Some(IncrementalDistance),
                "Inside Diameter" => Some(InsideDiameter),
                "Identification" => Some(Identification),
                "Imperfections - Thins" => Some(ImperfectionsThins),
                "Input Low Voltage" => Some(InputLowVoltage),
                "Granulated Ingot Size" => Some(GranulatedIngotSize),
                "Autoignition Temperature" => Some(AutoignitionTemperature),
                "Input Gas Rate" => Some(InputGasRate),
                "Input Current" => Some(InputCurrent),
                "Inherent Viscosity" => Some(InherentViscosity),
                "Input Setup Time" => Some(InputSetupTime),
                "Input Hold Time" => Some(InputHoldTime),
                "Indium" => Some(Indium),
                "Imperfections - Neps" => Some(ImperfectionsNeps),
                "Iridium" => Some(Iridium),
                "Impact Adhesion" => Some(ImpactAdhesion),
                "Impurities" => Some(Impurities),
                "Input Low Current" => Some(InputLowCurrent),
                "Initial Deformation Point" => Some(InitialDeformationPoint),
                "Insolubles" => Some(Insolubles),
                "Input High Current" => Some(InputHighCurrent),
                "Input Leakage Current" => Some(InputLeakageCurrent),
                "Primary Irritation Index" => Some(PrimaryIrritationIndex),
                "Input Clamp Diode Voltage" => Some(InputClampDiodeVoltage),
                "Ideal Diameter" => Some(IdealDiameter),
                "Inclusion Rating" => Some(InclusionRating),
                "Input High Voltage" => Some(InputHighVoltage),
                "Interrupt Pulse Period" => Some(InterruptPulsePeriod),
                "Integrated Differential" => Some(IntegratedDifferential),
                "Interrupt Setup Time" => Some(InterruptSetupTime),
                "Iodine" => Some(Iodine),
                "Incrustation Factor" => Some(IncrustationFactor),
                "Inside Diameter, Minimum" => Some(CodeIX),
                "Index Differential" => Some(IndexDifferential),
                "Induction" => Some(Induction),
                "Ingot Pound Size" => Some(IngotPoundSize),
                "Junction Temperature" => Some(JunctionTemperature),
                "Jominy Hardenability" => Some(JominyHardenability),
                "Knoop" => Some(Knoop),
                "Krypton" => Some(Krypton),
                "K&N Holdout" => Some(CodeKN),
                "Strength Coefficient" => Some(StrengthCoefficient),
                "Legal Weight" => Some(LegalWeight),
                "Leaf Grade" => Some(LeafGrade),
                "Lights" => Some(Lights),
                "Lanthanum" => Some(Lanthanum),
                "Langlier Index" => Some(LanglierIndex),
                "Lithium" => Some(Lithium),
                "Low Propagation Delay Time" => Some(LowPropagationDelayTime),
                "Lethal Concentration, 50% (\"LC-50\")" => Some(CodeLC5),
                "Longitudinal Center of Gravity" => Some(LongitudinalCenterOfGravity),
                "Lawrencium" => Some(Lawrencium),
                "Lethal Dose, 50% (\"LD-50\")" => Some(CodeLD5),
                "Limited Dome Height (LDH)" => Some(CodeLDH),
                "Lutetium" => Some(Lutetium),
                "Leaf" => Some(Leaf),
                "Long Fibers" => Some(LongFibers),
                "Leg" => Some(Leg),
                "Liquid Injection Test Rate" => Some(LiquidInjectionTestRate),
                "Limit Intrinsic Viscosity" => Some(LimitIntrinsicViscosity),
                "Long Leg" => Some(LongLeg),
                "Light Load" => Some(LightLoad),
                "Length, Maximum" => Some(CodeLM),
                "Length" => Some(Length),
                "Long Length" => Some(LongLength),
                "Loss on Ignition" => Some(LossOnIgnition),
                "Loss on Drying" => Some(LossOnDrying),
                "Low Boilers" => Some(LowBoilers),
                "Camber of Pattern Line" => Some(CamberOfPatternLine),
                "Liquefied Petroleum Gas Factor" => Some(LiquefiedPetroleumGasFactor),
                "Labeled Product Life" => Some(LabeledProductLife),
                "Line Pressure" => Some(LinePressure),
                "Short Leg" => Some(ShortLeg),
                "Lock Seam Adhesion" => Some(LockSeamAdhesion),
                "Length Shrinkage" => Some(LengthShrinkage),
                "Linear Sheet Swelling" => Some(LinearSheetSwelling),
                "Lengthwise Spacing" => Some(LengthwiseSpacing),
                "Liner Top Depth" => Some(LinerTopDepth),
                "Long Width" => Some(LongWidth),
                "Minimum Weight (for Weight)" => Some(CodeM),
                "Melt Range" => Some(MeltRange),
                "Maximum Differential Pressure" => Some(MaximumDifferentialPressure),
                "Maximum Static Pressure" => Some(MaximumStaticPressure),
                "Area" => Some(Area),
                "Minimum Speed" => Some(MinimumSpeed),
                "Maximum Speed" => Some(MaximumSpeed),
                "Mean Average" => Some(MeanAverage),
                "Maturity" => Some(Maturity),
                "Mendelevium" => Some(Mendelevium),
                "Min./Max Cuttable Width" => Some(MinMaxCuttableWidth),
                "Magnetic Contamination" => Some(MagneticContamination),
                "Measurement Voltage" => Some(MeasurementVoltage),
                "Method Detection Limit; Minimum concentration of a substance that can be measured and reported with 99% confidence that analyte concentration is greater than zero" => {
                    Some(CodeMDL)
                }
                "Maximum Input Low Voltage" => Some(MaximumInputLowVoltage),
                "Media Depth" => Some(MediaDepth),
                "Meter Factor" => Some(MeterFactor),
                "Melt Time" => Some(MeltTime),
                "Meat Protein" => Some(MeatProtein),
                "Efficient Rate-Reservoir" => Some(EfficientRateReservoir),
                "Minimum Input High Voltage" => Some(MinimumInputHighVoltage),
                "Mercury" => Some(Mercury),
                "Melting Point" => Some(MeltingPoint),
                "Highest Torque" => Some(HighestTorque),
                "Minimum" => Some(Minimum),
                "Micronaire" => Some(Micronaire),
                "Milk Fat" => Some(MilkFat),
                "Major Section (Stepped)" => Some(CodeMJ),
                "Microseperometer (MSEP)" => Some(CodeMK),
                "Management" => Some(Management),
                "Minimum Average" => Some(MinimumAverage),
                "Mottles" => Some(Mottles),
                "Moisture" => Some(Moisture),
                "Mortality" => Some(Mortality),
                "Mullen Pop" => Some(MullenPop),
                "Production Rate-Well" => Some(ProductionRateWell),
                "Many Press Test" => Some(ManyPressTest),
                "MCQuaid" => Some(McQuaid),
                "Module R (R Bar)" => Some(CodeMR),
                "Minor Section (Stepped)" => Some(CodeMS),
                "Moisture Content" => Some(MoistureContent),
                "Maximum Total Depth" => Some(MaximumTotalDepth),
                "Multiplier" => Some(Multiplier),
                "Mullen" => Some(Mullen),
                "Maximum Average" => Some(MaximumAverage),
                "Molecular Weight" => Some(MolecularWeight),
                "Maximum" => Some(Maximum),
                "Magnetizing Field" => Some(MagnetizingField),
                "Actual Net Weight" => Some(ActualNetWeight),
                "Number per Package" => Some(NumberPerPackage),
                "Number per Bundle" => Some(NumberPerBundle),
                "Number per Coil Group" => Some(NumberPerCoilGroup),
                "Neodymium" => Some(Neodymium),
                "Neutralization Number" => Some(NeutralizationNumber),
                "Neon" => Some(Neon),
                "Nobelium" => Some(Nobelium),
                "Number of Items per Package Label" => Some(NumberOfItemsPerPackageLabel),
                "Number of Splices per Package Label" => {
                    Some(NumberOfSplicesPerPackageLabel)
                }
                "Nil Ductility Test" => Some(NilDuctilityTest),
                "Number of Sheets per Package Label" => {
                    Some(NumberOfSheetsPerPackageLabel)
                }
                "Nesting Factor" => Some(NestingFactor),
                "Number per Lift" => Some(NumberPerLift),
                "Number Pkgs. per Master Pack" => Some(NumberPkgsPerMasterPack),
                "Net Net Weight" => Some(NetNetWeight),
                "Nominal (Target, Aim)" => Some(CodeNO),
                "Number of Cosigners" => Some(NumberOfCosigners),
                "Non-Volatile Matter" => Some(NonVolatileMatter),
                "Number of References" => Some(NumberOfReferences),
                "NOx Emissions Performance" => Some(NOxEmissionsPerformance),
                "Percent of Specified" => Some(PercentOfSpecified),
                "Number per Skid" => Some(NumberPerSkid),
                "Number per Unit" => Some(NumberPerUnit),
                "N Value" => Some(NValue),
                "Excess Weight Over Maximum" => Some(ExcessWeightOverMaximum),
                "Orifice - Inside Diameter" => Some(OrificeInsideDiameter),
                "Offset" => Some(Offset),
                "Observed American Petroleum Institute Gravity" => {
                    Some(ObservedAmericanPetroleumInstituteGravity)
                }
                "Osmium" => Some(Osmium),
                "Observed Temperature" => Some(ObservedTemperature),
                "Output Low Voltage" => Some(OutputLowVoltage),
                "Oil/Condensate Gravity" => Some(OilCondensateGravity),
                "Oil/Condensate Test Rate" => Some(OilCondensateTestRate),
                "Outside Diameter" => Some(OutsideDiameter),
                "Odor" => Some(Odor),
                "Output Low Current" => Some(OutputLowCurrent),
                "Output High Voltage" => Some(OutputHighVoltage),
                "Output High Current" => Some(OutputHighCurrent),
                "Overhead Height, Receiving Door" => Some(CodeOH),
                "Output Off Current Low" => Some(OutputOffCurrentLow),
                "Oil" => Some(Oil),
                "Output Off Current High" => Some(OutputOffCurrentHigh),
                "Output Short-Circuit Current" => Some(OutputShortCircuitCurrent),
                "Output Disable Time from Low Level of a 3-State Output" => {
                    Some(OutputDisableTimeFromLowLevelOfA3StateOutput)
                }
                "Olefins" => Some(Olefins),
                "Outside Diameter, Maximum" => Some(CodeOM),
                "Output Disable Time from High Level of a 3-State Output" => {
                    Some(OutputDisableTimeFromHighLevelOfA3StateOutput)
                }
                "Output Enable Time from Low Level of a 3-State Output" => {
                    Some(OutputEnableTimeFromLowLevelOfA3StateOutput)
                }
                "Openness" => Some(Openness),
                "Output Enable Time from High Level of a 3-State Output" => {
                    Some(OutputEnableTimeFromHighLevelOfA3StateOutput)
                }
                "Distance Between Outside Runners" => Some(DistanceBetweenOutsideRunners),
                "Organic Carbon" => Some(OrganicCarbon),
                "Open Circuits" => Some(OpenCircuits),
                "Output Delay Time" => Some(OutputDelayTime),
                "Others Each" => Some(OthersEach),
                "Odor Threshold" => Some(OdorThreshold),
                "Others Total" => Some(OthersTotal),
                "Opacity" => Some(Opacity),
                "Overall Width" => Some(OverallWidth),
                "Ownership Share" => Some(OwnershipShare),
                "Oxidizable Substance" => Some(OxidizableSubstance),
                "Oxidizing Substance" => Some(OxidizingSubstance),
                "Operating Weight" => Some(OperatingWeight),
                "Price" => Some(Price),
                "Package Separation" => Some(PackageSeparation),
                "Particle Size" => Some(ParticleSize),
                "Pressure" => Some(Pressure),
                "Plug Back Total Depth" => Some(PlugBackTotalDepth),
                "Per Hundred Linear Yards" => Some(PerHundredLinearYards),
                "Platinum" => Some(Platinum),
                "Casing/Liner Tubing Depth" => Some(CasingLinerTubingDepth),
                "Pump Depth from Ground" => Some(PumpDepthFromGround),
                "Potassium" => Some(Potassium),
                "Magnetic Permeability" => Some(MagneticPermeability),
                "Promethium" => Some(Promethium),
                "Perforation Feet Open" => Some(PerforationFeetOpen),
                "Polonium" => Some(Polonium),
                "Pulse Setup Time" => Some(PulseSetupTime),
                "pH" => Some(PH),
                "Hardwood Fiber" => Some(HardwoodFiber),
                "Pulse Hold Time" => Some(PulseHoldTime),
                "Pick Off" => Some(PickOff),
                "Pulse Width" => Some(PulseWidth),
                "Pulse Recovery Time" => Some(PulseRecoveryTime),
                "Percent Defective" => Some(PercentDefective),
                "Practice" => Some(Practice),
                "Palladium" => Some(Palladium),
                "Percent of Order (-, +)" => Some(CodePO),
                "Completion" => Some(Completion),
                "Physical Description - Outer Diameter" => {
                    Some(PhysicalDescriptionOuterDiameter)
                }
                "Pour Point" => Some(PourPoint),
                "Powder/Paste Package Size" => Some(PowderPastePackageSize),
                "Proprietary Shade" => Some(ProprietaryShade),
                "Plutonium" => Some(Plutonium),
                "Practical Quantitation Limit; Lowest concentration of a substance which can be consistently determined within +/- 20% of the true concentration by 75% of the laboratories tested in a performance evaluation study" => {
                    Some(CodePQL)
                }
                "Praseodymium" => Some(Praseodymium),
                "Proportion Alive" => Some(ProportionAlive),
                "Prior Experience" => Some(PriorExperience),
                "Pressure Factor" => Some(PressureFactor),
                "Product Index" => Some(ProductIndex),
                "Product Level" => Some(ProductLevel),
                "Proportion Normal" => Some(ProportionNormal),
                "Processability" => Some(Processability),
                "Product Reportable Quantity" => Some(ProductReportableQuantity),
                "Porosity" => Some(Porosity),
                "Proportion Fertilized" => Some(ProportionFertilized),
                "Protactinium" => Some(Protactinium),
                "Percent Solution Actual" => Some(PercentSolutionActual),
                "Past Performance" => Some(PastPerformance),
                "Softwood Fiber" => Some(SoftwoodFiber),
                "Pits" => Some(Pits),
                "Pressure Base" => Some(PressureBase),
                "Picks" => Some(Picks),
                "Purchased Width" => Some(PurchasedWidth),
                "Processed Waste" => Some(ProcessedWaste),
                "Physical Description - Weight" => Some(PhysicalDescriptionWeight),
                "Power Factor" => Some(PowerFactor),
                "Purity" => Some(Purity),
                "Percent of Water" => Some(PercentOfWater),
                "Pipe Size Nominal" => Some(PipeSizeNominal),
                "Volatile Organic Compounds Plus Water" => {
                    Some(VolatileOrganicCompoundsPlusWater)
                }
                "Quality Index" => Some(QualityIndex),
                "Quantity or Loading Average" => Some(QuantityOrLoadingAverage),
                "Quantity or Loading Maximum" => Some(QuantityOrLoadingMaximum),
                "Quality or Concentration Average" => Some(QualityOrConcentrationAverage),
                "Quality or Concentration Minimum" => Some(QualityOrConcentrationMinimum),
                "Quality or Concentration Maximum" => Some(QualityOrConcentrationMaximum),
                "Duration" => Some(Duration),
                "Abundance" => Some(Abundance),
                "Biomass" => Some(Biomass),
                "Size Class" => Some(SizeClass),
                "Quality" => Some(Quality),
                "Reportable Quantity" => Some(ReportableQuantity),
                "Per Unit Dunnage" => Some(PerUnitDunnage),
                "Hemoglobin" => Some(Hemoglobin),
                "Hematocrit" => Some(Hematocrit),
                "Epoetin Starting Dosage" => Some(EpoetinStartingDosage),
                "Creatinine" => Some(Creatinine),
                "Speed" => Some(Speed),
                "Speed Limit" => Some(SpeedLimit),
                "Relative Fraction of Pure Long-Chain Cellulose" => {
                    Some(RelativeFractionOfPureLongChainCellulose)
                }
                "Relative Fraction of Total Cellulose" => {
                    Some(RelativeFractionOfTotalCellulose)
                }
                "Relative Humidity" => Some(RelativeHumidity),
                "Radius" => Some(Radius),
                "Roof Adjustment Factor" => Some(RoofAdjustmentFactor),
                "Range Value" => Some(RangeValue),
                "Radius of Corner" => Some(RadiusOfCorner),
                "Readpoint" => Some(Readpoint),
                "Ream Weight" => Some(ReamWeight),
                "Reactivity" => Some(Reactivity),
                "Reducing Substance" => Some(ReducingSubstance),
                "Refining" => Some(Refining),
                "Refractive Index" => Some(RefractiveIndex),
                "Reflectance" => Some(Reflectance),
                "Resistance" => Some(Resistance),
                "Resistivity" => Some(Resistivity),
                "Radium" => Some(Radium),
                "Rhenium" => Some(Rhenium),
                "Rubidium" => Some(Rubidium),
                "Rockwell-C" => Some(RockwellC),
                "Rockwell-B" => Some(RockwellB),
                "Reduction Ration" => Some(ReductionRation),
                "RMS Range (Side 1)" => Some(CodeRM),
                "Required Interrupt Release" => Some(RequiredInterruptRelease),
                "Reset Pulse Width" => Some(ResetPulseWidth),
                "Rear Over-Hang of Vehicle" => Some(RearOverHangOfVehicle),
                "Oxygen from a Renewable Oxygenate" => {
                    Some(OxygenFromARenewableOxygenate)
                }
                "Reduction of Area" => Some(ReductionOfArea),
                "Radon" => Some(Radon),
                "Reduction Ratio" => Some(ReductionRatio),
                "RMS Range (Side 2)" => Some(CodeRS),
                "Roll Size" => Some(RollSize),
                "Rounds Ammunition/Military" => Some(RoundsAmmunitionMilitary),
                "Reporting Temperature Base" => Some(ReportingTemperatureBase),
                "Rhodium" => Some(Rhodium),
                "Usage Deviation (Applies to Kilowatt Hours, Kilowatt Demand and Reactive Demand)" => {
                    Some(CodeRUD)
                }
                "Ruthenium" => Some(Ruthenium),
                "Reid Vapor Pressure" => Some(ReidVaporPressure),
                "Rolling Width" => Some(RollingWidth),
                "Ridges" => Some(Ridges),
                "Ratio" => Some(Ratio),
                "State Weight" => Some(StateWeight),
                "Smoothness" => Some(Smoothness),
                "Selvedge on Beam" => Some(SelvedgeOnBeam),
                "Sheffield Smoothness" => Some(SheffieldSmoothness),
                "Surface Strength" => Some(SurfaceStrength),
                "Stiffness" => Some(Stiffness),
                "Saturation" => Some(Saturation),
                "Sediment" => Some(Sediment),
                "Solubility" => Some(Solubility),
                "Site Atmospheric Pressure" => Some(SiteAtmosphericPressure),
                "Pulp Impurities" => Some(PulpImpurities),
                "Start" => Some(Start),
                "Hemicellulose" => Some(Hemicellulose),
                "Sort Code CIE LAB" => Some(SortCodeCieLab),
                "Salinity; Salt level in a sample of seawater" => Some(CodeSAL),
                "Saponification Number" => Some(SaponificationNumber),
                "Sort Code CMC" => Some(SortCodeCmc),
                "Schedule Number (Pipe Size)" => Some(CodeSC),
                "Schedule" => Some(Schedule),
                "Strength" => Some(Strength),
                "Selvage Left" => Some(SelvageLeft),
                "Severity" => Some(Severity),
                "Samarium" => Some(Samarium),
                "Short Fiber Content" => Some(ShortFiberContent),
                "Slit Width" => Some(SlitWidth),
                "Strontium" => Some(Strontium),
                "Shelf Life" => Some(ShelfLife),
                "Supply Current" => Some(SupplyCurrent),
                "Silica (Silicon Dioxide)" => Some(CodeSIL),
                "Short Circuits" => Some(ShortCircuits),
                "Shrinkage" => Some(Shrinkage),
                "Short Length" => Some(ShortLength),
                "Solderability" => Some(Solderability),
                "Slagging Index" => Some(SlaggingIndex),
                "Shear" => Some(Shear),
                "SAM-B Rating" => Some(SamBRating),
                "SAM-D Rating" => Some(SamDRating),
                "Stain" => Some(Stain),
                "Sort Code CIE LCH" => Some(SortCodeCieLch),
                "Solids" => Some(Solids),
                "Softening Range" => Some(SofteningRange),
                "Splinter Count" => Some(SplinterCount),
                "Specific Gravity" => Some(SpecificGravity),
                "Sphere" => Some(Sphere),
                "Separator Pressure" => Some(SeparatorPressure),
                "Static Pressure" => Some(StaticPressure),
                "Shipped Quantity" => Some(ShippedQuantity),
                "Selvage Right" => Some(SelvageRight),
                "Silver" => Some(Silver),
                "Stop Recovery Startup Time" => Some(StopRecoveryStartupTime),
                "Stability" => Some(Stability),
                "Short Term Exposure Limit" => Some(ShortTermExposureLimit),
                "Staple" => Some(Staple),
                "Shipped Units" => Some(ShippedUnits),
                "Suspended Matter" => Some(SuspendedMatter),
                "Surface Roughness" => Some(SurfaceRoughness),
                "Surface Tension" => Some(SurfaceTension),
                "Scandium" => Some(Scandium),
                "Survival" => Some(Survival),
                "Short Width" => Some(ShortWidth),
                "Sodium" => Some(Sodium),
                "S10 Minus S18 Value" => Some(S10MinusS18Value),
                "Service Interrupt Duration" => Some(ServiceInterruptDuration),
                "Skid Height" => Some(SkidHeight),
                "Tare Weight" => Some(TareWeight),
                "Tire Pressure" => Some(TirePressure),
                "Tube - Inside Diameter" => Some(TubeInsideDiameter),
                "Technical" => Some(Technical),
                "Single End Break" => Some(SingleEndBreak),
                "Skein Break" => Some(SkeinBreak),
                "T50" => Some(T50),
                "T90" => Some(T90),
                "Thickness Heavy End (Tapered/Stepped)" => Some(CodeTA),
                "Taste" => Some(Taste),
                "Thickness Small End (Tapered/Stepped)" => Some(CodeTB),
                "Temperature" => Some(Temperature),
                "Tire Tread Contact Length" => Some(TireTreadContactLength),
                "Thin Aluminas" => Some(ThinAluminas),
                "Perforation Top Depth" => Some(PerforationTopDepth),
                "Tenacity" => Some(Tenacity),
                "Autodecomposition Temperature" => Some(AutodecompositionTemperature),
                "Storage Temperature" => Some(StorageTemperature),
                "Texture" => Some(Texture),
                "Tensile" => Some(Tensile),
                "Thin Sulfides" => Some(ThinSulfides),
                "Thickness" => Some(Thickness),
                "Thin Silicates" => Some(ThinSilicates),
                "Total Supply Current" => Some(TotalSupplyCurrent),
                "Timer Pulse Width" => Some(TimerPulseWidth),
                "Tapered/Stepped Length Type" => Some(TaperedSteppedLengthType),
                "Length Type: Multiples" => Some(LengthTypeMultiples),
                "Timer Period" => Some(TimerPeriod),
                "Terbium" => Some(Terbium),
                "Aquatic Toxicity" => Some(AquaticToxicity),
                "Torque" => Some(Torque),
                "Toxic Emissions Performance" => Some(ToxicEmissionsPerformance),
                "Thorium" => Some(Thorium),
                "Temperature Factor" => Some(TemperatureFactor),
                "Tubing Pressure - Flowing" => Some(TubingPressureFlowing),
                "Threshold Planning Quantity" => Some(ThresholdPlanningQuantity),
                "Tubing Pressure - Shutin" => Some(TubingPressureShutin),
                "Thin Globular Oxides" => Some(ThinGlobularOxides),
                "Length Type: Random" => Some(LengthTypeRandom),
                "Trash Area" => Some(TrashArea),
                "Trash Count" => Some(TrashCount),
                "Tire Diameter" => Some(TireDiameter),
                "Transmittance" => Some(Transmittance),
                "Transmissivity; Measure of the quantity of light that passes through a given volume of seawater; also used to measure turbidity and to estimate plant growing zones in the ocean" => {
                    Some(CodeTRS)
                }
                "Transmissivity Pathlength; The length of the path taken to arrive at transmissivity measurements" => {
                    Some(CodeTRT)
                }
                "Length Type: Specific" => Some(LengthTypeSpecific),
                "Trim Size" => Some(TrimSize),
                "Time" => Some(Time),
                "Trailer Tongue Length" => Some(TrailerTongueLength),
                "Technetium" => Some(Technetium),
                "Turbidity" => Some(Turbidity),
                "Thallium" => Some(Thallium),
                "Maximum True Vertical Depth" => Some(MaximumTrueVerticalDepth),
                "Top" => Some(Top),
                "Tire Width" => Some(TireWidth),
                "Thulium" => Some(Thulium),
                "Tear Strength" => Some(TearStrength),
                "Weight per Unit" => Some(WeightPerUnit),
                "Uranium" => Some(Uranium),
                "Cube" => Some(Cube),
                "Usage" => Some(Usage),
                "Unipunch Adhesion" => Some(UnipunchAdhesion),
                "Uniformity" => Some(Uniformity),
                "Unknowns" => Some(Unknowns),
                "Oxygenation Level" => Some(OxygenationLevel),
                "Vapor Density" => Some(VaporDensity),
                "Vapor Pressure" => Some(VaporPressure),
                "V-Bend Adhesion" => Some(VBendAdhesion),
                "Vertical Center of Gravity" => Some(VerticalCenterOfGravity),
                "Height, Van Door" => Some(CodeVH),
                "Vinyl" => Some(Vinyl),
                "Viscosity" => Some(Viscosity),
                "Voltage" => Some(Voltage),
                "VOC Emissions Performance" => Some(VocEmissionsPerformance),
                "Volume" => Some(Volume),
                "Volatiles" => Some(Volatiles),
                "Volatiles by Volume" => Some(VolatilesByVolume),
                "Volatiles by Weight" => Some(VolatilesByWeight),
                "Volume Split to Others" => Some(VolumeSplitToOthers),
                "Width, Van Door" => Some(CodeVW),
                "Volume Weight" => Some(VolumeWeight),
                "Reformulated Fuel Level" => Some(ReformulatedFuelLevel),
                "Weight per Unit of Area" => Some(WeightPerUnitOfArea),
                "Web" => Some(Web),
                "Web Depth/Height" => Some(WebDepthHeight),
                "Width" => Some(Width),
                "Water Depth" => Some(WaterDepth),
                "Wolfram" => Some(Wolfram),
                "Weight Loss" => Some(WeightLoss),
                "Wait Recovery Startup Time" => Some(WaitRecoveryStartupTime),
                "Whiteness" => Some(Whiteness),
                "Winding Loss" => Some(WindingLoss),
                "Wall Thickness" => Some(WallThickness),
                "Width, Maximum" => Some(CodeWM),
                "Water-Oil Distribution Coefficient" => {
                    Some(WaterOilDistributionCoefficient)
                }
                "Wellhead Pressure-Flowing" => Some(WellheadPressureFlowing),
                "Water/Product Level" => Some(WaterProductLevel),
                "Wellhead Pressure Shutin" => Some(WellheadPressureShutin),
                "Wrinkles" => Some(Wrinkles),
                "Water Test Rate" => Some(WaterTestRate),
                "Width Shrinkage" => Some(WidthShrinkage),
                "Weight" => Some(Weight),
                "Water/Tank Bottom Level" => Some(WaterTankBottomLevel),
                "Weight per Unit of Length" => Some(WeightPerUnitOfLength),
                "Wax Pick" => Some(WaxPick),
                "Maximum Weight (for Rate)" => Some(CodeX),
                "Xenon" => Some(Xenon),
                "Side Height, Flat Bed With Removable Sides" => Some(CodeXH),
                "Specified" => Some(Specified),
                "Squareness" => Some(Squareness),
                "Spool Size" => Some(SpoolSize),
                "Yttrium" => Some(Yttrium),
                "Yield" => Some(Yield),
                "Ytterbium" => Some(Ytterbium),
                "Yarn Count" => Some(YarnCount),
                "Yield Point Elongation" => Some(YieldPointElongation),
                "Aluminum" => Some(Aluminum),
                "Arsenic" => Some(Arsenic),
                "Boron" => Some(Boron),
                "Bismuth" => Some(Bismuth),
                "N-Butane" => Some(NButane),
                "Benzene" => Some(Benzene),
                "Carbon" => Some(Carbon),
                "Calcium" => Some(Calcium),
                "Columbium" => Some(Columbium),
                "Carbon Dioxide" => Some(CarbonDioxide),
                "Cerium" => Some(Cerium),
                "Carbon Monoxide" => Some(CarbonMonoxide),
                "Cobalt" => Some(Cobalt),
                "Chromium" => Some(Chromium),
                "Copper" => Some(Copper),
                "Load Factor" => Some(LoadFactor),
                "Ethane" => Some(Ethane),
                "Sulfate Sulfur" => Some(SulfateSulfur),
                "Iron" => Some(Iron),
                "Newspaper--Full Page" => Some(NewspaperFullPage),
                "Iron plus Silicon" => Some(IronPlusSilicon),
                "Organic Sulfur" => Some(OrganicSulfur),
                "Germanium" => Some(Germanium),
                "Hydrogen" => Some(Hydrogen),
                "Heptane" => Some(Heptane),
                "Hydrogen Sulfide" => Some(HydrogenSulfide),
                "Hexane" => Some(Hexane),
                "I-Butane" => Some(IButane),
                "I-Pentane" => Some(IPentane),
                "Magnesium" => Some(Magnesium),
                "Manganese" => Some(Manganese),
                "Molybdenum" => Some(Molybdenum),
                "Methane" => Some(Methane),
                "Nitrogen" => Some(Nitrogen),
                "Niobium" => Some(Niobium),
                "Nickel" => Some(Nickel),
                "Neo-Pentane" => Some(NeoPentane),
                "Oxygen" => Some(Oxygen),
                "Octane" => Some(Octane),
                "Phosphorous" => Some(Phosphorous),
                "Lead" => Some(Lead),
                "Propane" => Some(Propane),
                "N-Pentane" => Some(NPentane),
                "Pyritic Sulfur" => Some(PyriticSulfur),
                "Sulfur" => Some(Sulfur),
                "Antimony" => Some(Antimony),
                "Selenium" => Some(Selenium),
                "Silicon" => Some(Silicon),
                "Tin" => Some(Tin),
                "Tantalum" => Some(Tantalum),
                "Newspaper--Tabloid Page" => Some(NewspaperTabloidPage),
                "Tellurium" => Some(Tellurium),
                "Titanium" => Some(Titanium),
                "Vanadium" => Some(Vanadium),
                "Tungsten" => Some(Tungsten),
                "Zinc" => Some(Zinc),
                "Zirconium" => Some(Zirconium),
                "Mutually Defined" => Some(MutuallyDefined),
                _ => None,
            }
        }
    }
}
impl Serialize for MeasurementQualifier {
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
    type Value = MeasurementQualifier;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Measurement Qualifier")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementQualifier::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Measurement Qualifier: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MeasurementQualifier::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Measurement Qualifier: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for MeasurementQualifier {
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