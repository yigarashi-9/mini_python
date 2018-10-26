use std::rc::Rc;

use opcode::*;
use env::*;
use error::*;

use object::*;
use object::excobj::*;
use object::generic::*;
use object::typeobj::*;

pub type PyRes<T> = Result<T, ()>;

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

    fn exec(&mut self, code: &Code, env: Rc<Env>) -> PyRes<Rc<PyObject>> {
        let mut retval = Err(());
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
                    match typ.tp_fun_add {
                        Some(ref fun) => {
                            let res = fun(left, right);
                            if res.is_ok() {
                                self.push(res.expect("Never fails"));
                                self.pc += 1;
                                continue;
                            }
                        },
                        None => {
                            pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                             "no __add__ operation");
                        }
                    }
                    why = Why::WhyException;
                    retval = Err(());
                },
                &Opcode::BinaryLt => {
                    let right = self.pop();
                    let left = self.pop();
                    let ob_type = left.ob_type();
                    let typ = ob_type.pytype_typeobj_borrow();
                    match typ.tp_fun_lt {
                        Some(ref fun) => {
                            let res = fun(left, right);
                            if res.is_ok() {
                                self.push(res.expect("Never fails"));
                                self.pc += 1;
                                continue;
                            }
                        },
                        None => {
                            pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                             "no __lt__ operation");
                        }
                    }
                    why = Why::WhyException;
                    retval = Err(());
                },
                &Opcode::BinaryEq => {
                    let right = self.pop();
                    let left = self.pop();
                    let ob_type = left.ob_type();
                    let typ = ob_type.pytype_typeobj_borrow();
                    match typ.tp_fun_eq {
                        Some(ref fun) => {
                            let res = fun(left, right);
                            if res.is_ok() {
                                self.push(res.expect("Never fails"));
                                self.pc += 1;
                                continue;
                            }
                        },
                        None => {
                            pyerr_set_string(PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                             "no __eq__ operation");
                        }
                    }
                    why = Why::WhyException;
                    retval = Err(());
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
                    let res = call_func(fun, &args);
                    if res.is_ok() {
                        self.push(res.expect("Never fails"));
                        self.pc += 1;
                        continue;
                    }
                    why = Why::WhyException;
                    retval = Err(());
                },
                &Opcode::ReturnValue => {
                    retval = Ok(self.pop());
                    why = Why::WhyReturn;
                },
                &Opcode::LoadAttr(ref id) => {
                    let v = self.pop();
                    let attr = PyObject::from_string(id.clone());
                    let res = pyobj_get_attr(v, attr);
                    if res.is_ok() {
                        self.push(res.expect("Never fails"));
                        self.pc += 1;
                        continue;
                    }
                    why = Why::WhyException;
                    retval = Err(());
                },
                &Opcode::StoreAttr(ref id) => {
                    let lv = self.pop();
                    let rv = self.pop();
                    let attr = PyObject::from_string(id.clone());
                    let res = pyobj_set_attr(lv, attr, rv);
                    if res.is_ok() {
                        self.pc += 1;
                        continue;
                    }
                    why = Why::WhyException;
                    retval = Err(());
                },
                &Opcode::BinarySubScr => {
                    let v2 = self.pop();
                    let v1 = self.pop();

                    if v1.pylist_check() {
                        let index = pyobj_to_i32(v2);
                        if index.is_ok() {
                            let res = v1.pylist_getitem(index.expect("Never fails") as usize);
                            if res.is_ok() {
                                self.push(res.expect("Never fails"));
                                self.pc += 1;
                                continue;
                            }
                        }
                    } else if v1.pydict_check() {
                        let res = v1.pydict_lookup(v2);
                        if res.is_ok() {
                            match res.expect("Never fails") {
                                Some(res) => {
                                    self.push(res);
                                    self.pc += 1;
                                    continue;
                                },
                                None => {
                                    pyerr_set_string(
                                        PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                        "item not found");
                                }
                            }
                        }
                    } else {
                        pyerr_set_string(
                            PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                            "subscripting not supported");
                    };

                    why = Why::WhyException;
                    retval = Err(());
                },
                &Opcode::StoreSubScr => {
                    let v2 = self.pop();
                    let v1 = self.pop();
                    let rv = self.pop();
                    let res = v1.pydict_update(v2, rv);

                    if res.is_ok() {
                        self.pc += 1;
                        continue;
                    }
                    why = Why::WhyException;
                    retval = Err(());
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
                    let b = pyobj_to_bool(cond);
                    if b.is_ok() {
                        if b.expect("Never fails") {
                            self.pc = addr;
                        } else {
                            self.pc += 1;
                        };
                        continue;
                    }
                    why = Why::WhyException;
                    retval = Err(());
                },
                &Opcode::PopJumpIfFalse(addr) => {
                    let cond = self.pop();
                    let b = pyobj_to_bool(cond);
                    if b.is_ok() {
                        if b.expect("Never fails") {
                            self.pc += 1;
                        } else {
                            self.pc = addr;
                        };
                        continue;
                    }
                    why = Why::WhyException;
                    retval = Err(());
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
                    retval = Ok(PyObject::from_i32(addr as i32));
                    why = Why::WhyContinue;

                },
                &Opcode::GetIter => {
                    let v = self.pop();
                    match v.ob_type().pytype_tp_iter() {
                        Some(ref iterfun) => {
                            let iter = iterfun(Rc::clone(&v));
                            if iter.is_ok() {
                                self.push(iter.expect("Never fails"));
                                self.pc += 1;
                                continue;
                            }
                        },
                        None => {
                            pyerr_set_string(
                                PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                                "__iter__ is not found");
                        }
                    }
                    why = Why::WhyException;
                    retval = Err(());
                },
                &Opcode::ForIter(addr) => {
                    let it = self.top();
                    let nextfun = it.ob_type().pytype_tp_iternext().expect("Never fails");
                    let next = nextfun(Rc::clone(&it));
                    if next.is_ok() {
                        match next.expect("Never fails") {
                            Some(next) => {
                                self.push(next);
                                self.pc += 1;
                                continue;
                            },
                            None => {
                                if pyerr_check(PY_STOPITERATION_TYPE.with(|tp| Rc::clone(tp))) {
                                    pyerr_clear();
                                } else if !pyerr_occurred() {
                                    self.pop();
                                    self.pc = addr;
                                    continue;
                                } else {
                                    why = Why::WhyException;
                                    retval = Err(());
                                }
                            }
                        }
                    } else {
                        why = Why::WhyException;
                        retval = Err(());
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
                        let res = type_call(exc, &vec![]);
                        if res.is_ok() {
                            exc = res.expect("Never fails");
                            pyerr_set(exc);
                        }
                    } else if PyObject::pyexc_is_exc_instance(Rc::clone(&exc)) {
                        pyerr_set(exc);
                    } else {
                        pyerr_set_string(
                            PY_TYPEERROR_TYPE.with(|tp| Rc::clone(tp)),
                            "raise expects exception type or exception instance");
                    }
                    why = Why::WhyException;
                    retval = Err(());
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
                    let bases = PyObject::pylist_from_vec(&self.pop_as_vec(nbases));

                    let new_env = Rc::new(Env::new_child(&env, &vec![], &vec![]));
                    eval(&codeobj.pycode_code(), Rc::clone(&new_env));
                    let dictobj = new_env.dictobj();

                    let meta = PY_TYPE_TYPE.with(|tp| Rc::clone(tp));
                    let cls = meta.pytype_tp_call().unwrap()(Rc::clone(&meta), &vec![nameobj, bases, dictobj]);

                    if cls.is_ok() {
                        self.push(cls.expect("Never fails"));
                        self.pc += 1;
                        continue;
                    }
                    why = Why::WhyException;
                    retval = Err(());
                },
            }

            while self.blocks.len() > 0 {

                {
                    let block = self.blocks.last().unwrap();

                    if block.b_type == BlockType::LoopBlock && why == Why::WhyContinue {
                        why = Why::WhyNot;
                        let ret = Rc::clone(retval.as_ref().expect("Continue ret addr"));
                        self.pc = pyobj_to_i32(ret).expect("Never fails") as usize;
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

pub fn eval(code: &Code, env: Rc<Env>) -> PyRes<Rc<PyObject>> {
    let mut stack_machine = StackMachine::new();
    stack_machine.exec(code, env)
}
