use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::*;

thread_local! (
    pub static PY_NONE_TYPE: Rc<PyObject> = {
        let nonetp = PyTypeObject {
            tp_name: "None".to_string(),
            tp_base: None,
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(nonetp))),
        })
    };

    pub static PY_NONE_OBJECT: Rc<PyObject> = {
        PY_NONE_TYPE.with(|tp| {
            Rc::new(PyObject {
                ob_type: Some(Rc::clone(&tp)),
                inner: PyInnerObject::NoneObj
            })
        })
    }
);

impl PyObject {
    pub fn none_obj() -> Rc<PyObject> {
        PY_NONE_OBJECT.with(|obj| { Rc::clone(&obj) })
    }
}
