use std::rc::Rc;

use env::Env;
use object::object::*;
use object::typeobj::*;
use syntax::{Id, Program};

pub struct PyMethodObject {
    pub ob_type: Rc<PyTypeObject>,
    pub ob_self: Rc<PyObject>,
    pub env: Rc<Env>,
    pub parms: Vec<Id>,
    pub code: Program,
}

thread_local! (
    pub static PY_METHOD_TYPE: Rc<PyTypeObject> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "method".to_string(),
                tp_hash: Some(Box::new(default_hash)),
                tp_bool: None,
                tp_fun_eq: None,
                tp_fun_add: None,
                tp_fun_lt: None,
                tp_len: None,
                tp_dict: None,
            };
            Rc::new(tp)
        })
    }
);
