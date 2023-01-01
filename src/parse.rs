use std::{fmt, io, str::FromStr};

use combine::{
    attempt, between, choice, error::StringStreamError, many, many1, none_of, not_followed_by,
    parser, parser::char::newline, parser::char::string, satisfy, sep_by, sep_end_by, skip_many,
    stream::position, token, ParseError, Parser, Stream,
};

use word::parse_line;

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
        sep_by(parse_line(), newline())
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

#[derive(Debug, PartialEq, Eq)]
pub enum Word {
    Text(String),
    MathInline(String),
    MathDisplay,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::Text(s) => write!(f, "{}", s),
            Word::MathInline(s) => write!(f, "{}", s),
            Word::MathDisplay => Ok(()),
        }
    }
}
