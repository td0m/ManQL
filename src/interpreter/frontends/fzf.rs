use super::Frontend;
use crate::ast::{Snippet, Token};
use std::{
    io::Write,
    process::{Command, Stdio},
};

const DEFAULT_ARGS: &'static [&str] = &[
    "--preview-window",
    "top:1",
    "--reverse",
    "--border",
    "-e",
    "-i",
];

pub struct FZF {}

impl Frontend for FZF {
    fn select_snippet<'a>(&self, snippets: &'a Vec<Snippet>) -> &'a Snippet {
        let mut p = Command::new("fzf")
            .args(DEFAULT_ARGS.iter().chain(&[
                "--preview",
                "echo {} | cut -d'\t' -f2", // second column is the preview
                "--tabstop",
                &FZF::get_tabsize(snippets).to_string(),
            ]))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = p.stdin.as_mut().unwrap();

        // TODO: change that to vec and .join it later
        let mut displayed = String::from("");
        for snippet in snippets {
            // TODO: actually get the 1st executable value
            let command = format!("{}", snippet);
            displayed.push_str(&format!("{}\t{}\n", &snippet.description, command));
        }
        stdin.write(displayed.as_bytes()).unwrap();

        let o = p.wait_with_output().unwrap();
        let output_str = std::str::from_utf8(&o.stdout).unwrap();
        let columns_output: Vec<&str> = output_str.split("\t").collect();
        let chosen_action = columns_output.get(0).unwrap();

        for snippet in snippets {
            if &snippet.description == chosen_action {
                return snippet;
            }
        }

        panic!()
    }
    fn select_set_value(&self, tokens: &Vec<Token>, i: usize, set: &Vec<String>) -> String {
        let token = tokens.get(i).unwrap();
        match token {
            Token::Text(_) => panic!("Expected reference, given text"),
            Token::Reference(name) => {
                let mut p = Command::new("fzf")
                    .args(DEFAULT_ARGS.iter().chain(&[
                        "--bind",
                        "alt-enter:print-query",
                        "--preview",
                        &self.preview(tokens, i),
                        "--prompt",
                        &format!("<{}> ", name),
                    ]))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let stdin = p.stdin.as_mut().unwrap();

                let mut displayed: Vec<String> = vec![];
                for option in set {
                    displayed.push(String::from(option));
                }
                stdin.write(displayed.join("\n").as_bytes()).unwrap();

                let o = p.wait_with_output().unwrap();
                let output_str = std::str::from_utf8(&o.stdout).unwrap().trim_end();

                String::from(output_str)
            }
        }
    }
}

impl FZF {
    pub fn new() -> Self {
        FZF {}
    }

    // get the longest description and add padding to it to get the tab size
    fn get_tabsize(snippets: &Vec<Snippet>) -> usize {
        let mut size = 4;
        for s in snippets {
            if s.description.len() > size {
                size = s.description.len();
            }
        }
        size + 2 // we need at least 1 padding, otherwise widest column will no be displayed properly
    }

    fn preview(&self, tokens: &Vec<Token>, i: usize) -> String {
        let mut preview = String::from("echo \"");
        for (j, token) in tokens.iter().enumerate() {
            if i == j {
                preview.push_str("$(tput setaf 4){}$(tput sgr 0)");
            } else {
                let mut v = format!("{}", token);
                if let Token::Reference(r) = token {
                    if j < i {
                        v = String::from(r);
                    }
                }
                v = v.replace("\"", "\\\"");
                preview.push_str(&v);
            }
        }
        preview.push('"');
        preview
    }
}
