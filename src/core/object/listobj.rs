use std::cell::RefCell;
use std::rc::Rc;

use object::object::*;
use object::typeobj::*;

pub struct PyListObject {
    pub ob_type: Rc<PyTypeObject>,
    pub list: RefCell<Vec<Rc<PyObject>>>,
}

impl PyListObject {
    pub fn from_vec(v: Vec<Rc<PyObject>>) -> PyListObject {
        PyListObject {
            ob_type: Rc::new(PyTypeObject::new_list()),
            list: RefCell::new(v.iter().map(|v|{ Rc::clone(&v) }).collect()),
        }
    }

    pub fn getitem_index(&self, key: &Rc<PyObject>) -> Option<Rc<PyObject>> {
        let key = pyobj_to_i32(Rc::clone(key));
        print!("{}", key as usize);
        match self.list.borrow().get(key as usize) {
            Some(item) => Some(Rc::clone(item)),
            None => None,
        }
    }
}

fn list_len(v: Rc<PyObject>) -> Rc<PyObject> {
    match *v {
        PyObject::ListObj(ref obj) => Rc::new(PyObject::from_i32(obj.list.borrow().len() as i32)),
        _ => panic!("TypeError: list_len")
    }
}

fn list_bool(v: Rc<PyObject>) -> Rc<PyObject> {
    match *v {
        PyObject::ListObj(ref obj) => Rc::new(PyObject::from_bool(!(obj.list.borrow().is_empty()))),
        _ => panic!("TypeError: list_bool")
    }
}

pub fn new_list_type_object() -> PyTypeObject {
    PyTypeObject {
        ob_type: Some(Rc::new(PyTypeObject::new_type())),
        tp_name: "list".to_string(),
        tp_hash: None,
        tp_bool: Some(Box::new(list_bool)),
        tp_fun_eq: None,
        tp_fun_add: None,
        tp_fun_lt: None,
        tp_len: Some(Box::new(list_len)),
        tp_dict: None,
    }
}
