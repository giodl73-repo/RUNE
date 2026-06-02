use rune_core::{ContractRegistration, DescriptorCollectionDocument, RuneContract};
use rune_derive::RuneContract as DeriveRuneContract;

pub const COLLECTION_ID: &str = "example.shape_calculator_contracts";
pub const COLLECTION_VERSION: &str = "v0";

#[derive(Debug, Clone, Copy, PartialEq, DeriveRuneContract)]
#[rune(
    id = "example.shape.circle",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-074",
    invariant(id = "shape.circle.radius.non_negative", text = "radius >= 0"),
    extension(
        namespace = "shape.calculator",
        name = "area_formula",
        value = "pi * radius^2"
    ),
    extension(
        namespace = "shape.calculator",
        name = "perimeter_formula",
        value = "2 * pi * radius"
    )
)]
pub struct Circle {
    #[rune_field(
        required = true,
        unit = "unit",
        min = "0",
        sensitivity = "public",
        example = "2.0",
        stability = "stable",
        alias = "r"
    )]
    pub radius: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, DeriveRuneContract)]
#[rune(
    id = "example.shape.rectangle",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-074",
    invariant(id = "shape.rectangle.width.non_negative", text = "width >= 0"),
    invariant(id = "shape.rectangle.height.non_negative", text = "height >= 0"),
    extension(
        namespace = "shape.calculator",
        name = "area_formula",
        value = "width * height"
    ),
    extension(
        namespace = "shape.calculator",
        name = "perimeter_formula",
        value = "2 * (width + height)"
    )
)]
pub struct Rectangle {
    #[rune_field(
        required = true,
        unit = "unit",
        min = "0",
        sensitivity = "public",
        example = "3.0",
        stability = "stable",
        alias = "w"
    )]
    pub width: f64,
    #[rune_field(
        required = true,
        unit = "unit",
        min = "0",
        sensitivity = "public",
        example = "4.0",
        stability = "stable",
        alias = "h"
    )]
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq, DeriveRuneContract)]
#[rune(
    id = "example.shape.calculate",
    version = "v0",
    kind = "command",
    requirement = "RUNE-REQ-074",
    invariant(id = "shape.calculate.precision.range", text = "precision <= 12"),
    extension(
        namespace = "shape.calculator",
        name = "operation",
        value = "calculate_area_and_perimeter"
    )
)]
pub struct CalculateShape {
    #[rune_field(
        required = true,
        sensitivity = "internal",
        example = "shape-001",
        stability = "stable"
    )]
    pub shape_id: String,
    #[rune_field(
        required = true,
        min = "0",
        max = "12",
        sensitivity = "public",
        example = "2",
        stability = "stable"
    )]
    pub precision: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, DeriveRuneContract)]
#[rune(
    id = "example.shape.calculated",
    version = "v0",
    kind = "event",
    requirement = "RUNE-REQ-074",
    invariant(id = "shape.calculated.area.non_negative", text = "area >= 0"),
    invariant(
        id = "shape.calculated.perimeter.non_negative",
        text = "perimeter >= 0"
    )
)]
pub struct ShapeCalculated {
    #[rune_field(
        required = true,
        unit = "square-unit",
        min = "0",
        sensitivity = "public",
        example = "12.57",
        stability = "stable"
    )]
    pub area: f64,
    #[rune_field(
        required = true,
        unit = "unit",
        min = "0",
        sensitivity = "public",
        example = "12.57",
        stability = "stable"
    )]
    pub perimeter: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeMetrics {
    pub area: f64,
    pub perimeter: f64,
}

pub const RUNE_CONTRACTS: &[ContractRegistration] = &[
    ContractRegistration {
        name: "Circle",
        descriptor: Circle::descriptor,
    },
    ContractRegistration {
        name: "Rectangle",
        descriptor: Rectangle::descriptor,
    },
    ContractRegistration {
        name: "CalculateShape",
        descriptor: CalculateShape::descriptor,
    },
    ContractRegistration {
        name: "ShapeCalculated",
        descriptor: ShapeCalculated::descriptor,
    },
];

pub fn circle_metrics(circle: Circle) -> ShapeMetrics {
    ShapeMetrics {
        area: std::f64::consts::PI * circle.radius * circle.radius,
        perimeter: 2.0 * std::f64::consts::PI * circle.radius,
    }
}

pub fn rectangle_metrics(rectangle: Rectangle) -> ShapeMetrics {
    ShapeMetrics {
        area: rectangle.width * rectangle.height,
        perimeter: 2.0 * (rectangle.width + rectangle.height),
    }
}

pub fn descriptor_collection() -> Result<DescriptorCollectionDocument, String> {
    DescriptorCollectionDocument::from_registrations(
        COLLECTION_ID,
        COLLECTION_VERSION,
        RUNE_CONTRACTS,
        "RUNE-REG-001",
    )
}
