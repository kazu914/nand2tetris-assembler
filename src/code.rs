pub fn dest_to_binary(mnemonic: &Option<String>) -> String {
    match mnemonic.as_deref() {
        Some("M") => "001".to_string(),
        Some("D") => "010".to_string(),
        Some("MD") => "011".to_string(),
        Some("A") => "100".to_string(),
        Some("AM") => "101".to_string(),
        Some("AD") => "110".to_string(),
        Some("AMD") => "111".to_string(),
        None => "000".to_string(),
        _ => panic!("Unknown-Mnemonic error"),
    }
}

pub fn jump_to_binary(mnemonic: &Option<String>) -> String {
    match mnemonic.as_deref() {
        Some("JGT") => "001".to_string(),
        Some("JEQ") => "010".to_string(),
        Some("JGE") => "011".to_string(),
        Some("JLT") => "100".to_string(),
        Some("JNE") => "101".to_string(),
        Some("JLE") => "110".to_string(),
        Some("JMP") => "111".to_string(),
        None => "000".to_string(),
        _ => panic!("Unknown-Mnemonic error"),
    }
}

pub fn comp_to_binary(mnemonic: &Option<String>) -> String {
    match mnemonic.as_deref() {
        Some("0") => "0101010".to_string(),
        Some("1") => "0111111".to_string(),
        Some("-1") => "0111010".to_string(),
        Some("D") => "0001100".to_string(),
        Some("A") => "0110000".to_string(),
        Some("M") => "1110000".to_string(),
        Some("!D") => "0001101".to_string(),
        Some("!A") => "0110001".to_string(),
        Some("!M") => "1110001".to_string(),
        Some("-D") => "0001111".to_string(),
        Some("-A") => "0110011".to_string(),
        Some("-M") => "1110011".to_string(),
        Some("D+1") => "0011111".to_string(),
        Some("A+1") => "0110111".to_string(),
        Some("M+1") => "1110111".to_string(),
        Some("D-1") => "0001110".to_string(),
        Some("A-1") => "0110010".to_string(),
        Some("M-1") => "1110010".to_string(),
        Some("D+A") => "0000010".to_string(),
        Some("D+M") => "1000010".to_string(),
        Some("D-A") => "0010011".to_string(),
        Some("D-M") => "1010011".to_string(),
        Some("A-D") => "0000111".to_string(),
        Some("M-D") => "1000111".to_string(),
        Some("D&A") => "0000000".to_string(),
        Some("D&M") => "1000000".to_string(),
        Some("D|A") => "0010101".to_string(),
        Some("D|M") => "1010101".to_string(),
        _ => panic!("Unknown-Mnemonic error"),
    }
}

pub fn decimal_to_binary(decimal: &String) -> String {
    let parsed_decimal = decimal.parse::<i32>().expect("Invalid decimal");
    format!("{:0>16b}", parsed_decimal)
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dest_to_binary() {
        assert_eq!("000".to_string(), dest_to_binary(&None));
    }

    #[test]
    #[should_panic(expected = "Unknown-Mnemonic error")]
    fn test_dest_to_binary_panic() {
        dest_to_binary(&Some("UnknownMnemonic".to_string()));
    }

    #[test]
    fn test_jump_to_binary() {
        assert_eq!("000".to_string(), jump_to_binary(&None));
    }

    #[test]
    #[should_panic(expected = "Unknown-Mnemonic error")]
    fn test_jump_to_binary_panic() {
        jump_to_binary(&Some("UnknownMnemonic".to_string()));
    }

    #[test]
    #[should_panic(expected = "Unknown-Mnemonic error")]
    fn test_comp_to_binary() {
        comp_to_binary(&Some("UnknownMnemonic".to_string()));
    }

    #[test]
    fn test_decimal_to_binary() {
        assert_eq!(decimal_to_binary(&"100".to_string()), "0000000001100100");
    }
}
