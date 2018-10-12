use object::*;
use opcode::*;
use syntax::*;

struct AddrInfo {
    start: Addr,
    cont: Option<Addr>,
}

impl AddrInfo {
    fn new() -> AddrInfo {
        AddrInfo {
            start: 0,
            cont: None,
        }
    }

    fn change_start(&self, start: Addr) -> AddrInfo {
        AddrInfo {
            start: start,
            cont: self.cont
        }
    }
}

fn compile_expr(expr: &Expr) -> Code {
    let mut code = vec![];
    match expr {
        &Expr::VarExpr(ref id) => code.push(Opcode::LoadName(id.clone())),
        &Expr::IntExpr(i) => code.push(Opcode::LoadConst(PyObject::from_i32(i))),
        &Expr::BoolExpr(b) => code.push(Opcode::LoadConst(PyObject::from_bool(b))),
        &Expr::StrExpr(ref s) => code.push(Opcode::LoadConst(PyObject::from_string(s.clone()))),
        &Expr::NoneExpr => code.push(Opcode::LoadConst(PyObject::none_obj())),
        &Expr::AddExpr(ref e1, ref e2) => {
            code.append(&mut compile_expr(e1));
            code.append(&mut compile_expr(e2));
            code.push(Opcode::BinaryAdd);
        },
        &Expr::LtExpr(ref e1, ref e2) => {
            code.append(&mut compile_expr(e1));
            code.append(&mut compile_expr(e2));
            code.push(Opcode::BinaryLt);
        },
        &Expr::EqEqExpr(ref e1, ref e2) => {
            code.append(&mut compile_expr(e1));
            code.append(&mut compile_expr(e2));
            code.push(Opcode::BinaryEq);
        },
        &Expr::CallExpr(ref fun, ref args) => {
            code.append(&mut compile_expr(fun));
            for arg in args {
                code.append(&mut compile_expr(arg));
            };
            code.push(Opcode::CallFunction(args.len()));
        },
        &Expr::AttrExpr(ref e, ref ident) => {
            code.append(&mut compile_expr(e));
            code.push(Opcode::LoadAttr(ident.clone()));
        },
        &Expr::SubscrExpr(ref e1, ref e2) => {
            code.append(&mut compile_expr(e1));
            code.append(&mut compile_expr(e2));
            code.push(Opcode::BinarySubScr);
        },
        &Expr::ListExpr(ref cl) => {
            for c in cl {
                code.append(&mut compile_expr(c));
            };
            code.push(Opcode::BuildList(cl.len()));
        },
        &Expr::DictExpr(ref pl) => {
            for (e1, e2) in pl {
                code.append(&mut compile_expr(e1));
                code.append(&mut compile_expr(e2));
            };
            code.push(Opcode::BuildMap(pl.len()));
        },
    };
    code
}

fn compile_target(target: &Target) -> Code {
    let mut code = vec![];
    match target {
        &Target::IdentTarget(ref id) => {
            code.push(Opcode::StoreName(id.clone()));
        },
        &Target::AttrTarget(ref lexpr, ref id) => {
            code.append(&mut compile_expr(lexpr));
            code.push(Opcode::StoreAttr(id.clone()));
        },
        &Target::SubscrTarget(ref e1, ref e2) => {
            code.append(&mut compile_expr(e1));
            code.append(&mut compile_expr(e2));
            code.push(Opcode::StoreSubScr);
        }
    };
    code
}

fn compile_simple_stmt(stmt: &SimpleStmt, addr_info: AddrInfo) -> Code {
    let mut code = vec![];
    match stmt {
        &SimpleStmt::ExprStmt(ref expr) => {
            code.append(&mut compile_expr(expr));
            code.push(Opcode::PopTop);
        },
        &SimpleStmt::AssignStmt(ref target, ref expr) => {
            code.append(&mut compile_expr(expr));
            code.append(&mut compile_target(target));
        },
        &SimpleStmt::ReturnStmt(ref expr) => {
            code.append(&mut compile_expr(expr));
            code.push(Opcode::ReturnValue);
        },
        &SimpleStmt::BreakStmt => code.push(Opcode::BreakLoop),
        &SimpleStmt::ContinueStmt => {
            if let Some(cont) = addr_info.cont {
                code.push(Opcode::ContinueLoop(cont));
            } else {
                panic!("continue outside loop block")
            }
        }
        &SimpleStmt::AssertStmt(ref expr) => {
            let mut expr_code = compile_expr(expr);
            let pop_jump_addr = addr_info.start + expr_code.len() + 2;
            code.append(&mut expr_code);
            code.push(Opcode::PopJumpIfTrue(pop_jump_addr));
            code.push(Opcode::Panic);
        }
    };
    code
}

