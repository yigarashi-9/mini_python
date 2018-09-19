use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use object::*;
use object::object::PyObject;
use object::dictobj::PyDictObject;

pub type HashFunc = Option<Box<dyn Fn(Rc<PyObject>) -> u64>>;
pub type UnaryOp = Option<Box<dyn Fn(Rc<PyObject>) -> Rc<PyObject>>>;
pub type BinaryOp = Option<Box<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>>>;

pub struct PyTypeObject {
    pub ob_type: Option<Rc<PyTypeObject>>,
    pub tp_name: String,
    pub tp_hash: HashFunc,
    pub tp_bool: UnaryOp,
    pub tp_fun_eq: BinaryOp,
    pub tp_fun_add: BinaryOp,
    pub tp_fun_lt: BinaryOp,
    pub tp_len: UnaryOp,
    pub tp_dict: Option<Rc<PyDictObject>>,
}

pub fn default_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    (&*obj as *const PyObject).hash(&mut hasher);
    hasher.finish()
}

thread_local! (
    pub static PY_TYPE_TYPE: Rc<PyTypeObject> = Rc::new(PyTypeObject::new_type())
);

impl PyTypeObject {
    pub fn new_type() -> PyTypeObject {
        PyTypeObject {
            ob_type: None,
            tp_name: "type".to_string(),
            tp_hash: Some(Box::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_dict: None,
        }
    }

    pub fn tp_dict_ref(&self) -> &Option<Rc<PyDictObject>> {
        &self.tp_dict
    }
}
