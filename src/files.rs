use crate::data::{Token, Primitive, token_to_id, primitive_to_id};
use crate::tobytes::*;
use std::fs::{read, write};

const KEYWORDS: [&str; 24] = ["return", "returnf", "func", "if", "elseif", "else", "loop", "while", "for", "continue", "break", "try", "catch", "finally", "class", "constructor", "static", "private", "public", "readonly", "const", "import", "from", "as"];

const TYPELST: [&str; 11] = ["string", "bool", "int", "short", "long", "byte", "uint", "ushort", "ulong", "float", "double"];

fn l_load(data: &Vec<u8>) -> Result<Vec<Token>, String> {
    let mut v: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let l: usize = data.len();
    // println!("{:?}", data);
    loop {
        if i >= l {
            break;
        }
        let id = data[i];
        if id == 1 || id == 4 || id == 5 || id == 6 {
            let length: usize = bytes_to_big(&data[i+1..i+9]);
            // println!("{}, {:?}", i, data);
            // println!("{}, {:?}", length, &data[i+1..i+9]);
            let string: String = match String::from_utf8(Vec::from(&data[i+9..i+9+length])) {Ok(s)=>s,Err(_)=>{return Err(String::from("INVALID STRING LOAD NON UTF8"));}};
            v.push(match id {
                1 => Token::Ptr(string),
                4 => Token::Dir(string),
                5 => Token::Kwd(string),
                6 => Token::Lit(string),
                _ => {return Err(String::from("UNEXPECTED TOKEN TYPE FOR STRING LOAD"));},
            });
            i += 8 + length;
        } else if id == 0 {
            let length: usize = bytes_to_big(&data[i+1..i+9]);
            // println!("{}, {:?}", length, &data[i+1..i+9]);
            v.push(Token::Grp(l_load(&Vec::from(&data[i+9..i+9+length]))?));
            i += 8 + length;
        } else if id == 3 {
            v.push(Token::Opr(*&data[i+1]));
            i += 1;
        } else if id == 8 {
            v.push(Token::Typ(*&data[i+1]));
            i += 1;
        } else if id == 2 {
            i += 1;
            // load primitive values
            match &data[i] {
                0 => {
                    let length: usize = bytes_to_big(&data[i+1..i+9]);
                    let string: String = match String::from_utf8(Vec::from(&data[i+9..i+9+length])) {Ok(s)=>s,Err(_)=>{return Err(String::from("INVALID STRING LOAD NON UTF8"));}};
                    v.push(Token::Dat(Primitive::String(string)));
                    i += 8 + length;
                },
                1 => {
                    v.push(Token::Dat(Primitive::Bool(match data[i+1] {0=>false,_=>true})));
                    i += 1;
                },
                2 => {
                    v.push(Token::Dat(Primitive::Int(bytes_to_i32(&data[i+1..i+5]))));
                    i += 4;
                },
                3 => {
                    v.push(Token::Dat(Primitive::Short(bytes_to_i16(&data[i+1..i+3]))));
                    i += 2;
                },
                4 => {
                    v.push(Token::Dat(Primitive::Long(bytes_to_i64(&data[i+1..i+9]))));
                    i += 8;
                },
                5 => {
                    v.push(Token::Dat(Primitive::Byte(data[i+1])));
                    i += 1;
                },
                6 => {
                    v.push(Token::Dat(Primitive::UInt(bytes_to_u32(&data[i+1..i+5]))));
                    i += 4;
                },
                7 => {
                    v.push(Token::Dat(Primitive::UShort(bytes_to_u16(&data[i+1..i+3]))));
                    i += 2;
                },
                8 => {
                    v.push(Token::Dat(Primitive::ULong(bytes_to_u64(&data[i+1..i+9]))));
                    i += 8;
                },
                9 => {
                    v.push(Token::Dat(Primitive::Float(f32::from_bits(bytes_to_u32(&data[i+1..i+5])))));
                    i += 4;
                },
                10 => {
                    v.push(Token::Dat(Primitive::Double(f64::from_bits(bytes_to_u64(&data[i+1..i+9])))));
                    i += 8;
                },
                _ => {return Err("INVALID PRIMITIVE TYPE ID".to_string());},
            };
        } else if id == 7 {
            v.push(Token::Sym(data[i+1] as char));
            i += 1;
        }
        i += 1;
    }
    return Ok(v);
}