fn compile_compound_stmt(stmt: &CompoundStmt, addr_info: AddrInfo) -> Code {
    let mut code = vec![];
    match stmt {
        &CompoundStmt::IfStmt(ref expr, ref prog_then, ref prog_else) => {
            let mut expr_code = compile_expr(expr);
            let mut addr = addr_info.start + expr_code.len() + 1;
            let mut then_code = compile_program(prog_then, addr_info.change_start(addr));
            addr += then_code.len() + 1;
            let pop_jump_addr = addr;
            let mut else_code = compile_program(prog_else, addr_info.change_start(addr));
            addr += else_code.len();

            code.append(&mut expr_code);
            code.push(Opcode::PopJumpIfFalse(pop_jump_addr));
            code.append(&mut then_code);
            code.push(Opcode::JumpAbsolute(addr));
            code.append(&mut else_code);
        },
        &CompoundStmt::WhileStmt(ref expr, ref prog) => {
            let mut expr_code = compile_expr(expr);
            let mut addr = addr_info.start + 1 + expr_code.len() + 1;
            let body_addr_info = AddrInfo {
                start: addr,
                cont: Some(addr_info.start + 1),
            };
            let mut body_code = compile_program(prog, body_addr_info);
            addr += body_code.len() + 1;

            code.push(Opcode::SetupLoop(addr - addr_info.start + 1));
            code.append(&mut expr_code);
            code.push(Opcode::PopJumpIfFalse(addr));
            code.append(&mut body_code);
            code.push(Opcode::JumpAbsolute(addr_info.start + 1));
            code.push(Opcode::PopBlock);
        },
        &CompoundStmt::ForStmt(ref target, ref expr, ref prog) => {
            let mut expr_code = compile_expr(expr);
            let mut addr = addr_info.start + expr_code.len() + 3;
            let for_iter_addr = addr - 1;
            let mut target_code = compile_target(target);
            addr += target_code.len();
            let body_addr_info = AddrInfo {
                start: addr,
                cont: Some(for_iter_addr),
            };
            let mut body_code = compile_program(prog, body_addr_info);
            addr += body_code.len() + 1;

            code.push(Opcode::SetupLoop(addr - addr_info.start + 1));
            code.append(&mut expr_code);
            code.push(Opcode::GetIter);
            code.push(Opcode::ForIter(addr));
            code.append(&mut target_code);
            code.append(&mut body_code);
            code.push(Opcode::JumpAbsolute(for_iter_addr));
            code.push(Opcode::PopBlock);
        },
        &CompoundStmt::DefStmt(ref id, ref parms, ref prog) => {
            let body_code = compile(prog);
            code.push(Opcode::LoadConst(PyObject::pycode_new(body_code, parms.clone())));
            code.push(Opcode::LoadConst(PyObject::from_string(id.clone())));
            code.push(Opcode::MakeFunction);
            code.push(Opcode::StoreName(id.clone()));
        },
        &CompoundStmt::ClassStmt(ref id, ref bases, ref prog) => {
            for base in bases {
                code.append(&mut compile_expr(base));
            };
            let body_code = compile(prog);
            code.push(Opcode::LoadConst(PyObject::pycode_new(body_code, vec![])));
            code.push(Opcode::LoadConst(PyObject::from_str(id)));
            code.push(Opcode::MakeClass(bases.len()));
            code.push(Opcode::StoreName(id.clone()));
        }
    };
    code
}

fn compile_stmt(stmt: &Stmt, addr_info: AddrInfo) -> Code {
    match stmt {
        &Stmt::StmtSimple(ref simple_stmt) => compile_simple_stmt(simple_stmt, addr_info),
        &Stmt::StmtCompound(ref compound_stmt) =>
            compile_compound_stmt(compound_stmt, addr_info),
    }
}

fn compile_program(prog: &Program, addr_info: AddrInfo) -> Code {
    let mut code = vec![];
    let mut addr = addr_info.start;
    for stmt in prog {
        let new_addr_info = addr_info.change_start(addr);
        let mut stmt_code = compile_stmt(stmt, new_addr_info);
        addr += stmt_code.len();
        code.append(&mut stmt_code);
    };
    code
}

pub fn compile(prog: &Program) -> Code {
    let mut code = compile_program(prog, AddrInfo::new());
    code.push(Opcode::LoadConst(PyObject::none_obj()));
    code.push(Opcode::ReturnValue);
    code
}
