use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;

use error::*;
use eval::PyRes;
use object::{PyObject, PyInnerObject};
use object::excobj::*;
use object::typeobj::*;


thread_local! (
    pub static PY_LONG_TYPE: Rc<PyObject> = {
        let longtp = PyTypeObject {
            tp_name: "int".to_string(),
            tp_hash: Some(Rc::new(PyObject::pylong_hash)),
            tp_bool: Some(Rc::new(PyObject::pylong_bool)),
            tp_fun_eq: Some(Rc::new(PyObject::pylong_eq)),
            tp_fun_add: Some(Rc::new(PyObject::pylong_add)),
            tp_fun_lt: Some(Rc::new(PyObject::pylong_lt)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(longtp))),
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
                ob_type: Some(Rc::clone(&tp)),
                ob_dict: None,
                inner: PyInnerObject::LongObj(Rc::new(inner))
            })
        })
    }

    fn pylong_eq(self: Rc<Self>, rv: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::LongObj(ref l_obj) => {
                match rv.inner {
                    PyInnerObject::LongObj(ref r_obj) => {
                        return Ok(PyObject::from_bool(l_obj.n == r_obj.n));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        pyerr_set_string(
            PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
            "__eq__ expects int objects"
        );
        Err(())
    }

    fn pylong_add(self: Rc<Self>, rv: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::LongObj(ref l_obj) => {
                match rv.inner {
                    PyInnerObject::LongObj(ref r_obj) => {
                        return Ok(PyObject::from_i32(l_obj.n + r_obj.n));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        pyerr_set_string(
            PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
            "__add__ expects int objects"
        );
        Err(())
    }

    fn pylong_lt(self: Rc<Self>, rv: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::LongObj(ref l_obj) => {
                match rv.inner {
                    PyInnerObject::LongObj(ref r_obj) => {
                        return Ok(PyObject::from_bool(l_obj.n < r_obj.n));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        pyerr_set_string(
            PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
            "__lt__ expects int objects"
        );
        Err(())
    }

    fn pylong_hash(obj: Rc<PyObject>) -> PyRes<u64> {
        let mut hasher = DefaultHasher::new();
        match obj.inner {
            PyInnerObject::LongObj(ref obj) => obj.n.hash(&mut hasher),
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "__hash__ expects int objects"
                );
                return Err(());
            }
        };
        Ok(hasher.finish())
    }

    fn pylong_bool(v: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
        match v.inner {
            PyInnerObject::LongObj(ref obj) => Ok(PyObject::from_bool(obj.n > 0)),
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "__bool__ expects int objects"
                );
                Err(())
            }
        }
    }
}
