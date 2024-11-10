use quote::quote;
use syn::{ExprStruct, FieldValue};

use crate::{str_to_cstr, TSResult};

pub fn handle(init: ExprStruct) -> TSResult {
    let ExprStruct {
        attrs,
        path,
        fields,
        rest,
        dot2_token,
        ..
    } = init;

    let mut mapped = Vec::with_capacity(fields.len());
    for i in fields {
        let FieldValue {
            attrs,
            member,
            colon_token,
            expr,
        } = i;

        let lit = str_to_cstr::convert_if_lit(expr)?;
        mapped.push(quote! {
            #(#attrs)*
            #member #colon_token #lit
        });
    }

    Ok(quote! {
        #(#attrs)*
        #path {
            #(#mapped,)*
            #dot2_token #rest
        }
    })
}
