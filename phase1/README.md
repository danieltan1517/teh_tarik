# Building A Lexer in Rust

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
