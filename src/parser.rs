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
    symbol: Option<String>,
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
            symbol: None,
            dest: None,
            comp: None,
            jump: None,
            command_type: None,
        }
    }

    pub fn advance(&mut self) {
        if self.has_more_commands {
            self.clear();
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

    fn clear(&mut self) {
        self.symbol = None;
        self.dest = None;
        self.comp = None;
        self.jump = None;
    }

    pub fn parse(&mut self) {
        let command = &self.commands[self.index];
        match self.command_type {
            Some(CommandType::Acommand) => self.symbol = Some(self.parse_a(&command)),
            Some(CommandType::Lcommand) => self.symbol = Some(self.parse_l(&command)),
            Some(CommandType::Ccommand) => {
                let (dest, comp, jump) = self.parse_c(&command);
                self.dest = dest;
                self.comp = comp;
                self.jump = jump;
            }
            None => (),
        }
    }

    fn parse_a(&self, command: &String) -> String {
        command.trim_matches('@').to_string()
    }

    fn parse_l(&self, command: &String) -> String {
        command
            .trim_matches('(')
            .to_string()
            .trim_matches(')')
            .to_string()
    }

    fn parse_c(&self, command: &String) -> (Option<String>, Option<String>, Option<String>) {
        if !command.contains(";") {
            self.parse_c_without_semi_colon(&command)
        } else if !command.contains("=") {
            self.parse_c_without_equal(&command)
        } else {
            self.parse_c_with_both(&command)
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_a() {
        let commands: Vec<String> = vec![];
        let parser = Parser::new(commands);
        let symbol = parser.parse_a(&"@value".to_string());
        assert_eq!(symbol, "value".to_string());
    }
    #[test]
    fn test_parse_l() {
        let commands: Vec<String> = vec![];
        let parser = Parser::new(commands);
        let symbol = parser.parse_l(&"(loop)".to_string());
        assert_eq!(symbol, "loop".to_string());
    }
    #[test]
    fn test_parse_c_without_equal() {
        let commands: Vec<String> = vec![];
        let parser = Parser::new(commands);
        let (dest, comp, jump) = parser.parse_c_without_equal(&"D;loop".to_string());
        assert_eq!(dest, None);
        assert_eq!(comp, Some("D".to_string()));
        assert_eq!(jump, Some("loop".to_string()));
    }
    #[test]
    fn test_parse_c_without_semi_colon() {
        let commands: Vec<String> = vec![];
        let parser = Parser::new(commands);
        let (dest, comp, jump) = parser.parse_c_without_semi_colon(&"D=M".to_string());
        assert_eq!(dest, Some("D".to_string()));
        assert_eq!(comp, Some("M".to_string()));
        assert_eq!(jump, None);
    }
    #[test]
    fn test_parse_c_with_both() {
        let commands: Vec<String> = vec![];
        let parser = Parser::new(commands);
        let (dest, comp, jump) = parser.parse_c_with_both(&"D=A;loop".to_string());
        assert_eq!(dest, Some("D".to_string()));
        assert_eq!(comp, Some("A".to_string()));
        assert_eq!(jump, Some("loop".to_string()));
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
