extern crate core;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use core::syntax::*;
use core::lexer::*;
use core::parser::*;
use core::env::*;
use core::eval::*;

fn main() {
    let file = File::open("test.py").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut prog = String::new();
    buf_reader.read_to_string(&mut prog).expect("Error: read_to_string");
    let tokens = tokenize(prog);
    let program = tokens.into_iter().peekable().parse();
    let mut env = Env::new();
    match program.exec(&mut env) {
        CtrlOp::Nop => (),
        _ => panic!("Invalid control operator")
    };
    match env.get(&String::from("x")) {
        &Value::IntVal(i) => print!("{}", i),
        &Value::BoolVal(b) => print!("{}", b),
        &Value::NoneVal => print!("None"),
        &Value::FunVal(_, _) => print!("<fun>"),
    };
}
