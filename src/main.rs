use std::env;
use std::fs;
use std::cmp::Ordering;
use std::iter::Peekable;

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
    let mut line_number: usize = 1;
    let mut col_number:  usize = 1;
    let mut token_array: Vec<LexerToken> = vec![];

    loop {
      let (token, rest) = lexer(&code);

      match token {
      TokenType::FunctionKeyword => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::FunctionKeyword,
        });
        col_number += 4;
      }

      TokenType::Whitespace => {
        col_number += code.len() - rest.len();
      }

      TokenType::Newline => {
        line_number += 1;
        col_number = 1;
      }
     
      TokenType::Identifier(ident) => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Identifier(ident.clone()),
        });
        col_number += ident.len();
      }

      TokenType::LeftParenthesis => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::LeftParenthesis,
        });
        col_number += 1;
      }

      TokenType::RightParenthesis => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::RightParenthesis,
        });
        col_number += 1;
      }

      TokenType::Comma => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Comma,
        });
        col_number += 1;
      }

      TokenType::LeftCurlyBrace => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::LeftCurlyBrace,
        });
        col_number += 1;
      }

      TokenType::RightCurlyBrace => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::RightCurlyBrace,
        });
        col_number += 1;
      }

      TokenType::IntKeyword => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::IntKeyword,
        });
        col_number += 3;
      }

      TokenType:: Semicolon => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Semicolon,
        });
        col_number += 1;
      }

      TokenType::Plus => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Plus,
        });
        col_number += 1;
      }

      TokenType::Subtract => { 
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Subtract,
        });
        col_number += 1;
      }

      TokenType::Multiply => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Multiply,
        });
        col_number += 1;
      }

      TokenType::Divide => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Divide,
        });
        col_number += 1;
      }

      TokenType::Modulus => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Modulus,
        });
        col_number += 1;
      }

      TokenType::Assign => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::Assign,
        });
        col_number += 1;
      }

      TokenType::ReturnKeyword => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::ReturnKeyword,
        });
        col_number += 6;
      }

      TokenType::PrintKeyword => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::PrintKeyword,
        });
        col_number += 5;
      }

      TokenType::InputKeyword => {
        token_array.push(LexerToken {
          line : line_number,
          column : col_number,
          token_type : TokenType::InputKeyword,
        });
        col_number += 5;
      }
     
      TokenType::Error(message) => {
        println!("**Error at line {}, column {}: {}", line_number, col_number, message);
      }
     
      TokenType::EOF => {
        break;
      }
     
      }

      code = rest;
    }

    let mut iter = token_array.into_iter().peekable();
    let node = parse_program(&mut iter);
    match node {

    CodeNode::Code(code) => {
        println!("{}", code);
    }
    CodeNode::Error(message) => {
        println!("{}", message);
    }
    CodeNode::Epsilon => {
        println!("**Error. There is no code provided.");
    }

    }
}

struct LexerToken {
  line:       usize,
  column:     usize,
  token_type: TokenType,
}

enum CodeNode {
  Code(String),
  Error(String),
  Epsilon,
}

#[derive(Debug)]
enum TokenType {
  FunctionKeyword,
  LeftParenthesis,
  RightParenthesis,
  LeftCurlyBrace,
  RightCurlyBrace,
  Whitespace,
  Newline,
  ReturnKeyword,
  IntKeyword,
  PrintKeyword,
  InputKeyword,
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
    let mut token = TokenType::EOF;
    let mut index = None::<usize>;
    for (i, chr) in code.chars().enumerate() {
        index = Some(i);
        match state {
        StateMachine::Init => {
          if chr == '\n' || chr == '\r' {
            token = TokenType::Newline;
            break;
          } else if chr.is_whitespace() {
            state = StateMachine::Whitespace;
          } else if chr >= '0' && chr <= '9' {
            state = StateMachine::Number;
          } else if chr.is_alphabetic() {
            state = StateMachine::Identifier;
          } else if chr == '(' {
            token = TokenType::LeftParenthesis;
            break;
          } else if chr == ')' {
            token = TokenType::RightParenthesis;
            break;
          } else if chr == '{' {
            token = TokenType::LeftCurlyBrace;
            break;
          } else if chr == '}' {
            token = TokenType::RightCurlyBrace;
            break;
          } else if chr == ',' {
            token = TokenType::Comma;
            break;
          } else if chr == ';' {
            token = TokenType::Semicolon;
            break;
          } else if chr == '+' {
            token = TokenType::Plus;
            break;
          } else if chr == '-' {
            token = TokenType::Subtract;
            break;
          } else if chr == '*' {
            token = TokenType::Multiply;
            break;
          } else if chr == '/' {
            token = TokenType::Divide;
            break;
          } else if chr == '%' {
            token = TokenType::Modulus;
            break;
          } else if chr == '=' {
            token = TokenType::Assign;
            break;
          } else {
            let message = format!("Unidentified symbol '{}'", chr);
            token = TokenType::Error(message);
            break;
          }
        }
       
        StateMachine::Number => {
          if chr >= '0' && chr <= '9' {
            state = StateMachine::Number;
          } else {
            let error_string = String::from("Numbers cannot have letters inside of them.");
            token = TokenType::Error(error_string);
            break;
          }
        }
       
        StateMachine::Identifier => {
          if chr.is_whitespace() {
            let ident = &code[0..i];
            token = identifier_or_keyword(&ident);
            break;
          } else if chr.is_alphabetic() || chr == '_' ||  (chr >= '0' && chr <= '9') {
            state = StateMachine::Identifier;
          } else {
            let ident = &code[0..i];
            token = identifier_or_keyword(&ident);
            break;
          }
        }

        StateMachine::Whitespace => {
          if chr.is_whitespace() {
            state = StateMachine::Whitespace;
          } else {
            token = TokenType::Whitespace;
            break;
          }
        }

        }
    }

