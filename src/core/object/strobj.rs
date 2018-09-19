use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use object::object::*;
use object::typeobj::*;

fn add_str_str(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match *lv {
        PyObject::StrObj(ref l_obj) => {
            match *rv {
                PyObject::StrObj(ref r_obj) =>
                    Rc::new(PyObject::from_string(format!("{}{}", l_obj.s, r_obj.s))),
                _ => panic!("Type Error: add_str_str"),
            }
        },
        _ => panic!("Type Error: add_str_str"),
    }
}

fn eq_str_str(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match *lv {
        PyObject::StrObj(ref l_obj) => {
            match *rv {
                PyObject::StrObj(ref r_obj) =>
                    Rc::new(PyObject::from_bool(l_obj.s == r_obj.s)),
                _ => panic!("Type Error: eq_str_str"),
            }
        },
        _ => panic!("Type Error: eq_str_str"),
    }
}

fn str_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    match *obj {
        PyObject::StrObj(ref obj) => obj.s.hash(&mut hasher),
        _ => panic!("Type Error: str_hash")
    };
    hasher.finish()
}

fn str_len(v: Rc<PyObject>) -> Rc<PyObject> {
    match *v {
        PyObject::StrObj(ref obj) => Rc::new(PyObject::from_i32(obj.s.len() as i32)),
        _ => panic!("Type Error: str_len")
    }
}

thread_local! (
    pub static PY_STRING_TYPE: Rc<PyTypeObject> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "str".to_string(),
                tp_hash: Some(Box::new(str_hash)),
                tp_bool: None,
                tp_fun_eq: Some(Box::new(eq_str_str)),
                tp_fun_add: Some(Box::new(add_str_str)),
                tp_fun_lt: None,
                tp_len: Some(Box::new(str_len)),
                tp_dict: None,
            };
            Rc::new(tp)
        })
    }
);

pub struct PyStringObject {
    pub ob_type: Rc<PyTypeObject>,
    s: String,
}

impl PyStringObject {
    pub fn from_string(raw_string: String) -> PyStringObject {
        PY_STRING_TYPE.with(|tp| {
            PyStringObject {
                ob_type: Rc::clone(&tp),
                s: raw_string
            }
        })
    }
}
