use crate::attrs::{parse_crate_attributes, FieldAttributes, StructAttributes};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed, Index};

pub fn implement_struct_body(_struct_attrs: StructAttributes, fields: Fields, to_field_getter: fn(field_name: TokenStream2) -> TokenStream2) -> TokenStream2 {
	match fields {
		Fields::Named(FieldsNamed { named: fields, .. }) => {
			let sub_expressions = fields.into_iter().map(|field| {
				let FieldAttributes { pad_before, pad_after } = parse_crate_attributes(&field.attrs).expect("Could not parse FieldAttributes");
				let name = &field.ident;
				let ty = &field.ty;
				let field_getter = to_field_getter(quote! { #name });
				let pad_before = if pad_before != 0 {
					quote! {
						::encde::util::write_padding(writer, #pad_before)?;
					}
				} else {
					quote! {}
				};
				let pad_after = if pad_after != 0 {
					quote! {
						::encde::util::write_padding(writer, #pad_after)?;
					}
				} else {
					quote! {}
				};
				quote! {
					#pad_before
					<#ty as ::encde::Encode>::encode(&#field_getter, writer)?;
					#pad_after
				}
			});
			quote! {
				#(#sub_expressions)*
			}
		}
		Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) => {
			let sub_expressions = fields.into_iter().enumerate().map(|(idx, field)| {
				let FieldAttributes { pad_before, pad_after } = parse_crate_attributes(&field.attrs).expect("Could not parse FieldAttributes");
				let index: Index = idx.into();
				let ty = &field.ty;
				let field_getter = to_field_getter(quote! { #index });
				let pad_before = if pad_before != 0 {
					quote! {
						::encde::util::write_padding(writer, #pad_before)?;
					}
				} else {
					quote! {}
				};
				let pad_after = if pad_after != 0 {
					quote! {
						::encde::util::write_padding(writer, #pad_after)?;
					}
				} else {
					quote! {}
				};
				quote! {
					#pad_before
					<#ty as ::encde::Encode>::encode(&#field_getter, writer)?;
					#pad_after
				}
			});
			quote! {
				#(#sub_expressions)*
			}
		}
		// zero-sized type has no representation
		Fields::Unit => quote! {},
	}
}
