#[path = "../common/mod.rs"]
mod common;

use common::*;
use core::fmt::Debug;
use struct_variant::struct_variant;

#[struct_variant(Shape)]
enum ShapeEnum {
	Circle,
	Rectangle,
}

#[struct_variant(Shape)]
#[derive(Debug)]
enum ShapeEnumWithDerive {
	Circle,
	Rectangle,
}

fn main() {
	assert_shape_bound::<ShapeEnum>();
	assert_shape_from_area_pattern!(ShapeEnum);

	assert_shape_bound::<ShapeEnumWithDerive>();
	assert_shape_from_area_pattern!(ShapeEnumWithDerive);
	assert_shape_from::<ShapeEnumWithDerive>()
		.iter()
		.for_each(|shape| {
			assert_compile_time_trait::<dyn Debug>(shape);
		});
}
