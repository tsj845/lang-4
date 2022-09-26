use std::{collections::HashMap};
use std::fmt;
use crate::data::{hash, Token, ClassInstObj, PropsObj, FuncObj};

pub type VarScope = HashMap<u64, Token>;

pub struct ClassObj {
    oid: u64,
    myname: String,
    iprops: PropsObj,
    current_inst_id: u64,
    pub stat_funcs: HashMap<u64, FuncObj>,
    pub inst_funcs: HashMap<u64, FuncObj>,
}

impl ClassObj {
    pub fn new(oid: u64, name: &str, statics: Vec<FuncObj>, insts: Vec<FuncObj>) -> ClassObj {
        let mut st: HashMap<u64, FuncObj> = HashMap::new();
        let mut it: HashMap<u64, FuncObj> = HashMap::new();
        for f in statics {
            st.insert(hash(&f.name), f);
        }
        for f in insts {
            it.insert(hash(&f.name), f);
        }
        ClassObj {
            oid: oid,
            myname: name.to_owned(),
            iprops: PropsObj {},
            current_inst_id: 0,
            stat_funcs: st,
            inst_funcs: it,
        }
    }
    pub fn create(&mut self) -> ClassInstObj {
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
    class_inst_store: HashMap<(u64, u64), ClassInstObj>,
    var_scopes: Vec<VarScope>,
    var_tracks: Vec<Vec<(u64, u64)>>,
    scope_count: usize,
    current_class_obj_id: u64,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            class_obj_store: HashMap::new(),
            class_inst_store: HashMap::new(),
            var_scopes: Vec::new(),
            var_tracks: Vec::new(),
            scope_count: 0,
            current_class_obj_id: 0,
        }
    }
    pub fn push_scope(&mut self) -> () {
        self.var_scopes.push(HashMap::new());
        self.var_tracks.push(Vec::new());
    }
    pub fn pop_scope(&mut self) -> bool {
        if self.var_scopes.len() == 0 {
            return false;
        }
        self.var_scopes.pop();
        let track = self.var_tracks.pop().unwrap();
        for item in track {
            self.rem_class_inst(item.0, item.1);
        }
        return true;
    }
    pub fn add_class_obj(&mut self, cname: &str) -> () {
        self.class_obj_store.insert(hash(&cname), ClassObj::new(0, cname, vec![], vec![]));
    }
    pub fn add_class_inst(&mut self, cid: u64) -> Result<&ClassInstObj, ()> {
        let x: &mut ClassObj = match self.class_obj_store.get_mut(&cid) {Some(c)=>c,None=>{return Err(());}};
        let i = x.create();
        let idv = i.id;
        self.class_inst_store.insert((cid, i.id), i);
        return Ok(self.class_inst_store.get(&(cid, idv)).unwrap());
    }
    pub fn rem_class_obj(&mut self, cid: u64) -> bool {
        return match self.class_obj_store.remove(&cid) {Some(_)=>true,_=>false};
    }
    pub fn rem_class_inst(&mut self, cid: u64, id: u64) -> bool {
        return match self.class_inst_store.remove(&(cid, id)) {Some(_)=>true,_=>false};
    }
    pub fn get_prim_var(&self, varid: u64) -> Result<&Token, bool> {
        if self.scope_count == 0 {
            return Err(false);
        }
        let mut i: usize = self.scope_count - 1;
        loop {
            if self.var_scopes[i].contains_key(&varid) {
                return Ok(match self.var_scopes[i].get(&varid) {Some(x)=>x,None=>{return Err(true);}});
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        return Err(true);
    }
    pub fn set_prim_var(&mut self, varid: u64, val: Token) -> Result<(), ()> {
        if self.scope_count == 0 {
            return Err(());
        }
        self.var_scopes[self.scope_count-1].insert(varid, val);
        return Ok(());
    }
    pub fn class_static_op(&mut self, cid: u64, opid: u64) -> Result<&FuncObj, ()> {
        let x: &ClassObj = match self.class_obj_store.get(&cid) {Some(c)=>c,None=>{return Err(());}};
        return match x.stat_funcs.get(&opid) {Some(f)=>Ok(f),None=>Err(())};
    }
    pub fn class_inst_op(&mut self, cid: u64, opid: u64) -> Result<&FuncObj, ()> {
        let x: &ClassObj = match self.class_obj_store.get(&cid) {Some(c)=>c,None=>{return Err(());}};
        return match x.inst_funcs.get(&opid) {Some(f)=>Ok(f),None=>Err(())};
    }
}