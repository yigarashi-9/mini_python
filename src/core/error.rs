use std::cell::RefCell;
use std::rc::Rc;

use object::PyObject;
use object::generic::*;

thread_local! (
    pub static EXC_INDICATOR: RefCell<Option<Rc<PyObject>>> = RefCell::new(None);
);

pub fn pyerr_occurred() -> bool {
    EXC_INDICATOR.with(|ind| { ind.borrow().is_some() })
}

pub fn pyerr_check(exception: Rc<PyObject>) -> bool {
    EXC_INDICATOR.with(|ind| {
        match ind.borrow().as_ref() {
            Some(exc) => pyobj_isinstance(Rc::clone(exc), exception),
            None => false,
        }
    })
}

pub fn pyerr_set(err: Rc<PyObject>) {
    EXC_INDICATOR.with(|ind| { ind.replace(Some(err)) });
}

pub fn pyerr_set_string(exception: Rc<PyObject>, s: &str) {
    if !PyObject::pyexc_is_exc_subclass(Rc::clone(&exception)) {
        panic!("Implementation Error: pyerr_set_string");
    }

    match call_func(exception, &vec![PyObject::from_str(s)]) {
        Ok(err) => {
            pyerr_set(err);
        },
        Err(_) => {} // エラーを入れ子にする仕組みが必要
    }
}

pub fn pyerr_clear() {
    EXC_INDICATOR.with(|ind| { ind.replace(None) });
}
