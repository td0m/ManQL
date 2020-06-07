mod fzf;

use crate::ast::{Snippet, Token};
pub use fzf::FZF;

pub trait Frontend {
    // takes an already ordered list of snippets (from most to least used)
    // returns the index of the selected snippet
    fn select_snippet<'a>(&self, snippets: &'a Vec<Snippet>) -> &'a Snippet;
    // takes an array of tokens (for displaying the command live) and set with valid values
    // returns user input. It doesn't have to be one in the set
    fn select_set_value(&self, tokens: &Vec<Token>, i: usize, set: &Vec<String>) -> String;
}
