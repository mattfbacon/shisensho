use crate::attrs::{parse_crate_attributes, StructAttributes};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, DataStruct};

pub fn implement(attrs: Vec<Attribute>, data: DataStruct) -> TokenStream2 {
	let attrs: StructAttributes = parse_crate_attributes(&attrs).expect("Could not parse StructAttributes");
	super::common::implement_struct_body(attrs, data.fields, |ident| quote! { self.#ident })
}
