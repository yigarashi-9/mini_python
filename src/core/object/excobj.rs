use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::generic::*;
use object::typeobj::*;

thread_local! (
    pub static PY_BASEEXC_TYPE: Rc<PyObject> = {
        let bexctp = PyTypeObject {
            tp_name: "BaseException".to_string(),
            tp_base: None,
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_call: None,
            tp_getattro: None,
            tp_setattro: None,
            tp_iter: None,
            tp_iternext: None,
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(bexctp))),
        })
    };

    pub static PY_EXC_TYPE: Rc<PyObject> = {
        let bexctp = PyTypeObject {
            tp_name: "Exception".to_string(),
            tp_base: PY_BASEEXC_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_call: None,
            tp_getattro: None,
            tp_setattro: None,
            tp_iter: None,
            tp_iternext: None,
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(bexctp))),
        })
    }
);

impl PyObject {
    pub fn pyexc_is_exc_subclass(self: Rc<Self>) -> bool {
        pyobj_issubclass(self, PY_BASEEXC_TYPE.with(|tp| { Rc::clone(tp) }))
    }

    pub fn pyexc_is_exc_instance(self: Rc<Self>) -> bool {
        pyobj_isinstance(self, PY_BASEEXC_TYPE.with(|tp| { Rc::clone(tp) }))
    }
}