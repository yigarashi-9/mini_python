use std::rc::Rc;

use object::boolobj::*;
use object::dictobj::*;
use object::funobj::*;
use object::instobj::*;
use object::listobj::*;
use object::longobj::*;
use object::methodobj::*;
use object::noneobj::*;
use object::rustfunobj::*;
use object::strobj::*;
use object::typeobj::*;


pub enum PyObject {
    BoolObj(Rc<PyBoolObject>),
    DictObj(Rc<PyDictObject>),
    FunObj(Rc<PyFunObject>),
    InstObj(Rc<PyInstObject>),
    ListObj(Rc<PyListObject>),
    LongObj(Rc<PyLongObject>),
    MethodObj(Rc<PyMethodObject>),
    NoneObj(Rc<PyNoneObject>),
    RustFunObj(Rc<PyRustFunObject>),
    StrObj(Rc<PyStringObject>),
    TypeObj(Rc<PyTypeObject>),
}

impl PartialEq for PyObject {
    fn eq(&self, other: &PyObject) -> bool {
        self as *const _ == other as *const _
    }
}

impl PyObject {
    pub fn ob_type(&self) -> Rc<PyTypeObject> {
        match self {
            &PyObject::LongObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::BoolObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::StrObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::NoneObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::FunObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::InstObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::MethodObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::ListObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::DictObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::RustFunObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::TypeObj(ref obj) =>
                match obj.ob_type {
                    Some(ref ob_type) => Rc::clone(ob_type),
                    None => Rc::clone(obj),
                },
        }
    }

    pub fn from_i32(raw_i32: i32) -> PyObject {
        PyObject::LongObj(Rc::new(PyLongObject::from_i32(raw_i32)))
    }

    pub fn from_bool(raw_bool: bool) -> PyObject {
        PyObject::BoolObj(Rc::new(PyBoolObject::from_bool(raw_bool)))
    }

    pub fn from_str(s: &str) -> PyObject {
        PyObject::from_string(s.to_string())
    }

    pub fn from_string(raw_string: String) -> PyObject {
        PyObject::StrObj(Rc::new(PyStringObject::from_string(raw_string)))
    }

    pub fn from_vec(v: Vec<Rc<PyObject>>) -> PyObject {
        PyObject::ListObj(Rc::new(PyListObject::from_vec(v)))
    }

    pub fn none_obj() -> PyObject {
        PY_NONE_TYPE.with(|tp| {
            PyObject::NoneObj(Rc::new(PyNoneObject { ob_type: Rc::clone(&tp) }))
        })
    }

    pub fn new_dict() -> PyObject {
        PyObject::DictObj(Rc::new(PyDictObject::new()))
    }

    pub fn lookup(&self, key: &Rc<PyObject>) -> Option<Rc<PyObject>> {
        match self {
            &PyObject::DictObj(ref obj) => obj.lookup(key),
            _ => panic!("Type Error: PyObject lookup")
        }
    }

    pub fn update(&self, key: Rc<PyObject>, value: Rc<PyObject>) {
        match self {
            &PyObject::DictObj(ref obj) => obj.update(key, value),
            _ => panic!("Type Error: PyObject update")
        }
    }
}

pub fn pyobj_is_bool(v: Rc<PyObject>) -> bool {
    match Rc::clone(&v).ob_type().tp_bool.as_ref() {
        Some(ref fun) => {
            match *fun(Rc::clone(&v)) {
                PyObject::BoolObj(ref obj) => obj.b,
                _ => panic!("Type Error: pyobj_is_bool")
            }
        },
        None => {
            match Rc::clone(&v).ob_type().tp_len.as_ref() {
                Some(ref fun) => {
                    match *fun(Rc::clone(&v)) {
                        PyObject::LongObj(ref obj) => obj.n > 0,
                        _ => panic!("Type Error: pyobj_is_bool")
                    }
                },
                None => panic!("Type Error: pyobj_is_bool")
            }
        }
    }
}

pub fn pyobj_to_i32(v: Rc<PyObject>) -> i32 {
    match *v {
        PyObject::LongObj(ref obj) => obj.n,
        PyObject::BoolObj(ref obj) => if obj.b { 1 } else { 0 },
        _ => panic!("Type Error: pyobj_to_i32"),
    }
}
