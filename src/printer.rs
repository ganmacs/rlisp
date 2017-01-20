use super::Node;

pub fn lprint(result: Node) {
    match result {
        Node::Int(v) => println!("{}", v),
        x => println!("{:?}", x)
    }
}
