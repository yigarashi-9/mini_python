use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use object::PyObject;
use syntax::Id;

pub struct Env {
    parent: Option<Rc<Env>>,
    map: RefCell<HashMap<Id, Rc<PyObject>>>,
}

impl Env {
    pub fn new() -> Env {
        Env { parent: None, map: RefCell::new(HashMap::new()) }
    }

    pub fn new_child(parent: &Rc<Env>, keys: &Vec<Id>, vals: &Vec<Rc<PyObject>>) -> Env {
        if keys.len() != vals.len() {
            panic!("Number of keys and that of vals are different {} {}", keys.len(), vals.len())
        }
        let mut map = HashMap::new();
        for (k, v) in keys.iter().zip(vals.iter()) {
            map.insert(k.clone(), Rc::clone(v));
        }
        Env { parent: Some(Rc::clone(parent)), map: RefCell::new(map) }
    }

    pub fn get(self: &Rc<Env>, key: &Id) -> Rc<PyObject> {
        match self.map.borrow().get(key) {
            Some(ref v) => Rc::clone(v),
            None => match self.parent {
                Some(ref parent) => Rc::clone(parent).get(key),
                None => panic!("Unbound variable: {}", key),
            }
        }
    }

    pub fn update(self: &Rc<Env>, key: Id, val: Rc<PyObject>) -> () {
        self.map.borrow_mut().insert(key, val);
    }

    pub fn dictobj(self: &Rc<Env>) -> Rc<PyObject> {
        let dictobj = PyObject::pydict_new();
        for (k, v) in self.map.borrow().iter() {
            let key = PyObject::from_string(k.clone());
            dictobj.pydict_update(key, Rc::clone(v));
        }
        dictobj
    }
}
