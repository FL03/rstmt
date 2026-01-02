#![crate_name = "rstmt_macros"]
//! Various procedural macros for the `rstmt` ecosystem focused on streamling the composition 
//! of musical pieces, chords, notes, and more. 
extern crate proc_macro;

mod ast {
    pub use self::composition::*;

    pub mod composition;
}

mod impls {
    pub use self::impl_composition::impl_composition;

    pub mod impl_composition;
}

use crate::ast::CompositionAst;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// The [`binary_wrapper!`] macro generates implementations for the core binary operations
/// onto a generic wrapper type. It supports both tuple structs and structs with named fields.
///
/// ```rust
/// use rstmt_macros::composition;
/// 
/// composition! {}
/// ```
#[proc_macro]
pub fn composition(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as CompositionAst);
    let output = impls::impl_composition(ast);
    output.into()
}
