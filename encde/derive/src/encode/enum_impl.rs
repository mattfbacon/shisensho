use crate::attrs::{get_repr_attribute, parse_crate_attributes, EnumAttributes, StructAttributes, StructVariantAttributes, VariantAttributes};
use crate::tracker::DiscriminantTracker;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, DataEnum, Fields, FieldsNamed, FieldsUnnamed};

pub fn implement(attrs: Vec<Attribute>, data: DataEnum) -> TokenStream2 {
	let EnumAttributes {} = parse_crate_attributes(&attrs).expect("Could not parse EnumAttributes");
	let enum_repr: String = get_repr_attribute(&attrs).expect("Could not parse repr attribute").unwrap_or_else(|| "u32".to_owned());
	let mut tracker = DiscriminantTracker::new_from_type(&enum_repr);
	let enum_repr = Ident::new(&enum_repr, Span::call_site());
	let mut has_at_least_one_variant: bool = false;
	let sub_expressions = data.variants.into_iter().map(|variant| {
		has_at_least_one_variant = true;
		let ident = &variant.ident;
		let ident = quote! { Self::#ident };
		let discriminant: TokenStream2;
		let matcher = match &variant.fields {
			Fields::Named(FieldsNamed { named: ref fields, .. }) => {
				let StructVariantAttributes {
					variant_attrs: VariantAttributes { wire_tag },
					struct_attrs: StructAttributes {},
				} = parse_crate_attributes(&variant.attrs).expect("Could not parse StructVariantAttributes");
				discriminant = tracker.next_variant(&wire_tag, &variant.discriminant.map(|(_, expr)| expr)).unwrap_or_else(syn::Error::into_compile_error);
				let fields = fields.iter().map(|field| {
					let field_var: Ident = format_ident!("{}{}", crate::FIELD_PREFIX, field.ident.as_ref().expect("Named struct field does not have a name (???)"));
					let field_name: &Ident = field.ident.as_ref().expect("Named struct field does not have a name (???)");
					quote! { #field_name: ref #field_var }
				});
				quote! { { #(#fields),* } }
			}
			Fields::Unnamed(FieldsUnnamed { unnamed: ref fields, .. }) => {
				let VariantAttributes { wire_tag } = parse_crate_attributes(&variant.attrs).expect("Could not parse VariantAttributes");
				discriminant = tracker.next_variant(&wire_tag, &variant.discriminant.map(|(_, expr)| expr)).unwrap_or_else(syn::Error::into_compile_error);
				let fields = fields.iter().enumerate().map(|(idx, _field)| {
					let field_var = format_ident!("{}{}", crate::FIELD_PREFIX, idx);
					quote! { #field_var }
				});
				quote! { (#(ref #fields),*) }
			}
			Fields::Unit => {
				let VariantAttributes { wire_tag } = parse_crate_attributes(&variant.attrs).expect("Could not parse VariantAttributes");
				discriminant = tracker.next_variant(&wire_tag, &variant.discriminant.map(|(_, expr)| expr)).unwrap_or_else(syn::Error::into_compile_error);
				quote! {}
			}
		};
		let StructVariantAttributes {
			variant_attrs: VariantAttributes { wire_tag: _ },
			struct_attrs,
		} = parse_crate_attributes(&variant.attrs).expect("Could not parse StructVariantAttributes");
		let encode_tag = quote! {
			::encde::Encode::encode(&((#discriminant) as #enum_repr), writer)?;
		};
		let encode_members = super::common::implement_struct_body(struct_attrs, variant.fields, |ident| {
			let ident = format_ident!("{}{}", crate::FIELD_PREFIX, ident.to_string());
			quote! { #ident }
		});
		quote! {
			#ident #matcher => { #encode_tag #encode_members }
		}
	});
	let ret = quote! {
		match self {
			#(#sub_expressions),*
		}
	};
	if has_at_least_one_variant {
		ret
	} else {
		// don't get weird errors if there are no enum variants
		quote! {}
	}
}
