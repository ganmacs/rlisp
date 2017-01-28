use std::rc::Rc;
use node::{Prim, Node, Bool, rint, rcar, rcdar, rcddar, rcdr, rsym, rcell, rquote, rbool};
use env::Env;
use evaluator::*;
use error::EvalError;

pub fn prim_let(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
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

pub fn prim_lambda(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let lambda_args = try!(rcar(args));
    let body = rcell(rsym("progn"), try!(rcdr(args)));

    Ok(Node::Prim(Prim::Lambda(renv.clone(), Rc::new(lambda_args), Rc::new(body))))
}

pub fn prim_if(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let cond = try!(rcar(args).and_then( |ref v| eval(renv, v) ));
    let clause = if cond == Node::Bool(Bool::False) {
        rcddar(args)
    } else {
        rcdar(args)
    };
    clause.and_then( |ref v| eval(renv, v))
}

pub fn prim_progn(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    match *args {
        Node::Cell(ref car, ref cdr) => {
            let ret = try!(eval(renv, car));
            let rest = try!(prim_progn(renv, cdr));
            Ok(if rest == Node::Nil { ret } else  { rest })
        }
        ref x => Ok(x.clone())
    }
}

pub fn prim_quote(_: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    rcar(args)
}

pub fn prim_define(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
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

pub fn prim_eq(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let ref eargs = try!(eval_list(renv, args));
    let ref car = try!(rcar(eargs));
    let ref cdr = try!(rcdr(eargs));
    let ref fun = |x, y| x == y;
    let ret = try!(do_cmp(fun, car, cdr));
    Ok(rbool(ret))
}

pub fn prim_lt(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let ref eargs = try!(eval_list(renv, args));
    let ref car = try!(rcar(eargs));
    let ref cdr = try!(rcdr(eargs));
    let ret = try!(do_cmp(&|x, y| x < y, car, cdr));
    Ok(rbool(ret))
}

pub fn prim_lte(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let ref eargs = try!(eval_list(renv, args));
    let ref car = try!(rcar(eargs));
    let ref cdr = try!(rcdr(eargs));
    let ret = try!(do_cmp(&|x, y| x <= y, car, cdr));
    Ok(rbool(ret))
}

pub fn prim_gt(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let ref eargs = try!(eval_list(renv, args));
    let ref car = try!(rcar(eargs));
    let ref cdr = try!(rcdr(eargs));
    let ret = try!(do_cmp(&|x, y| x > y, car, cdr));
    Ok(rbool(ret))
}

pub fn prim_gte(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let ref eargs = try!(eval_list(renv, args));
    let ref car = try!(rcar(eargs));
    let ref cdr = try!(rcdr(eargs));
    let ret = try!(do_cmp(&|x, y| x >= y, car, cdr));
    Ok(rbool(ret))
}

pub fn prim_mul(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let ret = try!(eval_list(renv, args).and_then( |ref v| do_mul(v) ));
    Ok(rint(ret))
}

pub fn prim_sub(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let ref eargs = try!(eval_list(renv, args));
    let ref car = try!(rcar(eargs));
    let ref cdr = try!(rcdr(eargs));
    Ok(rint(try!(do_sub(car, cdr))))
}

pub fn prim_add(renv: &mut Env<Node>, args: &Node) -> EvalResult<Node> {
    let ret = try!(eval_list(renv, args).and_then( |ref v| do_add(v) ));
    Ok(rint(ret))
}

pub fn do_cmp<F>(f: &F, l: &Node, r: &Node) -> EvalResult<bool> where F: Fn(i32, i32) -> bool {
    match (l, r) {
        (ref rv@&Node::Int(_), &Node::Cell(ref v1, ref v2)) => {
            Ok(try!(do_cmp(f, rv, v1)) && try!(do_cmp(f, v1, v2)))
        },
        (&Node::Int(v1), &Node::Int(v2)) => Ok(f(v1, v2)),
        (_, &Node::Nil) => Ok(true),
        (_, _) => Err(EvalError::WrongTypeArg)
    }
}

fn do_mul(lst: &Node) -> EvalResult<i32> {
    match *lst {
        Node::Cell(ref car, ref cdr) => Ok(try!(do_mul(car)) * try!(do_mul(cdr))),
        Node::Int(k) => Ok(k),
        Node::Nil => Ok(1),
        _ => Err(EvalError::WrongTypeArg)
    }
}

fn do_sub(base: &Node, rest: &Node) -> EvalResult<i32> {
    match (base, rest) {
        (ref n@&Node::Int(_), &Node::Cell(ref v1, ref v2)) => {
            let ref re = Node::Int(try!(do_sub(n, v1)));
            do_sub(re, v2)
        }
        (&Node::Int(v1), &Node::Int(v2)) => Ok(v1 - v2),
        (&Node::Int(v), &Node::Nil) => Ok(v),
        (_, _) => Err(EvalError::WrongTypeArg)
    }
}

fn do_add(lst: &Node) -> EvalResult<i32> {
    match *lst {
        Node::Cell(ref car, ref cdr) => Ok(try!(do_add(car)) + try!(do_add(cdr))),
        Node::Int(k) => Ok(k),
        Node::Nil => Ok(0),
        _ => Err(EvalError::WrongTypeArg)
    }
}
