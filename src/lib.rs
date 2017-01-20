mod node;
mod parser;

use node::Node;

pub struct Rlisp;

pub fn run(input: &'static str) ->  Result<Node, &'static str> {    // specific type
    match parser::parse(input) {
        Ok(result) => eval(result),
        Err(v) => Err(v.to_str()),
    }
}

fn eval(ast: Node) -> Result<Node, &'static str> {    // specific type
    match ast {
        Node::Cell { car: x,  cdr: y } => {
            let fun = eval(*x);
            match try!(fun) {
                Node::Fn { name: n } => {
                    match n {
                        "+" => Ok(Node::Int{ v: prim_add(*y)}),
                        _ => Err("Unknow function")
                    }
                },
                _ => Err("others")
            }
        },
        x => Ok(x)
    }
}

fn prim_add(ast: Node) -> i32 {
    match ast {
        Node::Cell { car: x,  cdr: y } => {
            if let Node::Int{ v: k } = *x {
                k + prim_add(*y)
            } else {
                0
            }
        },
        _ => 0
    }
}

pub fn lprint(result: Node) {
    match result {
        Node::Int { v } => println!("{}", v),
        x => println!("{:?}", x)
    }
}
