# Phase 1: Building A Lexer in Rust

### Introduction

A lexer converts text into meaningful tokens belonging to categories defined by a "lexer" program. In a programming language compiler, a lexer partitions text into tokens such as identifiers, operators, grouping symbols, and data types.

For example, if we have a statement:
```
c = a + b;
```

The lexer should partition the statement into the following tokens:
```
Identifier("c")
Assign
Identifier("a")
Plus
Identifier("b")
Semicolon
```

To do another example, if we have a statement:
```
int [8] array;
```

The lexer should partition the statement into the following tokens:
```
Int
LeftBracket
Number(8)
RightBracket
Identifier("array")
Semicolon
```

For your lexer, your program should be written in the following way:
```
fn main() {

   // 1) open and read text contents from a file 
   // 2) lex the text contents of the file
   // 3) print out the tokens that have been lexed
}
```

The lexer takes as input a piece of code represented as a string, and outputs a list of tokens based on the input. If the lexer detects an invalid token, then the 
lexer spits out an error, and the compiler should halt. If there is an error detected, the compiler should inform the user about what is wrong.

The lexer must meet the following functionality:
- The lexer should have correct rules for detecting valid/invalid identifiers based on the language 
specification
- The lexer should detect and remove comments properly (e.g. `#This is a comment` should parse as 
one token and then removed from the list of tokens)
- The lexer should detect and report proper error messages for invalid identifiers and unrecognized 
symbols (e.g. invalid identifier tokens such as `2a` is an error)
The lexer is allowed to halt at the first error.
Please take note that a lexer does not need to check for errors such as misplaced semicolons, balanced parenthesis, or balanced curly brace. Handling those error cases is up to the parser in Phase 2.
Comments should not be part of the output list of tokens.


### Table of Tokens

For your lexer, this is the complete list of tokens you need to identify for Phase 1.

|Symbol                | Token Name   |
|----------------------|--------------|
|func                  | Func         |
|return                | Return       |
|int                   | Int          |
|print                 | Print        |
|read                  | Read         |
|while                 | While        |
|if                    | If           |
|else                  | Else         |
|break                 | Break        |
|continue              | Continue     |
|(                     | LeftParen    |
|)                     | RightParen   |
|{                     | LeftCurly    |
|}                     | RightCurly   |
|[                     | LeftBracket  |
|]                     | RightBracket |
|,                     | Comma        |
|;                     | Semicolon    |
|+                     | Plus         |
|-                     | Subtract     |
|*                     | Multiply     |
|/                     | Divide       |
|%                     | Modulus      |
|=                     | Assign       |
|<                     | Less         |
|<=                    | LessEqual    |
|>                     | Greater      |
|>=                    | GreaterEqual |
|==                    | Equality     |
|!=                    | NotEqual     |
|variable_name         | Ident        |
|10311517              | Num          |

#### Variable Identifier Names

Variables begin with an upper or lower case letters A-Z followed by a sequence of underscores or numbers. Examples include:
```
int variable_name;
int var1;
int october_31_1517;
```

#### Comments

Comments can be single line comments starting with `#`. For example:

```
int x; #This is a variable declaration.
```


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
multiple finite state machines using a `StateMachine` enum to represent the different states.
We will use if statements to execute different finite state machines.

```
fn lex(mut code: &str) -> Result<Vec<Token>, String> {
  let mut tokens: Vec<Token> = vec![];
  while code.len() > 0 {
    let (success, token, rest) = lex_number(code);
    if success {
      code = rest; 
      tokens.push(token);
      continue;
    } 
 
    let (success, rest) = lex_space(code);
    if success {
      code = rest;
      continue;
    }

    if code.starts_with("+") {
      code = &code[1..];
      tokens.push(Token::Plus);
      continue;
    }

    if code.starts_with("-") {
      code = &code[1..];
      tokens.push(Token::Subtract);
      continue;
    }

    if code.starts_with("*") {
      code = &code[1..];
      tokens.push(Token::Multiply);
      continue;
    }

    if code.starts_with("/") {
      code = &code[1..];
      tokens.push(Token::Divide);
      continue;
    }

    if code.starts_with("%") {
      code = &code[1..];
      tokens.push(Token::Modulus);
      continue;
    }

    if code.starts_with("=") {
      code = &code[1..];
      tokens.push(Token::Assign);
      continue;
    }

    let (success, token, rest) = lex_identifier(code);
    if success {
      code = rest;
      tokens.push(token);
      continue;
    }

    let symbol = unrecognized_symbol(code);
    return Err(format!("Unidentified symbol {symbol}"));

  }

  return Ok(tokens);
}
```

