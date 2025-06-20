use std::{fmt, collections::hash_map::DefaultHasher, hash::Hash, hash::Hasher};

pub const KEYWORDS: [&str; 28] = ["return", "returnf", "func", "if", "elseif", "else", "loop", "while", "for", "continue", "break", "try", "catch", "finally", "class", "constructor", "static", "private", "public", "readonly", "const", "import", "export", "from", "as", "interface", "extends", "implements"];

pub const TYPELST: [&str; 15] = ["string", "bool", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "void"];

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    return s.finish();
}

pub fn token_to_id(t: &Token) -> u8 {
    return match t {
        Token::Grp(_) => 0,
        Token::Ptr(_) => 1,
        Token::Dat(_) => 2,
        Token::Opr(_) => 3,
        Token::Dir(_) => 4,
        Token::Kwd(_) => 5,
        Token::Lit(_) => 6,
        Token::Sym(_) => 7,
        Token::Typ(_) => 8,
    };
}

pub fn primitive_to_id(p: &Primitive) -> u8 {
    return match p {
        Primitive::String(_) => 0,
        Primitive::Bool(_) => 1,
        Primitive::Int(_) => 2,
        Primitive::Short(_) => 3,
        Primitive::Long(_) => 4,
        Primitive::Byte(_) => 5,
        Primitive::UInt(_) => 6,
        Primitive::UShort(_) => 7,
        Primitive::ULong(_) => 8,
        Primitive::Float(_) => 9,
        Primitive::Double(_) => 10,
        Primitive::Null => 11,
    };
}

#[derive(Clone, Copy)]
pub enum PrimType {
    String = 0,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Void
}

#[derive(Clone, Copy)]
pub enum Keyword {
    Return = 0,
    Returnf,
    Func,
    If,
    Elseif,
    Else,
    Loop,
    While,
    For,
    Continue,
    Break,
    Try,
    Catch,
    Finally,
    Class,
    Static,
    Private,
    Public,
    Readonly,
    Const,
    Import,
    Export,
    From,
    As,
    NoMatch
}

#[derive(Clone, Copy)]
pub enum Directive {
    Wrapper = 0,
    Wrap,
    MustOverride,
    NoOverride,
    Seperate,
    Unsafe,
    IsUnsafe,
    LIBCALL,
    Test,
    NoMatch
}

#[derive(Clone)]
pub enum Primitive {
    String(String),
    Bool(bool),
    Int(i32),
    Short(i16),
    Long(i64),
    Byte(u8),
    UInt(u32),
    UShort(u16),
    ULong(u64),
    Float(f32),
    Double(f64),
    Null,
}

impl fmt::Debug for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if match self {Primitive::Null=>true,_=>false} {
            return f.debug_tuple("Null").finish();
        }
        let mut v: String = match self {
            Primitive::String(s)=>s.to_string(),
            Primitive::Bool(s)=>format!("{}", s),
            Primitive::Int(s)=>format!("{}", s),
            Primitive::Short(s)=>format!("{}", s),
            Primitive::Long(s)=>format!("{}", s),
            Primitive::Byte(s)=>format!("{}", s),
            Primitive::UInt(s)=>format!("{}", s),
            Primitive::UShort(s)=>format!("{}", s),
            Primitive::ULong(s)=>format!("{}", s),
            Primitive::Float(s)=>format!("{}", s),
            Primitive::Double(s)=>format!("{}", s),
            _=>"".to_string(),
        };
        // let c = format!("\x1b[38;2;{}m", match std::env::var("ansi"){Ok(va)=>va,Err(s)=>{panic!("{}",s);}});
        v = String::from(match self {Primitive::Int(_)=>"\x1b[38;2;200;255;175m",Primitive::Short(_)=>"\x1b[38;2;200;255;175m",Primitive::Long(_)=>"\x1b[38;2;200;255;175m",Primitive::Byte(_)=>"\x1b[38;2;200;255;175m",Primitive::UShort(_)=>"\x1b[38;2;200;255;175m",Primitive::UInt(_)=>"\x1b[38;2;200;255;175m",Primitive::ULong(_)=>"\x1b[38;2;200;255;175m",Primitive::Float(_)=>"\x1b[38;2;200;255;175m",Primitive::Double(_)=>"\x1b[38;2;200;255;175m",Primitive::String(_)=>"\x1b[38;2;255;200;0m",Primitive::Bool(_)=>"\x1b[38;2;200;100;255m",_=>"\x1b[0m"}) + &v + "\x1b[0m";
        print!("{}({})",match self {Primitive::String(_)=>"String",Primitive::Bool(_)=>"Bool",Primitive::Int(_)=>"Int",Primitive::Short(_)=>"Short",Primitive::Long(_)=>"Long",Primitive::Byte(_)=>"Byte",Primitive::UInt(_)=>"UInt",Primitive::UShort(_)=>"UShort",Primitive::ULong(_)=>"ULong",Primitive::Float(_)=>"Float",Primitive::Double(_)=>"Double",_=>""},v);
        return Ok(());
    }
}

