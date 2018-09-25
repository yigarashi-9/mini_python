pub mod boolobj;
pub mod dictobj;
pub mod funobj;
pub mod generic;
pub mod instobj;
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

use self::dictobj::PyDictObject;
use self::funobj::PyFunObject;
use self::instobj::PyInstObject;
use self::listobj::PyListObject;
use self::longobj::PyLongObject;
use self::methodobj::PyMethodObject;
use self::rustfunobj::PyRustFunObject;
use self::strobj::PyStringObject;
use self::typeobj::PyTypeObject;

pub enum PyInnerObject {
    DictObj(Rc<PyDictObject>),
    FunObj(Rc<PyFunObject>),
    InstObj(Rc<PyInstObject>),
    ListObj(Rc<PyListObject>),
    LongObj(Rc<PyLongObject>),
    MethodObj(Rc<PyMethodObject>),
    NoneObj,
    RustFunObj(Rc<PyRustFunObject>),
    StrObj(Rc<PyStringObject>),
    TypeObj(Rc<RefCell<PyTypeObject>>),
}

pub struct PyObject {
    pub ob_type: Rc<RefCell<PyTypeObject>>,
    pub inner: PyInnerObject,
}

impl PyObject {
    pub fn ob_type_ref(&self) -> &Rc<RefCell<PyTypeObject>> {
        &self.ob_type
    }
}

impl PartialEq for PyObject {
    fn eq(&self, other: &PyObject) -> bool {
        self as *const _ == other as *const _
    }
}
