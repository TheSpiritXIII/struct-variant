use std::collections::HashMap;

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::{Diagnostic, Level};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, Fields, Ident, Item, TraitBound};

// TODO: https://github.com/rust-lang/rust/issues/54722
// TODO: https://github.com/rust-lang/rust/issues/54140
#[proc_macro_attribute]
pub fn struct_variant(metadata: TokenStream, input: TokenStream) -> TokenStream {
	let item: Item = syn::parse(input).expect("Failed to parse input token stream");
	let sealed_ident: TraitBound =
		syn::parse(metadata).expect("Failed to parse metadata token stream");

	let enum_item = if let syn::Item::Enum(ref enum_item) = item {
		enum_item
	} else {
		let e = Error::new_spanned(item, "Expected enum").to_compile_error();
		return e.into();
	};

	let mut struct_map = HashMap::new();
	let mut diagnostic_list = Vec::new();
	for variant in &enum_item.variants {
		if !variant.attrs.is_empty() {
			diagnostic_list.push(Diagnostic::spanned(
				variant.span(),
				Level::Error,
				"Expected struct name: found attributes".to_string(),
			));
		}
		if !matches!(variant.fields, Fields::Unit) {
			diagnostic_list.push(Diagnostic::spanned(
				variant.span(),
				Level::Error,
				"Expected struct name: found fields".to_string(),
			));
		}
		if variant.discriminant.is_some() {
			diagnostic_list.push(Diagnostic::spanned(
				variant.span(),
				Level::Error,
				"Expected struct name: found discriminant".to_string(),
			));
		}
		if let Some(variant_duplicate) = struct_map.insert(&variant.ident, variant) {
			diagnostic_list.push(
				Diagnostic::spanned(
					variant.span(),
					Level::Error,
					format!("Duplicate variant name: {}", variant.ident),
				)
				.span_note(
					variant_duplicate.span(),
					"Duplicate variant name first found here".to_string(),
				),
			);
		}
	}
	if !diagnostic_list.is_empty() {
		let r = quote! {
			#(#diagnostic_list)*
		};
		return r.into();
	}

	let attrs = &enum_item.attrs;
	let vis = &enum_item.vis;
	let ident = &enum_item.ident;
	let generics = &enum_item.generics;

	// TODO: https://github.com/rust-lang/rust/issues/75294
	let struct_list: Vec<&Ident> = struct_map.iter().map(|(id, _)| *id).sorted().collect();
	let x = quote! {
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

		impl<'a> AsRef<dyn #sealed_ident + 'a> for #ident {
			fn as_ref(&self) -> &(dyn #sealed_ident + 'a) {
				match self {
					#( #ident::#struct_list(ref value) => value ),*
				}
			}
		}
	};
	x.into()
}
