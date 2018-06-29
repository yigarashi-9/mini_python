use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::rc::Rc;

use env::Env;
use syntax::{Id, Program};

pub enum PyObject {
    LongObj(Rc<PyLongObject>),
    BoolObj(Rc<PyBoolObject>),
    StrObj(Rc<PyStringObject>),
    NoneObj(Rc<PyNoneObject>),
    FunObj(Rc<PyFuncObject>),
    InstObj(Rc<PyInstObject>),
    MethodObj(Rc<PyMethodObject>),
    DictObj(Rc<PyDictObject>),
    TypeObj(Rc<PyTypeObject>),
}

impl PyObject {
    pub fn ob_type(&self) -> Rc<PyTypeObject> {
        match self {
            &PyObject::LongObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::BoolObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::StrObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::NoneObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::FunObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::InstObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::MethodObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::DictObj(ref obj) => Rc::clone(&obj.ob_type),
            &PyObject::TypeObj(ref obj) =>
                match obj.ob_type {
                    Some(ref ob_type) => Rc::clone(ob_type),
                    None => Rc::clone(obj),
                },
        }
    }

    pub fn from_i32(raw_i32: i32) -> PyObject {
        PyObject::LongObj(Rc::new(PyLongObject::from_i32(raw_i32)))
    }

    pub fn from_bool(raw_bool: bool) -> PyObject {
        PyObject::BoolObj(Rc::new(PyBoolObject::from_bool(raw_bool)))
    }

    pub fn from_string(raw_string: String) -> PyObject {
        PyObject::StrObj(Rc::new(PyStringObject::from_string(raw_string)))
    }

    pub fn none_obj() -> PyObject {
        PyObject::NoneObj(Rc::new(PyNoneObject { ob_type: Rc::new(PyTypeObject::new_none()) }))
    }

    pub fn from_hashmap(raw_hashmap: HashMap<Rc<PyObject>, Rc<PyObject>>) -> PyObject {
        PyObject::DictObj(Rc::new(PyDictObject::from_hashmap(raw_hashmap)))
    }

    pub fn lookup(&self, key: &Rc<PyObject>) -> Option<Rc<PyObject>> {
        match self {
            &PyObject::DictObj(ref obj) => obj.lookup(key),
            _ => panic!("Type Error: PyObject lookup")
        }
    }

    pub fn update(&self, key: Rc<PyObject>, value: Rc<PyObject>) {
        match self {
            &PyObject::DictObj(ref obj) => obj.update(key, value),
            _ => panic!("Type Error: PyObject update")
        }
    }
}

impl PartialEq for PyObject {
    fn eq(&self, other: &PyObject) -> bool {
        self.ob_type().tp_fun_eq.expect("Type Error: No __eq__")(self, other).to_bool()
    }
}

impl Eq for PyObject {}

impl Hash for PyObject {
    fn hash<H>(&self, hasher: &mut H) where H: Hasher {
        hasher.write_u64(self.ob_type().tp_hash.expect("Type Error: Unhashbable")(self))
    }
}

impl PyObject {
    pub fn to_bool(&self) -> bool {
        match self {
            PyObject::LongObj(ref obj) => obj.n != 0,
            PyObject::BoolObj(ref obj) => obj.b,
            PyObject::NoneObj(ref obj) => false,
            _ => true,
        }
    }
}

pub struct PyLongObject {
    pub ob_type: Rc<PyTypeObject>,
    n: i32,
}

impl PyLongObject {
    pub fn from_i32(raw_i32: i32) -> PyLongObject {
        PyLongObject {
            ob_type: Rc::new(PyTypeObject::new_int()),
            n: raw_i32,
        }
    }
}

pub struct PyBoolObject {
    pub ob_type: Rc<PyTypeObject>,
    b: bool,
}

impl PyBoolObject {
    pub fn from_bool(raw_bool: bool) -> PyBoolObject {
        PyBoolObject {
            ob_type: Rc::new(PyTypeObject::new_int()),
            b: raw_bool,
        }
    }
}

