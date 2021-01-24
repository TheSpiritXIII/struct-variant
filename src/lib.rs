// #![feature(extended_key_value_attributes)]
// #[doc = include_str!("../README.md")]

use std::collections::HashMap;

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, Diagnostic, Level};
use quote::quote;
use syn::{
	braced,
	parenthesized,
	parse,
	parse::{Parse, ParseStream, Parser},
	punctuated::Punctuated,
	token::{Brace, Paren},
	Attribute,
	Generics,
	Ident,
	Token,
	TraitBound,
	Visibility,
};

struct Field {
	pub paren_token: Paren,
	pub ident: Ident,
}

impl Parse for Field {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let content;
		Ok(Field {
			paren_token: parenthesized!(content in input),
			ident: content.parse()?,
		})
	}
}

struct Variant {
	pub ident: Ident,
	pub field: Option<Field>,
}

impl Parse for Variant {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		Ok(Variant {
			ident: input.parse()?,
			field: {
				if input.peek(Paren) {
					let field: Field = input.parse()?;
					Some(field)
				} else {
					None
				}
			},
		})
	}
}

struct VariantEnum {
	pub attrs: Vec<Attribute>,
	pub vis: Visibility,
	pub enum_token: Token![enum],
	pub ident: Ident,
	pub generics: Generics,
	pub brace_token: Brace,
	pub variants: Punctuated<Variant, Token![,]>,
}

impl Parse for VariantEnum {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let content;
		Ok(VariantEnum {
			attrs: input.call(Attribute::parse_outer)?,
			vis: input.parse()?,
			enum_token: input.parse()?,
			ident: input.parse()?,
			generics: input.parse()?,
			brace_token: braced!(content in input),
			variants: content.parse_terminated(Variant::parse)?,
		})
	}
}

// TODO: https://github.com/rust-lang/rust/issues/54722
// TODO: https://github.com/rust-lang/rust/issues/54140
#[proc_macro_error]
#[proc_macro_attribute]
pub fn struct_variant(metadata: TokenStream, input: TokenStream) -> TokenStream {
	let parser = Punctuated::<TraitBound, Token![+]>::parse_terminated;
	let bound_item = match parser.parse(metadata) {
		Ok(item) => item,
		Err(e) => Diagnostic::spanned(
			e.span(),
			Level::Error,
			format!("Unable to parse struct variant attribute: {} ", e),
		)
		.abort(),
	};

	let enum_item: VariantEnum = match parse(input) {
		Ok(item) => item,
		Err(e) => Diagnostic::spanned(
			e.span(),
			Level::Error,
			format!("Failed to parse struct variant input: {}", e),
		)
		.abort(),
	};

	let mut struct_map = HashMap::new();
	for variant in &enum_item.variants {
		let ident = &variant.ident;
		if let Some(variant_duplicate) = struct_map.insert(ident.clone(), variant) {
			Diagnostic::spanned(
				variant.ident.span(),
				Level::Error,
				format!("Duplicate variant name: {}", &ident),
			)
			.span_note(
				variant_duplicate.ident.span(),
				"Duplicate variant name first found here".to_string(),
			)
			.emit()
		}
	}

	let attrs = &enum_item.attrs;
	let vis = &enum_item.vis;
	let ident = &enum_item.ident;
	let generics = &enum_item.generics;

	let enum_list: Vec<_> = struct_map
		.values()
		.map(|variant| {
			let struct_ident = variant
				.field
				.as_ref()
				.map_or(&variant.ident, |field| &field.ident);
			(&variant.ident, struct_ident)
		})
		.sorted()
		.collect();
	let bound_list: Vec<&Ident> = bound_item
		.iter()
		.map(|trait_bound| trait_bound.path.get_ident())
		.map(Option::unwrap)
		.collect();

	let enum_field = enum_list.iter().map(|(variant_ident, struct_ident)| {
		quote! {
			#variant_ident(#struct_ident)
		}
	});

	let from_impl = enum_list.iter().map(|(variant_ident, struct_ident)| {
		quote! {
			impl From<#struct_ident> for #ident {
				fn from(value: #struct_ident) -> Self {
					Self::#variant_ident(value)
				}
			}
		}
	});

	let variant_list: Vec<_> = enum_list.iter().map(|(id, _)| id).collect();
	let as_ref_match_arm = quote! {
		#( #ident::#variant_list(ref value) => value ),*
	};

	// TODO: https://github.com/rust-lang/rust/issues/75294
	let result = quote! {
		#(#attrs)*
		#vis enum #ident#generics {
			#(#enum_field),*
		}

		#(#from_impl)*

		#(
			impl<'a> AsRef<dyn #bound_list + 'a> for #ident {
				fn as_ref(&self) -> &(dyn #bound_list + 'a) {
					match self {
						#as_ref_match_arm
					}
				}
			}
		)*
	};
	result.into()
}

#[test]
fn ui() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/fail/missing-struct.rs");
	t.compile_fail("tests/fail/enum-syntax.rs");
	t.compile_fail("tests/fail/not-enum.rs");
	t.pass("tests/pass/bound-single.rs");
	t.pass("tests/pass/bound-none.rs");
	t.pass("tests/pass/bound-multi.rs");
	t.pass("tests/pass/rename.rs");
	// t.pass("tests/pass/generic.rs");
}
