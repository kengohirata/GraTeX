use std::iter::Peekable;

use crate::ast;
use crate::token;

use super::command::token_to_ast_command;
use super::Ast;

type PeekableWords = Peekable<Box<dyn Iterator<Item = token::Word>>>;

pub fn token_to_ast(doc: token::Document) -> ast::Ast {
    let words_iter: Box<dyn Iterator<Item = token::Word>> = Box::new(doc.words.into_iter());
    let mut words: PeekableWords = words_iter.peekable();
    let mut ast = ast::Ast::new();
    let mut paragraph = ast::Paragraph::new();

    // let dummy_vec: &mut dyn Iterator<Item = token::Word> = &mut vec![].into_iter();
    // let dummy: &RefCell<PeekableWords> = &RefCell::new(dummy_vec.peekable());

    while let Some(word) = words.next() {
        match word {
            token::Word::Text(s) => {
                paragraph.push(ast::Word::Text(s));
            }
            token::Word::Command(c) => {
                let mut args = Vec::new();
                match c.arg_num() {
                    Some(n) => {
                        let mut iter: Box<dyn Iterator<Item = token::Word>> = Box::new(words);
                        for _ in 0..n {
                            iter =
                                Box::new(iter.skip_while(|w| !matches!(w, token::Word::Lines(_))));
                            if let Some(token::Word::Lines(doc)) = iter.next() {
                                args.push(doc);
                            } else {
                                eprintln!("Error: Command {:?} needs {} arguments", c, n);
                            }
                        }
                        words = iter.peekable();
                    }
                    None => {
                        while let Some(token::Word::Lines(doc)) =
                            words.next_if(|w| matches!(w, token::Word::Lines(_)))
                        {
                            args.push(doc);
                        }
                    }
                }
                if let Some(w) = token_to_ast_command(c, args) {
                    paragraph.push(w);
                }

                // while let Some(token::Word::Text(s)) = words.peek() {
                //     args.push(words.next().unwrap());
                // }
                // paragraph.push(doc_to_ast_command(c, args));
            }
            token::Word::Lines(doc) => {
                let Ast(mut ps) = token_to_ast(doc);
                if ps.len() > 1 {
                    let first_p = ps.remove(0);
                    paragraph.0.extend(first_p.0);
                    ast.0.push(paragraph);
                    paragraph = match ps.pop() {
                        Some(p) => p,
                        None => ast::Paragraph::new(),
                    };
                    ast.0.extend(ps);
                } else if ps.len() == 1 {
                    let new_p = ps.remove(0);
                    paragraph.0.extend(new_p.0);
                }
            }
            token::Word::Comment(_) => continue,
            token::Word::Env(s, d) => {
                paragraph.0.push(ast::Word::Env(s, token_to_ast(d)));
            }
            token::Word::Dollar => {
                let mut s = String::new();
                while let Some(word) = words.next_if(|w| !matches!(w, token::Word::Dollar)) {
                    s += &format!("{word}");
                }
                assert_eq!(words.next(), Some(token::Word::Dollar));
                let s = ast::Word::MathInline(ast::make_upper_substitute(s));
                paragraph.0.push(s);
            }
            token::Word::EndLine => {
                if matches!(words.peek(), Some(token::Word::EndLine)) {
                    ast.push(paragraph);
                    paragraph = ast::Paragraph::new();
                    // words = dummy.replace(words);
                    let ws: Box<dyn Iterator<Item = token::Word>> =
                        Box::new(words.skip_while(|w| matches!(w, token::Word::EndLine)));
                    words = ws.peekable();
                }
            }
        }
    }
    if paragraph.0.len() > 0 {
        ast.push(paragraph);
    }
    ast
}

#[test]
fn test_doc_to_ast() {
    use std::str::FromStr;
    // let s = "Hello\n$ hi $ World!\n{I am}\n{super \n\n man}\\ref[]{cor:21}";
    let s = "a \\emph{b} c";
    let doc = token::Document::from_str(s).unwrap();
    println!("{}", doc);
    let ast = token_to_ast(doc);
    println!("{:?}", ast);
    println!("{}", ast);
}
