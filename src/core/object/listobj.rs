use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::noneobj::*;
use object::rustfunobj::*;
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};

fn list_len(v: Rc<PyObject>) -> Rc<PyObject> {
    match v.inner {
        PyInnerObject::ListObj(ref obj) => PyObject::from_i32(obj.list.borrow().len() as i32),
        _ => panic!("TypeError: list_len")
    }
}

fn list_bool(v: Rc<PyObject>) -> Rc<PyObject> {
    match v.inner {
        PyInnerObject::ListObj(ref obj) => PyObject::from_bool(!(obj.list.borrow().is_empty())),
        _ => panic!("TypeError: list_bool")
    }
}

thread_local! (
    pub static PY_LIST_TYPE: Rc<PyObject> = {
        let mut tp_methods = vec![];
        tp_methods.push(Rc::new(PyObject {
            ob_type: PY_RUSTFUN_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::RustFunObj(Rc::new(PyRustFunObject {
                name: "append".to_string(),
                ob_self: None,
                rust_fun: PyRustFun::MethO(Rc::new(pylist_append)),
            })),
        }));
        let listtp = PyTypeObject {
            tp_name: "list".to_string(),
            tp_base: None,
            tp_hash: None,
            tp_bool: Some(Rc::new(list_bool)),
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: Some(Rc::new(list_len)),
            tp_call: None,
            tp_getattro: None,
            tp_methods: Some(tp_methods),
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
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

    pub fn pylist_getitem(&self, index: usize) -> Rc<PyObject> {
        match self.inner {
            PyInnerObject::ListObj(ref obj) => {
                match obj.list.borrow().get(index) {
                    Some(item) => Rc::clone(item),
                    None => panic!("Out of range Error: pylist_getitem")
                }
            },
            _ => panic!("Type Error: pylist_getitem")
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

}

pub fn pylist_append(slf: Rc<PyObject>, elm: Rc<PyObject>) -> Rc<PyObject> {
    match slf.inner {
        PyInnerObject::ListObj(ref obj) => {
            obj.list.borrow_mut().push(Rc::clone(&elm));
            PY_NONE_OBJECT.with(|ob| { Rc::clone(ob) })
        },
        _ => panic!("Type Error: pylist_append")
    }
}
