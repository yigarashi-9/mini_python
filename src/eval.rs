use std::rc::Rc;

use syntax::*;
use env::*;

trait Evaluable {
    fn eval(&self, env: Rc<Env>) -> Value;
}

impl Evaluable for Expr {
    fn eval(&self, env: Rc<Env>) -> Value {
        match self {
            &Expr::VarExpr(ref id) => get(env, id),
            &Expr::IntExpr(i) => Value::IntVal(i),
            &Expr::BoolExpr(b) => Value::BoolVal(b),
            &Expr::NoneExpr => Value::NoneVal,
            &Expr::AddExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                match v1 {
                    Value::IntVal(i1) => match v2 {
                        Value::IntVal(i2) => Value::IntVal(i1 + i2),
                        _ => panic!("Type error"),
                    },
                    _ => panic!("Type error"),
                }
            },
            &Expr::LtExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                match v1 {
                    Value::IntVal(i1) => match v2 {
                        Value::IntVal(i2) => Value::BoolVal(i1 < i2),
                        _ => panic!("Type error"),
                    },
                    _ => panic!("Type error"),
                }
            },
            &Expr::EqEqExpr(ref e1, ref e2) => {
                let v1 = e1.eval(Rc::clone(&env));
                let v2 = e2.eval(Rc::clone(&env));
                match v1 {
                    Value::IntVal(i1) => match v2 {
                        Value::IntVal(i2) => Value::BoolVal(i1 == i2),
                        _ => panic!("Type error"),
                    },
                    _ => panic!("Type error"),
                }
            },
            &Expr::CallExpr(ref fun, ref args) => {
                let funv = fun.eval(Rc::clone(&env));
                let vals = args.into_iter().map(|x| x.eval(Rc::clone(&env))).collect();
                match funv {
                    Value::FunVal(env, keys, prog) => {
                        match prog.exec(Rc::new(new_child(Rc::clone(&env), keys, vals))) {
                            CtrlOp::Nop => Value::NoneVal,
                            CtrlOp::Return(val) => val,
                            _ => panic!("Invalid control operator"),
                        }
                    },
                    _ => panic!("Not callable"),
                }
            }
        }
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
    Return(Value),
    Break,
    Continue,
}

pub trait Executable {
    fn exec(&self, env: Rc<Env>) -> CtrlOp;
}

impl Executable for SimpleStmt {
    fn exec(&self, env: Rc<Env>) -> CtrlOp {
        match self {
            &SimpleStmt::AssignStmt(ref id, ref expr) => {
                let v = expr.eval(Rc::clone(&env));
                update(env, id.clone(), v);
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
                update(Rc::clone(&env), id.clone(),
                       Value::FunVal(Rc::clone(&env), parms.clone(), prog.clone()));
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
