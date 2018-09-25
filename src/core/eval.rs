use std::cell::RefCell;
use std::rc::Rc;

use syntax::*;
use env::*;

use object::*;
use object::funobj::*;
use object::generic::*;
use object::instobj::*;
use object::listobj::*;
use object::methodobj::*;
use object::rustfunobj::*;
use object::typeobj::*;


impl Expr {
    fn eval(&self, env: Rc<Env>) -> Rc<PyObject> {
        match self {
            &Expr::VarExpr(ref id) => env.get(id),
            &Expr::IntExpr(i) => PyObject::from_i32(i),
            &Expr::BoolExpr(b) => PyObject::from_bool(b),
            &Expr::StrExpr(ref s) => PyObject::from_string(s.clone()),
            &Expr::NoneExpr => PyObject::none_obj(),
            &Expr::AddExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                let typ = Rc::clone(v1.ob_type_ref());
                let typ_borrowed = typ.borrow();
                (typ_borrowed.tp_fun_add.as_ref().unwrap())(v1, v2)
            },
            &Expr::LtExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                let typ = Rc::clone(v1.ob_type_ref());
                let typ_borrowed = typ.borrow();
                (typ_borrowed.tp_fun_lt.as_ref().unwrap())(v1, v2)
            },
            &Expr::EqEqExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                let typ = Rc::clone(v1.ob_type_ref());
                let typ_borrowed = typ.borrow();
                (typ_borrowed.tp_fun_eq.as_ref().unwrap())(v1, v2)
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
                match v1.inner {
                    PyInnerObject::ListObj(ref _obj) => {
                        v1.getitem_index(&v2).unwrap()
                    },
                    PyInnerObject::DictObj(ref _obj) => {
                        v1.lookup(&v2).unwrap()
                    },
                    _ => panic!("Type Error: eval SubscrExpr"),
                }
            },
            &Expr::ListExpr(ref cl) => {
                let v: Vec<Rc<PyObject>> = cl.iter().map(|e|{ e.eval(Rc::clone(&env)) }).collect();
                PyObject::from_vec(v)
            },
            &Expr::DictExpr(ref pl) => {
                let mut dictobj = PyObject::new_dict();
                for (e1, e2) in pl {
                    let v1 = e1.eval(Rc::clone(&env));
                    let v2 = e2.eval(Rc::clone(&env));
                    dictobj.update(v1, v2);
                }
                dictobj
            }
        }
    }
}

