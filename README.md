# Compiler in Rust Project

## Overview

This project aims to develop a compiler written in Rust, capable of translating a subset of a programming language into executable code. The compiler will consist of several stages including lexical analysis.

## Roadmap

### Phase 1: Lexer and Parsing Arithmetic (April 22nd, 2024)

- **Lexer Implementation**:
  - Design and implement a lexer to tokenize input source code.
  - Define token types for identifiers, keywords, literals (e.g., integers, floats), operators, and delimiters.
  - Implement basic error handling for invalid tokens.

- **Parser Implementation**:
  - Design and implement a parser to generate an abstract syntax tree (AST) from tokenized input.
  - Define grammar rules for arithmetic expressions including addition, subtraction, multiplication, division, and parentheses.
  - Implement recursive descent parsing algorithm for expression parsing.
  - Integrate the lexer with the parser to consume tokens.

- **AST Structure**:
  - Define data structures for representing AST nodes corresponding to arithmetic expressions.
  - Implement AST building logic in the parser.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.