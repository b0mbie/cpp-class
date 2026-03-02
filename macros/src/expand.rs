use proc_macro2::{
	TokenStream as TokenStream2,
	Span,
};
use quote::{
	ToTokens,
	quote, format_ident,
};
use std::fmt::Write;
use syn::{
	parse::{
		Parse, ParseStream,
	},
	punctuated::Punctuated,
	spanned::Spanned,
	GenericParam, WhereClause,
	TypeParam,
	Token,
	Ident, Path,
	Result,
};

use crate::items::*;
use crate::parse::*;

macro_rules! bail {
	($span:expr, $($tt:tt)*) => {
		return ::syn::Result::Err(::syn::Error::new($span, ::core::format_args!($($tt)*)))
	};
}

pub struct Invocation {
	crate_path: Path,
	class: Class,
	generic_args: TokenStream2,
}

impl Parse for Invocation {
	fn parse(input: ParseStream) -> Result<Self> {
		let crate_path = input.parse()?;
		let class: Class = input.parse()?;
		let generic_args =
			class.generics.params.iter()
			.map(move |param| match param {
				GenericParam::Lifetime(lt) => lt.to_token_stream(),
				GenericParam::Type(TypeParam {
					attrs, ident, ..
				}) => quote! {#(#attrs)* #ident},
				GenericParam::Const(kst) => kst.to_token_stream(),
			});
		let generic_args = quote! { <#(#generic_args,)*> };
		Ok(Self {
			crate_path, class,
			generic_args,
		})
	}
}

fn copy_impl(
	name: &Ident,
	params: Option<&Punctuated<GenericParam, Token![,]>>,
	args: Option<&TokenStream2>,
	where_clause: Option<&WhereClause>,
) -> TokenStream2 {
	quote! {
		impl<#params> Copy for #name #args #where_clause {}
		impl<#params> Clone for #name #args #where_clause {
			fn clone(&self) -> Self { *self }
		}
	}
}

impl Invocation {
	/// Returns the kind of C++ class this is,
	/// for figuring out inheritance.
	fn class_kind(&self) -> Result<ClassKind<'_>> {
		let Items { ref virt, ref non_virt } = self.class.items;
		let is_virtual = !virt.is_empty();
		Ok(if let Some(Inheritance { ref base, ref rest }) = self.class.inheritance {
			if base.is_virtual {
				ClassKind::VirtOrFromVirt {
					base: Some(base),
					rest: Some(rest),
					non_virt, virt,
				}
			} else if is_virtual {
				ClassKind::VirtOrFromVirt {
					base: None,
					rest: Some(rest),
					non_virt, virt,
				}
			} else {
				ClassKind::NonVirt {
					base: Some(base),
					rest: Some(rest),
					non_virt,
				}
			}
		} else if is_virtual {
			ClassKind::VirtOrFromVirt {
				base: None,
				rest: None,
				non_virt, virt,
			}
		} else {
			ClassKind::NonVirt {
				base: None,
				rest: None,
				non_virt,
			}
		})
	}

	pub fn expand(&self) -> Result<TokenStream2> {
		let result = self.expand_impl();
		if let Some(ref inh) = self.class.inheritance {
			let definition = result?;
			let Self {
				crate_path,
				class: Class {
					name,
					generics: Generics { params },
					where_clause,
					..
				},
				generic_args,
			} = self;
			let base_ty = &inh.base.path;
			Ok(quote! {
				unsafe impl<#params> #crate_path::Inherits<#base_ty> for #name #generic_args #where_clause {}
				#definition
			})
		} else {
			result
		}
	}

