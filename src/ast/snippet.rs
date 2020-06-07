use std::fmt;

#[derive(Default, Debug)]
pub struct Snippet {
    pub description: String,
    pub values: Vec<SnippetValue>,
}

impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::from("");
        for value in &self.values {
            for token in &value.tokens {
                out.push_str(&format!("{}", token));
            }
        }
        write!(f, "{}", out)
    }
}

#[derive(Default, Debug)]
pub struct SnippetValue {
    pub decorators: Vec<ValueDecorator>,
    pub tokens: Vec<Token>,
}

#[derive(Default, Debug)]
pub struct ValueDecorator {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Token {
    Text(String),
    Reference(String),
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Reference(r) => write!(f, "{{{{{}}}}}", r),
            Token::Text(s) => write!(f, "{}", s)
        }
    }
}