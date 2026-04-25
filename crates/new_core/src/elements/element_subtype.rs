// Generated from buildingSMART IFC4X3_ADD2.exp.
// Source: every EXPRESS entity attribute named `PredefinedType`.
// Schema scope: IFC4x3 ADD2 / IFC 4.3.2 semantic schema.
//
// Uses exact IFC EXPRESS enumerator names as Rust variants.
// That is intentional: it avoids case-conversion errors and preserves schema literals.
//
// Rule: USERDEFINED requires the related ObjectType/ElementType string to carry the custom label.
// Rule: NOTDEFINED means the subtype is unknown or inherited from the assigned IfcTypeObject.

use serde::{Deserialize, Serialize};

/// IFC `IfcActionRequestTypeEnum` used by `IfcActionRequest.PredefinedType`.
///
/// Monolith target: `ActionRequest`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionRequestSubType {
    EMAIL,
    FAX,
    PHONE,
    POST,
    VERBAL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcActuatorTypeEnum` used by `IfcActuator.PredefinedType`.
///
/// Monolith target: `Actuator`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActuatorSubType {
    ELECTRICACTUATOR,
    HANDOPERATEDACTUATOR,
    HYDRAULICACTUATOR,
    PNEUMATICACTUATOR,
    THERMOSTATICACTUATOR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcAirTerminalBoxTypeEnum` used by `IfcAirTerminalBox.PredefinedType`.
///
/// Monolith target: `AirTerminalBox`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AirTerminalBoxSubType {
    CONSTANTFLOW,
    VARIABLEFLOWPRESSUREDEPENDANT,
    VARIABLEFLOWPRESSUREINDEPENDANT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcAirTerminalTypeEnum` used by `IfcAirTerminal.PredefinedType`.
///
/// Monolith target: `AirTerminal`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AirTerminalSubType {
    DIFFUSER,
    GRILLE,
    LOUVRE,
    REGISTER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcAirToAirHeatRecoveryTypeEnum` used by `IfcAirToAirHeatRecovery.PredefinedType`.
///
/// Monolith target: `AirToAirHeatRecovery`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AirToAirHeatRecoverySubType {
    FIXEDPLATECOUNTERFLOWEXCHANGER,
    FIXEDPLATECROSSFLOWEXCHANGER,
    FIXEDPLATEPARALLELFLOWEXCHANGER,
    HEATPIPE,
    ROTARYWHEEL,
    RUNAROUNDCOILLOOP,
    THERMOSIPHONCOILTYPEHEATEXCHANGERS,
    THERMOSIPHONSEALEDTUBEHEATEXCHANGERS,
    TWINTOWERENTHALPYRECOVERYLOOPS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcAlarmTypeEnum` used by `IfcAlarm.PredefinedType`.
///
/// Monolith target: `Alarm`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlarmSubType {
    BELL,
    BREAKGLASSBUTTON,
    LIGHT,
    MANUALPULLBOX,
    RAILWAYCROCODILE,
    RAILWAYDETONATOR,
    SIREN,
    WHISTLE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcAlignmentCantSegmentTypeEnum` used by `IfcAlignmentCantSegment.PredefinedType`.
///
/// Monolith target: `AlignmentCantSegment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignmentCantSegmentSubType {
    BLOSSCURVE,
    CONSTANTCANT,
    COSINECURVE,
    HELMERTCURVE,
    LINEARTRANSITION,
    SINECURVE,
    VIENNESEBEND,
}

/// IFC `IfcAlignmentHorizontalSegmentTypeEnum` used by `IfcAlignmentHorizontalSegment.PredefinedType`.
///
/// Monolith target: `AlignmentHorizontalSegment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignmentHorizontalSegmentSubType {
    BLOSSCURVE,
    CIRCULARARC,
    CLOTHOID,
    COSINECURVE,
    CUBIC,
    HELMERTCURVE,
    LINE,
    SINECURVE,
    VIENNESEBEND,
}

/// IFC `IfcAlignmentTypeEnum` used by `IfcAlignment.PredefinedType`.
///
/// Monolith target: `Alignment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignmentSubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcAlignmentVerticalSegmentTypeEnum` used by `IfcAlignmentVerticalSegment.PredefinedType`.
///
/// Monolith target: `AlignmentVerticalSegment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignmentVerticalSegmentSubType {
    CIRCULARARC,
    CLOTHOID,
    CONSTANTGRADIENT,
    PARABOLICARC,
}

/// IFC `IfcAnalysisModelTypeEnum` used by `IfcStructuralAnalysisModel.PredefinedType`.
///
/// Monolith target: `StructuralAnalysisModel`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisModelSubType {
    IN_PLANE_LOADING_2D,
    LOADING_3D,
    OUT_PLANE_LOADING_2D,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcAnnotationTypeEnum` used by `IfcAnnotation.PredefinedType`.
///
/// Monolith target: `Annotation`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnnotationSubType {
    CONTOURLINE,
    DIMENSION,
    ISOBAR,
    ISOLUX,
    ISOTHERM,
    LEADER,
    SURVEY,
    SYMBOL,
    TEXT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcAudioVisualApplianceTypeEnum` used by `IfcAudioVisualAppliance.PredefinedType`.
///
/// Monolith target: `AudioVisualAppliance`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioVisualApplianceSubType {
    AMPLIFIER,
    CAMERA,
    COMMUNICATIONTERMINAL,
    DISPLAY,
    MICROPHONE,
    PLAYER,
    PROJECTOR,
    RECEIVER,
    RECORDINGEQUIPMENT,
    SPEAKER,
    SWITCHER,
    TELEPHONE,
    TUNER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBeamTypeEnum` used by `IfcBeam.PredefinedType`.
///
/// Monolith target: `Beam`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeamSubType {
    BEAM,
    CORNICE,
    DIAPHRAGM,
    EDGEBEAM,
    GIRDER_SEGMENT,
    HATSTONE,
    HOLLOWCORE,
    JOIST,
    LINTEL,
    PIERCAP,
    SPANDREL,
    T_BEAM,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBearingTypeEnum` used by `IfcBearing.PredefinedType`.
///
/// Monolith target: `Bearing`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BearingSubType {
    CYLINDRICAL,
    DISK,
    ELASTOMERIC,
    GUIDE,
    POT,
    ROCKER,
    ROLLER,
    SPHERICAL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBoilerTypeEnum` used by `IfcBoiler.PredefinedType`.
///
/// Monolith target: `Boiler`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoilerSubType {
    STEAM,
    WATER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBridgePartTypeEnum` used by `IfcBridgePart.PredefinedType`.
///
/// Monolith target: `BridgePart`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BridgePartSubType {
    ABUTMENT,
    DECK,
    DECK_SEGMENT,
    FOUNDATION,
    PIER,
    PIER_SEGMENT,
    PYLON,
    SUBSTRUCTURE,
    SUPERSTRUCTURE,
    SURFACESTRUCTURE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBridgeTypeEnum` used by `IfcBridge.PredefinedType`.
///
/// Monolith target: `Bridge`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BridgeSubType {
    ARCHED,
    CABLE_STAYED,
    CANTILEVER,
    CULVERT,
    FRAMEWORK,
    GIRDER,
    SUSPENSION,
    TRUSS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBuildingElementPartTypeEnum` used by `IfcBuildingElementPart.PredefinedType`.
///
/// Monolith target: `BuildingElementPart`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingElementPartSubType {
    APRON,
    ARMOURUNIT,
    INSULATION,
    PRECASTPANEL,
    SAFETYCAGE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBuildingElementProxyTypeEnum` used by `IfcBuildingElementProxy.PredefinedType`.
///
/// Monolith target: `BuildingElementProxy`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingElementProxySubType {
    COMPLEX,
    ELEMENT,
    PARTIAL,
    PROVISIONFORSPACE,
    PROVISIONFORVOID,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBuildingSystemTypeEnum` used by `IfcBuildingSystem.PredefinedType`.
