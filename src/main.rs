use logos::Logos;
use colored::*;
use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use std::process::Command; 
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

// --- DEFINICIÓN DE TIPOS ---
#[derive(Debug, Clone, PartialEq)]
enum VasoType {
    Int(i32),
    VBit(u8, String), 
    Str(String),
}

impl std::fmt::Display for VasoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VasoType::Int(n) => write!(f, "{}", n),
            VasoType::Str(s) => write!(f, "{}", s),
            VasoType::VBit(v, msg) => {
                let s = match v {
                    0 => "off".normal(),
                    1 => "on".green(),
                    2 => "loading".yellow(),
                    3 => "error".red().bold(),
                    4 => "unknown".magenta(),
                    _ => "unknown".normal()
                };
                if msg.is_empty() {
                    write!(f, "{}", s)
                } else {
                    write!(f, "{}(\"{}\")", s, msg) 
                }
            }
        }
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    #[token("fn")] Function,
    #[token("val")] Value,
    #[token("var")] Variable, 
    #[token("mold")] Mold,
    #[token("new")] New,
    #[token("print")] Print,
    #[token("input")] Input, 
    #[token("if")] If,
    #[token("else")] Else,
    #[token("while")] While, 
    #[token("match")] Match, 
    #[token("use")] Use, 

    #[token("off")] LitOff,
    #[token("on")] LitOn,
    #[token("loading")] LitLoading,
    #[token("error")] LitError,
    #[token("unknown")] LitUnknown,

    #[token("vbit")] TypeVBit,
    #[token("int")] TypeInt, 
    
    #[regex("-?[0-9]+", |lex| lex.slice().parse().ok())] NumberLiteral(i32), 

    #[token(":=")] AssignPascal,
    #[token("=")]  AssignC,
    #[token("=>")] Arrow,

    #[token(":")] Colon,
    #[token(";")] Semicolon,
    #[token(".")] Dot,
    #[token(",")] Comma,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("==")] Equals,
    #[token("!=")] NotEquals, 
    #[token("<")] LessThan,
    #[token(">")] GreaterThan,
    
    #[token("+=")] PlusAssign, 
    #[token("-=")] MinusAssign, 
    #[token("*=")] MulAssign,   
    #[token("/=")] DivAssign,   
    #[token("+")] Plus,
    #[token("-")] Minus,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())] Identifier(String),
    #[regex("\"[^\"]*\"", |lex| lex.slice().to_string())] StringLiteral(String),

    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[regex(r"//.*", logos::skip)] 
    Error,
}

struct StructDef {
    fields: Vec<(String, String)>,
}

// --- STANDARD LIBRARY CLEANED ---
fn call_std_function(module: &str, func: &str, args: Vec<VasoType>) -> VasoType {
    match module {
        "Time" => {
            match func {
                "now" => {
                    let start = SystemTime::now();
                    let since = start.duration_since(UNIX_EPOCH).expect("Time fail");
                    VasoType::Int(since.as_secs() as i32)
                },
                _ => VasoType::VBit(3, format!("Time.{} not found", func))
            }
        },
        "Math" => {
            match func {
                "random" => {
                    let mut rng = rand::thread_rng();
                    VasoType::Int(rng.gen_range(0..100))
                }, 
                _ => VasoType::VBit(3, format!("Math.{} not found", func))
            }
        },
        "File" => {
            match func {
                "read" => {
                    if let Some(VasoType::Str(path)) = args.get(0) {
                        match fs::read_to_string(path) {
                            Ok(content) => VasoType::Str(content),
                            Err(e) => VasoType::VBit(3, format!("IO Error: {}", e))
                        }
                    } else { VasoType::VBit(3, "Arg Error".to_string()) }
                },
                "write" => {
                     if let (Some(VasoType::Str(path)), Some(VasoType::Str(content))) = (args.get(0), args.get(1)) {
                        match fs::write(path, content) {
                            Ok(_) => VasoType::VBit(1, "File Written".to_string()),
                            Err(e) => VasoType::VBit(3, format!("Write Error: {}", e))
                        }
                     } else { VasoType::VBit(3, "Arg Error".to_string()) }
                },
                _ => VasoType::VBit(3, format!("File.{} not found", func))
            }
        },
        "Sys" => {
            match func {
                "os" => VasoType::Str(env::consts::OS.to_string()),
                "exec" => {
                    if let (Some(VasoType::Str(cmd)), Some(VasoType::Str(arg1))) = (args.get(0), args.get(1)) {
                        // FIX: Logica limpia para Windows/Unix sin variables unused
                        let is_windows = cfg!(target_os = "windows");
                        
                        let mut command = if is_windows {
                            let mut c = Command::new("cmd");
                            c.arg("/C").arg(format!("{} {}", cmd, arg1));
                            c
                        } else {
                            let mut c = Command::new(cmd);
                            c.arg(arg1);
                            c
                        };

                        match command.output() {
                            Ok(output) => {
                                if output.status.success() {
                                    // EXITO: Devolvemos ON para que la aritmetica funcione (test + build = on)
                                    VasoType::VBit(1, "".to_string()) 
                                } else {
                                    let err = String::from_utf8_lossy(&output.stderr).to_string();
                                    let msg = if err.is_empty() { String::from_utf8_lossy(&output.stdout).to_string() } else { err };
                                    VasoType::VBit(3, format!("CMD Failed: {}", msg.trim()))
                                }
                            },
                            Err(e) => VasoType::VBit(3, format!("Exec Error: {}", e))
                        }
                    } else {
                        VasoType::VBit(3, "Sys.exec needs (cmd, arg)".to_string())
                    }
                },
                _ => VasoType::VBit(3, "Sys func error".to_string())
            }
        },
        _ => VasoType::VBit(3, format!("Module {} not found", module))
    }
}

