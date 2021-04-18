#[macro_use]
extern crate quote;
extern crate syn;

extern crate macros;

mod derive_insn;

use proc_macro::TokenStream;

#[proc_macro_derive(Instruction, attributes(match_code, mask, format))]
pub fn instruction(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    match derive_insn::expand(&ast) {
        Ok(s) => s.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
