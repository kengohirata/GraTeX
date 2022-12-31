use std::{fmt, io, str::FromStr};

use combine::{
    between, eof, error::StringStreamError, many, many1, none_of, optional, parser::char::newline,
    satisfy, sep_end_by, stream::position, token, ParseError, Parser, Stream, sep_by,
};

#[cfg(test)]
mod test;

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
    Math(String),
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::Text(s) => write!(f, "{}", s),
            Word::Math(s) => write!(f, "{}", s),
        }
    }
}

fn parse_line<Input>() -> impl Parser<Input, Output = Line>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        parse_pure_spaces(),
        sep_end_by(parse_math().or(parse_text()), parse_pure_spaces()),
    )
        .map(|(_, words)| Line {
            words,
            comments: None,
        })
}

fn parse_text<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(none_of(['$', '\t', '\n', ' '].iter().cloned())).map(Word::Text)
}

fn parse_math<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(
        token('$'),
        token('$'),
        many1::<String, _, _>(none_of(['$'].iter().cloned())),
    )
    .and(many(none_of([' ', '\n', '\t'])))
    .map(|(s, t): (String, String)| {
        if s.is_empty() {
            Word::Math("XX".to_string())
        } else {
            let mut s_alpha = take_alph_and_to_upper(s);
            s_alpha.push_str(&t);
            Word::Math(s_alpha)
        }
    })
}

fn take_alph_and_to_upper(s: String) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_uppercase()
}

fn parse_pure_spaces<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many(satisfy(|c: char| c != '\n' && c.is_whitespace())).map(|_: String| ())
}
