use std::result;
use node::{Node, rcell, rnil};
use env::Env;

pub type Result<T> = result::Result<T, RError>;

#[derive(Debug, Clone)]
pub enum RError {
    E,                     // must be fix
    UnknowSymbol,
    WrongTypeArg
}

fn apply(renv: &mut Env<Node>, fun: &Node, args: &Node) -> Result<Node> {
    match *fun {
        Node::Prim(ref prim) => prim(renv, args),
        _ => Err(RError::UnknowSymbol)
    }
}

pub fn eval_list(renv: &mut Env<Node>, ast: &Node) -> Result<Node> {
    match *ast {
        Node::Cell(ref car, ref cdr) => Ok(rcell(try!(eval(renv, car)), try!(eval_list(renv, cdr)))),
        Node::Nil => Ok(rnil()),
        _ => Err(RError::E),
    }
}

pub fn eval(renv: &mut Env<Node>, ast: &Node) -> Result<Node> {    // specific type
    match *ast {
        Node::Int(_) => Ok(ast.clone()),
        Node::Cell(ref car, ref cdr) => {
            let f = try!(eval(renv, car));
            apply(renv, &f, cdr)
        }
        Node::Sym(ref v) => match renv.find(v) {
            Some(k) => Ok(k.clone()),
            None => Err(RError::UnknowSymbol)
        },
        _ => Err(RError::E)
    }
}
