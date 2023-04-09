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
    // `]`
    *s = s.replace(" ,", ",");
    *s = s.replace(" .",  ".");
    *s = s.replace(" ]",  "]");

    // delete spaces after '['
    *s = s.replace("[ ",  "[");

    // delete space before and after `'` and `"`
    let re = Regex::new(" ?['\"] ?").unwrap();
    *s = re.replace_all(s, "").to_string();
}
