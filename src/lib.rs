use proc_macro::TokenStream;
use quote::{quote};


#[proc_macro]
pub fn nigga(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
        fn deez() -> i32 {
            122
        }
    ))
}