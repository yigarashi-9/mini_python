use std::rc::Rc;

use env::*;

use object::object::*;
use object::rustfunobj::*;

fn builtin_len(obj: Rc<PyObject>) -> Rc<PyObject> {
    match obj.ob_type().tp_len {
        Some(ref fun) => (*fun)(Rc::clone(&obj)),
        None => panic!("Type Error: builtin_len"),
    }
}

macro_rules! set_builtin_fun {
    ($env:expr, $id:expr, $flag:ident, $fun:ident) => {
        PY_RUSTFUN_TYPE.with(|tp| {
            let obj = PyRustFunObject {
                ob_type: Rc::clone(&tp),
                ob_self: None,
                rust_fun: PyRustFun::$flag(Box::new($fun))
            };
            let obj = Rc::new(PyObject::RustFunObj(Rc::new(obj)));
            $env.update($id.to_string(), obj);
        })
    }
}

pub fn load_builtins(env: Rc<Env>) {
    set_builtin_fun!(env, "len", MethO, builtin_len);
}
