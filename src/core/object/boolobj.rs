use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::longobj::{PyLongObject, PY_LONG_TYPE};
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};

thread_local! (
    pub static PY_BOOL_TYPE: Rc<PyObject> = {
        let booltp = PY_LONG_TYPE.with(|longtp| {
            PyTypeObject {
                tp_name: "bool".to_string(),
                tp_base: Some(Rc::clone(longtp)),
                ..Default::default()
            }
        });
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(booltp))),
        })
    };

    pub static PY_TRUE: Rc<PyObject> = {
        PY_BOOL_TYPE.with(|tp| {
            let inner = PyLongObject { n: 1 };
            Rc::new(PyObject {
                ob_type: Some(Rc::clone(tp)),
                ob_dict: None,
                inner: PyInnerObject::LongObj(Rc::new(inner))
            })
        })
    };

    pub static PY_FALSE: Rc<PyObject> = {
        PY_BOOL_TYPE.with(|tp| {
            let inner = PyLongObject { n: 0 };
            Rc::new(PyObject {
                ob_type: Some(Rc::clone(tp)),
                ob_dict: None,
                inner: PyInnerObject::LongObj(Rc::new(inner))
            })
        })
    }
);

impl PyObject {
    pub fn from_bool(raw_bool: bool) -> Rc<PyObject> {
        if raw_bool {
            PY_TRUE.with(|obj| { Rc::clone(&obj) })
        } else {
            PY_FALSE.with(|obj| { Rc::clone(&obj) })
        }
    }
}
