use std::rc::Rc;

use syntax::*;
use env::*;

use object::*;
use object::generic::*;
use object::listobj::*;
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
                let ob_type = v1.ob_type();
                let typ = ob_type.pytype_typeobj_borrow();
                (typ.tp_fun_add.as_ref().expect("Add"))(v1, v2)
            },
            &Expr::LtExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                let ob_type = v1.ob_type();
                let typ = ob_type.pytype_typeobj_borrow();
                (typ.tp_fun_lt.as_ref().unwrap())(v1, v2)
            },
            &Expr::EqEqExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                let ob_type = v1.ob_type();
                let typ = ob_type.pytype_typeobj_borrow();
                (typ.tp_fun_eq.as_ref().unwrap())(v1, v2)
            },
            &Expr::CallExpr(ref fun, ref args) => {
                let funv = fun.eval(Rc::clone(&env));
                let mut vals = args.into_iter().map(|x| x.eval(Rc::clone(&env))).collect();
                call_func(funv, &mut vals)
            },
            &Expr::AttrExpr(ref e, ref ident) => {
                let v = e.eval(Rc::clone(&env));
                pyobj_get_attro(v, PyObject::from_string(ident.clone())).unwrap()
            },
            &Expr::SubscrExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                if v1.pylist_check() {
                    v1.pylist_getitem(pyobj_to_i32(v2) as usize)
                } else if v1.pydict_check() {
                    v1.pydict_lookup(v2).unwrap()
                } else {
                    panic!("Type Error: eval SubscrExpr")
                }
            },
            &Expr::ListExpr(ref cl) => {
                let v: Vec<Rc<PyObject>> = cl.iter().map(|e|{ e.eval(Rc::clone(&env)) }).collect();
                PyObject::pylist_from_vec(&v)
            },
            &Expr::DictExpr(ref pl) => {
                let mut dictobj = PyObject::pydict_new();
                for (e1, e2) in pl {
                    let v1 = e1.eval(Rc::clone(&env));
                    let v2 = e2.eval(Rc::clone(&env));
                    dictobj.pydict_update(v1, v2);
                }
                dictobj
            }
        }
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
                        v1.pydict_update(v2, rv);
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
                let funv = PyObject::pyfun_new(&env, parms, prog);
                Rc::clone(&env).update(id.clone(), funv);
                CtrlOp::Nop
            },
            &CompoundStmt::ClassStmt(ref id, ref bases, ref prog) => {
                let new_env = Rc::new(Env::new_child(&env, &vec![], &vec![]));
                match prog.exec(Rc::clone(&new_env)) {
                    CtrlOp::Nop => (),
                    _ => panic!("Runtime Error: Invalid control operator")
                }
                let dictobj = new_env.dictobj();
                let bases: Vec<Rc<PyObject>> = bases.iter().map(|e| { e.eval(Rc::clone(&env)) }).collect();

                let cls = PyObject::pytype_new();
                {
                    let mut typ = cls.pytype_typeobj_borrow_mut();
                    typ.tp_dict = Some(Rc::clone(&dictobj));
                    typ.tp_name = id.clone();
                    typ.tp_bases = Some(PyObject::pylist_from_vec(&bases));
                }

                for base in &bases {
                    let mut typ = base.pytype_typeobj_borrow_mut();
                    if typ.tp_subclasses.is_none() {
                        typ.tp_subclasses = Some(PyObject::pylist_from_vec(&vec![]));
                    }
                    pylist_append(Rc::clone(typ.tp_subclasses.as_ref().unwrap()), Rc::clone(&cls));
                }

                pytype_ready(Rc::clone(&cls));
                env.update(id.clone(), cls);
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
