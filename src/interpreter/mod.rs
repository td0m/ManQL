mod cache;
pub(crate) mod frontends;

use cache::Cache;

use crate::ast::{Scope, Set, SetValue, Snippet, Token};
use frontends::Frontend;
use std::process::Command;

pub struct Interpreter<'a, F: Frontend> {
    scope: &'a Scope,
    frontend: F,
    cache: Cache,
}

impl<'a, F: Frontend> Interpreter<'a, F> {
    pub fn new(scope: &'a Scope, frontend: F) -> Self {
        let home = dirs::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let cache = Cache::new(&format!("{}/{}", home, ".manql.cache.json"));
        Interpreter {
            scope,
            frontend,
            cache,
        }
    }

    pub fn run(&mut self) {
        let ordered_snippets = &mut self.scope.snippets.clone();
        ordered_snippets.sort_by(|a, b| self.cache.snippet_partial_cmp(a, b));
        let snippet = self.frontend.select_snippet(ordered_snippets);
        self.cache.push_snippet(snippet);
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
                        for j in i + 1..value.tokens.len() {
                            let v = value.tokens.get(j).unwrap();
                            all_tokens.push(v.clone());
                        }
                        // TODO: sort these by most recently used (by using cache history)
                        let mut set_strings = Self::resolve_set_values(set);
                        set_strings.append(&mut self.cache.get_custom_values(set));
                        set_strings.sort_by(|a, b| self.cache.value_partial_cmp(set, a, b));
                        let value = self.frontend.select_set_value(&all_tokens, i, &set_strings);
                        // if you ctrl+c it leaves an empty string
                        if value.len() > 0 {
                            if !set_strings.iter().any(|s| s == &value) {
                                self.cache.push_custom_value(set, &value)
                            }
                            self.cache.push_set_value(set, &value);
                            tokens.push(Token::Reference(value));
                        } else {
                            break;
                        }
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
        self.cache.save();
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
            if set.identifier == name {
                return set;
            }
        }
        panic!()
    }

    // TODO: move this out of this impl and unit test
    fn resolve_set_values(set: &Set) -> Vec<String> {
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
