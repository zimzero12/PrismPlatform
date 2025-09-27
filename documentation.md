# ✍️ The Trace Programming Language (v0.2 — Pre-Turing)

> **A work in progress. Not yet Turing-complete. Not yet safe. But becoming correct.**

Trace is a minimal, human-readable language designed for **clarity** and **pedagogical value**. It is **not** a general-purpose language—yet.

This document describes the **current capabilities** of Trace as of v0.2. Features marked as *planned* are not implemented.

---

## 🧱 Core Principles

- **Readability**: Code should read like plain instructions.
- **Explicitness**: Errors are never hidden. Undefined behavior is forbidden by design.
- **Simplicity**: Start small. Grow only when the foundation is solid.

---

## ✅ Implemented Features

### 1. Printing Output

Use `say` to print text or variable values.

```trace
say "Hello, World!"
create name = "Alice"
say name

### 2. Variables

Use create to declare and assign a variable. Only numbers (f64) and text (String) are supported.

```trace
create score = 100
create message = "Game started"

⚠️ Important: As of v0.2, referencing an undefined variable prints an error and returns "undefined".
This will change in v0.3: undefined variables will cause a hard evaluation error.

### 3. Comments

Anything after # is ignored.

```trace
create x = 42  # The answer
# say x  <-- this line is skipped

🚧 Planned for v0.3 (Not Yet Implemented)
The following are not functional but are part of the immediate roadmap:

Conditionals:
trace


1
2
3
if score > 90 {
    say "Great job!"
}
Loops:
trace


1
2
3
4
loop {
    say "Running..."
    # (will require a break mechanism)
}
Arithmetic:
trace


1
create total = 5 + 3
Functions (post-v0.3)
❌ Do not attempt to use if, loop, or + in v0.2—they exist only as parser stubs and will be ignored or cause errors. 

🧪 Error Philosophy (Evolving)
In v0.2, the interpreter prioritizes non-crashing execution—but this is being revised.

v0.3 will enforce:

Parse errors → halt with line/column info (planned)
Runtime errors (e.g., undefined variable) → explicit TraceError, not silent fallbacks
Invalid numbers (e.g., "123abc") → parse failure, not 0.0
📜 License
This project is open source. See LICENSE for details.

Remember: Trace is a learning vehicle. Its value is in its evolution—not its current state. 