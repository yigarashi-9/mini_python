use std::iter::Peekable;
use syntax::*;
use token::Token;

/*
program -> statement* EOF

block -> Indent statement+ Dedent | e

statement -> simple_stmt
           | compound_stmt

simple_stmt ->
  | Ident(s) = expr NewLine
  | Return expr?
  | Continue
  | Break
  | Assert expr

compound_stmt ->
  | If expr Colon NewLine block Else block
  | While expr Colon NewLine block
  | Def Ident(s) LParen parm_list RParen Colon NewLine block

parm_list ->
  | Ident(s)
  | Ident(s) Comma parm_list
  | e

expr -> eexpr Lt eexpr
      | eexpr

eexpr -> pexpr EqEq pexpr
       | pexpr

pexpr -> cexpr Plus pexpr
       | cexpr

cexpr -> aexpr LParen arg_list RParen
       | aexpr

arg_list ->
  | expr
  | expr Comma arg_list
  | e

aexpr -> LParen expr RParen
       | True
       | False
       | Ident
       | Int
       | None

 */

pub trait TokenStream {
    fn parse(&mut self) -> Program;
    fn program(&mut self) -> Program;
    fn block(&mut self) -> Program;
    fn statement(&mut self) -> Stmt;
    fn simple_stmt(&mut self) -> SimpleStmt;
    fn is_compound(&mut self) -> bool;
    fn compound_stmt(&mut self) -> CompoundStmt;
    fn if_stmt(&mut self) -> CompoundStmt;
    fn while_stmt(&mut self) -> CompoundStmt;
    fn def_stmt(&mut self) -> CompoundStmt;
    fn parm_list(&mut self) -> Vec<Id>;
    fn expr(&mut self) -> Expr;
    fn eexpr(&mut self) -> Expr;
    fn pexpr(&mut self) -> Expr;
    fn cexpr(&mut self) -> Expr;
    fn arg_list(&mut self) -> Vec<Expr>;
    fn is_expr(&mut self) -> bool;
    fn aexpr(&mut self) -> Expr;
    fn match_token(&mut self, token: Token) -> bool;
    fn consume(&mut self, token: Token) -> ();
    fn consume_ident(&mut self) -> String;
    fn consume_int(&mut self) -> i32;
}

impl<I: Iterator<Item = Token>> TokenStream for Peekable<I> {
    fn parse(&mut self) -> Program {
        self.program()
    }

    fn program(&mut self) -> Program {
        let mut prog: Program = vec![];
        loop {
            match self.peek() {
                Some(&Token::EOF) => break,
                Some(_) => prog.push(self.statement()),
                None => panic!("Parse Error: program"),
            }
        };
        prog
    }

    fn block(&mut self) -> Program {
        let mut prog: Program = vec![];
        match self.peek() {
            Some(&Token::Indent) => {
                self.consume(Token::Indent);
                prog.push(self.statement());
                loop {
                    match self.peek() {
                        Some(&Token::Dedent) => {
                            self.consume(Token::Dedent);
                            break
                        },
                        Some(_) => prog.push(self.statement()),
                        _ => panic!("Parse Error: block")
                    }
                }
            },
            _ => (),
        };
        prog
    }

    fn statement(&mut self) -> Stmt {
        if self.is_compound() {
            Stmt::StmtCompound(self.compound_stmt())
        } else {
            Stmt::StmtSimple(self.simple_stmt())
        }
    }

    fn simple_stmt(&mut self) -> SimpleStmt {
        match self.peek() {
            Some(&Token::Break) => {
                self.consume(Token::Break);
                self.consume(Token::NewLine);
                SimpleStmt::BreakStmt
            },
            Some(&Token::Continue) => {
                self.consume(Token::Continue);
                self.consume(Token::NewLine);
                SimpleStmt::ContinueStmt
            },
            Some(&Token::Return) => {
                self.consume(Token::Return);
                let mut expr = Expr::NoneExpr;
                if !self.match_token(Token::NewLine) {
                    expr = self.expr();
                }
                self.consume(Token::NewLine);
                SimpleStmt::ReturnStmt(expr)
            },
            Some(&Token::Assert) => {
                self.consume(Token::Assert);
                let expr = self.expr();
                self.consume(Token::NewLine);
                SimpleStmt::AssertStmt(expr)
            },
            _ => {
                let ident = self.consume_ident();
                self.consume(Token::Eq);
                let expr = self.expr();
                self.consume(Token::NewLine);
                SimpleStmt::AssignStmt(ident, expr)
            },
        }
    }

    fn is_compound(&mut self) -> bool {
        match self.peek() {
            Some(&Token::If) => true,
            Some(&Token::While) => true,
            Some(&Token::Def) => true,
            _ => false,
        }
    }

    fn compound_stmt(&mut self) -> CompoundStmt {
        match self.peek() {
            Some(&Token::If) => self.if_stmt(),
            Some(&Token::While) => self.while_stmt(),
            Some(&Token::Def) => self.def_stmt(),
            _ => panic!("Parse Error: compound_stmt"),
        }
    }

    fn if_stmt(&mut self) -> CompoundStmt {
        self.consume(Token::If);
        let expr = self.expr();
        self.consume(Token::Colon);
        self.consume(Token::NewLine);
        let prog_then = self.block();
        self.consume(Token::Else);
        self.consume(Token::Colon);
        self.consume(Token::NewLine);
        let prog_else = self.block();
        CompoundStmt::IfStmt(expr, prog_then, prog_else)
    }

