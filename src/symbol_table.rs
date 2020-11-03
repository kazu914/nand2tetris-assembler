use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, String>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable::make_predefined_symbol_table()
    }

    pub fn add_entry(&mut self, symbol: String, address: String) {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &'static str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> String {
        self.table
            .get(symbol)
            .expect("Entry not found error")
            .to_string()
    }

    fn make_predefined_symbol_table() -> SymbolTable {
        let mut symbol_table = SymbolTable {
            table: HashMap::new(),
        };
        symbol_table.add_entry("SP".to_string(), format!("{:0>16b}", 0));
        symbol_table.add_entry("LCL".to_string(), format!("{:0>16b}", 1));
        symbol_table.add_entry("ARG".to_string(), format!("{:0>16b}", 2));
        symbol_table.add_entry("THIS".to_string(), format!("{:0>16b}", 3));
        symbol_table.add_entry("THAT".to_string(), format!("{:0>16b}", 4));

        symbol_table.add_entry("R0".to_string(), format!("{:0>16b}", 0));
        symbol_table.add_entry("R1".to_string(), format!("{:0>16b}", 1));
        symbol_table.add_entry("R2".to_string(), format!("{:0>16b}", 2));
        symbol_table.add_entry("R3".to_string(), format!("{:0>16b}", 3));
        symbol_table.add_entry("R4".to_string(), format!("{:0>16b}", 4));
        symbol_table.add_entry("R5".to_string(), format!("{:0>16b}", 5));
        symbol_table.add_entry("R6".to_string(), format!("{:0>16b}", 6));
        symbol_table.add_entry("R7".to_string(), format!("{:0>16b}", 7));
        symbol_table.add_entry("R8".to_string(), format!("{:0>16b}", 8));
        symbol_table.add_entry("R9".to_string(), format!("{:0>16b}", 9));
        symbol_table.add_entry("R10".to_string(), format!("{:0>16b}", 10));
        symbol_table.add_entry("R11".to_string(), format!("{:0>16b}", 11));
        symbol_table.add_entry("R12".to_string(), format!("{:0>16b}", 12));
        symbol_table.add_entry("R13".to_string(), format!("{:0>16b}", 13));
        symbol_table.add_entry("R14".to_string(), format!("{:0>16b}", 14));
        symbol_table.add_entry("R15".to_string(), format!("{:0>16b}", 15));

        symbol_table.add_entry("SCREEN".to_string(), format!("{:0>16b}", 16384));
        symbol_table.add_entry("KBD".to_string(), format!("{:0>16b}", 24576));
        symbol_table
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_symbol_table_predefined() {
        let table = SymbolTable::new();
        assert_eq!(table.contains("R0"), true);
        assert_eq!(table.get_address("R0"), format!("{:0>16b}", 0));

        assert_eq!(table.contains("KBD"), true);
        assert_eq!(table.get_address("KBD"), format!("{:0>16b}", 24576));
    }
    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::new();
        let address = "0000000001100100".to_string();
        table.add_entry("symbol".to_string(), address.clone());
        assert_eq!(table.contains("symbol"), true);
        assert_eq!(table.get_address("symbol"), address);
    }

    #[test]
    fn test_symbol_table_not_contain() {
        let table = SymbolTable::new();
        assert_eq!(table.contains("symbol"), false);
    }

    #[test]
    #[should_panic(expected = "Entry not found error")]
    fn test_symbol_table_panic() {
        let table = SymbolTable::new();
        table.get_address("symbol");
    }
}
