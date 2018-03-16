use std::collections::HashMap;
use syntax::*;

pub type Env = HashMap<Id, Value>;

trait Evaluable {
    fn eval(self, env: &Env) -> Value;
}

impl Evaluable for Expr {
    fn eval(self, env: &Env) -> Value {
        match self {
            Expr::VarExpr(ref id) => match env.get(id).unwrap() {
                &Value::IntVal(i) => Value::IntVal(i.clone()),
                &Value::BoolVal(b) => Value::BoolVal(b.clone()),
            },
            Expr::IntExpr(i) => Value::IntVal(i),
            Expr::BoolExpr(b) => Value::BoolVal(b),
            Expr::AddExpr(e1, e2) => {
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
            Expr::LtExpr(e1, e2) => {
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

pub trait Executable {
    fn exec(self, env: &mut Env) -> ();
}

impl Executable for SimpleStmt {
    fn exec(self, env: &mut Env) -> () {
        match self {
            SimpleStmt::AssignStmt(id, expr) => {
                let v = expr.eval(env);
                env.insert(id, v);
            },
        }
    }
}

impl Executable for CompoundStmt {
    fn exec(self, env: &mut Env) -> () {
        match self {
            CompoundStmt::IfStmt(expr, prog_then, prog_else) => {
                let cond = expr.eval(env);
                match cond {
                    Value::BoolVal(b) => (if b { prog_then } else { prog_else }).exec(env),
                    _ => panic!("Type error"),
                }
            },
        }
    }
}

impl Executable for Stmt {
    fn exec(self, env: &mut Env) -> () {
        match self {
            Stmt::StmtSimple(simple_stmt) => simple_stmt.exec(env),
            Stmt::StmtCompound(compound_stmt) => compound_stmt.exec(env)
        }
    }
}

impl Executable for Program {
    fn exec(self, env: &mut Env) -> () {
        for stmt in self {
            stmt.exec(env)
        }
    }
}
