/*
    Appellation: composition <module>
    Created At: 2026.01.01:23:24:37
    Contrib: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::token::Impl;
use syn::{AngleBracketedGenericArguments, Ident, Token, WhereClause, braced};

/// The abstract syntax tree for the `binary_wrapper` macro input;
/// e.g. `impl A { Add.add, Sub.sub }` or `impl B.field { Add.add, Sub.sub }`
pub struct CompositionAst {
    pub scale: Ident,
}

impl Parse for CompositionAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // parse the `scale` keyword
        let scale = input.parse::<Ident>()?;
        
        Ok(Self { scale })
    }
}
