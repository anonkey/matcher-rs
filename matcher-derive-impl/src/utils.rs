use proc_macro2::{TokenStream, TokenTree};
use std::collections::HashMap;

use quote::ToTokens;

use crate::matcher::{BaseMatcher, TypeOrVector};

pub fn match_with_vector_f<
    MatcherType,
    ReferenceType,
    Callback: FnOnce(&MatcherType, &ReferenceType) -> bool,
>(
    matcher: BaseMatcher<MatcherType>,
    value: Option<&ReferenceType>,
    callback: Callback,
) -> bool
where
    Callback: Copy,
{
    match matcher {
        Some(values_vec_or_type) => match value {
            Some(value) => match values_vec_or_type {
                TypeOrVector::Type(match_value) => callback(&match_value, value),
                TypeOrVector::Vector(vector) => vector
                    .iter()
                    .any(|match_value| callback(match_value, value)),
            },
            None => false,
        },
        None => true,
    }
}

pub fn match_with_vector<DataType>(matcher: BaseMatcher<DataType>, value: Option<&DataType>) -> bool
where
    DataType: std::cmp::PartialEq,
{
    match_with_vector_f(matcher, value, |match_value, value| match_value == value)
}

pub fn match_with_hashmap_vector<DataType>(
    matcher: BaseMatcher<HashMap<String, TypeOrVector<DataType>>>,
    value: Option<&HashMap<String, DataType>>,
) -> bool
where
    DataType: std::cmp::PartialEq + Clone,
{
    match_with_vector_f(matcher, value, |matcher, value_to_match| {
        matcher.iter().all(|(key, match_values)| {
            let actor_value = value_to_match.get(key);

            match_with_vector(Some(match_values.clone()), actor_value)
        })
    })
}

pub fn print_stream(stream: TokenStream) {
    let iter = stream.clone().into_iter();
    eprintln!("Print NODE: {}", stream);
    for item in iter {
        match item {
            TokenTree::Group(group) => {
                eprintln!("group ");
                eprintln!("{}", group.into_token_stream());
                // print_stream(group.into_token_stream());
            }
            TokenTree::Ident(ident) => {
                eprintln!("ident {}", ident.into_token_stream());
            }
            TokenTree::Punct(punct) => {
                eprintln!("punct {}", punct.into_token_stream());
            }
            TokenTree::Literal(literal) => {
                eprintln!("literal {}", literal.into_token_stream());
            }
        }
    }
}

pub fn print_ast(ast: &syn::DeriveInput) {
    eprintln!("Print AST: {}", ast.ident);
}
