#[path="../common/mod.rs"]
mod common;

use common::*;
use struct_variant::struct_variant;

#[struct_variant]
enum ShapeEnumNoBound {
	Circle,
	Rectangle,
	DoesNotExistNoBound // Will error.
}

#[struct_variant(Shape)]
enum ShapeEnumShapeBound {
	Circle,
	Rectangle,
	DoesNotExistShapeBound // Will error.
}

fn main() {}
