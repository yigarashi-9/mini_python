use std::cell::RefCell;
use std::rc::Rc;

use eval::PyRes;
use object::{PyObject, PyInnerObject};
use object::typeobj::*;

pub struct PyRustFunObject {
    pub name: String,
    pub ob_self: Option<Rc<PyObject>>,
    pub rust_fun: PyRustFun,
}

#[derive(Clone)]
pub enum PyRustFun {
    MethO(Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> PyRes<Rc<PyObject>>>),
}

thread_local! (
    pub static PY_RUSTFUN_TYPE: Rc<PyObject> = {
        let rfuntp =  PyTypeObject {
            tp_name: "rustfunction".to_string(),
            tp_hash: Some(Rc::new(default_hash)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
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
