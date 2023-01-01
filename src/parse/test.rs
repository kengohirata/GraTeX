use super::word::*;
use super::*;

#[test]
fn test_paragraph_from_str() {
    let s = "let $A$\n be $B$.   \n and let $C$ be a 2-category.  \n \n\n  aa \\[ y \\]  \n  ";
    let p = Paragraph::from_str(s);
    // dbg!(&p);
    assert!(p.is_ok());
    match p {
        Ok(p) => println!("{}", p),
        Err(_) => unreachable!(),
    }
}

// #[test]
// fn test_math_display() {
//     let result = parse_m().parse("[a\\a\\]");
//     print!("{:?}", result);
//     let math = Word::MathDisplay;
//     assert_eq!(result, Ok((math, "")))
// }

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
