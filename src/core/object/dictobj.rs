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
    pub static PY_DICT_TYPE: Rc<PyObject> = {
        let dicttp = PyTypeObject {
            tp_name: "dict".to_string(),
            tp_base: None,
            tp_hash: None,
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: Some(Rc::new(dict_len)),
            tp_call: None,
            tp_getattro: None,
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(dicttp))),
        })
    }
);

pub struct PyDictObject {
    pub dict: RefCell<PyHashMap>,
}

impl PyObject {
    pub fn pydict_new() -> Rc<PyObject> {
        let inner =  PyDictObject {
            dict: RefCell::new(PyHashMap::new()),
        };
        Rc::new(PyObject {
            ob_type: PY_DICT_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::DictObj(Rc::new(inner))
        })
    }

    pub fn pydict_check(&self) -> bool {
        PY_DICT_TYPE.with(|tp| { (&self.ob_type).as_ref() == Some(tp) })
    }

    pub fn pydict_lookup(&self, key: Rc<PyObject>) -> Option<Rc<PyObject>> {
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

    pub fn pydict_update(&self, key: Rc<PyObject>, value: Rc<PyObject>) {
        match self.inner {
            PyInnerObject::DictObj(ref obj) => obj.dict.borrow_mut().insert(key, value),
            _ => panic!("Type Error: PyObject update")
        }
    }
}
