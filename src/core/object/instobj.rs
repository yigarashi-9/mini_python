use std::rc::Rc;

use object::PyObject;

pub struct PyInstObject {
    pub class: Rc<PyObject>,
    pub dict: Rc<PyObject>,
}
