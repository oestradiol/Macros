use macros_lib::c_struct as r#impl;
use proc_macro::TokenStream;

#[proc_macro]
pub fn c_struct(input: TokenStream) -> TokenStream {
    let (Ok(input) | Err(input)) = r#impl(input.into()).map(|(mut s, i)| {
        s.extend(i);
        s
    });
    input.into()
}
