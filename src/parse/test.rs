use super::*;

#[test]
fn test_paragraph_from_str() {
    let s = "let $A$\n be $B$   \n and let $C$ be a 2-category.  \n \n\n  aa   \n  ";
    let p = Paragraph::from_str(s);
    dbg!(&p);
    assert!(p.is_ok());
    match p {
        Ok(p) => println!("{}", p),
        Err(_) => unreachable!(),
    }
}

#[test]
fn test_paragraph_display() {
    let l1 = Line {
        words: vec![
            Word::Text("Define".to_string()),
            Word::Math("A".to_string()),
            Word::Text("as".to_string()),
            Word::Math("B".to_string()),
        ],
        comments: None,
    };
    let l2 = Line {
        words: vec![
            Word::Text("Assume".to_string()),
            Word::Math("X".to_string()),
            Word::Text("is".to_string()),
            Word::Math("Y".to_string()),
        ],
        comments: None,
    };
    let p = Paragraph {
        lines: vec![l1, l2],
    };
    println!("{}", p)
}

#[test]
fn test_line() {
    let s = "Define $A$ as $B$\n    ";
    let result = parse_line().parse(s).map(|r| r.0);
    let line = Line {
        words: vec![
            Word::Text("Define".to_string()),
            Word::Math("A".to_string()),
            Word::Text("as".to_string()),
            Word::Math("B".to_string()),
        ],
        comments: None,
    };
    assert_eq!(result, Ok(line))
}

#[test]
fn test_math() {
    let result = parse_math().parse("$A$-alg");
    let math = Word::Math("A-alg".to_string());
    print!("{}", math);
    assert_eq!(result, Ok((math, "")))
}

#[test]
fn test_text() {
    let result = parse_text().parse("abc xyz");
    let text = Word::Text("abc".to_string());
    print!("{}", text);
    assert_eq!(result, Ok((text, " xyz")))
}

#[test]
fn test_space() {
    let s = "    b";
    let result = parse_pure_spaces().parse(s);
    assert_eq!(result, Ok(((), "b")));
    let s = "  
     ";
    let result = parse_pure_spaces().parse(s);
    assert!(result.is_ok())
}
