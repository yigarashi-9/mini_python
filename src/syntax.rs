pub type Id = String;

#[derive(Clone)]
pub enum Value {
    IntVal(i32),
    BoolVal(bool),
    NoneVal,
    FunVal(Vec<Id>, Program)
}

#[derive(Clone)]
pub enum Expr {
    VarExpr(Id),
    IntExpr(i32),
    BoolExpr(bool),
    NoneExpr,
    AddExpr(Box<Expr>, Box<Expr>),
    EqEqExpr(Box<Expr>, Box<Expr>),
    CallExpr(Box<Expr>, Vec<Expr>)
}

#[derive(Clone)]
pub enum SimpleStmt {
    AssignStmt(Id, Expr),
    BreakStmt,
    ContinueStmt,
    ReturnStmt(Expr),
    AssertStmt(Expr)
}

#[derive(Clone)]
pub enum CompoundStmt {
    IfStmt(Expr, Program, Program),
    WhileStmt(Expr, Program),
    DefStmt(Id, Vec<Id>, Program)
}

#[derive(Clone)]
pub enum Stmt {
    StmtSimple(SimpleStmt),
    StmtCompound(CompoundStmt)
}

pub type Program = Vec<Stmt>;
