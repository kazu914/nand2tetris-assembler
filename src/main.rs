use assembler::code;
use assembler::parser;
use assembler::symbol_table;
use clap::Clap;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(Clap, Debug)]
#[clap(
    name = "Assembler in nand2teris project",
    version = "1.0.0",
    author = "Kazuaki Nomura",
    about = "Assembler of Hack computer"
)]
struct Opts {
    #[clap(name = "FILE")]
    asm_file: Option<String>,
}

fn read_lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open.");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to read")).collect()
}

pub struct Assembler {
    parser: parser::Parser,
    symbol_table: symbol_table::SymbolTable,
    rom_address: usize,
    result: Vec<String>,
}

impl Assembler {
    pub fn new(lines: Vec<String>) -> Assembler {
        let parser = parser::Parser::new(lines);
        let symbol_table = symbol_table::SymbolTable::new();
        Assembler {
            parser,
            symbol_table,
            rom_address: 0,
            result: Vec::new(),
        }
    }

    fn parse(&mut self) {
        self.first_path();
        self.parser.init_index();
        self.second_path();
    }

    fn first_path(&mut self) {
        while self.parser.has_more_commands {
            self.parser.advance();
            match &self.parser.command_type {
                None => continue,
                Some(parser::CommandType::Lcommand) => {
                    let symbol = self.parser.symbol.clone().unwrap();
                    let address = format!("{:0>16b}", self.rom_address);
                    self.symbol_table.add_entry(symbol, address);
                }
                Some(_) => {
                    self.rom_address += 1;
                }
            }
        }
    }

    fn second_path(&mut self) {
        while self.parser.has_more_commands {
            self.parser.advance();
            let res: Option<String> = match &self.parser.command_type {
                None => None,
                Some(parser::CommandType::Lcommand) => None,
                Some(parser::CommandType::Ccommand) => {
                    let dest = code::dest_to_binary(&self.parser.dest);
                    let comp = code::comp_to_binary(&self.parser.comp);
                    let jump = code::jump_to_binary(&self.parser.jump);
                    Some(format!("111{}{}{}", comp, dest, jump))
                }
                Some(parser::CommandType::Acommand) => self.process_a_command(),
            };

            match res {
                Some(line) => self.result.push(line),
                None => continue,
            }
        }
    }

    fn process_a_command(&mut self) -> Option<String> {
        match &self.parser.symbol {
            None => None,
            Some(symbol) => {
                if symbol.parse::<usize>().is_ok() {
                    Some(format!("{}", code::decimal_to_binary(symbol)))
                } else if self.symbol_table.contains(symbol.to_string()) {
                    Some(format!("{}", self.symbol_table.get_address(symbol)))
                } else {
                    self.symbol_table.add_entry(symbol.clone(), None);
                    Some(format!("{}", self.symbol_table.get_address(symbol)))
                }
            }
        }
    }

    pub fn output(&self, output_file: &str) {
        let mut output = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(output_file)
            .unwrap();
        for line in &self.result {
            writeln!(output, "{}", line).unwrap();
        }
    }
}

fn main() {
    let opts = Opts::parse();
    let asm_file = opts.asm_file.expect("filename is not specified");
    let lines = read_lines_from_file(&asm_file);
    let mut assembler = Assembler::new(lines);
    assembler.parse();
    assembler.output(asm_file.to_string().replace(".asm", ".hack").as_str());
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

        let symbol_address = format!("{:0>16b}", 2);

        let mut assembler = Assembler::new(lines);
        assembler.first_path();
        assert_eq!(assembler.symbol_table.contains("symbol".to_string()), true);
        assert_eq!(assembler.symbol_table.get_address("symbol"), symbol_address);
    }
}
