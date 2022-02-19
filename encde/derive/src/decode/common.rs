use crate::attrs::{parse_crate_attributes, FieldAttributes, StructAttributes};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Fields, FieldsNamed, FieldsUnnamed, Index};

fn to_field_name(field_ident: TokenStream2) -> TokenStream2 {
	let ident = format_ident!("{}{}", crate::FIELD_PREFIX, field_ident.to_string());
	quote! { #ident }
}

fn maybe_read_padding(padding: usize) -> TokenStream2 {
	if padding != 0 {
		quote! { ::encde::util::read_padding(reader, #padding)?; }
	} else {
		quote! {}
	}
}

pub fn implement_struct_body(type_name: &TokenStream2, _struct_attrs: StructAttributes, fields: Fields) -> TokenStream2 {
	match fields {
		Fields::Named(FieldsNamed { named: fields, .. }) => {
			let (sub_expressions, field_assignments) = fields
				.into_iter()
				.map(|field| {
					let FieldAttributes { pad_before, pad_after } = parse_crate_attributes(&field.attrs).expect("Could not parse FieldAttributes");
					let actual_name = &field.ident;
					let field_type = &field.ty;
					let pad_before = maybe_read_padding(pad_before);
					let pad_after = maybe_read_padding(pad_after);
					let temp_name = to_field_name(quote! { #actual_name });

					let sub_expression = quote! {
						#pad_before
						let #temp_name: #field_type = ::encde::Decode::decode(reader)?;
						#pad_after
					};
					let field_assignment = quote! {
						#actual_name: #temp_name
					};
					(sub_expression, field_assignment)
				})
				.fold((vec![], vec![]), |mut acc, item| {
					acc.0.push(item.0);
					acc.1.push(item.1);
					acc
				});
			quote! {
				#(#sub_expressions)*
				Ok(#type_name{ #(#field_assignments),* })
			}
		}
		Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) => {
			let (sub_expressions, field_assignments) = fields
				.into_iter()
				.enumerate()
				.map(|(idx, field)| {
					let FieldAttributes { pad_before, pad_after } = parse_crate_attributes(&field.attrs).expect("Could not parse FieldAttributes");
					let index: Index = idx.into();
					let pad_before = maybe_read_padding(pad_before);
					let pad_after = maybe_read_padding(pad_after);
					let temp_name = to_field_name(quote! { #index });

					let sub_expression = quote! {
						#pad_before
						let #temp_name = ::encde::Decode::decode(reader)?;
						#pad_after
					};
					let field_assignment = quote! {
						#temp_name
					};
					(sub_expression, field_assignment)
				})
				.fold((vec![], vec![]), |mut acc, item| {
					acc.0.push(item.0);
					acc.1.push(item.1);
					acc
				});
			quote! {
				#(#sub_expressions)*
				Ok(#type_name(#(#field_assignments),*))
			}
		}
		// zero-sized type has no representation
		Fields::Unit => quote! {
			Ok(#type_name)
		},
	}
}
