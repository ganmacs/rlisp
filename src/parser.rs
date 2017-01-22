use std::iter;
use std::str;
use super::Node;

struct Lexer<'a> {
    input: iter::Peekable<str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: iter::Peekable<str::Chars<'a>>) -> Lexer<'a> {
        Lexer { input: input }
    }

    pub fn next(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn next_no_whitespace(&mut self) -> Option<char> {
        self.comsume_whitespace();
        self.input.next()
    }

    pub fn peek(&mut self) -> Option<char> {
        self.input.peek().map( |c| c.to_owned() )
    }

    // Need to remove side-effect ?
    pub fn peek_no_whitespace(&mut self) -> Option<char> {
        self.comsume_whitespace();
        self.input.peek().map( |c| c.to_owned() )
    }

    fn comsume_whitespace(&mut self) {
        while self.peek().map( |c| c.is_whitespace() ).unwrap_or(false) {
            self.input.next();
        }
    }
}

pub type ParseResult = Result<Node, ParseError>;

#[derive(Debug)]
pub enum ParseError {
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

pub fn parse(input: &str) -> ParseResult {
    read(&mut Lexer::new(input.chars().peekable()))
}

fn read(input: &mut Lexer) ->  ParseResult {
    match input.next_no_whitespace() {
        None => Ok(Node::Nil),
        Some(c) =>
            match c {
                '(' => read_list(input),
                '+' => Ok(Node::Fn { name: "+" }),
                '0'...'9' => read_number(c, input),
                _ => {
                    Err(ParseError::InvalidSyntax)
                }
            }
    }
}

fn read_list(input: &mut Lexer) -> ParseResult {
    match input.peek_no_whitespace() {
        None => Err(ParseError::UnmatchedParen),
        Some(')') => Ok(Node::Nil),
        _ => {
            let car = try!(read(input));
            let cdr = try!(read(input));
            Ok(Node::Cell(Box::new(car), Box::new(cdr)))
        },
    }
}

fn read_int(c: char, input: &mut Lexer) -> ParseResult {
    let mut v = String::new();
    v.push(c);

    while let Some(n) = input.peek() {
        match n {
            '0'...'9' => {
                v.push(n);
                input.next();
            },
            _ => return Err(ParseError::InvalidSyntax),
        }
    }

    Ok(Node::Int(i32::from_str_radix(&v, 10).unwrap()))
}

fn read_number(c: char, input: &mut Lexer) ->  ParseResult {
    read_int(c, input)
}
