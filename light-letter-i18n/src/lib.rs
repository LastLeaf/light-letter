#![recursion_limit="128"]

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;
use syn::parse::*;
use syn::punctuated::Punctuated;

struct Translation {
    name: Ident,
    str_map: Punctuated<(LitStr, LitStr), Token![;]>,
}

impl Parse for Translation {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let content;
        braced!(content in input);
        let str_map = Punctuated::parse_terminated_with(&content, |input| {
            let src: LitStr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let trans: LitStr = input.parse()?;
            Ok((src, trans))
        })?;
        Ok(Self {
            name,
            str_map,
        })
    }
}

impl ToTokens for Translation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { name, str_map } = self;
        let list: Vec<_> = str_map.iter().map(|item| {
            let src = &item.0;
            let trans = &item.1;
            quote! {
                (#src) => { #trans };
                (#src, $($t:expr),*) => { (format!(#trans, $($t),*)) };
            }
        }).collect();
        tokens.append_all(quote! {
            #[macro_export]
            macro_rules! #name {
                #(#list)*
            }
        });
    }
}

#[proc_macro]
pub fn translation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Translation);
    let ret = quote! {
        #input
    };
    TokenStream::from(ret)
}
