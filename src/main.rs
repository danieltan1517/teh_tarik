use std::env;
use std::fs;
use std::cmp::Ordering;

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
    let mut line_number = 1;
    let mut col_number  = 1;
    loop {
      let (token, rest) = lexer(&code);
      
      match token {
      TokenType::FunctionKeyword => {
        println!("[func] KEYWORD");
      }

      TokenType::Whitespace => {
        
      }

      TokenType::Newline => {
        println!("NEWLINE");
        line_number += 1;
        col_number = 1;
      }
     
      TokenType::Identifier(token) => {
        println!("IDENT [{}]", token);
      }

      TokenType::LeftParenthesis => {
        println!("LEFT PARENTHESIS '('");
      }

      TokenType::RightParenthesis => {
        println!("RIGHT PARENTHESIS ')'");
      }

      TokenType::Comma => {
        println!("COMMA ','");
      }

      TokenType::LeftCurlyBrace => {
        println!("LEFT CURLY BRACE '{{'");
      }

      TokenType::RightCurlyBrace => {
        println!("RIGHT CURLY BRACE '}}'");
      }
      TokenType:: Semicolon => {
        println!("SEMICOLON ';'");
      }

      TokenType::Plus => {
        println!("PLUS '+'");
      }

      TokenType::Subtract => { 
        println!("SUBTRACT '-'");
      }

      TokenType::Multiply => {
        println!("MULTIPLY '*'");
      }

      TokenType::Divide => {
        println!("DIVIDE '/'");
      }

      TokenType::Modulus => {
        println!("MODULUS '%'");
      }

      TokenType::Assign => {
        println!("ASSIGN '='");
      }
     
      TokenType::Error(message) => {
        println!("**Error at line {}, column {}: {}", line_number, col_number, message);
      }
     
      TokenType::EOF => {
        println!("END OF FILE");
        break;
      }
     
      }

      col_number += code.len() - rest.len();
      code = rest;
    }
}

enum TokenType {
  FunctionKeyword,
  LeftParenthesis,
  RightParenthesis,
  LeftCurlyBrace,
  RightCurlyBrace,
  Whitespace,
  Newline,
  Comma,
  Semicolon,

  // mathematical operators.
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign, // =

  // comparison operators
  /*Less,
  LessEqual,
  Equal,
  Greater,
  GreaterEqual,*/

  Identifier(String),
  Error(String),
  EOF,
}

fn lexer(code: &str) -> (TokenType, &str) {
    let mut state = StateMachine::Init;
    for (i, chr) in code.chars().enumerate() {
        match state {
        StateMachine::Init => {
          if chr == '\n' || chr == '\r' {
            let rest  = &code[i + 1..];
            return (TokenType::Newline, rest);
          } else if chr.is_whitespace() {
            state = StateMachine::Whitespace;
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
          } else if chr == ',' {
            return (TokenType::Comma, &code[1..]);
          } else if chr == ';' {
            let rest  = &code[i + 1..];
            return (TokenType::Semicolon, rest);
          } else if chr == '+' {
            let rest  = &code[i + 1..];
            return (TokenType::Plus, rest);
          } else if chr == '-' {
            let rest  = &code[i + 1..];
            return (TokenType::Subtract, rest);
          } else if chr == '*' {
            let rest  = &code[i + 1..];
            return (TokenType::Multiply, rest);
          } else if chr == '/' {
            let rest  = &code[i + 1..];
            return (TokenType::Divide, rest);
          } else if chr == '%' {
            let rest  = &code[i + 1..];
            return (TokenType::Modulus, rest);
          } else if chr == '=' {
            let rest  = &code[i + 1..];
            return (TokenType::Assign, rest);
          } else {
            let message = format!("Unidentified symbol '{}'", chr);
            let rest  = &code[i + 1..];
            return (TokenType::Error(message), rest);
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
            let token = &code[0..i];
            let rest  = &code[i..];
            let token_type = identifier_or_keyword(&token);
            return (token_type, rest);
          } else if chr.is_alphabetic() || chr == '_' ||  (chr >= '0' && chr <= '9') {
            state = StateMachine::Identifier;
          } else {
            let token = &code[0..i];
            let rest  = &code[i..];
            let token_type = identifier_or_keyword(&token);
            return (token_type, rest);
          }
        }

        StateMachine::Whitespace => {
          if chr.is_whitespace() {
            state = StateMachine::Whitespace;
          } else {
            let rest = &code[i..];
            return (TokenType::Whitespace, rest);
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
      Whitespace,
    }

    // subfunction to determine whether it is an identifier or keyword.
    fn identifier_or_keyword(token: &str) -> TokenType {
       if "func".cmp(&token) == Ordering::Equal {
         return TokenType::FunctionKeyword;
       }

       let ident: String = String::from(token);
       return TokenType::Identifier(ident);
    }
}











