use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use object::PyObject;

pub type HashFunc = Option<Rc<dyn Fn(Rc<PyObject>) -> u64>>;
pub type UnaryOp = Option<Rc<dyn Fn(Rc<PyObject>) -> Rc<PyObject>>>;
pub type BinaryOp = Option<Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>>>;

pub struct PyTypeObject {
    pub ob_type: Option<Rc<RefCell<PyTypeObject>>>,
    pub tp_name: String,
    pub tp_base: Option<Rc<RefCell<PyTypeObject>>>,
    pub tp_hash: HashFunc,
    pub tp_bool: UnaryOp,
    pub tp_fun_eq: BinaryOp,
    pub tp_fun_add: BinaryOp,
    pub tp_fun_lt: BinaryOp,
    pub tp_len: UnaryOp,
    pub tp_dict: Option<Rc<PyObject>>,
    pub tp_subclasses: Option<Rc<PyObject>>,
}

pub fn default_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    (&*obj as *const PyObject).hash(&mut hasher);
    hasher.finish()
}

thread_local! (
    pub static PY_TYPE_TYPE: Rc<RefCell<PyTypeObject>>
        = Rc::new(RefCell::new(PyTypeObject::new_type()))
);

impl PyTypeObject {
    pub fn new_type() -> PyTypeObject {
        PyTypeObject {
            ob_type: None,
            tp_name: "type".to_string(),
            tp_base: None,
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_dict: None,
            tp_subclasses: None,
        }
    }

    pub fn tp_dict_ref(&self) -> &Option<Rc<PyObject>> {
        &self.tp_dict
    }
}


pub fn pytype_ready(typ: &mut PyTypeObject) {
    if typ.tp_base.is_none() {
        return
    }

    let tp_base = Rc::clone(typ.tp_base.as_ref().unwrap());
    let tp_base_borrowed = tp_base.borrow();

    if typ.tp_hash.is_none() && tp_base_borrowed.tp_hash.is_some() {
        typ.tp_hash = tp_base_borrowed.tp_hash.clone();
    }

    if typ.tp_bool.is_none() && tp_base_borrowed.tp_bool.is_some() {
        typ.tp_bool = tp_base_borrowed.tp_bool.clone();
    }

    if typ.tp_fun_eq.is_none() && tp_base_borrowed.tp_fun_eq.is_some() {
        typ.tp_fun_eq = tp_base_borrowed.tp_fun_eq.clone();
    }

    if typ.tp_fun_add.is_none() && tp_base_borrowed.tp_fun_add.is_some() {
        typ.tp_fun_add = tp_base_borrowed.tp_fun_add.clone();
    }

    if typ.tp_fun_lt.is_none() && tp_base_borrowed.tp_fun_lt.is_some() {
        typ.tp_fun_lt = tp_base_borrowed.tp_fun_lt.clone();
    }
}
