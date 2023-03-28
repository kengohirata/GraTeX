use super::Ast;

pub fn write_env(f: &mut std::fmt::Formatter<'_>, env: &str, ast: &Ast) -> std::fmt::Result {
    match env {
        "itemize" | "enumerate" => write!(f, "{ast}"),
        _ if env.starts_with("align") | env.starts_with("equ") => writeln!(f, "\n%%%% MATH %%%%"),
        _ => {
            writeln!(f, "\n% ---------- \\begin: {env} ----------")?;
            write!(f, "{ast}")?;
            writeln!(f, "\n% ---------- \\end: {env} ----------")
        }
    }
}
