use proc_macro2::TokenStream as TokenStream2;
use syn::{
	parse::{
		Parse, ParseStream,
	},
	Result,
	Attribute, Visibility,
	Ident, Type,
	Token,
};
use quote::{
	ToTokens, quote,
};

pub struct Item {
	pub attrs: Vec<Attribute>,
	pub vis: Visibility,
	pub kind: Kind,
}
impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for attr in &self.attrs {
			attr.to_tokens(tokens);
		}
		self.vis.to_tokens(tokens);
		match &self.kind {
			Kind::Field(f) => f.to_tokens(tokens),
			Kind::Use(u) => u.to_tokens(tokens),
		}
	}
}

pub enum Kind {
	Field(Field),
	Use(super::Use),
}

pub struct Field {
	pub name: Ident,
	pub ty: Box<Type>,
}
impl Parse for Field {
	fn parse(input: ParseStream) -> Result<Self> {
		let name = input.parse()?;
		let _: Token![:] = input.parse()?;
		let ty = input.parse()?;
		Ok(Self {
			name, ty,
		})
	}
}
impl ToTokens for Field {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let Self { name, ty } = self;
		quote! {
			#name: #ty
		}.to_tokens(tokens)
	}
}
