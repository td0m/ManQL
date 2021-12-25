mod scope;
mod set;
mod snippet;
mod value;

use pest::iterators::{Pair, Pairs};

use crate::lexer::*;

pub use scope::*;
pub use set::*;
pub use snippet::*;
pub use value::*;

pub fn build(tree: &mut Pairs<Rule>) -> Scope {
    let mut scope = Scope {
        ..Default::default()
    };
    while let Some(next) = tree.next() {
        match next.as_rule() {
            Rule::set => {
                let mut inner: Pairs<Rule> = next.into_inner();
                let fst = inner.next().unwrap();
                if let Rule::identifier = fst.as_rule() {
                    scope
                        .sets
                        .insert(fst.as_str().to_string(), parse_set(&mut inner));
                } else {
                    panic!("invalid token");
                }
                // scope.sets.insert(identifier, rest);
            }
            Rule::EOI => {
                break;
            }
            f => {
                todo!("{:#?}", f);
            }
        }
    }
    scope
}

fn parse_set(tree: &mut Pairs<Rule>) -> Set {
    let mut set = Set {
        ..Default::default()
    };

    if let Rule::value = tree.peek().unwrap().as_rule() {}

    for v in tree {
        let content = v.as_str().to_string();
        set.values.push(match v.as_rule() {
            Rule::string_double => Value::Text(content),
            Rule::identifier => Value::Reference(content),
            Rule::code => Value::Code(content),
            _ => todo!("unimplemented rule"),
        });
    }

    set
}
