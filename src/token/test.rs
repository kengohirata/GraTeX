use super::*;

#[test]
fn test_paragraph_from_str() {
    let s = 
    "\\begin{a}\\begin{b}  \\begin{c} $ yeah$ \\end{c} %aiueo
    \\hello\\ \\bye
    \\begin{d} \\end{d}\\end{b}\\end{a} %aa";
    let p = Document::from_str(s);
    // dbg!(&p);
    assert!(p.is_ok());
    match p {
        Ok(p) => println!("{p}"),
        Err(_) => unreachable!(),
    }

    // let s = "let $A$\n be $B$. \n \n\n  \\todo{ }  \\[ y \\] \\begin{align}  \n \\end{align}  ";
    // let p = Document::from_str(s);
    // // dbg!(&p);
    // assert!(p.is_ok());
    // match p {
    //     Ok(p) => println!("{p}"),
    //     Err(_) => unreachable!(),
    // }
}

#[test]
fn test_debug() {
    let s = 
    r"\arr[r]
    I am a Ph\@. D\@. student";
    let p = Document::from_str(s);
    dbg!(&p);
    assert!(p.is_ok());
    match p {
        Ok(p) => println!("{p}"),
        Err(_) => unreachable!(),
    }
}
#[test]
fn test_debug2() {
    let s = 
    r"\textbf{a}
    \\\ \+\[
        yeah
    \]";
    let p = Document::from_str(s);
    dbg!(&p);
    assert!(p.is_ok());
    match p {
        Ok(p) => println!("{p}"),
        Err(_) => unreachable!(),
    }
}

// #[test]
// fn test_text() {
//     let result = parse_text().parse("abc xyz");
//     let text = Word::Text("abc".to_string());
//     print!("{}", text);
//     assert_eq!(result, Ok((text, " xyz")))
// }

// #[test]
// fn test_space() {
//     let s = "    b";
//     let result = parse_pure_spaces().parse(s);
//     assert_eq!(result, Ok(((), "b")));
//     let s = "
//      ";
//     let result = parse_pure_spaces().parse(s);
//     assert!(result.is_ok())
// }

// #[test]
// fn test_make_upper_substitute() {
//     let s = "abcd".to_string();
//     let subst = make_upper_substitute(s);
//     assert_eq!(subst, "ABC".to_string());

//     let s = "ab".to_string();
//     let subst = make_upper_substitute(s);
//     assert_eq!(subst, "ABX".to_string());

//     let s = "--a--".to_string();
//     let subst = make_upper_substitute(s);
//     assert_eq!(subst, "AXX".to_string());
// }
