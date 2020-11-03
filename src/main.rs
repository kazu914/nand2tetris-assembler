use assembler::code;
use assembler::parser;
use assembler::symbol_table;

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

pub struct Assembler {
    parser: parser::Parser,
    symbol_table: symbol_table::SymbolTable,
}

impl Assembler {
    pub fn new(lines: Vec<String>) -> Assembler {
        let parser = parser::Parser::new(lines);
        let symbol_table = symbol_table::SymbolTable::new();
        Assembler {
            parser,
            symbol_table,
        }
    }

    fn parse(&mut self) {
        while self.parser.has_more_commands {
            self.parser.advance();
            match &self.parser.command_type {
                None => continue,
                Some(parser::CommandType::Ccommand) => {
                    let dest = code::dest_to_binary(&self.parser.dest);
                    let comp = code::comp_to_binary(&self.parser.comp);
                    let jump = code::jump_to_binary(&self.parser.jump);
                    println!("111{}{}{}", comp, dest, jump);
                }
                Some(parser::CommandType::Acommand) => {
                    let symbol = code::decimal_to_binary(&self.parser.symbol);
                    println!("{}", symbol);
                }
                Some(parser::CommandType::Lcommand) => println!("pass:L"),
            }
        }
    }
}

fn main() {
    let lines = read_lines_from_file("./Add.asm");
    let mut assembler = Assembler::new(lines);
    assembler.parse()
}
