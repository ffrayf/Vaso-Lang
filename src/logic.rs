use crate::types::VasoType;
use crate::tokens::Token;

// --- FIX 1: Restauramos la función que pide stdlib.rs ---
pub fn check_errors(args: &Vec<VasoType>) -> Option<VasoType> {
    for arg in args {
        if let VasoType::VBit(3, msg) = arg {
            return Some(VasoType::VBit(3, msg.clone()));
        }
    }
    None
}

// --- FIX 2: Cambiamos i32 a u8 para coincidir con la definición de VBit ---
fn get_vbit_priority(level: u8) -> i32 {
    match level {
        3 => 100, // Error
        4 => 50,  // Unknown
        2 => 25,  // Loading
        1 => 10,  // On
        0 => 0,   // Off
        _ => -1
    }
}

pub fn apply_op(l: VasoType, r: &VasoType, op: &Token) -> VasoType {
    match (l, r) {
        // --- A. ARITMÉTICA PURA (Int vs Int) ---
        (VasoType::Int(a), VasoType::Int(b)) => {
            match op {
                Token::Plus | Token::PlusAssign => VasoType::Int(a + b),
                Token::Minus | Token::MinusAssign => VasoType::Int(a - b),
                Token::MulAssign => VasoType::Int(a * b),
                Token::DivAssign => if *b != 0 { VasoType::Int(a / b) } else { VasoType::VBit(3, "Division by Zero".to_string()) },
                
                // Comparaciones Numéricas
                Token::LessThan => if a < *b { VasoType::VBit(1, "".to_string()) } else { VasoType::VBit(0, "".to_string()) },
                Token::GreaterThan => if a > *b { VasoType::VBit(1, "".to_string()) } else { VasoType::VBit(0, "".to_string()) },
                Token::Equals => if a == *b { VasoType::VBit(1, "".to_string()) } else { VasoType::VBit(0, "".to_string()) },
                
                _ => VasoType::VBit(3, "Invalid Integer Op".to_string())
            }
        },

        // --- B. MANIPULACIÓN DE TEXTO (Str vs Str) ---
        (VasoType::Str(a), VasoType::Str(b)) => {
            match op {
                Token::Plus | Token::PlusAssign => VasoType::Str(format!("{}{}", a, b)),
                Token::Equals => if a == *b { VasoType::VBit(1, "".to_string()) } else { VasoType::VBit(0, "".to_string()) },
                _ => VasoType::VBit(3, "Invalid String Op".to_string())
            }
        },

        // --- C. MAGIA DE V-BITS (Dominancia de Estado) ---
        (VasoType::VBit(l_lvl, l_msg), VasoType::VBit(r_lvl, r_msg)) => {
            if matches!(op, Token::Plus | Token::PlusAssign) {
                let l_p = get_vbit_priority(l_lvl);
                let r_p = get_vbit_priority(*r_lvl);
                
                if l_p >= r_p {
                     VasoType::VBit(l_lvl, l_msg)
                } else {
                     VasoType::VBit(*r_lvl, r_msg.clone())
                }
            } else if matches!(op, Token::Equals) {
                 if l_lvl == *r_lvl { VasoType::VBit(1, "".to_string()) } else { VasoType::VBit(0, "".to_string()) }
            } else {
                VasoType::VBit(3, "Invalid VBit Op".to_string())
            }
        },

        // Caso D: INFECCIÓN
        (VasoType::VBit(lvl, msg), _) => VasoType::VBit(lvl, msg),
        (_, VasoType::VBit(lvl, msg)) => VasoType::VBit(*lvl, msg.clone()),
        
        _ => VasoType::VBit(3, "TypeError: Mismatch".to_string())
    }
}