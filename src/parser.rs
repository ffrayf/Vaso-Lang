use crate::types::VasoType;
use crate::tokens::Token;
use crate::memory::MemoryStack;
use std::ops::Range;

// Actualizado: Ahora acepta tuplas (Token, Range<usize>)
pub fn extract_args(tokens: &Vec<(Token, Range<usize>)>, start_idx: usize, memory: &MemoryStack) -> (Vec<VasoType>, usize) {
    let mut args = Vec::new();
    let mut i = start_idx;
    
    // Accedemos a .0 porque ahora es una tupla (token, span)
    if let Some((Token::LParen, _)) = tokens.get(i) {
        i += 1;
        while i < tokens.len() {
            match &tokens[i] {
                (Token::RParen, _) => { i += 1; break; },
                (Token::StringLiteral(s), _) => args.push(VasoType::Str(s.clone())),
                (Token::NumberLiteral(n), _) => args.push(VasoType::Int(*n)),
                (Token::Identifier(n), _) => { 
                    if let Some(val) = memory.get(n) { 
                        args.push(val.clone()); 
                    } else {
                        args.push(VasoType::VBit(3, format!("Var '{}' not found", n)));
                    }
                },
                (Token::Comma, _) => {},
                _ => {}
            }
            i += 1;
        }
    }
    (args, i)
}