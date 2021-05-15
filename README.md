# struct-variant
[![Crates][crates-badge]][crates-url]
[![License][license-badge]][license-url]
[![Build][build-badge]][build-url]

[crates-badge]: https://img.shields.io/crates/v/struct-variant.svg
[crates-url]: https://crates.io/crates/struct-variant
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/TheSpiritXIII/struct-variant/blob/main/LICENSE
[build-badge]: https://github.com/TheSpiritXIII/struct-variant/workflows/Rust/badge.svg
[build-url]: https://github.com/TheSpiritXIII/struct-variant/actions?query=workflow%3ARust+branch%3Amain

Minimal helper macro to generate an enum out of a list of structs.

## 60 Seconds Example
Install:
```rust
cargo install struct-variant;
```

Setup:
```rust
// Common trait.
trait Event {
	fn apply(&self);
};

// First trait implementation.
struct MouseEvent {
	// ...
}
impl Event for MouseEvent {
	fn apply(&self) {
		println!("Applied mouse");
	}
}

// Second trait implementation.
struct KeyboardEvent {
	// ...
}
impl Event for KeyboardEvent {
	fn apply(&self) {
		println!("Applied keyboard");
	}
}

// The *magic*!
#[struct_variant(Event)]
enum EventEnum {
	MouseEvent,
	KeyboardEvent,
}
```

Result:
```rust
fn process_event(event: EventEnum) {
	// No need to match to get inner type.
	event.as_ref().apply();

	// But you can still match if you want to.
	match event {
		EventEnum::MouseEvent(_) => println!("Got mouse"),
		EventEnum::KeyboardEvent(_) => println!("Got keyboard"),
	}
}

fn main() {
	// From<MouseEvent> for EventEnum implemented for you.
	let mouse_event = MouseEvent {};
	process_event(mouse_event.into())
}
```

## Motivation
Suppose you have a trait `Shape` which is implemented by a finite amount of structs:
```rust
trait Shape {
	fn area(&self) -> usize;
}
struct Circle { ... }
struct Rectangle { ... };
```

Occasionally we may want to pass around a dynamic `Shape` type but want the ability to downcast. In this scenario, [`std::any::Any`](https://doc.rust-lang.org/std/any/trait.Any.html) would not work because it relies on information during the compile time. We can pass a `&dyn Shape` but you wouldn't have enough type information. To get around both of those issues, we can create an enum that holds all of our variants:
```rust
enum ShapeEnum {
	Circle(Circle),
	Rectangle(Rectangle),
}
```

Now, instead of passing a `dyn Shape`, we can just pass the `ShapeEnum` and use the `match` keyword for downcasting. The problem with this approach is twofold:
1. Each variant implements `Shape` so why doesn't `ShapeEnum` implement all methods from `Shape`?
2. There's additional boilerplate. In the above example, we write each struct twice. Other languages have sealed types.

This library helps with both of those issues:
```rust
#[struct_variant(Shape)]
enum ShapeEnum {
	Circle,
	Rectangle,
}
```

We include `Shape` in the macro attribute which indicates that all structs in the enum implement `Shape`. You can use [`std::convert::AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) to cast your type to any type listed in the macro attribute. Now we can use this more conveniently:

```rust
fn print_shape(shape: ShapeEnum) {
	// We can use pattern matching to downcast.
	let name = match shape {
		ShapeEnum::Circle(_) => "Circle",
		ShapeEnum::Rectangle(_) => "Rectangle",
	};

	// AsRef<dyn Shape> is implemented for you.
	println!("Shape: {}, Area: {}", name, shape.as_ref().area());
}
```

[`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html) is implemented for each struct so you can also upcast:
```rust
fn print_area(shape: &dyn Shape) {
	println!("Area: {}", shape.area());
}

let circle: ShapeEnum = Circle::with_radius(2).into();
print_area(circle.as_ref());
```

In these examples, `ShapeEnum` includes two implementations of `Shape`: `Circle` and `Rectangle`. A type that implements _all_ subtypes of a base type is known as _sealed type_ in other languages. While this library can't guarantee that you include all subtypes in your enum, with due diligence you can write libraries that do. The [sealed trait design pattern](https://rust-lang.github.io/api-guidelines/future-proofing.html) ensures that consumers of your library cannot add new subtypes and thus in conjunction with this library, you can ensure they are sealed types.

## Trait Bounds
Earlier examples created a `ShapeEnum` type over the `Shape` trait. The macro attribute allows multiple trait bounds:
```rust
#[struct_variant(Shape + Debug)]
enum ShapeEnumWithDebug {
	Circle,
	Rectangle,
}

fn print_debug(debug: &dyn Debug) {
	println!("Debug: {:?}", debug);
}

let circle: ShapeEnumWithDebug = Circle::with_radius(2).into();
print_debug(circle.as_ref());
println!("Manual: {:?}", AsRef::<dyn Debug>::as_ref(&circle));
```

Or you can also opt for no trait bounds at all. You'd still get all the [`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html) implementations:
```rust
#[struct_variant]
enum ShapeEnumNoBounds {
	Circle,
	Rectangle,
}
```

## Name Conflicts
One source of contention with this library are structs with conflicting names. Suppose we have two different `Foo` in both the `bar` and `baz` namespaces. One simple way to use them with this library is using `as` when importing one or both of them:
```rust
use bar::Foo as FooBar;
use baz::Foo;

#[struct_variant]
enum Qux {
	FooBar,
	Foo
}
```

An alternative to this is using the standard enum syntax to declare which struct it uses:
```rust
use baz::Foo;

#[struct_variant]
enum Qux {
	FooBar(bar::Foo),
	Foo
}
```

The downside with both approaches is that you have to use the re-exported name during pattern matching.

We can use the same solution for supporting multiple variants of the same generic type:
```rust
struct Foo<X> {
	phantom: PhantomType<X>,
};

enum Qux {
	FooU8(Foo<u8>),
	FooI8(Foo<i8>),
}
```

## Contributing
Contributions are welcome and highly appreciated!

Please run the formatter and linter first:
```rust
cargo +nightly fmt
cargo clippy
```
