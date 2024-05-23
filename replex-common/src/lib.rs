extern crate proc_macro;

mod enums;
mod structs;

use proc_macro::TokenStream;

#[proc_macro]
pub fn enum_imports(input: TokenStream) -> TokenStream {
    enums::imports(input)
}

#[proc_macro]
pub fn struct_imports(input: TokenStream) -> TokenStream {
    structs::imports(input)
}

#[proc_macro_attribute]
pub fn enum_derives(args: TokenStream, input: TokenStream) -> TokenStream {
    enums::derives(args, input)
}

#[proc_macro_attribute]
pub fn struct_derives(args: TokenStream, input: TokenStream) -> TokenStream {
    structs::derives(args, input)
}
