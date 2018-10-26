use std::rc::Rc;

use error::*;
use env::*;
use eval::{PyRes, eval};
use object::*;
use object::boolobj::*;
use object::excobj::*;
use object::methodobj::*;
use object::noneobj::*;
use object::rustfunobj::*;
use object::typeobj::*;

pub fn pyobj_to_bool(v: Rc<PyObject>) -> PyRes<bool> {
    let ob_type = v.ob_type();
    let typ = ob_type.pytype_typeobj_borrow();
    match typ.tp_bool.as_ref() {
        Some(ref fun) => {
            let res = fun(Rc::clone(&v))?;

            if PY_TRUE.with(|obj| { res == *obj }) {
                Ok(true)
            } else if PY_FALSE.with(|obj| { res == *obj }) {
                Ok(false)
            } else {
                pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                 "boolean object expected");
                Err(())
            }
        },
        None => match typ.tp_len.as_ref() {
            Some(ref fun) => Ok(pyobj_to_i32(fun(Rc::clone(&v))?)? > 0),
            None => {
                pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                 "boolean object expected");
                Err(())
            }
        }
    }
}

pub fn pyobj_to_i32(v: Rc<PyObject>) -> PyRes<i32> {
    match v.inner {
        PyInnerObject::LongObj(ref obj) => Ok(obj.n),
        _ => {
            pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                             "int object expected");
            Err(())
        }
    }
}

pub fn pyobj_to_string(v: Rc<PyObject>) -> PyRes<String> {
    match v.inner {
        PyInnerObject::StrObj(ref obj) => Ok(obj.s.clone()),
        _ => {
            pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                             "str object expected");
            Err(())
        }
    }
}

pub fn pyobj_issubclass(v: Rc<PyObject>, typ: Rc<PyObject>) -> bool {
    if !PyObject::pytype_check(&v) { return false }

    match v.pytype_tp_mro() {
        Some(ref tp_mro) => {
            for i in 0..(tp_mro.pylist_size()) {
                let base = tp_mro.pylist_getitem(i).unwrap();
                if base == typ { return true; }
            };
            false
        },
        None => panic!("Implementation Error: pyobj_issubclass")
    }
}

pub fn pyobj_isinstance(v: Rc<PyObject>, typ: Rc<PyObject>) -> bool {
    let v_type = v.ob_type();
    match v_type.pytype_tp_mro() {
        Some(ref tp_mro) => {
            for i in 0..(tp_mro.pylist_size()) {
                let base = tp_mro.pylist_getitem(i).unwrap();
                if base == typ { return true; }
            };
            false
        },
        None => panic!("Implementation Error: pyobj_issubclass")
    }
}

pub fn call_func(funv: Rc<PyObject>, args: &Vec<Rc<PyObject>>) -> PyRes<Rc<PyObject>> {
    match funv.inner {
        PyInnerObject::FunObj(ref fun) => {
            eval(&fun.codeobj.pycode_code(),
                 Rc::new(Env::new_child(&fun.env, &fun.codeobj.pycode_argnames(), args)))
        },
        PyInnerObject::MethodObj(ref method) => {
            let mut vals = vec![Rc::clone(&method.ob_self)];
            let mut args = args.clone();
            vals.append(&mut args);
            eval(&method.codeobj.pycode_code(),
                 Rc::new(Env::new_child(&method.env, &method.codeobj.pycode_argnames(), &vals)))
        },
        PyInnerObject::RustFunObj(ref obj) => {
            match obj.rust_fun {
                PyRustFun::MethO(ref fun) => {
                    if args.len() != 1 {
                        pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                         "1 argument expected");
                        return Err(())
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
                None => {
                    pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                     "callable object expected");
                    Err(())
                }
            }
        }
    }
}

pub fn bind_self(value: &Rc<PyObject>, slf: Rc<PyObject>) -> Rc<PyObject> {
    match value.inner {
        PyInnerObject::FunObj(ref fun) => {
            Rc::new(PyObject {
                ob_type: PY_METHOD_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
                ob_dict: None,
                inner: PyInnerObject::MethodObj(Rc::new(
                    PyMethodObject {
                        ob_self: Rc::clone(&slf),
                        env: Rc::clone(&fun.env),
                        codeobj: Rc::clone(&fun.codeobj),
                    }))
            })
        },
        PyInnerObject::RustFunObj(ref obj) => {
            Rc::new(PyObject {
                ob_type: PY_RUSTFUN_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
                ob_dict: None,
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

pub fn pyobj_get_attr(value: Rc<PyObject>, key: Rc<PyObject>) -> PyRes<Rc<PyObject>> {
    let ob_type = value.ob_type();
    let mut res = None;

    if let Some(ref tp_getattro) = ob_type.pytype_typeobj_borrow().tp_getattro {
        res = tp_getattro(value, key)?;
    };

    match res {
        Some(res) => Ok(res),
        None => {
            pyerr_set_string(
                PY_ATTRIBUTEERROR_TYPE.with(|tp| Rc::clone(tp)),
                "failed to find attribute");
            Err(())
        }
    }

}

pub fn pyobj_generic_get_attro(value: Rc<PyObject>, key: Rc<PyObject>) -> PyRes<Option<Rc<PyObject>>> {
    let mut ret_val = None;
    if let Some(ref ob_dict) = value.ob_dict {
        ret_val = ob_dict.pydict_lookup(Rc::clone(&key))?;
    };

    if ret_val.is_none() {
        ret_val = type_getattro(value.ob_type(), Rc::clone(&key))?;
    };

    match ret_val {
        Some(ret_val) => Ok(Some(bind_self(&ret_val, Rc::clone(&value)))),
        None => {
            pyerr_set_string(
                PY_ATTRIBUTEERROR_TYPE.with(|tp| Rc::clone(tp)),
                "failed to find attribute");
            Err(())
        }
    }
}

pub fn pyobj_set_attr(value: Rc<PyObject>, key: Rc<PyObject>, rvalue: Rc<PyObject>) -> PyRes<()> {
    let ob_type = value.ob_type();
    let typ = ob_type.pytype_typeobj_borrow();
    match typ.tp_setattro {
        Some(ref tp_setattro) => tp_setattro(value, key, rvalue),
        None => {
            pyerr_set_string(
                PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                "__setattribute__ expected");
            Err(())
        }
    }
}

pub fn pyobj_generic_set_attro(value: Rc<PyObject>, key: Rc<PyObject>, rvalue: Rc<PyObject>) -> PyRes<()> {
    match value.ob_dict {
        Some(ref ob_dict) => ob_dict.pydict_update(key, rvalue),
        None => panic!("Implementation Error: pyobj_generic_set_attro")
    }
}
