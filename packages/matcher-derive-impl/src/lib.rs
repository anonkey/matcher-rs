extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

pub mod matcher;
pub mod parse_struct;
pub mod utils;

pub fn matcher_derive_enum(enum_data: &ItemEnum) -> TokenStream {
    let struct_ident = enum_data.ident.clone();

    let res = quote! {

      impl Matcher for #struct_ident {
        type AllMatcher = Self;
        fn match_all(&self, matcher: matcher_derive_impl::matcher::BaseMatcher<Self::AllMatcher>) -> bool {
          matcher_derive_impl::utils::match_with_vector(matcher, Some(self))
        }
    }

    };

    res
}

pub fn impl_matcher_derive(item: &mut syn::Item) -> proc_macro2::TokenStream {
    match item {
        syn::Item::Struct(data) => parse_struct::matcher_derive_struct(data),
        syn::Item::Enum(data) => matcher_derive_enum(data),
        _ => panic!("expected struct or enum"),
    }
}
