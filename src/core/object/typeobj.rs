use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use syntax::{Id};
use object::*;
use object::generic::*;

pub type HashFun = dyn Fn(Rc<PyObject>) -> u64;
pub type UnaryOp = dyn Fn(Rc<PyObject>) -> Rc<PyObject>;
pub type BinaryOp = dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>;
pub type VarArgFun = dyn Fn(Rc<PyObject>, &Vec<Rc<PyObject>>) -> Rc<PyObject>;
pub type GetAttroFun = dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Option<Rc<PyObject>>;
pub type SetAttroFun = dyn Fn(Rc<PyObject>, Rc<PyObject>, Rc<PyObject>) -> ();

pub struct PyTypeObject {
    pub tp_name: String,
    pub tp_base: Option<Rc<PyObject>>,
    pub tp_hash: Option<Rc<HashFun>>,
    pub tp_bool: Option<Rc<UnaryOp>>,
    pub tp_fun_eq: Option<Rc<BinaryOp>>,
    pub tp_fun_add: Option<Rc<BinaryOp>>,
    pub tp_fun_lt: Option<Rc<BinaryOp>>,
    pub tp_len: Option<Rc<UnaryOp>>,
    pub tp_call: Option<Rc<VarArgFun>>,
    pub tp_getattro: Option<Rc<GetAttroFun>>,
    pub tp_setattro: Option<Rc<SetAttroFun>>,
    pub tp_methods: Option<Vec<Rc<PyObject>>>,
    pub tp_dict: Option<Rc<PyObject>>,
    pub tp_bases: Option<Rc<PyObject>>,
    pub tp_mro: Option<Rc<PyObject>>,
    pub tp_subclasses: Option<Rc<PyObject>>,
}

thread_local! (
    pub static PY_TYPE_TYPE: Rc<PyObject> = {
        let tp = PyTypeObject {
            tp_name: "type".to_string(),
            tp_base: None,
            tp_hash: Some(Rc::new(default_hash)),
            tp_bool: None,
            tp_fun_eq: Some(Rc::new(type_eq)),
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_call: Some(Rc::new(type_call)),
            tp_getattro: Some(Rc::new(type_getattro)),
            tp_setattro: Some(Rc::new(type_setattro)),
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: None,
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(tp)))
        })
    }
);

fn type_eq(slf: Rc<PyObject>, other: Rc<PyObject>) -> Rc<PyObject> {
    PyObject::from_bool(slf == other)
}

impl PartialEq for PyTypeObject {
    fn eq(&self, other: &PyTypeObject) -> bool {
        self as *const _ == other as *const _
    }
}

impl PyObject {
    pub fn pytype_new() -> Rc<PyObject> {
        let tp = PyTypeObject {
            tp_name: "".to_string(),
            tp_base: None,
            tp_hash: None,
            tp_bool: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_len: None,
            tp_call: None,
            tp_getattro: None,
            tp_setattro: None,
            tp_methods: None,
            tp_dict: None,
            tp_bases: None,
            tp_mro: None,
            tp_subclasses: None,
        };
        Rc::new(PyObject {
            ob_type: PY_TYPE_TYPE.with(|tp| { Some(Rc::clone(tp)) }),
            ob_dict: None,
            inner: PyInnerObject::TypeObj(Rc::new(RefCell::new(tp)))
        })
    }

    pub fn pytype_typeobj_borrow(&self) -> Ref<PyTypeObject> {
        match self.inner {
            PyInnerObject::TypeObj(ref typ) => typ.borrow(),
            _ => panic!("Type Error: pytype_inner")
        }
    }

    pub fn pytype_typeobj_borrow_mut(&self) -> RefMut<PyTypeObject> {
        match self.inner {
            PyInnerObject::TypeObj(ref typ) => typ.borrow_mut(),
            _ => panic!("Type Error: pytype_inner")
        }
    }

