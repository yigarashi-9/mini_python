use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};
use object::pyhashmap::PyHashMap;

fn dict_len(v: Rc<PyObject>) -> Rc<PyObject> {
    match v.inner {
        PyInnerObject::DictObj(ref obj) => obj.dict.borrow().len(),
        _ => panic!("TypeError: dict_len")
    }
}

thread_local! (
    pub static PY_DICT_TYPE: Rc<RefCell<PyTypeObject>> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "dict".to_string(),
                tp_base: None,
                tp_hash: None,
                tp_bool: None,
                tp_fun_eq: None,
                tp_fun_add: None,
                tp_fun_lt: None,
                tp_len: Some(Rc::new(dict_len)),
                tp_dict: None,
                tp_bases: None,
                tp_mro: None,
                tp_subclasses: None,
            };
            Rc::new(RefCell::new(tp))
        })
    }
);

pub struct PyDictObject {
    pub dict: RefCell<PyHashMap>,
}

impl PyObject {
    pub fn new_dict() -> Rc<PyObject> {
        PY_DICT_TYPE.with(|tp| {
            let inner =  PyDictObject {
                dict: RefCell::new(PyHashMap::new()),
            };
            Rc::new(PyObject {
                ob_type: Rc::clone(&tp),
                inner: PyInnerObject::DictObj(Rc::new(inner))
            })
        })
    }

    pub fn lookup(&self, key: &Rc<PyObject>) -> Option<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::DictObj(ref obj) => {
                match obj.dict.borrow().get(key) {
                    Some(v) => Some(Rc::clone(v)),
                    None => None
                }
            },
            _ => panic!("Type Error: PyObject lookup")
        }
    }

    pub fn update(&self, key: Rc<PyObject>, value: Rc<PyObject>) {
        match self.inner {
            PyInnerObject::DictObj(ref obj) => obj.dict.borrow_mut().insert(key, value),
            _ => panic!("Type Error: PyObject update")
        }
    }
}
