use std::cell::RefCell;
use std::rc::Rc;

use object::{PyObject, PyInnerObject};
use object::generic::*;
use object::typeobj::{PyTypeObject, PY_TYPE_TYPE};

fn list_len(v: Rc<PyObject>) -> Rc<PyObject> {
    match v.inner {
        PyInnerObject::ListObj(ref obj) => Rc::new(PyObject::from_i32(obj.list.borrow().len() as i32)),
        _ => panic!("TypeError: list_len")
    }
}

fn list_bool(v: Rc<PyObject>) -> Rc<PyObject> {
    match v.inner {
        PyInnerObject::ListObj(ref obj) => Rc::new(PyObject::from_bool(!(obj.list.borrow().is_empty()))),
        _ => panic!("TypeError: list_bool")
    }
}

thread_local! (
    pub static PY_LIST_TYPE: Rc<PyTypeObject> = {
        PY_TYPE_TYPE.with(|tp| {
            let tp = PyTypeObject {
                ob_type: Some(Rc::clone(&tp)),
                tp_name: "list".to_string(),
                tp_hash: None,
                tp_bool: Some(Box::new(list_bool)),
                tp_fun_eq: None,
                tp_fun_add: None,
                tp_fun_lt: None,
                tp_len: Some(Box::new(list_len)),
                tp_dict: None,
            };
            Rc::new(tp)
        })
    }
);

pub struct PyListObject {
    pub list: RefCell<Vec<Rc<PyObject>>>,
}

impl PyObject {
    pub fn from_vec(v: Vec<Rc<PyObject>>) -> PyObject {
        PY_LIST_TYPE.with(|tp| {
            let inner = PyListObject {
                list: RefCell::new(v.iter().map(|v|{ Rc::clone(&v) }).collect()),
            };
            PyObject {
                ob_type: Rc::clone(&tp),
                inner: PyInnerObject::ListObj(Rc::new(inner))
            }
        })
    }

    pub fn getitem_index(&self, key: &Rc<PyObject>) -> Option<Rc<PyObject>> {
        let key = pyobj_to_i32(Rc::clone(key));
        match self.inner {
            PyInnerObject::ListObj(ref obj) => {
                match obj.list.borrow().get(key as usize) {
                    Some(item) => Some(Rc::clone(item)),
                    None => None,
                }
            },
            _ => panic!("Type Error: getitem_index")
        }
    }
}
