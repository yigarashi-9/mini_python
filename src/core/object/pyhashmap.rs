use std::rc::Rc;

use object::PyObject;

pub struct PyHashMap {
    table: Vec<(u64, Rc<PyObject>, Rc<PyObject>)>,
}

impl PyHashMap {
    pub fn new() -> PyHashMap {
        PyHashMap { table: vec![] }
    }

    pub fn get(&self, key: Rc<PyObject>) -> Option<&Rc<PyObject>> {
        self.table.iter().find_map(|ref tuple| {
            let ob_type = key.ob_type();
            if tuple.0 == ob_type.pytype_typeobj_borrow().tp_hash.as_ref().unwrap()(Rc::clone(&key)) {
                Some(&tuple.2)
            } else {
                None
            }
        })
    }

    pub fn insert(&mut self, key: Rc<PyObject>, value: Rc<PyObject>) {
        let ob_type = key.ob_type();
        let hash = ob_type.pytype_typeobj_borrow().tp_hash.as_ref().unwrap()(Rc::clone(&key));
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
