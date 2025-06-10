use crate::data::{Keyword, PrimType, Directive, TYPELST, KEYWORDS};

impl From<u8> for Directive {
    fn from(num: u8) -> Self {
        return match num {
            0 => Self::Wrapper,
            1 => Self::Wrap,
            2 => Self::MustOverride,
            3 => Self::NoOverride,
            4 => Self::Seperate,
            5 => Self::Unsafe,
            6 => Self::IsUnsafe,
            7 => Self::LIBCALL,
            8 => Self::Test,
            _ => Self::NoMatch
        };
    }
}

impl From<&u8> for Directive {
    fn from(num: &u8) -> Self {
        return Self::from(*num);
    }
}

impl From<&str> for Directive {
    fn from(x: &str) -> Self {
        return match x {
            "wrapper" => Self::Wrapper,
            "wrap" => Self::Wrap,
            "must_override" => Self::MustOverride,
            "no_override" => Self::NoOverride,
            "seperate" => Self::Seperate,
            "unsafe" => Self::Unsafe,
            "is_unsafe" => Self::IsUnsafe,
            "LIBCALL" => Self::LIBCALL,
            "test" => Self::Test,
            _ => Self::NoMatch,
        };
    }
}

impl From<Directive> for u8 {
    fn from(x: Directive) -> Self {
        return x as isize as u8;
    }
}

impl From<&Directive> for u8 {
    fn from(x: &Directive) -> Self {
        return *x as isize as u8;
    }
}

impl PartialEq<u8> for Directive {
    fn eq(&self, other: &u8) -> bool {
        return *other == *self as u8;
    }
}

impl PartialEq<&u8> for Directive {
    fn eq(&self, other: &&u8) -> bool {
        return **other == *self as u8;
    }
}

impl PartialEq<u8> for &Directive {
    fn eq(&self, other: &u8) -> bool {
        return *other == **self as u8;
    }
}

impl From<Directive> for &str {
    fn from(x: Directive) -> Self {
        return match x {
            Directive::Wrapper => "wrapper",
            Directive::Wrap => "wrap",
            Directive::MustOverride => "must_override",
            Directive::NoOverride => "no_override",
            Directive::Seperate => "seperate",
            Directive::Unsafe => "unsafe",
            Directive::IsUnsafe => "is_unsafe",
            Directive::LIBCALL => "LIBCALL",
            Directive::Test => "Test",
            Directive::NoMatch => "INVALID DIRECTIVE"
        };
    }
}

impl From<Directive> for String {
    fn from(x: Directive) -> Self {
        return <&str>::from(x).to_owned();
    }
}

impl PartialEq<&str> for Directive {
    fn eq(&self, other: &&str) -> bool {
        return *other == <&str>::from(*self);
    }
}

impl PartialEq<Directive> for &str {
    fn eq(&self, other: &Directive) -> bool {
        return other == self;
    }
}

impl PartialEq<&Directive> for &str {
    fn eq(&self, other: &&Directive) -> bool {
        return *other == self;
    }
}

impl From<String> for Directive {
    fn from(x: String) -> Self {
        return Self::from(x.as_str());
    }
}

impl PartialEq<String> for PrimType {
    fn eq(&self, other: &String) -> bool {
        return other == TYPELST[*self as usize];
    }
}

impl PartialEq<String> for &PrimType {
    fn eq(&self, other: &String) -> bool {
        return *other == TYPELST[**self as usize];
    }
}

impl PartialEq<&str> for PrimType {
    fn eq(&self, other: &&str) -> bool {
        return *other == TYPELST[*self as usize];
    }
}

impl PartialEq<&str> for &PrimType {
    fn eq(&self, other: &&str) -> bool {
        return *other == TYPELST[**self as usize];
    }
}

impl PartialEq<PrimType> for String {
    fn eq(&self, other: &PrimType) -> bool {
        return other == self;
    }
}

impl PartialEq<&PrimType> for String {
    fn eq(&self, other: &&PrimType) -> bool {
        return *other == self;
    }
}

impl PartialEq<PrimType> for &str {
    fn eq(&self, other: &PrimType) -> bool {
        return other == self;
    }
}

impl PartialEq<&PrimType> for &str {
    fn eq(&self, other: &&PrimType) -> bool {
        return *other == self;
    }
}

impl From<PrimType> for usize {
    fn from(x: PrimType) -> Self {
        return x as u8 as usize;
    }
}

impl From<&str> for PrimType {
    fn from(s: &str) -> Self {
        return match s {
            "string" => Self::String,
            "bool" => Self::Bool,
            "u8" => Self::U8,
            "u16" => Self::U16,
            "u32" => Self::U32,
            "u64" => Self::U64,
            "u128" => Self::U128,
            "i8" => Self::I8,
            "i16" => Self::I16,
            "i32" => Self::I32,
            "i64" => Self::I64,
            "i128" => Self::I128,
            "f32" => Self::F32,
            "f64" => Self::F64,
            "void" => Self::Void,
            _ => Self::Void,
        };
    }
}

