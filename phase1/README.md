# Building A Lexer in Rust

### Introduction

[How to install Rust](https://www.rust-lang.org/learn/get-started)

Rust is a compiled programming language designed for high performance just like
C and C++. Rust tracks lifetimes of memory and verifies software 
to prevent memory bugs, data races, and buffer overflows at a compiler level. Rust
uses a rich type system to allow Rust to formally verify the correctness of a program.

[Documentation for the Rust Programming Language](https://www.rust-lang.org/learn)

Rust uses a Borrow Checker to verify that software memory errors do not occur. Here
are the basic, simple rules of the borrow checker:
* Each value in Rust has a variable that is the owner
* There is only one mutable reference to a variable at a time
* There are multiple immutable references to a variable at a time
* References must always be valid. A reference cannot go out of scope
* Variables cannot be borrowed as mutable and immutable at the same time

There are many software programs that are unrepresentable when using a borrow checker. To
turn off the borrow checker to represent those structures you can use `unsafe` to 
represent those critical sections. Ideally, `unsafe` programs should be minimized.

### Getting Started

Create a new project with `cargo new compiler_project`.

`cd compiler_project/src`.

Open the `main.rs` file using a text editor/IDE.

### Hello Rust!

Let us begin with a simple "Hello Rust!" program:

```
fn main() {
    // hello rust!
    println!("Hello Rust!");
}
```

`println!` is a macro which prints out a string. A macro is denoted by the `!` after the
macro call. Rust macros are a powerful part of Rust's powerful metaprogramming system.

Just like C, comments in Rust are declared with `//` and multiline comments are denoted with
`/* comment */`. Unlike C, multiline comments in Rust can be nested.

To run your program, type `cargo run` to execute your program.

### Variable Declarations

Variable Declarations are declared with `let`, followed by the variable name.
In the C programming language, variables are mutable by default.
However, in Rust, variables are read-only constants by default.
To make a variable mutable, put 'mut' in front of the variable.

```
// declares a 'variable' that is read-only. 
// Type Inference tells us 'variable' is an integer
let variable = 0;

// here's a set of different ways to print 'variable' to the screen
// all of them do the same thing.
println!("variable = {}", variable);
println!("variable = {variable}");

// if you try to mutate 'variable', it will result in a compile error.
// uncomment the following line to get a compile error:
// variable += 100;
```

### Mutable Variable Declarations

A mutable variable declaration can be declared with `mut`.
```
// declare a mutable variable
let mut var = 0;
while var < 3 {
    // mutate the 'var'
    println!("var = {}", var);
    var += 1;
}
```

### Block Expressions Evaluation

You can denote a block of code using curly braces `{ }`. Unlike C, blocks of code can
evaluate to values and assigned to variables, as shown in the examples below:

```
// create a block of code that evaluates to an expression.
// https://doc.rust-lang.org/reference/expressions/block-expr.html
let v = {
    let mut num = 0;
    while num < 5 {
        num += 1;
    }
    num 
};

println!("v = {}", v);
```

Due to this programming language feature, this allows someone to omit the return keyword and
the code compiles correctly:
```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Strings and &str

[String Documentation](https://doc.rust-lang.org/std/string/struct.String.html)

Unlike C, Rust has two string types:
* `String`
* `&str`

`String` is a mutable resizable array of `u8` characters. `&str` is a read-only string reference 
that is read-only. String literals such as `"Dog"` are always `&str`.

```
let string_ref: &str = "Dog";
let string: String = String::from(string_ref);
let string: String = String::from("Dog");
let string_ref: &str = &string;
```

You can create a `String` through the `format!` macro.

```
let num1 = 50;
let num2 = 100;
let s: String = format!("{} + {} = {}", num1, num2, num1 + num2);
println!("{}", s);
```

The following code above demonstrates how to convert between `&str` and `String` types.

Rust allows someone to create substring references using `string_variable[start..end]`.

```
let name = "Ada Lovelace";
let first_name = &name[0..3];
let last_name =  &name[4..];
println!("{first_name}");
println!("{last_name}");
```

### References

Similar to C++, Rust has a reference type. References can be used to pass data read-only. A `&mut ` will
make the data modifiable by the function.

```
let mut num: i32 = 4;
function(&num);
function_with_ref(&mut num);
println!("num: i32 = {}", num);

fn function(num: &i32) {
    println!("num: &i32 = {}", num);
}

fn function_with_ref(num: &mut i32) {
    // add 200 to num.
    *num += 200;
    println!("num: &i32 = {}", num);
}
```

### Pattern Matching

`match` is the Rust equivalent to switch statements in C. Unlike C, `match` statements do not default to fallthrough.
`match` performs pattern matching on any arbitrary type. `_` is a default case, and if none of the cases are met, `_`
is executed.
```
let animal = "cat";
match animal {
"cow" => {
    println!("cow says: \"Moo!\"");
}
"cat" => {
    println!("cat says: \"Meow!\"");
}
"dog" => {
    println!("dog says: \"Wuff!\"");
}
_ => {
    println!("default case = {}", animal);
}

}

let num = 3;
match num {
1 => println!("January is the first month of the year."),
2 => println!("Febuary is the second month of the year."),
3 => println!("March is the third month of the year."),
_ => println!("...Etc."),
}
```

### Struct

Documentation: [Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

Structs are a grouping of variables together. Rust struct declarations are as follows:
```
struct Vec3 {
    x: float,
    y: float,
    z: float,
}
```

You can initialize a struct as follows:
```
let vec3 = Vec3 {
    x : 1.0,
    y : 1.0,
    z : 1.0,
};
```

### Enum

Documentation: [Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

Enums are a way of saying a value is one of a possible set of values.
Unlike C, Rust enums can have values associated with that particular enum value.
for example, a `Num` has a `i32` value associated exclusively with `Num`, 
but Plus, Subtract, Multiply, etc. have no values associated with it. Enums simulate
a "Sum Type" available within functional programming languages such as Haskell, Scheme, and Common Lisp,
and bring those concepts down to imperative compiled performant languages such as Rust.

```
#[derive(Debug, Clone)]
enum Token {
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign,
  Num(i32),
}
```

### Option and Result Types

Documentation: [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

Unlike C, Rust does not have a `null` type. Instead, `Errors` and `None` Types are
used to represent and document error pathways and make error pathways explicit. An
`Option` type is either `Some(value)`, or `None`, and Rust enforces that the programmer
checks that an `Option` unwraps correctly before accessing the `value` itself.

The `Option` enum can be summarized as follows:
```
enum Option {
    Some(val),
    None,
}
```

Option values can be unwrapped in the following ways:
```
let option: Option<i32> = Some(1);
if let Some(value) = option {
    // value has been unwrapped. use it.
    println!("value = {}", value);
}

match option {

Some(value) => {
    // value has been unwrapped. use it.
    println!("value = {}", value);
}

None => {

}

}
```

`Result` has almost similar functionality to `Option`. `Ok(value)` is equivalent to
`Some(value)`. Instead of `None`, however, `Err` in `Result` allows someone to send additional
error message information on top of `Err`.

```
enum Result {
    Ok(val),
    Err(error_info),
}
```

### Get Commandline Arguments

This is how to obtain commandline arguments from Rust. We will be using commandline arguments to pass the name
of the file to compile.

[Command Line Arguments Documentation](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)

```
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lexer.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }
}
```

To run your program with commandline arguments `arg1` `arg2` arg3`, type `cargo run arg1 arg2 arg3` to execute your program
with the commandline arguments.

