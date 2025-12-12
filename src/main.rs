use logos::Logos;
use colored::*;
use std::env;
use std::fs;
use std::collections::HashMap;

// Eliminamos std::process::Command para que no te de warning (ya que no lo usas en este demo)
// use std::process::Command; 

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    // Palabras clave
    #[token("fn")] Function,
    #[token("val")] Value,
    #[token("var")] Variable, 
    #[token("use")] Use,
    #[token("print")] Print,
    #[token("if")] If,
    #[token("else")] Else,
    #[token("while")] While, 

    // Tipos y Lógica Vaso
    #[token("vbit")] VBitType,
    #[regex("[0-4]", |lex| lex.slice().parse().ok(), priority = 2)] VBitLiteral(u8), 
    #[regex("[0-9]+", |lex| lex.slice().parse().ok())] NumberLiteral(i32),

    // Símbolos y Operadores
    #[token(":=")] Assign,
    #[token(":")] Colon,
    #[token(".")] Dot,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token(";")] Semicolon,
    #[token("==")] Equals,
    #[token("<")] LessThan,
    #[token(">")] GreaterThan,
    #[token("+")] Plus,
    #[token("+=")] PlusAssign, 
    #[token("-")] Minus,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())] Identifier(String),
    #[regex("\"[^\"]*\"", |lex| lex.slice().to_string())] StringLiteral(String),

    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[regex(r"//.*", logos::skip)] 
    Error,
}

fn main() {
    println!("{}", "\n🥃  VASO ENGINE v0.5 (Final Fix)".bold().cyan());
    println!("{}", "=====================================".cyan());

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return; }
    
    let code = fs::read_to_string(&args[1]).expect("❌ ERROR CRÍTICO: No encuentro el archivo .vs");
    let tokens: Vec<Token> = Token::lexer(&code).filter_map(Result::ok).collect();
    
    let mut memory: HashMap<String, String> = HashMap::new();
    let mut loop_stack: Vec<usize> = Vec::new(); 

    println!("DEBUG: Leí {} tokens (instrucciones).", tokens.len());
    println!("⚙️  Ejecutando...\n--- CONSOLA ---");

    let mut i = 0;
    loop {
        if i >= tokens.len() { break; }

        let current_token = &tokens[i];
        let mut consumed = false; 

        match current_token {
            
            // 1. PRINT
            Token::Print => {
                if let (Some(Token::LParen), Some(content), Some(Token::RParen)) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    match content {
                        Token::StringLiteral(s) => println!("{}", s.trim_matches('"')),
                        Token::Identifier(n) => if let Some(v) = memory.get(n) { println!("{}", v); },
                        Token::NumberLiteral(n) => println!("{}", n),
                        Token::VBitLiteral(n) => println!("{}", n),
                        _ => {}
                    }
                    i += 4; consumed = true;
                }
            },

            // 2. ASIGNACIÓN INICIAL
            Token::Value | Token::Variable => {
                if let (Some(Token::Identifier(name)), Some(Token::Assign), val_token) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    
                    let value = match val_token {
                        Some(Token::StringLiteral(s)) => Some(s.trim_matches('"').to_string()),
                        Some(Token::NumberLiteral(n)) => Some(n.to_string()),
                        Some(Token::VBitLiteral(n)) => Some(n.to_string()),
                        _ => None,
                    };
                    if let Some(v) = value {
                        memory.insert(name.clone(), v);
                        i += 4; consumed = true;
                    }
                }
            },

            // 3. REASIGNACIÓN (EL FIX ESTÁ AQUÍ)
            Token::Identifier(name) => {
                // Chequeamos si sigue un +=
                if let Some(Token::PlusAssign) = tokens.get(i+1) {
                    
                    // AQUI ESTA LA MAGIA: Aceptamos NumberLiteral O VBitLiteral
                    let increment_val = match tokens.get(i+2) {
                        Some(Token::NumberLiteral(n)) => Some(*n),
                        Some(Token::VBitLiteral(n)) => Some(*n as i32), // Convertimos vbit a numero
                        _ => None,
                    };

                    if let Some(num) = increment_val {
                         if let Some(val_str) = memory.get(name) {
                             if let Ok(val_int) = val_str.parse::<i32>() {
                                 let res = val_int + num;
                                 memory.insert(name.clone(), res.to_string());
                                 i += 3; consumed = true;
                             }
                         }
                    }
                }
            },

            // 4. WHILE
            Token::While => {
                let loop_start_index = i; 
                let mut cond = false;

                if let (Some(Token::Identifier(n)), Some(op), Some(Token::NumberLiteral(limit))) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    
                    if let Some(mem_val) = memory.get(n) {
                        if let Ok(current_val) = mem_val.parse::<i32>() {
                            match op {
                                Token::LessThan => cond = current_val < *limit,
                                Token::GreaterThan => cond = current_val > *limit,
                                Token::Equals => cond = current_val == *limit,
                                _ => {}
                            }
                        }
                    }
                }

                if cond {
                    loop_stack.push(loop_start_index);
                    i += 5; consumed = true;
                } else {
                    let mut brackets = 1;
                    i += 5; 
                    while i < tokens.len() && brackets > 0 {
                        i += 1;
                        match tokens.get(i) {
                            Some(Token::LBrace) => brackets += 1,
                            Some(Token::RBrace) => brackets -= 1,
                            _ => {}
                        }
                    }
                    consumed = true;
                }
            },

            // 5. CIERRE DE BUCLE
            Token::RBrace => {
                if let Some(return_index) = loop_stack.pop() {
                    i = return_index; consumed = true; 
                }
            },
            
            // 6. IF / USE (Simplificados para que no molesten en este demo)
            Token::If | Token::Use => { i+=1; }, 

            _ => {} 
        }

        if !consumed { i += 1; }
    }
    println!("---------------");
    println!("{}", "\n✅ Ejecución finalizada.".green());
}