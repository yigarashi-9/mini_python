pub type Id = String;

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
    ListExpr(Vec<Expr>),
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
            &Expr::ListExpr(_) => "ListExpr",
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
