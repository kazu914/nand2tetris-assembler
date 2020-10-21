use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Parser {
    has_more_commands: bool,
    commands: Vec<String>,
    index: usize,
    dest: Option<String>,
    comp: Option<String>,
    jump: Option<String>,
}

impl Parser {
    pub fn new(commands: Vec<String>) -> Parser {
        let has_more_commands = commands.len() > 0;
        Parser {
            commands,
            index: 0,
            has_more_commands,
            dest: None,
            comp: None,
            jump: None,
        }
    }

    pub fn advance(&mut self) {
        if self.has_more_commands {
            let flag = self.commands[self.index].chars().nth(0);
            match flag {
                Some('/') => (),
                Some('@') => println!("@"), // TODO
                Some('(') => println!("("), // TODO
                Some(_) => {
                    self.parse_c();
                    println!("{:?}{:?}{:?}", self.dest, self.comp, self.jump)
                }
                None => (),
            }
            self.index = self.index + 1;
            self.has_more_commands = self.commands.len() > self.index;
        }
    }

    pub fn parse_c(&mut self) {
        if !self.commands[self.index].contains(";") {
            let splited_equal: Vec<&str> = self.commands[self.index].split("=").collect();
            self.dest = Some(splited_equal[0].to_string());
            self.comp = Some(splited_equal[1].to_string());
            self.jump = None;
        } else if !self.commands[self.index].contains("=") {
            let splited_semicolon: Vec<&str> = self.commands[self.index].split(";").collect();
            self.dest = None;
            self.comp = Some(splited_semicolon[0].to_string());
            self.jump = Some(splited_semicolon[1].to_string());
        } else {
            let splited_equal: Vec<&str> = self.commands[self.index].split("=").collect();
            let splited_semicolon: Vec<&str> = splited_equal[1].split(";").collect();
            self.dest = Some(splited_equal[0].to_string());
            self.comp = Some(splited_semicolon[0].to_string());
            self.jump = Some(splited_semicolon[1].to_string());
        }
    }
}
fn read_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open.");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to read")).collect()
}

pub fn main() {
    let lines = read_lines_from_file("./Add.asm");
    let mut parser = Parser::new(lines);

    while parser.has_more_commands {
        parser.advance();
    }
}
