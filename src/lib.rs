use std::collections::HashMap;

use itertools::Itertools;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Ident};

// TODO: https://github.com/rust-lang/rust/issues/54722
// TODO: https://github.com/rust-lang/rust/issues/54140
#[proc_macro_attribute]
pub fn struct_variant(metadata: TokenStream, input: TokenStream) -> TokenStream {
	let item: syn::Item = syn::parse(input).expect("Failed to parse input token stream");
	let sealed_ident: Ident = syn::parse(metadata).expect("Failed to parse metadata token stream");

	let enum_item = if let syn::Item::Enum(ref enum_item) = item {
		enum_item
	} else {
		let e = Error::new_spanned(item, "Expected enum").to_compile_error();
		return e.into();
	};

	let mut struct_map = HashMap::new();
	for variant in &enum_item.variants {
		let mut error_list = Vec::new();
		if !variant.attrs.is_empty() {
			let e = Error::new_spanned(variant, "Expected struct name: found attributes")
				.to_compile_error();
			error_list.push(e);
		}
		if !matches!(variant.fields, syn::Fields::Unit) {
			let e = Error::new_spanned(variant, "Expected struct name: found fields")
				.to_compile_error();
			error_list.push(e);
		}
		if variant.discriminant.is_some() {
			let e = Error::new_spanned(variant, "Expected struct name: found discriminant")
				.to_compile_error();
			error_list.push(e);
		}
		if let Some(a) = struct_map.insert(&variant.ident, variant) {
			let e = Error::new_spanned(
				variant,
				format!("Duplicate variant name: {}", variant.ident),
			)
			.to_compile_error();
			// TODO: Make this a warning.
			let e2 = Error::new_spanned(
				a,
				format!("Duplicate variant name first found here: {}", variant.ident),
			)
			.to_compile_error();
			error_list.push(e);
			error_list.push(e2);
		}
		if !error_list.is_empty() {
			let r = quote! {
				#(#error_list)*
			};
			return r.into();
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
