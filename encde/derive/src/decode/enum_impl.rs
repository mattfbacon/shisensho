use crate::attrs::{get_repr_attribute, parse_crate_attributes, EnumAttributes, StructVariantAttributes, VariantAttributes};
use crate::tracker::DiscriminantTracker;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Attribute, DataEnum};

pub fn implement(attrs: Vec<Attribute>, data: DataEnum, enum_name: &Ident) -> TokenStream2 {
	let EnumAttributes {} = parse_crate_attributes(&attrs).expect("Could not parse EnumAttributes");
	let enum_repr = get_repr_attribute(&attrs).expect("Could not parse repr attribute").unwrap_or_else(|| "u32".to_owned());
	let mut tracker = DiscriminantTracker::new_from_type(&enum_repr);
	let sub_expressions: Vec<_> = data
		.variants
		.into_iter()
		.map(|variant| {
			let ident = &variant.ident;
			let ident = quote! { Self::#ident };
			let StructVariantAttributes {
				struct_attrs,
				variant_attrs: VariantAttributes { wire_tag },
			} = parse_crate_attributes(&variant.attrs).expect("Could not parse StructVariantAttributes");
			let discriminant = tracker.next_variant(&wire_tag, &variant.discriminant.map(|(_, expr)| expr)).unwrap_or_else(syn::Error::into_compile_error);
			let implementation = super::common::implement_struct_body(&ident, struct_attrs, variant.fields);
			quote! {
				#discriminant => { #implementation }
			}
		})
		.collect(); // collect so all the variants are processed by LastVariant
	let enum_repr = Ident::new(&enum_repr, Span::call_site());
	let (num_enum_values, enum_values) = {
		let emitted = tracker.emitted();
		(emitted.len(), emitted.iter())
	};
	quote! {
		static ALLOWED_VALUES: [::encde::UnknownSignInt; #num_enum_values] = [ #(#enum_values),* ];
		let discriminant: #enum_repr = ::encde::Decode::decode(reader)?;
		match discriminant {
			#(#sub_expressions),*
			__encde_unexpected => Err(::encde::Error::UnrecognizedEnumDiscriminant{
				enum_name: stringify!(#enum_name),
				expected: &ALLOWED_VALUES,
				actual: __encde_unexpected.into(),
			})
		}
	}
}
