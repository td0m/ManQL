pub(crate) mod frontends;

use crate::ast::{Scope, Set, Snippet, Token, SetValue};
use frontends::Frontend;
use std::process::Command;

pub struct Interpreter<'a, F: Frontend> {
    scope: &'a Scope,
    frontend: F,
}

impl<'a, F: Frontend> Interpreter<'a, F> {
    pub fn new(scope: &'a Scope, frontend: F) -> Self {
        Interpreter { scope, frontend }
    }

    pub fn run(&self) {
		let snippet = self.frontend.select_snippet(&self.scope.snippets);
		let mut interpolated: Vec<String> = vec![];
		for value in &snippet.values {
			let mut tokens: Vec<Token> = vec![];
			for (i, token) in value.tokens.iter().enumerate() {
				match token {
					Token::Text(t) => tokens.push(Token::Text(String::from(t))),
					Token::Reference(set_name) => {
						let set = self.find_set(set_name);
						let mut all_tokens: Vec<Token> = tokens.clone();
						all_tokens.push(token.clone());
						for j in i+1..value.tokens.len() {
							let v = value.tokens.get(j).unwrap();
							all_tokens.push(v.clone());
						}
						// TODO: sort these by most recently used (by using cache history)
						let set_strings = self.resolve_set_values(set);
						let set_value = self.frontend.select_set_value(&all_tokens, i, &set_strings);
						tokens.push(Token::Reference(set_value));
					}
				}
			}
			let mut string = String::from("");
			for t in tokens {
				match t {
					Token::Reference(r) => string.push_str(&r),
					Token::Text(t) => string.push_str(&t),
				}
			}
			interpolated.push(string);
		}
		println!("{}", interpolated.join("\n"));
		/*
		for (i, token) in snippet.tokens {
			if token is Reference as set_reference {
				set = find_set_by_name(set_reference)
				strings = resolve_set_values(set)
				user_value = self.frontend.select_set_value(snippet.tokens, strings)
				snippet.tokens.set(0, Token::Text(user_value))
			}
		}
		*/
	}

	fn find_set(&self, name: &str) -> &'a Set {
		for set in &self.scope.sets {
			if set.identifier==name {
				return set
			}
		}
		panic!()
	}
	
	fn resolve_set_values(&self, set: &Set) -> Vec<String> {
		let mut values: Vec<String> = vec![];
		for set_value in &set.values {
			match set_value {
				SetValue::Text(s) => values.push(String::from(s)),
				SetValue::Code(c) => {
					values.push(exec(c));
				}
				SetValue::SubsetOf(c) => {
					values.push(String::from(c));
				}
			}
		}
		values
	}
}

// TODO: move that to a different file
fn exec(cmd: &str) -> String {
    let stdout = Command::new("bash")
        .args(&["-c", cmd])
        .output()
        .unwrap()
        .stdout;
    std::str::from_utf8(&stdout).unwrap().trim().to_owned()
}