fn apply_dominance(val1: &VasoType, val2: &VasoType) -> VasoType {
    let get_rank = |v: u8| -> u8 {
        match v { 3 => 5, 4 => 4, 2 => 3, 1 => 2, 0 => 1, _ => 0 }
    };

    match (val1, val2) {
        (VasoType::VBit(v1, m1), VasoType::VBit(v2, m2)) => {
            if get_rank(*v1) >= get_rank(*v2) { VasoType::VBit(*v1, m1.clone()) } else { VasoType::VBit(*v2, m2.clone()) }
        },
        (VasoType::VBit(v, m), VasoType::Int(_)) => VasoType::VBit(*v, m.clone()),
        (VasoType::Int(_), VasoType::VBit(v, m)) => VasoType::VBit(*v, m.clone()),
        (VasoType::VBit(v, m), VasoType::Str(_)) => VasoType::VBit(*v, m.clone()),
        (VasoType::Str(_), VasoType::VBit(v, m)) => VasoType::VBit(*v, m.clone()),
        (VasoType::Int(n1), VasoType::Int(n2)) => VasoType::Int(n1 + n2),
        (VasoType::Str(s1), VasoType::Str(s2)) => VasoType::Str(format!("{}{}", s1, s2)),
        _ => VasoType::VBit(3, "Type Mismatch".to_string())
    }
}

fn extract_args(tokens: &Vec<Token>, start_idx: usize, memory: &HashMap<String, VasoType>) -> (Vec<VasoType>, usize) {
    let mut args = Vec::new();
    let mut i = start_idx;
    if let Some(Token::LParen) = tokens.get(i) {
        i += 1;
        while i < tokens.len() {
            match &tokens[i] {
                Token::RParen => { i += 1; break; },
                Token::StringLiteral(s) => args.push(VasoType::Str(s.trim_matches('"').to_string())),
                Token::NumberLiteral(n) => args.push(VasoType::Int(*n)),
                Token::Identifier(n) => { if let Some(val) = memory.get(n) { args.push(val.clone()); } },
                Token::Comma => {},
                _ => {}
            }
            i += 1;
        }
    }
    (args, i)
}

