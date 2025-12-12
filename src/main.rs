use logos::Logos;
use colored::*;
use std::env;
use std::fs;
use std::collections::HashMap; // Importamos la memoria RAM

#[derive(Logos, Debug, PartialEq, Clone)] // Agregamos Clone para poder guardar tokens
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

fn main() {
    println!("{}", "\n🥃  VASO ENGINE v0.2 (Live Execution)".bold().cyan());
    println!("{}", "========================================".cyan());

    // 1. Leer argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", "❌ Error: Falta el archivo.".red());
        return;
    }
    let filename = &args[1];
    
    // 2. Leer archivo
    let code = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            println!("{}", "❌ Error leyendo el archivo.".red());
            return;
        }
    };

    // 3. Tokenizar (Convertimos todo el código a una lista lista para usar)
    let tokens: Vec<Token> = Token::lexer(&code)
        .filter_map(Result::ok) 
        .collect();

    // 4. Inicializar Memoria RAM
    let mut memory: HashMap<String, String> = HashMap::new();

    println!("⚙️  Ejecutando programa Vaso...\n");
    println!("--- SALIDA DEL PROGRAMA ---");

    // 5. BUCLE DE EJECUCIÓN (INTÉRPRETE)
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            
            // CASO: PRINT
            // Buscamos: print ( "texto" ) ;
            Token::Print => {
                if let Some(Token::LParen) = tokens.get(i+1) {
                    match tokens.get(i+2) {
                        // Si es texto literal: print("Hola")
                        Some(Token::StringLiteral(texto)) => {
                            println!("{}", texto.trim_matches('"'));
                        },
                        // Si es variable: print(x)
                        Some(Token::Identifier(nombre)) => {
                            if let Some(valor) = memory.get(nombre) {
                                println!("{}", valor);
                            } else {
                                println!("⚠️ Error: Variable '{}' no existe", nombre);
                            }
                        },
                        _ => {}
                    }
                    // Avanzamos el cursor (Print + ( + Contenido + ) + ;)
                    // Nota: Esto es simplificado, asume que siempre hay punto y coma
                }
                // Avanzamos manual para no quedarnos pegados
                i += 1; 
            },

            // CASO: ASIGNACIÓN (VAL)
            // Buscamos: val x := "valor" ;
            Token::Value => {
                if let (Some(Token::Identifier(nombre)), Some(Token::Assign), Some(Token::StringLiteral(valor))) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    
                    // Guardamos en la memoria del HashMap
                    memory.insert(nombre.clone(), valor.trim_matches('"').to_string());
                    
                    // Avanzamos más rápido porque ya leímos 4 tokens
                    i += 4;
                    continue; 
                }
                i += 1;
            },

            // Si es otra cosa, seguimos avanzando
            _ => i += 1,
        }
    }

    println!("---------------------------");
    println!("{}", "\n✅ Ejecución finalizada.".green());
}