use std::cell::RefCell;
use std::rc::Rc;

use env::Env;
use object::{PyObject, PyInnerObject};
use object::typeobj::*;

pub struct PyMethodObject {
    pub ob_self: Rc<PyObject>,
    pub env: Rc<Env>,
    pub codeobj: Rc<PyObject>,
}

thread_local! (
    pub static PY_METHOD_TYPE: Rc<PyObject> = {
        let methtp = PyTypeObject {
            tp_name: "method".to_string(),
            tp_hash: Some(Rc::new(default_hash)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(methtp))),
        })
    }
);
