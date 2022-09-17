use crate::files::{save, load, compile};
use crate::data::Token;

pub struct Runner {
}

impl Runner {
    pub fn run (&mut self, v: Vec<String>) {
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
        if flags == 1 {
            save(&v[1], &match compile(&v[0]) {Ok(x)=>x,Err(e)=>{panic!("{}", e);}});
        }
        if flags == 2 {
            self.start(match load(&v[0]) {Ok(x)=>x,Err(e)=>{panic!("{}", e);}});
        }
        if flags == 4 {
            self.start(match compile(&v[0]) {Ok(x)=>x,Err(e)=>{panic!("{}", e);}});
        }
        if flags == 5 {
            let dat = match compile(&v[0]) {Ok(x)=>x,Err(e)=>{panic!("{}", e);}};
            save(&v[1], &dat);
            self.start(dat);
        }
    }
    fn start(&mut self, data: Vec<Token>) {
        println!("{data:?}");
    }
}