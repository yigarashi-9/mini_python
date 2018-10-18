use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use std::rc::Rc;

use opcode::*;
use lexer::*;
use parser::*;
use compile::*;
use env::Env;
use eval::*;
use error::*;
use builtinmodule::*;

pub fn run_prog_string(prog: String) {
    match tokenize(prog) {
        Ok(tokens) => {
            let ast = tokens.into_iter().peekable().parse();
            let code = compile(&ast);
            // print_code(&code);
            let env = Rc::new(Env::new());
            pyerr_clear();
            load_builtins(Rc::clone(&env));
            eval(&code, env);
        },
        Err(err) => panic!(err.to_string()),
    };
}

pub fn run(path: &str) {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut prog = String::new();
    buf_reader.read_to_string(&mut prog).expect("Error: read_to_string");
    run_prog_string(prog);
}
