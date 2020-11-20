//! A trivial Rust macro to derive the Display trait for any type with the Debug trait
//!
//! This is a very lightweight crate: check the README for more info



extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Display)]
pub fn display_derive(input: TokenStream) -> TokenStream {

    let ast = syn::parse(input).expect("TokenStream could not be parsed");
    impl_display_derive(&ast)
}


fn impl_display_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {

	    impl Display for #name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{:?}", self)
			}
		}
    };
    gen.into()
}