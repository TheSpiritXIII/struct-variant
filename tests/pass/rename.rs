#[path = "../common/mod.rs"]
mod common;

use common::*;
use core::fmt::Debug;
use struct_variant::struct_variant;

#[struct_variant(Shape)]
enum ShapeEnum {
	Round(Circle),
	Rectangle,
}

#[struct_variant(Shape)]
#[derive(Debug)]
enum ShapeEnumWithDerive {
	Round(Circle),
	Rectangle,
}

fn main() {
	assert_shape_bound::<ShapeEnum>();
	assert_shape_from_area::<ShapeEnum>(|variant| {
		match variant {
			ShapeEnum::Round(circle) => circle.area(),
			ShapeEnum::Rectangle(rectangle) => rectangle.area(),
		}
	});

	assert_shape_bound::<ShapeEnumWithDerive>();
	assert_shape_from_area::<ShapeEnumWithDerive>(|variant| {
		match variant {
			ShapeEnumWithDerive::Round(circle) => circle.area(),
			ShapeEnumWithDerive::Rectangle(rectangle) => rectangle.area(),
		}
	});
	assert_shape_from::<ShapeEnumWithDerive>()
		.iter()
		.for_each(|shape| {
			assert_compile_time_trait::<dyn Debug>(shape);
		});
}
