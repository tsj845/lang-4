use crate::files::{save, load, compile};
use crate::data::{Token, ArgsObj, token_to_id};
use crate::storage::{ClassObj, Storage};

pub type EResult = Result<usize, String>;
pub type GResult<T> = Result<T, String>;

/**
 * RUNNER FLAG TABLE
 * BYTE 1 {
 *      bit 1 - unsafe enable
 *      bit 2 - unsafe function declaration
 *      bit 3 - unassigned
 *      bit 4 - unassigned
 *      bit 5 - unassigned
 *      bit 6 - unassigned
 *      bit 7 - unassigned
 *      bit 8 - unassigned
 * }
 */

pub struct Runner {
    store : Storage,
    flags : [u8; 1],
}

impl Runner {
    pub fn new () -> Runner {
        Runner {
            store: Storage::new(),
            flags: [0],
        }
    }
    pub fn start (&mut self, v: Vec<String>) {
        let mut flags: u16 = 0;
        let mut i: usize = 1;
        let l: usize = v.len();
        loop {
            if i >= l {
                break;
            }
            match match v[i].as_ref() {
                "-s" => 0,
                "-l" => 1,
                "-r" => 2,
                _ => -1,
            } {-1=>{},x=>{flags=flags|(1<<x)}};
            i += 1;
        }
        println!("{flags}");
        if v[0] == "--help" {
            println!("lang4 {{filename}} [-slr] | [--help] [--version]\n\n-s      compile and save\n-l      load and run\n-r      compile and run\n\n");
            return;
        }
        if v[0] == "--version" {
            println!("lang4 0.0.0");
            return;
        }
        println!("RUNNING LANG 4 WITH {}", match flags {0=>"nothing",1=>"compile & save",2=>"load & run",4=>"compile & run",5=>"compile & save & run",_=>"invalid"});
        let mut tx = ClassObj::new(0, "Test", vec![], vec![]);
        println!("{tx:?}\n{:?}", tx.create());
        if flags == 1 {
            save(&v[1], &match compile(&v[0]) {Ok(x)=>x,Err(e)=>{panic!("{}", e);}});
        }
        if flags == 2 {
            self.run(&match load(&v[0]) {Ok(x)=>x,Err(e)=>{panic!("{}", e);}});
        }
        if flags == 4 {
            self.run(&match compile(&v[0]) {Ok(x)=>x,Err(e)=>{panic!("{}", e);}});
        }
        if flags == 5 {
            let dat = match compile(&v[0]) {Ok(x)=>x,Err(e)=>{panic!("{}", e);}};
            save(&v[1], &dat);
            self.run(&dat);
        }
    }
    fn process_arglst(&self, data: &Token) -> GResult<Vec<Token>> {
        let mut a: Vec<Token> = Vec::new();
        let lst: &Vec<Token> = match data {Token::Grp(x)=>x,_=>{return Err("INTERNAL TOKEN TYPE INCONSISTENCY: EXPECTED GRP, GOT OTHER".to_owned());}};
        let l: usize = lst.len();
        let mut i: usize = 0;
        while i < l {
            a.push(lst[i].clone());
            i += 1;
        }
        return Ok(a);
        // return Err("NOT IMPLEMENTED".to_owned());
    }
    fn in_unsafe(&self) -> GResult<()> {
        if self.flags[0] & 1 == 0 {
            return Err("UnsafeViolationError(attempted to invoke unsafe directive or function outside of an unsafe block)".to_owned());
        }
        return Ok(());
    }
    fn eval_directive(&mut self, data: &Vec<Token>, mut i: usize) -> EResult {
        if token_to_id(&data[i+1]) != 0 {
            return Err("INVALID TOKEN: EXPECTED GRP AFTER DIR, GOT OTHER".to_owned());
        }
        if match &data[i+2] {Token::Sym(c)=>match c {';'=>false,_=>true},_=>true} {
            return Err("MISSING SEMICOLON AFTER DIRECTIVE".to_owned());
        }
        let arglst: Vec<Token> = self.process_arglst(&data[i+1])?;
        match &data[i] {Token::Dir(dir)=>match dir.as_str() {
            "wrapper" => {if arglst.len() > 0 {return Err("ARG COUNT ERROR: EXPECTED NONE, GOT OTHER".to_owned());}},
            "wrap" => {},
            "parent" => {},
            "proto" => {},
            "implements" => {},
            "must_override" => {},
            "no_override" => {},
            "seperate" => {},
            "unsafe" => {
                self.flags[0] |= 1;
            },
            "is_unsafe" => {
                self.flags[0] |= 2;
            },
            "check" => {},
            "clamp" => {},
            "transmute" => {
                self.in_unsafe()?;
            },
            "test" => {println!("{arglst:?}");},
            _ => {return Err("UNRECOGNIZED DIRECTIVE".to_owned());},
        },_=>{return Err("INTERNAL TOKEN TYPE INCONSISTENCY: EXPECTED DIR, GOT OTHER".to_owned());}};
        return Ok(i);
    }
    fn preprocess(&mut self, data: &Vec<Token>, mut i: usize) -> EResult {
        let l: usize = data.len();
        while i < l {
            let tok: &Token = &data[i];
            let tid: u8 = token_to_id(tok);
            match tid {
                4 => {i = self.eval_directive(&data, i)?;},
                _ => {}
            };
            i += 1;
        }
        return Ok(i);
    }
    fn run(&mut self, data: &Vec<Token>) {
        println!("{data:?}");

        let l: usize = data.len();
        let mut i: usize = 0;
        i = match self.preprocess(&data, i) {Ok(j)=>j,Err(e)=>{println!("ERROR:\n{e}");return;}};
    }
}