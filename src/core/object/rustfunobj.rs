use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::*;

pub struct PyRustFunObject {
    pub name: String,
    pub ob_self: Option<Rc<PyObject>>,
    pub rust_fun: PyRustFun,
}

#[derive(Clone)]
pub enum PyRustFun {
    MethO(Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>>),
}

thread_local! (
    pub static PY_RUSTFUN_TYPE: Rc<PyObject> = {
        let rfuntp =  PyTypeObject {
            tp_name: "rustfunction".to_string(),
            tp_base: None,
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(rfuntp))),
        })
    }
);

impl PyObject {
    pub fn pyrustfun_name(self: Rc<PyObject>) -> String {
        match self.inner {
            PyInnerObject::RustFunObj(ref obj) => obj.name.clone(),
            _ => panic!("Type Error: pyrustfun_name"),
        }
    }
}
