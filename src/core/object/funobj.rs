use std::cell::RefCell;
use std::rc::Rc;

use env::Env;
use object::*;
use object::typeobj::*;
use syntax::{Id, Program};

pub struct PyFunObject {
    pub env: Rc<Env>,
    pub parms: Vec<Id>,
    pub code: Program,
}

thread_local! (
    pub static PY_FUN_TYPE: Rc<PyObject> = {
        let funtp =  PyTypeObject {
            tp_name: "function".to_string(),
            tp_base: None,
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_call: None,
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(funtp))),
        })
    }
);

impl PyObject {
    pub fn pyfun_new(env: &Rc<Env>, parms: &Vec<Id>, code: &Program) -> Rc<PyObject>{
        Rc::new(PyObject {
            ob_type: PY_FUN_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            inner: PyInnerObject::FunObj(Rc::new(PyFunObject {
                env: Rc::clone(env),
                parms: parms.clone(),
                code: code.clone(),
            }))
        })
    }
}
