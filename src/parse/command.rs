use std::fmt;

use combine::{
    attempt, between, choice, many1, parser::char::string, satisfy, token, unexpected_any, value,
    ParseError, Parser, Stream,
};

use super::word::{make_upper_substitute, parse_pure_spaces};
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    WithString {
        name: StringInputs,
        contents: String,
    },
    WithText {
        name: TextInputs,
        contents: Box<Vec<Line>>,
    },
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::WithString {
                name: StringInputs::Begin,
                contents,
            } => write!(f, "\n% ---------- \\begin: {} ----------\n", contents),
            Command::WithString {
                name: StringInputs::End,
                contents,
            } => write!(f, "\n% ---------- \\end: {} ----------\n", contents),
            Command::WithString {
                name: StringInputs::Section(n),
                contents,
            } => {
                for _ in 0..*n {
                    write!(f, "#")?
                }
                writeln!(f, " {}", contents)
            }
            Command::WithString {
                name: StringInputs::Label,
                ..
            } => Ok(()),
            Command::WithString {
                name: StringInputs::Cite,
                contents,
            } => write!(f, "[{}]", make_upper_substitute(contents.to_owned())),
            Command::WithText {
                name: TextInputs::Emph,
                contents,
            } => {
                for line in contents.iter() {
                    if line.words.is_empty() {
                        write!(f, "\n\n")?;
                    } else {
                        for word in line.words.iter() {
                            // [FIXME] may produce extra spaces
                            write!(f, "{}", word)?;
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CommandNames {
    StringInputCommands(StringInputs),
    TextInputCommands(TextInputs),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StringInputs {
    Begin,
    End,
    Section(u8),
    Label,
    Cite,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextInputs {
    Emph,
}

impl std::str::FromStr for CommandNames {
    type Err = StringStreamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CommandNames::*;
        let ok = match s {
            "begin" => StringInputCommands(StringInputs::Begin),
            "end" => StringInputCommands(StringInputs::End),
            "section" => StringInputCommands(StringInputs::Section(1)),
            "subsection" => StringInputCommands(StringInputs::Section(2)),
            "label" => StringInputCommands(StringInputs::Label),
            "cite" => StringInputCommands(StringInputs::Cite),
            "emph" => TextInputCommands(TextInputs::Emph),
            _ => return Err(StringStreamError::UnexpectedParse),
        };
        Ok(ok)
    }
}

impl CommandNames {
    pub const KEYWORDS: [&str; 7] = [
        "begin",
        "end",
        "section",
        "subsection",
        "label",
        "emph",
        "cite",
    ];
}

pub fn parse_command<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    parse_command_name().then(|name| {
        between(
            token('{'),
            token('}'),
            match name {
                CommandNames::StringInputCommands(name) => many1(satisfy(|ch| ch != '}'))
                    .map(move |s| Command::WithString { name, contents: s })
                    .left(),
                CommandNames::TextInputCommands(name) => parse_lines()
                    .map(move |ls| Command::WithText {
                        name,
                        contents: Box::new(ls),
                    })
                    .right(),
            },
        )
    })
}

// Command call, e.g., `\begin`
fn parse_command_name<Input>() -> impl Parser<Input, Output = CommandNames>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(between(
        (token('\\'), parse_pure_spaces()),
        parse_pure_spaces(),
        choice(CommandNames::KEYWORDS.map(|s| attempt(string(&s)))),
    ))
    .then(|s| match CommandNames::from_str(s) {
        Ok(cn) => value(cn).left(),
        Err(_) => unexpected_any("Failed to match a string to a known name.").right(),
    })
}
