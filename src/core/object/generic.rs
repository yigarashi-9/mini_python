use std::rc::Rc;

use object::*;

pub fn pyobj_is_bool(v: Rc<PyObject>) -> bool {
    match Rc::clone(&v).ob_type.tp_bool.as_ref() {
        Some(ref fun) => {
            match fun(Rc::clone(&v)).inner {
                PyInnerObject::BoolObj(ref obj) => obj.b,
                _ => panic!("Type Error: pyobj_is_bool")
            }
        },
        None => {
            match Rc::clone(&v).ob_type.tp_len.as_ref() {
                Some(ref fun) => {
                    match fun(Rc::clone(&v)).inner {
                        PyInnerObject::LongObj(ref obj) => obj.n > 0,
                        _ => panic!("Type Error: pyobj_is_bool")
                    }
                },
                None => panic!("Type Error: pyobj_is_bool")
            }
        }
    }
}

pub fn pyobj_to_i32(v: Rc<PyObject>) -> i32 {
    match v.inner {
        PyInnerObject::LongObj(ref obj) => obj.n,
        PyInnerObject::BoolObj(ref obj) => if obj.b { 1 } else { 0 },
        _ => panic!("Type Error: pyobj_to_i32"),
    }
}
