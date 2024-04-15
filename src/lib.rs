extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Token, parse::{ParseStream, Parse}, Result, LitInt, Expr};

#[allow(unused)]
enum Length {
    Literal(LitInt),
    Constant(Ident),
}

impl Parse for Length {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitInt) {
            let lit_int: LitInt = input.parse()?;
            Ok(Length::Literal(lit_int))
        } else {
            let ident: Ident = input.parse()?;
            Ok(Length::Constant(ident))
        }
    }
}

#[allow(unused)]
struct FillInput {
    item: Expr,
    semi_token: Token![;],
    length: Length,
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

    match length {
        Length::Literal(lit) => {
            let length = lit.base10_parse().expect("Incorrect length");

            let mut inner_tokens = TokenStream2::new();
            for _ in 0..length {
                inner_tokens.extend(quote!{#item,})
            }

            let tokens = quote! {
                [#inner_tokens]
            };

            tokens.into()
        },
        Length::Constant(ident) => {
            let tokens = quote! {
                {
                    let mut array = [#item; #ident];
                    for i in 1..#ident {
                        array[i] = #item;
                    }

                    array
                }
            };

            tokens.into()
        },
    }
}