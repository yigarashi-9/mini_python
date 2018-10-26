use std::cell::RefCell;
use std::rc::Rc;

use error::*;
use eval::PyRes;
use object::{PyObject, PyInnerObject};
use object::excobj::*;
use object::noneobj::*;
use object::rustfunobj::*;
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};


thread_local! (
    pub static PY_LIST_TYPE: Rc<PyObject> = {
        let mut tp_methods = vec![];
        tp_methods.push(Rc::new(PyObject {
            ob_type: PY_RUSTFUN_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::RustFunObj(Rc::new(PyRustFunObject {
                name: "append".to_string(),
                ob_self: None,
                rust_fun: PyRustFun::MethO(Rc::new(PyObject::pylist_append)),
            })),
        }));
        let listtp = PyTypeObject {
            tp_name: "list".to_string(),
            tp_bool: Some(Rc::new(PyObject::pylist_bool)),
            tp_len: Some(Rc::new(PyObject::pylist_len)),
            tp_iter: Some(Rc::new(PyObject::pylist_iter)),
            tp_methods: Some(tp_methods),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(listtp))),
        })
    }
);

pub struct PyListObject {
    list: RefCell<Vec<Rc<PyObject>>>,
}

impl PyObject {
    pub fn pylist_from_vec(v: &Vec<Rc<PyObject>>) -> Rc<PyObject> {
        PY_LIST_TYPE.with(|tp| {
            let inner = PyListObject {
                list: RefCell::new(v.iter().map(|v|{ Rc::clone(&v) }).collect()),
            };
            Rc::new(PyObject {
                ob_type: Some(Rc::clone(&tp)),
                ob_dict: None,
                inner: PyInnerObject::ListObj(Rc::new(inner))
            })
        })
    }

    pub fn pylist_check(&self) -> bool {
        PY_LIST_TYPE.with(|tp| { (&self.ob_type).as_ref() == Some(tp) })
    }

    fn pylist_len(self: Rc<Self>) -> PyRes<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::ListObj(ref obj) => Ok(PyObject::from_i32(obj.list.borrow().len() as i32)),
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "len expects list object"
                );
                Err(())
            }
        }
    }

    fn pylist_bool(self: Rc<Self>) -> PyRes<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::ListObj(ref obj) => Ok(PyObject::from_bool(!(obj.list.borrow().is_empty()))),
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "len expects list object"
                );
                Err(())
            }
        }
    }

    pub fn pylist_getitem(&self, index: usize) -> PyRes<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::ListObj(ref obj) => {
                match obj.list.borrow().get(index) {
                    Some(item) => Ok(Rc::clone(item)),
                    None => {
                        pyerr_set_string(
                            PY_INDEXERROR_TYPE.with(|tp| Rc::clone(tp)),
                            "len expects list object"
                        );
                        Err(())
                    }
                }
            },
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "__getitem__ expects list object"
                );
                Err(())
            }
        }
    }

    pub fn pylist_size(&self) -> usize {
        match self.inner {
            PyInnerObject::ListObj(ref obj) => {
                obj.list.borrow().len()
            },
            _ => panic!("Type Error: pylist_size")
        }
    }

    pub fn pylist_clone(&self) -> Vec<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::ListObj(ref obj) => {
                obj.list.borrow().clone()
            },
            _ => panic!("Type Error: pylist_clone")
        }
    }

    pub fn pylist_append(self: Rc<Self>, elm: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::ListObj(ref obj) => {
                obj.list.borrow_mut().push(Rc::clone(&elm));
                Ok(PY_NONE_OBJECT.with(|ob| { Rc::clone(ob) }))
            },
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "append expects list object"
                );
                Err(())
            }
        }
    }

    pub fn pylist_iter(self: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
        if !self.pylist_check() {
            pyerr_set_string(
                PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                "__iter__ expects list object"
            );
            return Err(())
        };
        Ok(Rc::new(PyObject {
            ob_type: PY_LISTITER_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::ListIterObj(Rc::new(RefCell::new(
                PyListIterObject {
                    it_index: 0,
                    it_seq: self,
                }
            )))
        }))
    }
}

pub struct PyListIterObject {
    it_index: usize,
    it_seq: Rc<PyObject>,
}

thread_local! (
    pub static PY_LISTITER_TYPE: Rc<PyObject> = {
        let itertp = PyTypeObject {
            tp_name: "listiter".to_string(),
            tp_iternext: Some(Rc::new(PyObject::pylistiter_next)),
            ..Default::default()
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(itertp))),
        })
    }
);

impl PyObject {
    pub fn pylistiter_check(&self) -> bool {
        PY_LISTITER_TYPE.with(|tp| { (&self.ob_type).as_ref() == Some(tp) })
    }

    pub fn pylistiter_next(self: Rc<PyObject>) -> PyRes<Option<Rc<PyObject>>> {
        match self.inner {
            PyInnerObject::ListIterObj(ref it) => {
                let mut it = it.borrow_mut();
                if it.it_index >= it.it_seq.pylist_size() {
                    Ok(None)
                } else {
                    let res = it.it_seq.pylist_getitem(it.it_index).unwrap();
                    it.it_index += 1;
                    Ok(Some(res))
                }
            },
            _ => {
                pyerr_set_string(
                    PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                    "__next__ expects listiter object"
                );
                Err(())
            }
        }
    }
}
