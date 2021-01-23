pub trait Shape {
	fn area(&self) -> f64;
}

#[derive(Debug)]
pub struct Circle {
	radius: usize,
}

impl Circle {
	pub fn with_radius(radius: usize) -> Self {
		Self { radius }
	}
}

impl Shape for Circle {
	fn area(&self) -> f64 {
		core::f64::consts::PI * self.radius.pow(2) as f64
	}
}

#[derive(Debug)]
pub struct Rectangle {
	width: usize,
	height: usize,
}

impl Rectangle {
	pub fn new(width: usize, height: usize) -> Self {
		Self { width, height }
	}
}

impl Shape for Rectangle {
	fn area(&self) -> f64 {
		self.width as f64 * self.height as f64
	}
}
