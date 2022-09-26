use lang_4 as this;

use this::runner::{Runner};
// use this::files::{save, load, compile};
// use this::data::*;
use std::env::{args,Args};

fn main () {
    let mut a: Args = args();
    a.next();
    let mut v: Vec<String> = Vec::<String>::with_capacity(a.len());
    v.extend(a);
    // let length: usize = v.len();
    // std::env::set_var("ansi", &v[length-1]);
    Runner::new().start(v);
    // let v: Vec<Token> = vec![Token::Dat(Primitive::String(String::from("BYTE"))),Token::Dat(Primitive::Bool(true)),Token::Dat(Primitive::Byte(10)),Token::Grp(vec![Token::Dat(Primitive::Int(1)),Token::Opr(20),Token::Dat(Primitive::Int(1))])];
    // save("test.cl4", v);
    // println!("{:?}", load("test.cl4"));
    // let toks: Vec<Token> = compile("test.l4").unwrap();
    // println!("{}, {:?}", toks.len(), toks);
}