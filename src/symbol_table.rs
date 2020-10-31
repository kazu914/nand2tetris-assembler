use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<&'static str, &'static str>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, symbol: &'static str, address: &'static str) {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &'static str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &'static str) -> &'static str {
        self.table.get(symbol).expect("Entry not found error")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::new();
        let address = "0000000001100100";
        table.add_entry("symbol", address);
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
