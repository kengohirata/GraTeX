// use super::command::parse_command;
use super::{command::Command, *};
use combine::{
    attempt, between, choice, many, many1, none_of, parser,
    parser::char::{letter, string},
    satisfy, sep_end_by, token, unexpected_any, value, ParseError, Parser, Stream,
};

parser! {
    pub fn parse_words[Input]()(Input) -> Vec<Word>
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ]{
        parse_pure_spaces()
            .with(sep_end_by(parse_word(), parse_pure_spaces()))
    }
}

pub fn parse_word<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    parse_pure_spaces().with(choice((
        parse_env(),
        between(token('{'), token('}'), parse_words()).map(|words| Word::Lines(Document { words })),
        parse_math_display(),
        parse_math_inline(),
        parse_comments().map(Word::Comment),
        parse_command().map(Word::Command),
        parse_text(),
        parse_endl(),
    )))
}

pub fn parse_env<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        attempt(between(
            string("\\begin{"),
            token('}'),
            many1(satisfy(|ch| ch != '}')),
        )),
        parse_words(),
        attempt(between(
            string("\\end{"),
            token('}'),
            many1(satisfy(|ch| ch != '}')),
        )),
    )
        .map(|(begin, contents, _end): (String, Vec<Word>, String)| {
            //[TODO] Check if begin and end are the same
            Word::Env(begin, Document { words: contents })
        })
}

pub fn parse_math_display<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(
        attempt(string("\\[")),
        attempt(string("\\]")),
        parse_words(),
    )
    .map(|words: Vec<Word>| Word::Env("equation".to_string(), Document { words }))
}

pub fn parse_math_inline<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    token('$').map(|_| Word::Dollar)
}

fn parse_comments<Input>() -> impl Parser<Input, Output = Comments>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    token('%')
        .with(many(satisfy(|ch| ch != '\n')))
        .map(Comments)
}

fn parse_command<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(token('\\').with(choice((
        many1(letter()).then(|s: String| match &s as &str {
            "begin" => unexpected_any("begin").right(),
            "end" => unexpected_any("end").right(),
            _ => value(s).map(|s| match Command::from_str(&s) {
                Ok(c) => c,
                Err(_) => Command::Unknown(s),
            }).left(),
        }),
        none_of(['[', ']'].iter().cloned()).map(Command::Symbol),
    ))))
}

fn parse_text<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((many1(none_of(
        ['$', '\t', '\n', ' ', '{', '}', '%', '\\'].iter().cloned(),
    ))
    .map(Word::Text),))
}

pub fn parse_pure_spaces<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many(satisfy(|c: char| c != '\n' && c.is_whitespace())).map(|_: String| ())
}

pub fn parse_endl<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    token('\n').map(|_| Word::EndLine)
}
