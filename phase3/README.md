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

For Phase 3, you will be doing code generation for the following statements:
* add
* math 
* array
* function

You will also be doing semantic error checking (see the semantic error checking Phase 3 section)

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

You can include other files, such as `interpeter.rs` using:

```
mod interpreter;
```

This will include the `interpreter.rs` file into `main.rs` for use. This is similar to `#include`
in C.


### Running the Example

Just like Phase 1 and Phase 2, you can hit `cargo run` to run the interpreter. To run the interpreter
on the examples, type `cargo run examples/add.tt` to run the interpreter on `examples/add.tt`. Observe
the control flow of the program and take note of how the intermediate representation is generated.

Output the intermediate represention as a `String`. Afterward, call `execute_ir` to interpret and
run the generated IR.

The following pseudocode describes the structure of the program, where `lex` is the lexing code from
Phase 1, and `parse` is from Phase 2.

```
let tokens = lex(code)?; 
let generated_code: String = parse(tokens)?;
interpreter::execute_ir(&generated_code);
```

Generate code as a `String` in the function `parse_program`. If `parse_program` is successful, 
call `execute_ir` to interpret and run the ir code. If `parse_program` fails, return an error.

```
match parse_program(&tokens, &mut index) {

Ok(generated_code) => {
    interpreter::execute_ir(&generated_code);
}

Err(e) => {
    // handle error message.
}

}
```

### Interpreter

Copy the `interpreter.rs` file and paste it into your project. In your main file `main.rs`, do the following:
```
mod interpreter;

fn main() {

  // ....rest of the compiler

  let generated_code: String = parse(tokens);
  interpreter::execute_ir(&generated_code);
}

```

You can include the interpreter found in `interpreter.rs` as part of your project. You do **not** need to make
any modifications to the interpreter. You can make any change you want to the existing interpreter code.
The interpreter code as found in `interpreter.rs` should be sufficient to complete Phase 3 and 4.

### IR Syntax and Semantics

An intermediate representation (IR) is the data structure or code used internally by an interpreter or compiler to
represent pseudo-assembly. A (low-level/backend) compiler generally takes the IR, performs compiler optimizations 
on the IR, and translates that IR into assembly language. The IR is a way to allow a compiler suite to target multiple
computer architectures and operating systems. The IR is a portable pseudo-assembly 
language representation that is further compiled into real assembly.

The IR of real-world compilers such as GCC or Clang can be incredibly difficult to program for, and
for this class, we will only be generating a simple IR built for teaching students compilers. We will
be generating IR for a provided interpreter, and running that interpreter to run the generated IR. The
interpreter is available in `interpreter.rs`.

**Pass the IR to the function `execute_ir` as a string, and the interpreter will run the code for you.**

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
| %input value              | store an integer from standard input into 'value'                                |
| %call dest, func(a,b)     | calls a function 'func' with parameters (a,b). Stores the return value in 'dest' |
| %ret  value               | return 'value' from the function.                                                |
| :label                    | declares a label ':label'. Used in branching code                                |
| %jmp  :label              | jumps to ':label' unconditionally                                                |
| %branch_if var, :label    | jumps to ':label' if var is 1. Does nothing if var is 0                          |
| %branch_ifn var, :label   | jumps to ':label' if var is 0. Does nothing if var is 1                          |

IR instructions take up exactly one line per instruction. You cannot output multiple IR instructions on a single line. 
Anything after the semicolon `;` will be treated as a comment.
The semicolon denotes a comment that goes until the end of the line.
```
%add c, a, b; adding 'a' and 'b' to get 'c'
```

### Translating Expressions into IR

Trivial Expressions such as `c = a + b;` can be translated trivially into `%add c, a, b` easily. However, more complex expressions, such as
`d = a + b * c` requires special handling. `b * c` needs to be done before adding it to `a`, because expressions must follow operator 
precedence. To do more complex expressions, store `b * c` in an intermediate register `temp` before adding it with `a` and getting `d`. For
example, `d = a + b * c` can be translated as:

```
%int temp
%mult temp, b, c   ; do b * c
%add d, a, temp    ; do d = a + b * c
```

There are many ways to translate a complex expression into assembly (e.g. `d = c + a + b` can be done as `d = (c + a) + b` or `d = c + (a + b)`).
As long as the answer to the expression remains the correct, any IR generated is fine.

To handle complex expressions, break down the complex expressions into smaller expressions, generate the code 
for the base case small expressions, and recursively generate the bigger expressions from the generated code of
the small expressions.

You can create an `Expression` struct to store the data from a subexpression. Use `code` to represent the generated code
from a subexpression, and `name` can be used to represent the intermediate values that store the results of the subexpression.

```
struct Expression {
  code: String,
  name: String,
}
```

### Output

In previous phases of the project, you created a lexical analyzer and parser for your custom 
programming language. In this phase of the project, you will take a syntactically correct program, 
verify it has no semantic errors, and generating the corresponding intermediate code for the given 
program. The generated code will be executed using the provided interpreter.

In every syntactically correct program written using the compiler, there must be at least one 
function named “main”, which will be the entry point of the problem, and a program should be able 
to define multiple functions in addition to “main”. “main” will take no parameters as an input, and 
“main” does not return a value.

Only integer scalar values can be passed as parameters to functions, and functions that are not main 
must return a single integer value. Functions cannot take arrays as argument parameters. Multiple 
return values are not supported, and functions outside of “main” cannot return nothing. All 
functions must return exactly one single integer value.

You can assume that all variable and function declarations will occur ahead of the time before usage
of the variable or function. Any function declared after where it is used can be assumed to be an 
error.

Just like the previous phase, you should perform one-pass code generation and directly output the 
generated code. There is no need to build or traverse a syntax tree. However, you will need to 
maintain a symbol table during code generation.

Do code generation **ONLY** for the following example programs:
* add.tt
* array.tt
* function.tt
* math.tt
  
We will do code generation for the other half of the example programs in the next and final phase.

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
    int c;
    a = 10;
    b = 2;
    c = add(a, b);
    print(c);
    c = mul(c, a + b);
    print(c);
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
%int c
%mov a, 10
%mov b, 2
%int _temp3
%call _temp3, add(a, b, )
%mov c, _temp3
%out c
%int _temp4
%add _temp4, a, b
%int _temp5
%call _temp5, mul(c, _temp4, )
%mov c, _temp5
%out c
%endfunc
```

The output of the program should be:
```
12
144
```

### Semantic Error Check

In addition to IR code generation, you must also catch semantic errors. The semantic errors you are assigned to catch are:

* Using a variable without having declared it
* Calling a function which has not been defined
* Not defining a main function
* Defining a variable more than once
* Type mismatch: using a scalar integer variable as an array of integers
* Type mismatch: using an array of integers as a scalar integer
* Creating an array of size <= 0.

You may optionally catch other possible semantic errors in addition to the ones list here (e.g. calling a function with the
wrong number of parameters), but **that is optional** and not required.

To catch semantic errors, you will need to create a data structure called a symbol table. A symbol table is a data structure 
**you create** that keeps track of each identifier along with some identifier data associated with it. Any data structure or combination of data structures may be used 
as a symbol table (e.g. arrays, linked lists, trees, hash tables). The most basic data structure you may create for this project is 
an array, then using linear search on an array to catch semantic errors.

### Rubric

Phase 3 will be graded out of a total of 100 points. Partial credit will be given if a test case is 
partially correct. Partial credit will be given if a test case is partially correct. Code correctness will 
be tested using the “cargo run” and/or “cargo test” command.

Demo/Group Participation/Code compiles 10 points

Each of the following test cases are worth 20 points each:
* add.tt
* math.tt
* array.tt
* function.tt

Error handling 10 points

All projects can be turned in up to 1 week late. Each day the project is late, 3% will be deducted per
day for up to 21%. After a week, projects will not be accepted. 