    pub fn pytype_tp_dict(&self) -> Option<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::TypeObj(ref typ) => typ.borrow().tp_dict.clone(),
            _ => panic!("Type Error: pytype_tp_dict")
        }
    }

    pub fn pytype_tp_base(&self) -> Option<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::TypeObj(ref typ) => typ.borrow().tp_base.clone(),
            _ => panic!("Type Error: pytype_tp_base")
        }
    }

    pub fn pytype_tp_bases(&self) -> Option<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::TypeObj(ref typ) => typ.borrow().tp_bases.clone(),
            _ => panic!("Type Error: pytype_tp_bases")
        }
    }

    pub fn pytype_tp_mro(&self) -> Option<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::TypeObj(ref typ) => typ.borrow().tp_mro.clone(),
            _ => panic!("Type Error: pytype_tp_mro")
        }
    }

    pub fn pytype_tp_subclasses(&self) -> Option<Rc<PyObject>> {
        match self.inner {
            PyInnerObject::TypeObj(ref typ) => typ.borrow().tp_subclasses.clone(),
            _ => panic!("Type Error: pytype_tp_subclasses")
        }
    }
}

pub fn default_hash(obj: Rc<PyObject>) -> u64 {
    let mut hasher = DefaultHasher::new();
    (&*obj as *const PyObject).hash(&mut hasher);
    hasher.finish()
}

impl PyTypeObject {
    pub fn tp_dict_ref(&self) -> &Option<Rc<PyObject>> {
        &self.tp_dict
    }
}

fn type_call(typ: Rc<PyObject>, args: &Vec<Rc<PyObject>>) -> Rc<PyObject> {

    if PY_TYPE_TYPE.with(|tp| { tp == &typ }) {
        if args.len() == 1 {
            return Rc::clone(&args[0].ob_type())
        } else {
            panic!("Type Error: type_call")
        }
    }

    let dictobj = PyObject::pydict_new();
    let instance = Rc::new(PyObject {
        ob_type: Some(Rc::clone(&typ)),
        ob_dict: Some(Rc::clone(&dictobj)),
        inner: PyInnerObject::InstObj,
    });
    match pyobj_get_attro(Rc::clone(&instance), PyObject::from_str("__init__")) {
        Some(init_fun) => call_func(Rc::clone(&init_fun), args),
        None => PyObject::none_obj()
    };
    instance
}

pub fn type_getattro(value: Rc<PyObject>, key: Rc<PyObject>) -> Option<Rc<PyObject>> {
    match value.inner {
        PyInnerObject::TypeObj(ref typ) => {
            if let Some(ref mro) = typ.borrow().tp_mro {
                if !mro.pylist_check() {
                    panic!("Type Error: type_getattro mro");
                }

                let mut ret_val = None;
                for i in 0..(mro.pylist_size()) {
                    let tmp = mro.pylist_getitem(i).pytype_tp_dict().as_ref().unwrap().pydict_lookup(Rc::clone(&key));
                    if tmp.is_some() {
                        ret_val = tmp;
                        break;
                    }
                };
                ret_val
            } else {
                println!("Error");
                match value.pytype_tp_dict() {
                    Some(ref tp_dict) => tp_dict.pydict_lookup(Rc::clone(&key)),
                    None => None,
                }
            }
        },
        _ => panic!("Type Error: type_getattro")
    }
}

pub fn type_setattro(value: Rc<PyObject>, key: Rc<PyObject>, rvalue: Rc<PyObject>) {
    let tp_dict = value.pytype_tp_dict().expect("No tp_dict");
    tp_dict.pydict_update(Rc::clone(&key), Rc::clone(&rvalue));
    update_slot(Rc::clone(&value), pyobj_to_string(Rc::clone(&key)), Rc::clone(&rvalue));
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
        dict.pydict_lookup(PyObject::from_str(s)).map(unaryop_from_pyobj)
    }

fn binop_from_pyobj(obj: Rc<PyObject>) ->
    Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>> {
        Rc::new(move |x, y| call_func(Rc::clone(&obj), &mut vec![x, y]))
    }

