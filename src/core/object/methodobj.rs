use std::cell::RefCell;
use std::rc::Rc;

use env::Env;
use object::PyObject;
use object::typeobj::*;
use syntax::{Id, Program};

pub struct PyMethodObject {
    pub ob_self: Rc<PyObject>,
    pub env: Rc<Env>,
    pub parms: Vec<Id>,
    pub code: Program,
}

thread_local! (
    pub static PY_METHOD_TYPE: Rc<RefCell<PyTypeObject>> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "method".to_string(),
                tp_base: None,
                tp_hash: Some(Rc::new(default_hash)),
                tp_bool: None,
                tp_fun_eq: None,
                tp_fun_add: None,
                tp_fun_lt: None,
                tp_len: None,
                tp_dict: None,
                tp_bases: None,
                tp_mro: None,
                tp_subclasses: None,
            };
            Rc::new(RefCell::new(tp))
        })
    }
);
