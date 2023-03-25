use combine::{error::StringStreamError, stream::position, Parser};
use std::{fmt, io, str::FromStr};

use self::word::parse_words;
// mod command;
#[cfg(test)]
mod test;
mod word;

#[derive(Debug)]
pub enum Error<E> {
    Io(io::Error),
    Parse(E),
}

impl<E> fmt::Display for Error<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{err}"),
            Error::Parse(ref err) => write!(f, "{err}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub words: Vec<Word>,
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for word in self.words.iter() {
            write!(f, "{word}")?;
        }
        Ok(())
    }
}

impl FromStr for Document {
    type Err = Error<StringStreamError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_words()
            .parse(position::Stream::new(s))
            .map_err(Error::Parse)
            .map(|words| Document { words: words.0 })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comments(String);

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Section(u8),
    Label,
    Cite,
    Ref,
    Font,
    Item,
    Space,
    Unknown(String),
}

impl Command {
    pub const KEYWORDS: [&str; 13] = [
        "section",
        "subsection",
        "label",
        "emph",
        "cite",
        "ref",
        "cref",
        "Cref",
        "item",
        "textrm",
        "textbf",
        "textit",
        " ",
    ];
}

impl std::str::FromStr for Command {
    type Err = StringStreamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;
        let ok = match s {
            "section" => Section(1),
            "subsection" => Section(2),
            "label" => Label,
            "cite" => Cite,
            "ref" | "cref" | "Cref" => Ref,
            "emph" => Font,
            c if c.strip_prefix("text").is_some() => Font,
            "item" => Item,
            " " => Space,
            _ => return Err(StringStreamError::UnexpectedParse),
        };
        Ok(ok)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Section(n) => {
                for _ in 0..*n {
                    write!(f, "#")?
                }
                Ok(())
            }
            Command::Label => write!(f, "\\LABEL"),
            Command::Cite => write!(f, "\\CITE"),
            Command::Ref => write!(f, "\\REF"),
            Command::Font => write!(f, "\\FONT"),
            Command::Item => write!(f, "\\ITEM"),
            Command::Space => write!(f, ""),
            Command::Unknown(s) => write!(f, "\\{}", s.to_uppercase()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Word {
    Text(String),
    Command(Command),
    Lines(Document),
    Comment(Comments),
    Env(String, Document),
    Dollar,
    EndLine,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::Text(s) => write!(f, "{s} "),
            Word::Command(c) => write!(f, "{c}"),
            Word::Lines(p) => write!(f, "{p}"),
            Word::Comment(s) => write!(f, "%{}", s.0),
            Word::Env(name, d) => write!(f, "\\BEGIN{{{name}}}{d}\\END{{{name}}}"),
            Word::EndLine => write!(f,"↵\n"),
            Word::Dollar => write!(f, "$"),
        }
    }
}

impl Word {
    pub fn is_empty_word(&self) -> bool {
        match self {
            Word::Text(s) => s.is_empty(),
            Word::Command(_) => false,
            Word::Lines(_) => false,
            Word::Comment(_) => false,
            Word::Env(_, _) => false,
            Word::EndLine => true,
            Word::Dollar => false,
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
