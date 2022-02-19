use proc_macro2::Span;
use quote::ToTokens;
use syn::{
	parenthesized,
	parse::{Parse, ParseStream, Result as ParseResult},
	punctuated::Punctuated,
	Attribute, Error as ParseError, LitInt, Token,
};

mod kw {
	use syn::custom_keyword;
	custom_keyword!(pad_before);
	custom_keyword!(pad_after);
	custom_keyword!(wire_tag);
	custom_keyword!(sync);
}

/// The type implementing this trait can be obtained by combining a sequence of attributes of type T
pub trait Collectible: Default {
	type Item;

	fn update(&mut self, item: &Self::Item);
	fn collect(attrs: &[Self::Item]) -> Self {
		let mut result = Self::default();
		for attr in attrs {
			result.update(attr);
		}
		result
	}
}

#[derive(Clone)]
pub enum VariantWireTag {
	Implicit,
	/// Synchronize the library's tag with Rust's tag
	Sync,
	Explicit(Box<syn::Expr>),
}
#[derive(Clone)]
pub struct VariantWireTagSpanned {
	pub tag: VariantWireTag,
	pub span: Span,
}
impl syn::spanned::Spanned for VariantWireTagSpanned {
	fn span(&self) -> Span {
		self.span
	}
}
impl std::ops::Deref for VariantWireTagSpanned {
	type Target = VariantWireTag;
	fn deref(&self) -> &Self::Target {
		&self.tag
	}
}

pub enum EnumAttribute {}
pub enum StructAttribute {}
pub enum VariantAttribute {
	WireTag(VariantWireTagSpanned),
}
pub enum FieldAttribute {
	PadBefore(usize),
	PadAfter(usize),
}

fn parse_int<T>(input: &ParseStream) -> ParseResult<T>
where
	T: std::str::FromStr,
	T::Err: std::fmt::Display,
{
	let literal = input.parse::<LitInt>()?;
	literal.base10_parse::<T>().map_err(|err| ParseError::new(literal.span(), err))
}

impl Parse for FieldAttribute {
	fn parse(input: ParseStream) -> ParseResult<Self> {
		let look = input.lookahead1();
		if look.peek(kw::pad_before) {
			input.parse::<kw::pad_before>()?;
			input.parse::<Token![=]>()?;
			let size: usize = parse_int(&input)?;
			Ok(FieldAttribute::PadBefore(size))
		} else if look.peek(kw::pad_after) {
			input.parse::<kw::pad_after>()?;
			input.parse::<Token![=]>()?;
			let size: usize = parse_int(&input)?;
			Ok(FieldAttribute::PadAfter(size))
		} else {
			Err(look.error())
		}
	}
}
impl Parse for VariantAttribute {
	fn parse(input: ParseStream) -> ParseResult<Self> {
		let span = input.span();
		let look = input.lookahead1();
		if look.peek(kw::wire_tag) {
			input.parse::<kw::wire_tag>()?;
			if input.peek(Token![=]) {
				input.parse::<Token![=]>()?;
				if input.peek(kw::sync) {
					input.parse::<kw::sync>()?;
					Ok(VariantAttribute::WireTag(VariantWireTagSpanned { tag: VariantWireTag::Sync, span }))
				} else {
					Ok(VariantAttribute::WireTag(VariantWireTagSpanned {
						tag: VariantWireTag::Explicit(input.parse()?),
						span,
					}))
				}
			} else if input.is_empty() {
				Ok(VariantAttribute::WireTag(VariantWireTagSpanned { tag: VariantWireTag::Implicit, span }))
			} else {
				Err(syn::Error::new(
					span,
					"Invalid wire_tag attribute.\nYou may have wanted `#[encde(wire_tag = 3)]` (explicit tag), `#[encde(wire_tag)]` (implicit tag), or `#[encde(wire_tag = sync)]` (use actual Rust discriminant as tag)",
				))
			}
		} else {
			Err(look.error())
		}
	}
}
impl Parse for StructAttribute {
	fn parse(input: ParseStream) -> ParseResult<Self> {
		let look = input.lookahead1();
		Err(look.error())
	}
}
impl Parse for EnumAttribute {
	fn parse(input: ParseStream) -> ParseResult<Self> {
		let look = input.lookahead1();
		Err(look.error())
	}
}

