# Methodology

For clarity and easier future-proofing, this software will closely follow the data structure of **smartBuilding IFC**.

IFC is surprisingly compatible with ECS because its BIM data structure is more horizontal, unlike Revit’s garbage vertical object-oriented structure.

## Naming Convention

For now, the following naming convention has been adopted. It follows two principles:

1. **Clarity and ease of communication**
2. **Alignment with IFC methodology**

| Monolith Term     | IFC Term                                               | Revit Term                                                       |
| ----------------- | ------------------------------------------------------ | ---------------------------------------------------------------- |
| `ElementKind`     | `IfcWall`, `IfcBeam`, `IfcColumn` — entity class       | Category                                                         |
| `ElementKindType` | `PredefinedType` enum — `RETAININGWALL`, `JOIST`, etc. | No clean equivalent; closest is family purpose/category behavior |
| `ElementKindSpec` | `IfcTypeObject` — `IfcWallType`, `IfcBeamType`, etc.   | Family Type                                                      |
| `Element`         | `IfcObject` occurrence                                 | Instance                                                         |

**Note**: `ElementKindType` is an enum field value - an identifier only, basically a name that further categorizes an Element. `ElementKindSpec` is the "type" equivalent in revit which is the shared object definitions that are used e.g. `Brick Wall 200mm`, `Signal Data Outlet`, etc