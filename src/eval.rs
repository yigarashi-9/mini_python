use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use syntax::*;
use env::*;

trait Evaluable {
    fn eval(&self, env: Rc<Env>) -> Rc<Value>;
}

impl Evaluable for Expr {
    fn eval(&self, env: Rc<Env>) -> Rc<Value> {
        match self {
            &Expr::VarExpr(ref id) => env.get(id),
            &Expr::IntExpr(i) => Rc::new(Value::IntVal(i)),
            &Expr::BoolExpr(b) => Rc::new(Value::BoolVal(b)),
            &Expr::NoneExpr => Rc::new(Value::NoneVal),
            &Expr::AddExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                match *v1 {
                    Value::IntVal(i1) => match *v2 {
                        Value::IntVal(i2) => Rc::new(Value::IntVal(i1 + i2)),
                        _ => panic!("Type error"),
                    },
                    _ => panic!("Type error"),
                }
            },
            &Expr::LtExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                match *v1 {
                    Value::IntVal(i1) => match *v2 {
                        Value::IntVal(i2) => Rc::new(Value::BoolVal(i1 < i2)),
                        _ => panic!("Type error"),
                    },
                    _ => panic!("Type error"),
                }
            },
            &Expr::EqEqExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                match *v1 {
                    Value::IntVal(i1) => match *v2 {
                        Value::IntVal(i2) => Rc::new(Value::BoolVal(i1 == i2)),
                        _ => panic!("Type error"),
                    },
                    _ => panic!("Type error"),
                }
            },
            &Expr::CallExpr(ref fun, ref args) => {
                let funv = fun.eval(Rc::clone(&env));
                let mut vals = args.into_iter().map(|x| x.eval(Rc::clone(&env))).collect();
                call_func(funv, &mut vals)
            },
            &Expr::AttrExpr(ref e, ref ident) => {
                let v = e.eval(Rc::clone(&env));
                get_attr(v, ident)
            }
        }
    }
}

fn call_func(funv: Rc<Value>, args: &mut Vec<Rc<Value>>) -> Rc<Value> {
    match *funv {
        Value::FunVal(ref funenv, ref keys, ref prog) => {
            match prog.exec(Rc::new(Env::new_child(Rc::clone(funenv), keys, args))) {
                CtrlOp::Nop => Rc::new(Value::NoneVal),
                CtrlOp::Return(val) => val,
                _ => panic!("Invalid control operator"),
            }
        },
        Value::MethodVal(ref self_val, ref funenv, ref keys, ref prog) => {
            let mut vals = vec![Rc::clone(self_val)];
            vals.append(args);
            match prog.exec(Rc::new(Env::new_child(Rc::clone(funenv), keys, &vals))) {
                CtrlOp::Nop => Rc::new(Value::NoneVal),
                CtrlOp::Return(val) => val,
                _ => panic!("Invalid control operator"),
            }
        },
        Value::ClassVal(ref map) => {
            let instance = create_instance(Rc::clone(&funv), map);
            match *instance {
                Value::InstanceVal(_, ref map) => {
                    let init_fun_opt = get_val(map, &"__init__".to_string());
                    match init_fun_opt {
                        Some(init_fun) => call_func(Rc::clone(&init_fun), args),
                        None => Rc::new(Value::NoneVal)
                    };
                },
                _ => panic!("Never happenes")
            }
            instance

        },
        _ => panic!("Type Error: Callable expected"),
    }
}

fn create_instance(class_val: Rc<Value>, map: &RefCell<HashMap<Id, Rc<Value>>>) -> Rc<Value> {
    let instance = Rc::new(Value::InstanceVal(class_val, RefCell::new(HashMap::new())));
    match *instance {
        Value::InstanceVal(_, ref ret_map) => {
            for (key, val) in map.borrow().iter() {
                ret_map.borrow_mut().insert(
                    key.clone(), instantiate_value(Rc::clone(val), &instance));
            }
        },
        _ => panic!("Never happenes")
    };
    instance
}

