# ✍️ The Trace Programming Language Guide (v0.2)

Welcome to the official documentation for Trace, a language designed for simplicity and human-readability.

## Core Concepts

- **Simplicity First:** Trace is designed to be read like a set of instructions. There are no semicolons, curly braces, or complex symbols.
- **Statements:** Every line in Trace is a command that tells the computer to do something.

---

## 1. Printing to the Console

The most basic command is `say`. It prints a value to the screen. Right now, it only supports printing text literals.

A **text literal** is any sequence of characters enclosed in double quotes.

**Syntax:**
`say "your text here"`

**Example:**
```trace
say "Hello, World!"