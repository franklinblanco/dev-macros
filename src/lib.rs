use proc_macro::TokenStream;
use quote::{quote};


#[proc_macro]
pub fn nigga(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
            122
    ))
}