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

use std::rc::Rc;

use self::boolobj::PyBoolObject;
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
    BoolObj(Rc<PyBoolObject>),
    DictObj(Rc<PyDictObject>),
    FunObj(Rc<PyFunObject>),
    InstObj(Rc<PyInstObject>),
    ListObj(Rc<PyListObject>),
    LongObj(Rc<PyLongObject>),
    MethodObj(Rc<PyMethodObject>),
    NoneObj,
    RustFunObj(Rc<PyRustFunObject>),
    StrObj(Rc<PyStringObject>),
    TypeObj(Rc<PyTypeObject>),
}

pub struct PyObject {
    pub ob_type: Rc<PyTypeObject>,
    pub inner: PyInnerObject,
}

impl PyObject {
    pub fn ob_type_ref(&self) -> &Rc<PyTypeObject> {
        &self.ob_type
    }
}

impl PartialEq for PyObject {
    fn eq(&self, other: &PyObject) -> bool {
        self as *const _ == other as *const _
    }
}
