extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Token, parse::{ParseStream, Parse}, Result, LitInt, Expr};

#[allow(unused)]
struct FillInput {
    item: Expr,
    semi_token: Token![;],
    length: LitInt,
}

impl Parse for FillInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let item = input.parse()?;
        let semi_token = input.parse()?;
        let length = input.parse()?;
        Ok(FillInput { item, semi_token, length })
    }
}

#[proc_macro]
pub fn fill(input: TokenStream) -> TokenStream {
    let FillInput { item, length, .. } = parse_macro_input!(input as FillInput);
    
    let length = length.base10_parse().expect("Incorrect length");

    let mut inner_tokens = TokenStream2::new();
    for _ in 0..length {
        inner_tokens.extend(quote!{#item,})
    }

    let tokens = quote! {
        [#inner_tokens]
    };

    tokens.into()
}