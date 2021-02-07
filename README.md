# struct-variant
Minimal helper macros to generate enums out of a group of structs.

## Usage
Imagine a trait `Shape` which is implemented by a finite amount of structs:
```rust
trait Shape {
	fn area(&self) -> usize;
}
struct Circle { ... }
struct Rectangle { ... };
```

Occasionally we may want to pass around a dynamic `Shape` type. In this scenario, [`std::any::Any`](https://doc.rust-lang.org/std/any/trait.Any.html) would not work because it relies on information during the compile time. We can pass a `&dyn Shape` but you wouldn't have enough type information to downcast back to the original types. To get around both of those issues, we can create an enum that holds all of our variants:
```rust
enum ShapeEnum {
	Circle(Circle),
	Rectangle(Rectangle),
}
```

Now instead of passing a `dyn Shape`, we can just pass the `ShapeEnum` and use the `match` keyword for downcasting.

The problem with this approach is twofold:
1. There's additional boilerplate. In the above example, we write each struct twice. Other languages have integrated sealed types.
2. Each variant implements `Shape` so why can't I call `Shape::area`?

This library helps with both of those issues:
```rust
#[struct_variant(Shape)]
enum ShapeEnum {
	Circle,
	Rectangle,
}
```

Boilerplate is slightly reduced. Each struct is listed only once. We also include `Shape` in the macro attribute which indicates that all structs in the enum implement `Shape`. You can use `AsRef` to cast your type to any type listed in the macro attribute. Now we can use this more conveniently:

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

`From` is implemented for each struct:
```rust
fn print_area(shape: &dyn Shape) {
	println!("Area: {}", shape.area());
}

let circle: ShapeEnum = Circle::with_radius(2).into();
print_area(circle.as_ref());
```

In these examples, `ShapeEnum` includes implementations of `Shape`. A type that implements _all_ subtypes of a base type is known as _sealed type_ in other languages.

While this library can't guarantee that you include all subtypes, with due diligence you can write libraries that do. The [sealed trait design pattern](https://rust-lang.github.io/api-guidelines/future-proofing.html) ensures that any implementations of your library cannot add new subtypes and thus in conjunction with this library, you can ensure they are sealed types.

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

Or you can also opt for no trait bounds at all. You'd still get all the `From` implementations but you'd have to pattern match whenever you use it:
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
