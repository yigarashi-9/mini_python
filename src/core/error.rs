use std::cell::RefCell;
use std::rc::Rc;

use object::PyObject;

thread_local! (
    pub static EXC_INDICATOR: RefCell<Option<Rc<PyObject>>> = RefCell::new(None);
);

pub fn pyerr_occurred() -> bool {
    EXC_INDICATOR.with(|ind| { ind.borrow().is_some() })
}

pub fn pyerr_set(err: Rc<PyObject>) {
    EXC_INDICATOR.with(|ind| { ind.replace(Some(err)) });
}

pub fn pyerr_clear() {
    EXC_INDICATOR.with(|ind| { ind.replace(None) });
}
