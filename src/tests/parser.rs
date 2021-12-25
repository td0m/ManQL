use std::{collections::HashMap, vec};

use crate::{ast::*, lexer::*};
use pest::Parser;

pub fn ast(input: &str, expected: &Scope) {
    let tokens = SnailLexer::parse(Rule::line, input);
    if let Err(e) = tokens {
        panic!("{:#?}", e);
    }
    let scope = build(&mut tokens.unwrap());
    assert_eq!(expected, &scope);
}

#[test]
fn single_element_set() {
    ast(
        "hi = \"hello\"",
        &Scope {
            sets: HashMap::from([(
                String::from("hi"),
                Set {
                    vartype: vec![],
                    values: vec![Value::Text(String::from("hello"))],
                },
            )]),
            snippets: vec![],
        },
    );
}

#[test]
fn multi_element_set() {
    ast(
        "hi = \"hello\" | \"my\" | \"world!\"",
        &Scope {
            sets: HashMap::from([(
                String::from("hi"),
                Set {
                    vartype: vec![],
                    values: vec![
                        Value::Text(String::from("hello")),
                        Value::Text(String::from("my")),
                        Value::Text(String::from("world!")),
                    ],
                },
            )]),
            snippets: vec![],
        },
    );
}
