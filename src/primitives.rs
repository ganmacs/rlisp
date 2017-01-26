use node::{Node, rint, car as cc};
use env::Env;
use evaluator::*;

pub fn prim_progn(renv: &mut Env<Node>, args: &Node) -> Result<Node> {
    match *args {
        Node::Cell(ref car, ref cdr) => {
            let ret = try!(eval(renv, car));
            let rest = try!(prim_progn(renv, cdr));
            Ok(if rest == Node::Nil { ret } else  { rest })
        }
        ref x => Ok(x.clone())
    }
}

pub fn prim_define(renv: &mut Env<Node>, args: &Node) -> Result<Node> {
    match *args {
        Node::Cell(ref car, ref cdr) => {
            if let Node::Sym(ref s) = **car {
                let ccdr = try!(cc(cdr));
                let ret = try!(eval(renv, &ccdr));
                renv.register(s.to_string(), ret.clone());
                return Ok(ret);
            }
        },
        _ => ()
    }
    Err(RError::E)
}

pub fn prim_sub(renv: &mut Env<Node>, args: &Node) -> Result<Node> {
    let lst = try!(eval_list(renv, args));
    match lst {
        Node::Cell(ref car, ref cdr) => {
            if let Node::Int(base) = **car {
                return Ok(rint(do_sub(base, cdr)))
            }
        },
        _ => ()
    }

    Err(RError::E)
}

pub fn prim_add(renv: &mut Env<Node>, args: &Node) -> Result<Node> {
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
