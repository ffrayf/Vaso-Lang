mod types;
mod tokens;
mod logic;
mod stdlib;
mod parser;
mod memory;

use logos::Logos;
use colored::*;
use std::env;
use std::fs;
use std::collections::HashMap;
use types::{VasoType, StructDef};
use tokens::Token;
use logic::apply_op; 
use stdlib::call_std_function;
use parser::extract_args;
use memory::MemoryStack;
use std::ops::Range;

// --- DEBUGGER ---
fn get_line_number(code: &str, index: usize) -> usize {
    code[..index].matches('\n').count() + 1
}

fn report_error(msg: &str, span: &Range<usize>, code: &str) {
    let line = get_line_number(code, span.start);
    println!("{} {} {}", "❌ ERROR [Line".red().bold(), line.to_string().red().bold(), "]:".red().bold());
    println!("   >> {}", msg.white());
}

fn main() {
    println!("{}", "\n🥃  VASO ENGINE v3.0 (Performance & Resilience)".bold().cyan());
    println!("{}", "===============================================".cyan());

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { println!("❌ Uso: cargo run <archivo.vs> [args...]"); return; }
    
    let code = fs::read_to_string(&args[1]).expect("❌ ERROR: No encuentro el archivo .vs");
    
    // --- 1. LEXER ESTRICTO (Feedback TF: No ignorar errores) ---
    let tokens: Vec<(Token, Range<usize>)> = Token::lexer(&code)
        .spanned()
        .map(|(res, span)| {
            match res {
                Ok(token) => (token, span),
                Err(_) => {
                    // Si encontramos un caracter invalido, pánico inmediato con contexto.
                    let line = get_line_number(&code, span.start);
                    eprintln!("❌ CRITICAL LEXER ERROR [Line {}]: Invalid character found.", line);
                    std::process::exit(1);
                }
            }
        })
        .collect();
    
    let mut mem_stack = MemoryStack::new();
    let _structs: HashMap<String, StructDef> = HashMap::new();
    
    // --- 2. JUMP MAP (Feedback TF: Solución O(1) vs O(n²)) ---
    // Mapeamos el inicio de cada bloque '{' con su final '}'.
    let mut jump_map: HashMap<usize, usize> = HashMap::new();
    let mut brace_stack: Vec<usize> = Vec::new();

    for (i, (token, _)) in tokens.iter().enumerate() {
        match token {
            Token::LBrace => brace_stack.push(i),
            Token::RBrace => {
                if let Some(start_idx) = brace_stack.pop() {
                    jump_map.insert(start_idx, i); // Key: Inicio, Value: Final
                }
            },
            _ => {}
        }
    }
    // --------------------------------------------------------

    let mut current_depth = 0;
    let mut skip_else_at_depth: HashMap<usize, bool> = HashMap::new();
    let mut call_stack: Vec<(usize, usize)> = Vec::new();
    
    // Optimizacion memoria: Usar indices seria mejor, pero por ahora mantenemos esto
    struct LoopState { depth: usize, start_idx: usize, iter_var: Option<String>, iter_list: Option<Vec<VasoType>>, iter_pos: usize }
    let mut active_loops: Vec<LoopState> = Vec::new();

    // 3. PRE-SCAN DEFINITIONS
    let mut temp_i = 0;
    while temp_i < tokens.len() {
        match &tokens[temp_i] {
            (Token::Mold, _) => { /* Struct logic placeholder */ },
            (Token::Function, _) => { 
                if let (Some((Token::Identifier(fn_name), _)), Some((Token::LParen, _))) = (tokens.get(temp_i+1), tokens.get(temp_i+2)) {
                    let mut fn_args = Vec::new();
                    let mut k = temp_i + 3;
                    while k < tokens.len() {
                        match &tokens[k] {
                            (Token::RParen, _) => break,
                            (Token::Identifier(arg_name), _) => fn_args.push(arg_name.clone()),
                            _ => {}
                        }
                        k += 1;
                    }
                    let body_start = k + 1; 
                    mem_stack.set_global(fn_name.clone(), VasoType::Function(body_start, fn_args));
                }
            },
            _ => {}
        }
        temp_i += 1;
    }

    // 4. EJECUCION OPTIMIZADA
    let mut i = 0;
    loop {
        if i >= tokens.len() { break; }
        let (current_token, _current_span) = &tokens[i];
        let mut consumed = false; 

        match current_token {
            // SKIPPER OPTIMIZADO (Usando Jump Map)
            Token::Function => {
                // Buscamos donde empieza el cuerpo de la funcion
                let mut scan = i;
                while scan < tokens.len() {
                    if let (Token::LBrace, _) = tokens[scan] {
                        if let Some(&end_idx) = jump_map.get(&scan) {
                            i = end_idx; // TELETRANSPORTACION ⚡
                            consumed = true;
                        }
                        break;
                    }
                    scan += 1;
                }
                if !consumed { i += 1; consumed = true; } // Safety fallback
            },

            Token::LBrace => { current_depth += 1; },
            
            Token::RBrace => { 
                let mut handled = false;
                
                // Loop Logic
                if let Some(loop_state) = active_loops.last_mut() {
                    if loop_state.depth == current_depth {
                        if let (Some(var_name), Some(list)) = (&loop_state.iter_var, &loop_state.iter_list) {
                            loop_state.iter_pos += 1;
                            if loop_state.iter_pos < list.len() {
                                mem_stack.set(var_name.clone(), list[loop_state.iter_pos].clone());
                                i = loop_state.start_idx;
                                current_depth -= 1;
                                consumed = true;
                                handled = true;
                            } else {
                                active_loops.pop(); 
                            }
                        } else {
                            // While
                            i = loop_state.start_idx; 
                            current_depth -= 1; 
                            consumed = true; 
                            handled = true;
                        }
                    }
                }
                
                // Return Logic
                if !handled && !call_stack.is_empty() {
                    let (_, base_depth) = call_stack.last().unwrap();
                    if current_depth == *base_depth + 1 { 
                         let (ret_index, _) = call_stack.pop().unwrap();
                         mem_stack.pop_scope();
                         i = ret_index;
                         consumed = true;
                         handled = true;
                         current_depth -= 1; 
                    }
                }
                if !handled {
                    if current_depth > 0 { current_depth -= 1; } 
                }
            },

            Token::For => {
                if let (Some((Token::Identifier(var_name), _)), Some((Token::In, _)), Some((Token::Identifier(list_name), _))) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    
                    let mut list_val = None;
                    if let Some(VasoType::List(l)) = mem_stack.get(list_name) {
                        list_val = Some(l.clone());
                    }

                    if let Some(list) = list_val {
                        let loop_depth = current_depth + 1;
                        let mut enter_loop = false;
                        let is_active = active_loops.last().map(|l| l.depth == loop_depth && l.start_idx == i + 4).unwrap_or(false);

                        if !is_active {
                            if !list.is_empty() {
                                mem_stack.set(var_name.clone(), list[0].clone());
                                active_loops.push(LoopState { 
                                    depth: loop_depth, 
                                    start_idx: i + 4, 
                                    iter_var: Some(var_name.clone()),
                                    iter_list: Some(list),
                                    iter_pos: 0
                                });
                                enter_loop = true;
                            }
                        } else {
                            enter_loop = true;
                        }

                        if enter_loop {
                            i += 4; 
                            consumed = true; 
                        } else {
                            // SKIP OPTIMIZADO
                            if let Some((Token::LBrace, _)) = tokens.get(i+4) {
                                if let Some(&end_idx) = jump_map.get(&(i+4)) {
                                    i = end_idx; // TELETRANSPORTACION ⚡
                                    consumed = true;
                                }
                            }
                        }
                    } else {
                        report_error("FOR loop expects a List variable", _current_span, &code);
                    }
                }
            },

            Token::While => {
                let mut cond = false;
                let mut offset = 1;
                // ... (Lógica de condiciones igual que antes) ...
                if let Some((Token::Identifier(n1), _)) = tokens.get(i+1) {
                    if let (Some((Token::LessThan, _)), Some((Token::NumberLiteral(v2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let Some(VasoType::Int(v1)) = mem_stack.get(n1) { if v1 < v2 { cond = true; } }
                         offset = 3;
                    }
                    else if let (Some((Token::LessThan, _)), Some((Token::Identifier(n2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let (Some(VasoType::Int(v1)), Some(VasoType::Int(v2))) = (mem_stack.get(n1), mem_stack.get(n2)) { 
                             if v1 < v2 { cond = true; } 
                         }
                         offset = 3;
                    }
                    else if let (Some((Token::Equals, _)), Some((Token::NumberLiteral(v2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let Some(VasoType::Int(v1)) = mem_stack.get(n1) { if v1 == v2 { cond = true; } }
                         offset = 3;
                    }
                    else if let (Some((Token::Equals, _)), Some((Token::Identifier(n2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let (Some(VasoType::Int(v1)), Some(VasoType::Int(v2))) = (mem_stack.get(n1), mem_stack.get(n2)) { 
                             if v1 == v2 { cond = true; } 
                         }
                         offset = 3;
                    }
                }

                let loop_depth = current_depth + 1;
                if cond {
                    let is_active = active_loops.last().map(|l| l.depth == loop_depth).unwrap_or(false);
                    if !is_active {
                        active_loops.push(LoopState { depth: loop_depth, start_idx: i, iter_var: None, iter_list: None, iter_pos: 0 });
                    }
                    i += offset;
                } else {
                    if let Some(last) = active_loops.last() {
                        if last.depth == loop_depth { active_loops.pop(); }
                    }
                    // SKIP OPTIMIZADO
                    // Buscamos la llave de apertura
                    let mut brace_idx = i;
                    while brace_idx < tokens.len() {
                        if let (Token::LBrace, _) = tokens[brace_idx] {
                            if let Some(&end_idx) = jump_map.get(&brace_idx) {
                                i = end_idx; // TELETRANSPORTACION ⚡
                                consumed = true;
                            }
                            break;
                        }
                        brace_idx += 1;
                    }
                }
            },

            Token::Print => {
                // ... (Lógica de print igual) ...
                if let Some((Token::LParen, _)) = tokens.get(i+1) {
                    if let (Some((Token::Identifier(n), _)), Some((Token::RParen, _))) = (tokens.get(i+2), tokens.get(i+3)) {
                        if let Some(v) = mem_stack.get(n) { println!("{}", v); } else { println!(""); } 
                        i += 4; consumed = true;
                    }
                    else if let (Some((Token::StringLiteral(s), _)), Some((Token::RParen, _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         println!("{}", s);
                         i += 4; consumed = true;
                    }
                    else if let (Some(token_lit), Some((Token::RParen, _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         match token_lit {
                             (Token::LitOff, _) => println!("off"),
                             (Token::LitOn, _) => println!("on"),
                             (Token::LitLoading, _) => println!("loading"),
                             (Token::LitError, _) => println!("error"),
                             (Token::LitUnknown, _) => println!("unknown"),
                             _ => {}
                         }
                         i += 4; consumed = true;
                    }
                }
            },

            Token::If => {
                let mut cond = false;
                let mut offset = 1;
                // ... (Lógica de condiciones If igual) ...
                if let Some((Token::Identifier(n1), _)) = tokens.get(i+1) {
                    if let (Some((Token::GreaterThan, _)), Some((Token::Identifier(n2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let (Some(VasoType::Int(v1)), Some(VasoType::Int(v2))) = (mem_stack.get(n1), mem_stack.get(n2)) {
                             if v1 > v2 { cond = true; }
                         }
                         offset = 3;
                    } 
                    else if let (Some((Token::GreaterThan, _)), Some((Token::NumberLiteral(v2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let Some(VasoType::Int(v1)) = mem_stack.get(n1) {
                             if v1 > v2 { cond = true; }
                         }
                         offset = 3;
                    } 
                    else if let (Some((Token::Equals, _)), Some((Token::StringLiteral(s2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let Some(VasoType::Str(s1)) = mem_stack.get(n1) {
                             if s1 == s2 { cond = true; }
                         }
                         offset = 3;
                    } 
                    else if let (Some((Token::Equals, _)), Some((Token::NumberLiteral(v2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let Some(VasoType::Int(v1)) = mem_stack.get(n1) {
                             if v1 == v2 { cond = true; }
                         }
                         offset = 3;
                    }
                    else if let (Some((Token::Equals, _)), Some((Token::Identifier(n2), _))) = (tokens.get(i+2), tokens.get(i+3)) {
                         if let (Some(val1), Some(val2)) = (mem_stack.get(n1), mem_stack.get(n2)) {
                             if val1 == val2 { cond = true; }
                         }
                         offset = 3;
                    }
                    else if let Some(VasoType::VBit(1, _)) = mem_stack.get(n1) { cond = true; }
                }

                if cond { 
                    skip_else_at_depth.insert(current_depth, true);
                    i += offset; 
                } else {
                    skip_else_at_depth.insert(current_depth, false);
                    // SKIP OPTIMIZADO
                    let mut brace_idx = i;
                    while brace_idx < tokens.len() {
                        if let (Token::LBrace, _) = tokens[brace_idx] {
                            if let Some(&end_idx) = jump_map.get(&brace_idx) {
                                i = end_idx; // TELETRANSPORTACION ⚡
                                consumed = true;
                            }
                            break;
                        }
                        brace_idx += 1;
                    }
                }
            },

            Token::Else => {
                let should_skip = *skip_else_at_depth.get(&current_depth).unwrap_or(&false);
                if should_skip {
                    // SKIP OPTIMIZADO
                    if let (Token::LBrace, _) = tokens[i+1] {
                        if let Some(&end_idx) = jump_map.get(&(i+1)) {
                            i = end_idx; // TELETRANSPORTACION ⚡
                            consumed = true;
                        }
                    }
                }
            },

            Token::Match => {
               if let (Some((Token::Identifier(var_name), _)), Some((Token::LBrace, _))) = (tokens.get(i+1), tokens.get(i+2)) {
                    let current_val = mem_stack.get(var_name).cloned().unwrap_or(VasoType::VBit(4, "".to_string())); 
                    let mut j = i + 3;
                    let mut match_executed = false;
                    
                    while j < tokens.len() {
                        if let (Token::RBrace, _) = tokens[j] { break; } 
                        
                        if let (Some((case_token, _)), Some((Token::Arrow, _)), Some((Token::LBrace, _))) = (tokens.get(j), tokens.get(j+1), tokens.get(j+2)) {
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
                                    if let (Token::RBrace, _) = tokens[k] { break; }
                                    
                                    match &tokens[k] {
                                        (Token::Print, _) => {
                                             if let (Some((Token::LParen, _)), Some((Token::StringLiteral(s), _)), Some((Token::RParen, _))) = (tokens.get(k+1), tokens.get(k+2), tokens.get(k+3)) {
                                                 println!("{}", s);
                                             } else if let (Some((Token::LParen, _)), Some((Token::Identifier(n), _)), Some((Token::RParen, _))) = (tokens.get(k+1), tokens.get(k+2), tokens.get(k+3)) {
                                                 if let Some(v) = mem_stack.get(n) { println!("{}", v); }
                                             }
                                             k += 3;
                                        },
                                        (Token::Identifier(target_name), _) => {
                                            if let (Some((assign, _)), Some((val_token, _))) = (tokens.get(k+1), tokens.get(k+2)) {
                                                if matches!(assign, Token::AssignC | Token::AssignPascal) {
                                                    let val_to_assign = match val_token {
                                                        Token::NumberLiteral(n) => Some(VasoType::Int(*n)),
                                                        Token::StringLiteral(s) => Some(VasoType::Str(s.clone())),
                                                        Token::Identifier(source_name) => mem_stack.get(source_name).cloned(),
                                                        _ => None
                                                    };
                                                    if let Some(v) = val_to_assign {
                                                        mem_stack.set(target_name.clone(), v);
                                                    }
                                                    k += 2;
                                                } else if matches!(assign, Token::PlusAssign | Token::MinusAssign) {
                                                     let r_val_opt = match val_token {
                                                         Token::NumberLiteral(n) => Some(VasoType::Int(*n)),
                                                         Token::Identifier(n) => mem_stack.get(n).cloned(),
                                                         _ => None
                                                     };
                                                     if let Some(r_val) = r_val_opt {
                                                         if let Some(l_val) = mem_stack.get(target_name) {
                                                             let res = apply_op(l_val.clone(), &r_val, assign);
                                                             mem_stack.set(target_name.clone(), res);
                                                         }
                                                     }
                                                     k += 2;
                                                }
                                            }
                                        },
                                        _ => {}
                                    }
                                    k += 1;
                                }
                            }

                            // --- SKIP OPTIMIZADO (MATCH CASE) ---
                            // Usamos el Jump Map para saltar el bloque del caso actual
                            // j+2 es la posicion del LBrace '{' del caso
                            if let Some(&end_idx) = jump_map.get(&(j+2)) {
                                j = end_idx + 1; // Saltamos justo despues del '}'
                                continue;
                            }
                        }
                        j += 1;
                    }
                    
                    // --- SKIP OPTIMIZADO (MATCH BLOCK ENTERO) ---
                    // Saltamos todo el bloque match
                    // i+2 es la posicion del LBrace '{' principal del match
                    if let Some(&end_idx) = jump_map.get(&(i+2)) {
                        i = end_idx;
                        consumed = true;
                    }
                }
            },

            Token::Identifier(name) => {
                // ... (Lógica de identificador igual que antes) ...
                if let Some((Token::LParen, _)) = tokens.get(i+1) {
                    let fn_def = mem_stack.get(name).cloned();
                    if let Some(VasoType::Function(body_idx, param_names)) = fn_def {
                        let (call_args, next_idx) = extract_args(&tokens, i+1, &mem_stack);
                        i = next_idx;
                        call_stack.push((i, current_depth));
                        mem_stack.push_scope();
                        for (idx, param) in param_names.iter().enumerate() {
                            if let Some(val) = call_args.get(idx) {
                                mem_stack.set(param.clone(), val.clone());
                            }
                        }
                        i = body_idx; 
                        consumed = true;
                    }
                }
                
                if !consumed {
                    if let (Some((assign, _span)), Some((val_token, val_span))) = (tokens.get(i+1), tokens.get(i+2)) {
                        if matches!(assign, Token::AssignC | Token::AssignPascal) {
                            let final_val;
                            if let Token::LBracket = val_token {
                                let mut list_items = Vec::new();
                                let mut k = i + 3;
                                while k < tokens.len() {
                                    match &tokens[k] {
                                        (Token::RBracket, _) => break,
                                        (Token::NumberLiteral(n), _) => list_items.push(VasoType::Int(*n)),
                                        (Token::StringLiteral(s), _) => list_items.push(VasoType::Str(s.clone())),
                                        _ => {}
                                    }
                                    k += 1;
                                }
                                final_val = Some(VasoType::List(list_items));
                                i = k; 
                            }
                            else {
                                final_val = match val_token {
                                    Token::LitOff => Some(VasoType::VBit(0, "".to_string())),
                                    Token::LitOn => Some(VasoType::VBit(1, "".to_string())),
                                    Token::LitLoading => Some(VasoType::VBit(2, "".to_string())),
                                    Token::LitError => Some(VasoType::VBit(3, "Generic Error".to_string())), 
                                    Token::LitUnknown => Some(VasoType::VBit(4, "".to_string())), 
                                    Token::NumberLiteral(n) => Some(VasoType::Int(*n)),
                                    Token::StringLiteral(s) => Some(VasoType::Str(s.clone())),
                                    Token::Identifier(n) => mem_stack.get(n).cloned(),
                                    _ => None
                                };
                            }

                            if let Some(v) = final_val { 
                                mem_stack.set(name.clone(), v); 
                                if !matches!(val_token, Token::LBracket) { i += 3; }
                                consumed = true; 
                            } else {
                                if !matches!(val_token, Token::Identifier(_)) {
                                    report_error("Invalid assignment value", val_span, &code);
                                }
                            }
                        }
                        else if matches!(assign, Token::PlusAssign | Token::MinusAssign) { 
                             let r_val_opt = match val_token {
                                 Token::LitOff => Some(VasoType::VBit(0, "".to_string())),
                                 Token::LitOn => Some(VasoType::VBit(1, "".to_string())),
                                 Token::LitLoading => Some(VasoType::VBit(2, "".to_string())),
                                 Token::LitError => Some(VasoType::VBit(3, "Generic Error".to_string())),
                                 Token::LitUnknown => Some(VasoType::VBit(4, "".to_string())),
                                 Token::NumberLiteral(n) => Some(VasoType::Int(*n)),
                                 Token::StringLiteral(s) => Some(VasoType::Str(s.clone())),
                                 Token::Identifier(n) => mem_stack.get(n).cloned(),
                                 _ => None
                             };

                             if let Some(r_val) = r_val_opt {
                                 if let Some(l_val) = mem_stack.get(name) {
                                     let res = apply_op(l_val.clone(), &r_val, assign);
                                     mem_stack.set(name.clone(), res);
                                     i += 3; consumed = true;
                                 }
                             }
                        }
                    }
                }
            },
            Token::Value | Token::Variable => {
                // ... (Lógica de var igual que antes) ...
                if let (Some((Token::Identifier(name), _)), Some((assign, _)), Some((val_token, _))) = 
                       (tokens.get(i+1), tokens.get(i+2), tokens.get(i+3)) {
                    if matches!(assign, Token::AssignC | Token::AssignPascal) {
                        let mut lib_call = false;
                        if let Token::Identifier(module) = val_token {
                            if let (Some((Token::Dot, _)), Some((Token::Identifier(func), _))) = (tokens.get(i+4), tokens.get(i+5)) {
                                let (args, next_idx) = extract_args(&tokens, i+6, &mem_stack);
                                let result = call_std_function(module, func, args);
                                mem_stack.set(name.clone(), result);
                                i = next_idx; consumed = true; lib_call = true;
                            }
                        }
                        if !lib_call {
                            match val_token {
                                Token::NumberLiteral(n) => { mem_stack.set(name.clone(), VasoType::Int(*n)); i += 4; consumed = true; },
                                Token::StringLiteral(s) => { mem_stack.set(name.clone(), VasoType::Str(s.clone())); i += 4; consumed = true; },
                                Token::Identifier(src_name) => {
                                    if let Some(val) = mem_stack.get(src_name).cloned() {
                                        mem_stack.set(name.clone(), val);
                                        i += 4; consumed = true;
                                    }
                                },
                                Token::LitOn => { mem_stack.set(name.clone(), VasoType::VBit(1, "".to_string())); i += 4; consumed = true; },
                                Token::LitOff => { mem_stack.set(name.clone(), VasoType::VBit(0, "".to_string())); i += 4; consumed = true; },
                                Token::LitLoading => { mem_stack.set(name.clone(), VasoType::VBit(2, "".to_string())); i += 4; consumed = true; },
                                Token::LitError => { mem_stack.set(name.clone(), VasoType::VBit(3, "Generic Error".to_string())); i += 4; consumed = true; },
                                Token::LitUnknown => { mem_stack.set(name.clone(), VasoType::VBit(4, "".to_string())); i += 4; consumed = true; },
                                _ => {}
                            }
                        }
                    }
                }
            },
            
            Token::Mold | Token::Semicolon => { i += 1; consumed = true; }, 
            _ => {}
        }
        if !consumed { i += 1; }
    }
    println!("---------------");
    println!("{}", "✅ Ejecución finalizada.".green());
}