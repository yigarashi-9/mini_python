use std::cell::RefCell;
use std::rc::Rc;

use object::PyObject;
use object::typeobj::*;

pub struct PyRustFunObject {
    pub ob_self: Option<Rc<PyTypeObject>>,
    pub rust_fun: PyRustFun,
}

pub enum PyRustFun {
    MethO(Rc<dyn Fn(Rc<PyObject>) -> Rc<PyObject>>),
}

thread_local! (
    pub static PY_RUSTFUN_TYPE: Rc<RefCell<PyTypeObject>> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp =  PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "rustfunction".to_string(),
                tp_base: None,
                tp_hash: Some(Rc::new(default_hash)),
                tp_bool: None,
                tp_fun_eq: None,
                tp_fun_add: None,
                tp_fun_lt: None,
                tp_len: None,
                tp_dict: None,
                tp_subclasses: None,
            };
            Rc::new(RefCell::new(tp))
        })
    }
);
