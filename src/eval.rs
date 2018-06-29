use std::collections::HashMap;
use std::rc::Rc;

use syntax::*;
use env::*;
use object::*;

impl Expr {
    fn eval(&self, env: Rc<Env>) -> Rc<PyObject> {
        match self {
            &Expr::VarExpr(ref id) => env.get(id),
            &Expr::IntExpr(i) => Rc::new(PyObject::from_i32(i)),
            &Expr::BoolExpr(b) => Rc::new(PyObject::from_bool(b)),
            &Expr::StrExpr(ref s) => Rc::new(PyObject::from_string(s.clone())),
            &Expr::NoneExpr => Rc::new(PyObject::none_obj()),
            &Expr::AddExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                (v1.ob_type().tp_fun_add.unwrap())(&*v1, &*v2)
            },
            &Expr::LtExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                (v1.ob_type().tp_fun_lt.unwrap())(&*v1, &*v2)
            },
            &Expr::EqEqExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                (v1.ob_type().tp_fun_eq.unwrap())(&*v1, &*v2)
            },
            &Expr::CallExpr(ref fun, ref args) => {
                let funv = fun.eval(Rc::clone(&env));
                let mut vals = args.into_iter().map(|x| x.eval(Rc::clone(&env))).collect();
                call_func(funv, &mut vals)
            },
            &Expr::AttrExpr(ref e, ref ident) => {
                let v = e.eval(Rc::clone(&env));
                get_attr(&v, ident).unwrap()
            },
            &Expr::SubscrExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                v1.lookup(&v2).unwrap()
            },
            &Expr::DictExpr(ref pl) => {
                let mut hmap = HashMap::new();
                for (e1, e2) in pl {
                    let v1 = e1.eval(Rc::clone(&env));
                    let v2 = e2.eval(Rc::clone(&env));
                    hmap.insert(v1, v2);
                }
                Rc::new(PyObject::from_hashmap(hmap))
            }
        }
    }
}

fn call_func(funv: Rc<PyObject>, args: &mut Vec<Rc<PyObject>>) -> Rc<PyObject> {
    match *funv {
        PyObject::FunObj(ref fun) => {
            match fun.code.exec(Rc::new(Env::new_child(&fun.env, &fun.parms, args))) {
                CtrlOp::Nop => Rc::new(PyObject::none_obj()),
                CtrlOp::Return(val) => val,
                _ => panic!("Invalid control operator"),
            }
        },
        PyObject::MethodObj(ref method) => {
            let mut vals = vec![Rc::clone(&method.ob_self)];
            vals.append(args);
            match method.code.exec(Rc::new(Env::new_child(&method.env, &method.parms, &vals))) {
                CtrlOp::Nop => Rc::new(PyObject::none_obj()),
                CtrlOp::Return(val) => val,
                _ => panic!("Invalid control operator"),
            }
        },
        PyObject::TypeObj(ref cls) => {
            let dictval = Rc::new(PyDictObject::from_hashmap(HashMap::new()));
            let instance = Rc::new(PyObject::InstObj(Rc::new(
                PyInstObject {
                    ob_type: Rc::clone(cls),
                    class: Rc::clone(cls),
                    dict: dictval,
                })));
            match get_attr(&instance, &"__init__".to_string()) {
                Some(init_fun) => call_func(Rc::clone(&init_fun), args),
                None => Rc::new(PyObject::none_obj())
            };
            instance
        },
        _ => panic!("Type Error: Callable expected"),
    }
}

fn make_method(value: Rc<PyObject>, instance_ref: &Rc<PyObject>) -> Rc<PyObject> {
    match *value {
        PyObject::FunObj(ref fun) => Rc::new(PyObject::MethodObj(Rc::new(
            PyMethodObject {
                ob_type: Rc::new(PyTypeObject::new_method()),
                ob_self: Rc::clone(instance_ref),
                env: Rc::clone(&fun.env),
                parms: fun.parms.clone(),
                code: fun.code.clone(),
            }))),
        _ => Rc::clone(&value),
    }
}

fn get_attr(value: &Rc<PyObject>, key: &Id) -> Option<Rc<PyObject>> {
    let keyval = Rc::new(PyObject::from_string(key.clone()));
    match **value {
        PyObject::TypeObj(ref typ) => match typ.tp_dict_ref() {
            &Some(ref dict) => dict.lookup(&keyval),
            &None => panic!("Type Error: get_attr"),
        },
        PyObject::InstObj(ref inst) => {
            match inst.dict.lookup(&keyval) {
                Some(ret_val) => Some(ret_val),
                None => match inst.class.tp_dict_ref() {
                    &Some(ref dict) =>  match dict.lookup(&keyval) {
                        Some(ret_val) => Some(make_method(Rc::clone(&ret_val), &value)),
                        None => None
                    },
                    &None => panic!("Type Error: get_attr")
                }
            }
        },
        _ => panic!("Type Error: get_attr")
    }
}

