use proc_macro2::TokenStream as TokenStream2;
use syn::{
	parse::{
		Parse, ParseStream,
	},
	Result,
	Attribute, Visibility,
	Token,
	Ident,
	GenericArgument,
};
use quote::{
	ToTokens, quote,
};

pub mod non_virt;
pub mod virt;

pub struct Items {
	pub virt: Vec<virt::Item>,
	pub non_virt: Vec<non_virt::Item>,
}

impl Parse for Items {
	fn parse(input: ParseStream) -> Result<Self> {
		struct Item {
			pub attrs: Vec<Attribute>,
			pub vis: Visibility,
			pub kind: ItemKind,
		}
		impl Parse for Item {
			fn parse(input: ParseStream) -> Result<Self> {
				let attrs = input.call(Attribute::parse_outer)?;
				let vis = input.parse()?;
				let kind = input.parse()?;
				let _: Token![;] = input.parse()?;
				Ok(Self {
					attrs,
					vis,
					kind,
				})
			}
		}

		enum ItemKind {
			VirtFn(virt::VirtFn),
			NonVirtField(non_virt::Field),
			VirtUse(Use),
			NonVirtUse(Use),
		}

		impl ItemKind {
			fn parse_virtual(input: ParseStream) -> Result<Self> {
				let _: Token![virtual] = input.parse()?;
				let lookahead = input.lookahead1();
				if lookahead.peek(Token![use]) {
					let _: Token![use] = input.parse()?;
					Ok(Self::VirtUse(input.parse()?))
				} else {
					input.parse().map(Self::VirtFn)
				}
			}
		}

		impl Parse for ItemKind {
			fn parse(input: ParseStream) -> Result<Self> {
				let lookahead = input.lookahead1();
				if lookahead.peek(Token![virtual]) {
					Self::parse_virtual(input)
				} else if lookahead.peek(Token![use]) {
					let _: Token![use] = input.parse()?;
					input.parse().map(Self::NonVirtUse)
				} else if lookahead.peek(Ident) {
					input.parse().map(Self::NonVirtField)
				} else {
					Err(
						lookahead.error()
					)
				}
			}
		}

		let mut virt = Vec::new();
		let mut non_virt = Vec::new();
		while !input.is_empty() {
			let Item { attrs, vis, kind, .. } = input.parse()?;
			match kind {
				ItemKind::VirtFn(inner) => {
					virt.push(virt::Item {
						attrs, vis,
						kind: virt::Kind::Fn(inner),
					})
				}
				ItemKind::NonVirtField(inner) => {
					non_virt.push(non_virt::Item {
						attrs, vis,
						kind: non_virt::Kind::Field(inner),
					})
				}
				ItemKind::VirtUse(inner) => {
					virt.push(virt::Item {
						attrs, vis,
						kind: virt::Kind::Use(inner),
					});
				}
				ItemKind::NonVirtUse(inner) => {
					non_virt.push(non_virt::Item {
						attrs, vis,
						kind: non_virt::Kind::Use(inner),
					});
				}
			}
		}
		Ok(Self {
			virt,
			non_virt,
		})
	}
}

pub struct Use {
	pub what: Box<GenericArgument>,
	pub field: Ident,
}
impl Parse for Use {
	fn parse(input: ParseStream) -> Result<Self> {
		let what = input.parse()?;
		let _: Token![as] = input.parse()?;
		let field = input.parse()?;
		Ok(Self {
			what, field,
		})
	}
}
impl ToTokens for Use {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let Self { what: ty, field } = self;
		quote! {
			#field: ::core::marker::PhantomData<#ty>
		}.to_tokens(tokens)
	}
}
