/// Selectable BIM product kind.
///
/// This enum mirrors practical IFC4x3 product occurrence entities,
/// Notes:
/// - This is not every IFC entity.
/// - This includes physical elements, spatial objects, positioning objects,
///   ports, and structural-analysis products.
/// - Prefer specific variants over generic fallback variants when authoring.
/// - Generic/deprecated variants are kept mainly for import compatibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElementKind {
    // Annotation
    Annotation, // IfcAnnotation

    // Built elements
    BuiltElement, // IfcBuiltElement; generic built-element fallback
    Beam,
    Bearing,
    BuildingElementProxy, // Legacy catch-all; avoid for new authoring
    Chimney,
    Column,
    Course,
    Covering,
    CurtainWall,
    DeepFoundation,
    CaissonFoundation,
    Pile,
    Door,
    EarthworksElement,
    EarthworksFill,
    ReinforcedSoil,
    Footing,
    Kerb,
    Member,
    MooringDevice,
    NavigationElement,
    Pavement,
    Plate,
    Rail,
    Railing,
    Ramp,
    RampFlight,
    Roof,
    ShadingDevice,
    Slab,
    Stair,
    StairFlight,
    TrackElement,
    Wall,
    WallStandardCase,
    Window,

    // Civil fallback
    CivilElement, // Deprecated IFC fallback; keep for import only

    // Distribution and MEP base kinds
    DistributionElement, // Generic/deprecated authoring target
    DistributionControlElement,
    DistributionFlowElement,

    // MEP control elements
    Actuator,
    Alarm,
    Controller,
    FlowInstrument,
    ProtectiveDeviceTrippingUnit,
    Sensor,
    UnitaryControlElement,

    // MEP distribution chambers
    DistributionChamberElement,

    // MEP energy conversion devices
    EnergyConversionDevice,
    AirToAirHeatRecovery,
    Boiler,
    Burner,
    Chiller,
    Coil,
    Condenser,
    CooledBeam,
    CoolingTower,
    ElectricGenerator,
    ElectricMotor,
    Engine,
    EvaporativeCooler,
    Evaporator,
    HeatExchanger,
    Humidifier,
    MotorConnection,
    SolarDevice,
    Transformer,
    TubeBundle,
    UnitaryEquipment,

    // MEP flow controllers
    FlowController, // Generic/deprecated authoring target
    AirTerminalBox,
    Damper,
    DistributionBoard,
    ElectricDistributionBoard,
    ElectricTimeControl,
    FlowMeter,
    ProtectiveDevice,
    SwitchingDevice,
    Valve,

    // MEP flow fittings
    FlowFitting,
    CableCarrierFitting,
    CableFitting,
    DuctFitting,
    JunctionBox,
    PipeFitting,

    // MEP moving devices
    FlowMovingDevice,
    Compressor,
    Fan,
    Pump,

    // MEP flow segments
    FlowSegment,
    CableCarrierSegment,
    CableSegment,
    ConveyorSegment,
    DuctSegment,
    PipeSegment,

    // MEP flow storage devices
    FlowStorageDevice,
    ElectricFlowStorageDevice,
    Tank,

    // MEP terminals
    FlowTerminal,
    AirTerminal,
    AudioVisualAppliance,
    CommunicationsAppliance,
    ElectricAppliance,
    FireSuppressionTerminal,
    Lamp,
    LightFixture,
    LiquidTerminal,
    MedicalDevice,
    MobileTelecommunicationsAppliance,
    Outlet,
    SanitaryTerminal,
    Signal,
    SpaceHeater,
    StackTerminal,
    WasteTerminal,

    // MEP treatment devices
    FlowTreatmentDevice,
    DuctSilencer,
    ElectricFlowTreatmentDevice,
    Filter,
    Interceptor,

    // Assemblies and components
    ElementAssembly,
    ElementComponent,
    BuildingElementPart,
    DiscreteAccessory,
    Fastener,
    ImpactProtectionDevice,
    MechanicalFastener,

    // Reinforcement and tensioning
    ReinforcingBar,
    ReinforcingMesh,
    Tendon,
    TendonAnchor,
    TendonConduit,

    // Signs and vibration components
    Sign,
    VibrationDamper,
    VibrationIsolator,
 
    // Feature elements
    ProjectionElement,
    EarthworksCut,
    OpeningElement,
    VoidingFeature,
    SurfaceFeature,

    // Furnishing
    FurnishingElement,
    Furniture,
    SystemFurnitureElement,

    // Geographic and geotechnical products
    GeographicElement,
    GeotechnicalElement,
    GeotechnicalAssembly,
    Borehole,
    Geomodel,
    Geoslice,
    GeotechnicalStratum,

    // Transportation
    TransportationDevice,
    TransportElement,
    Vehicle,

    // Virtual products
    VirtualElement,

    // Linear infrastructure and positioning
    LinearElement,
    AlignmentCant,
    AlignmentHorizontal,
    AlignmentSegment,
    AlignmentVertical,
    Port,
    DistributionPort,
    PositioningElement,
    Grid,
    LinearPositioningElement,
    Alignment,
    Referent,

    // Spatial products
    ExternalSpatialStructureElement,
    ExternalSpatialElement,
    SpatialStructureElement,
    BuildingStorey,
    Facility,
    Bridge,
    Building,
    MarineFacility,
    Railway,
    Road,
    FacilityPart,
    BridgePart,
    FacilityPartCommon,
    MarinePart,
    RailwayPart,
    RoadPart,
    Site,
    Space,
    SpatialZone,

    // Structural analysis products
    StructuralAction,
    StructuralCurveAction,
    StructuralLinearAction,
    StructuralPointAction,
    StructuralSurfaceAction,
    StructuralPlanarAction,
    StructuralReaction,
    StructuralCurveReaction,
    StructuralPointReaction,
    StructuralSurfaceReaction,
    StructuralCurveConnection,
    StructuralPointConnection,
    StructuralSurfaceConnection,
    StructuralCurveMember,
    StructuralCurveMemberVarying,
    StructuralSurfaceMember,
    StructuralSurfaceMemberVarying,
}