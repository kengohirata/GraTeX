mod command;
mod token_to_ast;
pub use command::Command;

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

#[derive(Debug, PartialEq, Eq)]
pub enum Word {
    Text(String),
    Env(String, Ast),
    MathInline(String),
    Command(command::Command),
    Lines(Ast),
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