fn instantiate_value(value: Rc<Value>, instance_ref: &Rc<Value>) -> Rc<Value> {
    match *value {
        Value::FunVal(ref funenv, ref params, ref prog) => {
            let method = Value::MethodVal(Rc::clone(instance_ref),
                                          Rc::clone(funenv),
                                          params.clone(),
                                          prog.clone());
            Rc::new(method)
        },
        _ => Rc::clone(&value)
    }
}

fn get_val(map: &RefCell<HashMap<Id, Rc<Value>>>, id: &Id) -> Option<Rc<Value>> {
    match map.borrow().get(id) {
        Some(v) => Some(Rc::clone(v)),
        None => None
    }
}

fn get_attr(value: Rc<Value>, id: &Id) -> Rc<Value> {
    match *value {
        Value::ClassVal(ref map) => {
            Rc::clone(map.borrow().get(id).unwrap())
        },
        Value::InstanceVal(ref class, ref map) => {
            let val = get_val(map, id);
            match val {
                Some(ret_val) => Rc::clone(&ret_val),
                None => {
                    let method = instantiate_value(
                        Rc::clone(&get_attr(Rc::clone(class), id)),
                        &value);
                    map.borrow_mut().insert(id.clone(), Rc::clone(&method));
                    method
                }
            }
        },
        _ => panic!("Type Error: get_attr")
    }
}

fn update_attr(value: Rc<Value>, id: &Id, rvalue: Rc<Value>) {
    match *value {
        Value::ClassVal(ref map) => {
            map.borrow_mut().insert(id.clone(), rvalue);
        },
        Value::InstanceVal(ref class, ref map) => {
            map.borrow_mut().insert(id.clone(), rvalue);
        },
        _ => panic!("Type Error: update_attr")
    }
}

fn is_true_value(res: &Value) -> bool {
    match res {
        &Value::IntVal(i) => i != 0,
        &Value::BoolVal(b) => b,
        &Value::NoneVal => false,
        _ => true,
    }
}

pub enum CtrlOp {
    Nop,
    Return(Rc<Value>),
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
                        let lv = lexpr.eval(Rc::clone(&env));
                        let rv = rexpr.eval(Rc::clone(&env));
                        update_attr(lv, id, rv);
                    },
                    _ => {
                        panic!("Type Error: AssignStmt");
                    }
                };
                CtrlOp::Nop
            },
            &SimpleStmt::ReturnStmt(ref expr) => {
                CtrlOp::Return(expr.eval(Rc::clone(&env)))
            },
            &SimpleStmt::BreakStmt => CtrlOp::Break,
            &SimpleStmt::ContinueStmt => CtrlOp::Continue,
            &SimpleStmt::AssertStmt(ref expr) => {
                if is_true_value(&expr.eval(Rc::clone(&env))) {
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
                if is_true_value(&expr.eval(Rc::clone(&env))) {
                    prog_then.exec(Rc::clone(&env))
                } else {
                    prog_else.exec(Rc::clone(&env))
                }
            },
            &CompoundStmt::WhileStmt(ref expr, ref prog) => {
                while is_true_value(&expr.eval(Rc::clone(&env))) {
                    match prog.exec(Rc::clone(&env)) {
                        CtrlOp::Return(e) => return CtrlOp::Return(e),
                        CtrlOp::Break => break,
                        _ => continue,
                    }
                };
                CtrlOp::Nop
            }
            &CompoundStmt::DefStmt(ref id, ref parms, ref prog) => {
                Rc::clone(&env).update(
                    id.clone(),
                    Rc::new(Value::FunVal(Rc::clone(&env), parms.clone(), prog.clone())));
                CtrlOp::Nop
            },
            &CompoundStmt::ClassStmt(ref id, ref prog) => {
                let new_env = Rc::new(Env::new_child(Rc::clone(&env), &vec![], &vec![]));
                match prog.exec(Rc::clone(&new_env)) {
                    CtrlOp::Nop => (),
                    _ => panic!("Runtime Error: Invalid control operator")
                }
                env.update(id.clone(),
                           Rc::new(Value::ClassVal(RefCell::new(new_env.raw_map()))));
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
