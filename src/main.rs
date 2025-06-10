use lang_4 as this;

use this::runner::{Runner};
use std::env::{args,Args};

fn main () {
    let mut a: Args = args();
    a.next();
    let mut v: Vec<String> = Vec::<String>::with_capacity(a.len());
    v.extend(a);
    Runner::new().start(v);
}