use std::env;
use std::fs;
mod interpreter;

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

    Ok(generated_code) => {
        interpreter::execute_ir(&generated_code);
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
  Read,
  While,
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
  Less,
  LessEqual,
  Greater,
  GreaterEqual,

  Ident(String),
  Num(i32),
}

struct Expression {
  code: String,
  name: String,
}

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
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

    '<' => {
      i += 1;
      if i < bytes.len() {
        if (bytes[i] as char) == '=' {
          tokens.push(Token::LessEqual);
          i += 1;
        } else {
          tokens.push(Token::Less);
        }
      } else {
        tokens.push(Token::Less);
      }
    }

    '>' => {
      i += 1;
      if i < bytes.len() {
        if (bytes[i] as char) == '=' {
          tokens.push(Token::GreaterEqual);
          i += 1;
        } else {
          tokens.push(Token::Greater);
        }
      } else {
        tokens.push(Token::Greater);
      }
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
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    let mut generated_code = String::from("");
    loop {
        match parse_function(tokens, index)? {
        None => {
            break;
        }
        Some(func_code) => {
            generated_code += &func_code;
        }
        }
    }

    return Ok(generated_code);
}

// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<String>, String> {
    
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
    let func_ident = match next_result(tokens, index)? {
    Token::Ident(func_ident) => func_ident,
    _  => {return Err(String::from("functions must have a function identifier"));}
    };

    if !matches!(next_result(tokens, index)?, Token::LeftParen) {
        return Err(String::from("expected '('"));
    }
    if !matches!(next_result(tokens, index)?, Token::RightParen) {
        return Err(String::from("expected '('"));
    }

    if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
        return Err(String::from("expected '{'"));
    }

    let mut code = format!("%func {func_ident}()\n");

    loop {
        match parse_statement(tokens, index)? {
        None => {
            break;
        }
        Some(statement) => {
            code += &statement;
        }
        }
    }

    code += "%endfunc\n\n";

    if !matches!(next_result(tokens, index)?, Token::RightCurly) {
      return Err(String::from("expected '}'"));
    }

    return Ok(Some(code));
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<String>, String> {
    match peek(tokens, *index) {
    None => {
        return Ok(None);
    }

    Some(token) => {
        let codenode: Option<String>;
        match token {

        Token::RightCurly => {
            return Ok(None);
        }

        Token::Int => {
            *index += 1;
            match next_result(tokens, index)? {
            Token::Ident(ident) => {
                let statement = format!("%int {}\n", ident);
                codenode = Some(statement);
            }

            _ => {
                return Err(String::from("expected identifier"));
            }

            }
        }

        // while loop.
        Token::While => {
            *index += 1;
            todo!();
            // do code generation for while loops.
            let expr = parse_boolean_expr(tokens, index)?;

            if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
                return Err(String::from("expect '{' left curly brace token"));
            }

            // parsing the while loop body
            loop {
                match parse_statement(tokens, index)? {
                None => {
                    break;
                }
                Some(statement) => {
                    // += statement
                }
                }
            }

            if !matches!(next_result(tokens, index)?, Token::RightCurly) {
                return Err(String::from("expect '}' right curly brace token"));
            }

            codenode = Some(String::from(""));
            return Ok(codenode);
        }

        Token::Ident(ident) => {
            *index += 1;
            if !matches!(next_result(tokens, index)?, Token::Assign) {
                return Err(String::from("expected '=' assignment operator"));
            }
            let expr = parse_expression(tokens, index)?;
            let code = format!("{}%mov {}, {}\n", expr.code, ident, expr.name);
            codenode = Some(code);
        }

        Token::Return => {
            *index += 1;
            let expr = parse_expression(tokens, index)?;
            let code = format!("{}%ret {}\n", expr.code, expr.name);
            codenode = Some(code);
        }

        Token::Print => {
            *index += 1;
            if !matches!(next_result(tokens, index)?, Token::LeftParen) {
                return Err(String::from("expect '(' closing statement"));
            }

            let expr = parse_expression(tokens, index)?;
            let code = format!("{}%out {}\n", expr.code, expr.name);
            if !matches!(next_result(tokens, index)?, Token::RightParen) {
                return Err(String::from("expect ')' closing statement"));
            }
            codenode = Some(code);
        }

        Token::Read => {
            *index += 1;
            if !matches!(next_result(tokens, index)?, Token::LeftParen) {
                return Err(String::from("expect '(' closing statement"));
            }

            let expr = parse_expression(tokens, index)?;
            let code = format!("{}%input {}\n", expr.code, expr.name);

            if !matches!(next_result(tokens, index)?, Token::RightParen) {
                return Err(String::from("expect ')' closing statement"));
            }
            codenode = Some(code);
        }

        _ => {
             return Err(String::from("invalid statement."));
        }

        }

        if !matches!(next_result(tokens, index)?, Token::Semicolon) {
            return Err(String::from("expect ';' closing statement"));
        }

        return Ok(codenode);
    }

    }
}

// parsing an expression such as:
// "a" (alone)
// "a + b"
// "a * b"
// "a - b"
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut expr = parse_term(tokens, index)?;
    let opcode = match peek_result(tokens, *index)? {
    Token::Plus => "%add",
    Token::Subtract => "%sub",
    Token::Multiply => "%mult",
    Token::Divide => "%div",
    Token::Modulus => "%mod",

    _ => { 
        return Ok(expr); 
    }

    };

    *index += 1;
    let m_expr = parse_term(tokens, index)?;
    let t = create_temp();
    let instr = format!("%int {}\n{opcode} {}, {}, {}\n", t, t, expr.name, m_expr.name);
    expr.code += &m_expr.code;
    expr.code += &instr;
    expr.name = t;

    return Ok(expr);
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    match next_result(tokens, index)? {

    Token::Ident(ident) => {
        let expr = Expression {
            code : String::from(""),
            name : ident.clone(),
        };
        return Ok(expr);
    }

    Token::Num(num) => {
        let expr = Expression {
            code : String::from(""),
            name : format!("{}", num),
        };
        return Ok(expr);
    }

    _ => {
        return Err(String::from("invalid expression"));
    }

    }
}

fn parse_boolean_expr(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let expr1 = parse_term(tokens, index)?;
    if !matches!(next_result(tokens, index)?, Token::Less) {
        return Err(String::from("expected '<' less operator."));
    }
    let expr2 = parse_term(tokens, index)?;
    let t = create_temp();
    let code = format!("{}{}%int {t}\n%lt {t}, {}, {}\n", expr1.code, expr2.code, expr1.name, expr2.name);
    let name = t;
    let expr = Expression {
        code : code,
        name : name,
    };
    return Ok(expr);
}





