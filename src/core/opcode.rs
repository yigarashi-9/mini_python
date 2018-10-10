use std::fmt;
use std::rc::Rc;

use syntax::Id;
use object::PyObject;

pub type Addr = usize;
pub type Offset = usize;
pub type Code = Vec<Opcode>;

#[derive(Clone)]
pub enum Opcode {
    PopTop,
    LoadConst(Rc<PyObject>),
    LoadName(Id),
    StoreName(Id),
    BinaryAdd,
    BinaryEq,
    BinaryLt,
    MakeFunction,
    CallFunction(usize),
    ReturnValue,
    LoadAttr(Id),
    StoreAttr(Id),
    BinarySubScr,
    StoreSubScr,
    BuildList(usize),
    BuildMap(usize),
    PopJumpIfTrue(Addr),
    PopJumpIfFalse(Addr),
    JumpAbsolute(Addr),
    SetupLoop(Offset),
    BreakLoop,
    ContinueLoop(Addr),
    GetIter,
    ForIter(Addr),
    PopBlock,
    MakeClass(usize),
    Panic,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Opcode::PopTop => write!(f, "{}", "PopTop"),
            &Opcode::LoadConst(ref _obj) => write!(f, "{}", "LoadConst"),
            &Opcode::LoadName(ref id) => write!(f, "{} {}", "LoadName", id),
            &Opcode::StoreName(ref id) => write!(f, "{} {}", "StoreName", id),
            &Opcode::BinaryAdd => write!(f, "{}", "BinaryAdd"),
            &Opcode::BinaryLt => write!(f, "{}", "BinaryLt"),
            &Opcode::BinaryEq => write!(f, "{}", "BinaryEq"),
            &Opcode::MakeFunction => write!(f, "{}", "MakeFunction"),
            &Opcode::CallFunction(argcnt) => write!(f, "{} {}", "CallFunction", argcnt),
            &Opcode::ReturnValue => write!(f, "{}", "ReturnValue"),
            &Opcode::LoadAttr(ref id) => write!(f, "{} {}", "LoadAttr", id),
            &Opcode::StoreAttr(ref id) => write!(f, "{} {}", "StoreAttr", id),
            &Opcode::BinarySubScr => write!(f, "{}", "BinarySubscr"),
            &Opcode::StoreSubScr => write!(f, "{}", "StoreSubscr"),
            &Opcode::BuildList(len) => write!(f, "{} {}", "BuildList", len),
            &Opcode::BuildMap(len) => write!(f, "{} {}", "BuildMap", len),
            &Opcode::PopJumpIfTrue(addr) => write!(f, "{} {}", "PopJumpIfTrue", addr),
            &Opcode::PopJumpIfFalse(addr) => write!(f, "{} {}", "PopJumpIfFalse", addr),
            &Opcode::JumpAbsolute(addr) => write!(f, "{} {}", "JumpAbsolute", addr),
            &Opcode::SetupLoop(offset) => write!(f, "{} {}", "SetupLoop", offset),
            &Opcode::BreakLoop => write!(f, "{}", "BreakLoop"),
            &Opcode::ContinueLoop(addr)=> write!(f, "{} {}", "ContinueLoop", addr),
            &Opcode::GetIter=> write!(f, "{}", "GetIter"),
            &Opcode::ForIter(addr)=> write!(f, "{} {}", "ForIter", addr),
            &Opcode::PopBlock=> write!(f, "{}", "PopBlock"),
            &Opcode::MakeClass(nbases)=> write!(f, "{} {}", "MakeClass", nbases),
            &Opcode::Panic=> write!(f, "{}", "Panic"),
        }
    }
}

pub fn print_code(code: &Code) {
    for i in 0..code.len() {
        println!("{:<5} {}", i, &code[i])
    }
}