    fn while_stmt(&mut self) -> CompoundStmt {
        self.consume(Token::While);
        let expr = self.expr();
        self.consume(Token::Colon);
        self.consume(Token::NewLine);
        let prog = self.block();
        CompoundStmt::WhileStmt(expr, prog)
    }

    fn def_stmt(&mut self) -> CompoundStmt {
        self.consume(Token::Def);
        let fun_name = self.consume_ident();
        self.consume(Token::LParen);
        let parm_list = self.parm_list();
        self.consume(Token::RParen);
        self.consume(Token::Colon);
        self.consume(Token::NewLine);
        let prog = self.block();
        CompoundStmt::DefStmt(fun_name, parm_list, prog)
    }

    fn parm_list(&mut self) -> Vec<Id> {
        let mut pl: Vec<Id>  = vec![];
        match self.peek() {
            Some(&Token::Ident(_)) => pl.push(self.consume_ident()),
            Some(_) => return pl,
            _ => panic!("Parse Error: parm_list"),
        };
        loop {
            match self.peek() {
                Some(&Token::Comma) => {
                    self.consume(Token::Comma);
                    pl.push(self.consume_ident());
                },
                Some(_) => break,
                _ => panic!("Parse Error: parm_list"),
            }
        };
        pl
    }

    fn expr(&mut self) -> Expr {
        let expr1 = self.eexpr();
        match self.peek() {
            Some(&Token::Lt) => {
                self.consume(Token::Lt);
                let expr2 = self.eexpr();
                Expr::LtExpr(Box::new(expr1), Box::new(expr2))
            },
            Some(_) => expr1,
            None => panic!("Parse Error: expr"),
        }
    }

    fn eexpr(&mut self) -> Expr {
        let expr1 = self.pexpr();
        match self.peek() {
            Some(&Token::EqEq) => {
                self.consume(Token::EqEq);
                let expr2 = self.pexpr();
                Expr::EqEqExpr(Box::new(expr1), Box::new(expr2))
            },
            Some(_) => expr1,
            None => panic!("Parse Error: expr"),
        }
    }

    fn pexpr(&mut self) -> Expr {
        let expr1 = self.cexpr();
        match self.peek() {
            Some(&Token::Plus) => {
                self.consume(Token::Plus);
                let expr2 = self.pexpr();
                Expr::AddExpr(Box::new(expr1), Box::new(expr2))
            },
            Some(_) => expr1,
            None => panic!("Parse Error: pexpr"),
        }
    }

    fn cexpr(&mut self) -> Expr {
        let expr = self.aexpr();
        match self.peek() {
            Some(&Token::LParen) => {
                self.consume(Token::LParen);
                let arg_list = self.arg_list();
                self.consume(Token::RParen);
                Expr::CallExpr(Box::new(expr), arg_list)
            },
            Some(_) => expr,
            None => panic!("Parse Error: pexpr"),
        }
    }

    fn arg_list(&mut self) -> Vec<Expr> {
        let mut al: Vec<Expr>  = vec![];

        if self.is_expr() {
            al.push(self.expr());
        } else {
            return al;
        }

        loop {
            match self.peek() {
                Some(&Token::Comma) => {
                    self.consume(Token::Comma);
                    al.push(self.expr());
                },
                Some(_) => break,
                _ => panic!("Parse Error: parm_list"),
            }
        };
        al
    }

    fn is_expr(&mut self) -> bool {
        match self.peek() {
            Some(&Token::LParen) => true,
            Some(&Token::True) => true,
            Some(&Token::False) => true,
            Some(&Token::Ident(_)) => true,
            Some(&Token::Int(_)) => true,
            Some(&Token::None) => true,
            _ => false,
        }
    }

    fn aexpr(&mut self) -> Expr {
        match self.peek().unwrap() {
            &Token::LParen => {
                self.consume(Token::LParen);
                let expr = self.expr();
                self.consume(Token::RParen);
                expr
            },
            &Token::True => {
                self.consume(Token::True);
                Expr::BoolExpr(true)
            },
            &Token::False => {
                self.consume(Token::False);
                Expr::BoolExpr(false)
            },
            &Token::Ident(_) => {
                let ident = self.consume_ident();
                Expr::VarExpr(ident)
            },
            &Token::Int(_) => {
                let i = self.consume_int();
                Expr::IntExpr(i)
            },
            _ => panic!("Parse Error: aexpr"),
        }
    }

    fn match_token(&mut self, token: Token) -> bool {
        match self.peek() {
            Some(token_) if token == *token_ => true,
            _ => false,
        }
    }

    fn consume(&mut self, token: Token) -> () {
        if self.match_token(token) {
            self.next();
        } else {
            panic!("Unexpected token");
        }
    }

    fn consume_ident(&mut self) -> String {
        match self.next() {
            Some(Token::Ident(ref s)) => s.clone(),
            _ => panic!("Unexpected token"),
        }
    }

    fn consume_int(&mut self) -> i32 {
        match self.next() {
            Some(Token::Int(ref i)) => i.clone(),
            _ => panic!("Unexpected token"),
        }
    }
}
