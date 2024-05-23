use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

pub fn imports(_input: TokenStream) -> TokenStream {
    let imports = quote! {
        use bincode::{Decode, Encode};
        use serde::{Serialize, Deserialize};
        use strum_macros::{Display, EnumString};
        use yaserde_derive::{YaSerialize, YaDeserialize};
    };

    TokenStream::from(imports)
}

pub fn derives(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let output = match input {
        Item::Enum(item_enum) => {
            quote! {
                #[derive(
                    Clone,
                    Debug,
                    Default,
                    Display,
                    EnumString,
                    PartialEq,
                    PartialOrd,
                    Encode,
                    Decode,
                    Serialize,
                    Deserialize,
                    YaSerialize,
                    YaDeserialize,
                )]
                #item_enum
            }
        }
        _ => {
            return syn::Error::new_spanned(input, "This attribute can only be used with enums")
                .to_compile_error()
                .into();
        }
    };

    TokenStream::from(output)
}
