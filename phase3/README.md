# Doing Simple Code Generation in Rust

### Introduction

Now that the lexer and parser is built, we can now do code generation. You will take a high level 
language grammar and translate that high level language grammar into an intermediate representation.
Represent the intermediate representation as a `String`.

We will be splitting code generation into two halves: "simple" code generation and "complicated" code
generation. In "simple" code generation, you will be doing function calls, move statements, arithmetic
statements, return statements, and input/output statements. Anything that is not a function call, move statement, or
arithmetic statement will be done in the second half of the assignment. **This means that comparison
operations, labels, and branches will NOT be done during this phase. Loops and branching statements
will be done in Phase 4.**

We are doing only "simple" code generation, that is code that contains linear control flow and starts from
the top and ends at the bottom, with no branches or jumping around. 

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

Here is the entire instruction set IR for the interpreter you will be using to run the generated code:

| Instruction               | Description                                                                      |
|---------------------------|----------------------------------------------------------------------------------|
| %func func(%int a, %int b)| declares a function named 'function' with parameters a and b in that order       |
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

### Generated Example IR Code

Here are some examples of possible generated IR outputs. One can generate any IR code for the given code, as
long as the generated IR functions in the same way. **Any IR generated is acceptable, as long as it outputs
the same numbers**.

#### add
Given the following `add.tt` program:

```
func main() {
  int a;
  int b;
  int c;
  a = 100;
  b = 50;
  c = a + b;
  print(c);
}
```

Here is a possible generated IR code for `add.tt`:

```
%func main()
%int a
%int b
%int c
%mov a, 100 
%mov b, 50 
%add c, a, b
%out c
%endfunc
```

The output of `add.tt` should be:
```
150
```

---
#### math
Given the following `math.tt` program:
```
# A simple program which shows mathematical operations.

func main() {
  int a;
  int b;
  int c;

  a = 100;
  b = 50;

  # This should output '150'
  c = a + b;
  print(c);

  # This should output '50'
  c = a - b;
  print(c);

  # This should output '5000'
  c = a * b;
  print(c);

  # This should output '2'
  c = a / b;
  print(c);

  # This should output '0'
  c = a % b;
  print(c);

  # Complex Expression. (4 + 2) * 7
  a = 4;
  b = 7;
  c = (a + 2) * b;
  print(c);
}
```

Here's a possible generated IR code for `math.tt`:
```
%func main()
%int a
%int b
%int c
%mov a, 100
%mov b, 50
%int _temp1
%add _temp1, a, b
%mov c, _temp1
%out c
%int _temp2
%sub _temp2, a, b
%mov c, _temp2
%out c
%int _temp3
%mult _temp3, a, b
%mov c, _temp3
%out c
%int _temp4
%div _temp4, a, b
%mov c, _temp4
%out c
%int _temp5
%mod _temp5, a, b
%mov c, _temp5
%out c
%mov a, 4
%mov b, 7
%int _temp6
%add _temp6, a, 2
%int _temp7
%mult _temp7, _temp6, b
%mov c, _temp7
%out c
%endfunc
```

Take note that `c = (a + 2) * b;` is broken down into multiple instructions, with several intermediate registers to restore temporary results.

```
%int _temp6
%add _temp6, a, 2
%int _temp7
%mult _temp7, _temp6, b
%mov c, _temp7
```

The output of `math.tt` should be:
```
150
50
5000
2
0
42
```

---
#### array

Given the follow `array.tt` example:

```
func main() {
    int [4] array;

    # Should print out '2'
    array[0] = 2;
    print(array[0]);

    # Should print out '4'
    array[1] = array[0] + array[0];
    print(array[1]);

    # Should print out '8'
    array[2] = array[1] + 2 * 2;
    print(array[2]);

}
```

An example possible IR could be:
```
%func main()
%int[] array, 4
%mov [array + 0], 2
%int _temp1
%mov _temp1, [array + 0]
%out _temp1
%int _temp2
%mov _temp2, [array + 0]
%int _temp3
%mov _temp3, [array + 0]
%int _temp4
%add _temp4, _temp2, _temp3
%mov [array + 1], _temp4
%int _temp5
%mov _temp5, [array + 1]
%out _temp5
%int _temp6
%mov _temp6, [array + 1]
%int _temp7
%mult _temp7, 2, 2
%int _temp8
%add _temp8, _temp6, _temp7
%mov [array + 2], _temp8
%int _temp9
%mov _temp9, [array + 2]
%out _temp9
%endfunc
```

When running the intepreter, it should output:
```
2
4
8
```
---
### function

Given the following `function.tt`:
```
func add(int a, int b) {
    return a + b;
}

func mul(int a, int b) {
     return a * b;
}

func main() {
    int a;
    int b;
    a = add(10, 2);
    print(a);
    b = mul(a, a + b);
    print(b);
}
```

You can output the following IR:
```
%func add(%int a, %int b, )
%int _temp1
%add _temp1, a, b
%ret _temp1
%endfunc

%func mul(%int a, %int b, )
%int _temp2
%mult _temp2, a, b
%ret _temp2
%endfunc

%func main()
%int a
%int b
%int _temp3
%call _temp3, add(10, 2, )
%mov a, _temp3
%out a
%int _temp4
%add _temp4, a, b
%int _temp5
%call _temp5, mul(a, _temp4, )
%mov b, _temp5
%out b
%endfunc
```

The output of the program should be:
```
12
144
```


