use std::cell::RefCell;
use std::rc::Rc;

use error::*;
use eval::PyRes;
use object::{PyObject, PyInnerObject};
use object::excobj::*;
use object::generic::*;
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};
use object::pyhashmap::PyHashMap;


thread_local! (
    pub static PY_DICT_TYPE: Rc<PyObject> = {
        let dicttp = PyTypeObject {
            tp_name: "dict".to_string(),
            tp_len: Some(Rc::new(PyObject::pydict_len)),
            tp_getattro: Some(Rc::new(pyobj_generic_get_attro)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(dicttp))),
        })
    }
);

pub struct PyDictObject {
    pub dict: RefCell<PyHashMap>,
}

impl PyObject {
    pub fn pydict_new() -> Rc<PyObject> {
        let inner =  PyDictObject {
            dict: RefCell::new(PyHashMap::new()),
        };
        Rc::new(PyObject {
            ob_type: PY_DICT_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::DictObj(Rc::new(inner))
        })
    }

    fn pydict_len(v: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
        match v.inner {
            PyInnerObject::DictObj(ref obj) => Ok(obj.dict.borrow().len()),
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "dict object expected");
                Err(())
            }
        }
    }

    pub fn pydict_check(&self) -> bool {
        PY_DICT_TYPE.with(|tp| { (&self.ob_type).as_ref() == Some(tp) })
    }

    pub fn pydict_lookup(&self, key: Rc<PyObject>) -> PyRes<Option<Rc<PyObject>>> {
        let ob_type = key.ob_type();
        let hash = match ob_type.pytype_typeobj_borrow().tp_hash {
            Some(ref tp_hash) => tp_hash(Rc::clone(&key))?,
            None => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "unhasshable dict key");
                return Err(());
            }
        };

        match self.inner {
            PyInnerObject::DictObj(ref obj) => {
                match obj.dict.borrow().get(hash) {
                    Some(v) => Ok(Some(Rc::clone(v))),
                    None => Ok(None)
                }
            },
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "__getitem__ expects dict object");
                Err(())
            }
        }
    }

    pub fn pydict_update(&self, key: Rc<PyObject>, value: Rc<PyObject>) -> PyRes<()> {
        let ob_type = key.ob_type();
        let hash = match ob_type.pytype_typeobj_borrow().tp_hash {
            Some(ref tp_hash) => tp_hash(Rc::clone(&key))?,
            None => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "unhasshable dict key");
                return Err(());
            }
        };

        match self.inner {
            PyInnerObject::DictObj(ref obj) => {
                obj.dict.borrow_mut().insert(hash, key, value);
                Ok(())
            },
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "__setitem__ expects dict object");
                Err(())
            }
        }
    }
}
