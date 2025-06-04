# BadeBhai Lang 🚀

[![Language](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Open%20Source-blue.svg)](#)
[![Status](https://img.shields.io/badge/Status-Development-yellow.svg)](#)

**BadeBhai Lang** is a dynamically-typed programming language implemented in Rust, featuring Hindi/Hinglish keywords that make programming more accessible to Hindi speakers. It's based on the Lox language from Robert Nystrom's "Crafting Interpreters" book but adapted with Indian language elements.

## 🌟 Features

### Core Language Features
- **Dynamic Typing**: Variables can hold values of any type
- **Object-Oriented Programming**: Support for classes, methods, and inheritance
- **First-Class Functions**: Functions are values that can be stored in variables and passed around
- **Lexical Scoping**: Variables are scoped to the block where they're defined
- **Garbage Collection**: Automatic memory management
- **Native Functions**: Built-in functions like `clock()` for system operations

### Language Constructs
- Variables and Constants
- Arithmetic and Logical Operations
- Control Flow (if/else, loops)
- Functions and Methods
- Classes and Inheritance
- Comments (single-line `//` and multi-line `/* */`)

## 📝 Syntax Overview

### Keywords (Hindi/Hinglish)
| English | BadeBhai | Usage |
|---------|----------|-------|
| `and` | `aur` | Logical AND operator |
| `class` | `class` | Class definition |
| `else` | `else` | Else condition |
| `false` | `galat` | Boolean false |
| `for` | `for` | For loop |
| `fun` | `fun` | Function definition |
| `if` | `if` | If condition |
| `nil` | `nil` | Null value |
| `or` | `ya` | Logical OR operator |
| `print` | `dikaho` | Print statement |
| `return` | `return` | Return statement |
| `super` | `super` | Parent class reference |
| `this` | `this` | Current object reference |
| `true` | `sahi` | Boolean true |
| `var` | `yehai` | Variable declaration |
| `while` | `jabtak` | While loop |
| `break` | `ruko` | Break statement |

### Basic Syntax Examples

#### Variable Declaration
```badebhai
yehai name = "BadeBhai";
yehai age = 25;
yehai isActive = sahi;
```

#### Functions
```badebhai
fun greet(name) {
    dikaho "Hello, " + name + "!";
}

greet("World");
```

#### Control Flow
```badebhai
yehai x = 10;

if (x > 5) {
    dikaho "x is greater than 5";
} else {
    dikaho "x is less than or equal to 5";
}

// While loop
yehai counter = 0;
jabtak (counter < 5) {
    dikaho counter;
    counter = counter + 1;
}
```

#### Classes and Objects
```badebhai
class Person {
    init(name, age) {
        this.name = name;
        this.age = age;
    }
    
    introduce() {
        dikaho "Hi, I'm " + this.name + " and I'm " + this.age + " years old.";
    }
}

yehai person = Person("Raj", 30);
person.introduce();
```

## 🛠️ Installation & Usage

### Prerequisites
- Rust (2021 edition or later)
- Cargo package manager

### Building from Source
```bash
# Clone the repository
git clone https://github.com/Ujjwal238/BadeBhai-lang.git
cd BadeBhai-lang/main

# Build the project
cargo build --release

# Run the interpreter
cargo run
```

### Running BadeBhai Programs

#### Interactive Mode (REPL)
```bash
cargo run
```
This starts an interactive prompt where you can type BadeBhai code line by line.

#### File Execution
```bash
cargo run path/to/your/file.bb
```

### Example Programs

#### Fibonacci Sequence (`fib.bb`)
```badebhai
fun fibonacci(n) {
    yehai a = 0;
    yehai b = 1;
    
    dikaho a;
    dikaho b;
    
    jabtak (n > 2) {
        yehai c = a + b;
        dikaho c;
        a = b;
        b = c;
        n = n - 1;
    }
}

fibonacci(10);
```

#### Simple Hello World (`test.bb`)
```badebhai
dikaho "Hello, BadeBhai Lang!";
```

## 🏗️ Architecture

BadeBhai Lang follows a tree-walk interpreter architecture with the following phases:

### 1. Scanning Phase
- **File**: `scanner.rs`
- **Purpose**: Converts source code into tokens
- **Features**: 
  - Handles Hindi/Hinglish keywords
  - Supports string literals, numbers, identifiers
  - Manages comments (single-line and multi-line)

### 2. Parsing Phase
- **File**: `parser.rs`
- **Purpose**: Builds Abstract Syntax Tree (AST) from tokens
- **Features**:
  - Recursive descent parser
  - Handles operator precedence
  - Error recovery mechanisms

### 3. Static Analysis
- **File**: `resolver.rs`
- **Purpose**: Performs semantic analysis and scope resolution
- **Features**:
  - Variable binding resolution
  - Scope chain management
  - Static error detection

### 4. Interpretation
- **File**: `interpreter.rs`
- **Purpose**: Executes the AST
- **Features**:
  - Tree-walk evaluation
  - Runtime environment management
  - Error handling and reporting

## 📁 Project Structure

```
BadeBhai-lang/
├── main/
│   ├── src/
│   │   ├── main.rs              # Entry point and REPL
│   │   ├── scanner.rs           # Lexical analysis
│   │   ├── token.rs             # Token definitions
│   │   ├── token_type.rs        # Token type enumeration
│   │   ├── parser.rs            # Syntax analysis
│   │   ├── expr.rs              # Expression AST nodes
│   │   ├── stmt.rs              # Statement AST nodes
│   │   ├── interpreter.rs       # AST evaluation
│   │   ├── resolver.rs          # Static analysis
│   │   ├── environment.rs       # Variable scope management
│   │   ├── object.rs            # Runtime value types
│   │   ├── callable.rs          # Function call interface
│   │   ├── lox_function.rs      # User-defined functions
│   │   ├── lox_class.rs         # Class definitions
│   │   ├── lox_instance.rs      # Object instances
│   │   ├── native_functions.rs  # Built-in functions
│   │   ├── error.rs             # Error handling
│   │   ├── ast_printer.rs       # AST debugging utility
│   │   ├── fib.bb               # Fibonacci example
│   │   ├── test.bb              # Simple test file
│   │   └── test2.bb             # Advanced examples
│   ├── Cargo.toml               # Rust project configuration
│   └── Cargo.lock               # Dependency lock file
├── assests/
│   ├── final_report.pdf         # Project documentation
│   └── flowdig.html             # Interpreter flow diagram
└── README.md                    # This file
```

## 🎯 Language Specification

### Data Types
- **Numbers**: 64-bit floating point (`123`, `3.14159`)
- **Strings**: UTF-8 text in double quotes (`"Hello"`)
- **Booleans**: `sahi` (true) and `galat` (false)
- **Nil**: `nil` represents absence of value

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Logical**: `aur` (and), `ya` (or), `!` (not)
- **Assignment**: `=`

### Statements
- **Expression Statement**: Any expression followed by `;`
- **Print Statement**: `dikaho expression;`
- **Variable Declaration**: `yehai identifier = expression;`
- **Block Statement**: `{ statements... }`
- **If Statement**: `if (condition) statement else statement`
- **While Loop**: `jabtak (condition) statement`
- **For Loop**: `for (initializer; condition; increment) statement`
- **Function Declaration**: `fun identifier(parameters) { body }`
- **Class Declaration**: `class identifier { methods... }`
- **Return Statement**: `return expression;`

### Built-in Functions
- **`clock()`**: Returns current time in milliseconds since Unix epoch

## 🐛 Error Handling

BadeBhai Lang provides comprehensive error reporting with:
- **Lexical Errors**: Invalid characters, unterminated strings
- **Syntax Errors**: Malformed expressions and statements
- **Runtime Errors**: Type mismatches, undefined variables
- **Semantic Errors**: Variable redefinition, invalid operations

## 🤝 Contributing

This project welcomes contributions! Here are some ways you can help:

1. **Report Bugs**: File issues for any bugs you encounter
2. **Suggest Features**: Propose new language features or improvements
3. **Add Examples**: Create more example programs in BadeBhai
4. **Improve Documentation**: Help make the documentation clearer
5. **Optimize Performance**: Suggest or implement performance improvements

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📚 Resources

- **Crafting Interpreters**: [https://craftinginterpreters.com/](https://craftinginterpreters.com/)
- **Rust Programming Language**: [https://www.rust-lang.org/](https://www.rust-lang.org/)
- **Tree-Walk Interpreter Flow**: See `assests/flowdig.html`

## 🎮 Examples Gallery

### Object-Oriented Programming
```badebhai
class Animal {
    init(name) {
        this.name = name;
    }
    
    speak() {
        dikaho this.name + " makes a sound";
    }
}

class Dog {
    init(name) {
        super.init(name);
    }
    
    speak() {
        dikaho this.name + " barks!";
    }
}

yehai dog = Dog("Buddy");
dog.speak(); // Output: Buddy barks!
```

### Higher-Order Functions
```badebhai
fun makeCounter() {
    yehai count = 0;
    fun increment() {
        count = count + 1;
        return count;
    }
    return increment;
}

yehai counter = makeCounter();
dikaho counter(); // 1
dikaho counter(); // 2
dikaho counter(); // 3
```

## 🔮 Future Enhancements

Potential improvements for future versions:
- **Standard Library**: Built-in modules for common operations
- **File I/O**: Reading and writing files
- **Error Handling**: Try-catch mechanisms
- **More Data Types**: Arrays, dictionaries, sets
- **Package System**: Module imports and exports
- **Debugging Tools**: Integrated debugger and profiler
- **Compilation**: Bytecode compilation for better performance

## 📄 License

This project is open source. Feel free to use, modify, and distribute as needed.

## 👥 Credits

- **Author**: Ujjwal238
- **Inspiration**: Robert Nystrom's "Crafting Interpreters"
- **Language**: Rust Programming Language
- **Community**: Hindi programming language enthusiasts

---

**Made with ❤️ for the Hindi programming community**

> "BadeBhai Lang: Making programming accessible in your mother tongue!"