pub struct PyStringObject {
    pub ob_type: Rc<PyTypeObject>,
    s: String,
}

impl PyStringObject {
    pub fn from_string(raw_string: String) -> PyStringObject {
        PyStringObject {
            ob_type: Rc::new(PyTypeObject::new_str()),
            s: raw_string
        }
    }
}

pub struct PyNoneObject {
    pub ob_type: Rc<PyTypeObject>,
}

pub struct PyFuncObject {
    pub ob_type: Rc<PyTypeObject>,
    pub env: Rc<Env>,
    pub parms: Vec<Id>,
    pub code: Program,
}

pub struct PyInstObject {
    pub ob_type: Rc<PyTypeObject>,
    pub class: Rc<PyTypeObject>,
    pub dict: Rc<PyDictObject>,
}

pub struct PyMethodObject {
    pub ob_type: Rc<PyTypeObject>,
    pub ob_self: Rc<PyObject>,
    pub env: Rc<Env>,
    pub parms: Vec<Id>,
    pub code: Program,
}

pub struct PyDictObject {
    pub ob_type: Rc<PyTypeObject>,
    pub dict: RefCell<HashMap<Rc<PyObject>, Rc<PyObject>>>
}

impl PyDictObject {
    pub fn new() -> PyDictObject {
        PyDictObject::from_hashmap(HashMap::new())
    }

    pub fn from_hashmap(raw_hashmap: HashMap<Rc<PyObject>, Rc<PyObject>>) -> PyDictObject {
        PyDictObject {
            ob_type: Rc::new(PyTypeObject::new_dict()),
            dict: RefCell::new(raw_hashmap),
        }
    }

    pub fn lookup(&self, key: &Rc<PyObject>) -> Option<Rc<PyObject>> {
        match self.dict.borrow().get(key) {
            Some(v) => Some(Rc::clone(v)),
            None => None
        }
    }

    pub fn update(&self, key: Rc<PyObject>, value: Rc<PyObject>) {
        self.dict.borrow_mut().insert(key, value);
    }
}

pub type PyBinaryOp = fn(&PyObject, &PyObject) -> Rc<PyObject>;
pub type PyHashFunc = fn(&PyObject) -> u64;

pub struct PyTypeObject {
    pub ob_type: Option<Rc<PyTypeObject>>,
    pub tp_name: String,
    pub tp_hash: Option<PyHashFunc>,
    pub tp_fun_eq: Option<PyBinaryOp>,
    pub tp_fun_add: Option<PyBinaryOp>,
    pub tp_fun_lt: Option<PyBinaryOp>,
    pub tp_dict: Option<Rc<PyDictObject>>,
}

fn eq_long_long(lv: &PyObject, rv: &PyObject) -> Rc<PyObject> {
    match lv {
        &PyObject::LongObj(ref l_obj) => {
            match rv {
                &PyObject::LongObj(ref r_obj) =>
                    Rc::new(PyObject::from_bool(l_obj.n == r_obj.n)),
                _ => panic!("Type Error: eq_long_long"),
            }
        },
        _ => panic!("Type Error: eq_long_long"),
    }
}

fn eq_str_str(lv: &PyObject, rv: &PyObject) -> Rc<PyObject> {
    match lv {
        &PyObject::StrObj(ref l_obj) => {
            match rv {
                &PyObject::StrObj(ref r_obj) =>
                    Rc::new(PyObject::from_bool(l_obj.s == r_obj.s)),
                _ => panic!("Type Error: eq_str_str"),
            }
        },
        _ => panic!("Type Error: eq_str_str"),
    }
}

fn add_long_long(lv: &PyObject, rv: &PyObject) -> Rc<PyObject> {
    match lv {
        &PyObject::LongObj(ref l_obj) => {
            match rv {
                &PyObject::LongObj(ref r_obj) => Rc::new(PyObject::from_i32(l_obj.n + r_obj.n)),
                _ => panic!("Type Error: add_long_long"),
            }
        },
        _ => panic!("Type Error: add_long_long"),
    }
}

