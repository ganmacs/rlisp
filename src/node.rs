use std::fmt;
use std::rc::Rc;
use env::Env;
use error::EvalError;
use evaluator::EvalResult;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Int(i32),
    Sym(String),
    Prim(Prim),
    Bool(Bool),
    Cell(Rc<Node>, Rc<Node>),
    Nil,
}

#[derive(Clone)]
pub enum Prim {
    Proc(Rc<Fn(&mut Env<Node>, &Node) -> EvalResult<Node>>),
    Lambda(Env<Node>, Rc<Node>, Rc<Node>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Bool {
    True,
    False,
}

pub fn node_to_vec(mut node: Node) -> Vec<Node> {
    let args = &mut Vec::new();

    while node != rnil() {
        let l = rcar(&node).unwrap();
        let r = rcdr(&node).unwrap();
        args.push(l);
        node = r;
    }

    args.clone()
}

pub fn rcdar(cell: &Node) -> EvalResult<Node> {
    rcdr(cell).and_then(|ref v| rcar(v))
}

pub fn rcddar(cell: &Node) -> EvalResult<Node> {
    rcdr(cell).and_then(|ref v| rcdr(v)).and_then(|ref v| rcar(v))
}

pub fn rcar(cell: &Node) -> EvalResult<Node> {
    if let Node::Cell(ref car, _) = *cell {
        Ok((**car).clone())
    } else {
        Err(EvalError::WrongTypeArg)
    }
}

pub fn car_ref(cell: &Node) -> EvalResult<&Node> {
    if let &Node::Cell(ref car, _) = cell {
        Ok(car)
    } else {
        Err(EvalError::WrongTypeArg)
    }
}

pub fn rcdr(cell: &Node) -> EvalResult<Node> {
    if let Node::Cell(_, ref cdr) = *cell {
        Ok((**cdr).clone())
    } else {
        Err(EvalError::WrongTypeArg)
    }
}

pub fn sym_to_str(sym: &Node) -> EvalResult<&str> {
    if let &Node::Sym(ref name) = sym {
        Ok(name)
    } else {
        Err(EvalError::WrongTypeArg)
    }
}

pub fn rint(n: i32) -> Node {
    Node::Int(n)
}

pub fn rnil() -> Node {
    Node::Nil
}

pub fn rbool(v: bool) -> Node {
    if v { rtrue() } else { rfalse() }
}
pub fn rtrue() -> Node {
    Node::Bool(Bool::True)
}

pub fn rfalse() -> Node {
    Node::Bool(Bool::False)
}

pub fn rcell(car: Node, cdr: Node) -> Node {
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
        self == other
    }
}

impl fmt::Debug for Prim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Primtive function")
    }
}
