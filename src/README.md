# Source Directory
This directory includes source codes of harmonia.

## Introduction to Source Code
Harmonia source code follows Rust's naming conventions.
See [RFC430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md)
for more details.

## Introduction to `harmonia`
Harmonia has a structure that handles processes step by step.
The working principle of the interpreter roughly consists of the steps described below.

### 1. Lexer
The package `./lexer` is Lexer.
Makes lexical analysis and segments harmonia source code into tokens.

### 2. Parser
The package `./parser` is Parser.
Makes syntax analysis and builds abstract syntax tree of code.
