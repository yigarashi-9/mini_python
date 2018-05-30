use syntax::*;
use env::*;

trait Evaluable {
    fn eval(&self, env: &Env) -> Value;
}

impl Evaluable for Expr {
    fn eval(&self, env: &Env) -> Value {
        match self {
            &Expr::VarExpr(ref id) => match env.get(id) {
                &Value::IntVal(i) => Value::IntVal(i),
                &Value::BoolVal(b) => Value::BoolVal(b),
                &Value::NoneVal => Value::NoneVal,
                &Value::FunVal(ref params, ref prog) =>
                    Value::FunVal(params.clone(), prog.clone()),
            },
            &Expr::IntExpr(i) => Value::IntVal(i),
            &Expr::BoolExpr(b) => Value::BoolVal(b),
            &Expr::NoneExpr => Value::NoneVal,
            &Expr::AddExpr(ref e1, ref e2) => {
                let v1 = e1.eval(env);
                let v2 = e2.eval(env);
                match v1 {
                    Value::IntVal(i1) => match v2 {
                        Value::IntVal(i2) => Value::IntVal(i1 + i2),
                        _ => panic!("Type error"),
                    },
                    _ => panic!("Type error"),
                }
            },
            &Expr::LtExpr(ref e1, ref e2) => {
                let v1 = e1.eval(env);
                let v2 = e2.eval(env);
                match v1 {
                    Value::IntVal(i1) => match v2 {
                        Value::IntVal(i2) => Value::BoolVal(i1 < i2),
                        _ => panic!("Type error"),
                    },
                    _ => panic!("Type error"),
                }
            },
            &Expr::CallExpr(ref fun, ref args) => {
                let funv = fun.eval(env);
                let vals = args.into_iter().map(|x| x.eval(env)).collect();
                match funv {
                    Value::FunVal(keys, prog) => {
                        let mut child_env = Env::new_child(env, keys, vals);
                        match prog.exec(&mut child_env) {
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
    fn exec(&self, env: &mut Env) -> CtrlOp;
}

impl Executable for SimpleStmt {
    fn exec(&self, env: &mut Env) -> CtrlOp {
        match self {
            &SimpleStmt::AssignStmt(ref id, ref expr) => {
                let v = expr.eval(env);
                env.update(id.clone(), v);
                CtrlOp::Nop
            },
            &SimpleStmt::ReturnStmt(ref expr) => {
                CtrlOp::Return(expr.eval(env))
            },
            &SimpleStmt::BreakStmt => CtrlOp::Break,
            &SimpleStmt::ContinueStmt => CtrlOp::Continue,
        }
    }
}

impl Executable for CompoundStmt {
    fn exec(&self, env: &mut Env) -> CtrlOp {
        match self {
            &CompoundStmt::IfStmt(ref expr, ref prog_then, ref prog_else) => {
                if is_true_value(&expr.eval(env)) {
                    prog_then.exec(env)
                } else {
                    prog_else.exec(env)
                }
            },
            &CompoundStmt::WhileStmt(ref expr, ref prog) => {
                while is_true_value(&expr.eval(env)) {
                    match prog.exec(env) {
                        CtrlOp::Return(e) => return CtrlOp::Return(e),
                        CtrlOp::Break => break,
                        _ => continue,
                    }
                };
                CtrlOp::Nop
            }
            &CompoundStmt::DefStmt(ref id, ref parms, ref prog) => {
                env.update(id.clone(), Value::FunVal(parms.clone(), prog.clone()));
                CtrlOp::Nop
            }
        }
    }
}

impl Executable for Stmt {
    fn exec(&self, env: &mut Env) -> CtrlOp {
        match self {
            &Stmt::StmtSimple(ref simple_stmt) => simple_stmt.exec(env),
            &Stmt::StmtCompound(ref compound_stmt) => compound_stmt.exec(env)
        }
    }
}

impl Executable for Program {
    fn exec(&self, env: &mut Env) -> CtrlOp {
        for stmt in self {
            match stmt.exec(env) {
                CtrlOp::Nop => continue,
                cop => return cop
            }
        };
        CtrlOp::Nop
    }
}
