use super::Scope;

// #[derive(Default, Debug, Clone)]

#[derive(Debug, PartialEq)]
pub struct Snippet {
    pub description: String,
    pub code: Vec<SnippetValue>,
    pub scope: Scope,
}

#[derive(Debug, PartialEq)]
pub struct SnippetValue {
    pub decorators: Vec<Decorator>,
    pub value: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Set(String, String),
}

#[derive(Debug, PartialEq)]
pub struct Decorator {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, PartialEq)]
pub enum Arg {
    Text(String),
}
