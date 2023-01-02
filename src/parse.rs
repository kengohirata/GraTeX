use combine::{error::StringStreamError, stream::position, Parser};
use std::{fmt, io, str::FromStr};

use self::word::parse_lines;
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
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Parse(ref err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paragraph {
    pub lines: Vec<Line>,
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.lines.iter() {
            if line.words.is_empty() {
                write!(f, "\n\n")?;
            } else {
                for word in line.words.iter() {
                    // [FIXME] may produce extra spaces
                    write!(f, "{} ", word)?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Paragraph {
    type Err = Error<StringStreamError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_lines()
            .parse(position::Stream::new(s))
            .map_err(Error::Parse)
            .map(|lines| Paragraph { lines: lines.0 })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    pub words: Vec<Word>,
    pub comments: Option<Comments>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Comments(String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Word {
    Text(String),
    MathInline(String),
    MathDisplay,
    Command(Command),
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::Text(s) => write!(f, "{}", s),
            Word::MathInline(s) => write!(f, "{}", s),
            Word::MathDisplay => Ok(()),
            Word::Command(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
    Begin(String),
    End(String),
    Section(u8, String),
    Ignore,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Begin(s) => write!(f, "\n% --- begin: {} ---\n", s),
            Command::End(s) => write!(f, "\n% --- end: {} ---\n", s),
            Command::Section(n, s) => {
                for _ in 0..*n {
                    write!(f, "#")?
                }
                writeln!(f, " {}", s)
            }
            Command::Ignore => Ok(()),
        }
    }
}

impl Command {
    pub const KEYWORDS: [&str; 5] = ["begin", "end", "section", "subsection",  "label"];

    pub fn from_strings(name: String, arg: String) -> Option<Command> {
        let c = match &*name {
            "begin" => Command::Begin(arg),
            "end" => Command::End(arg),
            "section" => Command::Section(1, arg),
            "subsection" => Command::Section(2, arg),
            "todo" => Command::Ignore,
            "label" => Command::Ignore,
            _ => return None,
        };
        Some(c)
    }
}
