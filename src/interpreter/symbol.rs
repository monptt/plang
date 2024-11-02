use core::fmt;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum SymbolName {
    VariableName(String),
}

impl fmt::Display for SymbolName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolName::VariableName(name) => {
                return write!(f, "{}", name);
            }
        }
    }
}
