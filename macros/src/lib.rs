use proc_macro::TokenStream;
use syn::parse_macro_input;

mod items;

mod expand;
mod parse;

/// Implementation detail of the actual `class!` macro that properly handles imports from the main crate.
#[proc_macro]
pub fn class_impl(input: TokenStream) -> TokenStream {
	let invocation: expand::Invocation = parse_macro_input!(input);
	invocation.expand().unwrap_or_else(move |e| e.into_compile_error()).into()
}
