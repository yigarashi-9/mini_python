use std::rc::Rc;

use env::*;

use object::*;
use object::boolobj::*;
use object::excobj::*;
use object::listobj::*;
use object::longobj::*;
use object::rustfunobj::*;
use object::typeobj::*;

fn builtin_len(_module: Rc<PyObject>, obj: Rc<PyObject>) -> Rc<PyObject> {
    let ob_type = obj.ob_type();
    let typ = ob_type.pytype_typeobj_borrow();
    match typ.tp_len {
        Some(ref fun) => (*fun)(Rc::clone(&obj)),
        None => panic!("Type Error: builtin_len"),
    }
}

macro_rules! set_builtin_fun {
    ($env:expr, $id:expr, $flag:ident, $fun:ident) => {
        let inner = PyRustFunObject {
            name: $id.to_string(),
            ob_self: None,
            rust_fun: PyRustFun::$flag(Rc::new($fun))
        };
        let obj = Rc::new(PyObject {
            ob_type: PY_RUSTFUN_TYPE.with(|tp| { Some(Rc::clone(&tp)) }),
            ob_dict: None,
            inner: PyInnerObject::RustFunObj(Rc::new(inner))
        });
        $env.update($id.to_string(), obj);
    }
}

pub fn load_builtins(env: Rc<Env>) {
    set_builtin_fun!(env, "len", MethO, builtin_len);
    env.update("type".to_string(), PY_TYPE_TYPE.with(|tp| { Rc::clone(tp) }));
    env.update("int".to_string(), PY_LONG_TYPE.with(|tp| { Rc::clone(tp) }));
    env.update("bool".to_string(), PY_BOOL_TYPE.with(|tp| { Rc::clone(tp) }));
    env.update("Exception".to_string(), PY_EXC_TYPE.with(|tp| { Rc::clone(tp) }));
    PY_BOOL_TYPE.with(|booltp| { pytype_ready(Rc::clone(booltp)) });
    PY_LIST_TYPE.with(|listtp| { pytype_ready(Rc::clone(listtp)) });
    PY_EXC_TYPE.with(|exctp| { pytype_ready(Rc::clone(exctp)) });
}
