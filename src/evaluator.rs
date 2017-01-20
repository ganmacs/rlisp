use super::Node;

pub fn eval(ast: Node) -> Result<Node, &'static str> {    // specific type
    match ast {
        Node::Cell(car, cdr) => {
            let fun = eval(*car);
            match try!(fun) {
                Node::Fn { name: n } => {
                    match n {
                        "+" => Ok(Node::Int(prim_add(*cdr))),
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
        Node::Cell(car, cdr) => {
            if let Node::Int(k) = *car {
                k + prim_add(*cdr)
            } else {
                0
            }
        },
        _ => 0
    }
}
