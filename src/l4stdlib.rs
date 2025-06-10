#![allow(non_upper_case_globals)]

use crate::{data::{Primitive, PrimType, FuncObj, ArgsObj, PropsObj, Token, Directive}, storage::{ClassObj, Interface, Enum, EnumItem}};

pub type ModProp = (ModType, &'static str);
pub type FResult<T> = Result<T, u8>;

pub enum ModType {
    Constant = 0,
    Function,
    Enum,
    Class,
    Interface,
}

pub enum ModVal {
    Constant(PrimType, Primitive),
    Function(FuncObj),
    Enum(Enum),
    Class(ClassObj),
    Interface(Interface),
    Raw(Box<dyn FnMut() -> ()>),
}

pub trait LibModule {
    fn get_names() -> &'static[ModProp];
    fn get_value(name: &ModProp, objid: u64) -> FResult<ModVal>;
}

const StdAutoProps: &[ModProp] = &[(ModType::Function,"println")];
pub struct StdAuto {}
impl LibModule for StdAuto {
    fn get_names() -> &'static[ModProp] {
        return StdAutoProps;
    }
    fn get_value(name: &ModProp, _:u64) -> FResult<ModVal> {
        return Ok(match name {
            (ModType::Function, "println") => ModVal::Function(FuncObj{name:"println".to_owned(),arglst:ArgsObj{arglst:Vec::new(),},toks:Vec::new(),}),
            _ => {return Err(1);}
        });
    }
}

const StdSysProps: &[ModProp] = &[(ModType::Constant,"test")];
pub struct StdSys {}
impl LibModule for StdSys {
    fn get_names() -> &'static[ModProp] {
        return StdSysProps;
    }
    fn get_value(name: &ModProp, _:u64) -> FResult<ModVal> {
        return Ok(match name {
            (ModType::Constant, "test") => ModVal::Constant(PrimType::Bool,Primitive::Bool(false)),
            _ => {return Err(1);}
        });
    }
}

const StdFsProps: &[ModProp] = &[(ModType::Class,"File")];
pub struct StdFs {}
impl LibModule for StdFs {
    fn get_names() -> &'static[ModProp] {
        return StdFsProps;
    }
    fn get_value(name: &ModProp, objid: u64) -> FResult<ModVal> {
        return Ok(match name {
            (ModType::Class, "File") => ModVal::Class(ClassObj::new(objid, "File", &[0], PropsObj{}, vec![
                FuncObj {
                    name: "open".to_owned(),
                    arglst: ArgsObj{
                        arglst: vec![
                            (PrimType::String as u8, "path".to_owned(), Token::Dat(Primitive::Null)),
                            (PrimType::String as u8, "mode".to_owned(), Token::Dat(Primitive::String("r".to_owned())))
                        ]
                    },
                    toks: vec![
                        Token::Dir(Directive::LIBCALL as u8),
                        Token::Grp(vec![Token::Dat(Primitive::String("NIE".to_owned()))])
                    ]
                }
            ],
            vec![
                FuncObj {
                    name: "close".to_owned(),
                    arglst: ArgsObj {
                        arglst: vec![]
                    },
                    toks: vec![
                        Token::Dir(Directive::LIBCALL as u8),
                        Token::Grp(vec![Token::Dat(Primitive::String("NIE".to_owned()))])
                    ]
                }
            ])),
            _ => {return Err(1);}
        });
    }
}

pub struct StdIo {}
impl StdIo {
    pub 
}