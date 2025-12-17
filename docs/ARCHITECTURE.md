 # Vaso Architecture 
 
 ## 1. Design Philosophy 
 Vaso is a conceptual scripting language designed specifically for CI/CD pipelines and System Orchestration. 
 
 In traditional scripting (Bash, Python), error handling is often verbose (if err != nil) or fragile (set -e). Vaso solves this by treating System State as a primitive type with automatic propagation. 
 
 ## 2. Core Innovation: V-Bits (Vaso Bits) 
 Instead of boolean (true/false) or numeric return codes, Vaso operations return a Rich State Type: 
 
 | Rank | State | Value | Description | 
 | :--- | :--- | :--- | :--- | 
 | 5 | Error | 3 | Critical failure. Carries a payload (e.g., "IO Error"). Dominates all. | 
 | 4 | Unknown | 4 | Null safety equivalent. Missing data. | 
 | 3 | Loading | 2 | Pending or In-progress state. | 
 | 2 | On | 1 | Success / True / Active. | 
 | 1 | Off | 0 | Inactive / False. | 
 
 ### The "State Dominance" Arithmetic 
 When combining multiple operations (e.g., executing a pipeline stages), the "worst" state automatically propagates without explicit control flow. 
 
 vaso 
 // If 'test' fails (Error), 'deploy' becomes Error automatically. 
 var deploy := test + build + release; 
 
 
 ## 3. Implementation Details 
 * Language: Rust (2021 Edition) 
 * Lexer: Generated using logos crate for high-performance tokenization. 
 * Parser: Hand-written recursive descent parser. 
 * Runtime: Tree-walking interpreter with a flat memory model (HashMap<String, VasoType>). 
 * FFI (Foreign Function Interface): 
 * Sys.exec: Wraps std::process::Command to execute OS processes cross-platform. 
 * File.read: Wraps std::fs for direct file system access. 
 
 ## 4. Cross-Platform Strategy 
 Vaso includes a lightweight abstraction layer for Sys.exec that automatically detects the host OS (Windows/Linux/macOS) and adjusts command arguments (e.g., wrapping cmd /C on Windows) to ensure script portability