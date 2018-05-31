use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use syntax::*;

pub struct Env {
    parent: Option<Rc<Env>>,
    map: RefCell<HashMap<Id, Value>>,
}

impl Env {
    pub fn new() -> Env {
        Env { parent: None, map: RefCell::new(HashMap::new()) }
    }

    pub fn new_child(parent: Rc<Env>, keys: Vec<Id>, vals: Vec<Value>) -> Env {
        if keys.len() != vals.len() {
            panic!("Number of keys and that of vals are different")
        }
        let mut map = HashMap::new();
        for (k, v) in keys.iter().zip(vals.iter()) {
            map.insert(k.clone(), v.clone());
        }
        Env { parent: Some(Rc::clone(&parent)), map: RefCell::new(map) }
    }

    pub fn get(self: Rc<Env>, key: &Id) -> Value {
        match self.map.borrow().get(key) {
            Some(ref v) => (*v).clone(),
            None => match self.parent {
                Some(ref parent) => Rc::clone(parent).get(key),
                None => panic!(format!("Unbound variable: {}", key)),
            }
        }
    }

    pub fn update(self: Rc<Env>, key: Id, val: Value) -> () {
        self.map.borrow_mut().insert(key, val);
    }
}
