use logos::Logos;
use colored::*;
use std::env;
use std::fs;
use std::collections::HashMap;

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
    #[token("==")] Equals, // Importante para comparar
    #[token("->")] Arrow,

    // Identificadores y Texto
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())] Identifier(String),
    #[regex("\"[^\"]*\"", |lex| lex.slice().to_string())] StringLiteral(String),

    // Ignorar basura
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[regex(r"//.*", logos::skip)] 
    Error,
}

fn main() {
    println!("{}", "\n🥃  VASO ENGINE v0.3 (Logic Core)".bold().cyan());
    println!("{}", "======================================".cyan());

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", "❌ Error: Falta el archivo.".red());
        return;
    }
    let filename = &args[1];
    
    let code = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => { return; }
    };

    let tokens: Vec<Token> = Token::lexer(&code).filter_map(Result::ok).collect();
    let mut memory: HashMap<String, String> = HashMap::new();

    println!("⚙️  Ejecutando lógica condicional...\n");
    println!("--- CONSOLA ---");

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            
            // CASO: PRINT
            Token::Print => {
                if let Some(Token::LParen) = tokens.get(i+1) {
                    match tokens.get(i+2) {
                        Some(Token::StringLiteral(texto)) => println!("{}", texto.trim_matches('"')),
                        Some(Token::Identifier(nombre)) => {
                            if let Some(valor) = memory.get(nombre) {
                                println!("{}", valor);
                            } else {
                                println!("⚠️ Variable '{}' vacía", nombre);
                            }
                        },
                        _ => {}
                    }
                }
                i += 1; 
            },

            // CASO: ASIGNACIÓN (VAL)
            Token::Value => {
                // Opción A: val x := "texto"
                if let (Some(Token::Identifier(nombre)), Some(Token::Assign), Some(Token::StringLiteral(valor))) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    memory.insert(nombre.clone(), valor.trim_matches('"').to_string());
                    i += 4; continue; 
                }
                // Opción B: val x : vbit = 3 (Simulamos que guarda el número como texto por ahora)
                if let (Some(Token::Identifier(nombre)), Some(Token::Colon), Some(Token::VBitType), Some(Token::Assign), Some(Token::VBitLiteral(valor))) =
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3), tokens.get(i+4), tokens.get(i+5)) {
                     memory.insert(nombre.clone(), valor.to_string());
                     i += 6; continue;
                }
                i += 1;
            },

            // CASO: IF (EL CEREBRO NUEVO) 🧠
            // Estructura: if variable == valor {
            Token::If => {
                let mut condition_true = false;

                // 1. Analizamos la condición: if variable == valor
                if let (Some(Token::Identifier(var_name)), Some(Token::Equals), Some(Token::VBitLiteral(val_num))) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    
                    // Buscamos la variable en memoria
                    if let Some(memory_val) = memory.get(var_name) {
                        // Comparamos si lo que hay en memoria es igual a lo que pide el IF
                        if memory_val == &val_num.to_string() {
                            condition_true = true;
                        }
                    }
                }

                // 2. Tomamos la decisión
                if condition_true {
                    // Si es VERDADERO: Simplemente avanzamos y dejamos que el código se ejecute normal
                    // Avanzamos 'if', 'var', '==', 'valor', '{' (5 tokens)
                    i += 5; 
                } else {
                    // Si es FALSO: Tenemos que SALTAR hasta encontrar la llave de cierre '}'
                    // Ignoramos todo el bloque de código
                    let mut brackets = 1;
                    i += 5; // Entramos al bloque virtualmente
                    while i < tokens.len() && brackets > 0 {
                        i += 1;
                        match tokens.get(i) {
                            Some(Token::LBrace) => brackets += 1, // Encontramos otro { anidado
                            Some(Token::RBrace) => brackets -= 1, // Encontramos el } de cierre
                            _ => {}
                        }
                    }
                    // Al salir del while, estamos en el '}', así que seguimos desde ahí
                }
            },

            _ => i += 1,
        }
    }
    println!("---------------");
    println!("{}", "\n✅ Lógica finalizada.".green());
}