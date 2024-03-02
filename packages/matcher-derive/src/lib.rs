extern crate proc_macro;
use matcher_derive_impl::impl_matcher_derive;

use proc_macro::TokenStream;

#[proc_macro_derive(Match, attributes(match_start_with))]
pub fn matcher_derive(stream: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let stream_copy = stream.clone();
    let mut item = syn::parse(stream_copy).unwrap();

    // Build the output, possibly using quasi-quotation
    let expanded = impl_matcher_derive(&mut item);

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
