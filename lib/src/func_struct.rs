use quote::quote;
use syn::{Error, Expr, ExprCall, ExprPath};

use crate::{str_to_cstr, TSResult};

pub fn handle(init: ExprCall) -> TSResult {
    let ExprCall {
        attrs, func, args, ..
    } = init;

    let Expr::Path(ExprPath {
        attrs: path_attrs,
        path,
        ..
    }) = *func
    else {
        return Err(Error::new_spanned(func, "expected path").to_compile_error());
    };

    let mut mapped = Vec::with_capacity(args.len());
    for i in args {
        mapped.push(str_to_cstr::convert_if_lit(i)?);
    }

    Ok(quote! {
        #(#attrs)*
        (
            #(#path_attrs)*
            #path
        )(
            #(#mapped,)*
        )
    })
}
