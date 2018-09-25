use std::rc::Rc;

use object::*;
use object::boolobj::*;

pub fn pyobj_is_bool(v: Rc<PyObject>) -> bool {
    let typ = Rc::clone(&v.ob_type);
    let typ_borrowed = typ.borrow();
    match typ_borrowed.tp_bool.as_ref() {
        Some(ref fun) => {
            let res = fun(Rc::clone(&v));

            if PY_TRUE.with(|obj| { res == *obj }) {
                true
            } else if PY_FALSE.with(|obj| { res == *obj }) {
                false
            } else {
                panic!("Type Error: pyobj_is_bool 1")
            }
        },
        None => {
            match typ_borrowed.tp_len.as_ref() {
                Some(ref fun) => {
                    match fun(Rc::clone(&v)).inner {
                        PyInnerObject::LongObj(ref obj) => obj.n > 0,
                        _ => panic!("Type Error: pyobj_is_bool 2")
                    }
                },
                None => panic!("Type Error: pyobj_is_bool 3")
            }
        }
    }
}

pub fn pyobj_to_i32(v: Rc<PyObject>) -> i32 {
    match v.inner {
        PyInnerObject::LongObj(ref obj) => obj.n,
        _ => panic!("Type Error: pyobj_to_i32"),
    }
}
