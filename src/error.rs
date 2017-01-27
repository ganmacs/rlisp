use std::error;
use std::fmt;

pub type RResult<T, E> where E: error::Error = Result<T, E>;

#[derive(Debug, PartialEq)]
    pub enum RLispError {
    EvalError(EvalError),
    ParseError(ParseError)
}

impl fmt::Display for RLispError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RLispError::EvalError(ref e) => write!(f, "Eval Error: {}", e),
            RLispError::ParseError(ref e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl error::Error for RLispError {
    fn description(&self) -> &str {
        match *self {
            RLispError::EvalError(ref e) => e.description(),
            RLispError::ParseError(ref e) => e.description(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidSyntax(u32),
    UnmatchedParen(u32),
    RequireString(u32)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidSyntax(ref p) => write!(f, "Invalid Syntax as: {}", p),
            ParseError::UnmatchedParen(ref p) => write!(f, "Unmatched Paren at {}", p),
            ParseError::RequireString(ref p) => write!(f, "Requred Charater at {}", p),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        ""
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    E,                     // must be fix
    UnknowSymbol(String),
    InvalidArgNumber,
    WrongTypeArg
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvalError::E => write!(f, "eval error must be fix"),
            EvalError::UnknowSymbol(ref s) => write!(f, "Unknow symbol: {}", s),
            EvalError::InvalidArgNumber => write!(f, "Invalid argument number"),
            EvalError::WrongTypeArg => write!(f, "Wrong type argument"),
        }
    }
}

impl error::Error for EvalError {
    fn description(&self) -> &str {
        ""
    }
}
