# Doing Code Generation in Rust

### Introduction

Now that the lexer and parser is built, we can now do code generation.


### Unsafe

Rust has two models of operation: safe Rust and unsafe Rust. Safe Rust is the default Rust state,
where Rust uses the borrow checker and static analysis to keep programs safe and secure from data
races, memory leaks, or invalid memory access. In memory unsafe languages such as C, a lot of formal
verfication is required to get an optimizing compiler to optimize the code correctly, since pointers
may point to anything and do anything it wants. In Rust, the borrow checker uses a conservative static
analy
The static analysis done on a Rust program allows the program to be easy to optimize by the compiler. 