fn get_wrapped_binop(dict: Rc<PyObject>, s: &str) ->
    Option<Rc<dyn Fn(Rc<PyObject>, Rc<PyObject>) -> Rc<PyObject>>> {
        dict.pydict_lookup(PyObject::from_str(s)).map(binop_from_pyobj)
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

fn update_slot_subclasses(value: Rc<PyObject>, key: Id, rvalue: Rc<PyObject>) {
    if let Some(ref subclasses) = value.pytype_tp_subclasses() {
        if !subclasses.pylist_check() { panic!("Type Error: updat_slot_subclasses") }
        for i in 0..(subclasses.pylist_size()) {
            let subclass = subclasses.pylist_getitem(i);
            let keyval = PyObject::from_string(key.clone());
            if subclass.pytype_tp_dict().as_ref().unwrap().pydict_lookup(keyval).is_none() {
                update_slot(Rc::clone(&subclass), key.clone(), Rc::clone(&rvalue));
            }
        }
    }
}

pub fn update_slot(value: Rc<PyObject>, key: Id, rvalue: Rc<PyObject>) {
    {
        let mut  typ = value.pytype_typeobj_borrow_mut();
        if key == "__add__".to_string() {
            typ.tp_fun_add = Some(binop_from_pyobj(Rc::clone(&rvalue)));
        } else if key == "__bool__".to_string() {
            typ.tp_bool = Some(unaryop_from_pyobj(Rc::clone(&rvalue)));
        } else if key == "__lt__".to_string() {
            typ.tp_fun_lt = Some(binop_from_pyobj(Rc::clone(&rvalue)));
        } else if key == "__eq__".to_string() {
            typ.tp_fun_eq = Some(binop_from_pyobj(Rc::clone(&rvalue)));
        }
    }
    update_slot_subclasses(Rc::clone(&value), key.clone(), Rc::clone(&rvalue));
}

pub fn pytype_ready(obj: Rc<PyObject>) {
    if obj.pytype_tp_dict().is_none() {
        let mut typ = obj.pytype_typeobj_borrow_mut();
        let dictobj = PyObject::pydict_new();

        if let Some(ref tp_methods) = typ.tp_methods {
            for meth in tp_methods {
                dictobj.pydict_update(PyObject::from_string(Rc::clone(meth).pyrustfun_name()), Rc::clone(&meth))
            }
        }
        typ.tp_dict = Some(dictobj);
    }

    let mut mro: Vec<Rc<PyObject>> = vec![];
    if let Some(ref bases) = obj.pytype_tp_bases() {
        let mut mro_list = vec![];
        if !(bases.pylist_check()) { panic!("Type Error: pytype_ready") }
        for i in 0..(bases.pylist_size()) {
            if let Some(mro) = bases.pylist_getitem(i).pytype_tp_mro() {
                mro_list.push(mro.pylist_clone());
            }
        }
        mro = linearlize(mro_list);
        mro.insert(0, Rc::clone(&obj));
    } else {
        mro.push(Rc::clone(&obj));
    }

    let mro_obj = PyObject::pylist_from_vec(&mro);
    obj.pytype_typeobj_borrow_mut().tp_mro = Some(Rc::clone(&mro_obj));
    pyobj_set_attro(Rc::clone(&obj), PyObject::from_str("__mro__"), mro_obj);

    if let Some(ref dictobj) = obj.pytype_tp_dict() {
        let mut typ = obj.pytype_typeobj_borrow_mut();
        if let Some(fun) = get_wrapped_unaryop(Rc::clone(&dictobj), "__bool__") {
            typ.tp_bool = Some(fun);
        }
        if let Some(fun) = get_wrapped_binop(Rc::clone(&dictobj), "__add__") {
            typ.tp_fun_add = Some(fun);
        }
        if let Some(fun) = get_wrapped_binop(Rc::clone(&dictobj), "__eq__") {
            typ.tp_fun_eq = Some(fun);
        }
        if let Some(fun) = get_wrapped_binop(Rc::clone(&dictobj), "__lt__") {
            typ.tp_fun_lt = Some(fun);
        }
        if let Some(fun) = get_wrapped_unaryop(Rc::clone(&dictobj), "__len__") {
            typ.tp_len = Some(fun);
        }
    }

    if let Some(ref base) = obj.pytype_tp_base() {
        inherit_method(&mut obj.pytype_typeobj_borrow_mut(), &base.pytype_typeobj_borrow());
    }

    if mro.len() >= 1 {
        for base in &mro[1..] {
            inherit_method(&mut obj.pytype_typeobj_borrow_mut(),
                           &base.pytype_typeobj_borrow())
        }
    }

    if obj.pytype_typeobj_borrow().tp_getattro.is_none() {
        let mut typ = obj.pytype_typeobj_borrow_mut();
        typ.tp_getattro = Some(Rc::new(pyobj_generic_get_attro));
    }

    if obj.pytype_typeobj_borrow().tp_setattro.is_none() {
        let mut typ = obj.pytype_typeobj_borrow_mut();
        typ.tp_setattro = Some(Rc::new(pyobj_generic_set_attro));
    }
}
