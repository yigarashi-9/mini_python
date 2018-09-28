use std::rc::Rc;

use eval::{CtrlOp, Executable};
use env::*;
use syntax::*;
use object::*;
use object::boolobj::*;
use object::methodobj::*;
use object::noneobj::*;
use object::rustfunobj::*;
use object::typeobj::*;

pub fn pyobj_is_bool(v: Rc<PyObject>) -> bool {
    let ob_type = v.ob_type();
    let typ = ob_type.pytype_typeobj_borrow();
    match typ.tp_bool.as_ref() {
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
        None => match typ.tp_len.as_ref() {
            Some(ref fun) => pyobj_to_i32(fun(Rc::clone(&v))) > 0,
            None => panic!("Type Error: pyobj_is_bool 3")
        }
    }
}

pub fn pyobj_to_i32(v: Rc<PyObject>) -> i32 {
    match v.inner {
        PyInnerObject::LongObj(ref obj) => obj.n,
        _ => panic!("Type Error: pyobj_to_i32"),
    }
}

pub fn call_func(funv: Rc<PyObject>, args: &Vec<Rc<PyObject>>) -> Rc<PyObject> {
    match funv.inner {
        PyInnerObject::FunObj(ref fun) => {
            match fun.code.exec(Rc::new(Env::new_child(&fun.env, &fun.parms, args))) {
                CtrlOp::Nop => PyObject::none_obj(),
                CtrlOp::Return(val) => val,
                _ => panic!("Invalid control operator"),
            }
        },
        PyInnerObject::MethodObj(ref method) => {
            let mut vals = vec![Rc::clone(&method.ob_self)];
            let mut args = args.clone();
            vals.append(&mut args);
            match method.code.exec(Rc::new(Env::new_child(&method.env, &method.parms, &vals))) {
                CtrlOp::Nop => PyObject::none_obj(),
                CtrlOp::Return(val) => val,
                _ => panic!("Invalid control operator"),
            }
        },
        PyInnerObject::RustFunObj(ref obj) => {
            match obj.rust_fun {
                PyRustFun::MethO(ref fun) => {
                    if args.len() != 1 {
                        panic!("Type error: call_func RustFunObj METH_O");
                    }
                    // Probably, slf cannot be None after module is implemented
                    let slf = match obj.ob_self {
                        Some(ref slf) => Rc::clone(slf),
                        None => PY_NONE_OBJECT.with(|ob| { Rc::clone(ob) })
                    };
                    (*fun)(slf, Rc::clone(&args[0]))
                }
            }
        },
        _ => {
            let ob_type = funv.ob_type();
            let typ = ob_type.pytype_typeobj_borrow();
            match typ.tp_call {
                Some(ref tp_call) => tp_call(Rc::clone(&funv), args),
                None => panic!("Type Error: Callable expected"),
            }
        }
    }
}

pub fn make_method(value: Rc<PyObject>, instance_ref: &Rc<PyObject>) -> Rc<PyObject> {
    match value.inner {
        PyInnerObject::FunObj(ref fun) => {
            PY_METHOD_TYPE.with(|tp| {
                Rc::new(PyObject {
                    ob_type: Some(Rc::clone(tp)),
                    inner: PyInnerObject::MethodObj(Rc::new(
                        PyMethodObject {
                            ob_self: Rc::clone(instance_ref),
                            env: Rc::clone(&fun.env),
                            parms: fun.parms.clone(),
                            code: fun.code.clone(),
                        }
                    ))
                })
            })
        },
        _ => Rc::clone(&value),
    }
}

pub fn get_attr_afterhook(value: &Rc<PyObject>, slf: Rc<PyObject>) -> Rc<PyObject> {
    match value.inner {
        PyInnerObject::RustFunObj(ref obj) => {
            Rc::new(PyObject {
                ob_type: PY_RUSTFUN_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
                inner: PyInnerObject::RustFunObj(Rc::new(PyRustFunObject {
                    name: obj.name.clone(),
                    ob_self: Some(Rc::clone(&slf)),
                    rust_fun: obj.rust_fun.clone(),
                }))
            })
        },
        _ => Rc::clone(value)
    }
}

pub fn get_attr(value: &Rc<PyObject>, key: &Id) -> Option<Rc<PyObject>> {
    let keyval = Rc::new(PyObject::from_string(key.clone()));
    let retval = match Rc::clone(value).inner {
        PyInnerObject::TypeObj(ref typ) => typ.borrow().tp_dict_ref().as_ref().unwrap().pydict_lookup(&keyval),
        PyInnerObject::InstObj(ref inst) => {
            match inst.dict.pydict_lookup(&keyval) {
                Some(ret_val) => Some(ret_val),
                None => {
                    if let Some(ref mro) = inst.class.pytype_tp_mro() {
                        if !(mro.pylist_check()) { return None }
                        for i in 0..(mro.pylist_size()) {
                            if let Some(ret_val) = get_attr(&mro.pylist_getitem(i), key) {
                                return Some(make_method(Rc::clone(&ret_val), &value))
                            }
                        }
                    };
                    None
                }
            }
        },
        _ => {
            let tp_dict = Rc::clone(&value).ob_type().pytype_tp_dict();
            match tp_dict {
                Some(tp_dict) => tp_dict.pydict_lookup(&keyval),
                None => None,
            }
        }
    };
    match retval {
        Some(ref retval) => Some(get_attr_afterhook(retval, Rc::clone(value))),
        None => None,
    }
}

pub fn update_attr(value: &Rc<PyObject>, key: Id, rvalue: Rc<PyObject>) {
    let keyval = PyObject::from_string(key.clone());
    let value = Rc::clone(value);
    match value.inner {
        PyInnerObject::TypeObj(ref typ) => {
            match typ.borrow().tp_dict_ref() {
                &Some(ref dict) => dict.pydict_update(Rc::clone(&keyval), Rc::clone(&rvalue)),
                &None => panic!("Type Error: update_attr")
            }
            update_slot(Rc::clone(&value), key.clone(), Rc::clone(&rvalue));
        },
        PyInnerObject::InstObj(ref inst) => {
            inst.dict.pydict_update(keyval, rvalue);
        },
        _ => panic!("Type Error: update_attr")
    }
}
