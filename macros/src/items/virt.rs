use proc_macro2::TokenStream as TokenStream2;
use syn::{
	parse::{
		Parse, ParseStream,
	},
	punctuated::Punctuated,
	parenthesized,
	Attribute, Visibility,
	Pat, Type,
	Ident,
	ReturnType,
	Token,
	Result,
};
use quote::{
	ToTokens, quote,
};

pub struct Item {
	pub attrs: Vec<Attribute>,
	pub vis: Visibility,
	pub kind: Kind,
}

pub enum Kind {
	Fn(VirtFn),
	Use(super::Use),
}

pub struct VirtFn {
	pub name: Ident,
	pub inputs: Punctuated<VirtFnArg, Token![,]>,
	pub output: ReturnType,
}
impl Parse for VirtFn {
	fn parse(input: ParseStream) -> Result<Self> {
		let _: Token![fn] = input.parse()?;
		let name = input.parse()?;

		let params;
		let _ = parenthesized!(params in input);

		Ok(Self {
			name,
			inputs: params.parse_terminated(VirtFnArg::parse, Token![,])?,
			output: input.parse()?,
		})
	}
}
impl ToTokens for VirtFn {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let Self {
			name, inputs, output,
		} = self;
		let inputs = inputs.iter().map(move |i| {
			let VirtFnArg { attrs, kind } = i;
			match kind {
				VirtFnArgKind::Variadic(v) => quote! { #(#attrs)* #v },
				VirtFnArgKind::Typed { pat, ty } => quote! { #(#attrs)* #pat: #ty },
			}
		});
		quote! {
			fn #name(#(#inputs,)*) #output
		}.to_tokens(tokens)
	}
}

pub struct VirtFnArg {
	pub attrs: Vec<Attribute>,
	pub kind: VirtFnArgKind,
}
impl Parse for VirtFnArg {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			attrs: input.call(Attribute::parse_outer)?,
			kind: input.parse()?,
		})
	}
}
impl ToTokens for VirtFnArg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for attr in &self.attrs {
			attr.to_tokens(tokens);
		}
		self.kind.to_tokens(tokens)
	}
}

pub enum VirtFnArgKind {
	Variadic(Token![...]),
	Typed {
		pat: Box<Pat>,
		ty: Box<Type>,
	},
}
impl Parse for VirtFnArgKind {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Token![...]) {
			input.parse().map(Self::Variadic)
		} else {
			let pat = Box::new(Pat::parse_single(input)?);
			let _: Token![:] = input.parse()?;
			let ty = input.parse()?;
			Ok(Self::Typed {
				pat,
				ty,
			})
		}
	}
}
impl ToTokens for VirtFnArgKind {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		match self {
			Self::Variadic(d) => d.to_tokens(tokens),
			Self::Typed { pat, ty } => {
				quote! { #pat: #ty }.to_tokens(tokens)
			}
		}
	}
}
