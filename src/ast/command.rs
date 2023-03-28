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
            assert_eq!(args.len(), 1);
            Word::Command(Command::Item)
        }
        token::Command::Space => return None,
        token::Command::Unknown(s) => Word::Text(s),
    };
    Some(w)
}

// impl fmt::Display for Command {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Command::WithString {
//                 name: StringInputs::Begin,
//                 contents,
//             } => write!(f, "\n% ---------- \\begin: {contents} ----------\n"),
//             Command::WithString {
//                 name: StringInputs::End,
//                 contents,
//             } => write!(f, "\n% ---------- \\end: {contents} ----------\n"),
//             Command::WithString {
//                 name: StringInputs::Section(n),
//                 contents,
//             } => {
//                 for _ in 0..*n {
//                     write!(f, "#")?
//                 }
//                 writeln!(f, " {contents}")
//             }
//             Command::WithString {
//                 name: StringInputs::Label,
//                 ..
//             } => Ok(()),
//             Command::WithString {
//                 name: StringInputs::Cite,
//                 contents,
//             } => write!(f, "[{}]", make_upper_substitute(contents.to_owned())),
//             Command::WithString {
//                 name: StringInputs::Ref,
//                 contents,
//             } => write!(f, "{} 7", make_ref_name(contents.to_owned())),
//             Command::WithText {
//                 name: TextInputs::Font,
//                 contents,
//             } => {
//                 todo!()
//                 // for p in contents.iter() {
//                 //     if line.words.is_empty() {
//                 //         write!(f, "\n\n")?;
//                 //     } else {
//                 //         for word in line.words.iter() {
//                 //             // [FIXME] may produce extra spaces
//                 //             write!(f, "{word}")?;
//                 //         }
//                 //     }
//                 // }
//                 // Ok(())
//             }
//             Command::NoArgs { name: NoArgs::Item } => write!(f, "\n- "),
//         }
//     }
// }

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum CommandName {
//     Section(u8),
//     Label,
//     Cite,
//     Ref,
//     Font,
//     Item,
// }

// impl std::str::FromStr for CommandNames {
//     type Err = StringStreamError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         use CommandNames::*;
//         let ok = match s {
//             "begin" => StringInputCommands(StringInputs::Begin),
//             "end" => StringInputCommands(StringInputs::End),
//             "section" => StringInputCommands(StringInputs::Section(1)),
//             "subsection" => StringInputCommands(StringInputs::Section(2)),
//             "label" => StringInputCommands(StringInputs::Label),
//             "cite" => StringInputCommands(StringInputs::Cite),
//             "ref" | "cref" | "Cref" => StringInputCommands(StringInputs::Ref),
//             "emph" => TextInputCommands(TextInputs::Font),
//             c if c.strip_prefix("text").is_some() => TextInputCommands(TextInputs::Font),
//             "item" => NoArgCommands(NoArgs::Item),
//             _ => return Err(StringStreamError::UnexpectedParse),
//         };
//         Ok(ok)
//     }
// }

// impl CommandName {
//     pub fn arg_num(&self) -> Option<usize> {
//         use CommandName::*;
//         let n = match self {
//             Section(_) | Label | Cite | Ref | Font => 1,
//             Item => 0,
//         };
//         Some(n)
//     }
// }

// pub fn parse_command<Input>() -> impl Parser<Input, Output = Command>
// where
//     Input: Stream<Token = char>,
//     Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//     parse_command_name().then(|name| {
//         if name.has_args() {
//             between(
//                 token('{'),
//                 token('}'),
//                 match name {
//                     CommandNames::StringInputCommands(name) => many1(satisfy(|ch| ch != '}'))
//                         .map(move |s| Command::WithString { name, contents: s })
//                         .left(),
//                     CommandNames::TextInputCommands(name) => parse_lines()
//                         .map(move |contents| Command::WithText { name, contents })
//                         .right(),
//                     CommandNames::NoArgCommands(_) => {
//                         unreachable!("Unreachable in parse_command_name")
//                     }
//                 },
//             )
//             .left()
//         } else {
//             value(name)
//                 .map(|name| Command::NoArgs {
//                     name: name.no_arg(),
//                 })
//                 .right()
//         }
//     })
// }

// // Command call, e.g., `\begin`
// fn parse_command_name<Input>() -> impl Parser<Input, Output = CommandNames>
// where
//     Input: Stream<Token = char>,
//     Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//     attempt(between(
//         (token('\\'), parse_pure_spaces()),
//         parse_pure_spaces(),
//         choice(CommandNames::KEYWORDS.map(|s| attempt(string(s)))),
//     ))
//     .then(|s| match CommandNames::from_str(s) {
//         Ok(cn) => value(cn).left(),
//         Err(_) => unexpected_any("Failed to match a string to a known name.").right(),
//     })
// }

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
