use std::collections::{HashMap, LinkedList};
use std::collections::hash_map::Entry;

#[derive(Debug, PartialEq, Clone)]
pub struct Env<T> {
    global: HashMap<String, T>,
    local: LinkedList<HashMap<String, T>>,
}

impl<T> Env<T> {
    pub fn new() -> Env<T> {
        Env {
            global: HashMap::new(),
            local: LinkedList::new(),
        }
    }

    pub fn push_local_scope(&mut self) {
        self.local.push_front(HashMap::new());
    }

    // Need a return value?
    pub fn pop_local_scope(&mut self) {
        self.local.pop_front();
    }

    pub fn register<S: Into<String>>(&mut self, key: S, value: T) {
        match self.local.front_mut() {
            None => self.global.insert(key.into(), value),
            Some(v) => v.insert(key.into(), value),
        };
    }

    pub fn entry<S: Into<String>>(&mut self, key: S) -> Entry<String, T> {
        let key = key.into();
        let mut v = self.global.entry(key.to_string());

        // Get newer pushed env
        for lhash in self.local.iter_mut().rev() {
            match (&v, lhash.entry(key.to_string())) {
                (&Entry::Vacant(_), nv) => v = nv,
                (&Entry::Occupied(_), o @ Entry::Occupied(_)) => v = o,
                (&Entry::Occupied(_), Entry::Vacant(_)) => {}
            }
        }
        v
    }

    pub fn find(&self, key: &str) -> Option<&T> {
        for lhash in self.local.iter() {
            if let v @ Some(_) = lhash.get(key) {
                return v;
            }
        }
        self.global.get(key)
    }

    pub fn debug_list_all_variable(&self) {
        for lhash in self.local.iter() {
            for (key, value) in lhash {
                println!("{:?} => llvm value in local", key);
            }
        }
        for (key, value) in &self.global {
            println!("{:?} => llvm value in global", key);
        }
    }
}
