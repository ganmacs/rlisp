#[derive(Debug)]
pub enum Node {
    Cell { car: Box<Node>, cdr: Box<Node> },
    Int { v: i32 },
    Fn { name: &'static str },
    Dot,
    RParen,
    Nil,
}
