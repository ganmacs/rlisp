use std::rc::Rc;
use std::ops::Deref;
use env::Env;
use std::fmt;

use evaluator::{Result as EResult, RError};

#[derive(Clone)]
pub struct Prim(pub Rc<Fn(&mut Env<Node>, &Node) -> EResult<Node>>);

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Int(i32),
    Sym(String),
    Prim(Prim),
    Bool(Bool),
    Cell(Rc<Node>, Rc<Node>),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Bool {
    True,
    False
}

pub fn rcdar(cell: &Node) -> EResult<Node> {
    rcdr(cell).and_then( |ref v| rcar(v) )
}

pub fn rcddar(cell: &Node) -> EResult<Node> {
    rcdr(cell).and_then( |ref v| rcdr(v) ).and_then( |ref v| rcar(v) )
}

pub fn rcar(cell: &Node) -> EResult<Node> {
    match cell {
        &Node::Cell(ref car, _) => Ok((**car).clone()),
        _ => Err(RError::WrongTypeArg)
    }
}

pub fn rcdr(cell: &Node) -> EResult<Node> {
    match cell {
        &Node::Cell(_, ref cdr) => Ok((**cdr).clone()),
        _ => Err(RError::WrongTypeArg)
    }
}

pub fn rint(n: i32) -> Node {
    Node::Int(n)
}

pub fn rnil() -> Node {
    Node::Nil
}

pub fn rtrue() -> Node {
    Node::Bool(Bool::True)
}

pub fn rfalse() -> Node {
    Node::Bool(Bool::False)
}

pub fn rcell(car: Node, cdr:  Node) -> Node {
    Node::Cell(Rc::new(car), Rc::new(cdr))
}

pub fn rlist(car: Node, cdr: Node) -> Node {
    rcell(car, rcell(cdr, Node::Nil))
}

pub fn rquote(v: Node) -> Node {
    rcell(Node::Sym("quote".to_string()), rcell(v, Node::Nil))
}

pub fn rsym<T: Into<String>>(s: T) -> Node {
    Node::Sym(s.into())
}

pub fn prim(v: Prim) -> Node {
    Node::Prim(v)
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
