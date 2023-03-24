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
        contents: Vec<Line>,
    },
    NoArgs {
        name: NoArgs,
    },
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::WithString {
                name: StringInputs::Begin,
                contents,
            } => write!(f, "\n% ---------- \\begin: {contents} ----------\n"),
            Command::WithString {
                name: StringInputs::End,
                contents,
            } => write!(f, "\n% ---------- \\end: {contents} ----------\n"),
            Command::WithString {
                name: StringInputs::Section(n),
                contents,
            } => {
                for _ in 0..*n {
                    write!(f, "#")?
                }
                writeln!(f, " {contents}")
            }
            Command::WithString {
                name: StringInputs::Label,
                ..
            } => Ok(()),
            Command::WithString {
                name: StringInputs::Cite,
                contents,
            } => write!(f, "[{}]", make_upper_substitute(contents.to_owned())),
            Command::WithString {
                name: StringInputs::Ref,
                contents,
            } => write!(f, "{} 7", make_ref_name(contents.to_owned())),
            Command::WithText {
                name: TextInputs::Font,
                contents,
            } => {
                for line in contents.iter() {
                    if line.words.is_empty() {
                        write!(f, "\n\n")?;
                    } else {
                        for word in line.words.iter() {
                            // [FIXME] may produce extra spaces
                            write!(f, "{word}")?;
                        }
                    }
                }
                Ok(())
            }
            Command::NoArgs { name: NoArgs::Item } => write!(f, "\n- "),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CommandNames {
    StringInputCommands(StringInputs),
    TextInputCommands(TextInputs),
    NoArgCommands(NoArgs),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StringInputs {
    Begin,
    End,
    Section(u8),
    Label,
    Cite,
    Ref,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextInputs {
    Font,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NoArgs {
    Item,
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
            "ref" | "cref" | "Cref" => StringInputCommands(StringInputs::Ref),
            "emph" => TextInputCommands(TextInputs::Font),
            c if c.strip_prefix("text").is_some() => TextInputCommands(TextInputs::Font),
            "item" => NoArgCommands(NoArgs::Item),
            _ => return Err(StringStreamError::UnexpectedParse),
        };
        Ok(ok)
    }
}

impl CommandNames {
    pub const KEYWORDS: [&str; 12] = [
        // "begin",
        // "end",
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
    ];

    fn has_args(&self) -> bool {
        !matches!(self, CommandNames::NoArgCommands(_))
    }
    fn no_arg(self) -> NoArgs {
        match self {
            Self::NoArgCommands(name) => name,
            _ => panic!("Panic at no_args. Use this function in asserted place."),
        }
    }
}

pub fn parse_command<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    parse_command_name().then(|name| {
        if name.has_args() {
            between(
                token('{'),
                token('}'),
                match name {
                    CommandNames::StringInputCommands(name) => many1(satisfy(|ch| ch != '}'))
                        .map(move |s| Command::WithString { name, contents: s })
                        .left(),
                    CommandNames::TextInputCommands(name) => parse_lines()
                        .map(move |contents| Command::WithText { name, contents })
                        .right(),
                    CommandNames::NoArgCommands(_) => {
                        unreachable!("Unreachable in parse_command_name")
                    }
                },
            )
            .left()
        } else {
            value(name)
                .map(|name| Command::NoArgs {
                    name: name.no_arg(),
                })
                .right()
        }
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
        choice(CommandNames::KEYWORDS.map(|s| attempt(string(s)))),
    ))
    .then(|s| match CommandNames::from_str(s) {
        Ok(cn) => value(cn).left(),
        Err(_) => unexpected_any("Failed to match a string to a known name.").right(),
    })
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
