use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::utils::{match_with_vector, match_with_vector_f};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TypeOrVector<T> {
    Type(T),
    Vector(Vec<T>),
}

pub type BaseMatcher<DataType> = Option<TypeOrVector<DataType>>;

pub trait Matcher {
    type AllMatcher: Clone;
    fn match_all(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool;
    fn match_start(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        self.match_all(matcher)
    }
}

impl Matcher for String {
    type AllMatcher = String;
    fn match_all(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        match_with_vector(matcher, Some(self))
    }

    fn match_start(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        match_with_vector_f(matcher, Some(self), |match_value, value| {
            value.starts_with(match_value)
        })
    }
}

impl Matcher for i64 {
    type AllMatcher = i64;
    fn match_all(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        match_with_vector(matcher, Some(self))
    }
}

impl Matcher for usize {
    type AllMatcher = usize;
    fn match_all(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        match_with_vector(matcher, Some(self))
    }
}

impl Matcher for bool {
    type AllMatcher = bool;
    fn match_all(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        match_with_vector(matcher, Some(self))
    }
}

impl<T: Matcher> Matcher for Option<T> {
    type AllMatcher = <T as Matcher>::AllMatcher;
    fn match_all(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        match self {
            Some(value) => value.match_all(matcher),
            None => matcher.is_none(),
        }
    }
}

impl<T: Matcher> Matcher for Vec<T> {
    type AllMatcher = <T as Matcher>::AllMatcher;

    fn match_all(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        match matcher {
            Some(matcher) => match matcher {
                TypeOrVector::Type(match_value) => {
                    for value in self.iter() {
                        if !value.match_all(Some(TypeOrVector::Type(match_value.clone()))) {
                            return false;
                        }
                    }
                    true
                }
                TypeOrVector::Vector(vector) => {
                    for (index, value) in self.iter().enumerate() {
                        let has_match = match vector.get(index) {
                            Some(match_value) => {
                                value.match_all(Some(TypeOrVector::Type(match_value.clone())))
                            }
                            None => return false,
                        };

                        if !has_match {
                            return false;
                        }
                    }
                    true
                }
            },
            None => true,
        }
    }
}

pub type HashMapMatcher<Key, Value> = HashMap<Key, TypeOrVector<Value>>;

impl<Key: Eq + Hash + Clone, Value: PartialEq + Clone> Matcher for HashMap<Key, Value> {
    type AllMatcher = HashMapMatcher<Key, Value>;
    fn match_all(&self, matcher: BaseMatcher<Self::AllMatcher>) -> bool {
        match matcher {
            Some(values_vec_or_type) => match values_vec_or_type {
                TypeOrVector::Type(match_value) => match_value.iter().all(|(key, match_values)| {
                    let actor_value = self.get(key);

                    match_with_vector(Some(match_values.clone()), actor_value)
                }),
                TypeOrVector::Vector(vector) => vector.iter().any(|match_value| {
                    match_value.iter().all(|(key, match_values)| {
                        let actor_value = self.get(key);

                        match_with_vector(Some(match_values.clone()), actor_value)
                    })
                }),
            },
            None => true,
        }
    }
}
