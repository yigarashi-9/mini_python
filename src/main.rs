extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use core::syntax::*;
use core::lexer::*;
use core::parser::*;
use core::eval::*;

fn main() {
    let file = File::open("test.py").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut prog = String::new();
    buf_reader.read_to_string(&mut prog);
    let tokens = tokenize(prog);

    print_tokens(&tokens);

    let program = tokens.into_iter().peekable().parse();
    let mut env = HashMap::new();
    program.exec(&mut env);
    match env.get(&String::from("x")).unwrap() {
        &Value::IntVal(i) => print!("{}", i),
        &Value::BoolVal(b) => print!("{}", b),
        _ => panic!("Error")
    };
}
