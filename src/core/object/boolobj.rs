use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use object::object::*;
use object::typeobj::*;

fn bool_bool(v: Rc<PyObject>) -> Rc<PyObject> {
    v
}

fn eq_bool_bool(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match *lv {
        PyObject::BoolObj(ref l_obj) => {
            match *rv {
                PyObject::BoolObj(ref r_obj) =>
                    Rc::new(PyObject::from_bool(l_obj.b == r_obj.b)),
                _ => panic!("Type Error: eq_bool_bool"),
            }
        },
        _ => panic!("Type Error: eq_bool_bool"),
    }
}

fn bool_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    match *obj {
        PyObject::BoolObj(ref obj) => obj.b.hash(&mut hasher),
        _ => panic!("Type Error: bool_hash")
    };
    hasher.finish()
}

thread_local! (
    pub static PY_BOOL_TYPE: Rc<PyTypeObject> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "bool".to_string(),
                tp_hash: Some(Box::new(bool_hash)),
                tp_bool: Some(Box::new(bool_bool)),
                tp_fun_eq: Some(Box::new(eq_bool_bool)),
                tp_fun_add: None,
                tp_fun_lt: None,
                tp_len: None,
                tp_dict: None,
            };
            Rc::new(tp)
        })
    }
);

pub struct PyBoolObject {
    pub ob_type: Rc<PyTypeObject>,
    pub b: bool,
}

impl PyBoolObject {
    pub fn from_bool(raw_bool: bool) -> PyBoolObject {
        PY_BOOL_TYPE.with(|tp| {
            PyBoolObject {
                ob_type: Rc::clone(&tp),
                b: raw_bool,
            }
        })
    }
}
