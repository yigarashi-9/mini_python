use std::rc::Rc;

use object::typeobj::*;

pub struct PyNoneObject {
    pub ob_type: Rc<PyTypeObject>,
}

pub fn new_none_type_object() -> PyTypeObject {
    PyTypeObject {
        ob_type: Some(Rc::new(PyTypeObject::new_type())),
        tp_name: "None".to_string(),
        tp_hash: Some(Box::new(default_hash)),
        tp_fun_eq: None,
        tp_fun_add: None,
        tp_fun_lt: None,
        tp_dict: None,
    }
}
