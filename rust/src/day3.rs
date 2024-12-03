use std::{iter::Peekable, str::Chars};

use itertools::Itertools;

#[derive(Debug)]
enum Token {
    Mul,
    OParen,
    CParen,
    Comma,
    Do,
    Dont,
    Num(i64),
    NotValid,
}
struct Lexer<I: Iterator<Item = char>> {
    src: Peekable<I>,
    curr: usize,
}
impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn new(src: Peekable<I>) -> Self {
        Self { src, curr: 0 }
    }
    fn peek(&self) {}
}
impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(letter) = self.src.next() {
            match letter {
                ',' => Some(Token::Comma),
                '(' => Some(Token::OParen),
                ')' => Some(Token::CParen),
                'm' => match (self.src.next(), self.src.next()) {
                    (Some('u'), Some('l')) => Some(Token::Mul),
                    (_, _) => Some(Token::NotValid),
                },
                _ => {
                    if letter.is_ascii_digit() {
                        let mut number = String::from(letter);
                        while let Some(x) = self.src.peek() {
                            if x.is_ascii_digit() {
                                if let Some(y) = self.src.next() {
                                    number.push(y);
                                }
                            } else {
                                break;
                            }
                        }
                        Some(Token::Num(number.parse().unwrap()))
                    } else if letter == 'd' {
                        if let Some('o') = self.src.next() {
                            if let Some('n') = self.src.peek() {
                                match (self.src.next(), self.src.next(), self.src.next()) {
                                    (Some('n'), Some('\''), Some('t')) => Some(Token::Dont),
                                    (_, _, _) => Some(Token::NotValid),
                                }
                            } else {
                                Some(Token::Do)
                            }
                        } else {
                            Some(Token::NotValid)
                        }
                    } else {
                        Some(Token::NotValid)
                    }
                }
            }
        } else {
            None
        }
    }
}
fn non_strict_parser(tokens: Vec<Token>) -> i64 {
    let mut tok = tokens.iter();
    let mut total = 0;

    while let Some(token) = tok.next() {
        if let Token::Mul = token {
            if let Some(Token::OParen) = tok.next() {
                if let Some(Token::Num(left)) = tok.next() {
                    if let Some(Token::Comma) = tok.next() {
                        if let Some(Token::Num(right)) = tok.next() {
                            if let Some(Token::CParen) = tok.next() {
                                total += left * right;
                            }
                        }
                    }
                }
            }
        }
    }
    total
}
fn strict_parser(tokens: Vec<Token>) -> i64 {
    let mut tok = tokens.iter();
    let mut total = 0;
    let mut dos = true;

    while let Some(token) = tok.next() {
        if let Token::Mul = token {
            if let Some(Token::OParen) = tok.next() {
                if let Some(Token::Num(left)) = tok.next() {
                    if let Some(Token::Comma) = tok.next() {
                        if let Some(Token::Num(right)) = tok.next() {
                            if let Some(Token::CParen) = tok.next() {
                                if dos {
                                    total += left * right;
                                }
                            }
                        }
                    }
                }
            }
        } else if let Token::Do = token {
            dos = true;
        } else if let Token::Dont = token {
            dos = false;
        }
    }
    total
}

pub fn part1() {
    let text = std::fs::read_to_string("../day3.txt").unwrap();
    let lexer = Lexer::new(text.chars().peekable());
    let tokens = lexer.collect::<Vec<_>>();

    let ans = non_strict_parser(tokens);
    println!("{:?}", ans)
}
pub fn part2() {
    let text = std::fs::read_to_string("../day3.txt").unwrap();
    let lexer = Lexer::new(text.chars().peekable());
    let tokens = lexer.collect::<Vec<_>>();

    let ans = strict_parser(tokens);
    println!("{:?}", ans)
}
