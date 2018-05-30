use std::collections::HashMap;
use syntax::*;

pub struct Env<'a> {
    parent: Option<&'a Env<'a>>,
    map: HashMap<Id, Value>,
}

impl <'a>Env<'a> {
    pub fn new() -> Env<'a> {
        Env { parent: None, map: HashMap::new() }
    }

    pub fn new_child(parent: &'a Env, keys: Vec<Id>, vals: Vec<Value>) -> Env<'a> {
        if keys.len() != vals.len() {
            panic!("Number of keys and that of vals are different")
        }

        let mut map = HashMap::new();
        for (k, v) in keys.iter().zip(vals.iter()) {
            map.insert(k.clone(), v.clone());
        }

        Env { parent: Some(parent), map: map }
    }

    pub fn get(&self, key: &Id) -> &Value {
        match self.map.get(key) {
            Some(ref v) => v,
            None => match self.parent {
                Some(ref parent) => parent.get(key),
                None => panic!("Unbound variable"),
            }
        }
    }

    pub fn update(&mut self, key: Id, val: Value) -> () {
        self.map.insert(key, val);
    }
}
