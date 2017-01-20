use std::iter::Peekable;
use std::str::Chars;
use node::Node;

pub type ParseResult = Result<Node, ParseError>;

#[derive(Debug)]
enum ParseError {
    InvalidSyntax,
    UnmatchedParen
}

impl ParseError {
    pub fn to_str(self) ->  &'static str {
        match self {
            ParseError::InvalidSyntax => "Invalid Syntax",
            ParseError::UnmatchedParen => "Unmatched Paren"
        }
    }
}

pub fn parse(input: &'static str) -> ParseResult {
    do_parse(&mut input.chars().peekable())
}

fn do_parse(input: &mut Peekable<Chars>) ->  ParseResult {
    match input.next() {
        None => Ok(Node::Nil),
        Some(c) =>
            match c {
                '(' => {
                    if Some(&')') == input.peek() {
                        return Ok(Node::Nil);
                    }
                    parse_list(input)
                },
                ' ' => do_parse(input),
                ')' => Ok(Node::RParen),
                '+' => Ok(Node::Fn { name: "+" }),
                x if x.is_digit(10) => parse_digit(x, input),
                _ => {
                    Err(ParseError::InvalidSyntax)
                }
            }
    }
}

fn parse_list(input: &mut Peekable<Chars>) -> ParseResult {
    let v = try!(do_parse(input));

    match v {
        Node::Nil => return Err(ParseError::UnmatchedParen),
        Node::Dot => return Err(ParseError::UnmatchedParen),
        Node::RParen => return Ok(Node::Nil),
        _ => ()
    };

    let k = try!(parse_list(input));

    Ok(Node::Cell {
        car: Box::new(v),
        cdr: Box::new(k)
    })
}

fn number(n: u32, input: &mut Peekable<Chars>) -> Node {
    let mut v = n;
    if let Some(x) = input.peek() {
        if x.is_digit(10) {
            v = v * 10 + x.to_digit(10).unwrap();
        }
    }
    Node::Int { v: v as i32 }
}

fn parse_digit(c: char, input: &mut Peekable<Chars>) ->  ParseResult {
    Ok(number(c.to_digit(10).unwrap(), input))
}
