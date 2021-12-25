use std::collections::HashMap;

use super::{Set, Snippet};

#[derive(Debug, Default, PartialEq)]
pub struct Scope {
    pub sets: HashMap<String, Set>,
    pub snippets: Vec<Snippet>,
}