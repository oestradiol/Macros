use std::ffi::CString;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Error, Expr, ExprLit, Lit, LitCStr, LitStr};

use super::TSResult;

pub fn convert(lit: LitStr) -> TSResult {
    let cstring = match CString::new(lit.value()) {
        Ok(cstring) => cstring,
        Err(e) => {
            return Err(Error::new_spanned(lit, e.to_string()).to_compile_error());
        }
    };

    let lit = LitCStr::new(&cstring, lit.span()).into_token_stream();
    Ok(lit)
}

pub fn convert_if_lit(i: Expr) -> TSResult {
    let res = match i {
        Expr::Macro(ref lit) => {
            let ts = lit.to_token_stream();
            let ts = match TokenStream::expand_expr(&ts.into()) {
                Err(e) => return Err(Error::new_spanned(i, e).to_compile_error()),
                Ok(res) => res,
            };

            match syn::parse(ts) {
                Err(e) => return Err(Error::new_spanned(i, e).to_compile_error()),
                Ok(Expr::Lit(ExprLit {
                    attrs,
                    lit: Lit::Str(lit),
                })) => {
                    let lit = convert(lit)?;
                    quote! { #(#attrs)* #lit }
                }
                _ => i.into_token_stream(),
            }
        }
        Expr::Lit(ExprLit {
            attrs,
            lit: Lit::Str(lit),
        }) => {
            let lit = convert(lit)?;
            quote! { #(#attrs)* #lit }
        }
        _ => i.into_token_stream(),
    };
    Ok(res)
}
