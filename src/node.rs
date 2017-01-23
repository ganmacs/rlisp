#[derive(Debug, PartialEq)]
pub enum Node {
    Cell(Box<Node>, Box<Node>),
    Int(i32),
    Sym(String),
    Fn { name: &'static str },
    Nil,
}

pub fn rint(n: i32) -> Node {
    Node::Int(n)
}

pub fn rnil() -> Node {
    Node::Nil
}

pub fn rcell(car: Node, cdr:  Node) -> Node {
    Node::Cell(Box::new(car), Box::new(cdr))
}

pub fn rquote(lst: Node) -> Node {
    rcell(Node::Sym("quote".to_string()), lst)
}

pub fn rsym<T: Into<String>>(s: T) -> Node {
    Node::Sym(s.into())
}
