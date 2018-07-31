use std::rc::Rc;

use env::Env;
use object::typeobj::*;
use syntax::{Id, Program};

pub struct PyFuncObject {
    pub ob_type: Rc<PyTypeObject>,
    pub env: Rc<Env>,
    pub parms: Vec<Id>,
    pub code: Program,
}

pub fn new_fun_type_object() -> PyTypeObject {
    PyTypeObject {
        ob_type: Some(Rc::new(PyTypeObject::new_type())),
        tp_name: "function".to_string(),
        tp_hash: Some(Box::new(default_hash)),
        tp_bool: None,
        tp_fun_eq: None,
        tp_fun_add: None,
        tp_fun_lt: None,
        tp_len: None,
        tp_dict: None,
    }
}
