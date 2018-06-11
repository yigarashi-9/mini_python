use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::ptr;
use std::rc::Rc;

use env::Env;

pub type Id = String;

pub enum Value {
    IntVal(i32),
    BoolVal(bool),
    StrVal(String),
    NoneVal,
    FunVal(Rc<Env>, Vec<Id>, Program),
    ClassVal(RefCell<HashMap<Id, Rc<Value>>>),
    InstanceVal(Rc<Value>, RefCell<HashMap<Id, Rc<Value>>>),
    MethodVal(Rc<Value>, Rc<Env>, Vec<Id>, Program),
    DictVal(RefCell<HashMap<Rc<Value>, Rc<Value>>>)
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::IntVal(i1), &Value::IntVal(i2)) => i1 == i2,
            (&Value::BoolVal(b1), &Value::BoolVal(b2)) => b1 == b2,
            (&Value::StrVal(ref s1), &Value::StrVal(ref s2)) => s1 == s2,
            (&Value::NoneVal, &Value::NoneVal) => true,
            _ => ptr::eq(self as *const Value, other as *const Value)
        }
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H>(&self, hasher: &mut H) where H: Hasher {
        match self {
            &Value::IntVal(i) => i.hash(hasher),
            &Value::BoolVal(b) => b.hash(hasher),
            &Value::StrVal(ref s) => s.hash(hasher),
            &Value::NoneVal => hasher.write_u64(u64::max_value()),
            &Value::DictVal(_) => panic!("Unhashable: DictVal"),
            _ => (self as *const Value).hash(hasher)
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    VarExpr(Id),
    IntExpr(i32),
    BoolExpr(bool),
    StrExpr(String),
    NoneExpr,
    AddExpr(Box<Expr>, Box<Expr>),
    LtExpr(Box<Expr>, Box<Expr>),
    EqEqExpr(Box<Expr>, Box<Expr>),
    CallExpr(Box<Expr>, Vec<Expr>),
    AttrExpr(Box<Expr>, Id),
    SubscrExpr(Box<Expr>, Box<Expr>),
    DictExpr(Vec<(Expr, Expr)>)
}

impl Expr {
    pub fn to_string(&self) -> String {
        let str = match self {
            &Expr::VarExpr(_) => "VarExpr",
            &Expr::IntExpr(_) => "IntExpr",
            &Expr::BoolExpr(_) => "BoolExpr",
            &Expr::StrExpr(_) => "StrExpr",
            &Expr::NoneExpr => "NoneExpr",
            &Expr::AddExpr(_, _) => "AddExpr",
            &Expr::LtExpr(_, _) => "LtExpr",
            &Expr::EqEqExpr(_, _) => "EqEqExpr",
            &Expr::CallExpr(_, _) => "CallExpr",
            &Expr::AttrExpr(_, _) => "AttrExpr",
            &Expr::SubscrExpr(_, _) => "SubscrExpr",
            &Expr::DictExpr(_) => "DictExpr",
        };
        str.to_string()
    }
}

#[derive(Clone)]
pub enum Target {
    IdentTarget(Id),
    AttrTarget(Box<Expr>, Id),
    SubscrTarget(Box<Expr>, Box<Expr>)
}

#[derive(Clone)]
pub enum SimpleStmt {
    ExprStmt(Expr),
    AssignStmt(Target, Expr),
    BreakStmt,
    ContinueStmt,
    ReturnStmt(Expr),
    AssertStmt(Expr)
}

#[derive(Clone)]
pub enum CompoundStmt {
    IfStmt(Expr, Program, Program),
    WhileStmt(Expr, Program),
    DefStmt(Id, Vec<Id>, Program),
    ClassStmt(Id, Program)
}

#[derive(Clone)]
pub enum Stmt {
    StmtSimple(SimpleStmt),
    StmtCompound(CompoundStmt)
}

pub type Program = Vec<Stmt>;
