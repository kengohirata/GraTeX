use super::*;

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
            parse_math_inline(),
            parse_text(),
            token('\\').with(
                parse_math_display(),
            ),
        ))
    }
}

parser! {
    fn parse_text[Input]()(Input) -> Word
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    ]{
        many1(none_of(['$', '\t', '\n', ' ', '\\'].iter().cloned())).map(Word::Text)
    }
}

pub fn parse_math_inline<Input>() -> impl Parser<Input, Output = Word>
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

pub fn parse_math_display<Input>() -> impl Parser<Input, Output = Word>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    token::<Input>('[')
        .with(skip_many(
            none_of(['\\'].iter().cloned())
                .or(attempt(token('\\').skip(not_followed_by(token(']'))))),
        ))
        .skip(string("\\]"))
        .map(|_| Word::MathDisplay)
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
