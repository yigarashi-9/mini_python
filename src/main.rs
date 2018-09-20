extern crate core;

use std::fs::File;
use std::io::{stderr, BufReader};
use std::io::prelude::*;
use std::rc::Rc;

use core::lexer::*;
use core::parser::*;
use core::env::Env;
use core::eval::*;
use core::builtinmodule::*;

fn main() -> std::io::Result<()> {
    let file = File::open("test.py").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut prog = String::new();
    buf_reader.read_to_string(&mut prog).expect("Error: read_to_string");
    match tokenize(prog) {
        Ok(tokens) => {
            let program = tokens.into_iter().peekable().parse();
            let env = Rc::new(Env::new());
            load_builtins(Rc::clone(&env));
            match program.exec(env) {
                CtrlOp::Nop => (),
                _ => panic!("Invalid control operator")
            }
        },
        Err(err) => {
            let mut stderr = stderr();
            stderr.write(err.to_string().as_bytes())?;
            stderr.flush()?;
        }
    };
    Ok(())
}
