# Doing Code Generation in Rust

### Introduction

Now that the lexer and parser is built, we can now do code generation. You will take a high level 
language grammar and translate that high level language grammar into an intermediate representation.
Represent the intermediate representation as a `String`.

### Unsafe

Documentation: [Unsafe](https://doc.rust-lang.org/std/keyword.unsafe.html)

Rust has two models of operation: safe Rust and unsafe Rust. Safe Rust is the default Rust state,
where Rust uses the borrow checker and static analysis to keep programs safe and secure from data
races, memory leaks, or invalid memory access. However, safe Rust static analysis is limited: 
many operations that are safe are not allowed in safe Rust. Many operations such as accessing global state,
having multiple mutable pointers to the same memory location, inline assembly, etc. are not allowed in safe Rust. 
We can use `unsafe` to write code that is impossible in safe Rust due to the limitations of the borrow
checker. In this case, we will use `unsafe` to write/read global variables.

```
static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}
```

We will use this function to create a unique intermediate value every time the function is called. The
first time this function is called, it will create a string `_temp0`, the second time it is called, 
it will generate a string `_temp1`, and so on. `VAR_NUM` is a global variable.

### Include Files

You can include other files, such as `compiler.rs` using:

```
mod compiler;
```

This will include the `compiler.rs` file into `main.rs` for use. This is similar to `#include`
in C.


### Running the Example

Just like Phase 1 and Phase 2, you can hit `cargo run` to run the compiler. To run the compiler
on the examples, type `cargo run examples/add.tt` to run the compiler on `examples/add.tt`. Observe
the control flow of the program and take note of how the intermediate representation is generated.

Output the intermediate represention as a `String`. Afterwards, call `compile_and_run` to compile and
run the generated IR.

The following pseudocode describes the structure of the program, where `lex` is the lexing code from
Phase 1, and `parse` is from Phase 2.

```
let tokens = lex(code)?; 
let generated_code: String = parse(tokens)?;
compiler::compile_and_run(&generated_code);
```

Generate code as a `String` in the function `parse_program`. If `parse_program` is successful, 
call `compile_and_run` to compile and run the code. If `parse_program` fails, return an error.

```
match parse_program(&tokens, &mut index) {

Ok(generated_code) => {
    compiler::compile_and_run(&generated_code);
}

Err(e) => {
    // handle error message.
}

}
```

### Interpreter

You can include the interpreter found in `compiler.rs` as part of your project. You do **not** need to make
any modifications to the interpreter. You can make any change you want to the existing interpreter code.
The interpreter code as found in `compiler.rs` should be sufficient enough to complete Phase 3 and 4.

### IR Syntax and Semantics

An intermediate representation is the data structure or code used internally by a compiler to
represent pseudo-assembly. The compiler takes the IR, performs compiler optimizations on the IR,
and translates that IR into assembly language. The IR is a way to allow a compiler to target multiple
computer architectures, multiple CPUs, or multiple operating systems. The IR is a portable pseudo-
assembly language representation that is eventually compiled down into real assembly.

The real IR of real compilers such as GCC or Clang can be incredibly difficult to program for, and
for this class, we will only be generating a simple IR built for teaching students compilers. We will
be generating IR for a provided interpreter, and running that interpreter to run the generated IR. The
interpreter is available in `compiler.rs`.

| Instruction               | Description                                                                      |
| %func func(a,b,c)         | declares a function named 'function' with parameters(a,b,c) in that order        |
| %endfunc                  | closes the existing function                                                     |
| %int  variable            | declares a 32 bit integer value named 'variable'                                 |
| %int [] array, 32         | declares an array of 32 bit integers of length 32                                |
| %mov  dest, src1          | dest = src1                                                                      |
| %mov  [array + i], src1   | array[i] = src1                                                                  |
| %mov  dest, [array + i]   | dest = array[i]                                                                  |
| %add  dest, src1, src2    | dest = src1 +  src2                                                              |
| %sub  dest, src1, src2    | dest = src1 -  src2                                                              |
| %mult dest, src1, src2    | dest = src1 *  src2                                                              |
| %div  dest, src1, src2    | dest = src1 /  src2                                                              |
| %mod  dest, src1, src2    | dest = src1 %  src2                                                              |
| %lt   dest, src1, src2    | dest = src1 <  src2                                                              |  
| %le   dest, src1, src2    | dest = src1 <= src2                                                              |
| %neq  dest, src1, src2    | dest = src1 != src2                                                              |
| %eq   dest, src1, src2    | dest = src1 == src2                                                              |
| %gt   dest, src1, src2    | dest = src1 >  src2                                                              |
| %ge   dest, src1, src2    | dest = src1 >= src2                                                              |
| %out  value               | prints out the value to standard output                                          |
| %in   value               | store an integer from standard input into 'value'                                |
| %call dest, func(a,b)     | calls a function 'func' with parameters (a,b). Stores the return value in 'dest' |
| %ret  value               | return 'value' from the function.                                                |
| %label                    | declares a label '%label'. Used in branching code                                |
| %jmp  %label              | jumps to '%label' unconditionally                                                |
| %branch_if var, %label    | jumps to '%label' if var is 1. Does nothing if var is 0                          |
| %branch_ifn var, %label   | jumps to '%label' if var is 0. Does nothing if var is 1                          |





