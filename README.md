# AshLang
**AshLang** is a functional programming language built with **Rust**.

*⚠️Work in Progress⚠️*

## 😎Why I built this?
1. Wanted to explore the process of building a programming language.
2. Just wanted to merge the features of other languages that I love into one language.
    - Simple syntax like `Kotlin`, `Go`, `V` & `Zig`.
    - Dynamic like `Python` & `JavaScript`.
    - Null-Safety like `V`, `Dart` & `Swift`.
    - As Fast as `Dart`.
3. Because I can 😜

## ✨Features
- Simple Functional Syntax.
- Dynamic Typing.
- Null-Safety Support.
- Interpreted.
- Language Server Support for VSCode.
    - Syntax Highlighting
    - Code Formatting
- Execution support with `Code Runner`
- Open Source
## 🛠️Building
- Tokenizer
- Parser
- Interpreter
- Code Formatter
- LSP Server for VSCode
## 🤔Usage

Execute the code

    ash_lang run ./code.ash

Get Tokens from code

    ash_lang run --tkn ./code.ash

Get AST (*Abstract Syntax Tree*) from code

    ash_lang run --ast ./code.ash

Format the code

    ash_lang fmt ./code.ash

## 📖Docs
### Data Types
1. Int
2. Double
3. String
4. Boolean
5. List
6. Map

## 📦Example
```rust
// AshLang code to count the occurrence of numbers in a map
fn main(){
  let i = 0, nums = [1, 2, 2, 3, 3, 3], counter = {};
  while (i < len(nums)){
    let num = get(nums, i);
    if (!has(counter, num)){
      counter = set(counter, num, 1);
    } else {
      let val = get(counter, num);
      counter = set(counter, num, (val + 1));
    }
    i += 1;
  }
  println(counter);
}
// {1: 1, 2: 2, 3: 3}
```
More examples are located in the `examples/` folder.

## 💪Contributors
- **Ayush Chothe**
    - Language Design
    - Implementation
        - Tokenizer
        - Parser
        - Interpreter
        - Code Formatter
        - LSP Server for VsCode