### Opening and read the entire file

Code to open and read the entire file. This is used to get the all the high level programming language code
from the file.

```
use std::fs;

fn main() {
    let filename = "file.txt";
    let code = match fs::read_to_string(filename) {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
        code
    } 

    };

    println!("Code:");
    println!("{}", code);
}
```

### Building a simple lexer

Let's build a simple lexer that identifies numbers with multiple digits, as well
as basic math operations such as `+`, `-`, `*`, `/`. A lexer can be represented as
a finite automata using a `StateMachine` enum to represent the different states.

```
fn lex(code: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = vec![];
    let mut token_start: usize = 0;
    let mut token_end:   usize = 0;
    let mut line_num:    i32   = 1;
    let mut col_num:     i32   = 1;
    let mut state_machine = StateMachine::Init;

    for character in code.chars() {

        // state machine transitions.
        state_machine = match state_machine {

        StateMachine::Init => {
            token_start = token_end;
            if character >= '0' && character <= '9' {
                StateMachine::Number
            } else {
                StateMachine::Init
            }
        }

        StateMachine::Number => {
            if character >= '0' && character <= '9' {
                StateMachine::Number
            } else {
                let number = create_number(token_start, token_end, code);
                tokens.push(Token::Num(number));
                StateMachine::Init
            }
        }

        };

        token_end += 1;

        // actions of state machine.
        match state_machine {

        StateMachine::Init => {
             match character {
             '+' => tokens.push(Token::Plus),
             '-' => tokens.push(Token::Subtract),
             '*' => tokens.push(Token::Multiply),
             '/' => tokens.push(Token::Divide),
             '%' => tokens.push(Token::Modulus),
             '=' => tokens.push(Token::Assign),
              _  => {
                 if !character.is_whitespace() {
                     let ident = &code[token_start..token_end];
                     let message = format!("Error at line {}:{}. Unidentified symbol '{}'", line_num, col_num, ident);
                     return Err(Box::from(message));
                 }
             }

             }
        }

        StateMachine::Number => {}

        };

        if character == '\n' {
            col_num = 1;
            line_num += 1;
        } else {
            col_num += 1;
        }
    }

    if matches!(state_machine, StateMachine::Number) {
        let number = create_number(token_start, token_end, code);
        tokens.push(Token::Num(number));
    }

    return Ok(tokens);

    fn create_number(start: usize, end: usize, code: &str) -> i32 {
        // this code should correctly parse because the lexer verified that this is correct.
        // quit.
        let token = &code[start..end];
        match token.parse::<i32>() {
        Err(_) => panic!("Error. Logic Error: Lexer failed to lex number \"{token}\" correctly"),
        Ok(num) => num,
        }
    }

    enum StateMachine {
        Init,
        Number,
    }
}
```

