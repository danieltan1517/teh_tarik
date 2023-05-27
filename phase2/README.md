# Building A Parser in Rust

### Introduction

Now that the lexer has been built, the parser takes the sequence of tokens, and identifies the grammar of the programming language based off of the sequence of tokens. If the lexer is the component of the compiler that identifies the “words” and “punctuation” of a programming language, the parser is the the component of the compiler that identifiers the “sentences” and “paragraphs” of the programming language.

The job of the parser is to identify which tokens represent while loops, if statements, function headers, the function body, variable declarations, and constant variables. If the parser finds a misplaced sequence of tokens, the parser should notify the programming language user of the misplaced tokens.

In Phase 2, we will be creating a parser. A parser takes a sequence of tokens, and determines what the sequence of tokens represents. For example, a sequence of tokens in order: identifier, equals sign, number (e.g. “variable = 0”) will be recognized as a statement by the parser. The output of the parser will be a print out of the production rules of the parser.

### Building a Parser

We will be building a simple top down recursive descent parser without backtracking. Let's start with a simple declaration statement `int a;`. We can parse a simple declaration statement with the following pseudocode:

```
parse_declaration_statement(tokens: Array<Token>) -> Result(value,Error) {
    t := next_token(tokens)?
    if t != INT KEYWORD,
        return Err("expected integer keyword")

    t := next_token(tokens)?
    if t != IDENTIFIER,
        return Err("expected identifier")
    
    t := next_token(tokens)?
    if t != SEMICOLON,
        return Err("expected semicolon ';'")

    return Success
}
```

A simple declaration statement is a sequence of an `integer keyword`, followed by an `identifier`, followed by a `semicolon`. This simple pseudo code checks that the `tokens` has the specificed sequence.

### Branching Parser Behavior

Programming Language Grammars can have branching behavior that allows for expressive power. For example, there are multiple possibilities `int a;` , `int a = 0;`, or `int a = b;`. Here is some pseudocode to parse that grammar:

```
parse_declaration_statement(tokens: Array<Token>) -> Result(value,Error) {
    t := next_token(tokens)?
    if t != INT KEYWORD,
        return Err("expected integer keyword")

    t := next_token(tokens)?
    if t != IDENTIFIER,
        return Err("expected identifier")
    
    t := next_token(tokens)?
    if t == SEMICOLON,
        return Success

    if t == EQUAL {
       t := next_token(tokens)?
       if t == NUMBER
            t := next_token(tokens)?
            if t == SEMICOLON,
                return Success
            else
                return Err("expected semicolon ';'")
            
       if t == IDENTIFIER
            t := next_token(tokens)?
            if t == SEMICOLON,
                return Success
            else
                return Err("expected semicolon ';'")

       return Err("expected number or identifier")
    }

    return Err("expected semicolon ';' or '=' assignment operator")
}
```

### Lifetimes

[Lifetimes Documentation](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)

A lifetime is a construct the Rust borrow checker uses to ensure all reference borrows
are valid. A variable lifetime begins when it is created and ends when it is destroyed.
A lifetime annotates a program telling the programmer that a reference must live as long
as the original object itself. A reference cannot refer to invalid objects that no longer exists,
and lifetime annotations ensure that there are no dangling references.

Consider a function `longest_string` which returns a reference to `longest_string`. The lifetime
annotations are required here to ensure that return value reference lasts as long as `a` and `b`.
If the return value refence does not last as long as `a` and `b`, this will result in a compiler
error. Lifetime annotations is how Rust tracks reference times to make sure a program is correct.

```
fn longest_string<'a>(a: &'a str, b: &'a str) -> &'a str {
  if a.len() > b.len() {
    a
  } else {
    b
  }
}
```

We will be using lifetimes in this project for getting a valid reference to the list of tokens
and using the borrow checker to verify that a lifetime remains valid. Please note that there are two
sets of the same function: a function that returns a `Result` and another one which returns `Option`.
This is because there are situations where returning nothing is not an error.

```
fn peek<'a>(tokens: &'a Vec<Token>, index: usize) -> Option<&'a Token> {
    if index < tokens.len() {
        return Some(&tokens[index])
    } else {
        return None
    }
}

fn peek_error<'a>(tokens: &'a Vec<Token>, index: usize) -> Result<&'a Token, Box<dyn Error>> {
    if index < tokens.len() {
        return Ok(&tokens[index])
    } else {
        return Err(Box::from("expected a token, but got nothing"))
    }
}

fn next<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Option<&'a Token> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Some(&tokens[ret])
    } else {
        return None
    }
}

fn next_error<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, Box<dyn Error>> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Ok(&tokens[ret])
    } else {
        return Err(Box::from("expected a token, but got nothing"))
    }
}
```

### Simple Parsing Expression Exercise

In the `src/main.rs` file associated with Phase 2, there is a parser for parsing an arbitrarily complex mathematical expressions.
It takes a complex math expression, such as `1 + 2 * (3 + 4)`, and parses out the math expression. It returns an integer
answer as a result from the expression.

The grammar provided is one way to handle operator precedence expressions corrrectly.

Can you figure out how to compute the right answer to

### Building the Parser

Start by creating




