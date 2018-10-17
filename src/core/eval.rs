use std::rc::Rc;

use opcode::*;
use env::*;
use error::*;

use object::*;
use object::generic::*;
use object::listobj::*;
use object::typeobj::*;

#[derive(PartialEq)]
enum Why {
    WhyNot,
    WhyReturn,
    WhyBreak,
    WhyContinue,
    WhyException,
}

#[derive(PartialEq)]
enum BlockType {
    LoopBlock,
    TryBlock,
}

struct Block {
    b_type: BlockType,
    b_handler: Addr,
    b_level: usize,
}

struct StackMachine {
    pc: usize,
    stack: Vec<Rc<PyObject>>,
    blocks: Vec<Block>,
}

impl StackMachine {
    fn new() -> StackMachine {
        StackMachine {
            pc: 0,
            stack: vec![],
            blocks: vec![],
        }
    }

    fn top(&self) -> Rc<PyObject> {
        Rc::clone(self.stack.last().expect("Top"))
    }

    fn pop(&mut self) -> Rc<PyObject> {
        match self.stack.pop() {
            Some(obj) => obj,
            None => panic!("Implementation Error: pop"),
        }
    }

    fn pop_as_vec(&mut self, len: usize) -> Vec<Rc<PyObject>> {
        let range = (self.stack.len() - len)..;
        self.stack.drain(range).collect()
    }

    fn push(&mut self, v: Rc<PyObject>) {
        self.stack.push(v);
    }

    fn unwind_stack(&mut self, level: usize) {
        while self.stack.len() > level {
            self.stack.pop();
        }
    }

