use std::rc::Rc;
use std::ops::Deref;
use env::Env;
use std::fmt;

use evaluator::{Result as EResult};

#[derive(Clone)]
pub struct Prim(pub Rc<Fn(&mut Env<Node>, &Node) -> EResult<Node>>);

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Int(i32),
    Sym(String),
    Prim(Prim),
    Cell(Rc<Node>, Rc<Node>),
    Nil,
}

pub fn rint(n: i32) -> Node {
    Node::Int(n)
}

pub fn rnil() -> Node {
    Node::Nil
}

pub fn rcell(car: Node, cdr:  Node) -> Node {
    Node::Cell(Rc::new(car), Rc::new(cdr))
}

pub fn rquote(lst: Node) -> Node {
    rcell(Node::Sym("quote".to_string()), lst)
}

pub fn rsym<T: Into<String>>(s: T) -> Node {
    Node::Sym(s.into())
}

impl PartialEq for Prim {
    fn eq(&self, other: &Self) -> bool {
        false                   // TODO: fix
    }
}

impl fmt::Debug for Prim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Primtive function")
    }
}

impl Deref for Prim {
    type Target = Rc<Fn(&mut Env<Node>, &Node) -> EResult<Node>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
