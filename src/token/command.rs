use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Section(u8),
    Label,
    Cite,
    Ref,
    Font,
    Item,
    Space,
    Unknown(String),
}

impl Command {
    pub const KEYWORDS: [&str; 17] = [
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
        "quad",
        "qquad",
        ",",
        "!",
        " ",
    ];

    pub fn arg_num(&self) -> Option<usize> {
        use Command::*;
        let n = match self {
            Section(_) | Label | Cite | Ref | Font => 1,
            Item | Space => 0,
            Unknown(_) => return None,
        };
        Some(n)
    }
}

impl std::str::FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;
        let ok = match s {
            "section" => Section(1),
            "subsection" => Section(2),
            "label" => Label,
            "cite" => Cite,
            "ref" | "cref" | "Cref" => Ref,
            "emph" => Font,
            c if c.strip_prefix("text").is_some() => Font,
            "item" => Item,
            "quad" | "qquad" | "," | "!" | " " => Space,
            _ => return Err(format!("Compiler BUG: Unknown command name found: {s}")),
        };
        Ok(ok)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Section(n) => {
                for _ in 0..*n {
                    write!(f, "#")?
                }
                Ok(())
            }
            Command::Label => write!(f, "\\LABEL"),
            Command::Cite => write!(f, "\\CITE"),
            Command::Ref => write!(f, "\\REF"),
            Command::Font => write!(f, "\\FONT"),
            Command::Item => write!(f, "\\ITEM"),
            Command::Space => write!(f, ""),
            Command::Unknown(s) => write!(f, "\\{}", s.to_uppercase()),
        }
    }
}
