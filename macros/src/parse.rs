use syn::{
	parse::{
		Parse, ParseStream,
	},
	punctuated::Punctuated,
	token::Brace,
	Result,
	Attribute,
	Visibility,
	Ident,
	Path,
	GenericParam, WhereClause,
	braced, Token,
};

use crate::items::*;

pub struct Class {
	pub attrs: Vec<Attribute>,
	pub vis: Visibility,
	pub name: Ident,
	pub generics: Generics,
	pub inheritance: Option<Inheritance>,
	pub where_clause: Option<WhereClause>,
	pub items_brace: Brace,
	pub items: Items,
}

impl Parse for Class {
	fn parse(input: ParseStream) -> Result<Self> {
		let item_tokens;
		Ok(Self {
			attrs: input.call(Attribute::parse_outer)?,
			vis: input.parse()?,
			name: input.parse()?,
			generics: input.parse()?,
			inheritance: input.call(Inheritance::parse_opt)?,
			where_clause: input.parse()?,
			items_brace: braced!(item_tokens in input),
			items: item_tokens.parse()?,
		})
	}
}

#[derive(Default)]
pub struct Generics {
	pub params: Punctuated<GenericParam, Token![,]>,
}

impl Parse for Generics {
	fn parse(input: ParseStream) -> Result<Self> {
		if !input.peek(Token![<]) {
			return Ok(Self::default())
		}
		let _: Token![<] = input.parse()?;
		let mut params = Punctuated::new();
		loop {
			if input.peek(Token![>]) {
				break
			}
			let value = input.parse()?;
			params.push_value(value);
			if !input.peek(Token![,]) {
				break
			}
			let punct = input.parse()?;
			params.push_punct(punct);
		}
		let _: Token![>] = input.parse()?;

		Ok(Self {
			params,
		})
	}
}

pub struct Inheritance {
	pub base: Inherited,
	pub rest: Punctuated<Inherited, Token![,]>,
}

impl Inheritance {
	pub fn parse_opt(input: ParseStream) -> Result<Option<Self>> {
		if input.peek(Token![:]) {
			input.parse().map(Some)
		} else {
			Ok(None)
		}
	}
}

impl Parse for Inheritance {
	fn parse(input: ParseStream) -> Result<Self> {
		let _: Token![:] = input.parse()?;
		let base = input.parse()?;
		let comma: Option<Token![,]> = input.parse()?;
		if comma.is_none() {
			return Ok(Self {
				base,
				rest: Punctuated::new(),
			})
		}
		Ok(Self {
			base,
			rest: {
				let mut inherited = Punctuated::new();
				loop {
					if input.peek(Brace) || input.peek(Token![where]) {
						break
					}
					let value = input.parse()?;
					inherited.push_value(value);
					if !input.peek(Token![,]) {
						break
					}
					let punct = input.parse()?;
					inherited.push_punct(punct);
				}
				inherited
			},
		})
	}
}

pub struct Inherited {
	pub attrs: Vec<Attribute>,
	pub vis: Visibility,
	pub is_virtual: bool,
	pub path: Path,
}

impl Parse for Inherited {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attrs: input.call(Attribute::parse_outer)?,
			vis: input.parse()?,
			is_virtual: input.parse::<Option<Token![virtual]>>()?.is_some(),
			path: input.parse()?,
		})
	}
}
