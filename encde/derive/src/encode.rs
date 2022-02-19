use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote, Attribute, Data, DeriveInput, GenericParam, Generics};

pub fn derive(input: DeriveInput) -> TokenStream2 {
	let name = input.ident;
	let generics = add_trait_bounds(input.generics);
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let implementation = implement(input.attrs, input.data);

	quote! {
		impl #impl_generics ::encde::Encode for #name #ty_generics #where_clause {
			fn encode(&self, writer: &mut dyn std::io::Write) -> ::encde::Result<()> {
				#![allow(non_snake_case)]
				#implementation
				Ok(())
			}
		}
	}
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param.bounds.push(parse_quote!(::encde::Encode));
		}
	}
	generics
}

mod common;
mod enum_impl;
mod struct_impl;

fn implement(attrs: Vec<Attribute>, data: Data) -> TokenStream2 {
	match data {
		Data::Struct(data) => struct_impl::implement(attrs, data),
		Data::Enum(data) => enum_impl::implement(attrs, data),
		Data::Union(_) => unimplemented!(),
	}
}
