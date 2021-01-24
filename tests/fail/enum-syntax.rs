use struct_variant::struct_variant;

#[struct_variant]
enum ShapeEnumNoBound {
	Circle { x: i32, y: i32 },
	Rectangle(Rectangle, Rectangle),
}

#[struct_variant(Shape)]
enum ShapeEnumShapeBound {
	Circle { x: i32, y: i32 },
	Rectangle(Rectangle, Rectangle),
}

fn main() {}
