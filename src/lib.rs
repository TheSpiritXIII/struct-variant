use std::collections::HashMap;

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, Diagnostic, Level};
use quote::quote;
use syn::{spanned::Spanned, Fields, Ident, Item, TraitBound};

// TODO: https://github.com/rust-lang/rust/issues/54722
// TODO: https://github.com/rust-lang/rust/issues/54140
#[proc_macro_error]
#[proc_macro_attribute]
pub fn struct_variant(metadata: TokenStream, input: TokenStream) -> TokenStream {
	let item: Item = if let Ok(item) = syn::parse(input) {
		item
	} else {
		Diagnostic::new(
			Level::Error,
			"Failed to parse input token stream".to_string(),
		)
		.abort();
	};
	let sealed_ident: TraitBound = if let Ok(item) = syn::parse(metadata) {
		item
	} else {
		Diagnostic::new(
			Level::Error,
			"Failed to parse metadata token stream".to_string(),
		)
		.abort();
	};

	let enum_item = if let syn::Item::Enum(ref enum_item) = item {
		enum_item
	} else {
		Diagnostic::spanned(item.span(), Level::Error, "Expected enum".to_string()).abort();
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
