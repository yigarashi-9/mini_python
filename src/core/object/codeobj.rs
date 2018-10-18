use std::cell::RefCell;
use std::rc::Rc;

use syntax::Id;
use opcode::Code;
use object::{PyObject, PyInnerObject};
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};

thread_local! (
    pub static PY_CODE_TYPE: Rc<PyObject> = {
        let codetp =  PyTypeObject {
            tp_name: "code".to_string(),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(codetp))),
        })
    };
);

pub struct PyCodeObject {
    co_code: Code,
    co_argnames: Vec<Id>,
}

impl PyObject {
    pub fn pycode_new(code: Code, argnames: Vec<Id>) -> Rc<PyObject> {
        Rc::new(PyObject {
            ob_type: PY_CODE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::CodeObj(Rc::new(PyCodeObject {
                co_code: code,
                co_argnames: argnames,
            }))
        })
    }

    pub fn pycode_code(self: &Rc<PyObject>) -> Code {
        match self.inner {
            PyInnerObject::CodeObj(ref obj) => obj.co_code.clone(),
            _ => panic!("Type Error: pycode_code")
        }
    }

    pub fn pycode_argnames(self: &Rc<PyObject>) -> Vec<Id> {
        match self.inner {
            PyInnerObject::CodeObj(ref obj) => obj.co_argnames.clone(),
            _ => panic!("Type Error: pycode_argnames")
        }
    }
}
