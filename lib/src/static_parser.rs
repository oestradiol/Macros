use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Error, ItemStatic,
};

pub(super) struct Parser {
    pub(super) r#static: ItemStatic,
    pub(super) input: TokenStream,
}
impl Parse for Parser {
    fn parse(input: ParseStream<'_>) -> Result<Self, Error> {
        let r#static = input.parse()?;
        let input = input.parse()?;
        let parser = Self { r#static, input };
        Ok(parser)
    }
}
