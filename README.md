# ✨ The Prism Platform ✨

> **A disciplined journey to build a programming language and operating system from first principles.**

This repository is the home of the **Prism Platform**, an open-source educational project with two long-term goals:

1. **Trace**: A simple, safe, and human-readable programming language.
2. **PrismOS**: A clean, intuitive operating system built around Trace.

**But today, we are focused on one thing: making Trace a correct, testable, and complete language.**

---

## 🎯 Current Status & Roadmap

We are in **Milestone v0.3**, working toward a **Turing-complete Trace interpreter** with proper error handling and a solid test suite.

### ✅ Immediate Goals (v0.3)
- [ ] Implement `if` statements and `loop` constructs  
- [ ] Add arithmetic support (`+` operator)  
- [ ] Replace all `unwrap`, `expect`, and silent fallbacks with explicit `Result<T, E>` error handling  
- [ ] Make undefined variable access a **hard error**  
- [ ] Write **20+ unit tests** covering lexer, parser, and evaluator edge cases  
- [ ] Remove all dead code and eliminate `#![allow(dead_code)]`  
- [ ] Publish this realistic roadmap  

> 🔜 **Success criterion**: `fibonacci(10)` must run correctly in Trace.

### 🌌 Long-Term Vision (Post v1.0)
- A self-hosting Trace compiler  
- A minimal runtime environment  
- **Eventually**: The foundations of **PrismOS** — but **not yet**.  
  *(Building an OS requires a stable language, a memory model, and systems expertise. We are not there.)*

This project is **not a product**. It is a **public learning log**, a **demonstration of incremental engineering**, and a **monument to rigor over hype**.

---

## 🛠️ Getting Started

### Prerequisites
Install the Rust toolchain using `rustup` (the official Rust installer):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

💡 On Windows? Download and run rustup-init.exe instead.
Ensure your PATH includes ~/.cargo/bin (or %USERPROFILE%\.cargo\bin on Windows).

## Running Trace

### Clone the repositories
```sh
git clone https://github.com/zimzero12/PrismPlatform.git
cd PrismPlatform | sh

### Run the REPL
```sh
cargo run | sh

### Execute a script
```sh
cargo run -- your_script.trace | sh

Warning: Trace is not stable. Breaking changes are expected weekly. 

## 📚 Documentation
See DOCUMENTATION.md for the current language specification.

## 🤝 Contributing & Learning
This project is open for educational collaboration. All contributions must:

Include tests
Use explicit error handling
Avoid dead or unused code
We welcome issue reports, especially around edge cases in parsing or evaluation.

"First, make it correct. Then, make it beautiful." 