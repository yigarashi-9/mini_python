use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::*;

fn add_str_str(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match lv.inner {
        PyInnerObject::StrObj(ref l_obj) => {
            match rv.inner {
                PyInnerObject::StrObj(ref r_obj) =>
                    PyObject::from_string(format!("{}{}", l_obj.s, r_obj.s)),
                _ => panic!("Type Error: add_str_str"),
            }
        },
        _ => panic!("Type Error: add_str_str"),
    }
}

fn eq_str_str(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match lv.inner {
        PyInnerObject::StrObj(ref l_obj) => {
            match rv.inner {
                PyInnerObject::StrObj(ref r_obj) =>
                    PyObject::from_bool(l_obj.s == r_obj.s),
                _ => panic!("Type Error: eq_str_str"),
            }
        },
        _ => panic!("Type Error: eq_str_str"),
    }
}

fn str_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    match obj.inner {
        PyInnerObject::StrObj(ref obj) => obj.s.hash(&mut hasher),
        _ => panic!("Type Error: str_hash")
    };
    hasher.finish()
}

fn str_len(v: Rc<PyObject>) -> Rc<PyObject> {
    match v.inner {
        PyInnerObject::StrObj(ref obj) => PyObject::from_i32(obj.s.len() as i32),
        _ => panic!("Type Error: str_len")
    }
}

thread_local! (
    pub static PY_STRING_TYPE: Rc<RefCell<PyTypeObject>> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "str".to_string(),
                tp_base: None,
                tp_hash: Some(Rc::new(str_hash)),
                tp_bool: None,
                tp_fun_eq: Some(Rc::new(eq_str_str)),
                tp_fun_add: Some(Rc::new(add_str_str)),
                tp_fun_lt: None,
                tp_len: Some(Rc::new(str_len)),
                tp_dict: None,
                tp_subclasses: None,
            };
            Rc::new(RefCell::new(tp))
        })
    }
);

pub struct PyStringObject {
    s: String,
}

impl PyObject {
    pub fn from_str(s: &str) -> Rc<PyObject> {
        PyObject::from_string(s.to_string())
    }

    pub fn from_string(raw_string: String) -> Rc<PyObject> {
        PY_STRING_TYPE.with(|tp| {
            let inner = PyStringObject { s: raw_string };
            Rc::new(PyObject {
                ob_type: Rc::clone(&tp),
                inner: PyInnerObject::StrObj(Rc::new(inner))
            })
        })
    }
}
