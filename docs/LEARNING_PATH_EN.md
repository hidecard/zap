# Zap Learning Path Curriculum

This guide provides a structured learning path from beginner to advanced Zap programming. Each level builds on the previous one and includes practical exercises.

## Table of Contents

- [Level 1: Beginner](#level-1-beginner)
- [Level 2: Intermediate](#level-2-intermediate)
- [Level 3: Advanced](#level-3-advanced)
- [Level 4: Expert](#level-4-expert)
- [Project Ideas](#project-ideas)
- [Resources](#resources)

---

## Level 1: Beginner

**Prerequisites:** Basic programming knowledge (variables, functions, loops)

**Duration:** 1-2 weeks

### Topics

1. **Getting Started**
   - Install Zap
   - Hello World program
   - Basic syntax structure
   - Running and debugging

2. **Core Concepts**
   - Variables and types (int, text, bool)
   - Arithmetic operations
   - String operations
   - Comments and code organization

3. **Control Flow**
   - `if` / `else` conditionals
   - `while` loops
   - `for` loops
   - `break` and `continue`

4. **Functions**
   - Defining functions
   - Parameters and return values
   - Function scope
   - Basic recursion

### Exercises

1. Write a program that calculates the sum of numbers from 1 to 100
2. Create a function that checks if a number is even or odd
3. Build a simple calculator that performs basic arithmetic
4. Write a program that prints a multiplication table

### Reference
- [Language Specification](LANGUAGE_SPEC_EN.md) - Sections 1-5
- [Learn Zap](LEARN_ZAP_EN.md) - Beginner tutorials

---

## Level 2: Intermediate

**Prerequisites:** Level 1 completion

**Duration:** 2-3 weeks

### Topics

1. **Data Structures**
   - Lists (creation, indexing, operations)
   - Maps (key-value pairs)
   - Structs (custom data types)
   - Nested data structures

2. **Advanced Functions**
   - Default parameters
   - Generic functions
   - Closures and captures
   - Higher-order functions

3. **Error Handling**
   - Option type (some/none)
   - Error type
   - Pattern matching
   - Safe error propagation

4. **Modules and Imports**
   - Creating modules
   - Importing symbols
   - Module scope
   - Reusable code organization

### Exercises

1. Build a todo list application with CRUD operations
2. Create a function that filters a list based on a predicate
3. Implement a simple hash map collision resolution
4. Write a module with reusable utility functions

### Reference
- [Language Specification](LANGUAGE_SPEC_EN.md) - Sections 6-10
- [Standard Library Index](STDLIB_INDEX_EN.md)
- [Package Guide](PACKAGE_EN.md)

---

## Level 3: Advanced

**Prerequisites:** Level 2 completion

**Duration:** 3-4 weeks

### Topics

1. **Object-Oriented Programming**
   - Classes and methods
   - Constructors
   - Inheritance
   - Member access and mutation

2. **Type System**
   - Type annotations
   - Generic types
   - Type inference
   - Type constraints

3. **Async Programming**
   - Tasks and async/await
   - Task scheduling
   - Resource limits
   - Async best practices

4. **Database Integration**
   - SQLite operations
   - Models and migrations
   - Query builders
   - Transactions

### Exercises

1. Build a REST API with web framework
2. Create a database-backed application
3. Implement async file processing pipeline
4. Build a class hierarchy for a domain model

### Reference
- [Language Specification](LANGUAGE_SPEC_EN.md) - Sections 11-15
- [Web Framework Guide](WEB_FRAMEWORK_EN.md)
- [Database Production Guide](DATABASE_PRODUCTION_EN.md)

---

## Level 4: Expert

**Prerequisites:** Level 3 completion

**Duration:** 4-6 weeks

### Topics

1. **Compiler Internals**
   - Bootstrap stages (B0-B4)
   - Type checking algorithms
   - Bytecode generation
   - Runtime execution

2. **Performance Optimization**
   - Benchmarking
   - Memory management
   - Algorithm optimization
   - Profiling techniques

3. **System Programming**
   - Filesystem operations
   - Process management
   - Network programming
   - System integration

4. **Package Ecosystem**
   - Creating packages
   - Publishing to registry
   - Dependency management
   - Semantic versioning

### Exercises

1. Contribute to the Zap compiler
2. Build a custom LSP language server
3. Create a production-ready web application
4. Publish a reusable package to the registry

### Reference
- [Bootstrap Contract](../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md)
- [Benchmark Harness](BENCHMARK_HARNESS_EN.md)
- [Registry Auth](REGISTRY_AUTH_EN.md)

---

## Project Ideas

### Beginner Projects
- [x] Hello World - Print "Hello, World!"
- [ ] Number Guessing Game - Random number with hints
- [ ] Temperature Converter - Celsius/Fahrenheit/Kelvin
- [ ] Simple Calculator - Basic arithmetic operations
- [ ] String Reverser - Reverse input strings

### Intermediate Projects
- [ ] Todo List - CRUD with file persistence
- [ ] Contact Manager - Store and search contacts
- [ ] File Analyzer - Count lines, words, characters
- [ ] Simple Game - Number guessing with scores
- [ ] Data Validator - Validate JSON-like structures

### Advanced Projects
- [ ] REST API - Web framework with endpoints
- [ ] CLI Tool - Command-line application
- [ ] Database App - SQLite-backed application
- [ ] Web Scraper - Fetch and parse web data
- [ ] Async Processor - Parallel file processing

### Expert Projects
- [ ] LSP Extension - VS Code language server
- [ ] Custom Package - Reusable library
- [ ] Compiler Contribution - Add language feature
- [ ] Performance Tool - Benchmarking framework
- [ ] System Monitor - Process/file watcher

---

## Resources

### Official Documentation
- [Language Specification](LANGUAGE_SPEC_EN.md)
- [Current Status](CURRENT_STATUS_EN.md)
- [Migration Guide](MIGRATION_GUIDE_EN.md)
- [Release Notes](RELEASE_2.11.18_EN.md)

### Learning Materials
- [Learn Zap](LEARN_ZAP_EN.md)
- [Examples](../examples/)
- [Syntax Guide](SYNTAX_GUIDE_EN.md)
- [Language Guide](LANGUAGE_GUIDE.md)

### Developer Tools
- [VS Code Extension](../vscode-extension/)
- [LSP Guide](ASYNC_LSP_EN.md)
- [Debugging Guide](RUNTIME_STATE_EN.md)
- [Testing Guide](../examples/testing/)

### Community
- [GitHub Issues](https://github.com/hidecard/zap/issues)
- [Contributing Guide](CONTRIBUTING.md)
- [RFC Template](RFC_TEMPLATE.md)
- [Security Policy](SECURITY.md)

---

## Learning Checklist

### Level 1: Beginner
- [ ] Install Zap and run hello world
- [ ] Understand basic types and operations
- [ ] Write if/else and loop programs
- [ ] Define and call functions
- [ ] Complete 3 beginner exercises

### Level 2: Intermediate
- [ ] Work with lists and maps
- [ ] Use default parameters and generics
- [ ] Handle errors with Option/Error types
- [ ] Create and import modules
- [ ] Complete 3 intermediate exercises

### Level 3: Advanced
- [ ] Define classes and methods
- [ ] Use type annotations and generics
- [ ] Write async code with tasks
- [ ] Integrate with SQLite database
- [ ] Complete 3 advanced exercises

### Level 4: Expert
- [ ] Understand bootstrap stages
- [ ] Profile and optimize code
- [ ] Work with system APIs
- [ ] Create and publish packages
- [ ] Complete 3 expert exercises

---

## Next Steps

After completing all levels:

1. **Contribute to Zap** - Open a pull request to fix a bug or add a feature
2. **Build a Real Project** - Create a production-ready application
3. **Share Your Knowledge** - Write tutorials or help others learn
4. **Stay Updated** - Follow the [roadmap](ROADMAP_EN.md) and changelog

---

## Tips for Success

1. **Practice Daily** - Code every day, even for 15 minutes
2. **Read Source Code** - Study examples and the compiler implementation
3. **Build Projects** - Apply knowledge to real problems
4. **Ask Questions** - Use GitHub issues when stuck
5. **Teach Others** - Explaining concepts reinforces learning

---

## Getting Help

If you're stuck:

- Check the [Language Specification](LANGUAGE_SPEC_EN.md)
- Review [Examples](../examples/)
- Search [GitHub Issues](https://github.com/hidecard/zap/issues)
- Open a new issue with your question
