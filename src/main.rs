mod ast;
mod interpreter;
mod parser;

extern crate serde_json;
#[macro_use]
extern crate pest_derive;


use ast::{Scope, Set, SetValue, Snippet, SnippetValue, Token};
use interpreter::frontends::FZF;
use interpreter::Interpreter;

fn main() {
    // TODO: if --validate flag is present:
    // run the SemanticAnalyzer to check for any errors
    // panic on error
    // TODO: if fzf, run fzf frontend
    // TODO: if search {{command}}, return TLDR style
    let scope = Scope {
        sets: vec![
            Set {
                identifier: String::from("user"),
                values: vec![
                    SetValue::Code(String::from("grep wheel /etc/group | cut -d':' -f4")),
                    SetValue::Text(String::from("root")),
                ],
            },
            Set {
                identifier: String::from("host"),
                values: vec![SetValue::Text(String::from("localhost"))],
            },
            Set {
                identifier: String::from("command"),
                values: vec![SetValue::Text(String::from("ls -la"))],
            },
        ],
        snippets: vec![
            Snippet {
                description: String::from("connect to remote server and run command"),
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
            Snippet {
                description: String::from("connect to remote server"),
                values: vec![SnippetValue {
                    decorators: vec![],
                    tokens: vec![
                        Token::Text(String::from("ssh ")),
                        Token::Reference(String::from("user")),
                        Token::Text(String::from("@")),
                        Token::Reference(String::from("host")),
                    ],
                }],
            },
        ],
    };
    let mut program = Interpreter::new(&scope, FZF::new());
    program.run();
}