fn main() {
    println!("{}", "\n🥃  VASO ENGINE v1.4.1 (Clean)".bold().cyan());
    println!("{}", "================================".cyan());

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { println!("❌ Uso: cargo run <archivo.vs>"); return; }
    
    let code = fs::read_to_string(&args[1]).expect("❌ ERROR: No encuentro el archivo .vs");
    let tokens: Vec<Token> = Token::lexer(&code).filter_map(Result::ok).collect();
    
    let mut memory: HashMap<String, VasoType> = HashMap::new();
    let mut structs: HashMap<String, StructDef> = HashMap::new();
    
    let mut temp_i = 0;
    while temp_i < tokens.len() {
        if let Token::Mold = tokens[temp_i] {
            if let Some(Token::Identifier(struct_name)) = tokens.get(temp_i+1) {
                let mut fields = Vec::new();
                let mut j = temp_i + 3; 
                while j < tokens.len() {
                    match &tokens[j] {
                        Token::RBrace => break,
                        Token::Identifier(field_name) => {
                            if let (Some(Token::Colon), Some(type_token)) = (tokens.get(j+1), tokens.get(j+2)) {
                                fields.push((field_name.clone(), format!("{:?}", type_token)));
                            }
                        },
                        _ => {}
                    }
                    j += 1;
                }
                structs.insert(struct_name.clone(), StructDef { fields });
            }
        }
        temp_i += 1;
    }

    let mut i = 0;
    loop {
        if i >= tokens.len() { break; }
        let current_token = &tokens[i];
        let mut consumed = false; 

        match current_token {
            Token::Print => {
                if let Some(Token::LParen) = tokens.get(i+1) {
                    if let (Some(Token::Identifier(obj)), Some(Token::Dot), Some(Token::Identifier(prop)), Some(Token::RParen)) = 
                           (tokens.get(i+2), tokens.get(i+3), tokens.get(i+4), tokens.get(i+5)) {
                        let key = format!("{}.{}", obj, prop);
                        if let Some(v) = memory.get(&key) { println!("{}", v); } else { println!("null"); }
                        i += 6; consumed = true;
                    }
                    else if let (Some(Token::Identifier(n)), Some(Token::RParen)) = (tokens.get(i+2), tokens.get(i+3)) {
                        if let Some(v) = memory.get(n) { println!("{}", v); } else { println!(""); } 
                        i += 4; consumed = true;
                    }
                    else if let (Some(Token::StringLiteral(s)), Some(Token::RParen)) = (tokens.get(i+2), tokens.get(i+3)) {
                         println!("{}", s.trim_matches('"'));
                         i += 4; consumed = true;
                    }
                    else if let (Some(token_lit), Some(Token::RParen)) = (tokens.get(i+2), tokens.get(i+3)) {
                         match token_lit {
                             Token::LitOff => println!("off"),
                             Token::LitOn => println!("on"),
                             Token::LitLoading => println!("loading"),
                             Token::LitError => println!("error"),
                             Token::LitUnknown => println!("unknown"),
                             _ => {}
                         }
                         i += 4; consumed = true;
                    }
                }
            },
            Token::Match => {
                if let (Some(Token::Identifier(var_name)), Some(Token::LBrace)) = (tokens.get(i+1), tokens.get(i+2)) {
                    let current_val = memory.get(var_name).cloned().unwrap_or(VasoType::VBit(4, "".to_string())); 
                    let mut j = i + 3;
                    let mut match_executed = false;
                    while j < tokens.len() {
                        if let Token::RBrace = tokens[j] { break; } 
                        if let (Some(case_token), Some(Token::Arrow), Some(Token::LBrace)) = (tokens.get(j), tokens.get(j+1), tokens.get(j+2)) {
                            let matches_case = match (case_token, &current_val) {
                                (Token::LitOn, VasoType::VBit(1, _)) => true,
                                (Token::LitOff, VasoType::VBit(0, _)) => true,
                                (Token::LitLoading, VasoType::VBit(2, _)) => true,
                                (Token::LitError, VasoType::VBit(3, _)) => true,
                                (Token::LitUnknown, VasoType::VBit(4, _)) => true,
                                _ => false
                            };
                            if matches_case && !match_executed {
                                match_executed = true;
                                let mut k = j + 3;
                                while k < tokens.len() {
                                    if let Token::RBrace = tokens[k] { break; }
                                    match &tokens[k] {
                                        Token::Print => {
                                             if let (Some(Token::LParen), Some(Token::StringLiteral(s)), Some(Token::RParen)) = 
                                                    (tokens.get(k+1), tokens.get(k+2), tokens.get(k+3)) {
                                                 println!("{}", s.trim_matches('"'));
                                             } else if let (Some(Token::LParen), Some(Token::Identifier(n)), Some(Token::RParen)) = 
                                                    (tokens.get(k+1), tokens.get(k+2), tokens.get(k+3)) {
                                                 if let Some(v) = memory.get(n) { println!("{}", v); }
                                             }
                                        },
                                        Token::Variable => {
                                            if let (Some(Token::Identifier(name)), Some(assign), Some(val_token)) = 
                                                   (tokens.get(k+1), tokens.get(k+2), tokens.get(k+3)) {
                                                if matches!(assign, Token::AssignC | Token::AssignPascal) {
                                                    if let (Token::Identifier(module), Some(Token::Dot), Some(Token::Identifier(func))) = 
                                                           (val_token, tokens.get(k+4), tokens.get(k+5)) {
                                                         let (args, _) = extract_args(&tokens, k+6, &memory);
                                                         let result = call_std_function(module, func, args);
                                                         memory.insert(name.clone(), result);
                                                    }
                                                }
                                            }
                                        },
                                        Token::Identifier(_n) => { 
                                             if let (Some(assign), Some(val_token)) = (tokens.get(k+1), tokens.get(k+2)) {
                                                if matches!(assign, Token::AssignC | Token::AssignPascal) {
                                                    if let (Token::Identifier(module), Some(Token::Dot), Some(Token::Identifier(func))) = 
                                                        (val_token, tokens.get(k+3), tokens.get(k+4)) {
                                                            let (args, _) = extract_args(&tokens, k+5, &memory);
                                                            let res = call_std_function(module, func, args);
                                                            if let (Some(Token::Print), Some(Token::LParen)) = (tokens.get(k+8), tokens.get(k+9)) {
                                                                println!("{}", res); 
                                                            }
                                                    }
                                                }
                                            }
                                        } 
                                        _ => {}
                                    }
                                    k += 1;
                                }
                            }
                            let mut k = j + 3;
                            let mut brackets = 1;
                            while k < tokens.len() && brackets > 0 {
                                k += 1;
                                match tokens[k] {
                                    Token::LBrace => brackets += 1,
                                    Token::RBrace => brackets -= 1,
                                    _ => {}
                                }
                            }
                            j = k + 1; 
                            continue;
                        }
                        j += 1;
                    }
                    let mut brackets = 1;
                    i += 3;
                    while i < tokens.len() && brackets > 0 {
                        match tokens[i] {
                            Token::LBrace => brackets += 1,
                            Token::RBrace => brackets -= 1,
                            _ => {}
                        }
                        i += 1;
                    }
                    consumed = true;
                    i -= 1;
                }
            },
            Token::If => {
                let mut cond = false;
                let mut offset = 1;
                if let Some(Token::Identifier(n1)) = tokens.get(i+1) {
                    if let (Some(Token::GreaterThan), Some(Token::Identifier(n2))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let (Some(VasoType::Int(v1)), Some(VasoType::Int(v2))) = (memory.get(n1), memory.get(n2)) {
                             if v1 > v2 { cond = true; }
                         }
                         offset = 3;
                    } else if let (Some(Token::GreaterThan), Some(Token::NumberLiteral(v2))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let Some(VasoType::Int(v1)) = memory.get(n1) {
                             if v1 > v2 { cond = true; }
                         }
                         offset = 3;
                    } else if let Some(VasoType::VBit(1, _)) = memory.get(n1) { cond = true; }
                }
                if cond { i += offset; } else {
                     let mut brackets = 0;
                     while i < tokens.len() {
                        if let Token::LBrace = tokens[i] { brackets = 1; i+=1; break; }
                        i += 1;
                     }
                    while i < tokens.len() && brackets > 0 {
                        match tokens[i] {
                            Token::LBrace => brackets += 1,
                            Token::RBrace => brackets -= 1,
                            _ => {}
                        }
                        i += 1;
                    }
                    consumed = true;
                    i -= 1;
                }
            },
            Token::Value | Token::Variable => {
                if let (Some(Token::Identifier(name)), Some(assign), Some(val_token)) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    if matches!(assign, Token::AssignC | Token::AssignPascal) {
                        let mut lib_call = false;
                        if let (Token::Identifier(module), Some(Token::Dot), Some(Token::Identifier(func))) = 
                               (val_token, tokens.get(i+4), tokens.get(i+5)) {
                             let (args, next_idx) = extract_args(&tokens, i+6, &memory);
                             let result = call_std_function(module, func, args);
                             memory.insert(name.clone(), result);
                             i = next_idx; consumed = true;
                             lib_call = true;
                        }
                        if !lib_call {
                            let val = match val_token {
                                Token::LitOff => Some(VasoType::VBit(0, "".to_string())),
                                Token::LitOn => Some(VasoType::VBit(1, "".to_string())),
                                Token::LitLoading => Some(VasoType::VBit(2, "".to_string())),
                                Token::LitError => {
                                    if let (Some(Token::LParen), Some(Token::StringLiteral(msg)), Some(Token::RParen)) = 
                                           (tokens.get(i+4), tokens.get(i+5), tokens.get(i+6)) {
                                         i += 3; Some(VasoType::VBit(3, msg.trim_matches('"').to_string()))
                                    } else { Some(VasoType::VBit(3, "".to_string())) }
                                },
                                Token::LitUnknown => Some(VasoType::VBit(4, "".to_string())),
                                Token::NumberLiteral(n) => Some(VasoType::Int(*n)),
                                Token::StringLiteral(s) => Some(VasoType::Str(s.trim_matches('"').to_string())),
                                Token::Identifier(n) => memory.get(n).cloned(),
                                Token::Input => {
                                    print!("> "); std::io::stdout().flush().unwrap();
                                    let mut input_text = String::new();
                                    std::io::stdin().read_line(&mut input_text).expect("Error");
                                    Some(VasoType::Str(input_text.trim().to_string()))
                                },
                                Token::New => {
                                     if let Some(Token::Identifier(struct_name)) = tokens.get(i+4) {
                                        if let Some(def) = structs.get(struct_name) {
                                            for (field, _) in &def.fields {
                                                let key = format!("{}.{}", name, field);
                                                memory.insert(key, VasoType::VBit(4, "".to_string())); 
                                            }
                                            i += 2; 
                                        }
                                    }
                                    None 
                                },
                                _ => None
                            };
                            if let Some(v) = val {
                                memory.insert(name.clone(), v);
                                i += 4; consumed = true;
                            } else if matches!(val_token, Token::New) { i += 4; consumed = true; }
                        }
                    }
                }
            },
            Token::Identifier(name) => {
                if let (Some(assign), Some(val_token)) = (tokens.get(i+1), tokens.get(i+2)) {
                    if matches!(assign, Token::AssignC | Token::AssignPascal) {
                        let val = match val_token {
                            Token::LitOff => Some(VasoType::VBit(0, "".to_string())),
                            Token::LitOn => Some(VasoType::VBit(1, "".to_string())),
                            Token::LitLoading => Some(VasoType::VBit(2, "".to_string())),
                            Token::LitError => Some(VasoType::VBit(3, "".to_string())),
                            Token::Identifier(n) => memory.get(n).cloned(),
                            _ => None
                        };
                         if let Some(v) = val { memory.insert(name.clone(), v); i += 3; consumed = true; }
                    }
                    else if let Token::PlusAssign = assign {
                        let right_val = match val_token {
                            Token::NumberLiteral(n) => Some(VasoType::Int(*n)),
                            Token::LitLoading => Some(VasoType::VBit(2, "".to_string())),
                            Token::LitError => Some(VasoType::VBit(3, "".to_string())),
                            Token::Identifier(n) => memory.get(n).cloned(),
                            _ => None
                        };
                        if let Some(r_val) = right_val {
                            if let Some(l_val) = memory.get(name) {
                                let result = apply_dominance(l_val, &r_val);
                                memory.insert(name.clone(), result);
                                i += 3; consumed = true;
                            }
                        }
                    }
                }
            },
            Token::Mold | Token::Semicolon | Token::While => { i += 1; consumed = true; }, 
            _ => {}
        }
        if !consumed { i += 1; }
    }
    println!("---------------");
    println!("{}", "✅ Ejecución finalizada.".green());
}
// --- AGREGAR AL FINAL DE src/main.rs ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dominance_hierarchy() {
        // Error > On
        let err = VasoType::VBit(3, "failed".to_string());
        let on = VasoType::VBit(1, "".to_string());
        let result = apply_dominance(&err, &on);
        
        // Debe ganar el error y conservar el mensaje
        if let VasoType::VBit(v, msg) = result {
            assert_eq!(v, 3);
            assert_eq!(msg, "failed");
        } else {
            panic!("Dominance failed");
        }
    }

    #[test]
    fn test_loading_over_on() {
        // Loading > On
        let loading = VasoType::VBit(2, "".to_string());
        let on = VasoType::VBit(1, "".to_string());
        let result = apply_dominance(&loading, &on);
        
        if let VasoType::VBit(v, _) = result {
            assert_eq!(v, 2);
        } else {
            panic!("Dominance failed");
        }
    }
}