use std::collections::HashMap;
use syntax::*;

pub type Env = HashMap<Id, Value>;

trait Evaluable {
    fn eval(&self, env: &Env) -> Value;
}

impl Evaluable for Expr {
    fn eval(&self, env: &Env) -> Value {
        match self {
            &Expr::VarExpr(ref id) => match env.get(id).unwrap() {
                &Value::IntVal(i) => Value::IntVal(i.clone()),
                &Value::BoolVal(b) => Value::BoolVal(b.clone()),
            },
            &Expr::IntExpr(i) => Value::IntVal(i),
            &Expr::BoolExpr(b) => Value::BoolVal(b),
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
        }
    }
}

pub enum CtrlOp {
    Break,
    Continue,
}

fn normal_result() -> Result<(), CtrlOp> { Ok(()) }

fn is_true_value(res: &Value) -> bool {
    match res {
        &Value::IntVal(i) => i != 0,
        &Value::BoolVal(b) => b,
    }
}

pub trait Executable {
    fn exec(&self, env: &mut Env) -> Result<(), CtrlOp>;
}

impl Executable for SimpleStmt {
    fn exec(&self, env: &mut Env) -> Result<(), CtrlOp> {
        match self {
            &SimpleStmt::AssignStmt(ref id, ref expr) => {
                let v = expr.eval(env);
                env.insert(id.clone(), v);
                normal_result()
            },
            &SimpleStmt::BreakStmt => return Err(CtrlOp::Break),
            &SimpleStmt::ContinueStmt => return Err(CtrlOp::Continue),
        }
    }
}

impl Executable for CompoundStmt {
    fn exec(&self, env: &mut Env) -> Result<(), CtrlOp> {
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
                        Err(CtrlOp::Break) => break,
                        _ => continue,
                    }
                };
                normal_result()
            }
        }
    }
}

impl Executable for Stmt {
    fn exec(&self, env: &mut Env) -> Result<(), CtrlOp> {
        match self {
            &Stmt::StmtSimple(ref simple_stmt) => simple_stmt.exec(env),
            &Stmt::StmtCompound(ref compound_stmt) => compound_stmt.exec(env)
        }
    }
}

impl Executable for Program {
    fn exec(&self, env: &mut Env) -> Result<(), CtrlOp> {
        for stmt in self {
            try!(stmt.exec(env))
        };
        normal_result()
    }
}
