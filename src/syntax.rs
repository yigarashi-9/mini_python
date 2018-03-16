pub type Id = String;

pub enum Value {
    IntVal(i32),
    BoolVal(bool)
}

pub enum Expr {
    VarExpr(Id),
    IntExpr(i32),
    BoolExpr(bool),
    AddExpr(Box<Expr>, Box<Expr>),
    LtExpr(Box<Expr>, Box<Expr>)
}

pub enum SimpleStmt {
    AssignStmt(Id, Expr)
}

pub enum CompoundStmt {
    IfStmt(Expr, Program, Program)
}

pub enum Stmt {
    StmtSimple(SimpleStmt),
    StmtCompound(CompoundStmt)
}

pub type Program = Vec<Stmt>;
