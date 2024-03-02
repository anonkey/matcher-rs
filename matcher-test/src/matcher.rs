use matcher_derive::Match;
use matcher_derive_impl::matcher::Matcher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, Match)]
pub struct Toto {
    pub id: String,
    #[match_start_with]
    pub name: String,
    pub toto: HashMap<String, String>,
}
