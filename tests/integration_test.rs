extern crate core;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::rc::Rc;

use core::lexer::*;
use core::parser::*;
use core::env::Env;
use core::eval::*;

fn run_prog_string(prog: String) {
    let tokens = tokenize(prog);
    let program = tokens.into_iter().peekable().parse();
    match program.exec(Rc::new(Env::new())) {
        CtrlOp::Nop => (),
        _ => panic!("InvalidCtrlOp")
    }
}

fn run(file_name: &str) {
    let path = ["tests/tests/", file_name, ".py"].join("");
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut prog = String::new();
    buf_reader.read_to_string(&mut prog).expect("Error: read_to_string");
    run_prog_string(prog)
}

macro_rules! test_cases {
    ( $( $i:ident ), * ) => {
        $(
            #[test]
            fn $i() {
                run(stringify!($i))
            }
        )*
    }
}

#[test]
fn assert_true() {
    run_prog_string("assert 42 == 42\n".to_string())
}

#[test]
#[should_panic]
fn assert_false() {
    run_prog_string("assert 1 == 42\n".to_string())
}

test_cases![
    if_false, if_true,
    while_normal, while_continue, while_break,
    def, def_argument, def_recursive, def_internal, def_ho, def_lexical_scope
];
