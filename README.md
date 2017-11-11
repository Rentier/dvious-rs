# dvious-rs

[![Build Status](https://travis-ci.org/Rentier/dvious-rs.svg?branch=master)](https://travis-ci.org/Rentier/dvious-rs)

This project offers different tools for dealing with DVI (Device independent) files.

## Requirements

- **kpsewhich** and the fonts used in the DVI (e.g. for Ubuntu Linux, both are in the **texlive-base** package)

## Disassembler

    extern crate dvious;
    use dvious::dvi::disassembler::disassemble;
    use dvious::dvi::opcodes::OpCode;

    let mut f = File::open("foo.dvi").unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).unwrap();
    let result = disassemble(buffer);