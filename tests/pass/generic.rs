use std::marker::PhantomData;
use struct_variant::struct_variant;

trait Marker {}

struct Value<X> {
	x: X,
}

impl<X> Marker for Value<X> {}

struct Phantom<X> {
	phantom: PhantomData<X>,
}

impl<X> Marker for Phantom<X> {}

#[struct_variant(Marker)]
enum GenericAlias {
	Unsized(Value<u8>),
	Sized(Value<i8>),
}

fn test_generic_alias() {
	let value =  Value { x: 1i8 };
	let alias: GenericAlias = value.into();
	assert!(matches!(alias, GenericAlias::Sized(value) if value.x == 1i8))
}

#[struct_variant(Marker)]
enum GenericShare<X> {
	Value(Value<X>),
	Phantom(Phantom<X>),
}

fn test_generic_share() {
	let value =  Value { x: 1i8 };
	let alias: GenericShare<_> = value.into();
	assert!(matches!(alias, GenericShare::Value(value) if value.x == 1i8))
}

#[struct_variant(Marker)]
enum Generic<X, Y> {
	Value(Value<X>),
	Phantom(Phantom<Y>),
}

fn test_generic() {
	let value =  Value { x: 1i8 };
	let alias: Generic<_, u8> = value.into();
	assert!(matches!(alias, Generic::Value(value) if value.x == 1i8))
}

fn main() {
	test_generic_alias();
	test_generic_share();
	test_generic();
}
