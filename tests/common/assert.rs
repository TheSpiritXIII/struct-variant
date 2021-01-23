// use std::any::{type_name, Any};

use crate::common::{Circle, Rectangle, Shape};

// pub fn assert_type<T: Any, U: Any>(value: &U) {
// 	let value_any = value as &dyn Any;

// 	if value_any.is::<T>() == false {
// 		panic!(
// 			"Expected type `{}` but found type `{}`",
// 			type_name::<U>(),
// 			type_name::<T>()
// 		);
// 	}
// }

pub const fn assert_compile_time_trait<T: ?Sized>(_: &T) {}

/// Causes a compiler error if the given type doesn't support From for each Shape variant.
pub fn assert_shape_from<U: From<Rectangle> + From<Circle>>() -> [U; 2] {
	let circle: U = Circle::with_radius(12).into();
	let rectangle: U = Rectangle::new(4, 5).into();
	[circle, rectangle]
}

/// Verifies that the `Shape::area` functions return the right value through the given function.
pub fn assert_shape_from_area<U: From<Rectangle> + From<Circle>>(area_fn: fn(&U) -> f64) {
	let variant_array = assert_shape_from::<U>();
	let variant_area_array = [452.3893421169302f64, 20f64];
	for (variant, area) in variant_array.iter().zip(variant_area_array.iter()) {
		assert_eq!(area_fn(variant), *area);
	}
}

/// Verifies that the shape is correctly bound to `dyn Shape`.
pub fn assert_shape_bound<U: From<Rectangle> + From<Circle> + AsRef<dyn Shape>>() {
	assert_shape_from_area::<U>(|variant| variant.as_ref().area());
}

/// Verifies that `Shape::area` functions return the right value through pattern matching.
#[macro_export]
macro_rules! assert_shape_from_area_pattern {
	(@match $var:ident, $($path:path),*) => {
		match $var {
			$(
				$path(value) => value.area(),
			)*
		}
	};
	($t:ident) => {
		assert_shape_from_area::<$t>(|variant| {
			assert_shape_from_area_pattern!(@match variant, $t::Circle, $t::Rectangle)
		});
	};
}
