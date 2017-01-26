use std::result;
use node::{Prim, Node, rcell, rnil, rcar, rcdr};
use env::Env;

pub type Result<T> = result::Result<T, RError>;

#[derive(Debug, Clone)]
pub enum RError {
    E,                     // must be fix
    UnknowSymbol(String),
    WrongTypeArg
}

fn apply_function(renv: &mut Env<Node>, args: (&Node, &Node), body: &Node) -> Result<Node> {
    register_all(renv, args.0, args.1);
    eval(renv, body)
}

fn register_all(renv: &mut Env<Node>, keys: &Node, values: &Node) {
    match (keys, values) {
        (&Node::Nil, &Node::Nil) => (),
        (&Node::Nil, _) => panic!("Invalid argument number"),
        (_, &Node::Nil) => panic!("Invalid argument number"),
        (_, _) => {
            let k = rcar(keys).unwrap(); // TODO fix
            let v = rcar(values).unwrap();
            match k {
                Node::Sym(key) => renv.register(key, v),
                _ => panic!("Invalid  token"),
            };
            register_all(renv, &rcdr(keys).unwrap(), &rcdr(values).unwrap());
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
                let ret = apply_function(new_env, (a, args), body);
                println!("{:?}", ret);
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

pub fn eval(renv: &mut Env<Node>, ast: &Node) -> Result<Node> {    // specific type
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
