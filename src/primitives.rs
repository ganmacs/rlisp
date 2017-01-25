use node::{Node, rint};
use env::Env;
use evaluator::*;

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
