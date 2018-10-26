use std::rc::Rc;

use object::PyObject;

pub struct PyHashMap {
    table: Vec<(u64, Rc<PyObject>, Rc<PyObject>)>,
}

impl PyHashMap {
    pub fn new() -> PyHashMap {
        PyHashMap { table: vec![] }
    }

    pub fn get(&self, hash: u64) -> Option<&Rc<PyObject>> {
        self.table.iter().find_map(|tuple| {
            if tuple.0 == hash {
                Some(&tuple.2)
            } else {
                None
            }
        })
    }

    pub fn insert(&mut self, hash: u64, key: Rc<PyObject>, value: Rc<PyObject>) {
        let new_entry = [(hash, Rc::clone(&key), Rc::clone(&value))];
        let i = self.table.iter().position(|ref tuple| tuple.0 == hash);
        match i {
            Some(i) => { self.table.splice(i..i, new_entry.iter().cloned()); },
            None => { self.table.push((hash, key, value)) },
        };
    }

    pub fn len(&self) -> Rc<PyObject> {
        PyObject::from_i32(self.table.len() as i32)
    }
}