fn call_func(funv: Rc<PyObject>, args: &mut Vec<Rc<PyObject>>) -> Rc<PyObject> {
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
        PyInnerObject::TypeObj(ref cls) => {
            let dictobj = PyObject::new_dict();
            let instance = Rc::new(PyObject {
                ob_type: Rc::clone(cls),
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

fn make_method(value: Rc<PyObject>, instance_ref: &Rc<PyObject>) -> Rc<PyObject> {
    match value.inner {
        PyInnerObject::FunObj(ref fun) => {
            PY_METHOD_TYPE.with(|tp| {
                Rc::new(PyObject {
                    ob_type: Rc::clone(tp),
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

fn get_attr(value: &Rc<PyObject>, key: &Id) -> Option<Rc<PyObject>> {
    let keyval = Rc::new(PyObject::from_string(key.clone()));
    match value.inner {
        PyInnerObject::TypeObj(ref typ) => typ.borrow().tp_dict_ref().as_ref().unwrap().lookup(&keyval),
        PyInnerObject::InstObj(ref inst) => {
            match inst.dict.lookup(&keyval) {
                Some(ret_val) => Some(ret_val),
                None => {
                    let mro = get_attr(&inst.class, &"__mro__".to_string()).unwrap();
                    match mro.inner {
                        PyInnerObject::ListObj(ref obj) => {
                            for base in obj.list.borrow().iter() {
                                match get_attr(base, key) {
                                    Some(ret_val) => return Some(make_method(Rc::clone(&ret_val), &value)),
                                    None => continue,
                                }
                            };
                            None
                        },
                        _ => panic!("Internal Error: get_attr mro"),
                    }
                }
            }
        },
        _ => panic!("Type Error: get_attr")
    }
}

fn update_attr(value: &Rc<PyObject>, key: Id, rvalue: Rc<PyObject>) {
    let keyval = PyObject::from_string(key);
    let value = Rc::clone(value);
    match value.inner {
        PyInnerObject::TypeObj(ref typ) => {
            match typ.borrow().tp_dict_ref() {
                &Some(ref dict) => dict.update(keyval, rvalue),
                &None => panic!("Type Error: update_attr")
            }
        },
        PyInnerObject::InstObj(ref inst) => {
            inst.dict.update(keyval, rvalue);
        },
        _ => panic!("Type Error: update_attr")
    }
}

pub fn unaryop_from_pyobj(obj: Rc<PyObject>) ->
    Rc<dyn Fn(Rc<PyObject>) -> Rc<PyObject>> {
        Rc::new(move |x| call_func(Rc::clone(&obj), &mut vec![x]))
    }

pub fn get_wrapped_unaryop(dict: Rc<PyObject>, s: &str) ->
    Option<Rc<dyn Fn(Rc<PyObject>) -> Rc<PyObject>>> {
        dict.lookup(&PyObject::from_str(s)).map(unaryop_from_pyobj)
    }

pub fn binop_from_pyobj(obj: Rc<PyObject>) ->
    Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>> {
        Rc::new(move |x, y| call_func(Rc::clone(&obj), &mut vec![x, y]))
    }

pub fn get_wrapped_binop(dict: Rc<PyObject>, s: &str) ->
    Option<Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>>> {
        dict.lookup(&PyObject::from_str(s)).map(binop_from_pyobj)
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
                if pyobj_is_bool(expr.eval(Rc::clone(&env))) {
                    CtrlOp::Nop
                } else {
                    panic!("AssertionError")
                }
            }
        }
    }
}

fn pick_winner(mro_list: &Vec<Vec<Rc<PyObject>>>) -> Rc<PyObject> {
    for mro in mro_list {
        let cand = &mro[0];

        let mut win = true;
        for others in mro_list {
            let (_, tail) = others.split_at(1);
            if tail.contains(cand) {
                win = false;
                break;
            }
        }

        if win { return Rc::clone(cand) };
    }
    panic!("pick_candidate: No candidate")
}

fn remove_winner(winner: Rc<PyObject>, mro_list: Vec<Vec<Rc<PyObject>>>) -> Vec<Vec<Rc<PyObject>>> {
    let mut new_list = vec![];
    for mro in mro_list {
        let mut new_mro = vec![];
        for class in mro {
            if &*winner as *const _ != &*class as *const _ { new_mro.push(Rc::clone(&class)); }
        }
        if new_mro.len() > 0 { new_list.push(new_mro); }
    };
    new_list
}

fn linearlize(arg: Vec<Vec<Rc<PyObject>>>) -> Vec<Rc<PyObject>> {
    let mut mro_list = arg;
    let mut mro = vec![];
    loop {
        if mro_list.len() == 0 {
            break;
        }
        let winner = pick_winner(&mro_list);
        mro.push(Rc::clone(&winner));
        mro_list = remove_winner(winner, mro_list);
    };
    mro
}


impl Executable for CompoundStmt {
    fn exec(&self, env: Rc<Env>) -> CtrlOp {
        match self {
            &CompoundStmt::IfStmt(ref expr, ref prog_then, ref prog_else) => {
                if pyobj_is_bool(expr.eval(Rc::clone(&env))) {
                    prog_then.exec(Rc::clone(&env))
                } else {
                    prog_else.exec(Rc::clone(&env))
                }
            },
            &CompoundStmt::WhileStmt(ref expr, ref prog) => {
                while pyobj_is_bool(expr.eval(Rc::clone(&env))) {
                    match prog.exec(Rc::clone(&env)) {
                        CtrlOp::Return(e) => return CtrlOp::Return(e),
                        CtrlOp::Break => break,
                        _ => continue,
                    }
                };
                CtrlOp::Nop
            }
            &CompoundStmt::DefStmt(ref id, ref parms, ref prog) => {
                let funv = PY_FUN_TYPE.with(|tp| {
                    PyObject {
                        ob_type: Rc::clone(&tp),
                        inner: PyInnerObject::FunObj(Rc::new(PyFunObject {
                            env: Rc::clone(&env),
                            parms: parms.clone(),
                            code: prog.clone(),
                        }))
                    }
                });
                Rc::clone(&env).update(id.clone(), Rc::new(funv));
                CtrlOp::Nop
            },
            &CompoundStmt::ClassStmt(ref id, ref bases, ref prog) => {
                let new_env = Rc::new(Env::new_child(&env, &vec![], &vec![]));
                match prog.exec(Rc::clone(&new_env)) {
                    CtrlOp::Nop => (),
                    _ => panic!("Runtime Error: Invalid control operator")
                }
                let dictobj = new_env.dictobj();
                let mut cls = PyTypeObject::new_type();
                cls.tp_dict = Some(Rc::clone(&dictobj));
                cls.ob_type = PY_TYPE_TYPE.with(|tp|{ Some(Rc::clone(&tp)) });
                cls.tp_name = id.clone();
                cls.tp_bool = get_wrapped_unaryop(Rc::clone(&dictobj), "__bool__");
                cls.tp_fun_add = get_wrapped_binop(Rc::clone(&dictobj), "__add__");
                cls.tp_fun_eq = get_wrapped_binop(Rc::clone(&dictobj), "__eq__");
                cls.tp_fun_lt = get_wrapped_binop(Rc::clone(&dictobj), "__lt__");
                cls.tp_len = get_wrapped_unaryop(Rc::clone(&dictobj), "__len__");

                let bases: Vec<Rc<PyObject>> = bases.iter().map(|e| { e.eval(Rc::clone(&env)) }).collect();
                let mut mro_list = vec![];
                for base in &bases {
                    let pylist = get_attr(&base, &"__mro__".to_string()).unwrap();
                    match pylist.inner {
                        PyInnerObject::ListObj(ref obj) => {
                            mro_list.push(obj.list.borrow().clone());
                        },
                        _ => panic!("Type Error: mro")
                    }
                }

                let clsobj = Rc::new(PY_TYPE_TYPE.with(|tp| {
                    PyObject {
                        ob_type: Rc::clone(&tp),
                        inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(cls)))
                    }
                }));

                let mut mro = linearlize(mro_list);
                mro.insert(0, Rc::clone(&clsobj));
                update_attr(&clsobj, "__mro__".to_string(), PyObject::from_vec(mro));

                for base in &bases {
                    let tp_subclasses = &base.ob_type.borrow().tp_subclasses;
                    if tp_subclasses.is_some() {
                        pylist_append(Rc::clone(tp_subclasses.as_ref().unwrap()), Rc::clone(&clsobj));
                    }
                }

                env.update(id.clone(), clsobj);
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