pub fn load(path: &str) -> Result<Vec<Token>, String> {
    let data: Vec<u8> = match read(path) {Ok(v)=>v,Err(_)=>{return Err(String::from("File IO Error"));}};
    return l_load(&data);
}

// recursive function to flatten groups
fn s_flatten(data: &Vec<Token>) -> Vec<u8> {
    let mut i: usize = 0;
    let l: usize = data.len();
    let mut v: Vec<u8> = Vec::<u8>::new();
    loop {
        if i >= l {
            break;
        }
        let id: u8 = token_to_id(&data[i]);
        v.push(id);
        if id == 0 {
            let flat: Vec<u8> = s_flatten(match &data[i] {Token::Grp(v)=>v,_=>{panic!("");}});
            v.extend(big_to_bytes(flat.len()));
            v.extend(flat);
        } else if id == 1 || id == 4 || id == 5 || id == 6 {
            let val: &String = match &data[i] {Token::Dir(s)=>s,Token::Kwd(s)=>s,Token::Ptr(s)=>s,Token::Lit(s)=>s,x=>{panic!("{}",token_to_id(&x));}};
            v.extend(big_to_bytes(val.len()));
            v.extend(val.as_bytes());
        } else if id == 2 {
            match &data[i] {Token::Dat(d)=>{
                v.push(primitive_to_id(d));
                match d {
                    Primitive::String(s) => {v.extend(big_to_bytes(s.len()));v.extend(s.as_bytes());},
                    Primitive::Bool(b) => {v.push(match b{true=>1u8,false=>0u8});},
                    Primitive::Int(val) => {v.extend(i32_to_bytes(*val));},
                    Primitive::Short(val) => {v.extend(i16_to_bytes(*val));},
                    Primitive::Long(val) => {v.extend(i64_to_bytes(*val));},
                    Primitive::Byte(val) => {v.push(*val);},
                    Primitive::UInt(val) => {v.extend(u32_to_bytes(*val));},
                    Primitive::UShort(val) => {v.extend(u16_to_bytes(*val));},
                    Primitive::ULong(val) => {v.extend(u64_to_bytes(*val));}
                    Primitive::Float(val) => {v.extend(u32_to_bytes(val.to_bits()));},
                    Primitive::Double(val) => {v.extend(u64_to_bytes(val.to_bits()));},
                    Primitive::Null => {},
                };
            },_=>{panic!("");}};
        } else if id == 3 || id == 8 {
            v.push(match &data[i] {Token::Opr(val)=>*val,Token::Typ(val)=>*val,_=>{panic!("");}});
        } else if id == 7 {
            v.push(match &data[i] {Token::Sym(c)=>*c as u8,_=>{panic!("");}});
        }
        i += 1;
    }
    return v;
}

pub fn save(path: &str, data: &Vec<Token>) {
    match write(path, s_flatten(data).as_slice()) {Ok(_)=>{}, Err(e)=>{println!("{}", e);}};
}

fn c_stamp(emsg: &str, lc: usize, cc: usize) -> String {
    return format!("{emsg} ({lc}, {cc})");
}

