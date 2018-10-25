use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::generic::*;
use object::typeobj::*;

thread_local! (
    pub static PY_BASEEXC_TYPE: Rc<PyObject> = {
        let bexctp = PyTypeObject {
            tp_name: "BaseException".to_string(),
            tp_hash: Some(Rc::new(default_hash)),
            tp_new: Some(Rc::new(pybaseexc_new)),
            ..Default::default()
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
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(bexctp))),
        })
    };

    pub static PY_TYPEERROR_TYPE: Rc<PyObject> = {
        let exctp = PyTypeObject {
            tp_name: "TypeError".to_string(),
            tp_base: PY_EXC_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            tp_hash: Some(Rc::new(default_hash)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(exctp))),
        })
    };

    pub static PY_STOPITERATION_TYPE: Rc<PyObject> = {
        let exctp = PyTypeObject {
            tp_name: "StopIteration".to_string(),
            tp_base: PY_EXC_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            tp_hash: Some(Rc::new(default_hash)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(exctp))),
        })
    };
);

impl PyObject {
    pub fn pyexc_is_exc_subclass(self: Rc<Self>) -> bool {
        pyobj_issubclass(self, PY_BASEEXC_TYPE.with(|tp| { Rc::clone(tp) }))
    }

    pub fn pyexc_is_exc_instance(self: Rc<Self>) -> bool {
        pyobj_isinstance(self, PY_BASEEXC_TYPE.with(|tp| { Rc::clone(tp) }))
    }
}

pub struct PyExcObject {
    args: Rc<PyObject>,
}

fn pybaseexc_new(exception: Rc<PyObject>, args: &Vec<Rc<PyObject>>) -> Rc<PyObject> {
    Rc::new(PyObject {
        ob_type: Some(exception),
        ob_dict: None,
        inner: PyInnerObject::ExcObj(Rc::new(PyExcObject { args: PyObject::pylist_from_vec(args) })),
    })
}
