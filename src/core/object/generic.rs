use std::rc::Rc;

use eval::{CtrlOp, Executable};
use env::*;
use syntax::*;
use object::*;
use object::boolobj::*;
use object::methodobj::*;
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

pub fn call_func(funv: Rc<PyObject>, args: &mut Vec<Rc<PyObject>>) -> Rc<PyObject> {
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
            vals.append(args);
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
                    } else {
                        (*fun)(Rc::clone(&args[0]))
                    }
                }
            }
        },
        PyInnerObject::TypeObj(ref _cls) => {
            let dictobj = PyObject::pydict_new();
            let instance = Rc::new(PyObject {
                ob_type: Some(Rc::clone(&funv)),
                inner: PyInnerObject::InstObj(Rc::new(
                    PyInstObject {
                        class: Rc::clone(&funv),
                        dict: dictobj,
                    }
                ))
            });
            match get_attr(&instance, &"__init__".to_string()) {
                Some(init_fun) => call_func(Rc::clone(&init_fun), args),
                None => PyObject::none_obj()
            };
            instance
        },
        _ => panic!("Type Error: Callable expected"),
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

pub fn get_attr(value: &Rc<PyObject>, key: &Id) -> Option<Rc<PyObject>> {
    let keyval = Rc::new(PyObject::from_string(key.clone()));
    match value.inner {
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
        _ => panic!("Type Error: get_attr")
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