	fn expand_impl(&self) -> Result<TokenStream2> {
		let Self {
			crate_path,
			class: Class {
				attrs, vis, name,
				generics: Generics { params },
				where_clause,
				items_brace,
				..
			},
			generic_args,
		} = self;
		Ok(match self.class_kind()? {
			ClassKind::NonVirt { base, rest, non_virt: fields } => {
				let inherited_fields = base.into_iter()
					.chain(rest.iter().flat_map(move |i| i.iter()))
					.map(move |i| i.field(crate_path));
				quote! {
					#[repr(C)]
					#(#attrs)*
					#vis struct #name<#params> #where_clause {
						#(#inherited_fields,)*
						#(#fields,)*
					}

					unsafe impl<#params> #crate_path::NonVirtual for #name #generic_args #where_clause {
						type This = Self;
					}
				}
			}
			ClassKind::VirtOrFromVirt { base, rest, non_virt, virt } => {
				let name_vt = format_ident!("{name}Vt");
				let name_data = format_ident!("{name}Data");

				let where_clause = where_clause.as_ref();

				let inherently_virtual = !virt.is_empty();

				let vtable_definition;
				let data_fields_first;
				{
					let own_vtable_params = inherently_virtual.then_some(params);
					let own_vtable_where = where_clause.filter(move |_| inherently_virtual);
					let vtable_fields = virt.iter()
						.map(move |item| {
							let virt::Item { attrs, vis, kind } = item;
							match kind {
								virt::Kind::Fn(virt::VirtFn {
									name: virt_fn_name, inputs, output,
								}) => {
									quote! {
										#(#attrs)*
										#vis #virt_fn_name: #crate_path::virtual_fn! {
											fn(#crate_path::This<#name #generic_args>, #inputs) #output
										},
									}
								}
								virt::Kind::Use(Use { what, field }) => {
									quote! {
										#(#attrs)*
										#vis #field: #crate_path::Usage<#what>,
									}
								}
							}
						})
						;

					let vtable_copy_impl = copy_impl(&name_vt, Some(params), Some(generic_args), where_clause);
					if let Some(base) = base {
						// Don't allow base class inheritance to possibly be gated by attributes,
						// as the base class is the only one integrated into the VTable.
						// TODO: Maybe allow `#[doc = ...]`?
						if let Some(first) = base.attrs.first() {
							bail!(
								first.span(),
								"\
base class inheritance cannot have attributes
this restriction is a current limitation of the macro implementation and may be lifted in the future",
							)
						}

						let base_field = base.field_ident();
						let Inherited { vis: base_vis, path: base_ty, .. } = base;

						vtable_definition = quote! {
							#[repr(C)]
							#vis struct #name_vt<#params> #where_clause {
								#base_vis #base_field: <#base_ty as #crate_path::Virtual>::VTable,
								#(#vtable_fields)*
							}
							#vtable_copy_impl
						};
						data_fields_first = Some(quote! {
							#base_vis #base_field: <#base_ty as #crate_path::Virtual>::Data,
						});
					} else {
						if !inherently_virtual {
							bail!(
								items_brace.span.join(),
								"internal error: non-deriving virtual class has no virtual functions of its own, but is considered virtual"
							)
						}
						vtable_definition = quote! {
							#[repr(C)]
							#vis struct #name_vt<#own_vtable_params> #own_vtable_where {
								#(#vtable_fields)*
							}
							#vtable_copy_impl
						};
						data_fields_first = None;
					}
				};
				let data_fields_rest = rest.iter().flat_map(move |i| i.iter().map(move |i| i.field(crate_path)));

				let data_has_generics = !non_virt.is_empty() || data_fields_first.is_some();
				let data_params = data_has_generics.then_some(params);
				let data_args = data_has_generics.then_some(generic_args);
				let data_clause = data_has_generics.then_some(where_clause);

				quote! {
					#[repr(C)]
					#(#attrs)*
					#vis struct #name<#params> #where_clause {
						pub vtable: #crate_path::VTablePtr<#name_vt #generic_args>,
						pub data: #name_data #data_args,
						
					}
					unsafe impl<#params>
						#crate_path::Virtual
						for #name #generic_args
					#where_clause
					{
						type This = Self;
						type VTable = #name_vt #generic_args;
						type Data = #name_data #data_args;
					}

					#vtable_definition

					#[repr(C)]
					#vis struct #name_data<#data_params> #data_clause {
						#data_fields_first
						#(#data_fields_rest,)*
						#(#non_virt,)*
					}
				}
			}
		})
	}
}

enum ClassKind<'a> {
	/// Non-virtual class,
	/// with optional inheritance from an optional non-virtual base.
	/// Is a `struct Name`.
	NonVirt {
		base: Option<&'a Inherited>,
		rest: Option<&'a Punctuated<Inherited, Token![,]>>,
		non_virt: &'a [non_virt::Item],
	},
	/// Any class,
	/// with inheritance from an optional virtual base.
	/// Is a `struct Name`, `struct NameVt` and `struct NameData`.
	VirtOrFromVirt {
		base: Option<&'a Inherited>,
		rest: Option<&'a Punctuated<Inherited, Token![,]>>,
		non_virt: &'a [non_virt::Item],
		virt: &'a [virt::Item],
	},
}

impl Inherited {
	pub fn field_ident(&self) -> Ident {
		let mut s = String::from("inherited");
		self.path.write_ident(&mut s);
		Ident::new(&s, Span::mixed_site())
	}

	pub fn field(&self, crate_path: &Path) -> TokenStream2 {
		let Self { attrs, vis, path, is_virtual } = self;
		let class_trait = {
			let s = if *is_virtual { "Virtual" } else { "NonVirtual" };
			Ident::new(s, Span::call_site())
		};
		let ident = self.field_ident();
		quote! {
			#(#attrs)*
			#vis #ident: <#path as #crate_path::#class_trait>::This
		}
	}
}

trait PathExt {
	fn write_ident(&self, ident_str: &mut String);
}

impl PathExt for Path {
	fn write_ident(&self, ident_str: &mut String) {
		macro_rules! append_char {
			($c:expr $(, $on_uppercase:expr)?) => {{
				let c: char = $c;
				if c.is_uppercase() {
					$($on_uppercase();)?
					for c in c.to_lowercase() {
						ident_str.push(c);
					}
				} else {
					ident_str.push(c);
				}
			}};
		}

		let mut segment_ident = String::new();
		for segment in &self.segments {
			segment_ident.clear();
			let _ = write!(segment_ident, "{}", segment.ident);
			ident_str.push('_');

			let mut chars = segment_ident.chars();
			if let Some(first_char) = chars.next() {
				append_char!(first_char);
				for c in chars {
					append_char!(c, || ident_str.push('_'));
				}
			}
		}
	}
}
