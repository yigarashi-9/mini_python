use std::cell::RefCell;
use std::rc::Rc;

use object::object::*;
use object::typeobj::*;
use object::pyhashmap::PyHashMap;

fn dict_len(v: Rc<PyObject>) -> Rc<PyObject> {
    match *v {
        PyObject::DictObj(ref obj) => obj.dict.borrow().len(),
        _ => panic!("TypeError: dict_len")
    }
}

thread_local! (
    pub static PY_DICT_TYPE: Rc<PyTypeObject> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "dict".to_string(),
                tp_hash: None,
                tp_bool: None,
                tp_fun_eq: None,
                tp_fun_add: None,
                tp_fun_lt: None,
                tp_len: Some(Box::new(dict_len)),
                tp_dict: None,
            };
            Rc::new(tp)
        })
    }
);

pub struct PyDictObject {
    pub ob_type: Rc<PyTypeObject>,
    pub dict: RefCell<PyHashMap>,
}

impl PyDictObject {
    pub fn new() -> PyDictObject {
        PY_DICT_TYPE.with(|tp| {
            PyDictObject {
                ob_type: Rc::clone(&tp),
                dict: RefCell::new(PyHashMap::new()),
            }
        })
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
