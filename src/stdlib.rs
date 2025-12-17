use crate::types::VasoType;
use crate::logic::check_errors;
use std::fs;
use std::env;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH, Duration}; // <--- Agregamos Duration
use std::thread; // <--- Agregamos thread para sleep
use rand::Rng;
use serde_json::Value;

pub fn call_std_function(module: &str, func: &str, args: Vec<VasoType>) -> VasoType {
    if let Some(err) = check_errors(&args) { return err; }

    match module {
        "Time" => match func {
            "now" => {
                let start = SystemTime::now();
                let since = start.duration_since(UNIX_EPOCH).expect("Time fail");
                VasoType::Int(since.as_secs() as i32)
            },
            "sleep" => { // <--- NUEVO: Time.sleep(ms)
                if let Some(VasoType::Int(ms)) = args.get(0) {
                    thread::sleep(Duration::from_millis(*ms as u64));
                    VasoType::VBit(1, "".to_string())
                } else { VasoType::VBit(3, "Arg Error: Sleep needs Int (ms)".to_string()) }
            },
            _ => VasoType::VBit(3, format!("Time.{} not found", func))
        },
        "Sys" => match func {
            "os" => VasoType::Str(env::consts::OS.to_string()), 
            "arg" => { // <--- NUEVO: Sys.arg(index)
                if let Some(VasoType::Int(idx)) = args.get(0) {
                    // Offset de 2 porque 0=binary, 1=script.vs
                    let real_idx = (idx + 2) as usize; 
                    match env::args().nth(real_idx) {
                        Some(val) => VasoType::Str(val),
                        None => VasoType::VBit(4, "No Arg".to_string())
                    }
                } else { VasoType::VBit(3, "Sys.arg needs Int".to_string()) }
            },
            // ... (Mantener exec igual que antes) ...
            "exec" => {
                if let (Some(VasoType::Str(cmd)), Some(VasoType::Str(arg1))) = (args.get(0), args.get(1)) {
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
                            if output.status.success() { VasoType::VBit(1, "".to_string()) } 
                            else {
                                let err = String::from_utf8_lossy(&output.stderr).to_string();
                                let msg = if err.is_empty() { String::from_utf8_lossy(&output.stdout).to_string() } else { err };
                                VasoType::VBit(3, format!("CMD Failed: {}", msg.trim()))
                            }
                        },
                        Err(e) => VasoType::VBit(3, format!("Exec Error: {}", e))
                    }
                } else { VasoType::VBit(3, "Sys.exec needs (cmd, arg)".to_string()) }
            },
            _ => VasoType::VBit(3, "Sys func error".to_string())
        },
        // ... (Mantener Math, Json, File igual que antes) ...
        "Math" => match func {
            "random" => {
                let mut rng = rand::thread_rng();
                VasoType::Int(rng.gen_range(0..100))
            }, 
            _ => VasoType::VBit(3, format!("Math.{} not found", func))
        },
        "Json" => match func {
            "parse" => {
                if let Some(VasoType::Str(json_str)) = args.get(0) {
                    match serde_json::from_str::<Value>(json_str) {
                        Ok(v) => VasoType::Json(v),
                        Err(e) => VasoType::VBit(3, format!("JSON Error: {}", e))
                    }
                } else { VasoType::VBit(3, format!("Json.parse needs (Str), got {:?}", args)) }
            },
            "get" => {
                if let (Some(VasoType::Json(val)), Some(VasoType::Str(key))) = (args.get(0), args.get(1)) {
                    match val.get(key) {
                        Some(v) => {
                            if let Some(i) = v.as_i64() { VasoType::Int(i as i32) }
                            else if let Some(s) = v.as_str() { VasoType::Str(s.to_string()) }
                            else if let Some(b) = v.as_bool() { 
                                if b { VasoType::VBit(1, "".to_string()) } else { VasoType::VBit(0, "".to_string()) }
                            }
                            else { VasoType::VBit(4, "Complex Type".to_string()) }
                        },
                        None => VasoType::VBit(4, format!("Field '{}' not found", key))
                    }
                } else { 
                    VasoType::VBit(3, format!("Json.get needs (Json, Str), got {:?}", args))
                }
            },
            _ => VasoType::VBit(3, format!("Json.{} not found", func))
        },
        "File" => match func {
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
        },
        _ => VasoType::VBit(3, format!("Module {} not found", module))
    }
}