fn lt_long_long(lv: &PyObject, rv: &PyObject) -> Rc<PyObject> {
    match lv {
        &PyObject::LongObj(ref l_obj) => {
            match rv {
                &PyObject::LongObj(ref r_obj) => Rc::new(PyObject::from_bool(l_obj.n < r_obj.n)),
                _ => panic!("Type Error: lt_long_long"),
            }
        },
        _ => panic!("Type Error: lt_long_long"),
    }
}

fn default_hash(obj: &PyObject) -> u64 {
    let mut hasher = DefaultHasher::new();
    (obj as *const PyObject).hash(&mut hasher);
    hasher.finish()
}

fn int_hash(obj: &PyObject) -> u64 {
    let mut hasher = DefaultHasher::new();
    match obj {
        &PyObject::LongObj(ref obj) => obj.n.hash(&mut hasher),
        _ => panic!("Type Error: int_hash")
    };
    hasher.finish()
}

fn bool_hash(obj: &PyObject) -> u64 {
    let mut hasher = DefaultHasher::new();
    match obj {
        &PyObject::BoolObj(ref obj) => obj.b.hash(&mut hasher),
        _ => panic!("Type Error: bool_hash")
    };
    hasher.finish()
}

fn str_hash(obj: &PyObject) -> u64 {
    let mut hasher = DefaultHasher::new();
    match obj {
        &PyObject::StrObj(ref obj) => obj.s.hash(&mut hasher),
        _ => panic!("Type Error: str_hash")
    };
    hasher.finish()
}

impl PyTypeObject {
    pub fn new_type() -> PyTypeObject {
        PyTypeObject {
            ob_type: None,
            tp_name: "type".to_string(),
            tp_hash: Some(default_hash),
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_dict: None,
        }
    }

    pub fn new_int() -> PyTypeObject {
        PyTypeObject {
            ob_type: Some(Rc::new(PyTypeObject::new_type())),
            tp_name: "int".to_string(),
            tp_hash: Some(int_hash),
            tp_fun_eq: Some(eq_long_long),
            tp_fun_add: Some(add_long_long),
            tp_fun_lt: Some(lt_long_long),
            tp_dict: None,
        }
    }

    pub fn new_str() -> PyTypeObject {
        PyTypeObject {
            ob_type: Some(Rc::new(PyTypeObject::new_type())),
            tp_name: "str".to_string(),
            tp_hash: Some(str_hash),
            tp_fun_eq: Some(eq_str_str),
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_dict: None,
        }
    }

    pub fn new_none() -> PyTypeObject {
        PyTypeObject {
            ob_type: Some(Rc::new(PyTypeObject::new_type())),
            tp_name: "None".to_string(),
            tp_hash: Some(default_hash),
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_dict: None,
        }
    }

    pub fn new_function() -> PyTypeObject {
        PyTypeObject {
            ob_type: Some(Rc::new(PyTypeObject::new_type())),
            tp_name: "function".to_string(),
            tp_hash: Some(default_hash),
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_dict: None,
        }
    }

    pub fn new_method() -> PyTypeObject {
        PyTypeObject {
            ob_type: Some(Rc::new(PyTypeObject::new_type())),
            tp_name: "method".to_string(),
            tp_hash: Some(default_hash),
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_dict: None,
        }
    }

    pub fn new_dict() -> PyTypeObject {
        PyTypeObject {
            ob_type: Some(Rc::new(PyTypeObject::new_type())),
            tp_name: "dict".to_string(),
            tp_hash: None,
            tp_fun_eq: None,
            tp_fun_add: None,
            tp_fun_lt: None,
            tp_dict: None,
        }
    }

    pub fn tp_dict_ref(&self) -> &Option<Rc<PyDictObject>> {
        &self.tp_dict
    }
}