///
/// Monolith target: `BuildingSystem`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingSystemSubType {
    FENESTRATION,
    FOUNDATION,
    LOADBEARING,
    OUTERSHELL,
    SHADING,
    TRANSPORT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBuiltSystemTypeEnum` used by `IfcBuiltSystem.PredefinedType`.
///
/// Monolith target: `BuiltSystem`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuiltSystemSubType {
    EROSIONPREVENTION,
    FENESTRATION,
    FOUNDATION,
    LOADBEARING,
    MOORING,
    OUTERSHELL,
    PRESTRESSING,
    RAILWAYLINE,
    RAILWAYTRACK,
    REINFORCING,
    SHADING,
    TRACKCIRCUIT,
    TRANSPORT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcBurnerTypeEnum` used by `IfcBurner.PredefinedType`.
///
/// Monolith target: `Burner`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BurnerSubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCableCarrierFittingTypeEnum` used by `IfcCableCarrierFitting.PredefinedType`.
///
/// Monolith target: `CableCarrierFitting`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CableCarrierFittingSubType {
    BEND,
    CONNECTOR,
    CROSS,
    JUNCTION,
    REDUCER,
    TEE,
    TRANSITION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCableCarrierSegmentTypeEnum` used by `IfcCableCarrierSegment.PredefinedType`.
///
/// Monolith target: `CableCarrierSegment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CableCarrierSegmentSubType {
    CABLEBRACKET,
    CABLELADDERSEGMENT,
    CABLETRAYSEGMENT,
    CABLETRUNKINGSEGMENT,
    CATENARYWIRE,
    CONDUITSEGMENT,
    DROPPER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCableFittingTypeEnum` used by `IfcCableFitting.PredefinedType`.
///
/// Monolith target: `CableFitting`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CableFittingSubType {
    CONNECTOR,
    ENTRY,
    EXIT,
    FANOUT,
    JUNCTION,
    TRANSITION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCableSegmentTypeEnum` used by `IfcCableSegment.PredefinedType`.
///
/// Monolith target: `CableSegment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CableSegmentSubType {
    BUSBARSEGMENT,
    CABLESEGMENT,
    CONDUCTORSEGMENT,
    CONTACTWIRESEGMENT,
    CORESEGMENT,
    FIBERSEGMENT,
    FIBERTUBE,
    OPTICALCABLESEGMENT,
    STITCHWIRE,
    WIREPAIRSEGMENT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCaissonFoundationTypeEnum` used by `IfcCaissonFoundation.PredefinedType`.
///
/// Monolith target: `CaissonFoundation`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaissonFoundationSubType {
    CAISSON,
    WELL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcChillerTypeEnum` used by `IfcChiller.PredefinedType`.
///
/// Monolith target: `Chiller`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChillerSubType {
    AIRCOOLED,
    HEATRECOVERY,
    WATERCOOLED,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcChimneyTypeEnum` used by `IfcChimney.PredefinedType`.
///
/// Monolith target: `Chimney`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChimneySubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCoilTypeEnum` used by `IfcCoil.PredefinedType`.
///
/// Monolith target: `Coil`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoilSubType {
    DXCOOLINGCOIL,
    ELECTRICHEATINGCOIL,
    GASHEATINGCOIL,
    HYDRONICCOIL,
    STEAMHEATINGCOIL,
    WATERCOOLINGCOIL,
    WATERHEATINGCOIL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcColumnTypeEnum` used by `IfcColumn.PredefinedType`.
///
/// Monolith target: `Column`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColumnSubType {
    COLUMN,
    PIERSTEM,
    PIERSTEM_SEGMENT,
    PILASTER,
    STANDCOLUMN,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCommunicationsApplianceTypeEnum` used by `IfcCommunicationsAppliance.PredefinedType`.
///
/// Monolith target: `CommunicationsAppliance`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommunicationsApplianceSubType {
    ANTENNA,
    AUTOMATON,
    COMPUTER,
    FAX,
    GATEWAY,
    INTELLIGENTPERIPHERAL,
    IPNETWORKEQUIPMENT,
    LINESIDEELECTRONICUNIT,
    MODEM,
    NETWORKAPPLIANCE,
    NETWORKBRIDGE,
    NETWORKHUB,
    OPTICALLINETERMINAL,
    OPTICALNETWORKUNIT,
    PRINTER,
    RADIOBLOCKCENTER,
    REPEATER,
    ROUTER,
    SCANNER,
    TELECOMMAND,
    TELEPHONYEXCHANGE,
    TRANSITIONCOMPONENT,
    TRANSPONDER,
    TRANSPORTEQUIPMENT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCompressorTypeEnum` used by `IfcCompressor.PredefinedType`.
///
/// Monolith target: `Compressor`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompressorSubType {
    BOOSTER,
    DYNAMIC,
    HERMETIC,
    OPENTYPE,
    RECIPROCATING,
    ROLLINGPISTON,
    ROTARY,
    ROTARYVANE,
    SCROLL,
    SEMIHERMETIC,
    SINGLESCREW,
    SINGLESTAGE,
    TROCHOIDAL,
    TWINSCREW,
    WELDEDSHELLHERMETIC,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCondenserTypeEnum` used by `IfcCondenser.PredefinedType`.
///
/// Monolith target: `Condenser`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CondenserSubType {
    AIRCOOLED,
    EVAPORATIVECOOLED,
    WATERCOOLED,
    WATERCOOLEDBRAZEDPLATE,
    WATERCOOLEDSHELLCOIL,
    WATERCOOLEDSHELLTUBE,
    WATERCOOLEDTUBEINTUBE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcConstructionEquipmentResourceTypeEnum` used by `IfcConstructionEquipmentResource.PredefinedType`.
///
/// Monolith target: `ConstructionEquipmentResource`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructionEquipmentResourceSubType {
    DEMOLISHING,
    EARTHMOVING,
    ERECTING,
    HEATING,
    LIGHTING,
    PAVING,
    PUMPING,
    TRANSPORTING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcConstructionMaterialResourceTypeEnum` used by `IfcConstructionMaterialResource.PredefinedType`.
///
/// Monolith target: `ConstructionMaterialResource`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructionMaterialResourceSubType {
    AGGREGATES,
    CONCRETE,
    DRYWALL,
    FUEL,
    GYPSUM,
    MASONRY,
    METAL,
    PLASTIC,
    WOOD,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcConstructionProductResourceTypeEnum` used by `IfcConstructionProductResource.PredefinedType`.
///
/// Monolith target: `ConstructionProductResource`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructionProductResourceSubType {
    ASSEMBLY,
    FORMWORK,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcControllerTypeEnum` used by `IfcController.PredefinedType`.
///
/// Monolith target: `Controller`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControllerSubType {
    FLOATING,
    MULTIPOSITION,
    PROGRAMMABLE,
    PROPORTIONAL,
    TWOPOSITION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcConveyorSegmentTypeEnum` used by `IfcConveyorSegment.PredefinedType`.
///
/// Monolith target: `ConveyorSegment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConveyorSegmentSubType {
    BELTCONVEYOR,
    BUCKETCONVEYOR,
    CHUTECONVEYOR,
    SCREWCONVEYOR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCooledBeamTypeEnum` used by `IfcCooledBeam.PredefinedType`.
///
/// Monolith target: `CooledBeam`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CooledBeamSubType {
    ACTIVE,
    PASSIVE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCoolingTowerTypeEnum` used by `IfcCoolingTower.PredefinedType`.
///
/// Monolith target: `CoolingTower`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoolingTowerSubType {
    MECHANICALFORCEDDRAFT,
    MECHANICALINDUCEDDRAFT,
    NATURALDRAFT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCostItemTypeEnum` used by `IfcCostItem.PredefinedType`.
///
/// Monolith target: `CostItem`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CostItemSubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCostScheduleTypeEnum` used by `IfcCostSchedule.PredefinedType`.
///
/// Monolith target: `CostSchedule`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CostScheduleSubType {
    BUDGET,
    COSTPLAN,
    ESTIMATE,
    PRICEDBILLOFQUANTITIES,
    SCHEDULEOFRATES,
    TENDER,
    UNPRICEDBILLOFQUANTITIES,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCourseTypeEnum` used by `IfcCourse.PredefinedType`.
///
/// Monolith target: `Course`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CourseSubType {
    ARMOUR,
    BALLASTBED,
    CORE,
    FILTER,
    PAVEMENT,
    PROTECTION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCoveringTypeEnum` used by `IfcCovering.PredefinedType`.
///
/// Monolith target: `Covering`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoveringSubType {
    CEILING,
    CLADDING,
    COPING,
    FLOORING,
    INSULATION,
    MEMBRANE,
    MOLDING,
    ROOFING,
    SKIRTINGBOARD,
    SLEEVING,
    TOPPING,
    WRAPPING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCrewResourceTypeEnum` used by `IfcCrewResource.PredefinedType`.
///
/// Monolith target: `CrewResource`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrewResourceSubType {
    OFFICE,
    SITE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcCurtainWallTypeEnum` used by `IfcCurtainWall.PredefinedType`.
///
/// Monolith target: `CurtainWall`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurtainWallSubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDamperTypeEnum` used by `IfcDamper.PredefinedType`.
///
/// Monolith target: `Damper`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamperSubType {
    BACKDRAFTDAMPER,
    BALANCINGDAMPER,
    BLASTDAMPER,
    CONTROLDAMPER,
    FIREDAMPER,
    FIRESMOKEDAMPER,
    FUMEHOODEXHAUST,
    GRAVITYDAMPER,
    GRAVITYRELIEFDAMPER,
    RELIEFDAMPER,
    SMOKEDAMPER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDiscreteAccessoryTypeEnum` used by `IfcDiscreteAccessory.PredefinedType`.
///
/// Monolith target: `DiscreteAccessory`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiscreteAccessorySubType {
    ANCHORPLATE,
    BIRDPROTECTION,
    BRACKET,
    CABLEARRANGER,
    ELASTIC_CUSHION,
    EXPANSION_JOINT_DEVICE,
    FILLER,
    FLASHING,
    INSULATOR,
    LOCK,
    PANEL_STRENGTHENING,
    POINTMACHINEMOUNTINGDEVICE,
    POINT_MACHINE_LOCKING_DEVICE,
    RAILBRACE,
    RAILPAD,
    RAIL_LUBRICATION,
    RAIL_MECHANICAL_EQUIPMENT,
    SHOE,
    SLIDINGCHAIR,
    SOUNDABSORPTION,
    TENSIONINGEQUIPMENT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDistributionBoardTypeEnum` used by `IfcDistributionBoard.PredefinedType`.
///
/// Monolith target: `DistributionBoard`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DistributionBoardSubType {
    CONSUMERUNIT,
    DISPATCHINGBOARD,
    DISTRIBUTIONBOARD,
    DISTRIBUTIONFRAME,
    MOTORCONTROLCENTRE,
    SWITCHBOARD,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDistributionChamberElementTypeEnum` used by `IfcDistributionChamberElement.PredefinedType`.
///
/// Monolith target: `DistributionChamberElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DistributionChamberElementSubType {
    FORMEDDUCT,
    INSPECTIONCHAMBER,
    INSPECTIONPIT,
    MANHOLE,
    METERCHAMBER,
    SUMP,
    TRENCH,
    VALVECHAMBER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDistributionPortTypeEnum` used by `IfcDistributionPort.PredefinedType`.
///
/// Monolith target: `DistributionPort`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DistributionPortSubType {
    CABLE,
    CABLECARRIER,
    DUCT,
    PIPE,
    WIRELESS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDistributionSystemEnum` used by `IfcDistributionSystem.PredefinedType`.
///
/// Monolith target: `DistributionSystem`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DistributionSystemEnum {
    AIRCONDITIONING,
    AUDIOVISUAL,
    CATENARY_SYSTEM,
    CHEMICAL,
    CHILLEDWATER,
    COMMUNICATION,
    COMPRESSEDAIR,
    CONDENSERWATER,
    CONTROL,
    CONVEYING,
    DATA,
    DISPOSAL,
    DOMESTICCOLDWATER,
    DOMESTICHOTWATER,
    DRAINAGE,
    EARTHING,
    ELECTRICAL,
    ELECTROACOUSTIC,
    EXHAUST,
    FIREPROTECTION,
    FIXEDTRANSMISSIONNETWORK,
    FUEL,
    GAS,
    HAZARDOUS,
    HEATING,
    LIGHTING,
    LIGHTNINGPROTECTION,
    MOBILENETWORK,
    MONITORINGSYSTEM,
    MUNICIPALSOLIDWASTE,
    OIL,
    OPERATIONAL,
    OPERATIONALTELEPHONYSYSTEM,
    OVERHEAD_CONTACTLINE_SYSTEM,
    POWERGENERATION,
    RAINWATER,
    REFRIGERATION,
    RETURN_CIRCUIT,
    SECURITY,
    SEWAGE,
    SIGNAL,
    STORMWATER,
    TELEPHONE,
    TV,
    VACUUM,
    VENT,
    VENTILATION,
    WASTEWATER,
    WATERSUPPLY,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDoorTypeEnum` used by `IfcDoor.PredefinedType`.
///
/// Monolith target: `Door`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoorSubType {
    BOOM_BARRIER,
    DOOR,
    GATE,
    TRAPDOOR,
    TURNSTILE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDuctFittingTypeEnum` used by `IfcDuctFitting.PredefinedType`.
///
/// Monolith target: `DuctFitting`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DuctFittingSubType {
    BEND,
    CONNECTOR,
    ENTRY,
    EXIT,
    JUNCTION,
    OBSTRUCTION,
    TRANSITION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDuctSegmentTypeEnum` used by `IfcDuctSegment.PredefinedType`.
///
/// Monolith target: `DuctSegment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DuctSegmentSubType {
    FLEXIBLESEGMENT,
    RIGIDSEGMENT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcDuctSilencerTypeEnum` used by `IfcDuctSilencer.PredefinedType`.
///
/// Monolith target: `DuctSilencer`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DuctSilencerSubType {
    FLATOVAL,
    RECTANGULAR,
    ROUND,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcEarthworksCutTypeEnum` used by `IfcEarthworksCut.PredefinedType`.
///
/// Monolith target: `EarthworksCut`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EarthworksCutSubType {
    BASE_EXCAVATION,
    CUT,
    DREDGING,
    EXCAVATION,
    OVEREXCAVATION,
    PAVEMENTMILLING,
    STEPEXCAVATION,
    TOPSOILREMOVAL,
    TRENCH,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcEarthworksFillTypeEnum` used by `IfcEarthworksFill.PredefinedType`.
///
/// Monolith target: `EarthworksFill`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EarthworksFillSubType {
    BACKFILL,
    COUNTERWEIGHT,
    EMBANKMENT,
    SLOPEFILL,
    SUBGRADE,
    SUBGRADEBED,
    TRANSITIONSECTION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcElectricApplianceTypeEnum` used by `IfcElectricAppliance.PredefinedType`.
///
/// Monolith target: `ElectricAppliance`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricApplianceSubType {
    DISHWASHER,
    ELECTRICCOOKER,
    FREESTANDINGELECTRICHEATER,
    FREESTANDINGFAN,
    FREESTANDINGWATERCOOLER,
    FREESTANDINGWATERHEATER,
    FREEZER,
    FRIDGE_FREEZER,
    HANDDRYER,
    KITCHENMACHINE,
    MICROWAVE,
    PHOTOCOPIER,
    REFRIGERATOR,
    TUMBLEDRYER,
    VENDINGMACHINE,
    WASHINGMACHINE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcElectricDistributionBoardTypeEnum` used by `IfcElectricDistributionBoard.PredefinedType`.
///
/// Monolith target: `ElectricDistributionBoard`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricDistributionBoardSubType {
    CONSUMERUNIT,
    DISTRIBUTIONBOARD,
    MOTORCONTROLCENTRE,
    SWITCHBOARD,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcElectricFlowStorageDeviceTypeEnum` used by `IfcElectricFlowStorageDevice.PredefinedType`.
///
/// Monolith target: `ElectricFlowStorageDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricFlowStorageDeviceSubType {
    BATTERY,
    CAPACITOR,
    CAPACITORBANK,
    COMPENSATOR,
    HARMONICFILTER,
    INDUCTOR,
    INDUCTORBANK,
    RECHARGER,
    UPS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcElectricFlowTreatmentDeviceTypeEnum` used by `IfcElectricFlowTreatmentDevice.PredefinedType`.
///
/// Monolith target: `ElectricFlowTreatmentDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricFlowTreatmentDeviceSubType {
    ELECTRONICFILTER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcElectricGeneratorTypeEnum` used by `IfcElectricGenerator.PredefinedType`.
///
/// Monolith target: `ElectricGenerator`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricGeneratorSubType {
    CHP,
    ENGINEGENERATOR,
    STANDALONE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcElectricMotorTypeEnum` used by `IfcElectricMotor.PredefinedType`.
///
/// Monolith target: `ElectricMotor`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricMotorSubType {
    DC,
    INDUCTION,
    POLYPHASE,
    RELUCTANCESYNCHRONOUS,
    SYNCHRONOUS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcElectricTimeControlTypeEnum` used by `IfcElectricTimeControl.PredefinedType`.
///
/// Monolith target: `ElectricTimeControl`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricTimeControlSubType {
    RELAY,
    TIMECLOCK,
    TIMEDELAY,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcElementAssemblyTypeEnum` used by `IfcElementAssembly.PredefinedType`.
///
/// Monolith target: `ElementAssembly`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementAssemblySubType {
    ABUTMENT,
    ACCESSORY_ASSEMBLY,
    ARCH,
    BEAM_GRID,
    BRACED_FRAME,
    CROSS_BRACING,
    DECK,
    DILATATIONPANEL,
    ENTRANCEWORKS,
    GIRDER,
    GRID,
    MAST,
    PIER,
    PYLON,
    RAIL_MECHANICAL_EQUIPMENT_ASSEMBLY,
    REINFORCEMENT_UNIT,
    RIGID_FRAME,
    SHELTER,
    SIGNALASSEMBLY,
    SLAB_FIELD,
    SUMPBUSTER,
    SUPPORTINGASSEMBLY,
    SUSPENSIONASSEMBLY,
    TRACKPANEL,
    TRACTION_SWITCHING_ASSEMBLY,
    TRAFFIC_CALMING_DEVICE,
    TRUSS,
    TURNOUTPANEL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcEngineTypeEnum` used by `IfcEngine.PredefinedType`.
///
/// Monolith target: `Engine`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EngineSubType {
    EXTERNALCOMBUSTION,
    INTERNALCOMBUSTION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcEvaporativeCoolerTypeEnum` used by `IfcEvaporativeCooler.PredefinedType`.
///
/// Monolith target: `EvaporativeCooler`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvaporativeCoolerSubType {
    DIRECTEVAPORATIVEAIRWASHER,
    DIRECTEVAPORATIVEPACKAGEDROTARYAIRCOOLER,
    DIRECTEVAPORATIVERANDOMMEDIAAIRCOOLER,
    DIRECTEVAPORATIVERIGIDMEDIAAIRCOOLER,
    DIRECTEVAPORATIVESLINGERSPACKAGEDAIRCOOLER,
    INDIRECTDIRECTCOMBINATION,
    INDIRECTEVAPORATIVECOOLINGTOWERORCOILCOOLER,
    INDIRECTEVAPORATIVEPACKAGEAIRCOOLER,
    INDIRECTEVAPORATIVEWETCOIL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcEvaporatorTypeEnum` used by `IfcEvaporator.PredefinedType`.
///
/// Monolith target: `Evaporator`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvaporatorSubType {
    DIRECTEXPANSION,
    DIRECTEXPANSIONBRAZEDPLATE,
    DIRECTEXPANSIONSHELLANDTUBE,
    DIRECTEXPANSIONTUBEINTUBE,
    FLOODEDSHELLANDTUBE,
    SHELLANDCOIL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcEventTypeEnum` used by `IfcEvent.PredefinedType`.
///
/// Monolith target: `Event`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventSubType {
    ENDEVENT,
    INTERMEDIATEEVENT,
    STARTEVENT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcExternalSpatialElementTypeEnum` used by `IfcExternalSpatialElement.PredefinedType`.
///
/// Monolith target: `ExternalSpatialElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExternalSpatialElementSubType {
    EXTERNAL,
    EXTERNAL_EARTH,
    EXTERNAL_FIRE,
    EXTERNAL_WATER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFacilityPartCommonTypeEnum` used by `IfcFacilityPartCommon.PredefinedType`.
///
/// Monolith target: `FacilityPartCommon`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacilityPartCommonSubType {
    ABOVEGROUND,
    BELOWGROUND,
    JUNCTION,
    LEVELCROSSING,
    SEGMENT,
    SUBSTRUCTURE,
    SUPERSTRUCTURE,
    TERMINAL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFanTypeEnum` used by `IfcFan.PredefinedType`.
///
/// Monolith target: `Fan`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FanSubType {
    CENTRIFUGALAIRFOIL,
    CENTRIFUGALBACKWARDINCLINEDCURVED,
    CENTRIFUGALFORWARDCURVED,
    CENTRIFUGALRADIAL,
    PROPELLORAXIAL,
    TUBEAXIAL,
    VANEAXIAL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFastenerTypeEnum` used by `IfcFastener.PredefinedType`.
///
/// Monolith target: `Fastener`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FastenerSubType {
    GLUE,
    MORTAR,
    WELD,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFilterTypeEnum` used by `IfcFilter.PredefinedType`.
///
/// Monolith target: `Filter`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterSubType {
    AIRPARTICLEFILTER,
    COMPRESSEDAIRFILTER,
    ODORFILTER,
    OILFILTER,
    STRAINER,
    WATERFILTER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFireSuppressionTerminalTypeEnum` used by `IfcFireSuppressionTerminal.PredefinedType`.
///
/// Monolith target: `FireSuppressionTerminal`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FireSuppressionTerminalSubType {
    BREECHINGINLET,
    FIREHYDRANT,
    FIREMONITOR,
    HOSEREEL,
    SPRINKLER,
    SPRINKLERDEFLECTOR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFlowInstrumentTypeEnum` used by `IfcFlowInstrument.PredefinedType`.
///
/// Monolith target: `FlowInstrument`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlowInstrumentSubType {
    AMMETER,
    COMBINED,
    FREQUENCYMETER,
    PHASEANGLEMETER,
    POWERFACTORMETER,
    PRESSUREGAUGE,
    THERMOMETER,
    VOLTMETER,
    VOLTMETER_PEAK,
    VOLTMETER_RMS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFlowMeterTypeEnum` used by `IfcFlowMeter.PredefinedType`.
///
/// Monolith target: `FlowMeter`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlowMeterSubType {
    ENERGYMETER,
    GASMETER,
    OILMETER,
    WATERMETER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFootingTypeEnum` used by `IfcFooting.PredefinedType`.
///
/// Monolith target: `Footing`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FootingSubType {
    CAISSON_FOUNDATION,
    FOOTING_BEAM,
    PAD_FOOTING,
    PILE_CAP,
    STRIP_FOOTING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcFurnitureTypeEnum` used by `IfcFurniture.PredefinedType`.
///
/// Monolith target: `Furniture`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FurnitureSubType {
    BED,
    CHAIR,
    DESK,
    FILECABINET,
    SHELF,
    SOFA,
    TABLE,
    TECHNICALCABINET,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcGeographicElementTypeEnum` used by `IfcGeographicElement.PredefinedType`.
///
/// Monolith target: `GeographicElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicElementSubType {
    SOIL_BORING_POINT,
    TERRAIN,
    VEGETATION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcGeotechnicalStratumTypeEnum` used by `IfcGeotechnicalStratum.PredefinedType`.
///
/// Monolith target: `GeotechnicalStratum`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeotechnicalStratumSubType {
    SOLID,
    VOID,
    WATER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcGridTypeEnum` used by `IfcGrid.PredefinedType`.
///
/// Monolith target: `Grid`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridSubType {
    IRREGULAR,
    RADIAL,
    RECTANGULAR,
    TRIANGULAR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcHeatExchangerTypeEnum` used by `IfcHeatExchanger.PredefinedType`.
///
/// Monolith target: `HeatExchanger`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeatExchangerSubType {
    PLATE,
    SHELLANDTUBE,
    TURNOUTHEATING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcHumidifierTypeEnum` used by `IfcHumidifier.PredefinedType`.
///
/// Monolith target: `Humidifier`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HumidifierSubType {
    ADIABATICAIRWASHER,
    ADIABATICATOMIZING,
    ADIABATICCOMPRESSEDAIRNOZZLE,
    ADIABATICPAN,
    ADIABATICRIGIDMEDIA,
    ADIABATICULTRASONIC,
    ADIABATICWETTEDELEMENT,
    ASSISTEDBUTANE,
    ASSISTEDELECTRIC,
    ASSISTEDNATURALGAS,
    ASSISTEDPROPANE,
    ASSISTEDSTEAM,
    STEAMINJECTION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcImpactProtectionDeviceTypeEnum` used by `IfcImpactProtectionDevice.PredefinedType`.
///
/// Monolith target: `ImpactProtectionDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImpactProtectionDeviceSubType {
    BUMPER,
    CRASHCUSHION,
    DAMPINGSYSTEM,
    FENDER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcInterceptorTypeEnum` used by `IfcInterceptor.PredefinedType`.
///
/// Monolith target: `Interceptor`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterceptorSubType {
    CYCLONIC,
    GREASE,
    OIL,
    PETROL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcInventoryTypeEnum` used by `IfcInventory.PredefinedType`.
///
/// Monolith target: `Inventory`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InventorySubType {
    ASSETINVENTORY,
    FURNITUREINVENTORY,
    SPACEINVENTORY,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcJunctionBoxTypeEnum` used by `IfcJunctionBox.PredefinedType`.
///
/// Monolith target: `JunctionBox`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JunctionBoxSubType {
    DATA,
    POWER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcKerbTypeEnum` used by `IfcKerb.PredefinedType`.
///
/// Monolith target: `Kerb`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KerbSubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcLaborResourceTypeEnum` used by `IfcLaborResource.PredefinedType`.
///
/// Monolith target: `LaborResource`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LaborResourceSubType {
    ADMINISTRATION,
    CARPENTRY,
    CLEANING,
    CONCRETE,
    DRYWALL,
    ELECTRIC,
    FINISHING,
    FLOORING,
    GENERAL,
    HVAC,
    LANDSCAPING,
    MASONRY,
    PAINTING,
    PAVING,
    PLUMBING,
    ROOFING,
    SITEGRADING,
    STEELWORK,
    SURVEYING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcLampTypeEnum` used by `IfcLamp.PredefinedType`.
///
/// Monolith target: `Lamp`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LampSubType {
    COMPACTFLUORESCENT,
    FLUORESCENT,
    HALOGEN,
    HIGHPRESSUREMERCURY,
    HIGHPRESSURESODIUM,
    LED,
    METALHALIDE,
    OLED,
    TUNGSTENFILAMENT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcLightFixtureTypeEnum` used by `IfcLightFixture.PredefinedType`.
///
/// Monolith target: `LightFixture`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightFixtureSubType {
    DIRECTIONSOURCE,
    POINTSOURCE,
    SECURITYLIGHTING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcLiquidTerminalTypeEnum` used by `IfcLiquidTerminal.PredefinedType`.
///
/// Monolith target: `LiquidTerminal`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiquidTerminalSubType {
    HOSEREEL,
    LOADINGARM,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcLoadGroupTypeEnum` used by `IfcStructuralLoadGroup.PredefinedType`.
///
/// Monolith target: `StructuralLoadGroup`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoadGroupSubType {
    LOAD_CASE,
    LOAD_COMBINATION,
    LOAD_GROUP,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcMarineFacilityTypeEnum` used by `IfcMarineFacility.PredefinedType`.
///
/// Monolith target: `MarineFacility`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarineFacilitySubType {
    BARRIERBEACH,
    BREAKWATER,
    CANAL,
    DRYDOCK,
    FLOATINGDOCK,
    HYDROLIFT,
    JETTY,
    LAUNCHRECOVERY,
    MARINEDEFENCE,
    NAVIGATIONALCHANNEL,
    PORT,
    QUAY,
    REVETMENT,
    SHIPLIFT,
    SHIPLOCK,
    SHIPYARD,
    SLIPWAY,
    WATERWAY,
    WATERWAYSHIPLIFT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcMarinePartTypeEnum` used by `IfcMarinePart.PredefinedType`.
///
/// Monolith target: `MarinePart`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarinePartSubType {
    ABOVEWATERLINE,
    ANCHORAGE,
    APPROACHCHANNEL,
    BELOWWATERLINE,
    BERTHINGSTRUCTURE,
    CHAMBER,
    CILL_LEVEL,
    COPELEVEL,
    CORE,
    CREST,
    GATEHEAD,
    GUDINGSTRUCTURE,
    HIGHWATERLINE,
    LANDFIELD,
    LEEWARDSIDE,
    LOWWATERLINE,
    MANUFACTURING,
    NAVIGATIONALAREA,
    PROTECTION,
    SHIPTRANSFER,
    STORAGEAREA,
    VEHICLESERVICING,
    WATERFIELD,
    WEATHERSIDE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcMechanicalFastenerTypeEnum` used by `IfcMechanicalFastener.PredefinedType`.
///
/// Monolith target: `MechanicalFastener`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MechanicalFastenerSubType {
    ANCHORBOLT,
    BOLT,
    CHAIN,
    COUPLER,
    DOWEL,
    NAIL,
    NAILPLATE,
    RAILFASTENING,
    RAILJOINT,
    RIVET,
    ROPE,
    SCREW,
    SHEARCONNECTOR,
    STAPLE,
    STUDSHEARCONNECTOR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcMedicalDeviceTypeEnum` used by `IfcMedicalDevice.PredefinedType`.
///
/// Monolith target: `MedicalDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MedicalDeviceSubType {
    AIRSTATION,
    FEEDAIRUNIT,
    OXYGENGENERATOR,
    OXYGENPLANT,
    VACUUMSTATION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcMemberTypeEnum` used by `IfcMember.PredefinedType`.
///
/// Monolith target: `Member`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberSubType {
    ARCH_SEGMENT,
    BRACE,
    CHORD,
    COLLAR,
    MEMBER,
    MULLION,
    PLATE,
    POST,
    PURLIN,
    RAFTER,
    STAY_CABLE,
    STIFFENING_RIB,
    STRINGER,
    STRUCTURALCABLE,
    STRUT,
    STUD,
    SUSPENDER,
    SUSPENSION_CABLE,
    TIEBAR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcMobileTelecommunicationsApplianceTypeEnum` used by `IfcMobileTelecommunicationsAppliance.PredefinedType`.
///
/// Monolith target: `MobileTelecommunicationsAppliance`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MobileTelecommunicationsApplianceSubType {
    ACCESSPOINT,
    BASEBANDUNIT,
    BASETRANSCEIVERSTATION,
    E_UTRAN_NODE_B,
    GATEWAY_GPRS_SUPPORT_NODE,
    MASTERUNIT,
    MOBILESWITCHINGCENTER,
    MSCSERVER,
    PACKETCONTROLUNIT,
    REMOTERADIOUNIT,
    REMOTEUNIT,
    SERVICE_GPRS_SUPPORT_NODE,
    SUBSCRIBERSERVER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcMooringDeviceTypeEnum` used by `IfcMooringDevice.PredefinedType`.
///
/// Monolith target: `MooringDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MooringDeviceSubType {
    BOLLARD,
    LINETENSIONER,
    MAGNETICDEVICE,
    MOORINGHOOKS,
    VACUUMDEVICE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcMotorConnectionTypeEnum` used by `IfcMotorConnection.PredefinedType`.
///
/// Monolith target: `MotorConnection`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MotorConnectionSubType {
    BELTDRIVE,
    COUPLING,
    DIRECTDRIVE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcNavigationElementTypeEnum` used by `IfcNavigationElement.PredefinedType`.
///
/// Monolith target: `NavigationElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavigationElementSubType {
    BEACON,
    BUOY,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcOccupantTypeEnum` used by `IfcOccupant.PredefinedType`.
///
/// Monolith target: `Occupant`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OccupantSubType {
    ASSIGNEE,
    ASSIGNOR,
    LESSEE,
    LESSOR,
    LETTINGAGENT,
    OWNER,
    TENANT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcOpeningElementTypeEnum` used by `IfcOpeningElement.PredefinedType`.
///
/// Monolith target: `OpeningElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OpeningElementSubType {
    OPENING,
    RECESS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcOutletTypeEnum` used by `IfcOutlet.PredefinedType`.
///
/// Monolith target: `Outlet`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutletSubType {
    AUDIOVISUALOUTLET,
    COMMUNICATIONSOUTLET,
    DATAOUTLET,
    POWEROUTLET,
    TELEPHONEOUTLET,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcPavementTypeEnum` used by `IfcPavement.PredefinedType`.
///
/// Monolith target: `Pavement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PavementSubType {
    FLEXIBLE,
    RIGID,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcPerformanceHistoryTypeEnum` used by `IfcPerformanceHistory.PredefinedType`.
///
/// Monolith target: `PerformanceHistory`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerformanceHistorySubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcPermitTypeEnum` used by `IfcPermit.PredefinedType`.
///
/// Monolith target: `Permit`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermitSubType {
    ACCESS,
    BUILDING,
    WORK,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcPileTypeEnum` used by `IfcPile.PredefinedType`.
///
/// Monolith target: `Pile`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PileSubType {
    BORED,
    COHESION,
    DRIVEN,
    FRICTION,
    JETGROUTING,
    SUPPORT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcPipeFittingTypeEnum` used by `IfcPipeFitting.PredefinedType`.
///
/// Monolith target: `PipeFitting`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PipeFittingSubType {
    BEND,
    CONNECTOR,
    ENTRY,
    EXIT,
    JUNCTION,
    OBSTRUCTION,
    TRANSITION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcPipeSegmentTypeEnum` used by `IfcPipeSegment.PredefinedType`.
///
/// Monolith target: `PipeSegment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PipeSegmentSubType {
    CULVERT,
    FLEXIBLESEGMENT,
    GUTTER,
    RIGIDSEGMENT,
    SPOOL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcPlateTypeEnum` used by `IfcPlate.PredefinedType`.
///
/// Monolith target: `Plate`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlateSubType {
    BASE_PLATE,
    COVER_PLATE,
    CURTAIN_PANEL,
    FLANGE_PLATE,
    GUSSET_PLATE,
    SHEET,
    SPLICE_PLATE,
    STIFFENER_PLATE,
    WEB_PLATE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcProcedureTypeEnum` used by `IfcProcedure.PredefinedType`.
///
/// Monolith target: `Procedure`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProcedureSubType {
    ADVICE_CAUTION,
    ADVICE_NOTE,
    ADVICE_WARNING,
    CALIBRATION,
    DIAGNOSTIC,
    SHUTDOWN,
    STARTUP,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcProjectOrderTypeEnum` used by `IfcProjectOrder.PredefinedType`.
///
/// Monolith target: `ProjectOrder`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectOrderSubType {
    CHANGEORDER,
    MAINTENANCEWORKORDER,
    MOVEORDER,
    PURCHASEORDER,
    WORKORDER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcProjectionElementTypeEnum` used by `IfcProjectionElement.PredefinedType`.
///
/// Monolith target: `ProjectionElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectionElementSubType {
    BLISTER,
    DEVIATOR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcProtectiveDeviceTrippingUnitTypeEnum` used by `IfcProtectiveDeviceTrippingUnit.PredefinedType`.
///
/// Monolith target: `ProtectiveDeviceTrippingUnit`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtectiveDeviceTrippingUnitSubType {
    ELECTROMAGNETIC,
    ELECTRONIC,
    RESIDUALCURRENT,
    THERMAL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcProtectiveDeviceTypeEnum` used by `IfcProtectiveDevice.PredefinedType`.
///
/// Monolith target: `ProtectiveDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtectiveDeviceSubType {
    ANTI_ARCING_DEVICE,
    CIRCUITBREAKER,
    EARTHINGSWITCH,
    EARTHLEAKAGECIRCUITBREAKER,
    FUSEDISCONNECTOR,
    RESIDUALCURRENTCIRCUITBREAKER,
    RESIDUALCURRENTSWITCH,
    SPARKGAP,
    VARISTOR,
    VOLTAGELIMITER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcPumpTypeEnum` used by `IfcPump.PredefinedType`.
///
/// Monolith target: `Pump`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PumpSubType {
    CIRCULATOR,
    ENDSUCTION,
    SPLITCASE,
    SUBMERSIBLEPUMP,
    SUMPPUMP,
    VERTICALINLINE,
    VERTICALTURBINE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRailTypeEnum` used by `IfcRail.PredefinedType`.
///
/// Monolith target: `Rail`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RailSubType {
    BLADE,
    CHECKRAIL,
    GUARDRAIL,
    RACKRAIL,
    RAIL,
    STOCKRAIL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRailingTypeEnum` used by `IfcRailing.PredefinedType`.
///
/// Monolith target: `Railing`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RailingSubType {
    BALUSTRADE,
    FENCE,
    GUARDRAIL,
    HANDRAIL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRailwayPartTypeEnum` used by `IfcRailwayPart.PredefinedType`.
///
/// Monolith target: `RailwayPart`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RailwayPartSubType {
    ABOVETRACK,
    DILATIONTRACK,
    LINESIDE,
    LINESIDEPART,
    PLAINTRACK,
    SUBSTRUCTURE,
    TRACK,
    TRACKPART,
    TURNOUTTRACK,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRailwayTypeEnum` used by `IfcRailway.PredefinedType`.
///
/// Monolith target: `Railway`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RailwaySubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRampFlightTypeEnum` used by `IfcRampFlight.PredefinedType`.
///
/// Monolith target: `RampFlight`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RampFlightSubType {
    SPIRAL,
    STRAIGHT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRampTypeEnum` used by `IfcRamp.PredefinedType`.
///
/// Monolith target: `Ramp`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RampSubType {
    HALF_TURN_RAMP,
    QUARTER_TURN_RAMP,
    SPIRAL_RAMP,
    STRAIGHT_RUN_RAMP,
    TWO_QUARTER_TURN_RAMP,
    TWO_STRAIGHT_RUN_RAMP,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcReferentTypeEnum` used by `IfcReferent.PredefinedType`.
///
/// Monolith target: `Referent`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReferentSubType {
    BOUNDARY,
    INTERSECTION,
    KILOPOINT,
    LANDMARK,
    MILEPOINT,
    POSITION,
    REFERENCEMARKER,
    STATION,
    SUPERELEVATIONEVENT,
    WIDTHEVENT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcReinforcedSoilTypeEnum` used by `IfcReinforcedSoil.PredefinedType`.
///
/// Monolith target: `ReinforcedSoil`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReinforcedSoilSubType {
    DYNAMICALLYCOMPACTED,
    GROUTED,
    REPLACED,
    ROLLERCOMPACTED,
    SURCHARGEPRELOADED,
    VERTICALLYDRAINED,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcReinforcingBarTypeEnum` used by `IfcReinforcingBar.PredefinedType`.
///
/// Monolith target: `ReinforcingBar`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReinforcingBarSubType {
    ANCHORING,
    EDGE,
    LIGATURE,
    MAIN,
    PUNCHING,
    RING,
    SHEAR,
    SPACEBAR,
    STUD,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcReinforcingMeshTypeEnum` used by `IfcReinforcingMesh.PredefinedType`.
///
/// Monolith target: `ReinforcingMesh`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReinforcingMeshSubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRoadPartTypeEnum` used by `IfcRoadPart.PredefinedType`.
///
/// Monolith target: `RoadPart`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoadPartSubType {
    BICYCLECROSSING,
    BUS_STOP,
    CARRIAGEWAY,
    CENTRALISLAND,
    CENTRALRESERVE,
    HARDSHOULDER,
    INTERSECTION,
    LAYBY,
    PARKINGBAY,
    PASSINGBAY,
    PEDESTRIAN_CROSSING,
    RAILWAYCROSSING,
    REFUGEISLAND,
    ROADSEGMENT,
    ROADSIDE,
    ROADSIDEPART,
    ROADWAYPLATEAU,
    ROUNDABOUT,
    SHOULDER,
    SIDEWALK,
    SOFTSHOULDER,
    TOLLPLAZA,
    TRAFFICISLAND,
    TRAFFICLANE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRoadTypeEnum` used by `IfcRoad.PredefinedType`.
///
/// Monolith target: `Road`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoadSubType {
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcRoofTypeEnum` used by `IfcRoof.PredefinedType`.
///
/// Monolith target: `Roof`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoofSubType {
    BARREL_ROOF,
    BUTTERFLY_ROOF,
    DOME_ROOF,
    FLAT_ROOF,
    FREEFORM,
    GABLE_ROOF,
    GAMBREL_ROOF,
    HIPPED_GABLE_ROOF,
    HIP_ROOF,
    MANSARD_ROOF,
    PAVILION_ROOF,
    RAINBOW_ROOF,
    SHED_ROOF,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSanitaryTerminalTypeEnum` used by `IfcSanitaryTerminal.PredefinedType`.
///
/// Monolith target: `SanitaryTerminal`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SanitaryTerminalSubType {
    BATH,
    BIDET,
    CISTERN,
    SANITARYFOUNTAIN,
    SHOWER,
    SINK,
    TOILETPAN,
    URINAL,
    WASHHANDBASIN,
    WCSEAT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSensorTypeEnum` used by `IfcSensor.PredefinedType`.
///
/// Monolith target: `Sensor`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensorSubType {
    CO2SENSOR,
    CONDUCTANCESENSOR,
    CONTACTSENSOR,
    COSENSOR,
    EARTHQUAKESENSOR,
    FIRESENSOR,
    FLOWSENSOR,
    FOREIGNOBJECTDETECTIONSENSOR,
    FROSTSENSOR,
    GASSENSOR,
    HEATSENSOR,
    HUMIDITYSENSOR,
    IDENTIFIERSENSOR,
    IONCONCENTRATIONSENSOR,
    LEVELSENSOR,
    LIGHTSENSOR,
    MOISTURESENSOR,
    MOVEMENTSENSOR,
    OBSTACLESENSOR,
    PHSENSOR,
    PRESSURESENSOR,
    RADIATIONSENSOR,
    RADIOACTIVITYSENSOR,
    RAINSENSOR,
    SMOKESENSOR,
    SNOWDEPTHSENSOR,
    SOUNDSENSOR,
    TEMPERATURESENSOR,
    TRAINSENSOR,
    TURNOUTCLOSURESENSOR,
    WHEELSENSOR,
    WINDSENSOR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcShadingDeviceTypeEnum` used by `IfcShadingDevice.PredefinedType`.
///
/// Monolith target: `ShadingDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShadingDeviceSubType {
    AWNING,
    JALOUSIE,
    SHUTTER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSignTypeEnum` used by `IfcSign.PredefinedType`.
///
/// Monolith target: `Sign`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignSubType {
    MARKER,
    MIRROR,
    PICTORAL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSignalTypeEnum` used by `IfcSignal.PredefinedType`.
///
/// Monolith target: `Signal`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalSubType {
    AUDIO,
    MIXED,
    VISUAL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSlabTypeEnum` used by `IfcSlab.PredefinedType`.
///
/// Monolith target: `Slab`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlabSubType {
    APPROACH_SLAB,
    BASESLAB,
    FLOOR,
    LANDING,
    PAVING,
    ROOF,
    SIDEWALK,
    TRACKSLAB,
    WEARING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSolarDeviceTypeEnum` used by `IfcSolarDevice.PredefinedType`.
///
/// Monolith target: `SolarDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SolarDeviceSubType {
    SOLARCOLLECTOR,
    SOLARPANEL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSpaceHeaterTypeEnum` used by `IfcSpaceHeater.PredefinedType`.
///
/// Monolith target: `SpaceHeater`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpaceHeaterSubType {
    CONVECTOR,
    RADIATOR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSpaceTypeEnum` used by `IfcSpace.PredefinedType`.
///
/// Monolith target: `Space`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpaceSubType {
    BERTH,
    EXTERNAL,
    GFA,
    INTERNAL,
    PARKING,
    SPACE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSpatialZoneTypeEnum` used by `IfcSpatialZone.PredefinedType`.
///
/// Monolith target: `SpatialZone`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpatialZoneSubType {
    CONSTRUCTION,
    FIRESAFETY,
    INTERFERENCE,
    LIGHTING,
    OCCUPANCY,
    RESERVATION,
    SECURITY,
    THERMAL,
    TRANSPORT,
    VENTILATION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcStackTerminalTypeEnum` used by `IfcStackTerminal.PredefinedType`.
///
/// Monolith target: `StackTerminal`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StackTerminalSubType {
    BIRDCAGE,
    COWL,
    RAINWATERHOPPER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcStairFlightTypeEnum` used by `IfcStairFlight.PredefinedType`.
///
/// Monolith target: `StairFlight`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StairFlightSubType {
    CURVED,
    FREEFORM,
    SPIRAL,
    STRAIGHT,
    WINDER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcStairTypeEnum` used by `IfcStair.PredefinedType`.
///
/// Monolith target: `Stair`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StairSubType {
    CURVED_RUN_STAIR,
    DOUBLE_RETURN_STAIR,
    HALF_TURN_STAIR,
    HALF_WINDING_STAIR,
    LADDER,
    QUARTER_TURN_STAIR,
    QUARTER_WINDING_STAIR,
    SPIRAL_STAIR,
    STRAIGHT_RUN_STAIR,
    THREE_QUARTER_TURN_STAIR,
    THREE_QUARTER_WINDING_STAIR,
    TWO_CURVED_RUN_STAIR,
    TWO_QUARTER_TURN_STAIR,
    TWO_QUARTER_WINDING_STAIR,
    TWO_STRAIGHT_RUN_STAIR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcStructuralCurveActivityTypeEnum` used by `IfcStructuralCurveAction.PredefinedType`.
///
/// Monolith target: `StructuralCurveAction`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructuralCurveActivitySubType {
    CONST,
    DISCRETE,
    EQUIDISTANT,
    LINEAR,
    PARABOLA,
    POLYGONAL,
    SINUS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcStructuralCurveMemberTypeEnum` used by `IfcStructuralCurveMember.PredefinedType`.
///
/// Monolith target: `StructuralCurveMember`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructuralCurveMemberSubType {
    CABLE,
    COMPRESSION_MEMBER,
    PIN_JOINED_MEMBER,
    RIGID_JOINED_MEMBER,
    TENSION_MEMBER,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcStructuralSurfaceActivityTypeEnum` used by `IfcStructuralSurfaceAction.PredefinedType`.
///
/// Monolith target: `StructuralSurfaceAction`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructuralSurfaceActivitySubType {
    BILINEAR,
    CONST,
    DISCRETE,
    ISOCONTOUR,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcStructuralSurfaceMemberTypeEnum` used by `IfcStructuralSurfaceMember.PredefinedType`.
///
/// Monolith target: `StructuralSurfaceMember`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructuralSurfaceMemberSubType {
    BENDING_ELEMENT,
    MEMBRANE_ELEMENT,
    SHELL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSubContractResourceTypeEnum` used by `IfcSubContractResource.PredefinedType`.
///
/// Monolith target: `SubContractResource`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubContractResourceSubType {
    PURCHASE,
    WORK,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSurfaceFeatureTypeEnum` used by `IfcSurfaceFeature.PredefinedType`.
///
/// Monolith target: `SurfaceFeature`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurfaceFeatureSubType {
    DEFECT,
    HATCHMARKING,
    LINEMARKING,
    MARK,
    NONSKIDSURFACING,
    PAVEMENTSURFACEMARKING,
    RUMBLESTRIP,
    SYMBOLMARKING,
    TAG,
    TRANSVERSERUMBLESTRIP,
    TREATMENT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSwitchingDeviceTypeEnum` used by `IfcSwitchingDevice.PredefinedType`.
///
/// Monolith target: `SwitchingDevice`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwitchingDeviceSubType {
    CONTACTOR,
    DIMMERSWITCH,
    EMERGENCYSTOP,
    KEYPAD,
    MOMENTARYSWITCH,
    RELAY,
    SELECTORSWITCH,
    STARTER,
    START_AND_STOP_EQUIPMENT,
    SWITCHDISCONNECTOR,
    TOGGLESWITCH,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcSystemFurnitureElementTypeEnum` used by `IfcSystemFurnitureElement.PredefinedType`.
///
/// Monolith target: `SystemFurnitureElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SystemFurnitureElementSubType {
    PANEL,
    SUBRACK,
    WORKSURFACE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTankTypeEnum` used by `IfcTank.PredefinedType`.
///
/// Monolith target: `Tank`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TankSubType {
    BASIN,
    BREAKPRESSURE,
    EXPANSION,
    FEEDANDEXPANSION,
    OILRETENTIONTRAY,
    PRESSUREVESSEL,
    STORAGE,
    VESSEL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTaskTypeEnum` used by `IfcTask.PredefinedType`.
///
/// Monolith target: `Task`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskSubType {
    ADJUSTMENT,
    ATTENDANCE,
    CALIBRATION,
    CONSTRUCTION,
    DEMOLITION,
    DISMANTLE,
    DISPOSAL,
    EMERGENCY,
    INSPECTION,
    INSTALLATION,
    LOGISTIC,
    MAINTENANCE,
    MOVE,
    OPERATION,
    REMOVAL,
    RENOVATION,
    SAFETY,
    SHUTDOWN,
    STARTUP,
    TESTING,
    TROUBLESHOOTING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTendonAnchorTypeEnum` used by `IfcTendonAnchor.PredefinedType`.
///
/// Monolith target: `TendonAnchor`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TendonAnchorSubType {
    COUPLER,
    FIXED_END,
    TENSIONING_END,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTendonConduitTypeEnum` used by `IfcTendonConduit.PredefinedType`.
///
/// Monolith target: `TendonConduit`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TendonConduitSubType {
    COUPLER,
    DIABOLO,
    DUCT,
    GROUTING_DUCT,
    TRUMPET,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTendonTypeEnum` used by `IfcTendon.PredefinedType`.
///
/// Monolith target: `Tendon`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TendonSubType {
    BAR,
    COATED,
    STRAND,
    WIRE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTrackElementTypeEnum` used by `IfcTrackElement.PredefinedType`.
///
/// Monolith target: `TrackElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrackElementSubType {
    BLOCKINGDEVICE,
    DERAILER,
    FROG,
    HALF_SET_OF_BLADES,
    SLEEPER,
    SPEEDREGULATOR,
    TRACKENDOFALIGNMENT,
    VEHICLESTOP,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTransformerTypeEnum` used by `IfcTransformer.PredefinedType`.
///
/// Monolith target: `Transformer`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransformerSubType {
    CHOPPER,
    COMBINED,
    CURRENT,
    FREQUENCY,
    INVERTER,
    RECTIFIER,
    VOLTAGE,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTransportElementTypeEnum` used by `IfcTransportElement.PredefinedType`.
///
/// Monolith target: `TransportElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportElementSubType {
    CRANEWAY,
    ELEVATOR,
    ESCALATOR,
    HAULINGGEAR,
    LIFTINGGEAR,
    MOVINGWALKWAY,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcTubeBundleTypeEnum` used by `IfcTubeBundle.PredefinedType`.
///
/// Monolith target: `TubeBundle`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TubeBundleSubType {
    FINNED,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcUnitaryControlElementTypeEnum` used by `IfcUnitaryControlElement.PredefinedType`.
///
/// Monolith target: `UnitaryControlElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitaryControlElementSubType {
    ALARMPANEL,
    BASESTATIONCONTROLLER,
    COMBINED,
    CONTROLPANEL,
    GASDETECTIONPANEL,
    HUMIDISTAT,
    INDICATORPANEL,
    MIMICPANEL,
    THERMOSTAT,
    WEATHERSTATION,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcUnitaryEquipmentTypeEnum` used by `IfcUnitaryEquipment.PredefinedType`.
///
/// Monolith target: `UnitaryEquipment`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitaryEquipmentSubType {
    AIRCONDITIONINGUNIT,
    AIRHANDLER,
    DEHUMIDIFIER,
    ROOFTOPUNIT,
    SPLITSYSTEM,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcValveTypeEnum` used by `IfcValve.PredefinedType`.
///
/// Monolith target: `Valve`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValveSubType {
    AIRRELEASE,
    ANTIVACUUM,
    CHANGEOVER,
    CHECK,
    COMMISSIONING,
    DIVERTING,
    DOUBLECHECK,
    DOUBLEREGULATING,
    DRAWOFFCOCK,
    FAUCET,
    FLUSHING,
    GASCOCK,
    GASTAP,
    ISOLATING,
    MIXING,
    PRESSUREREDUCING,
    PRESSURERELIEF,
    REGULATING,
    SAFETYCUTOFF,
    STEAMTRAP,
    STOPCOCK,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcVehicleTypeEnum` used by `IfcVehicle.PredefinedType`.
///
/// Monolith target: `Vehicle`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VehicleSubType {
    CARGO,
    ROLLINGSTOCK,
    VEHICLE,
    VEHICLEAIR,
    VEHICLEMARINE,
    VEHICLETRACKED,
    VEHICLEWHEELED,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcVibrationDamperTypeEnum` used by `IfcVibrationDamper.PredefinedType`.
///
/// Monolith target: `VibrationDamper`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VibrationDamperSubType {
    AXIAL_YIELD,
    BENDING_YIELD,
    FRICTION,
    RUBBER,
    SHEAR_YIELD,
    VISCOUS,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcVibrationIsolatorTypeEnum` used by `IfcVibrationIsolator.PredefinedType`.
///
/// Monolith target: `VibrationIsolator`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VibrationIsolatorSubType {
    BASE,
    COMPRESSION,
    SPRING,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcVirtualElementTypeEnum` used by `IfcVirtualElement.PredefinedType`.
///
/// Monolith target: `VirtualElement`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VirtualElementSubType {
    BOUNDARY,
    CLEARANCE,
    PROVISIONFORVOID,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcVoidingFeatureTypeEnum` used by `IfcVoidingFeature.PredefinedType`.
///
/// Monolith target: `VoidingFeature`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoidingFeatureSubType {
    CHAMFER,
    CUTOUT,
    EDGE,
    HOLE,
    MITER,
    NOTCH,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcWallTypeEnum` used by `IfcWall.PredefinedType`.
///
/// Monolith target: `Wall`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WallSubType {
    ELEMENTEDWALL,
    MOVABLE,
    PARAPET,
    PARTITIONING,
    PLUMBINGWALL,
    POLYGONAL,
    RETAININGWALL,
    SHEAR,
    SOLIDWALL,
    STANDARD,
    WAVEWALL,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcWasteTerminalTypeEnum` used by `IfcWasteTerminal.PredefinedType`.
///
/// Monolith target: `WasteTerminal`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WasteTerminalSubType {
    FLOORTRAP,
    FLOORWASTE,
    GULLYSUMP,
    GULLYTRAP,
    ROOFDRAIN,
    WASTEDISPOSALUNIT,
    WASTETRAP,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcWindowTypeEnum` used by `IfcWindow.PredefinedType`.
///
/// Monolith target: `Window`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowSubType {
    LIGHTDOME,
    SKYLIGHT,
    WINDOW,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcWorkCalendarTypeEnum` used by `IfcWorkCalendar.PredefinedType`.
///
/// Monolith target: `WorkCalendar`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkCalendarSubType {
    FIRSTSHIFT,
    SECONDSHIFT,
    THIRDSHIFT,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcWorkPlanTypeEnum` used by `IfcWorkPlan.PredefinedType`.
///
/// Monolith target: `WorkPlan`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkPlanSubType {
    ACTUAL,
    BASELINE,
    PLANNED,
    USERDEFINED,
    NOTDEFINED,
}

/// IFC `IfcWorkScheduleTypeEnum` used by `IfcWorkSchedule.PredefinedType`.
///
/// Monolith target: `WorkSchedule`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkScheduleSubType {
    ACTUAL,
    BASELINE,
    PLANNED,
    USERDEFINED,
    NOTDEFINED,
}

