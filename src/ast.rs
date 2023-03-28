mod command;
mod env;
mod token_to_ast;
use std::fmt;

pub use command::Command;
pub use token_to_ast::token_to_ast;

use self::env::write_env;

#[derive(Debug, PartialEq, Eq)]
pub struct Ast(Vec<Paragraph>);

impl Ast {
    pub fn new() -> Self {
        Ast(Vec::new())
    }
    pub fn push(&mut self, paragraph: Paragraph) {
        self.0.push(paragraph);
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter().peekable();
        while let Some(p) = iter.next() {
            write!(f, "{p}")?;
            if iter.peek().is_some() {
                write!(f, "\n\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paragraph(Vec<Word>);

impl Paragraph {
    pub fn new() -> Self {
        Paragraph(Vec::new())
    }
    pub fn push(&mut self, word: Word) {
        self.0.push(word);
    }
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter().peekable();
        while let Some(w) = iter.next() {
            write!(f, "{w}")?;
            if iter.peek().is_some() {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Word {
    Text(String),
    Env(String, Ast),
    MathInline(String),
    Command(command::Command),
    Lines(Ast),
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Word::Text(s) => write!(f, "{}", s),
            Word::Env(env, ast) => write_env(f, env, ast),
            Word::MathInline(s) => write!(f, "{s}"),
            Word::Command(c) => write!(f, "{c}"),
            Word::Lines(ast) => write!(f, "{ast}"),
        }
    }
}

pub fn make_upper_substitute(s: String) -> String {
    let mut s = take_alph_and_to_upper(s);
    if s.len() < 2 {
        for _ in 0..2 - s.len() {
            s.push('X');
        }
    } else {
        s.truncate(2);
    }
    s
}

fn take_alph_and_to_upper(s: String) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_uppercase()
}
