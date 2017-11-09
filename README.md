# dvious-rs

[![Build Status](https://travis-ci.org/Rentier/dvious-rs.svg?branch=master)](https://travis-ci.org/Rentier/dvious-rs)

This project offers different tools for dealing with DVI (Device independent) files.

## Disassembler

    use dvi::disassembler::disassemble;

    let mut f = File::open("foo.dvi").unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).unwrap();
    let opcodes = disassemble(buffer);