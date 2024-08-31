#![no_std]
extern crate proc_macro;
extern crate alloc;

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use rand::Rng;
use syn::{parse_macro_input, Ident, LitStr};

lazy_static! {
    static ref KEY: u64 = {
        let mut rng = rand::thread_rng();
        rng.gen_range(1000..10000)
    };
}

#[proc_macro]
pub fn gen_key(_item: TokenStream) -> TokenStream {
    let key: u64 = *KEY;
    TokenStream::from(
        quote!(
            const KEY: u64 = #key; 
            )
        )
}

#[proc_macro]
pub fn gen_const(item: TokenStream) -> TokenStream {
    let parsed = (parse_macro_input!(item as LitStr)).value();
    let key = *KEY;
    let hashed = parsed.as_bytes().iter().fold(key, |key, a| {(key << 5).wrapping_add(key).wrapping_add(*a as u64)});
    let constname = alloc::format!("{}HASH", parsed.to_uppercase());
    let constname: Ident = Ident::new(&constname, Span::call_site());  
    TokenStream::from(
        quote!(
            const #constname: u64 = #hashed;
            )
        )
}

#[proc_macro]
pub fn gen_struct(item: TokenStream) -> TokenStream {
    let parsed = (parse_macro_input!(item as LitStr)).value();
    let key = *KEY;
    let hashed = parsed.as_bytes().iter().fold(key, |key, a| {(key << 5).wrapping_add(key).wrapping_add(*a as u64)});
    let constname = alloc::format!("{}HASH", parsed.to_uppercase());
    let constname: Ident = Ident::new(&constname, Span::call_site());  
    TokenStream::from(
        quote!(
            const #constname: apihasher::ApiHasher = apihasher::ApiHasher {
                hash: #hashed,
                key: #key
            };
            )
        )
}
