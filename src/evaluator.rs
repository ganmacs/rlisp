use std::result;
use node::{Prim, Node, rcell, rnil, rcar, rcdr};
use env::Env;

pub type Result<T> = result::Result<T, RError>;

#[derive(Debug, Clone)]
pub enum RError {
    E,                     // must be fix
    UnknowSymbol(String),
    InvalidArgNumber,
    WrongTypeArg
}

fn apply_function(renv: &mut Env<Node>, args: (&Node, &Node), body: &Node) -> Result<Node> {
    try!(register_all(renv, args.0, args.1));
    eval(renv, body)
}

fn register_all(renv: &mut Env<Node>, keys: &Node, values: &Node) -> Result<Node> {
    match (keys, values) {
        (&Node::Nil, &Node::Nil) => Ok(Node::Nil),
        (&Node::Nil, _) | (_, &Node::Nil)  => Err(RError::InvalidArgNumber),
        (_, _) => {
            match try!(rcar(keys)) {
                Node::Sym(key) => {
                    renv.register(key, try!(rcar(values)));
                    register_all(renv, &try!(rcdr(keys)), &try!(rcdr(values)))
                },
                _ => Err(RError::WrongTypeArg),
            }
        }
    }
}

fn apply(renv: &mut Env<Node>, fun: &Node, args: &Node) -> Result<Node> {
    match *fun {
        Node::Prim(ref prim) => match prim {
            &Prim::Proc(ref f) => f(renv, args),
            &Prim::Lambda(ref v, ref a, ref body)  => {
                let new_env = &mut v.clone();
                new_env.push_local_scope();
                let ret = apply_function(new_env, (a, &try!(eval_list(renv, args))), body);
                new_env.pop_local_scope();
                ret
            },
        },
        _ => Err(RError::UnknowSymbol(format!("{:?}", fun)))
    }
}

pub fn eval_list(renv: &mut Env<Node>, ast: &Node) -> Result<Node> {
    match *ast {
        Node::Cell(ref car, ref cdr) => Ok(rcell(try!(eval(renv, car)), try!(eval_list(renv, cdr)))),
        Node::Nil => Ok(rnil()),
        _ => Err(RError::E),
    }
}

pub fn eval(renv: &mut Env<Node>, ast: &Node) -> Result<Node> {
    match *ast {
        Node::Int(_) | Node::Bool(_) | Node::Nil => Ok(ast.clone()),
        Node::Cell(ref car, ref cdr) => {
            let f = try!(eval(renv, car));
            apply(renv, &f, cdr)
        }
        Node::Sym(ref v) => match renv.find(v) {
            Some(k) => Ok(k.clone()),
            None => Err(RError::UnknowSymbol(v.to_owned()))
        },
        _ => Err(RError::E)
    }
}
