use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

extern crate dvious;
use dvious::dvi::disassembler::disassemble;
use dvious::dvi::opcodes::OpCode;

#[allow(unused_variables)]
fn main() {
    let app = App::new("dvious")
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

    let result = match app.subcommand() {
        ("disassemble", Some(sub)) => {
            let input = sub.value_of("INPUT").unwrap();
            disassemble_file(input)
        }
        _ => Ok(()),
    };

    match result {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => {
            eprintln!("An error occured: {:?}", why);
            process::exit(1);
        }
        Ok(file) => process::exit(0),
    };
}

fn disassemble_file(input: &str) -> Result<(), String> {
    let path = Path::new(input);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => return Err(format!("Could not open {}: {}", display, why.description())),
        Ok(file) => file,
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let opcodes = match disassemble(buffer) {
        Err(why) => return Err(format!("{:?}", why)),
        Ok(opcodes) => opcodes,
    };

    for opcode in opcodes {
        match opcode {
            OpCode::Pre { ref x, .. } => println!("{} | {}", opcode, String::from_utf8_lossy(x)),
            OpCode::FntDef { ref n, .. } => println!("{} | {}", opcode, String::from_utf8_lossy(n)),
            OpCode::Xxx { ref x, .. } => println!("{} | {}", opcode, String::from_utf8_lossy(x)),
            _ => println!("{}", opcode),
        }
    }

    Ok(())
}
