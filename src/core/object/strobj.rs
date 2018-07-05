use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use object::object::*;
use object::typeobj::*;

pub struct PyStringObject {
    pub ob_type: Rc<PyTypeObject>,
    s: String,
}

impl PyStringObject {
    pub fn from_string(raw_string: String) -> PyStringObject {
        PyStringObject {
            ob_type: Rc::new(PyTypeObject::new_str()),
            s: raw_string
        }
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

pub fn new_str_type_object() -> PyTypeObject {
    PyTypeObject {
        ob_type: Some(Rc::new(PyTypeObject::new_type())),
        tp_name: "str".to_string(),
        tp_hash: Some(Box::new(str_hash)),
        tp_fun_eq: Some(Box::new(eq_str_str)),
        tp_fun_add: None,
        tp_fun_lt: None,
        tp_dict: None,
    }
}
