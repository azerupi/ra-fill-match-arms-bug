use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

use quote::quote;

#[proc_macro_attribute]
pub fn payload(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let result = match item {
        Item::Impl(payload_impl) => {
            let output = quote! {
                #payload_impl
            };
        
            output
        },
        _ => todo!(),
    };

    result.into()
}
