# struct-variant
Provides macros for bounded run-time type information (RTTI) via enums.

## Usage
Imagine a trait `Shape` which is implemented by a finite amount of structs:
```rust
trait Shape {
	fn area(&self) -> usize;
}
struct Circle { ... }
struct Rectangle { ... };
```

Occasionally we may want to pass around a dynamic `Shape` type. In this scenario, `std::any::Any` would not work because it relies on information during the compile time. We can pass a `&dyn Shape` but you wouldn't have enough type information to downcast. To get around that, we can create an enum that holds all of our variants:
```rust
enum ShapeEnum {
	Circle(Circle),
	Rectangle(Rectangle),
}
```

Now instead of passing a `dyn Shape`, we can just pass the `ShapeEnum` and use the `match` keyword.

The problem with this approach is twofold:
1. There's an additional boilerplate. Some languages have sealed types as a keyword.
2. Each variant implements `Shape` so why can't I call `ShapeEnum::area`?

Using this library, we can reduce boilerplate, we can solve the other issue:
```rust
#[struct_variant(Shape)]
enum ShapeEnum {
	Circle,
	Rectangle,
}
```

Here, `Shape` indicates that all items in the enum implement `Shape`. Each item is listed out by the struct name -- it's enum name is automatically inferred from the same struct name. Now we can use this more conveniently:

```rust
fn print_shape(shape: ShapeEnum) {
	let name = match shape {
		ShapeEnum::Circle => "Circle",
		ShapeEnum::Rectangle => "Rectangle",
	};
	println!("Shape: {}, Area: {}", name, shape.as_ref().area());
}
```

We can also call any method with a `dyn Shape` argument with a `Shape` implementation directly, as the macro implements all `From` methods:
```rust
fn print_area(shape: &dyn Shape) {
	println!("Area: {}", shape.area());
}

let circle: ShapeEnum = Circle::with_radius(2).into();
print_area(circle.as_ref());
```

In these examples, `ShapeEnum` includes all implementations of `Shape`. We call this a _sealed type_. If instead it only included a subset of all `Shape` implementation, we can call this partially sealed.

## Trait Bounds
The annotation can include multiple trait bounds:
```rust
fn print_debug(debug: &dyn Debug) {
	println!("Debug: {:?}", debug);
}

#[struct_variant(Shape + Debug)]
enum ShapeEnumWithDebug {
	Circle,
	Rectangle,
}

let circle: ShapeEnumWithDebug = Circle::with_radius(2).into();
print_debug(circle.as_ref());
println!("Manual: {:?}", AsRef::<dyn Debug>::as_ref(&circle));
```

Or none, but you'd have to pattern match whenever you use it:
```rust
#[struct_variant]
enum ShapeEnumNoBounds {
	Circle,
	Rectangle,
}
```
