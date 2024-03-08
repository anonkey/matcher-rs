extern crate proc_macro;
use matcher_derive_impl::impl_matcher_derive;

use proc_macro::TokenStream;

#[proc_macro_derive(Match, attributes(match_start_with))]
pub fn matcher_derive(stream: TokenStream) -> TokenStream {
    let stream_copy = stream.clone();
    let mut item = syn::parse(stream_copy).unwrap();

    let expanded = impl_matcher_derive(&mut item);

    TokenStream::from(expanded)
}
