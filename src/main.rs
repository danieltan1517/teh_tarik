use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    // get commandline arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file.
    let filename = &args[1];
    let result = fs::read_to_string(filename);
    let high_level_code = match result {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
      code
    } 

    };

    // lexing the code.
    let mut code: &str = &high_level_code;
    println!("High Level Code: ");
    println!("{} ", code);
    println!("----------------");
    loop {
      let (token, rest) = lexer(&code);
      match token {
      TokenType::FunctionKeyword => {
        println!("function keyword");
      }
     
      TokenType::Identifier(token) => {
        println!("identifier [{}]", token);
      }

      TokenType::LeftParenthesis => {
        println!("left parenthesis '('");
      }

      TokenType::RightParenthesis => {
        println!("right parenthesis ')'");
      }

      TokenType::LeftCurlyBrace => {
        println!("left curly brace '{{'");
      }

      TokenType::RightCurlyBrace => {
        println!("right curly brace '}}'");
      }
     
      TokenType::Error(message) => {
        println!("**Error: {}", message);
      }
     
      TokenType::EOF => {
        println!("End of file");
        break;
      }
     
      }
      code = rest;
    }
}

enum TokenType {
  FunctionKeyword,
  LeftParenthesis,
  RightParenthesis,
  LeftCurlyBrace,
  RightCurlyBrace,
  Identifier(String),
  Error(String),
  EOF
}

fn lexer(code: &str) -> (TokenType, &str) {
    let mut state = StateMachine::Init;
    for (i, chr) in code.chars().enumerate() {
        match state {
        StateMachine::Init => {
          if chr.is_whitespace() {
            state = StateMachine::Init;
          } else if chr >= '0' && chr <= '9' {
            state = StateMachine::Number;
          } else if chr.is_alphabetic() {
            state = StateMachine::Identifier;
          } else if chr == '(' {
            let rest  = &code[i + 1..];
            return (TokenType::LeftParenthesis, rest);
          } else if chr == ')' {
            let rest  = &code[i + 1..];
            return (TokenType::RightParenthesis, rest);
          } else if chr == '{' {
            let rest  = &code[i + 1..];
            return (TokenType::LeftCurlyBrace, rest);
          } else if chr == '}' {
            let rest  = &code[i + 1..];
            return (TokenType::RightCurlyBrace, rest);
          }
        }
       
        StateMachine::Number => {
          if chr >= '0' && chr <= '9' {
            state = StateMachine::Number;
          } else {
            let error_string = String::from("Numbers cannot have letters inside of them.");
            return (TokenType::Error(error_string), "");
          }
        }
       
        StateMachine::Identifier => {
          if chr.is_whitespace() {
            let token = String::from(&code[0..i]);
            let rest  = &code[i..];
            return (TokenType::Identifier(token), rest);
          } else if chr.is_alphabetic() || chr == '_' ||  (chr >= '0' && chr <= '9') {
            state = StateMachine::Identifier;
          } else {
            let token = String::from(&code[0..i]);
            let rest  = &code[i..];
            return (TokenType::Identifier(token), rest);
          }
        }

        }
    }

    // split_at()
    return (TokenType::EOF, "");

    enum StateMachine {
      Init,
      Number,
      Identifier,
    }
}











