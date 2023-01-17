use super::parse::{Line, Paragraph, Word};

impl Paragraph {
    pub fn arrange(&mut self) {
        let Paragraph { lines } = self;
        lines.iter_mut().for_each(|l| l.arrange())
    }
}

impl Line {
    pub fn arrange(&mut self) {
        let Line { words, .. } = self;
        words.iter_mut().for_each(|w| w.arrange());
        words.retain(|w| !w.is_empty_word());
    }
}

impl Word {
    pub fn arrange(&mut self) {
        match self {
            Word::Text(s) => arrange_text_string(s),
            _ => (),
        }
    }
}

fn arrange_text_string(s: &mut String) {
    *s = s.replace("\\@", "");
    *s = s.replace("\\\'", "");
}
