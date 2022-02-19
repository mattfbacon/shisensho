use proc_macro::TokenStream as TokenStream1;
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod decode;
mod encode;
pub(crate) mod tracker;

#[proc_macro_derive(Encode, attributes(encde))]
pub fn encode_derive_wrapper(input: TokenStream1) -> TokenStream1 {
	let input = parse_macro_input!(input as DeriveInput);
	let expanded = encode::derive(input);
	expanded.into()
}

#[proc_macro_derive(Decode, attributes(encde))]
pub fn decode_derive_wrapper(input: TokenStream1) -> TokenStream1 {
	let input = parse_macro_input!(input as DeriveInput);
	let expanded = decode::derive(input);
	expanded.into()
}

pub(crate) const FIELD_PREFIX: &str = "__encde_field_name_prefix__";
