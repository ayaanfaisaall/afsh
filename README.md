# afsh - A Custom Shell and Scripting Language

`afsh` is a custom shell environment and scripting language built in Rust. It merges standard shell paradigms, such as pipelines, background execution, and redirections, with modern programming constructs like arrays, strict scoping, string interpolation, and logical control flow.

## Architecture

The `afsh` pipeline is built on top of modular, specialized crates:

* **[lexaf](https://crates.io/crates/lexaf):** The lexical analyzer and tokenizer. It acts as the "dumbest but fastest" step in the pipeline, reading raw text and categorizing it into structured tokens (words, keywords, strings, punctuation, and operators) while maintaining `Span` tracking.
* **[parsaf](https://crates.io/crates/parsaf):** The recursive descent parser and Abstract Syntax Tree (AST) generator. It consumes the flat sequence of tokens from `lexaf` and structures them into a logical, nested AST made up of `Stmt` nodes. It natively handles operator precedence, pipelines, control flow blocks, and strictly differentiates between raw shell commands and language base types.
* **interaf (In Development):** The interpreter. This component is currently in active development and will be integrated into the pipeline as soon as it is ready to execute the parsed AST.

*Note: Currently, running `afsh` will read your input, tokenize it, parse it, and output the generated AST (or visually display any syntax errors) to demonstrate the parsing capabilities while the interpreter is being finalized.*

## Core Features

### Prompt & Input Handling
`afsh` provides a robust, interactive command-line experience powered by `reedline`:
* **Vi Mode:** Native support for `vi` keybindings for efficient and familiar line editing right in the prompt.
* **Persistent History:** Command history is automatically saved and loaded across sessions via a `.afsh_history` file.

### Shell Primitives
* **Pipelines & Redirections:** Native shell parsing for pipes (`|`), standard inputs/outputs (`<`, `>`, `>>`), and background execution (`&`).
* **Logical Chaining:** Chain complex commands using `&&` and `||`.

### Modern Scripting Constructs
* **Variables & Data Types:** Supports assignment via the `let` keyword. Understands primitive values like numbers, booleans, strings, and structured arrays using brackets (`[]`).
* **Control Flow:** Write complex scripts with scoped `if` / `elif` / `else` blocks, `while` loops, and range-based iterators (`for iter in start to end`).
* **String Interpolation:** Dynamically evaluate variables embedded directly inside strings using curly braces (e.g., `"Welcome to {shell_name}"`).

### Developer Experience
* **Rich Error Reporting:** Thanks to the token span tracking in `lexaf`, `afsh` utilizes `miette` to present highly visual, pinpointed error diagnostics. If you forget to close a delimiter or write an invalid sequence, the shell points out exactly where it happened in your source code.

## af-lang

### Datatypes
af-lang consists of 5 datatypes:

* **Strings:** af-lang consists of interpolated strings which can resolve a variable inside them.
* **Numbers:** The maximum limit for the numbers which can be stored is 64 bits.
* **Decimals:** The maximum limit for the decimal (or float) is also 64 bits.
* **Bool:** af-lang also support bool (true or false), *(Note: 1 is not true neither 0 is false in af-lang.)*
* **Arrays:** The arrays in af-lang are a heterogenous dataset, you can add all of the datatypes inside arrays, (even arrays themselves).

### Keywords
Every statement inside af-lang returns an exitcode (i-e, 0, 1).

* **let:** let statement accepts any datatype to store in any variable, *(Note: A single word without "quotes" will be resolved as a variable.)*
* **print:** print statement allows all the datatypes like let, it also resolve single word as a variable.
* **if:** if statement in af-lang can have any command or any statement as its condition, it runs whenever a command returns a successful exitcode (i-e, 0).
* **while:** while statement also allows any statement or command inside its condition, and runs similar to if.
* **for:** for allows a loop for a given iterator over a given range.

### Example Syntax

```afsh
# Variable declaration and array support
let name = "afsh"
let target_ports = [80, 443, 80.80]

# String interpolation
print "Initializing {name} shell environment..."

# Conditional logic mixed with standard binary execution and chaining
if true {
    ping -c 1 8.8.8.8 && print "Internet is connected." || print "Offline mode."
}

# Native looping constructs
for i in 1 to 5 {
    print "Processing item..."
    break
}
```

### Unique Features
af-lang allows any statement inside if condition which means we can run this:

```afsh
if let a = {cat ~/any/file} {
    print a
} else {
    print "error: cat failed: file may not be present"
}
```

*Note: the scope of variable assignment is limited to if statement only, also when let fails, exitcode of let will be set to 1, which will fail the if statement and hence the else will run and print the error, without stopping the whole program.*

## License

afsh is licensed under MIT license.
