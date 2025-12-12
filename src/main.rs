use logos::Logos;
use colored::*;
use std::env;
use std::fs;
use std::collections::HashMap;
use std::process::Command; // Necesario para llamar a Python

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
    #[token("match")] Match,
    #[token("case")] Case,

    // Tipos y Lógica Vaso
    #[token("vbit")] VBitType,
    #[regex("[0-4]", |lex| lex.slice().parse().ok())] VBitLiteral(u8), 

    // Símbolos
    #[token(":=")] Assign,
    #[token(":")] Colon,
    #[token(".")] Dot,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token(";")] Semicolon,
    #[token("==")] Equals,
    #[token("->")] Arrow,

    // Identificadores y Texto
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())] Identifier(String),
    #[regex("\"[^\"]*\"", |lex| lex.slice().to_string())] StringLiteral(String),

    // Ignorar basura
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[regex(r"//.*", logos::skip)] 
    Error,
}

// Función auxiliar: Convierte tokens de Vaso a texto que Python entienda
fn tokens_to_python(tokens: &[Token]) -> String {
    let mut script = String::new();
    for token in tokens {
        match token {
            Token::Identifier(s) => script.push_str(&format!("{} ", s)),
            
            // CORRECCIÓN AQUÍ: Antes era format!("\"{}\" ", s)
            // Ahora pasamos 's' directo porque el Lexer YA incluyó las comillas del archivo original.
            Token::StringLiteral(s) => script.push_str(&format!("{} ", s)), 
            
            Token::Print => script.push_str("print "),
            Token::LParen => script.push_str("("),
            Token::RParen => script.push_str(")"),
            Token::Assign => script.push_str("="), 
            Token::Dot => script.push_str("."),
            Token::Semicolon => script.push_str("\n"), 
            _ => {}
        }
    }
    script
}

fn main() {
    println!("{}", "\n🥃  VASO ENGINE v0.4 (The Chameleon Bridge)".bold().cyan());
    println!("{}", "===========================================".cyan());

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return; }
    let filename = &args[1];
    
    let code = fs::read_to_string(filename).unwrap_or_default();
    let tokens: Vec<Token> = Token::lexer(&code).filter_map(Result::ok).collect();
    let mut memory: HashMap<String, String> = HashMap::new();

    println!("⚙️  Ejecutando Vaso + Python...\n");
    println!("--- CONSOLA ---");

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Print => {
                if let (Some(Token::LParen), Some(content)) = (tokens.get(i+1), tokens.get(i+2)) {
                    match content {
                        Token::StringLiteral(s) => println!("{}", s.trim_matches('"')),
                        Token::Identifier(n) => if let Some(v) = memory.get(n) { println!("{}", v); },
                        _ => {}
                    }
                }
                i += 1; 
            },

            Token::Value => {
                if let (Some(Token::Identifier(name)), Some(Token::Assign), Some(Token::StringLiteral(val))) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    memory.insert(name.clone(), val.trim_matches('"').to_string());
                }
                i += 1;
            },

            Token::If => {
               let mut cond = false;
               if let (Some(Token::Identifier(n)), Some(Token::Equals), Some(Token::VBitLiteral(v))) = 
                      (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                   if let Some(mem_val) = memory.get(n) {
                       if mem_val == &v.to_string() { cond = true; }
                   }
               }
               if !cond {
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
               } else {
                   i += 5;
               }
            },

            // --- EL PUENTE CAMALEÓN (USAR PYTHON) ---
            Token::Use => {
                if let (Some(Token::LParen), Some(Token::Identifier(lang)), Some(Token::RParen), Some(Token::Colon), Some(Token::LBrace)) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3), tokens.get(i+4), tokens.get(i+5)) {
                    
                    if lang == "python" {
                        // 1. Extraer código dentro de las llaves
                        let mut python_tokens = Vec::new();
                        let mut j = i + 6;
                        let mut brackets = 1;
                        
                        while j < tokens.len() && brackets > 0 {
                            match tokens.get(j) {
                                Some(Token::RBrace) => brackets -= 1,
                                Some(Token::LBrace) => brackets += 1,
                                Some(t) => if brackets > 0 { python_tokens.push(t.clone()) },
                                None => break,
                            }
                            if brackets > 0 { j += 1; }
                        }

                        // 2. Traducir a Python
                        let raw_script = tokens_to_python(&python_tokens);
                        
                        // 3. Ejecutar comando de sistema
                        println!("{}", "   🐍 [Python Bridge Active]".yellow());
                        let output = Command::new("python")
                            .arg("-c") 
                            .arg(&raw_script)
                            .output();

                        match output {
                            Ok(o) => {
                                // Imprimir éxito (Verde)
                                let result = String::from_utf8_lossy(&o.stdout);
                                for line in result.lines() {
                                    println!("   >> {}", line.green());
                                }
                                // Imprimir error si hubo (Rojo)
                                let error = String::from_utf8_lossy(&o.stderr);
                                if !error.is_empty() {
                                    println!("{}", "   ⚠️ Python Error:".red());
                                    println!("   {}", error.red());
                                }
                            },
                            Err(_) => println!("{}", "   ❌ Error: No se pudo llamar a Python".red()),
                        }
                        
                        // 4. Saltar lo que ya ejecutamos para no repetirlo
                        i += 6 + python_tokens.len() + 1; 
                        continue;
                    }
                }
                i += 1;
            },

            _ => i += 1,
        }
    }
    println!("---------------");
    println!("{}", "\n✅ Ejecución finalizada.".green());
}