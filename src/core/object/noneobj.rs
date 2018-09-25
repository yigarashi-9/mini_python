use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::*;

thread_local! (
    pub static PY_NONE_TYPE: Rc<RefCell<PyTypeObject>> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp =  PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "None".to_string(),
                tp_base: None,
                tp_hash: Some(Rc::new(default_hash)),
                tp_bool: None,
                tp_fun_eq: None,
                tp_fun_add: None,
                tp_fun_lt: None,
                tp_len: None,
                tp_dict: None,
            };
            Rc::new(RefCell::new(tp))
        })
    };

    pub static PY_NONE_OBJECT: Rc<PyObject> = {
        PY_NONE_TYPE.with(|tp| {
            Rc::new(PyObject {
                ob_type: Rc::clone(&tp),
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
