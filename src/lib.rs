// #![feature(extended_key_value_attributes)]
// #[doc = include_str!("../README.md")]

use std::collections::HashMap;

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, Diagnostic, Level};
use quote::quote;
use syn::{Fields, Ident, TraitBound, punctuated::Punctuated, spanned::Spanned, Token, parse::Parser};

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

	let enum_item: syn::ItemEnum = match syn::parse(input) {
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
		if !variant.attrs.is_empty() {
			Diagnostic::spanned(
				variant.span(),
				Level::Error,
				"Expected struct name: found attributes".to_string(),
			)
			.emit();
		}
		if !matches!(variant.fields, Fields::Unit) {
			Diagnostic::spanned(
				variant.span(),
				Level::Error,
				"Expected struct name: found fields".to_string(),
			)
			.emit();
		}
		if variant.discriminant.is_some() {
			Diagnostic::spanned(
				variant.span(),
				Level::Error,
				"Expected struct name: found discriminant".to_string(),
			)
			.emit();
		}
		if let Some(variant_duplicate) = struct_map.insert(&variant.ident, variant) {
			Diagnostic::spanned(
				variant.span(),
				Level::Error,
				format!("Duplicate variant name: {}", variant.ident),
			)
			.span_note(
				variant_duplicate.span(),
				"Duplicate variant name first found here".to_string(),
			)
			.emit()
		}
	}

	let attrs = &enum_item.attrs;
	let vis = &enum_item.vis;
	let ident = &enum_item.ident;
	let generics = &enum_item.generics;

	let struct_list: Vec<_> = struct_map.iter().map(|(id, _)| id).sorted().collect();
	let bound_list: Vec<&Ident> = bound_item
		.iter()
		.map(|trait_bound| trait_bound.path.get_ident())
		.map(Option::unwrap)
		.collect();

	let cast_tokens = quote! {
		#( #ident::#struct_list(ref value) => value ),*
	};

	// TODO: https://github.com/rust-lang/rust/issues/75294
	let result = quote! {
		#(#attrs)*
		#vis enum #ident#generics {
			#(#struct_list(#struct_list)),*
		}

		#(
			impl From<#struct_list> for #ident {
				fn from(value: #struct_list) -> Self {
					Self::#struct_list(value)
				}
			}
		)*

		#(
			impl<'a> AsRef<dyn #bound_list + 'a> for #ident {
				fn as_ref(&self) -> &(dyn #bound_list + 'a) {
					match self {
						#cast_tokens
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
	// t.pass("tests/pass/rename.rs");
	// t.pass("tests/pass/generic.rs");
}
