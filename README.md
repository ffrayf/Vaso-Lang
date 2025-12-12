# Vaso Programming Language 🥃

> **"Simple as a spoon, Fast as a rocket."**

Vaso is an **experimental programming language concept** designed to explore a new paradigm: unifying high-level simplicity with low-level hardware control, without the boilerplate.

This project is a **proof of concept (Prototype)**. It proposes a novel **5-state logic system (Quantum-Ready)** and seamless multi-language interoperability, aiming to strip away unnecessary abstractions to get closer to the metal, while keeping the syntax human-friendly.

## 💡 The Philosophy

We believe modern computing has become unnecessarily complex. Vaso aims to return to the roots:

* **Socratic Simplicity:** If a spoon doesn't need a manual, neither should a variable declaration. "Keep it simple" is our core dogma.
* **Hardware Agnosticism:** The language design intends to be smart enough to utilize the GPU (like an RTX series) or CPU automatically, blurring the line between host and device code.
* **Legacy-Free Logic:** Reimagining binary logic by introducing "V-Bits" to handle uncertainty natively, preparing for a future of AI and probabilistic computing.

## 🚀 Key Features (Proposed)

### 1. Hybrid Syntax
Write code as simple as Python or as controlled as C++. Vaso adapts to the complexity of the task, not the other way around.

### 2. The "Chameleon" Bridge
Native support to embed other languages directly to leverage existing ecosystems instantly.

```vaso
use(python): {
    import numpy as np
    # Seamless execution of external libraries
}
```

### 3. V-Bit Logic (Quantum-Ready)
Native support for a 5-state logic system, moving beyond traditional boolean (`true/false`) constraints:
* `0` = **No**
* `1` = **Yes**
* `2` = **Unknown** (Probabilistic)
* `3` = **Paradox** (Yes and No / Superposition)
* `4` = **Null** (Void)

### 4. Hardware Aware
Automatic optimization for heterogeneous computing. The goal is to allow the compiler to decide the best execution path (CPU vs GPU) based on workload density.

## ⚠️ Project Status: Pre-Alpha / Prototype

**Current version: v0.5.0 (Turing Logic Prototype)**

The current implementation acts as a **fully functional interpreter** capable of memory, logical flow, and now algorithmic loops. The architecture supports the fundamentals of universal computation.

**This is an experimental proposal, not yet a production-ready tool.**

## 🛠️ Example Code

```vaso
// file: demo.vs
fn main() {
    // 1. Simplicity & Mutability
    var counter := 0;

    // 2. Turing Completeness (Loops)
    while counter < 5 {
        print("System check...");
        print(counter);
        counter += 1;
    }

    // 3. V-Bit Logic (5-state)
    val status : vbit := 2; // State: Unknown
    if status == 2 {
        print("Status is uncertain (Quantum State).");
    }
}
```

## 📸 System Architecture & Demos

### Phase 5: Vaso v0.5.0 (Turing Logic) - Loops and Mutability 🔄
**Key Milestone.** The Vaso engine is now functionally **Turing Complete**. It features support for **While Loops**, **Mutable Variables** (`var`), and **Re-assignment** logic (`+=`), enabling algorithmic repetition and complex computation.


[Vaso Turing Logic Output](assets/demo_v05_turing.png)

### Phase 4: The Chameleon Bridge (v0.4) - Interoperability 🐍
**Breaking Barriers.** Vaso acts as a multi-language orchestrator. The `use(python)` block allows seamless embedding and execution of external Python scripts directly within Vaso code. This proves the "Glue Language" concept.

[Vaso Python Bridge](assets/demo_bridge.png)

### Phase 3: Logic Core (v0.3) - Intelligent Branching 🧠
The engine evolved into a decision-making machine. Vaso supports **Memory Management** (`HashMap`) and **Control Flow** (`if/else`).

[Vaso Logic Core](assets/demo_logic.png)

### Phase 2: The Engine (v0.2) - Live Execution
The first implementation of the linear interpreter. Capable of allocating memory variables and executing standard output instructions.

[Vaso Engine Execution](assets/demo_run_v0.2.png)

### Phase 1: The Lexer (v0.1) - Tokenization
The foundation. Vaso uses a custom Lexer built with `Logos` in Rust to break down source code into atomic tokens.

[Vaso Lexer Analysis](assets/demo_run.png)

## 🤝 Contribution & Acknowledgments

**Created by Fabian Fray.**

*This project is being developed with the assistance of advanced AI models for architectural brainstorming, rapid prototyping, and code optimization.*

We are looking for contributors who share the vision of simplifying the future of coding. If you are an engineer, a philosopher, or just curious, feel free to open a discussion.

---
*© 2025 The Vaso Project. Licensed under MIT.*