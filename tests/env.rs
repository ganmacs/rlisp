extern crate rlisp;

use rlisp::env::Env;
use rlisp::node::{Node, rint};

#[test]
fn test_register_and_get_int_value_when_local_is_empty() {
    let renv = &mut Env::new();
    renv.register("x", rint(1));
    assert_eq!(*(renv.find("x").unwrap()), rint(1));
}

#[test]
fn test_return_none_when_missing_key() {
    let renv: Env<Node> = Env::new();
    assert_eq!(renv.find("x"), None);
}


#[test]
fn test_push_and_pop_scope() {
    let renv = &mut Env::new();
    renv.register("x", rint(1));
    renv.push_local_scope();

    renv.register("x", rint(10));
    assert_eq!(*(renv.find("x").unwrap()), rint(10));

    renv.pop_local_scope();
    assert_eq!(*(renv.find("x").unwrap()), rint(1));
}
