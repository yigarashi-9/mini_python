use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use syntax::*;

pub struct Env {
    parent: Option<Rc<Env>>,
    map: RefCell<HashMap<Id, Rc<Value>>>,
}

impl Env {
    pub fn new() -> Env {
        Env { parent: None, map: RefCell::new(HashMap::new()) }
    }

    pub fn new_child(parent: Rc<Env>, keys: &Vec<Id>, vals: &Vec<Rc<Value>>) -> Env {
        if keys.len() != vals.len() {
            panic!("Number of keys and that of vals are different {} {}", keys.len(), vals.len())
        }
        let mut map = HashMap::new();
        for (k, v) in keys.iter().zip(vals.iter()) {
            map.insert(k.clone(), Rc::clone(v));
        }
        Env { parent: Some(Rc::clone(&parent)), map: RefCell::new(map) }
    }

    pub fn get(self: Rc<Env>, key: &Id) -> Rc<Value> {
        match self.map.borrow().get(key) {
            Some(ref v) => Rc::clone(v),
            None => match self.parent {
                Some(ref parent) => Rc::clone(parent).get(key),
                None => panic!("Unbound variable: {}", key),
            }
        }
    }

    pub fn update(self: Rc<Env>, key: Id, val: Rc<Value>) -> () {
        self.map.borrow_mut().insert(key, val);
    }

    pub fn raw_map(self: Rc<Env>) -> HashMap<Id, Rc<Value>> {
        self.map.borrow().clone()
    }
}
