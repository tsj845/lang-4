use std::collections::HashMap;
use std::fmt;
use crate::data::{hash, ArgsObj, Token, Primitive, ClassInstObj, PropsObj};

pub struct ClassObj {
    oid: usize,
    myname: String,
    iprops: PropsObj,
    current_inst_id: usize,
}

impl ClassObj {
    pub fn new(oid: usize, name: &str) -> ClassObj {
        ClassObj {
            oid: oid,
            myname: name.to_owned(),
            iprops: PropsObj {},
            current_inst_id: 0,
        }
    }
    pub fn create(&mut self, cargs: ArgsObj) -> ClassInstObj {
        let x = ClassInstObj::new(self.oid, self.current_inst_id, self.iprops.clone());
        self.current_inst_id += 1;
        return x;
    }
}

impl fmt::Debug for ClassObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClassObj").field("class_name", &self.myname).field("class_id", &self.oid).field("current_instance_id", &self.current_inst_id).finish()
    }
}

pub struct Storage {
    class_obj_store: HashMap<u64, ClassObj>,
    class_inst_store: HashMap<u64, ClassInstObj>,
    current_class_obj_id: usize,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            class_obj_store: HashMap::new(),
            class_inst_store: HashMap::new(),
            current_class_obj_id: 0,
        }
    }
    pub fn add_class_obj(&mut self, cname: &str) -> () {
        self.class_obj_store.insert(hash(&cname), ClassObj::new(0, cname));
    }
    pub fn class_static_op(&mut self, cid: u64, opid: u64, opargs: ArgsObj) -> Token {
        return Token::Dat(Primitive::Null);
    }
}