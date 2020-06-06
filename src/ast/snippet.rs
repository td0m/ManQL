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
                match token {
                    Token::Text(s) => out.push_str(&s),
                    Token::Reference(s) => out.push_str(&format!("{{{{{}}}}}", &s)),
                }
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

#[derive(Debug)]
pub enum Token {
    Text(String),
    Reference(String),
}