    fn exec(&mut self, code: &Code, env: Rc<Env>) -> Option<Rc<PyObject>> {
        let mut retval = None;
        let mut why = Why::WhyNot;

        while let Some(op) = code.get(self.pc) {
            match op {
                &Opcode::PopTop => {
                    self.pop();
                    self.pc += 1;
                    continue;
                },
                &Opcode::LoadConst(ref cnst) => {
                    self.push(Rc::clone(cnst));
                    self.pc += 1;
                    continue;
                },
                &Opcode::LoadName(ref id) => {
                    self.push(env.get(id));
                    self.pc += 1;
                    continue;
                },
                &Opcode::StoreName(ref id) => {
                    let top = self.pop();
                    env.update(id.clone(), top);
                    self.pc += 1;
                    continue;
                },
                &Opcode::BinaryAdd => {
                    let right = self.pop();
                    let left = self.pop();
                    let ob_type = left.ob_type();
                    let typ = ob_type.pytype_typeobj_borrow();
                    let res = (typ.tp_fun_add.as_ref().expect("Add"))(left, right);
                    self.push(res);
                    self.pc += 1;
                    continue;
                },
                &Opcode::BinaryLt => {
                    let right = self.pop();
                    let left = self.pop();
                    let ob_type = left.ob_type();
                    let typ = ob_type.pytype_typeobj_borrow();
                    let res = (typ.tp_fun_lt.as_ref().expect("Lt"))(left, right);
                    self.push(res);
                    self.pc += 1;
                    continue;
                },
                &Opcode::BinaryEq => {
                    let right = self.pop();
                    let left = self.pop();
                    let ob_type = left.ob_type();
                    let typ = ob_type.pytype_typeobj_borrow();
                    let res = (typ.tp_fun_eq.as_ref().expect("Eq"))(left, right);
                    self.push(res);
                    self.pc += 1;
                    continue;
                },
                &Opcode::MakeFunction => {
                    self.pop();  // qualname
                    let codeobj = self.pop();
                    self.push(PyObject::pyfun_new(&Rc::clone(&env), codeobj));
                    self.pc += 1;
                    continue;
                },
                &Opcode::CallFunction(argcnt) => {
                    let args = self.pop_as_vec(argcnt);
                    let fun = self.pop();

                    if let Some(res) = call_func(fun, &args) {
                        self.push(res);
                        self.pc += 1;
                        continue;
                    } else {
                        retval = None;
                    }
                },
                &Opcode::ReturnValue => {
                    retval = Some(self.pop());
                    why = Why::WhyReturn;
                },
                &Opcode::LoadAttr(ref id) => {
                    let v = self.pop();
                    let attr = PyObject::from_string(id.clone());
                    let res = pyobj_get_attro(v, attr).expect("LoadAttr");
                    self.push(res);
                    self.pc += 1;
                    continue;
                },
                &Opcode::StoreAttr(ref id) => {
                    let lv = self.pop();
                    let rv = self.pop();
                    let attr = PyObject::from_string(id.clone());
                    pyobj_set_attro(lv, attr, rv);
                    self.pc += 1;
                    continue;
                },
                &Opcode::BinarySubScr => {
                    let v2 = self.pop();
                    let v1 = self.pop();
                    if v1.pylist_check() {
                        self.push(v1.pylist_getitem(pyobj_to_i32(v2) as usize))
                    } else if v1.pydict_check() {
                        self.push(v1.pydict_lookup(v2).expect("BinarySubscr"))
                    } else {
                        panic!("BinarySubScr")
                    };
                    self.pc += 1;
                    continue;
                },
                &Opcode::StoreSubScr => {
                    let v2 = self.pop();
                    let v1 = self.pop();
                    let rv = self.pop();
                    v1.pydict_update(v2, rv);
                    self.pc += 1;
                    continue;
                },
                &Opcode::BuildList(len) => {
                    let vs = self.pop_as_vec(len);
                    self.push(PyObject::pylist_from_vec(&vs));
                    self.pc += 1;
                    continue;
                },
                &Opcode::BuildMap(len) => {
                    let mut dictobj = PyObject::pydict_new();
                    let vs = self.pop_as_vec(len * 2);
                    for i in 0..len {
                        dictobj.pydict_update(Rc::clone(&vs[i*2]), Rc::clone(&vs[i*2+1]));
                    };
                    self.push(dictobj);
                    self.pc += 1;
                    continue;
                },
                &Opcode::PopJumpIfTrue(addr) => {
                    let cond = self.pop();
                    if pyobj_to_bool(cond) {
                        self.pc = addr;
                    } else {
                        self.pc += 1;
                    };
                    continue;
                },
                &Opcode::PopJumpIfFalse(addr) => {
                    let cond = self.pop();
                    if pyobj_to_bool(cond) {
                        self.pc += 1;
                    } else {
                        self.pc = addr;
                    };
                    continue;
                },
                &Opcode::JumpAbsolute(addr) => {
                    self.pc = addr;
                    continue;
                },
                &Opcode::SetupLoop(offset) => {
                    self.blocks.push(Block {
                        b_type: BlockType::LoopBlock,
                        b_handler: self.pc + offset,
                        b_level: self.stack.len(),
                    });
                    self.pc += 1;
                    continue;
                },
                &Opcode::BreakLoop => {
                    why = Why::WhyBreak;
                },
                &Opcode::ContinueLoop(addr) => {
                    retval = Some(PyObject::from_i32(addr as i32));
                    why = Why::WhyContinue;

                },
                &Opcode::GetIter => {
                    let v = self.pop();
                    let iterfun = v.ob_type().pytype_tp_iter().expect("GetIter");
                    self.push(iterfun(Rc::clone(&v)));
                    self.pc += 1;
                    continue;
                },
                &Opcode::ForIter(addr) => {
                    let it = self.top();
                    let nextfun = it.ob_type().pytype_tp_iternext().expect("ForIter");
                    match nextfun(Rc::clone(&it)) {
                        Some(next) => {
                            self.push(next);
                            self.pc += 1;
                            continue;
                        },
                        None => {
                            self.pop();
                            self.pc = addr;
                            continue;
                        }
                    }
                },
                &Opcode::SetupExcept(offset) => {
                    self.blocks.push(Block {
                        b_type: BlockType::TryBlock,
                        b_handler: self.pc + offset,
                        b_level: self.stack.len(),
                    });
                    self.pc += 1;
                    continue;
                },
                &Opcode::Raise => {
                    let mut exc = self.pop();

                    if PyObject::pyexc_is_exc_subclass(Rc::clone(&exc)) {
                        exc = type_call(exc, &vec![]);
                    } else if !PyObject::pyexc_is_exc_instance(Rc::clone(&exc)) {
                        panic!("Type Error: Raise")
                    }
                    pyerr_set(exc);
                    why = Why::WhyException;
                },
                &Opcode::PopBlock => {
                    let block = self.blocks.pop().unwrap();
                    self.unwind_stack(block.b_level);
                    self.pc += 1;
                    continue;
                },
                &Opcode::MakeClass(nbases) => {
                    let nameobj = self.pop();
                    let codeobj = self.pop();
                    let bases = self.pop_as_vec(nbases);

                    let new_env = Rc::new(Env::new_child(&env, &vec![], &vec![]));
                    eval(&codeobj.pycode_code(), Rc::clone(&new_env));
                    let dictobj = new_env.dictobj();
                    let cls = PyObject::pytype_new();

                    {
                        let mut typ = cls.pytype_typeobj_borrow_mut();
                        typ.tp_dict = Some(Rc::clone(&dictobj));
                        typ.tp_name = pyobj_to_string(nameobj);
                        typ.tp_bases = Some(PyObject::pylist_from_vec(&bases));
                    }

                    for base in &bases {
                        let mut typ = base.pytype_typeobj_borrow_mut();
                        if typ.tp_subclasses.is_none() {
                            typ.tp_subclasses = Some(PyObject::pylist_from_vec(&vec![]));
                        }
                        pylist_append(Rc::clone(typ.tp_subclasses.as_ref().unwrap()),
                                      Rc::clone(&cls));
                    }

                    pytype_ready(Rc::clone(&cls));
                    self.push(cls);
                    self.pc += 1;
                    continue;
                },
            }

            if why == Why::WhyNot {
                why = Why::WhyException;
            }

            while self.blocks.len() > 0 {

                {
                    let block = self.blocks.last().unwrap();

                    if block.b_type == BlockType::LoopBlock && why == Why::WhyContinue {
                        why = Why::WhyNot;
                        let ret = Rc::clone(retval.as_ref().expect("Continue ret addr"));
                        self.pc = pyobj_to_i32(ret) as usize;
                        break;
                    }
                }

                let block = self.blocks.pop().unwrap();
                self.unwind_stack(block.b_level);

                if block.b_type == BlockType::LoopBlock && why == Why::WhyBreak {
                    why = Why::WhyNot;
                    self.pc = block.b_handler;
                    break;
                }

                if block.b_type == BlockType::TryBlock && why == Why::WhyException {
                    if !pyerr_occurred() { panic!("Implementation Error: try block") };
                    pyerr_clear();
                    why = Why::WhyNot;
                    self.pc = block.b_handler;
                    break;
                }

                // why == WhyReturn
            }

            if why != Why::WhyNot {
                break;
            }
        }
        retval
    }
}

pub fn eval(code: &Code, env: Rc<Env>) -> Option<Rc<PyObject>> {
    let mut stack_machine = StackMachine::new();
    stack_machine.exec(code, env)
}
