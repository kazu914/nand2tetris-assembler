use assembler::code;
use assembler::parser;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open.");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to read")).collect()
}

fn main() {
    let lines = read_lines_from_file("./Add.asm");
    let mut parser = parser::Parser::new(lines);
    while parser.has_more_commands {
        parser.advance();
        match &parser.command_type {
            None => continue,
            Some(parser::CommandType::Ccommand) => {
                let dest = code::dest_to_binary(&parser.dest);
                let comp = code::comp_to_binary(&parser.comp);
                let jump = code::jump_to_binary(&parser.jump);
                println!("111{}{}{}", comp, dest, jump);
            }
            Some(parser::CommandType::Acommand) => println!("pass:A"),
            Some(parser::CommandType::Lcommand) => println!("pass:L"),
        }
    }
}
