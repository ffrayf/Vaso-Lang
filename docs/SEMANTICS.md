# Vaso Language: Formal Semantics & State Algebra

## 1. The V-Bit Primitive (5-State Logic)
Vaso introduces `vbit` as a primitive type, not an enumeration.

| Value | Keyword | Semantics |
| :--- | :--- | :--- |
| `0` | `off` | Logical False, Inactive |
| `1` | `on` | Logical True, Active |
| `2` | `loading` | Pending, Latency (Non-blocking) |
| `3` | `error` | Exception, Failure (Blocking) |
| `4` | `unknown` | Uninitialized, Null Safety |

## 2. State Dominance Hierarchy
The core innovation is the **Deterministic Propagation** of states in arithmetic operations.

**Hierarchy Rank:** `Error` > `Unknown` > `Loading` > `On` > `Off`

### Arithmetic Rules (Integers + V-Bits)
* `Int + Loading` → `Loading` (Result is pending)
* `Int + Error` → `Error` (Result is corrupted)
* `Loading + Error` → `Error` (Failure overrides latency)

This eliminates `try/catch` boilerplate in async/unsafe contexts.

## 3. Standard Library Architecture
Vaso implements a modular StdLib accessible via the `use` keyword (e.g., `Time.now()`).