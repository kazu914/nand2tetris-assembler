use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

enum CommandType {
    Acommand,
    Ccommand,
    Lcommand,
}

struct Parser {
    has_more_commands: bool,
    commands: Vec<String>,
    index: usize,
    command_type: Option<CommandType>,
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
            command_type: None,
        }
    }

    pub fn advance(&mut self) {
        if self.has_more_commands {
            let flag = self.commands[self.index].chars().nth(0);
            match flag {
                Some('/') => (),
                Some('@') => self.command_type = Some(CommandType::Acommand),
                Some('(') => self.command_type = Some(CommandType::Lcommand),
                Some(_) => self.command_type = Some(CommandType::Ccommand),
                None => (),
            }
            self.parse();
            self.index = self.index + 1;
            self.has_more_commands = self.commands.len() > self.index;
        }
    }

    pub fn parse(&mut self) {
        match self.command_type {
            Some(CommandType::Acommand) => println!("A"),
            Some(CommandType::Lcommand) => println!("L"),
            Some(CommandType::Ccommand) => {
                let (dest, comp, jump) = self.parse_c();
                self.dest = dest;
                self.comp = comp;
                self.jump = jump;
            }
            None => (),
        }
    }

    pub fn parse_c(&mut self) -> (Option<String>, Option<String>, Option<String>) {
        if !self.commands[self.index].contains(";") {
            self.parse_c_without_semi_colon(&self.commands[self.index])
        } else if !self.commands[self.index].contains("=") {
            self.parse_c_without_equal(&self.commands[self.index])
        } else {
            self.parse_c_with_both(&self.commands[self.index])
        }
    }

    fn parse_c_without_semi_colon(
        &self,
        command: &String,
    ) -> (Option<String>, Option<String>, Option<String>) {
        let splited_equal: Vec<&str> = command.split("=").collect();
        (
            Some(splited_equal[0].to_string()),
            Some(splited_equal[1].to_string()),
            None,
        )
    }

    fn parse_c_without_equal(
        &self,
        command: &String,
    ) -> (Option<String>, Option<String>, Option<String>) {
        let splited_semicolon: Vec<&str> = command.split(";").collect();
        (
            None,
            Some(splited_semicolon[0].to_string()),
            Some(splited_semicolon[1].to_string()),
        )
    }

    fn parse_c_with_both(
        &self,
        command: &String,
    ) -> (Option<String>, Option<String>, Option<String>) {
        let splited_equal: Vec<&str> = command.split("=").collect();
        let splited_semicolon: Vec<&str> = splited_equal[1].split(";").collect();
        (
            Some(splited_equal[0].to_string()),
            Some(splited_semicolon[0].to_string()),
            Some(splited_semicolon[1].to_string()),
        )
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
