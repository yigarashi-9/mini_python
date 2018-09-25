use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use object::*;
use object::generic::*;

pub type HashFunc = Option<Rc<dyn Fn(Rc<PyObject>) -> u64>>;
pub type UnaryOp = Option<Rc<dyn Fn(Rc<PyObject>) -> Rc<PyObject>>>;
pub type BinaryOp = Option<Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>>>;

pub struct PyTypeObject {
    pub ob_type: Option<Rc<RefCell<PyTypeObject>>>,
    pub tp_name: String,
    pub tp_base: Option<Rc<RefCell<PyTypeObject>>>,
    pub tp_hash: HashFunc,
    pub tp_bool: UnaryOp,
    pub tp_fun_eq: BinaryOp,
    pub tp_fun_add: BinaryOp,
    pub tp_fun_lt: BinaryOp,
    pub tp_len: UnaryOp,
    pub tp_dict: Option<Rc<PyObject>>,
    pub tp_bases: Option<Rc<PyObject>>,
    pub tp_mro: Option<Rc<PyObject>>,
    pub tp_subclasses: Option<Rc<PyObject>>,
}

pub fn default_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    (&*obj as *const PyObject).hash(&mut hasher);
    hasher.finish()
}

thread_local! (
    pub static PY_TYPE_TYPE: Rc<RefCell<PyTypeObject>>
        = Rc::new(RefCell::new(PyTypeObject::new_type()))
);

impl PyTypeObject {
    pub fn new_type() -> PyTypeObject {
        PyTypeObject {
            ob_type: None,
            tp_name: "type".to_string(),
            tp_base: None,
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        }
    }

    pub fn tp_dict_ref(&self) -> &Option<Rc<PyObject>> {
        &self.tp_dict
    }
}

fn pick_winner(mro_list: &Vec<Vec<Rc<PyObject>>>) -> Rc<PyObject> {
    for mro in mro_list {
        let cand = &mro[0];

        let mut win = true;
        for others in mro_list {
            let (_, tail) = others.split_at(1);
            if tail.contains(cand) {
                win = false;
                break;
            }
        }

        if win { return Rc::clone(cand) };
    }
    panic!("pick_candidate: No candidate")
}

fn remove_winner(winner: Rc<PyObject>, mro_list: Vec<Vec<Rc<PyObject>>>) -> Vec<Vec<Rc<PyObject>>> {
    let mut new_list = vec![];
    for mro in mro_list {
        let mut new_mro = vec![];
        for class in mro {
            if &*winner as *const _ != &*class as *const _ { new_mro.push(Rc::clone(&class)); }
        }
        if new_mro.len() > 0 { new_list.push(new_mro); }
    };
    new_list
}

fn linearlize(arg: Vec<Vec<Rc<PyObject>>>) -> Vec<Rc<PyObject>> {
    let mut mro_list = arg;
    let mut mro = vec![];
    loop {
        if mro_list.len() == 0 {
            break;
        }
        let winner = pick_winner(&mro_list);
        mro.push(Rc::clone(&winner));
        mro_list = remove_winner(winner, mro_list);
    };
    mro
}

fn unaryop_from_pyobj(obj: Rc<PyObject>) ->
    Rc<dyn Fn(Rc<PyObject>) -> Rc<PyObject>> {
        Rc::new(move |x| call_func(Rc::clone(&obj), &mut vec![x]))
    }

fn get_wrapped_unaryop(dict: Rc<PyObject>, s: &str) ->
    Option<Rc<dyn Fn(Rc<PyObject>) -> Rc<PyObject>>> {
        dict.lookup(&PyObject::from_str(s)).map(unaryop_from_pyobj)
    }

fn binop_from_pyobj(obj: Rc<PyObject>) ->
    Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>> {
        Rc::new(move |x, y| call_func(Rc::clone(&obj), &mut vec![x, y]))
    }

fn get_wrapped_binop(dict: Rc<PyObject>, s: &str) ->
    Option<Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>>> {
        dict.lookup(&PyObject::from_str(s)).map(binop_from_pyobj)
    }

fn inherit_method(typ: &mut PyTypeObject, base: &PyTypeObject) {
    if typ.tp_hash.is_none() && base.tp_hash.is_some() {
        typ.tp_hash = base.tp_hash.clone();
    }

    if typ.tp_bool.is_none() && base.tp_bool.is_some() {
        typ.tp_bool = base.tp_bool.clone();
    }

    if typ.tp_fun_eq.is_none() && base.tp_fun_eq.is_some() {
        typ.tp_fun_eq = base.tp_fun_eq.clone();
    }

    if typ.tp_fun_add.is_none() && base.tp_fun_add.is_some() {
        typ.tp_fun_add = base.tp_fun_add.clone();
    }

    if typ.tp_fun_lt.is_none() && base.tp_fun_lt.is_some() {
        typ.tp_fun_lt = base.tp_fun_lt.clone();
    }
}

pub fn pytype_ready(obj: Rc<PyObject>) {
    let obj_cloned = Rc::clone(&obj);
    match obj_cloned.inner {
        PyInnerObject::TypeObj(ref typ) => {
            let mut mro: Vec<Rc<PyObject>> = vec![];

            if typ.borrow().tp_bases.is_some() {
                let mut mro_list = vec![];
                match typ.borrow().tp_bases.as_ref().unwrap().inner {
                    PyInnerObject::ListObj(ref obj) => {
                        for base in obj.list.borrow().iter() {
                            let pylist = get_attr(&base, &"__mro__".to_string()).unwrap();
                            match pylist.inner {
                                PyInnerObject::ListObj(ref obj) => {
                                    mro_list.push(obj.list.borrow().clone());
                                },
                                _ => panic!("pytype_ready")
                            }
                        }
                    },
                    _ => panic!("Type Error: pytype_ready tp_bases"),
                }
                mro = linearlize(mro_list);
                mro.insert(0, Rc::clone(&obj));
                update_attr(&obj, "__mro__".to_string(), PyObject::from_vec(&mro));
            }

            if typ.borrow().tp_dict.is_some() {
                let mut typ = typ.borrow_mut();
                let dictobj = Rc::clone(typ.tp_dict.as_ref().unwrap());
                typ.tp_bool = get_wrapped_unaryop(Rc::clone(&dictobj), "__bool__");
                typ.tp_fun_add = get_wrapped_binop(Rc::clone(&dictobj), "__add__");
                typ.tp_fun_eq = get_wrapped_binop(Rc::clone(&dictobj), "__eq__");
                typ.tp_fun_lt = get_wrapped_binop(Rc::clone(&dictobj), "__lt__");
                typ.tp_len = get_wrapped_unaryop(Rc::clone(&dictobj), "__len__");
            }

            if typ.borrow().tp_base.is_some() {
                let mut typ = typ.borrow_mut();
                let tp_base = Rc::clone(typ.tp_base.as_ref().unwrap());
                inherit_method(&mut typ, &tp_base.borrow());
            }

            if mro.len() >= 1 {
                for base in &mro[1..] {
                    let mut typ = typ.borrow_mut();
                    match base.inner {
                        PyInnerObject::TypeObj(ref obj) => {
                            inherit_method(&mut typ, &obj.borrow())
                        },
                        _ => panic!("Type Error: pytype_ready")
                    }
                }
            }
        },
        _ => panic!("Type Error: pytype_ready"),
    }
}
