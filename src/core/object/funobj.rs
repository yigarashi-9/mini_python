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
            tp_hash: Some(Rc::new(default_hash)),
            ..Default::default()
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
