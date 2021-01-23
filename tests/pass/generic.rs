use std::marker::PhantomData;
use struct_variant::struct_variant;

trait Marker {}

struct Value<X> {
	x: X,
}

impl Marker for Value<X> {}

struct Phantom<X> {
	phantom: PhantomType<X>,
}

impl Marker for Phantom<X> {}

#[struct_variant(Marker)]
enum GenericAlias {
	Sized(Value<isize>),
	Unsized(Value<usize>),
}

#[struct_variant(Marker)]
enum GenericShare<X> {
	Value(Value<X>),
	Phantom(Phantom<X>),
}

#[struct_variant(Marker)]
enum Generic<X, Y> {
	Value(Value<X>),
	Phantom(Phantom<Y>),
}

fn main() {
	todo!();
}
