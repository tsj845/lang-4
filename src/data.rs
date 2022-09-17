use std::fmt;

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
    Dir(String), // directive call
    Kwd(String), // keyword
    Lit(String), // literal text (names)
    Sym(char), // single character symbol
}

impl fmt::Debug for Token {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print!("{}(", match self {Token::Grp(_)=>"Grp",Token::Ptr(_)=>"ptr",Token::Dat(_)=>"Dat",Token::Opr(_)=>"Opr",Token::Dir(_)=>"Dir",Token::Kwd(_)=>"Kwd",Token::Lit(_)=>"Lit",Token::Sym(_)=>"Sym",});
        match self {
            Token::Grp(v)=>print!("{:?}", v),
            Token::Ptr(s)=>print!("{s}"),
            Token::Dat(p)=>print!("{p:?}"),
            Token::Opr(o)=>print!("\x1b[38;2;255;100;50m{}\x1b[0m",match o{0=>&"+",1=>&"-",2=>&"*",3=>&"/",4=>&"**",5=>&"%",6=>&"!",7=>&"&",8=>&"&&",9=>&"|",10=>&"||",11=>&"^",12=>&"<",13=>&">",14=>&"...",15=>&".",16=>&"?",17=>&"=",18=>&"<=",19=>&">=",20=>&"<<",21=>&">>",22=>&"|<",23=>&">|",24=>&"==",25=>&"!=",26=>&"!!=",27=>&"&=",28=>&"&&=",29=>&"|=",30=>&"||=",31=>&"^=",32=>&"+=",33=>&"-=",34=>&"*=",35=>&"/=",36=>&"**=",37=>&"%=",38=>&"$",39=>&"++",40=>&"--",41=>&"<<=",42=>&">>=",43=>&"|<=",44=>&">|=",45=>&",",46=>&":",_=>&"INVALID"}),
            Token::Dir(s)=>print!("{s}"),
            Token::Kwd(s)=>print!("{s}"),
            Token::Lit(s)=>print!("{s}"),
            Token::Sym(c)=>print!("{c}"),
        };
        print!(")");
        return Ok(());
    }
}