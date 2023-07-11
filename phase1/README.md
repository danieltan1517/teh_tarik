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
a finite automata using a `StateMachine` enum to represent the different states.

```
fn lex(code: &str) -> Result<Vec<Token>, String> {
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
                     return Err(String::from(message));
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
fn create_sign(start: usize, end: usize, code: &str) -> Result<Token, String> {
    let token = &code[start..end];
    match token {
    "<" => Ok(Token::Less),
    "<=" => Ok(Token::LessEqual),
    ">" => Ok(Token::Greater),
    ">=" => Ok(Token::GreaterEqual),
    "==" => Ok(Token::Equality),
    "=" => Ok(Token::Assign),
    "!=" => Ok(Token::NotEqual),
    _ => return Err(String::from(format!("invalid symbol {}", token))),
}
}
```