These following lines are used to determine and location number of the tokens for the purposes of error
handling. These are important for telling users about the location of the error in the case of badly formed input.

```
if character == '\n' {
    col_num = 1;
    line_num += 1;
} else {
    col_num += 1;
}
```

### Hints on Identifiers

An identifier could be identified in a way similar to how numbers are identified, just add a state to the state machine
for identifiers. `if`, `while`, `read` keywords conflicts with identifiers. When creating a identifier, check to see that
the string is not a keyword. If it is in the list of keywords, return the appropriate keyword. Else, create a string
using `String::from(token)` and create an identifier token.

```
fn create_identifier(token_start: usize, token_end: usize, code: &str) -> Token {
    let token = &code[token_start..token_end];
    match token {
    "func" => Token::Func,
    "return" => Token::Return,
    "int" => Token::Int,

    // ... all keywords...

    "read" => Token::Read,
    "while" => Token::While,
    "if" => Token::If,
    _ => Token::Ident(String::from(token)),
    }
}
```

Just like the way number tokens are handled, you can handle identifiers in the same exact way.

```
let ident_token = create_identifier(start, end, code);
tokens.push(ident_token);
```

Note that there are multiple ways to do this, and this is not the only way to cleanly implement this.

### Hints on Sign

Signs can be handled in a similar way to identifiers. Just add a state to the state machine for sign tokens.

```
fn create_sign(start: usize, end: usize, code: &str) -> Result<Token, Box<dyn Error>> {
    let token = &code[start..end];
    match token {
    "<" => Ok(Token::Less),
    "<=" => Ok(Token::LessEqual),
    ">" => Ok(Token::Greater),
    ">=" => Ok(Token::GreaterEqual),
    "==" => Ok(Token::Equality),
    "=" => Ok(Token::Assign),
    "!=" => Ok(Token::NotEqual),
    _ => return Err(Box::from(format!("invalid symbol {}", token))),
}
}
```

### Testing

Testing different parts of software and testing parts of software as a whole is a key way of showing software robustness.
Testing is a good way of showing that regressions do not occur when adding new features to software. Rust has an excellent
testing framework that we will make use of.

[Writing Tests in Rust](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

[Assert](https://doc.rust-lang.org/std/macro.assert.html)

A module is a way to split up code in Rust into hierarchical logical units and manage visibility between them. A module can contain functions, structs, other kinds of code, etc. 

`assert!()` is a macro that  checks that a boolean expression returns `true`. If it is false, this triggers an error.

```
#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;

    #[test]
    fn lexer_test() {
        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));
    }

}
```

The `#[cfg(test)]` macro marks the module as a module used to test the code. The `#[test]` macro marks the function as a test
to be run by the compiler. Anything not marked `#[test]` will not be considered a test.

To run all tests, type `cargo test` to run all the tests.

To run only one specific test, type `cargo test lexer_test` to run only that particular test.

Use Rust tests to ensure that the software you write is robust.

# Tips and Tricks

### TODO

The Rust Compiler Attempts to be thorough in static analysis and verification. However, while developing code,
you may want to focus on certain parts and save the implementation details of other parts for later. You can
use the macro `todo!();` to mark up sections of the code that are unfinished.

```
fn complicated_function(x: i32, y: i32, z: i32) -> i32 {
    if x < y {
       // do complex code
    } else {
       // I need to implement this later...
       todo!()
    }
}
```

### Panic

panics occur when unexpected runtime behavior happens, such as array index out of bounds, or integer overflow/underflow.
A panic can be manually triggered using the `panic!` macro.

```
panic!("this is a panic.");
```

### Unwrap

You can use `unwrap` on a `Result` or `Option` type to unwrap the value inside the `Result` or `Option` type.
If `unwrap` is used on `None` or `Err` values, the program panics.

```
let value: Option<i32> = Some(100);
let number = value.unwrap();
```


