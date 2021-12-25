extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod lexer;

use std::collections::HashMap;
use crate::pest::Parser;

use ast::*;
use lexer::*;

fn main() {
    // let scope = Scope {
    //     sets: HashMap::from([(
    //         "port".to_owned(),
    //         Set {
    //             vartype: vec![],
    //             values: vec![
    //                 Value::Text(String::from("22")),
    //                 Value::Text(String::from("88")),
    //             ],
    //         },
    //     )]),
    //     snippets: vec![],
    // };

    // let input = "hi = port | \"80\" | `echo 22`".to_owned();
    // let mut tree = SnailParser::parse(Rule::file, &input).unwrap();
    // // println!("{:#?}", tree);
    // let mut nodes = ast::build(&mut tree);
    // println!("{:#?}", nodes);

    // println!("{:#?}", scope);
}
