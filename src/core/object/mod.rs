pub mod boolobj;
pub mod codeobj;
pub mod dictobj;
pub mod funobj;
pub mod generic;
pub mod listobj;
pub mod longobj;
pub mod methodobj;
pub mod noneobj;
pub mod pyhashmap;
pub mod rustfunobj;
pub mod strobj;
pub mod typeobj;

use std::cell::RefCell;
use std::rc::Rc;

use self::codeobj::PyCodeObject;
use self::dictobj::PyDictObject;
use self::funobj::PyFunObject;
use self::listobj::{PyListObject, PyListIterObject};
use self::longobj::PyLongObject;
use self::methodobj::PyMethodObject;
use self::rustfunobj::PyRustFunObject;
use self::strobj::PyStringObject;
use self::typeobj::PyTypeObject;

pub enum PyInnerObject {
    CodeObj(Rc<PyCodeObject>),
    DictObj(Rc<PyDictObject>),
    FunObj(Rc<PyFunObject>),
    InstObj,
    ListObj(Rc<PyListObject>),
    ListIterObj(Rc<RefCell<PyListIterObject>>),
    LongObj(Rc<PyLongObject>),
    MethodObj(Rc<PyMethodObject>),
    NoneObj,
    RustFunObj(Rc<PyRustFunObject>),
    StrObj(Rc<PyStringObject>),
    TypeObj(Rc<RefCell<PyTypeObject>>),
}

pub struct PyObject {
    pub ob_type: Option<Rc<PyObject>>,
    pub ob_dict: Option<Rc<PyObject>>,
    pub inner: PyInnerObject,
}

impl PyObject {
    pub fn ob_type(self: &Rc<Self>) -> Rc<PyObject> {
        match self.ob_type {
            Some(ref obj) => Rc::clone(obj),
            None => Rc::clone(self)
        }
    }
}

impl PartialEq for PyObject {
    fn eq(&self, other: &PyObject) -> bool {
        self as *const _ == other as *const _
    }
}
