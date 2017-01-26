use std::rc::Rc;
use std::iter;
use std::str;
use node;
use node::Node;

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
    pub fn to_str(&self) ->  &'static str {
        match *self {
            ParseError::InvalidSyntax => "Invalid Syntax",
            ParseError::UnmatchedParen => "Unmatched Paren"
        }
    }
}

fn read_quote(lexer: &mut Lexer) -> ParseResult {
    let v = try!(read(lexer));
    Ok(node::rcell(Node::Sym("quote".to_string()), v))
}

fn read_list(lexer: &mut Lexer) -> ParseResult {
    match lexer.peek_no_whitespace() {
        None => Err(ParseError::UnmatchedParen),
        Some(')') => Ok(Node::Nil),
        _ => {
            let car = try!(read(lexer));
            let cdr = try!(read_list(lexer));
            Ok(Node::Cell(Rc::new(car), Rc::new(cdr)))
        },
    }
}

fn read_int(c: char, lexer: &mut Lexer) -> ParseResult {
    let radix = 10;
    let v = &mut String::new();
    v.push(c);

    while let Some(n) = lexer.peek() {
        if !n.is_digit(radix) {
            break;
        }
        v.push(n);
        lexer.next();
    }

    Ok(Node::Int(i32::from_str_radix(&v, radix).unwrap()))
}

fn read_number(lexer: &mut Lexer, c: char) ->  ParseResult {
    read_int(c, lexer)
}

fn read_symbol(lexer: &mut Lexer, c: char) ->  ParseResult {
    let v = &mut String::new();
    v.push(c);

    while let Some(c) = lexer.peek() {
        if !is_ident(c) {
            break;
        }
        v.push(c);
        lexer.next();
    }

    Ok(Node::Sym(v.to_owned()))
}

fn is_ident(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '=' | '<' | '>' |
        '+' | '-' | '/' | '%' => true,
        _ => false
    }
}

fn read(lexer: &mut Lexer) ->  ParseResult {
    match lexer.next_no_whitespace() {
        None => Ok(Node::Nil),
        Some(c) =>
            match c {
                '(' => read_list(lexer),
                '\'' => read_quote(lexer),
                '0'...'9' => read_number(lexer, c),
                _ => read_symbol(lexer, c)
            }
    }
}

pub fn parse<T: Into<String>>(input: T) -> ParseResult {
    read(&mut Lexer::new(input.into().chars().peekable()))
}
