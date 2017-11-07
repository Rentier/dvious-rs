mod dvi;

use std::io::prelude::*;
use std::fs::File;

use dvi::disassembler::disassemble;

#[allow(unused_variables)]
fn main() {
    let mut f = File::open("foo.txt").unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();
    let result = disassemble(buffer);
}
