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
    rom_address: usize,
}

impl Assembler {
    pub fn new(lines: Vec<String>) -> Assembler {
        let parser = parser::Parser::new(lines);
        let symbol_table = symbol_table::SymbolTable::new();
        Assembler {
            parser,
            symbol_table,
            rom_address: 0,
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

    fn first_path(&mut self) {
        while self.parser.has_more_commands {
            self.parser.advance();
            match &self.parser.command_type {
                None => continue,
                Some(parser::CommandType::Lcommand) => {
                    let symbol = self.parser.symbol.clone().unwrap();
                    let address = format!("{:0>16b}", self.rom_address + 1);
                    println!("{}{}", symbol, address);
                    self.symbol_table.add_entry(symbol, address);
                }
                Some(_) => {
                    self.rom_address += 1;
                    println!("{}", self.rom_address);
                }
            }
        }
    }
}

fn main() {
    let lines = read_lines_from_file("./Max.asm");
    let mut assembler = Assembler::new(lines);
    assembler.first_path()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_first_path() {
        let lines: Vec<String> = vec![
            "// symbol table test".to_string(),
            "@R0".to_string(),
            "D=M".to_string(),
            "(symbol)".to_string(),
            "D;JGT".to_string(),
        ];

        let symbol_address = format!("{:0>16b}", 3);

        let mut assembler = Assembler::new(lines);
        assembler.first_path();
        assert_eq!(assembler.symbol_table.contains("symbol"), true);
        assert_eq!(assembler.symbol_table.get_address("symbol"), symbol_address);
    }
}