pub fn parse_crate_attributes<T>(attrs: &[Attribute]) -> ParseResult<T>
where
	T: Collectible,
	T::Item: Parse,
{
	struct RawAttrs<T>(Punctuated<T, Token![,]>);
	impl<T: Parse> Parse for RawAttrs<T> {
		fn parse(input: ParseStream) -> ParseResult<Self> {
			let content;
			parenthesized!(content in input);
			Ok(RawAttrs(content.parse_terminated(T::parse)?))
		}
	}

	let mut ret = T::default();
	for raw_attr in attrs {
		let path = raw_attr.path.to_token_stream().to_string();
		if path != "encde" {
			continue;
		}

		let parsed_attrs: RawAttrs<<T as Collectible>::Item> = syn::parse2(raw_attr.tokens.clone())?;
		for attr in parsed_attrs.0 {
			ret.update(&attr);
		}
	}
	Ok(ret)
}

struct ReprAttribute(String);
impl syn::parse::Parse for ReprAttribute {
	fn parse(buf: &syn::parse::ParseBuffer<'_>) -> syn::parse::Result<Self> {
		let ty: syn::parse::ParseBuffer;
		parenthesized!(ty in buf);
		Ok(Self(ty.parse::<syn::Type>()?.into_token_stream().to_string()))
	}
}

pub fn get_repr_attribute(attrs: &[Attribute]) -> ParseResult<Option<String>> {
	for raw_attr in attrs.iter().rev() {
		let path = raw_attr.path.to_token_stream().to_string();
		if path != "repr" {
			continue;
		}
		let ReprAttribute(ty) = syn::parse2(raw_attr.tokens.clone()).expect("Could not parse ReprAttribute");
		return Ok(Some(ty));
	}
	Ok(None)
}

#[derive(Default)]
pub struct FieldAttributes {
	pub pad_before: usize,
	pub pad_after: usize,
}
impl Collectible for FieldAttributes {
	type Item = FieldAttribute;
	fn update(&mut self, item: &Self::Item) {
		match item {
			FieldAttribute::PadBefore(amt) => {
				self.pad_before = *amt;
			}
			FieldAttribute::PadAfter(amt) => {
				self.pad_after = *amt;
			}
		}
	}
}

#[derive(Default)]
pub struct EnumAttributes {}
impl Collectible for EnumAttributes {
	type Item = EnumAttribute;
	fn update(&mut self, _item: &Self::Item) {
		// match item {}
	}
}

#[derive(Default)]
pub struct StructAttributes {}
impl Collectible for StructAttributes {
	type Item = StructAttribute;
	fn update(&mut self, _item: &Self::Item) {
		// match item {}
	}
}

#[derive(Default)]
pub struct VariantAttributes {
	pub wire_tag: Option<VariantWireTagSpanned>,
}
impl Collectible for VariantAttributes {
	type Item = VariantAttribute;
	fn update(&mut self, item: &Self::Item) {
		match item {
			Self::Item::WireTag(tag) => {
				self.wire_tag = Some(tag.clone());
			}
		}
	}
}

pub enum StructVariantAttribute {
	StructAttr(StructAttribute),
	VariantAttr(VariantAttribute),
}
impl From<StructAttribute> for StructVariantAttribute {
	fn from(attr: StructAttribute) -> Self {
		StructVariantAttribute::StructAttr(attr)
	}
}
impl From<VariantAttribute> for StructVariantAttribute {
	fn from(attr: VariantAttribute) -> Self {
		StructVariantAttribute::VariantAttr(attr)
	}
}
impl Parse for StructVariantAttribute {
	fn parse(input: ParseStream) -> ParseResult<Self> {
		match StructAttribute::parse(&input.fork()) {
			Ok(struct_attr) => Ok(struct_attr.into()),
			Err(mut struct_attr_err) => match VariantAttribute::parse(input) {
				Ok(variant_attr) => Ok(variant_attr.into()),
				Err(variant_attr_err) => Err({
					struct_attr_err.extend(variant_attr_err);
					struct_attr_err
				}),
			},
		}
	}
}
#[derive(Default)]
pub struct StructVariantAttributes {
	pub struct_attrs: StructAttributes,
	pub variant_attrs: VariantAttributes,
}
impl Collectible for StructVariantAttributes {
	type Item = StructVariantAttribute;
	fn update(&mut self, item: &Self::Item) {
		match item {
			StructVariantAttribute::StructAttr(ref attr) => self.struct_attrs.update(attr),
			StructVariantAttribute::VariantAttr(ref attr) => self.variant_attrs.update(attr),
		}
	}
}