#[derive(Clone)]
pub enum Token {
    Grp(Vec<Token>), // group, must be executed as a whole
    Ptr(String), // pointer to an object
    Dat(Primitive), // primitive data
    Opr(u8), // operation
    Dir(u8), // directive call
    Kwd(u8), // keyword
    Lit(String), // literal text (names)
    Sym(char), // single character symbol
    Typ(u8), // value type
}

impl fmt::Debug for Token {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print!("{}(", match self {Token::Grp(_)=>"Grp",Token::Ptr(_)=>"Ptr",Token::Dat(_)=>"Dat",Token::Opr(_)=>"Opr",Token::Dir(_)=>"Dir",Token::Kwd(_)=>"Kwd",Token::Lit(_)=>"Lit",Token::Sym(_)=>"Sym",Token::Typ(_)=>"Type",});
        match self {
            Token::Grp(v)=>print!("{:?}", v),
            Token::Ptr(s)=>print!("\x1b[38;2;0;200;0m{s}\x1b[0m"),
            Token::Dat(p)=>print!("{p:?}"),
            Token::Opr(o)=>print!("\x1b[38;2;255;100;50m{}\x1b[0m",match o{0=>&"+",1=>&"-",2=>&"*",3=>&"/",4=>&"**",5=>&"%",6=>&"!",7=>&"&",8=>&"&&",9=>&"|",10=>&"||",11=>&"^",12=>&"<",13=>&">",14=>&"...",15=>&".",16=>&"?",17=>&"=",18=>&"<=",19=>&">=",20=>&"<<",21=>&">>",22=>&"|<",23=>&">|",24=>&"==",25=>&"!=",26=>&"!!=",27=>&"&=",28=>&"&&=",29=>&"|=",30=>&"||=",31=>&"^=",32=>&"+=",33=>&"-=",34=>&"*=",35=>&"/=",36=>&"**=",37=>&"%=",38=>&"$",39=>&"++",40=>&"--",41=>&"<<=",42=>&">>=",43=>&"|<=",44=>&">|=",45=>&",",46=>&":",_=>&"INVALID"}),
            Token::Dir(s)=>print!("\x1b[38;2;255;100;150m{s}\x1b[0m"),
            Token::Kwd(s)=>print!("\x1b[38;2;200;100;255m{s}\x1b[0m"),
            Token::Lit(s)=>print!("\x1b[38;2;255;150;0m{s}\x1b[0m"),
            Token::Sym(c)=>print!("\x1b[38;2;150;150;150m{c}\x1b[0m"),
            Token::Typ(t)=>print!("\x1b[38;2;0;225;225m{}\x1b[0m",match t{0=>"string",1=>"bool",2=>"int",3=>"short",4=>"long",5=>"byte",6=>"uint",7=>"ushort",8=>"ulong",9=>"float",10=>"double",_=>"INVALID"}),
        };
        print!(")");
        return Ok(());
    }
}

#[derive(Clone)]
pub struct ArgsObj {
    pub arglst: Vec<(u8, String, Token)>,
}

impl ArgsObj {
    pub fn new() -> ArgsObj {
        ArgsObj {
            arglst: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        return self.arglst.len();
    }
    pub fn extend<T: Iterator<Item = (u8, String, Token)>>(&mut self, items: T) {
        self.arglst.extend(items);
    }
    pub fn push(&mut self, item: (u8, String, Token)) {
        self.arglst.push(item);
    }
}

impl fmt::Debug for ArgsObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.arglst.iter()).finish()
    }
}

#[derive(Clone)]
pub struct PropsObj {}

impl fmt::Debug for PropsObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().finish()
    }
}

#[derive(Clone)]
pub struct FuncObj {
    pub name: String,
    pub arglst: ArgsObj,
    pub toks: Vec<Token>,
}

pub struct ClassInstObj {
    pub cid: u64,
    pub id: u64,
    pub props: PropsObj,
}

impl fmt::Debug for ClassInstObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClassInstObj").field("class_id", &self.cid).field("id", &self.id).field("props", &self.props).finish()
    }
}

impl ClassInstObj {
    pub fn new(cid: u64, id: u64, props: PropsObj) -> ClassInstObj {
        ClassInstObj {
            cid: cid,
            id: id,
            props: props,
        }
    }
}