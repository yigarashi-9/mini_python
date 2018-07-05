use std::cell::RefCell;
use std::rc::Rc;

use object::object::*;
use object::typeobj::*;
use object::pyhashmap::PyHashMap;

pub struct PyDictObject {
    pub ob_type: Rc<PyTypeObject>,
    pub dict: RefCell<PyHashMap>,
}

impl PyDictObject {
    pub fn new() -> PyDictObject {
        PyDictObject {
            ob_type: Rc::new(PyTypeObject::new_dict()),
            dict: RefCell::new(PyHashMap::new()),
        }
    }

    pub fn lookup(&self, key: &Rc<PyObject>) -> Option<Rc<PyObject>> {
        match self.dict.borrow().get(key) {
            Some(v) => Some(Rc::clone(v)),
            None => None
        }
    }

    pub fn update(&self, key: Rc<PyObject>, value: Rc<PyObject>) {
        self.dict.borrow_mut().insert(key, value);
    }
}

pub fn new_dict_type_object() -> PyTypeObject {
    PyTypeObject {
        ob_type: Some(Rc::new(PyTypeObject::new_type())),
        tp_name: "dict".to_string(),
        tp_hash: None,
        tp_fun_eq: None,
        tp_fun_add: None,
        tp_fun_lt: None,
        tp_dict: None,
    }
}
