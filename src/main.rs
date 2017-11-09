use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;
use std::process;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

mod dvi;
use dvi::disassembler::disassemble;

#[allow(unused_variables)]
fn main() {
    let matches = App::new("dvious")
        .version("0.1.0")
        .author("Jan-Christoph Klie <git@mrklie.com>")
        .about("Toolkit for DVI files")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("disassemble")
                .about("Disassembles the specified DVI file")
                .version("0.1.0")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("disassemble") {
        let input = matches.value_of("INPUT").unwrap();
        disassemble_file(input);
    }
}

fn disassemble_file(input: &str) {
    let path = Path::new(input);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => {
            writeln!(
                io::stderr(),
                "Couldn't open '{}': {}",
                display,
                why.description()
            ).unwrap();
            process::exit(1);
        }
        Ok(file) => file,
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let result = disassemble(buffer);

    for opcode in result {
        println!("{}", opcode);
    }
}
