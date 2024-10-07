# Phase 2: Building A Parser in Rust

### Introduction

Now that the lexer has been built, we will now build the parser. A parser takes the sequence of tokens created by the lexer, and a creates structural representation of the input while checking for correct syntax. If the lexer is the component of the compiler that identifies the “words” and “punctuation” of a programming language, the parser is the the component of the compiler that identifiers the “sentences” and “paragraphs” of the programming language.

The job of the parser is to identify which sequence of tokens represent while loops, if statements, function headers, the function body, etc. If the parser finds a misplaced sequence of tokens, the parser should notify the programming language user of the misplaced tokens.

In Phase 2, we will be creating a parser. A parser takes a sequence of tokens, and determines what the sequence of tokens represents. For example, a sequence of tokens in order: identifier, equals sign, number (e.g. “variable = 0”) will be recognized as a statement by the parser. The output of the parser will be a print out of the production rules of the parser.

### Parser Grammer

A parser is based off of a context free grammar. You can find the proper grammar of the Teh Tarik Programming Lanuage [here](https://cs.ucr.edu/~dtan004/CS152_Parsing.pdf)

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

**In Phase 2, when you call a function when doing top-down recursive descent parsing, make sure to propagate the error back up the calling stack so that the error can be caught correctly.**


### matches!() statement

[matches macro](https://doc.rust-lang.org/std/macro.matches.html)

This macro returns `true` when the two parameters are equivalent, and returns `false` when the two parameters
are not equivalent.

```
let token = Token::Func;
if matches!(token, Token::Func) {
    println!("True");
} else {
    println!("False");
}
```

### Lifetimes (Optional. Not needed for this assignment)

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

fn peek_result<'a>(tokens: &'a Vec<Token>, index: usize) -> Result<&'a Token, String> {
    if index < tokens.len() {
        return Ok(&tokens[index])
    } else {
        return Err(String::from("expected a token, but got nothing"))
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

fn next_result<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, String> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Ok(&tokens[ret])
    } else {
        return Err(String::from("expected a token, but got nothing"))
    }
}
```

### ? Operator

Documentation: [? Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used)

The `?` operator is an error propagation operation. If result of the operation causes an error, the execution of the program
stops and the error value is returned. In order for `?` operator to function correctly, the error value type **must** match the 
the function it is returning from. If the result is not an error, the `?` operator unwraps the `Result` or `Option` automatically
for you. This technique can simplify the error handling code in Rust.

**In Phase 2, when you call a function, make sure to propagate the error back up the calling stack so that the error can be caught correctly.**

For example, if you have two functions `parse_function` and `parse_statement`, you can simplify the match statements using the `?` operator in the following way:
```
fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

    // 1.) this is the conventional match statement
    match parse_statement(tokens, index) {
        Ok(()) => {}
        Err(e) => {return Err(e);}
    }

    // 2.) this is another shorthand equivalent way to write 1.) 
    parse_statement(tokens, index)?;
}

fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    todo!()
}
```
This only works when the Result Error return values match.

### Simple Parsing Expression Exercise

In the `src/main.rs` file associated with Phase 2, there is a parser for parsing an arbitrarily complex mathematical expressions.
It takes a complex math expression, such as `1 + 2 * (3 + 4)`, and parses out the math expression. It returns an integer
answer as a result from the expression.

The grammar provided is one way to handle operator precedence expressions corrrectly.

Can you figure out how to compute the correct answer to expression given the operator precedence?

### Building a Top Down Parser

Start by creating a function called `parse_program`. It will take in a list of tokens and index marking where the parser is.
It will return a return a `Result`, where `Result` can either be `Err` or it will be fine.
```
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    loop {
        match parse_function(tokens, index)? {
        None => {
            break;
        }
        Some(_) => {}
        }
    }

    return Ok(());
}
```

A program consists of multiple functions, and we loop over the tokens, parsing out the functions.

We then create another function called `parse_function` that will parse the functions.

Assuming that the function grammar is as follows:
```
func main() {
    // insert statements here...
}
```
We can write `parse_function` like this:

```
// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {
    
    match next(tokens, index) {
    None => {
        return Ok(None);
    }
    Some(token) => {
        if !matches!(token, Token::Func) {
            return Err(String::from("functions must begin with func"));
        }
    }

    }
    match next_result(tokens, index)? {
    Token::Ident(_) => {},
    _  => {return Err(String::from("functions must have a function identifier"));}
    };

    if !matches!( next_result(tokens, index)?, Token::LeftParen) {
        return Err(String::from("expected '('"));
    }

    loop {
       match next_result(tokens, index)? {

       Token::RightParen => {
           break;
       }

       Token::Int => {
           match next_result(tokens, index)? {
           Token::Ident(_) => {
               match peek_result(tokens, *index)? {
               Token::Comma => {
                   *index += 1;
               }
               Token::RightParen => {}
               _ => {
                   return Err(String::from("expected ',' or ')'"));
               }

               }
           }
           _ => {
                return Err(String::from("expected ident function parameter"));
           }

           }
       }

       _ => {
           return Err(String::from("expected 'int' keyword or ')' token"));
       }

       }
    }


    if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
        return Err(String::from("expected '{'"));
    }

    loop {
        match parse_statement(tokens, index)? {
        None => {
            break;
        }
        Some(()) => {}
        }
    }


    if !matches!(next_result(tokens, index)?, Token::RightCurly) {
      return Err(String::from("expected '}'"));
    }

    return Ok(Some(()));
}
```

Writing `parse_statement` follows a similar pattern to `parse_function` and `parse_program`. You can
modify the `parse_expression` example to make it into a statement.

### Submission
A correct and complete parser should be able to parse all the example programs correctly, transforming 
the string into a list of tokens. At the end of lexing, print out the tokens using a for loop. An 
example of this can be found in “phase2/src/main.rs”.

A parser must have the following functionality:
* It must detect correct grammars correctly
* Reject incorrect grammars

Error messages need to be coherent and useful for users of the programming language. Examples of
good error messages can be something like: “Error: missing ‘;’ semicolon at the end of the 
statement.” Or “Error: missing ‘)’ right parenthesis.”

The parser should be able to detect the grammar correctly. If a given language input is missing either left or right parenthesis, it is incorrect. A valid language input must have balanced curly braces, and inputs with imbalanced curly brace must be rejected.

### Rubric

Total Points: 100 points total

Demo/Group Participation 10 points

Proper Output for Example Test Cases 80 points (10 points each test case):

* add.tt
* array.tt
* break.tt
* function.tt
* if.tt
* loop.tt
* math.tt
* nested_loop.tt
  
Proper Output for Parser Errors 10 points

All projects can be turned in up to 1 week late. Each day the project is late, 3% will be deducted per
day for up to 21%. After a week, projects will not be accepted.




