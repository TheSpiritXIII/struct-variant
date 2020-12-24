use struct_variant::struct_variant;
use core::fmt::Debug;

trait Shape {
	fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
	radius: usize,
}

impl Circle {
	fn with_radius(radius: usize) -> Self {
		Self { radius }
	}
}

impl Shape for Circle {
	fn area(&self) -> f64 {
		core::f64::consts::PI * self.radius.pow(2) as f64
	}
}

#[derive(Debug)]
struct Rectangle {
	width: usize,
	height: usize,
}

impl Rectangle {
	fn new(width: usize, height: usize) -> Self {
		Self { width, height }
	}
}

impl Shape for Rectangle {
	fn area(&self) -> f64 {
		self.width as f64 * self.height as f64
	}
}

fn print_area(shape: &dyn Shape) {
	println!("Area: {}", shape.area());
}

#[struct_variant(Shape)]
#[derive(Debug)]
enum ShapeEnum {
	Circle,
	Rectangle,
}

fn print_shape(shape: &ShapeEnum) {
	let name = match shape {
		ShapeEnum::Circle(_) => "Circle",
		ShapeEnum::Rectangle(_) => "Rectangle",
	};
	println!("Shape: {}, Area: {}", name, shape.as_ref().area());
	println!("Debug ShapeEnum: {:?}", shape);
}

#[test]
fn test() {
	let circle: ShapeEnum = Circle::with_radius(2).into();
	let rectangle: ShapeEnum = Rectangle::new(2, 3).into();
	print_area(circle.as_ref());
	print_area(rectangle.as_ref());
	print_shape(&circle);
	print_shape(&rectangle);
	print_debug(&circle);
	print_debug(&rectangle);
}

#[struct_variant(Shape + Debug)]
enum ShapeEnumWithDebug {
	Circle,
	Rectangle,
}

fn print_debug<T: Debug + ?Sized>(debug: &T) {
	println!("Debug: {:?}", debug);
}

#[test]
fn test_multi() {
	let circle: ShapeEnumWithDebug = Circle::with_radius(2).into();
	let rectangle: ShapeEnumWithDebug = Rectangle::new(2, 3).into();
	print_area(circle.as_ref());
	print_area(rectangle.as_ref());
	print_debug::<dyn Debug>(circle.as_ref());
	println!("Manual: {:?}", AsRef::<dyn Debug>::as_ref(&circle));
	print_debug::<dyn Debug>(rectangle.as_ref());
	println!("Manual: {:?}", AsRef::<dyn Debug>::as_ref(&rectangle));
}

#[struct_variant]
enum ShapeEnumNoBounds {
	Circle,
	Rectangle,
}
