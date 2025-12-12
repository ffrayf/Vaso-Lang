use logos::Logos;
use colored::*;
use std::env;
use std::fs;

#[derive(Logos, Debug, PartialEq)]
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
    #[token(".")] Dot,      // <-- ¡AQUÍ ESTÁ EL PUNTO QUE FALTABA!
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

    // Ignorar basura (Espacios, Tabs, Saltos de linea Windows/Linux)
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[regex(r"//.*", logos::skip)] 
    Error,
}

fn main() {
    println!("{}", "\n🥃  VASO COMPILER v0.1 (Rusty Engine)".bold().cyan());
    println!("{}", "========================================".cyan());

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", "❌ Error: Falta el archivo.".red());
        return;
    }

    let filename = &args[1];
    println!("📄 Leyendo: {}", filename.yellow());

    let code = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            println!("{}", "❌ Error leyendo el archivo.".red());
            return;
        }
    };

    let lexer = Token::lexer(&code);

    println!("\n🔍  Análisis de Tokens:");
    println!("---------------------");

    for token in lexer {
        match token {
            // Palabras clave coloreadas
            Ok(Token::Function) => print!("{} ", "FN".blue().bold()),
            Ok(Token::Value) => print!("{} ", "VAL".green().bold()),
            Ok(Token::Variable) => print!("{} ", "VAR".green()),
            Ok(Token::Print) => print!("{} ", "PRINT".cyan()),
            Ok(Token::Use) => print!("{} ", "USE".yellow()),
            Ok(Token::If) => print!("{} ", "IF".magenta()),
            Ok(Token::Else) => print!("{} ", "ELSE".magenta()),
            Ok(Token::Match) => print!("{} ", "MATCH".magenta().bold()),
            Ok(Token::Case) => print!("{} ", "CASE".magenta()),
            
            // Tipos Vaso
            Ok(Token::VBitType) => print!("{} ", "TYPE:VBIT".purple().bold()),
            Ok(Token::VBitLiteral(n)) => print!("{} ", format!("<BIT:{}>", n).purple()),

            // Datos
            Ok(Token::Identifier(s)) => print!("ID({}) ", s),
            Ok(Token::StringLiteral(s)) => print!("\"{}\" ", s.italic()),

            // Símbolos limpios
            Ok(Token::Assign) => print!(":= "),
            Ok(Token::Colon) => print!(": "),
            Ok(Token::Dot) => print!("."), // <-- ¡AQUÍ IMPRIMIMOS EL PUNTO!
            Ok(Token::LBrace) => print!("{{ "),
            Ok(Token::RBrace) => print!("}} "),
            Ok(Token::LParen) => print!("( "),
            Ok(Token::RParen) => print!(") "),
            Ok(Token::Equals) => print!("== "),
            Ok(Token::Semicolon) => println!(";"), // Salto de línea

            // Cualquier otra cosa
            Ok(t) => print!("{:?} ", t),
            Err(_) => print!("{}", "?? ".red()),
        }
    }
    println!("\n---------------------\n✅ Compilación Exitosa.");
}