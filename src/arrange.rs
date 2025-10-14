use regex::Regex;

pub fn arrange_text_string(s: &mut String) {
    // many spaces => single space
    let re = Regex::new(r"[ \t]+").unwrap();
    *s = re.replace_all(s, " ").to_string();

    // delete space at the start of line
    *s = s.replace("\n ", "\n");

    // more than three `\n` => `\n\n`
    let re = Regex::new(r"\n\n\n+").unwrap();
    *s = re.replace_all(s, "\n\n").to_string();

    // delete spaces before
    // `,`
    // `.`
    // `-`
    // `]`
    // `)`
    let re = Regex::new(r" ([,\.\-\]\)])").unwrap();
    *s = re.replace_all(s, "$1").to_string();

    // delete spaces after
    // '['
    // `(`
    // `-`
    let re = Regex::new(r"([\(\[\-]) ").unwrap();
    *s = re.replace_all(s, "$1").to_string();

    // delete space before `'` and `"`
    let re = Regex::new(" ([\'\"])").unwrap();
    *s = re.replace_all(s, "$1").to_string();

    // delete `~` followed by space
    *s = s.replace("~ ", " ");

    // replace `` or '' with "
    *s = s.replace("``", "\"");
    *s = s.replace("''", "\"");
}

#[cfg(test)]
mod tests {
    use super::arrange_text_string;
    use rstest::rstest;
    #[rstest]
    #[case("  Hello   World!  ", " Hello World! ")]
    #[case("Hello\n\n\n\nWorld!", "Hello\n\nWorld!")]
    #[case(" - - -", "---")]
    #[case(" , , , ", ",,, ")]
    #[case("K~ B", "K B")]
    #[case("Kengo's", "Kengo's")]
    #[case("Kengo 's", "Kengo's")]
    #[case("``Hello''", "\"Hello\"")]
    #[case(" ( Hello ) ", " (Hello) ")]
    fn test_arrange_text_string(#[case] input: &str, #[case] expected: &str) {
        let mut s = input.to_string();
        arrange_text_string(&mut s);
        assert_eq!(s, expected);
    }
}
