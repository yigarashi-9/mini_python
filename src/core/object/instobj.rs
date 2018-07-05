use std::rc::Rc;

use object::typeobj::PyTypeObject;
use object::dictobj::PyDictObject;

pub struct PyInstObject {
    pub ob_type: Rc<PyTypeObject>,
    pub class: Rc<PyTypeObject>,
    pub dict: Rc<PyDictObject>,
}
