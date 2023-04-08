pub fn preprocess(s: &mut String) {
    if let Some(start) = s.find("\\begin{document}") {
        if let Some(end) = s.find("\\end{document}") {
            *s = s[start + 16..end].to_string();
        }
    }
    // println!("{s}")
}