fn update_attr(value: &Rc<PyObject>, key: Id, rvalue: Rc<PyObject>) {
    let keyval = Rc::new(PyObject::from_string(key));
    let value = Rc::clone(value);
    match *value {
        PyObject::TypeObj(ref typ) => {
            match typ.tp_dict_ref() {
                &Some(ref dict) => dict.update(keyval, rvalue),
                &None => panic!("Type Error: update_attr")
            }
        },
        PyObject::InstObj(ref inst) => {
            inst.dict.update(keyval, rvalue);
        },
        _ => panic!("Type Error: update_attr")
    }
}

pub enum CtrlOp {
    Nop,
    Return(Rc<PyObject>),
    Break,
    Continue,
}

pub trait Executable {
    fn exec(&self, env: Rc<Env>) -> CtrlOp;
}

impl Executable for SimpleStmt {
    fn exec(&self, env: Rc<Env>) -> CtrlOp {
        match self {
            &SimpleStmt::ExprStmt(ref expr) => {
                expr.eval(env);
                CtrlOp::Nop
            },
            &SimpleStmt::AssignStmt(ref target, ref rexpr) => {
                match target {
                    &Target::IdentTarget(ref id) => {
                        let v = rexpr.eval(Rc::clone(&env));
                        env.update(id.clone(), v);
                    },
                    &Target::AttrTarget(ref lexpr, ref id) => {
                        let rv = rexpr.eval(Rc::clone(&env));
                        let lv = lexpr.eval(Rc::clone(&env));
                        update_attr(&lv, id.clone(), rv);
                    },
                    &Target::SubscrTarget(ref e1, ref e2) => {
                        let rv = rexpr.eval(Rc::clone(&env));
                        let v1 = e1.eval(Rc::clone(&env));
                        let v2 = e2.eval(Rc::clone(&env));
                        v1.update(v2, rv);
                    },
                };
                CtrlOp::Nop
            },
            &SimpleStmt::ReturnStmt(ref expr) => {
                CtrlOp::Return(expr.eval(Rc::clone(&env)))
            },
            &SimpleStmt::BreakStmt => CtrlOp::Break,
            &SimpleStmt::ContinueStmt => CtrlOp::Continue,
            &SimpleStmt::AssertStmt(ref expr) => {
                if (&expr.eval(Rc::clone(&env))).to_bool() {
                    CtrlOp::Nop
                } else {
                    panic!("AssertionError")
                }
            }
        }
    }
}

impl Executable for CompoundStmt {
    fn exec(&self, env: Rc<Env>) -> CtrlOp {
        match self {
            &CompoundStmt::IfStmt(ref expr, ref prog_then, ref prog_else) => {
                if (&expr.eval(Rc::clone(&env))).to_bool() {
                    prog_then.exec(Rc::clone(&env))
                } else {
                    prog_else.exec(Rc::clone(&env))
                }
            },
            &CompoundStmt::WhileStmt(ref expr, ref prog) => {
                while (&expr.eval(Rc::clone(&env))).to_bool() {
                    match prog.exec(Rc::clone(&env)) {
                        CtrlOp::Return(e) => return CtrlOp::Return(e),
                        CtrlOp::Break => break,
                        _ => continue,
                    }
                };
                CtrlOp::Nop
            }
            &CompoundStmt::DefStmt(ref id, ref parms, ref prog) => {
                let funv = PyObject::FunObj(Rc::new(
                    PyFuncObject {
                        ob_type: Rc::new(PyTypeObject::new_function()),
                        env: Rc::clone(&env),
                        parms: parms.clone(),
                        code: prog.clone(),
                    }));
                Rc::clone(&env).update(id.clone(), Rc::new(funv));
                CtrlOp::Nop
            },
            &CompoundStmt::ClassStmt(ref id, ref prog) => {
                let new_env = Rc::new(Env::new_child(&env, &vec![], &vec![]));
                match prog.exec(Rc::clone(&new_env)) {
                    CtrlOp::Nop => (),
                    _ => panic!("Runtime Error: Invalid control operator")
                }
                let dictobj = PyDictObject::from_hashmap(new_env.dict());
                let mut cls = PyTypeObject::new_type();
                cls.ob_type = Some(Rc::new(PyTypeObject::new_type()));
                cls.tp_name = id.clone();
                cls.tp_dict = Some(Rc::new(dictobj));
                env.update(id.clone(), Rc::new(PyObject::TypeObj(Rc::new(cls))));
                CtrlOp::Nop
            }
        }
    }
}

impl Executable for Stmt {
    fn exec(&self, env: Rc<Env>) -> CtrlOp {
        match self {
            &Stmt::StmtSimple(ref simple_stmt) => simple_stmt.exec(Rc::clone(&env)),
            &Stmt::StmtCompound(ref compound_stmt) => compound_stmt.exec(Rc::clone(&env))
        }
    }
}

impl Executable for Program {
    fn exec(&self, env: Rc<Env>) -> CtrlOp {
        for stmt in self {
            match stmt.exec(Rc::clone(&env)) {
                CtrlOp::Nop => continue,
                cop => return cop
            }
        };
        CtrlOp::Nop
    }
}
