use std::fmt;

use super::{make_upper_substitute, token_to_ast::token_to_ast};
use crate::token;

use super::{Ast, Word};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Section(u8, Ast),
    Label,
    Cite(String),
    Ref(String),
    Item,
}

pub(super) fn token_to_ast_command(
    c: token::Command,
    mut args: Vec<token::Document>,
) -> Option<Word> {
    let w = match c {
        token::Command::Section(n) => {
            assert_eq!(args.len(), 1);
            Word::Command(Command::Section(n, token_to_ast(args.remove(0))))
        }
        token::Command::Label => return None,
        token::Command::Cite => {
            assert_eq!(args.len(), 1);
            let s = format!("{}", args[0]);
            Word::Command(Command::Cite(make_upper_substitute(s)))
        }
        token::Command::Ref => {
            assert_eq!(args.len(), 1);
            let s = format!("{}", args[0]);
            let name = make_ref_name(s);
            Word::Command(Command::Ref(format!("{} 7", name)))
        }
        token::Command::Font => {
            assert_eq!(args.len(), 1);
            Word::Lines(token_to_ast(args.remove(0)))
        }
        token::Command::Item => {
            assert_eq!(args.len(), 0);
            Word::Command(Command::Item)
        }
        token::Command::Space => return None,
        token::Command::Unknown(s) => Word::Text(s),
    };
    Some(w)
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Section(n, ast) => {
                for _ in 0..*n {
                    write!(f, "#")?
                }
                writeln!(f, " {ast}")
            }
            Command::Label => Ok(()),
            Command::Cite(s) => {
                write!(f, "[{s}]")
            }
            Command::Ref(s) => write!(f, "{s}"),
            Command::Item => write!(f, "\n  - "),
        }
    }
}

fn make_ref_name(s: String) -> String {
    let mut s = s
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_lowercase();
    s.truncate(3);
    let new_name = match &*s {
        "sec" => "Section",
        "sub" => "Section",
        "thm" | "the" => "Theorem",
        "lem" => "Lemma",
        "pro" | "prp" => "Proposition",
        "cor" => "Corollary",
        "def" => "Definition",
        c if c.strip_prefix("eg").is_some() => "Example",
        c if c.strip_prefix("ex").is_some() => "Example",
        c if c.strip_prefix("ax").is_some() => "Axiom",
        "rem" | "rmk" => "Remark",
        _ => "Theorem",
    };
    new_name.to_owned()
}
