use std::rc::Rc;

use object::longobj::*;
use object::boolobj::*;
use object::strobj::*;
use object::noneobj::*;
use object::dictobj::*;
use object::typeobj::*;
use object::methodobj::*;
use object::instobj::*;
use object::funobj::*;

pub enum PyObject {
    LongObj(Rc<PyLongObject>),
    BoolObj(Rc<PyBoolObject>),
    StrObj(Rc<PyStringObject>),
    NoneObj(Rc<PyNoneObject>),
    FunObj(Rc<PyFuncObject>),
    InstObj(Rc<PyInstObject>),
    MethodObj(Rc<PyMethodObject>),
    DictObj(Rc<PyDictObject>),
    TypeObj(Rc<PyTypeObject>),
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
            &PyObject::DictObj(ref obj) => Rc::clone(&obj.ob_type),
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

    pub fn none_obj() -> PyObject {
        PyObject::NoneObj(Rc::new(PyNoneObject { ob_type: Rc::new(PyTypeObject::new_none()) }))
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
