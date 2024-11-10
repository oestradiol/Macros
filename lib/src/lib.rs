#![feature(proc_macro_expand)]
extern crate proc_macro;

mod func_struct;
pub mod static_parser;
mod str_to_cstr;
mod struct_literal;

// Re-exports
pub use proc_macro2;
pub use quote;
pub use syn;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote as qt;
use static_parser::Parser;
use syn::{Error, Expr, ItemStatic};

type TSResult = Result<TokenStream2, TokenStream2>;

pub fn c_struct(input: TokenStream2) -> Result<(TokenStream2, TokenStream2), TokenStream2> {
    let Parser {
        r#static:
            ItemStatic {
                attrs,
                vis,
                static_token,
                mutability,
                ident,
                colon_token,
                ty,
                eq_token,
                expr,
                semi_token,
            },
        input,
    } = syn::parse2(input).map_err(|e| e.to_compile_error())?;

    let ts = match *expr {
        Expr::Struct(init) => struct_literal::handle(init),
        Expr::Call(init) => func_struct::handle(init),
        _ => Err(Error::new_spanned(expr, "expected a (non-unit) struct").to_compile_error()),
    }
    .map(|expr| {
        qt! {
            #(#attrs)*
            #vis #static_token #mutability #ident #colon_token #ty #eq_token
                #expr #semi_token
        }
    })?;

    let struct_input = (ts, input);
    Ok(struct_input)
}
