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

pub fn new_method_type_object() -> PyTypeObject {
    PyTypeObject {
        ob_type: Some(Rc::new(PyTypeObject::new_type())),
        tp_name: "method".to_string(),
        tp_hash: Some(Box::new(default_hash)),
        tp_bool: None,
        tp_fun_eq: None,
        tp_fun_add: None,
        tp_fun_lt: None,
        tp_len: None,
        tp_dict: None,
    }
}
