use std::cell::RefCell;
use std::rc::Rc;

use env::Env;
use object::{PyObject, PyInnerObject};
use object::typeobj::*;
use syntax::{Id, Program};

pub struct PyMethodObject {
    pub ob_self: Rc<PyObject>,
    pub env: Rc<Env>,
    pub parms: Vec<Id>,
    pub code: Program,
}

thread_local! (
    pub static PY_METHOD_TYPE: Rc<PyObject> = {
        let methtp = PyTypeObject {
            tp_name: "method".to_string(),
            tp_base: None,
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_call: None,
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(methtp))),
        })
    }
);
