use std::rc::Rc;
use node::{Prim, Node, Bool, rint, rcar, rcdar, rcddar, rcdr, rsym, rcell, rquote};
use env::Env;
use evaluator::*;
use error::EvalError;

pub fn prim_let(renv: &mut Env<Node>, args: &Node) -> EvalResult {
    let let_args = try!(rcar(args));
    let body = try!(rcdar(args));
    let (aargs, vargs) = transform(&let_args);
    let lambda = Node::Prim(Prim::Lambda(renv.clone(), Rc::new(vargs), Rc::new(body)));
    eval(renv, &rcell(rquote(lambda), aargs))
}

fn transform(node: &Node) -> (Node, Node) {
    match node {
        &Node::Cell(ref v, ref r) => {
            let nv = rcar(v).unwrap();
            let nr = rcdar(v).unwrap();
            let (ar, vr) = transform(r);
            (rcell(nr, ar), rcell(nv, vr))
        }
        _ => (Node::Nil, Node::Nil)
    }
}

pub fn prim_lambda(renv: &mut Env<Node>, args: &Node) -> EvalResult {
    let lambda_args = try!(rcar(args));
    let body = rcell(rsym("progn"), try!(rcdr(args)));

    Ok(Node::Prim(Prim::Lambda(renv.clone(), Rc::new(lambda_args), Rc::new(body))))
}

pub fn prim_if(renv: &mut Env<Node>, args: &Node) -> EvalResult {
    let cond = try!(rcar(args).and_then( |ref v| eval(renv, v) ));
    let clause = if cond == Node::Bool(Bool::False) {
        rcddar(args)
    } else {
        rcdar(args)
    };
    clause.and_then( |ref v| eval(renv, v))
}

pub fn prim_progn(renv: &mut Env<Node>, args: &Node) -> EvalResult {
    match *args {
        Node::Cell(ref car, ref cdr) => {
            let ret = try!(eval(renv, car));
            let rest = try!(prim_progn(renv, cdr));
            Ok(if rest == Node::Nil { ret } else  { rest })
        }
        ref x => Ok(x.clone())
    }
}

pub fn prim_quote(_: &mut Env<Node>, args: &Node) -> EvalResult {
    rcar(args)
}

pub fn prim_define(renv: &mut Env<Node>, args: &Node) -> EvalResult {
    match *args {
        Node::Cell(ref car, ref cdr) => {
            if let Node::Sym(ref s) = **car {
                let ccdr = try!(rcar(cdr));
                let ret = try!(eval(renv, &ccdr));
                renv.register(s.to_string(), ret.clone());
                return Ok(ret);
            }
        },
        _ => ()
    }
    Err(EvalError::E)
}

pub fn prim_sub(renv: &mut Env<Node>, args: &Node) -> EvalResult {
    let lst = try!(eval_list(renv, args));
    match lst {
        Node::Cell(ref car, ref cdr) => {
            if let Node::Int(base) = **car {
                return Ok(rint(do_sub(base, cdr)))
            }
        },
        _ => ()
    }

    Err(EvalError::E)
}

pub fn prim_add(renv: &mut Env<Node>, args: &Node) -> EvalResult {
    Ok(rint(do_add(&try!(eval_list(renv, args)))))
}

fn do_sub(base: i32, lst: &Node) -> i32 {
    match *lst {
        Node::Cell(ref car, ref cdr) => {
            if let Node::Int(k) = **car {
                do_sub(base - k, cdr)
            } else {
                base
            }
        },
        _ => base
    }
}

fn do_add(lst: &Node) -> i32 {
    match *lst {
        Node::Cell(ref car, ref cdr) => {
            if let Node::Int(k) = **car {
                k + do_add(cdr)
            } else {
                0
            }
        },
        _ => 0
    }
}
