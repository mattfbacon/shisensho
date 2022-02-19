use crate::attrs::{VariantWireTag, VariantWireTagSpanned};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::Token;

fn int_type_is_signed(ty: &str) -> bool {
	match ty.chars().next().expect("Int type is empty") {
		'u' => false,
		'i' => true,
		_ => panic!("Unknown enum repr type {}", ty),
	}
}

#[derive(Debug, Eq, Clone, Copy)]
pub enum UnknownSignInt {
	Signed(i64),
	Unsigned(u64),
}
impl PartialEq for UnknownSignInt {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(&Self::Signed(l), &Self::Signed(r)) => l == r,
			(&Self::Unsigned(l), &Self::Unsigned(r)) => l == r,
			(&Self::Unsigned(_), &Self::Signed(_)) => other.eq(self),
			(&Self::Signed(l), &Self::Unsigned(r)) => u64::try_from(l).map(|l| l == r).unwrap_or(false),
		}
	}
}
impl std::hash::Hash for UnknownSignInt {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Signed(x) => x.hash(state),
			Self::Unsigned(x) => x.hash(state),
		}
	}
}
impl std::fmt::Display for UnknownSignInt {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			UnknownSignInt::Signed(v) => v.fmt(formatter),
			UnknownSignInt::Unsigned(v) => v.fmt(formatter),
		}
	}
}
impl From<i64> for UnknownSignInt {
	fn from(t: i64) -> Self {
		Self::Signed(t)
	}
}
impl From<u64> for UnknownSignInt {
	fn from(t: u64) -> Self {
		Self::Unsigned(t)
	}
}
impl std::ops::AddAssign<u32> for UnknownSignInt {
	fn add_assign(&mut self, rhs: u32) {
		match self {
			Self::Signed(ref mut x) => {
				*x += rhs as i64;
			}
			Self::Unsigned(ref mut x) => {
				*x += rhs as u64;
			}
		}
	}
}
impl quote::ToTokens for UnknownSignInt {
	fn to_tokens(&self, stream: &mut TokenStream2) {
		match self {
			Self::Signed(x) => stream.extend(quote! { ::encde::UnknownSignInt::Signed(#x) }),
			Self::Unsigned(x) => stream.extend(quote! { ::encde::UnknownSignInt::Unsigned(#x) }),
		}
	}
}
impl syn::parse::Parse for UnknownSignInt {
	fn parse(stream: syn::parse::ParseStream) -> syn::parse::Result<Self> {
		if stream.peek(Token![-]) {
			stream.parse::<Token![-]>()?;
			Ok(Self::Signed(-stream.parse::<syn::LitInt>()?.base10_parse()?))
		} else {
			Ok(Self::Unsigned(stream.parse::<syn::LitInt>()?.base10_parse()?))
		}
	}
}
impl FromStr for UnknownSignInt {
	type Err = <i64 as FromStr>::Err;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some(s) = s.strip_prefix('-') {
			Ok(Self::Signed(s.trim().parse()?))
		} else {
			Ok(Self::Unsigned(s.trim().parse()?))
		}
	}
}

pub struct LastVariant {
	index: UnknownSignInt,
}
impl LastVariant {
	pub fn new_from_type(ty: &str) -> Self {
		Self::new(int_type_is_signed(ty))
	}
	pub fn new(signed: bool) -> Self {
		if signed {
			Self::new_signed(0)
		} else {
			Self::new_unsigned(0)
		}
	}
	pub fn new_signed(index: i64) -> Self {
		Self { index: UnknownSignInt::from(index) }
	}
	pub fn new_unsigned(index: u64) -> Self {
		Self { index: UnknownSignInt::from(index) }
	}

	pub fn current_expression(&self) -> TokenStream2 {
		TokenStream2::from_str(&format!("{}", self.index)).expect("Could not parse internally-generated integer (??)")
	}
	pub fn next_explicit(&mut self, expr: &str) -> TokenStream2 {
		fn parse<T>(x: &str) -> T
		where
			T: std::str::FromStr,
			T::Err: std::fmt::Display,
		{
			syn::LitInt::new(x.trim(), proc_macro2::Span::call_site()).base10_parse::<T>().expect("Invalid enum discriminant")
		}
		match self.index {
			UnknownSignInt::Signed(ref mut index) => {
				if let Some(expr) = expr.strip_prefix('-') {
					*index = -parse::<i64>(expr);
				} else {
					*index = parse(expr);
				}
			}
			UnknownSignInt::Unsigned(ref mut index) => {
				*index = parse(expr);
			}
		}
		// here we must post-increment the index since otherwise the same discriminant will be repeated twice for an implicit after an explicit
		let ret = self.current_expression();
		self.index += 1;
		ret
	}
	pub fn next_implicit(&mut self) -> TokenStream2 {
		// the struct starts with zero, so the first implicit should also be zero
		// this entails some modifications in next_explicit though
		let ret = self.current_expression();
		self.index += 1;
		ret
	}
}

pub struct DiscriminantTracker {
	rust_last_variant: LastVariant,
	// The library discriminant, which overrides the Rust discriminant if specified via an attribute
	encde_last_variant: LastVariant,
	emitted_values: HashSet<UnknownSignInt>,
}
impl DiscriminantTracker {
	pub fn new_from_type(ty: &str) -> Self {
		Self {
			rust_last_variant: LastVariant::new_from_type(ty),
			encde_last_variant: LastVariant::new_from_type(ty),
			emitted_values: Default::default(),
		}
	}
	fn _next_variant(&mut self, wire_tag: &Option<VariantWireTagSpanned>, rust_discriminant: &Option<syn::Expr>) -> TokenStream2 {
		let ret_if_no_wire_tag = match rust_discriminant {
			Some(explicit) => self.rust_last_variant.next_explicit(&explicit.to_token_stream().to_string()),
			None => self.rust_last_variant.next_implicit(),
		};
		match wire_tag {
			None => ret_if_no_wire_tag,
			Some(VariantWireTagSpanned { tag: wire_tag, .. }) => match wire_tag {
				VariantWireTag::Implicit => self.encde_last_variant.next_implicit(),
				VariantWireTag::Explicit(explicit) => self.encde_last_variant.next_explicit(&explicit.to_token_stream().to_string()),
				VariantWireTag::Sync => self.encde_last_variant.next_explicit(&ret_if_no_wire_tag.to_token_stream().to_string()),
			},
		}
	}
	pub fn next_variant(&mut self, wire_tag: &Option<VariantWireTagSpanned>, rust_discriminant: &Option<syn::Expr>) -> Result<TokenStream2, syn::Error> {
		let span = wire_tag
			.as_ref()
			.map(VariantWireTagSpanned::span)
			.or_else(|| rust_discriminant.as_ref().map(|e| e.span()))
			.unwrap_or_else(proc_macro2::Span::call_site);
		let ret = self._next_variant(wire_tag, rust_discriminant);
		let actual_value = syn::parse2::<UnknownSignInt>(ret.clone()).expect("Could not parse internally-generated integer (??)");
		if self.emitted_values.insert(actual_value) {
			Ok(ret)
		} else {
			Err(syn::Error::new(
				span,
				"This crate does not support multiple enum variants with the same discriminant. Maybe try a union or enum as the member of the enum variant?",
			))
		}
	}
	pub fn emitted(&self) -> &HashSet<UnknownSignInt> {
		&self.emitted_values
	}
}