impl From<String> for PrimType {
    fn from(s: String) -> Self {
        return PrimType::from(s.as_str());
    }
}

impl From<PrimType> for u8 {
    fn from(p: PrimType) -> Self {
        return p as isize as u8;
    }
}

impl From<&PrimType> for u8 {
    fn from(p: &PrimType) -> Self {
        return *p as isize as u8;
    }
}

impl PartialEq<u8> for PrimType {
    fn eq(&self, other: &u8) -> bool {
        return *other == *self as u8;
    }
}

impl PartialEq<u8> for &PrimType {
    fn eq(&self, other: &u8) -> bool {
        return *other == **self as u8;
    }
}

impl PartialEq<PrimType> for u8 {
    fn eq(&self, other: &PrimType) -> bool {
        return *self == *other as u8;
    }
}

impl PartialEq<&PrimType> for u8 {
    fn eq(&self, other: &&PrimType) -> bool {
        return *self == **other as u8;
    }
}

impl From<u8> for PrimType {
    fn from(num: u8) -> Self {
        return match num {
            0 => Self::String,
            1 => Self::Bool,
            2 => Self::U8,
            3 => Self::U16,
            4 => Self::U32,
            5 => Self::U64,
            6 => Self::U128,
            7 => Self::I8,
            8 => Self::I16,
            9 => Self::I32,
            10 => Self::I64,
            11 => Self::I128,
            12 => Self::F32,
            13 => Self::F64,
            14 => Self::Void,
            _ => Self::Void,
        };
    }
}

impl From<Keyword> for usize {
    fn from(x: Keyword) -> Self {
        return x as u8 as usize;
    }
}

impl From<Keyword> for String {
    fn from(x: Keyword) -> Self {
        return KEYWORDS[x as usize].to_owned();
    }
}

impl From<&Keyword> for String {
    fn from(x: &Keyword) -> Self {
        return String::from(*x);
    }
}

impl From<&str> for Keyword {
    fn from(x: &str) -> Self {
        return match x {
            "return" => Self::Return,
            "returnf" => Self::Returnf,
            "func" => Self::Func,
            "if" => Self::If,
            "elseif" => Self::Elseif,
            "else" => Self::Else,
            "loop" => Self::Loop,
            "while" => Self::While,
            "for" => Self::For,
            "continue" => Self::Continue,
            "break" => Self::Break,
            "try" => Self::Try,
            "catch" => Self::Catch,
            "finally" => Self::Finally,
            "class" => Self::Class,
            "static" => Self::Static,
            "private" => Self::Private,
            "public" => Self::Public,
            "readonly" => Self::Readonly,
            "const" => Self::Const,
            "import" => Self::Import,
            "export" => Self::Export,
            "from" => Self::From,
            "as" => Self::As,
            _ => Self::NoMatch
        };
    }
}

impl From<String> for Keyword {
    fn from(s: String) -> Self {
        return Keyword::from(s.as_str());
    }
}

impl From<Keyword> for u8 {
    fn from(x: Keyword) -> Self {
        return x as isize as u8;
    }
}

impl From<&Keyword> for u8 {
    fn from(x: &Keyword) -> Self {
        return *x as isize as u8;
    }
}

impl PartialEq<u8> for Keyword {
    fn eq(&self, other: &u8) -> bool {
        return *other == *self as u8;
    }
}

impl PartialEq<u8> for &Keyword {
    fn eq(&self, other: &u8) -> bool {
        return *other == **self as u8;
    }
}

impl PartialEq<Keyword> for u8 {
    fn eq(&self, other: &Keyword) -> bool {
        return *other == *self;
    }
}

impl PartialEq<&Keyword> for u8 {
    fn eq(&self, other: &&Keyword) -> bool {
        return **other == *self;
    }
}

impl From<u8> for Keyword {
    fn from (num: u8) -> Self {
        return match num {
            0 => Self::Return,
            1 => Self::Returnf,
            2 => Self::Func,
            3 => Self::If,
            4 => Self::Elseif,
            5 => Self::Else,
            6 => Self::Loop,
            7 => Self::While,
            8 => Self::For,
            9 => Self::Continue,
            10 => Self::Break,
            11 => Self::Try,
            12 => Self::Catch,
            13 => Self::Finally,
            14 => Self::Class,
            15 => Self::Static,
            16 => Self::Private,
            17 => Self::Public,
            18 => Self::Readonly,
            19 => Self::Const,
            20 => Self::Import,
            21 => Self::Export,
            22 => Self::From,
            23 => Self::As,
            _ => Self::NoMatch
        }
    }
}