use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;

use error::*;
use eval::PyRes;
use object::{PyObject, PyInnerObject};
use object::excobj::*;
use object::typeobj::*;

fn pystr_add(lv: Rc<PyObject>, rv: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
    match lv.inner {
        PyInnerObject::StrObj(ref l_obj) => {
            match rv.inner {
                PyInnerObject::StrObj(ref r_obj) => {
                    return Ok(PyObject::from_string(format!("{}{}", l_obj.s, r_obj.s)));
                },
                _ => {}
            }
        },
        _ => {}
    }

    pyerr_set_string(
        PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
        "__eq__ expects str objects"
    );
    Err(())
}

fn pystr_eq(lv: Rc<PyObject>, rv: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
    match lv.inner {
        PyInnerObject::StrObj(ref l_obj) => {
            match rv.inner {
                PyInnerObject::StrObj(ref r_obj) => {
                    return Ok(PyObject::from_bool(l_obj.s == r_obj.s));
                },
                _ => {}
            }
        },
        _ => {}
    }

    pyerr_set_string(
        PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
        "__eq__ expects str objects"
    );
    Err(())
}


fn pystr_hash(obj: Rc<PyObject>) -> PyRes<u64> {
    let mut hasher = DefaultHasher::new();
    match obj.inner {
        PyInnerObject::StrObj(ref obj) => obj.s.hash(&mut hasher),
        _ => {
            pyerr_set_string(
                PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                "__hash__ expects str objects"
            );
            return Err(());
        }
    };
    Ok(hasher.finish())
}

fn pystr_len(v: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
    match v.inner {
        PyInnerObject::StrObj(ref obj) => Ok(PyObject::from_i32(obj.s.len() as i32)),
        _ => {
            pyerr_set_string(
                PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                "__len__ expects str objects"
            );
            return Err(());
        }
    }
}

thread_local! (
    pub static PY_STRING_TYPE: Rc<PyObject> = {
        let strtp = PyTypeObject {
            tp_name: "str".to_string(),
            tp_hash: Some(Rc::new(pystr_hash)),
            tp_fun_eq: Some(Rc::new(pystr_eq)),
            tp_fun_add: Some(Rc::new(pystr_add)),
            tp_len: Some(Rc::new(pystr_len)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(strtp))),
        })
    }
);

pub struct PyStringObject {
    pub s: String,
}

impl PyObject {
    pub fn from_str(s: &str) -> Rc<PyObject> {
        PyObject::from_string(s.to_string())
    }

    pub fn from_string(raw_string: String) -> Rc<PyObject> {
        PY_STRING_TYPE.with(|tp| {
            let inner = PyStringObject { s: raw_string };
            Rc::new(PyObject {
                ob_type: Some(Rc::clone(&tp)),
                ob_dict: None,
                inner: PyInnerObject::StrObj(Rc::new(inner))
            })
        })
    }
}
