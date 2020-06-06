pub(crate) mod frontends;

use crate::ast::{Scope, Set, Snippet, Token};
use frontends::Frontend;

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
		println!("{}", snippet.description);
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
}
