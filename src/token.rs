use combine::{error::StringStreamError, stream::position, Parser};
use std::{fmt, io, str::FromStr};

use self::word::parse_words;
mod command;
mod word;
#[cfg(test)]
mod test;
pub use command::Command;

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
pub enum Word {
    Text(String),
    Command(command::Command),
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
            Word::EndLine => write!(f, "↵\n"),
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
