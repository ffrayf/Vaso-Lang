use serde_json::Value;
use colored::*;

#[derive(Debug, Clone, PartialEq)]
pub enum VasoType {
    Int(i32),
    VBit(u8, String), 
    Str(String),
    Json(Value),
    Function(usize, Vec<String>), 
    // --- NUEVO: Listas ---
    List(Vec<VasoType>),
}

impl std::fmt::Display for VasoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VasoType::Int(n) => write!(f, "{}", n),
            VasoType::Str(s) => write!(f, "{}", s),
            VasoType::Json(v) => write!(f, "{}", v),
            VasoType::Function(_, args) => write!(f, "fn({:?})", args),
            // Formato de lista: [1, 2, 3]
            VasoType::List(vec) => {
                write!(f, "[")?;
                for (i, val) in vec.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            },
            VasoType::VBit(v, msg) => {
                let s = match v {
                    0 => "off".normal(),
                    1 => "on".green(),
                    2 => "loading".yellow(),
                    3 => "error".red().bold(),
                    4 => "unknown".magenta(),
                    _ => "unknown".normal()
                };
                if msg.is_empty() { write!(f, "{}", s) } else { write!(f, "{}(\"{}\")", s, msg) }
            }
        }
    }
}

pub struct StructDef {
    #[allow(dead_code)]
    pub fields: Vec<(String, String)>,
}