fn c_rawc(mut lc: usize, mut cc: usize, contents: &[char], v: &mut Vec<Token>) -> Result<(), String> {
    let mut i: usize = 0;
    let l: usize = contents.len();
    let mut build: String = String::new();
    let mut tokid: [char; 3] = ['0', '0', '0'];
    let mut tidcount: usize = 0;
    while i < l {
        if contents[i] == '\n' || contents[i] == ' ' {
            cc += 1;
            if contents[i] == '\n' {
                lc += 1;
                cc = 0;
            }
            i += 1;
            continue;
        }
        tokid[tidcount] = contents[i];
        tidcount += 1;
        if tidcount == 3 {
            tidcount = 0;
            i += 2;
            if tokid == ['G','r','p'] {
                let mut gv: Vec<Token> = Vec::new();
                let mut d: usize = 1;
                let f = (i, lc, cc);
                loop {
                    if contents[i] == '(' {
                        d += 1;
                    }
                    if contents[i] == ')' {
                        d -= 1;
                        if d == 0 {
                            break;
                        }
                    }
                    cc += 1;
                    if contents[i] == '\n' {
                        cc = 0;
                        lc += 1;
                    }
                    i += 1;
                    if i == l {
                        return Err(c_stamp("UNCLOSED GRP TOKEN", lc, cc));
                    }
                }
                match c_rawc(f.1, f.2, &contents[f.0..i], &mut gv) {Ok(_)=>{},Err(e)=>{return Err(c_stamp(format!("GRP TOKEN CONTENTS ERROR: {e}").as_str(), f.1, f.2));}};
                v.push(Token::Grp(gv));
            } else if match tokid {['P','t','r']=>true,['D','i','r']=>true,['K','w','d']=>true,['L','i','t']=>true,_=>false} {
                if contents[i] != '"' {
                    return Err(c_stamp("MALFORMED STR LIKE TOKEN", lc, cc));
                }
                i += 1;
                loop {
                    if contents[i] == '"' {
                        break;
                    }
                    build.push(contents[i]);
                    i += 1;
                    if i == l {
                        return Err(c_stamp("UNCLOSED STR LIKE TOKEN", lc, cc));
                    }
                }
                i += 1;
                v.push(match tokid {
                    ['P','t','r'] => Token::Ptr(build),
                    ['D','i','r'] => Token::Dir(build),
                    ['K','w','d'] => Token::Kwd(build),
                    ['L','i','t'] => Token::Lit(build),
                    _ => {panic!("");}
                });
                build = String::new();
            } else if tokid == ['D','a','t'] {
                loop {
                    if contents[i] == '(' {
                        break;
                    }
                    if contents[i] == '\n' {
                        return Err(c_stamp("BROKEN DATA (NEWLINE)", lc, cc));
                    }
                    build.push(contents[i]);
                    i += 1;
                    cc += 1;
                    if i == l {
                        return Err(c_stamp("UNOPENED PRIMITIVE DECLARATION", lc, cc));
                    }
                }
                i += 1;
                cc += 1;
                let x = build;
                build = String::new();
                if x == "String" {
                    if contents[i] != '"' {
                        return Err(c_stamp("UNOPENED STRING PRIMITIVE VALUE", lc, cc));
                    }
                    i += 1;
                    cc += 1;
                }
                loop {
                    if contents[i] == ')' {
                        break;
                    }
                    if contents[i] == '"' && x == "String" && contents[i-1] != '\\' {
                        i += 1;
                        cc += 1;
                        if contents[i] != ')' {
                            return Err(c_stamp("UNCLOSED PRIMITIVE STR DECLARATION", lc, cc));
                        }
                        break;
                    }
                    if contents[i] == '\n' {
                        return Err(c_stamp("BROKEN DATA (NEWLINE)", lc, cc));
                    }
                    build.push(contents[i]);
                    cc += 1;
                    i += 1;
                    if i == l {
                        return Err(c_stamp("UNCLOSED PRIMITIVE DECLARATION", lc, cc));
                    }
                }
                v.push(Token::Dat(match x.as_str() {
                    "String" => Primitive::String(build),
                    "Bool" => Primitive::Bool(match build.as_str() {"true"=>true,"false"=>false,_=>{return Err(c_stamp("INVALID VALUE FOR BOOLEAN PRIMITIVE", lc, cc));}}),
                    "Int" => Primitive::Int(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING INT FROM {build}").as_str(), lc, cc));}}),
                    "Short" => Primitive::Short(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING SHORT FROM {build}").as_str(), lc, cc));}}),
                    "Long" => Primitive::Long(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING LONG FROM {build}").as_str(), lc, cc));}}),
                    "Byte" => Primitive::Byte(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING BYTE FROM {build}").as_str(), lc, cc));}}),
                    "UInt" => Primitive::UInt(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING UINT FROM {build}").as_str(), lc, cc));}}),
                    "UShort" => Primitive::UShort(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING USHORT FROM {build}").as_str(), lc, cc));}}),
                    "ULong" => Primitive::ULong(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING ULONG FROM {build}").as_str(), lc, cc));}}),
                    "Float" => Primitive::Float(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING FLOAT FROM {build}").as_str(), lc, cc));}}),
                    "Double" => Primitive::Double(match build.parse() {Ok(x)=>x,Err(_)=>{return Err(c_stamp(format!("ERROR PARSING DOUBLE FROM {build}").as_str(), lc, cc));}}),
                    t => {return Err(c_stamp(format!("INVALID PRIMITIVE TYPE ({t})").as_str(), lc, cc));}
                }));
                build = String::new();
                i += 1;
            } else if tokid == ['O','p','r'] {
                let freeze = i;
                v.push(Token::Opr(match contents[i] {
                    ',' => 45,
                    ':' => 46,
                    '+' => match i + 1 < l {false=>0,_=>match contents[i+1] {
                        '=' => {i+=1;32},
                        '+' => {i+=1;39},
                        _ => 0,
                    }},
                    '-' => match i + 1 < l {false=>1,_=>match contents[i+1] {
                        '=' => {i+=1;33},
                        '-' => {i+=1;40},
                        _ => 1,
                    }},
                    '*' => match i + 1 < l {false=>2,_=>match contents[i+1] {
                        '*' => match contents[i+2] {
                            '=' => {i+=2;36},
                            _ => {i+=1;4},
                        },
                        '=' => {i+=1;34},
                        _ => 2,
                    }},
                    '/' => match i + 1 < l {false=>3,_=>match contents[i+1] {
                        '=' => {i+=1;35},
                        _ => 3,
                    }},
                    '%' => match i + 1 < l {false=>5,_=>match contents[i+1] {
                        '=' => {i+=1;37},
                        _ => 5,
                    }},
                    '!' => match i + 1 < l {false=>6,_=>match contents[i+1] {
                        '=' => {i+=1;25},
                        '!' => match contents[i+2] {
                            '=' => {i+=2;26},
                            _ => {return Err(c_stamp("INVALID OPERATOR",lc,cc));},
                        },
                        _ => 6,
                    }},
                    '&' => match i + 1 < l {false=>7,_=>match contents[i+1] {
                        '&' => match contents[i+2] {
                            '=' => {i+=2;28},
                            _ => {i+=1;8},
                        },
                        '=' => {i+=1;27},
                        _ => 7,
                    }},
                    '|' => match i + 1 < l {false=>9,_=>{i+=1;match contents[i] {
                        '|' => match contents[i+1] {
                            '=' => {i+=1;30},
                            _ => 10,
                        },
                        '=' => 29,
                        '<' => match i + 1 < l {false=>22,_=>match contents[i+1] {
                            '=' => {i+=1;43},
                            _ => 22,
                        }},
                        _ => {i-=1;9},
                    }}},
                    '^' => match i + 1 < l {false=>11,_=>match contents[i+1] {
                        '=' => {i+=1;31},
                        _ => 11,
                    }},
                    '<' => match i + 1 < l {false=>12,_=>{i+=1;match contents[i] {
                        '=' => 18,
                        '<' => match i + 1 < l {false=>20,_=>match contents[i+1] {
                            '=' => {i+=1;41},
                            _ => 20,
                        }},
                        _ => {i-=1;12},
                    }}},
                    '>' => match i + 1 < l {false=>13,_=>{i+=1;match contents[i] {
                        '=' => 19,
                        '>' => match i + 1 < l {false=>21,_=>match contents[i+1] {
                            '=' => {i+=1;42},
                            _ => 21,
                        }},
                        '|' => match i + 1 < l {false=>23,_=>match contents[i+1] {
                            '=' => {i+=1;44},
                            _ => 23,
                        }},
                        _ => {i-=1;13},
                    }}},
                    '.' => match i + 1 < l {false=>15,_=>match contents[i+1] {
                        '.' => match contents[i+2] {
                            '.' => {i+=2;14},
                            _ => {return Err(c_stamp("INVALID OPERATOR",lc,cc));},
                        },
                        _ => 15,
                    }},
                    '?' => 16,
                    '=' => match i + 1 < l {false=>17,_=>match contents[i+1] {
                        '=' => {i+=1;24},
                        _ => 17,
                    }},
                    '$' => 38,
                    _ => {return Err(c_stamp("UNEXPECTED OPERATOR",lc,cc));},
                }));
                i += 1;
                cc += i - freeze;
            } else if tokid == ['S','y','m'] {
                v.push(Token::Sym(contents[i]));
                i += 1;
            } else if tokid == ['T','y','p'] {
                loop {
                    if contents[i] == ')' {
                        break;
                    }
                    if !contents[i].is_alphabetic() {
                        return Err(c_stamp("INVALID TYP TOKEN, TYPE MUST BE ALL ALPHA", lc, cc));
                    }
                    build.push(contents[i]);
                    i += 1;
                    if i == l {
                        return Err(c_stamp("UNCLOSED TYP TOKEN", lc, cc));
                    }
                }
                v.push(Token::Typ(match build.as_str() {
                    "string" => 0,
                    "bool" => 1,
                    "int" => 2,
                    "short" => 3,
                    "long" => 4,
                    "byte" => 5,
                    "uint" => 6,
                    "ushort" => 7,
                    "ulong" => 8,
                    "float" => 9,
                    "double" => 10,
                    _ => {return Err(c_stamp("INVALID TYP NAME", lc, cc));}
                }));
                build = String::new();
            } else {
                return Err(format!("INVALID TOKEN ID {tokid:?} ({lc}, {cc})"));
            }
        }
        i += 1;
    }
    return Ok(());
}

fn c_comp(contents: &[char], mut lc: usize, mut cc: usize) -> Result<Vec<Token>, String> {
    // for c in contents {
    //     print!("{c}");
    // }
    // println!();
    let mut v: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let l: usize = contents.len();
    let mut build: String = String::new();
    while i < l {
        cc += 1;
        // comments
        if i + 1 < l {
            if contents[i] == '/' && contents[i+1] == '/' {
                while i < l {
                    i += 1;
                    if contents[i] == '\n' {
                        lc += 1;
                        cc = 0;
                        break;
                    }
                }
                i += 1;
                continue;
            }
        }
        // whitespace
        if contents[i] == '\n' {lc+=1;cc=0;}
        if contents[i] == ' ' || contents[i] == '\n' || contents[i] == '\t' {i+=1;continue;}
        // strings
        if contents[i] == '"' {
            // println!("STR ENTER: {i}, {:?}", &contents[i..]);
            i += 1;
            cc += 1;
            let mut x = true;
            while i < l {
                cc += 1;
                if contents[i] == '"' {
                    v.push(Token::Dat(Primitive::String(build)));
                    build = String::new();
                    x = false;
                    break;
                }
                if contents[i] == '\n' {
                    println!("BROKEN STRING");
                    break;
                }
                if contents[i] == '\\' {
                    if i + 1 >= l {
                        return Err(c_stamp("EOF AFTER BACKSLASH", lc, cc));
                    }
                    build.push(match contents[i+1] {
                        'x' => match i+3<l{false=>{return Err(c_stamp("INVALID ESCAPE (EOF)",lc,cc));}_=>{i+=2;(match contents[i].to_digit(16){Some(a)=>a,None=>{return Err(c_stamp("INVALID ESCAPE (CHAR)",lc,cc));}}*16+match contents[i+1].to_digit(16){Some(a)=>a,None=>{return Err(c_stamp("INVALID ESCAPE (CHAR)",lc,cc));}}) as u8 as char}},
                        _ => contents[i+1],
                    });
                    i += 2;
                    continue;
                }
                build.push(contents[i]);
                i += 1;
            }
            if x {
                // println!("{}:{}; {}", lc, i, build);
                return Err(c_stamp(&format!("UNCLOSED STRING: {build}"),lc,cc));
            }
            i += 1;
            cc += 1;
            continue;
        }
        // int
        if contents[i].is_ascii_digit() {
            while i < l {
                if !contents[i].is_ascii_digit() {
                    break;
                }
                build.push(contents[i]);
                i += 1;
            }
            v.push(Token::Dat(Primitive::Int(build.parse().unwrap())));
            build = String::new();
            continue;
        }
        // operators
        if "+-*/%!&|^=<>?.$,:".contains(contents[i]) {
            let freeze = i;
            v.push(Token::Opr(match contents[i] {
                ',' => 45,
                ':' => 46,
                '+' => match i + 1 < l {false=>0,_=>match contents[i+1] {
                    '=' => {i+=1;32},
                    '+' => {i+=1;39},
                    _ => 0,
                }},
                '-' => match i + 1 < l {false=>1,_=>match contents[i+1] {
                    '=' => {i+=1;33},
                    '-' => {i+=1;40},
                    _ => 1,
                }},
                '*' => match i + 1 < l {false=>2,_=>match contents[i+1] {
                    '*' => match contents[i+2] {
                        '=' => {i+=2;36},
                        _ => {i+=1;4},
                    },
                    '=' => {i+=1;34},
                    _ => 2,
                }},
                '/' => match i + 1 < l {false=>3,_=>match contents[i+1] {
                    '=' => {i+=1;35},
                    _ => 3,
                }},
                '%' => match i + 1 < l {false=>5,_=>match contents[i+1] {
                    '=' => {i+=1;37},
                    _ => 5,
                }},
                '!' => match i + 1 < l {false=>6,_=>match contents[i+1] {
                    '=' => {i+=1;25},
                    '!' => match contents[i+2] {
                        '=' => {i+=2;26},
                        _ => {return Err(c_stamp("INVALID OPERATOR",lc,cc));},
                    },
                    _ => 6,
                }},
                '&' => match i + 1 < l {false=>7,_=>match contents[i+1] {
                    '&' => match contents[i+2] {
                        '=' => {i+=2;28},
                        _ => {i+=1;8},
                    },
                    '=' => {i+=1;27},
                    _ => 7,
                }},
                '|' => match i + 1 < l {false=>9,_=>{i+=1;match contents[i] {
                    '|' => match contents[i+1] {
                        '=' => {i+=1;30},
                        _ => 10,
                    },
                    '=' => 29,
                    '<' => match i + 1 < l {false=>22,_=>match contents[i+1] {
                        '=' => {i+=1;43},
                        _ => 22,
                    }},
                    _ => {i-=1;9},
                }}},
                '^' => match i + 1 < l {false=>11,_=>match contents[i+1] {
                    '=' => {i+=1;31},
                    _ => 11,
                }},
                '<' => match i + 1 < l {false=>12,_=>{i+=1;match contents[i] {
                    '=' => 18,
                    '<' => match i + 1 < l {false=>20,_=>match contents[i+1] {
                        '=' => {i+=1;41},
                        _ => 20,
                    }},
                    _ => {i-=1;12},
                }}},
                '>' => match i + 1 < l {false=>13,_=>{i+=1;match contents[i] {
                    '=' => 19,
                    '>' => match i + 1 < l {false=>21,_=>match contents[i+1] {
                        '=' => {i+=1;42},
                        _ => 21,
                    }},
                    '|' => match i + 1 < l {false=>23,_=>match contents[i+1] {
                        '=' => {i+=1;44},
                        _ => 23,
                    }},
                    _ => {i-=1;13},
                }}},
                '.' => match i + 1 < l {false=>15,_=>match contents[i+1] {
                    '.' => match contents[i+2] {
                        '.' => {i+=2;14},
                        _ => {return Err(c_stamp("INVALID OPERATOR",lc,cc));},
                    },
                    _ => 15,
                }},
                '?' => 16,
                '=' => match i + 1 < l {false=>17,_=>match contents[i+1] {
                    '=' => {i+=1;24},
                    _ => 17,
                }},
                '$' => 38,
                _ => {return Err(c_stamp("UNEXPECTED OPERATOR",lc,cc));},
            }));
            i += 1;
            cc += i - freeze;
            continue;
        }
        if contents[i].is_alphabetic() || contents[i] == '_' {
            while i < l {
                cc += 1;
                if !(contents[i].is_alphanumeric() || contents[i] == '_') {
                    break;
                }
                build.push(contents[i]);
                i += 1;
            }
            v.push(match build.as_str() {
                "true" => Token::Dat(Primitive::Bool(true)),
                "false" => Token::Dat(Primitive::Bool(false)),
                _ => {
                    if KEYWORDS.contains(&build.as_str()) {
                        Token::Kwd(build)
                    } else if TYPELST.contains(&build.as_str()) {
                        Token::Typ(match build.as_str() {
                            "string" => 0,
                            "bool" => 1,
                            "int" => 2,
                            "short" => 3,
                            "long" => 4,
                            "byte" => 5,
                            "uint" => 6,
                            "ushort" => 7,
                            "ulong" => 8,
                            "float" => 9,
                            "double" => 10,
                            _ => {panic!("");},
                        })
                    } else {
                        Token::Lit(build)
                    }
                }
            });
            build = String::new();
            continue;
        }
        if contents[i] == '@' {
            i += 1;
            cc += 1;
            while i < l {
                if !(contents[i].is_alphanumeric() || contents[i] == '_') {
                    break;
                }
                build.push(contents[i]);
                cc += 1;
                i += 1;
            }
            if build == "start_raw" {
                if &contents[i..i+3] != &['(',')',';'] {
                    return Err(c_stamp("INVALID START RAW STATEMENT", lc, cc));
                }
                i += 3;
                cc += 3;
                let ifreeze = (i, lc, cc);
                let mut ef: usize = 0;
                build = String::new();
                'outer : while i < l {
                    if contents[i] == '\n' {
                        lc += 1;
                        cc = 0;
                    }
                    if contents[i] == '@' {
                        ef = i;
                        while i < l {
                            build.push(contents[i]);
                            if contents[i] == ';' {
                                if build == "@end_raw();" {
                                    break 'outer;
                                }
                                return Err(c_stamp("DIRECTIVE EXPRESSIONS NOT ALLOWED INSIDE OF RAW SECTIONS", lc, cc));
                            }
                            i += 1;
                        }
                        return Err(c_stamp("UNCLOSED ENDING DIRECTIVE", lc, cc));
                    }
                    i += 1;
                    cc += 1;
                }
                if ef == 0 {
                    return Err(c_stamp("UNCLOSED RAW SECTION", ifreeze.1, ifreeze.2));
                }
                match c_rawc(ifreeze.1, ifreeze.2, &contents[ifreeze.0..ef], &mut v) {Ok(_)=>{},Err(e)=>{return Err(c_stamp(&e, lc, cc));}};
                i += 1;
                cc += 1;
            } else {
                v.push(Token::Dir(build));
            }
            build = String::new();
            continue;
        }
        if contents[i] == '(' {
            let mut depth: usize = 1;
            i += 1;
            cc += 1;
            while i < l {
                cc+=1;
                if contents[i] == '\n' {lc+=1;cc=0;}
                if contents[i] == ')' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                build.push(contents[i]);
                if contents[i] == '(' {
                    depth += 1;
                }
                i += 1;
            }
            v.push(Token::Grp(match c_comp(&build.chars().collect::<Vec<char>>(), lc, cc){Ok(v)=>v,Err(e)=>{return Err(c_stamp(&e, lc, cc));}}));
            build = String::new();
            i += 1;
            cc+=1;
            continue;
        }
        v.push(Token::Sym(contents[i]));
        i += 1;
    }
    return Ok(v);
}

pub fn compile(path: &str) -> Result<Vec<Token>, String> {
    let contents: Vec<char> = match String::from_utf8(match read(path){Ok(ve)=>ve,Err(_)=>{return Err("ERROR READING FILE".to_owned());}}){Ok(s)=>s,Err(_)=>{return Err("FILE CONTENTS NOT VALID UTF8".to_owned());}}.chars().collect();
    return c_comp(&contents[..], 0, 0);
}