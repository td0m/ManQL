mod ast;
mod interpreter;

use ast::{Scope, Set, SetValue, Snippet, SnippetValue, Token};
use interpreter::frontends::FZF;
use interpreter::Interpreter;

fn main() {
    // TODO: if --validate flag is present:
    // run the SemanticAnalyzer to check for any errors
    // panic on error
    let scope = Scope {
        sets: vec![
            Set {
                identifier: String::from("user"),
                values: vec![SetValue::Text(String::from("root"))],
            },
            Set {
                identifier: String::from("host"),
                values: vec![SetValue::Text(String::from("localhost"))],
            },
        ],
        snippets: vec![
            Snippet {
                description: String::from("remote connect to user"),
                values: vec![SnippetValue {
                    decorators: vec![],
                    tokens: vec![
                        Token::Text(String::from("ssh ")),
                        Token::Reference(String::from("user")),
                        Token::Text(String::from("@soton.cloud")),
                    ],
                }],
            },
            Snippet {
                description: String::from("ssh and run command"),
                values: vec![SnippetValue {
                    decorators: vec![],
                    tokens: vec![
                        Token::Text(String::from("ssh ")),
                        Token::Reference(String::from("user")),
                        Token::Text(String::from("@")),
                        Token::Reference(String::from("host")),
                        Token::Text(String::from(" ")),
                        Token::Reference(String::from("command")),
                    ],
                }],
            },
        ],
    };
    let program = Interpreter::new(&scope, FZF::new());
    program.run();
}
