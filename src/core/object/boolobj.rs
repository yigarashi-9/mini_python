use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};

fn bool_bool(v: Rc<PyObject>) -> Rc<PyObject> {
    v
}

fn eq_bool_bool(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match lv.inner {
        PyInnerObject::BoolObj(ref l_obj) => {
            match rv.inner {
                PyInnerObject::BoolObj(ref r_obj) =>
                    Rc::new(PyObject::from_bool(l_obj.b == r_obj.b)),
                _ => panic!("Type Error: eq_bool_bool"),
            }
        },
        _ => panic!("Type Error: eq_bool_bool"),
    }
}

fn bool_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    match obj.inner {
        PyInnerObject::BoolObj(ref obj) => obj.b.hash(&mut hasher),
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
    pub b: bool,
}

impl PyObject {
    pub fn from_bool(raw_bool: bool) -> PyObject {
        PY_BOOL_TYPE.with(|tp| {
            let inner = PyBoolObject { b: raw_bool };
            PyObject {
                ob_type: Rc::clone(&tp),
                inner: PyInnerObject::BoolObj(Rc::new(inner))
            }
        })
    }
}
