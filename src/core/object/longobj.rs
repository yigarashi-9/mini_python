use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};

fn eq_long_long(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match lv.inner {
        PyInnerObject::LongObj(ref l_obj) => {
            match rv.inner {
                PyInnerObject::LongObj(ref r_obj) =>
                    PyObject::from_bool(l_obj.n == r_obj.n),
                _ => panic!("Type Error: eq_long_long"),
            }
        },
        _ => panic!("Type Error: eq_long_long"),
    }
}

fn add_long_long(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match lv.inner {
        PyInnerObject::LongObj(ref l_obj) => {
            match rv.inner {
                PyInnerObject::LongObj(ref r_obj) => PyObject::from_i32(l_obj.n + r_obj.n),
                _ => panic!("Type Error: add_long_long"),
            }
        },
        _ => panic!("Type Error: add_long_long"),
    }
}

fn lt_long_long(lv: Rc<PyObject>, rv: Rc<PyObject>) -> Rc<PyObject> {
    match lv.inner {
        PyInnerObject::LongObj(ref l_obj) => {
            match rv.inner {
                PyInnerObject::LongObj(ref r_obj) => PyObject::from_bool(l_obj.n < r_obj.n),
                _ => panic!("Type Error: lt_long_long"),
            }
        },
        _ => panic!("Type Error: lt_long_long"),
    }
}

fn long_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    match obj.inner {
        PyInnerObject::LongObj(ref obj) => obj.n.hash(&mut hasher),
        _ => panic!("Type Error: long_hash")
    };
    hasher.finish()
}

fn long_bool(v: Rc<PyObject>) -> Rc<PyObject> {
    match v.inner {
        PyInnerObject::LongObj(ref obj) => PyObject::from_bool(obj.n > 0),
        _ => panic!("Type Error: long_bool")
    }
}

thread_local! (
    pub static PY_LONG_TYPE: Rc<RefCell<PyTypeObject>> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "int".to_string(),
                tp_base: None,
                tp_hash: Some(Rc::new(long_hash)),
                tp_bool: Some(Rc::new(long_bool)),
                tp_fun_eq: Some(Rc::new(eq_long_long)),
                tp_fun_add: Some(Rc::new(add_long_long)),
                tp_fun_lt: Some(Rc::new(lt_long_long)),
                tp_len: None,
                tp_dict: None,
            };
            Rc::new(RefCell::new(tp))
        })
    }
);

pub struct PyLongObject {
    pub n: i32,
}

impl PyObject {
    pub fn from_i32(raw_i32: i32) -> Rc<PyObject> {
        PY_LONG_TYPE.with(|tp| {
            let inner = PyLongObject { n: raw_i32 };
            Rc::new(PyObject {
                ob_type: Rc::clone(&tp),
                inner: PyInnerObject::LongObj(Rc::new(inner))
            })
        })
    }
}
