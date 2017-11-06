mod dvi;

use std::io::prelude::*;
use std::fs::File;

use dvi::disassembler::Disassembler;

fn main() {
    let mut f = File::open("foo.txt").unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    let mut disassembler = Disassembler::new(buffer);
    disassembler.disassemble();
}
