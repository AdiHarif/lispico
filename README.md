# Lispico

Lispico is a functional, Lisp-like programming language. It is designed to be minimalist and elegant, while still being pragmatic, flexible and useful.

This repository contains the source code for a Lispico interpreter written in Rust.

> **Note:** This project is a work in progress, see [Roadmap](#roadmap) section below.

## Key Characteristics

- **Functional** - Lispico is a functional language, meaning that functions are first-class citizens and all values are immutable.
- **Code-Data Interchangeability** - All programs are data, and any value is a syntactically correct program.
- **Minimalist** - Minimal amount of syntax rules and no keywords at all.

## Installation and Usage

> **Note:** You must have git and Rust installed to run Lispico.

To start the Lispico interpreter, clone the repository and run the project using Cargo:

```bash
git clone https://github.com/AdiHarif/lispico.git
cd lispico
cargo run
```

You can now start writing Lispico code in the REPL - each expression entered will be evaluated and the evaluation result will be printed:
```
$ (. 'a '(b c))
(a b c)

$ (.< '(a b c))
a

$ (? x 'a 'b)
b

$ (:= x 't)
$ (? x 'a 'b)
a
```

## Roadmap

The following features are planned for Lispico:
- Implementation:
    - [x] Core language features
        - [x] Core pre-defined operators
        - [x] Let expressions
        - [x] Anonymous functions
    - [x] Arithmetic operations
    - [ ] String operations
    - [ ] Recursion
    - [x] Module system
    - [ ] Standard library
    - [ ] CLI improvements
- Documentation
    - [ ] Language specification
    - [ ] Tutorial

## Contributing

Lispico is open source and contributions are welcome! Feel free to open an issue or submit a pull request for any of the following:
- Feature requests and suggestions
- Bug reports
- Fixes and code improvements
- Documentation