Create a "lex_numbers" state machine to recognize number such as 0, 12345, and 313. We
detect that at least the first letter in the string is an ASCII value '0' to '9', and make sure that
all letters after the first digit are also '0' to '9'.

```
// lex numbers.
fn lex_number(code: &str) -> (bool, Token, &str) {
  enum StateMachine {
    Start,
    Number,
  }

  let mut success = false;
  let mut state = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state {
    StateMachine::Start => {
      if letter >= '0' && letter <= '9' {
        state = StateMachine::Number;
        index += 1;
      } else {
        return (false, Token::NotToken, "");
      }
    }

    StateMachine::Number => {
      if letter >= '0' && letter <= '9' {
        state = StateMachine::Number;
        success = true;
        index += 1;
      } else {
        let num = code[..index].parse::<i32>().unwrap();
        return (true, Token::Num(num), &code[index..]);
      }
    }

    }
  }

  if success == true {
    let num: i32 = code.parse::<i32>().unwrap();
    return (true, Token::Num(num), "");
  } else {
    return (false, Token::NotToken, "");
  }
}
```

Create a "lex_identifier" state machine to recognize identifiers such as 'airplace', 'bay234', and 'variable3'. We
detect that at least the first letter in the string is an ASCII value 'a' to 'z' upper or lower case, 
and that the letters and digits following are also part of the identifier.
```
// lex identifiers.
fn lex_identifier(code: &str) -> (bool, Token, &str) {
  enum StateMachine {
    Start,
    Ident,
  }

  let mut success = false;
  let mut state = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state {
    StateMachine::Start => {
      if (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z'){
        state = StateMachine::Ident;
        index += 1;
      } else {
        return (false, Token::NotToken, "");
      }
    }

    StateMachine::Ident => {
      if (letter >= 'A' && letter <= 'Z') || (letter >= 'a' && letter <= 'z') || (letter >= '0' && letter <= '9') || letter == '_' {
        state = StateMachine::Ident;
        success = true;
        index += 1;
      } else {
        let token = &code[..index];
        return (true, create_identifier(token), &code[index..]);
      }
    }

    }
  }

  if success == true {
    return (true, create_identifier(code), "");
  } else {
    return (false, Token::NotToken, "");
  }
}
```

An identifier could be identified in a way similar to how numbers are identified, just add a state to the state machine
for identifiers. `if`, `while`, `read` keywords conflicts with identifiers. When creating a identifier, check to see that
the string is not a keyword. If it is in the list of keywords, return the appropriate keyword. Else, create a string
using `String::from(token)` and create an identifier token.

```
fn create_identifier(code: &str) -> Token {
  match code {
  "func" => Token::Func,
  "return" => Token::Return,
  "int" => Token::Int,

  // todo: implement all keywords...
  // ... all keywords...

  "read" => Token::Read,
  "while" => Token::While,
  "if" => Token::If,
  _ => Token::Ident(String::from(code)),
  }
}
```

### Submission
A correct and complete lexer should be able to lex all the example programs correctly, transforming 
the string into a list of tokens. At the end of lexing, print out the tokens using a for loop. An 
example of this can be found in “phase1/src/main.rs”.
Rubric
Demo/Group Participation 10 points
Proper Output for Example Test Cases 80 points (10 points each test case)
* add.tt
* array.tt
* break.tt
* function.tt
* if.tt
* loop.tt
* math.tt
* nested_loop.tt
  
Proper Output for Lexical Errors 10 points

All projects can be turned in up to 1 week late. Each day the project is late, 3% will be deducted per
day for up to 21%. After a week, projects will not be accepted.

