use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};
use std::hash::{Hash, Hasher};

pub enum StackObject {
    Integer(i64),
    Boolean(bool),
    HashMap(Rc<RefCell<ObjectMap>>),
    String(Rc<RefCell<String>>),
    Vector(Rc<RefCell<Vec<StackObject>>>)
}

pub type ObjectMap = HashMap<StackObject, StackObject>;
pub type ObjectVec = Vec<StackObject>;

impl StackObject {
    pub fn make_from<T: Into<StackObject>>(object:T) -> StackObject {
        object.into()
    }

    pub fn make<T: Into<StackObject> + Default>() -> StackObject {
        T::default().into()
    }

    pub fn can_hash(&self) -> bool {
        use StackObject::*;
        match self {
            Integer(..) | Boolean(..) | String(..) => {true}
            _ => false
        }
    }

    pub fn as_map_rc(&mut self) -> Option<Rc<RefCell<ObjectMap>>> {
        use StackObject::*;
        match self {
            HashMap(ptr) => {
                Some(ptr.clone())
            }
            _ => None
        }
    }

    pub fn as_vector_rc(&mut self) -> Option<Rc<RefCell<ObjectVec>>> {
        use StackObject::*;
        match self {
            Vector(ptr) => {
                Some(ptr.clone())
            }
            _ => None
        }
    }

    pub fn as_string_rc(&mut self) -> Option<Rc<RefCell<String>>> {
        use StackObject::*;
        match self {
            String(ptr) => {
                Some(ptr.clone())
            }
            _ => None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        use StackObject::*;
        match self {
            Integer(x) => Some(*x),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        use StackObject::*;
        match self {
            Boolean(x) => Some(*x),
            _ => None
        }
    }

    pub fn ref_clone(&self) -> StackObject {
        //somewhere between clone and copy:
        //does not perform deep copy, clones pointer types
        match self {
            StackObject::Integer(x) => {(*x).into()}
            StackObject::Boolean(x) => {(*x).into()}
            StackObject::HashMap(ptr) => {
                StackObject::HashMap(ptr.clone())
            }
            StackObject::String(ptr) => {
                StackObject::String(ptr.clone())
            }
            StackObject::Vector(ptr) => {
                StackObject::Vector(ptr.clone())
            }
        }
    }
}

impl From<i64> for StackObject {
    fn from(x: i64) -> Self {
        StackObject::Integer(x)
    }
}

impl From<bool> for StackObject {
    fn from(x: bool) -> Self {
        StackObject::Boolean(x)
    }
}

impl From<ObjectMap> for StackObject {
    fn from(obj: ObjectMap) -> Self {
        StackObject::HashMap(Rc::new(RefCell::new(obj)))
    }
}

impl From<ObjectVec> for StackObject {
    fn from(obj: ObjectVec) -> Self {
        StackObject::Vector(Rc::new(RefCell::new(obj)))
    }
}

impl From<String> for StackObject {
    fn from(obj: String) -> Self {
        StackObject::String(Rc::new(RefCell::new(obj)))
    }
}

impl Clone for StackObject {
    // clone performs deep clone
    fn clone(&self) -> Self {
        match self {
            StackObject::Integer(x) => {(*x).into()}
            StackObject::Boolean(x) => {(*x).into()}

            StackObject::HashMap(ptr) => {
                ptr.borrow().clone().into()
            }
            StackObject::String(ptr) => {
                ptr.borrow().clone().into()
            }
            StackObject::Vector(ptr) => {
                ptr.borrow().clone().into()
            }
        }
    }
}

impl PartialEq for StackObject {
    //right equality (deep equality for complex objects, content equality for strings)
    fn eq(&self, other: &Self) -> bool {
        use StackObject::*;
        //types must match
        std::mem::discriminant(self)==std::mem::discriminant(other) &&

            match (self, other) {
                (Integer(a), Integer(b)) => {a==b}
                (Boolean(a), Boolean(b)) => {a==b}
                (HashMap(ptr_1), HashMap(ptr_2))
                => {
                    Rc::ptr_eq(ptr_1, ptr_2) //memory location equality
                        || ptr_1==ptr_2
                }
                (String(ptr_1), String(ptr_2))
                => {
                    Rc::ptr_eq(ptr_1, ptr_2)
                        || ptr_1==ptr_2
                }
                (Vector(ptr_1), Vector(ptr_2))
                => {
                    Rc::ptr_eq(ptr_1, ptr_2)
                        || ptr_1==ptr_2
                }
                _ => panic!() // should never happen - discriminants are equal
            }
    }
}

impl Eq for StackObject {}

impl Hash for StackObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use StackObject::*;
        match self {
            Integer(x) => {x.hash(state)}
            Boolean(x) => {x.hash(state)}
            String(ptr) => {
                ptr.borrow().hash(state)
            }

            HashMap(_) => {panic!("hash of unhashable object (HashMap)")}

            Vector(_) => {panic!("hash of unhashable object (Vector)")}
        }
    }
}