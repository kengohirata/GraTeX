use super::*;
use combine::{
    attempt, between, choice, many, many1, none_of, not_followed_by, parser, parser::char::newline,
    parser::char::string, satisfy, sep_by, sep_end_by, skip_many, token, ParseError, Parser,
    Stream,
};

parser! {
    pub fn parse_lines[Input]()(Input) -> Vec<Line>
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ]{
        sep_by(parse_line(), newline())
    }
}

parser! {
    pub fn parse_line[Input]()(Input) -> Line
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ]{
    (
        parse_pure_spaces(),
        sep_end_by(parse_word(), parse_pure_spaces()),
    )
        .map(|(_, words)| Line {
            words,
            comments: None,
        })
    }
}

parser! {
    fn parse_word[Input]()(Input) -> Word
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ]{
        choice((
            parse_math_display(),
            parse_command(),
            parse_math_inline(),
            parse_text(),
        ))
    }
}

fn parse_text<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    // [WIP] should not read \ or }
    many1(none_of(['$', '\t', '\n', ' '].iter().cloned())).map(Word::Text)
}

// [FIXME] want to call parse_lines inside parse_emph,
// but it reads more than one words, which has incompatible
// return type.
//
// parser! {
//     fn parse_emph[Input]()(Input) -> Word
//     where [
//         Input: Stream<Token = char>,
//         Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
//     ]{
//         attempt(string("emph{")).with(parse_lines())
//     }
// }

fn parse_math_inline<Input>() -> impl Parser<Input, Output = Word>
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
        let mut s_alpha = make_upper_substitute(s);
        s_alpha.push_str(&t);
        Word::MathInline(s_alpha)
    })
}

fn parse_math_display<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    attempt(string("\\["))
        .with(skip_many(
            none_of(['\\'].iter().cloned())
                .or(attempt(token('\\').skip(not_followed_by(token(']'))))),
        ))
        .skip(string("\\]"))
        .map(|_| Word::MathDisplay)
}

fn parse_command<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        between(
            attempt(string("\\begin{")),
            token('}'),
            many1(none_of(['}'].iter().cloned())),
        )
        .map(|s| Word::Command(Command::Begin(s))),
        between(
            attempt(string("\\end{")),
            token('}'),
            many1(none_of(['}'].iter().cloned())),
        )
        .map(|s| Word::Command(Command::End(s))),
    ))
}

fn make_upper_substitute(s: String) -> String {
    let mut s = take_alph_and_to_upper(s);
    if s.len() < 3 {
        for _ in 0..3 - s.len() {
            s.push('X');
        }
    } else {
        s.truncate(3);
    }
    s
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
