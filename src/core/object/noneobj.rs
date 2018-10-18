use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::*;

thread_local! (
    pub static PY_NONE_TYPE: Rc<PyObject> = {
        let nonetp = PyTypeObject {
            tp_name: "None".to_string(),
            tp_hash: Some(Rc::new(default_hash)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(nonetp))),
        })
    };

    pub static PY_NONE_OBJECT: Rc<PyObject> = {
        PY_NONE_TYPE.with(|tp| {
            Rc::new(PyObject {
                ob_type: Some(Rc::clone(&tp)),
                ob_dict: None,
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
