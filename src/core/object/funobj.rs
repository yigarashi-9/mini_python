use std::cell::RefCell;
use std::rc::Rc;

use env::Env;
use object::*;
use object::typeobj::*;
use syntax::Id;
use opcode::Code;

pub struct PyFunObject {
    pub env: Rc<Env>,
    pub codeobj: Rc<PyObject>,
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
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(funtp))),
        })
    }
);

impl PyObject {
    pub fn pyfun_new(env: &Rc<Env>, codeobj: Rc<PyObject>) -> Rc<PyObject> {
        Rc::new(PyObject {
            ob_type: PY_FUN_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::FunObj(Rc::new(PyFunObject {
                env: Rc::clone(env),
                codeobj: codeobj,
            }))
        })
    }

    pub fn pyfun_code(self: Rc<Self>) -> Code {
        match self.inner {
            PyInnerObject::FunObj(ref obj) => obj.codeobj.pycode_code(),
            _ => panic!("Type Error: pyfun_code")
        }
    }

    pub fn pyfun_argnames(self: Rc<Self>) -> Vec<Id> {
        match self.inner {
            PyInnerObject::FunObj(ref obj) => obj.codeobj.pycode_argnames(),
            _ => panic!("Type Error: pyfun_code")
        }
    }
}
