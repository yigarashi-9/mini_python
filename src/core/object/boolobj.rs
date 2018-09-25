use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::longobj::{PyLongObject, PY_LONG_TYPE};
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};

thread_local! (
    pub static PY_BOOL_TYPE: Rc<RefCell<PyTypeObject>> = {
        PY_TYPE_TYPE.with(|tp| {
            PY_LONG_TYPE.with(|longtp| {
                let tp = PyTypeObject {
                    ob_type: Some(Rc::clone(&tp)),
                    tp_name: "bool".to_string(),
                    tp_base: Some(Rc::clone(&longtp)),
                    tp_hash: None,
                    tp_bool: None,
                    tp_fun_eq: None,
                    tp_fun_add: None,
                    tp_fun_lt: None,
                    tp_len: None,
                    tp_dict: None,
                };
                Rc::new(RefCell::new(tp))
            })
        })
    };

    pub static PY_TRUE: Rc<PyObject> = {
        PY_BOOL_TYPE.with(|tp| {
            let inner = PyLongObject { n: 1 };
            Rc::new(PyObject {
                ob_type: Rc::clone(&tp),
                inner: PyInnerObject::LongObj(Rc::new(inner))
            })
        })
    };

    pub static PY_FALSE: Rc<PyObject> = {
        PY_BOOL_TYPE.with(|tp| {
            let inner = PyLongObject { n: 0 };
            Rc::new(PyObject {
                ob_type: Rc::clone(&tp),
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