    match index {
    None => {
      return (token, code);
    }
    Some(idx) => {
      let (_, rest) = code.split_at(if idx == 0 {1} else {idx});
      return (token, rest);
    }

    } 

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
       if "return".cmp(&token) == Ordering::Equal {
         return TokenType::ReturnKeyword;
       }
       if "int".cmp(&token) == Ordering::Equal {
         return TokenType::IntKeyword;
       }
       if "print".cmp(&token) == Ordering::Equal {
         return TokenType::PrintKeyword;
       }
       if "input".cmp(&token) == Ordering::Equal {
         return TokenType::InputKeyword;
       }

       let ident: String = String::from(token);
       return TokenType::Identifier(ident);
    }
}

fn parse_program(mut tokens: &mut Peekable<std::vec::IntoIter<LexerToken>> ) -> CodeNode {
    let mut func_count = 0;
    loop {
        let node = parse_function(&mut tokens);
        match node {
        CodeNode::Epsilon => {
            if func_count == 0 {
                let message = String::from("**Error. No input detected.");
                return CodeNode::Error(message);
            }
            break;
        }

        CodeNode::Error(_) => {
            return node;
        }

        CodeNode::Code(code) => {
            func_count += 1;
            println!("{}", code);
        }

        }
    }

    return CodeNode::Code(String::from("code"));
}

fn parse_function(mut tokens: &mut Peekable<std::vec::IntoIter<LexerToken>>) -> CodeNode {
    // parse 'func' keyword.
    let mut line: usize = 0;
    let mut column: usize = 0;

    match tokens.next() {
    None => {
      return CodeNode::Epsilon;
    }

    Some(tok) => {
        if matches!(tok.token_type, TokenType::FunctionKeyword) == false {
            line = tok.line;
            column = tok.column;
            let message = format!("**Error at line {}:{}: Functions must begin with the 'func' keyword.", line, column);
            return CodeNode::Error(message);
        }
    }

    };
   
    // parse 'ident' func.
    let func_ident = match tokens.next() {
    None => {
        let message = format!("**Error at line {}:{}: expected function identifier.", line, column);
        return CodeNode::Error(message);
    }

    Some(tok) => {
        match tok.token_type {
        TokenType::Identifier(ident) => {
            line = tok.line;
            column = tok.column;
            ident
        }

        _ => {
             line = tok.line;
             column = tok.column;
             let message = format!("**Error at line {}:{}. expected function identifier.", line, column);
             return CodeNode::Error(message);
        }

        }
    }

    };
    
    // todo!();
    
    match tokens.next() {
    None => {
        let message = format!("**Error at line {}:{}. expected left parenthesis.", line, column);
        return CodeNode::Error(message);
    }

    Some(tok) => {
        match tok.token_type {
        TokenType::LeftParenthesis => {
            line = tok.line;
            column = tok.column;
        }

        _ => {
             let message = format!("**Error at line {}:{}. expected left parenthesis.", tok.line, tok.column);
             return CodeNode::Error(message);
        }

        }
    }

    }

    // TODO: put function parameters
    match tokens.next() {
    None => {
        let message = format!("**Error at line {}:{}. expected right parenthesis.", line, column);
        return CodeNode::Error(message);
    }

    Some(tok) => {
        match tok.token_type {
        TokenType::RightParenthesis => {
            line = tok.line;
            column = tok.column;
        }

        _ => {
             let message = format!("**Error at line {}:{}. expected right parenthesis.", tok.line, tok.column);
             return CodeNode::Error(message);
        }

        }
    }

    }

    match tokens.next() {
    None => {
        let message = format!("**Error at line {}:{}. expected left curly brace.", line, column);
        return CodeNode::Error(message);
    }

    Some(tok) => {
        match tok.token_type {
        TokenType::LeftCurlyBrace => {
            line = tok.line;
            column = tok.column;
        }

        _ => {
             let message = format!("**Error at line {}:{}. expected left curly brace.", tok.line, tok.column);
             return CodeNode::Error(message);
        }

        }
    }

    }

    loop {
        let node = parse_statement(&mut tokens);
        match node {
        CodeNode::Epsilon => {
            break;
        }

        CodeNode::Error(ident) => {
            return CodeNode::Error(ident);
        }

        CodeNode::Code(_) => {

        }

        }
    }

    match tokens.next() {
    None => {
        let message = format!("**Error at line {}:{}. expected right curly brace.", line, column);
        return CodeNode::Error(message);
    }

    Some(tok) => {
        match tok.token_type {
        TokenType::RightCurlyBrace => {
             return CodeNode::Code(format!("func {}", func_ident));
        }

        _ => {
             line = tok.line;
             column = tok.column;
             let message = format!("**Error at line {}:{}. expected right curly brace.", line, column);
             return CodeNode::Error(message);
        }

        }
    }

    }
}


fn parse_statement(mut tokens: &mut Peekable<std::vec::IntoIter<LexerToken>>) -> CodeNode {
    let t = match tokens.peek() {
    None => {
        return CodeNode::Epsilon;
    }
    Some(tok) => {
        &tok
    }

    };

    match t.token_type {

    TokenType::RightCurlyBrace => {
         return CodeNode::Epsilon;
    }

    TokenType::IntKeyword => {
         // declaration.
    }

    TokenType::PrintKeyword => {
         // do print parsing.
    }

    TokenType::InputKeyword => {
         // do input keyword.
    }

    TokenType::Identifier(_) => {
         // expression.
    }

    _ => {
         let line = t.line;
         let column = t.column;
         let message = format!("**Error at line {}:{}. Invalid statement.", line, column);
         return CodeNode::Error(message);
    }

    }

    todo!()
}


