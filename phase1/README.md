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

Unlike C, Rust has two string types:
* `String`
* `&str`

`String` is a mutable resizable array of `u8` characters. `&str` is a read-only string reference 
that is read-only. String literals such as `"Dog"` are always `&str`.

```
let string\_ref: &str = "Dog";
let string: String = String::from(string\_ref);
let string: String = String::from("Dog");
let string\_ref: &str = &string;
```

The following code above demonstrates how to convert between `&str` and `String` types.

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

Structs are a grouping of variables together. Rust struct declarations are as follows:
```
struct Vec3 {
    x: float,
    y: float,
    z: float,
}
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




