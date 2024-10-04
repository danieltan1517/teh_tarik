use std::env;
use std::fs;

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
    let code = match result {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
      code
    } 

    };

    let tokens = match lex(&code) {
    Err(error_message) => {
        println!("**Error**");
        println!("----------------------");
        println!("{}", error_message);
        println!("----------------------");
        return;
    }

    Ok(tokens) => tokens,
    
    };

    let mut index: usize = 0;
    match parse_program(&tokens, &mut index) {

    Ok(()) => {
        println!("Program Parsed Successfully.");
    }

    Err(message) => {
        println!("**Error**");
        println!("----------------------");
        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else {
            println!("Error: {message}");
            println!("----------------------");
        }
    }

    }
}

#[derive(Debug)]
enum Token {
  // keywords:
  Func,
  Return,
  Int,
  Print,
  While,
  Read,
  If,

  LeftParen,
  RightParen,
  LeftCurly,
  RightCurly,
  Comma,
  Semicolon,

  // mathematical operators.
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign,

  Ident(String),
  Num(i32),
}

// This is a lexer that parses numbers/identifiers and math operations
fn lex(code: &str) -> Result<Vec<Token>, String> {
  let bytes = code.as_bytes();
  let mut tokens: Vec<Token> = vec![];

  let mut i = 0;
  while i < bytes.len() {
    let c = bytes[i] as char;

    match c {

    '0'..='9' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let digit = bytes[i] as char;
        if digit >= '0' && digit <= '9' {
          i += 1;
        } else {
          break;
        }
      }
      let end = i;
      let string_token = &code[start..end];
      let number_value = string_token.parse::<i32>().unwrap();
      let token = Token::Num(number_value);
      tokens.push(token);
    }

    'a'..='z' | 'A'..='Z' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let letter = bytes[i] as char;
        if (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z') || (letter >= '0' && letter <= '9') {
          i += 1;
        } else {
          break;
        }
      }
      let end = i;
      let string_token = &code[start..end];
      let token = create_identifier(string_token);
      tokens.push(token);
    }

    '+' => {
      tokens.push(Token::Plus);
      i += 1;
    }

    '-' => {
      tokens.push(Token::Subtract);
      i += 1;
    }

    '*' => {
      tokens.push(Token::Multiply);
      i += 1;
    }

    '/' => {
      tokens.push(Token::Divide);
      i += 1;
    }

    '%' => {
      tokens.push(Token::Modulus);
      i += 1;
    }

    '=' => {
      tokens.push(Token::Assign);
      i += 1;
    }

    ';' => {
      tokens.push(Token::Semicolon);
      i += 1;
    }

    '(' => {
      tokens.push(Token::LeftParen);
      i += 1;
    }

    ')' => {
      tokens.push(Token::RightParen);
      i += 1;
    }

    '{' => {
      tokens.push(Token::LeftCurly);
      i += 1;
    }

    '}' => {
      tokens.push(Token::RightCurly);
      i += 1;
    }

    ',' => {
      tokens.push(Token::Comma);
      i += 1;
    }

    '#' => {
      i += 1;
      while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '\n' {
          i += 1;
          break;
        }
        i += 1;
      }
    }

    ' ' | '\n' => {
      i += 1;
    }

    _ => {
      return Err(format!("Unrecognized symbol '{}'", c));
    }

    }
  }

  return Ok(tokens);
}

fn create_identifier(code: &str) -> Token {
  match code {
  "func" => Token::Func,
  "return" => Token::Return,
  "int" => Token::Int,
  "print" => Token::Print,
  "read" => Token::Read,
  "while" => Token::While,
  "if" => Token::If,
  _ => Token::Ident(String::from(code)),
  }
}

// the <'a> is the "lifetimes" type annotations in Rust.
//
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

// parse programs with multiple functions
// loop over everything, outputting generated code.
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

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {
    match peek(tokens, *index) {
    None => {
        return Ok(None);
    }

    Some(token) => {
        match token {

        Token::RightCurly => {
            return Ok(None);
        }

        Token::Int => {
            *index += 1;
            match next_result(tokens, index)? {
            Token::Ident(_) => {}

            _ => {
                return Err(String::from("expected identifier"));
            }

            }
        }

        Token::Ident(_) => {
            *index += 1;
            if !matches!(next_result(tokens, index)?, Token::Assign) {
                return Err(String::from("expected '=' assignment operator"));
            }
            parse_expression(tokens, index)?;
        }

        Token::Return => {
            *index += 1;
            parse_expression(tokens, index)?;
        }

        Token::Print => {
            *index += 1;
            if !matches!(next_result(tokens, index)?, Token::LeftParen) {
                return Err(String::from("expect '(' closing statement"));
            }
            parse_expression(tokens, index)?;
            if !matches!(next_result(tokens, index)?, Token::RightParen) {
                return Err(String::from("expect ')' closing statement"));
            }
        }

        Token::Read => {
            *index += 1;
            if !matches!(next_result(tokens, index)?, Token::LeftParen) {
                return Err(String::from("expect '(' closing statement"));
            }

            parse_expression(tokens, index)?;

            if !matches!(next_result(tokens, index)?, Token::RightParen) {
                return Err(String::from("expect ')' closing statement"));
            }
        }

        _ => {
             return Err(String::from("invalid statement."));
        }

        }
        if !matches!(next_result(tokens, index)?, Token::Semicolon) {
            return Err(String::from("expect ';' closing statement"));
        }

        return Ok(Some(()));
    }

    }
}

// parsing a simple expression such as:
// "a" (alone)
// "a + b"
// "a * b"
// "a - b"
// NOTE: this cannot parse "complex" expressions such as "a + b * c".
// I leave "a + b * c" as an exercise for the student.
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

    parse_multiply_expression(tokens, index)?;
    loop {
       match peek_result(tokens, *index)? {

       Token::Plus => {
           *index += 1;
           parse_multiply_expression(tokens, index)?;
       }

       Token::Subtract => {
           *index += 1;
           parse_multiply_expression(tokens, index)?;
       }

       _ => { 
           break;
       }

       };
    }

    return Ok(());
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    parse_term(tokens, index)?;
    loop {
       match peek_result(tokens, *index)? {
       Token::Multiply => {
          *index += 1;
          parse_term(tokens, index)?;
       }

       Token::Divide => {
          *index += 1;
          parse_term(tokens, index)?;
       }

       Token::Modulus => {
          *index += 1;
          parse_term(tokens, index)?;
       }
  
       _ => {
           break;
       }

       };

    }

    return Ok(());
}

// a term is either a Number or an Identifier.
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    match next_result(tokens, index)? {

    Token::Ident(_) => {
        return Ok(());
    }

    Token::Num(_) => {
        return Ok(());
    }

    Token::LeftParen => {
        parse_expression(tokens, index)?;
        if !matches!(next_result(tokens, index)?, Token::RightParen) {
            return Err(String::from("missing right parenthesis ')'"));
        }

        return Ok(());
    }

    _ => {
        return Err(String::from("invalid expression"));
    }

    }
}


// writing tests!
#[cfg(test)]
mod tests {
    use crate::lex;
    use crate::parse_statement;

    #[test]
    fn test_statements() {

        // test that valid statements are correct.
        let tokens = lex("a = 1 + 2;").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        let tokens = lex("b = 1 / 2;").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();


        // test errors. missing semicolon
        let tokens = lex("b = 1 / 2").unwrap();
        assert!(matches!(parse_statement(&tokens, &mut 0), Err(_)));

    }

}

