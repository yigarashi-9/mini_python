use std::iter::Peekable;
use syntax::*;
use lexer::Token;

/*
program -> statement* EOF

block -> Indent statement+ Dedent | e

statement -> simple_stmt
           | compound_stmt

simple_stmt -> Ident(s) = expr NewLine

compound_stmt -> If expr Colon NewLine block Else block

expr -> pexpr Lt pexpr
      | pexpr

pexpr -> aexpr Plus pexpr
       | aexpr

aexpr -> LParen expr RParen
       | True
       | False
       | Ident
       | Int
 */

pub trait Parser {
    fn parse(&mut self) -> Program;
    fn program(&mut self) -> Program;
    fn block(&mut self) -> Program;
    fn statement(&mut self) -> Stmt;
    fn simple_stmt(&mut self) -> SimpleStmt;
    fn compound_stmt(&mut self) -> CompoundStmt;
    fn expr(&mut self) -> Expr;
    fn pexpr(&mut self) -> Expr;
    fn aexpr(&mut self) -> Expr;
    fn match_token(&mut self, token: Token) -> bool;
    fn consume(&mut self, token: Token) -> ();
    fn consume_ident(&mut self) -> String;
    fn consume_int(&mut self) -> i32;
}

impl<I: Iterator<Item = Token>> Parser for Peekable<I> {
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
        match self.peek() {
            Some(&Token::If) => Stmt::StmtCompound(self.compound_stmt()),
            Some(_) => Stmt::StmtSimple(self.simple_stmt()),
            None => panic!("Parse Error: statement"),
        }
    }

    fn simple_stmt(&mut self) -> SimpleStmt {
        let ident = self.consume_ident();
        self.consume(Token::Eq);
        let expr = self.expr();
        self.consume(Token::NewLine);
        SimpleStmt::AssignStmt(ident, expr)
    }

    fn compound_stmt(&mut self) -> CompoundStmt {
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

    fn expr(&mut self) -> Expr {
        let expr1 = self.pexpr();
        match self.peek() {
            Some(&Token::Lt) => {
                self.consume(Token::Lt);
                let expr2 = self.pexpr();
                Expr::LtExpr(Box::new(expr1), Box::new(expr2))
            },
            Some(_) => expr1,
            None => panic!("Parse Error: expr"),
        }
    }

    fn pexpr(&mut self) -> Expr {
        let expr1 = self.aexpr();
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
