use super::token::{Document, Word};

impl Document {
    pub fn arrange(&mut self) {
        let Document { words } = self;
        words.iter_mut().for_each(|l| l.arrange())
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
    *s = s.replace("\\\\", "");
}