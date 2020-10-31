pub fn dest_to_binary(mnemonic: Option<String>) -> String {
    match mnemonic.unwrap_or("unhandled".to_string()).as_str() {
        "M" => "001".to_string(),
        "D" => "010".to_string(),
        "MD" => "011".to_string(),
        "A" => "100".to_string(),
        "AM" => "101".to_string(),
        "AD" => "110".to_string(),
        "AMD" => "111".to_string(),
        "unhandled" => "000".to_string(),
        _ => panic!("Unknown-Mnemonic error"),
    }
}

pub fn jump_to_binary(mnemonic: Option<String>) -> String {
    match mnemonic.unwrap_or("unhandled".to_string()).as_str() {
        "JGT" => "001".to_string(),
        "JEQ" => "010".to_string(),
        "JGE" => "011".to_string(),
        "JLT" => "100".to_string(),
        "JNE" => "101".to_string(),
        "JLE" => "110".to_string(),
        "JMP" => "111".to_string(),
        "unhandled" => "000".to_string(),
        _ => panic!("Unknown-Mnemonic error"),
    }
}

pub fn comp_to_binary(mnemonic: Option<String>) -> String {
    match mnemonic.unwrap_or("unhandled".to_string()).as_str() {
        "0" => "0101010".to_string(),
        "1" => "0111111".to_string(),
        "-1" => "0111010".to_string(),
        "D" => "0001100".to_string(),
        "A" => "0110000".to_string(),
        "M" => "1110000".to_string(),
        "!D" => "0001101".to_string(),
        "!A" => "0110001".to_string(),
        "!M" => "1110001".to_string(),
        "-D" => "0001111".to_string(),
        "-A" => "0110011".to_string(),
        "-M" => "1110011".to_string(),
        "D+1" => "0011111".to_string(),
        "A+1" => "0110111".to_string(),
        "M+1" => "1110111".to_string(),
        "D-1" => "0001110".to_string(),
        "A-1" => "0110010".to_string(),
        "M-1" => "1110010".to_string(),
        "D+A" => "0000010".to_string(),
        "D+M" => "1000010".to_string(),
        "D-A" => "0010011".to_string(),
        "D-M" => "1010011".to_string(),
        "A-D" => "0000111".to_string(),
        "M-D" => "1000111".to_string(),
        "D&A" => "0000000".to_string(),
        "D&M" => "1000000".to_string(),
        "D|A" => "0010101".to_string(),
        "D|M" => "!010101".to_string(),
        _ => panic!("Unknown-Mnemonic error"),
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dest_to_binary() {
        assert_eq!("000".to_string(), dest_to_binary(None));
    }

    #[test]
    #[should_panic(expected = "Unknown-Mnemonic error")]
    fn test_dest_to_binary_panic() {
        dest_to_binary(Some("UnknownMnemonic".to_string()));
    }

    #[test]
    fn test_jump_to_binary() {
        assert_eq!("000".to_string(), jump_to_binary(None));
    }

    #[test]
    #[should_panic(expected = "Unknown-Mnemonic error")]
    fn test_jump_to_binary_panic() {
        jump_to_binary(Some("UnknownMnemonic".to_string()));
    }

    #[test]
    #[should_panic(expected = "Unknown-Mnemonic error")]
    fn test_comp_to_binary() {
        comp_to_binary(Some("UnknownMnemonic".to_string()));
    }
}
