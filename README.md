
 # 🥃 Vaso Language 
 
 > A Scripting Language for CI/CD with State Propagation 
 > Status: Experimental Learning Project / Proof of Concept 
 
 Vaso explores a new paradigm for DevOps scripting: What if error handling was mathematical instead of conditional
 
 
 ## The Problem 
 In Bash or Python pipelines, ensuring that a failure in step 1 stops step 5 requires verbose boilerplate: 
 
 bash 
 # Bash boilerplate 
 npm test 
 if [ $
 -ne 0 ]; then 
 echo "Tests failed" 
 exit 1 
 fi 
 
 
 ## The Vaso Solution 
 Vaso uses State Dominance Arithmetic. You simply "add" your pipeline stages together. If any stage fails, the error state dominates the result and carries the error message to the end. 
 
 vaso 
 // Vaso CI/CD Pipeline 
 var tests := Sys.exec("npm", "test"); 
 var build := Sys.exec("npm", "build"); 
 
 // Logic: If tests fail, 'pipeline' becomes Error automatically. 
 var pipeline := tests + build; 
 
 match pipeline { 
 on => { 
 print("✅ Pipeline Success. Deploying..."); 
 Sys.exec("aws", "s3 sync ..."); 
 } 
 error(msg) => { 
 print("🚨 Pipeline Failed: " + msg); 
 } 
 } 
 
 
 ## Key Features 
 * Rich V-Bits: States like Error, Loading, and On are primitives. 
 * Payload Propagation: Errors carry their context (e.g., "File not found") through math operations. 
 * System Overlord: Built-in Sys.exec and File.read for real orchestration. 
 * Cross-Platform: Scripts run on Windows and Linux without changes. 
 
 ## Installation & Usage 
 Built with Rust. 
 
 bash 
 # Clone and Run 
 git clone https://github.com/[ffrayf]/Vaso-Lang 
 cd vaso 
 cargo run -q examples/ci_pipeline.vs 
 
 
 ## Roadmap 
 This project served as a deep dive into compiler design, lexing, and type theory. 
 Future work may explore: 
 * [ ] Integration with HTTP APIs 
 * [ ] JSON Parsing for config files 
 * [ ] Rust RFC proposal for #[vbit] macro