use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::*;

thread_local! (
    pub static PY_NONE_TYPE: Rc<PyTypeObject> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp =  PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "None".to_string(),
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

impl PyObject {
    pub fn none_obj() -> Rc<PyObject> {
        PY_NONE_TYPE.with(|tp| {
            Rc::new(PyObject {
                ob_type: Rc::clone(&tp),
                inner: PyInnerObject::NoneObj
            })
        })
    }
}
