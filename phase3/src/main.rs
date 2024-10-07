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

    Ok(code) => {
        println!("Program Parsed Successfully.");
        println!("--------------------------------------------");
        println!("{code}");
        println!("--------------------------------------------");
        interpreter::execute_ir(&code);
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

  End,
}

struct Expression {
  code: String,
  name: String,
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

  tokens.push(Token::End);
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

// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));

    let mut code = String::new();
    while !at_end(tokens, *index) {
      match parse_function(tokens, index) {
      Ok(function_code) => {
        code += &function_code;
      }
      Err(e) => { return Err(e); }
      }
    }
    return Ok(code);
}

fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
  match tokens[index] {
  Token::End => { true }
  _ => { false }
  }
}

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}


// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    
    match tokens[*index] {
    Token::Func => { *index += 1; }
    _ => { return Err(String::from("functions must begin with func")); }
    }

    let mut function_code: String;

    match &tokens[*index] {
    Token::Ident(ident) => {
        *index += 1;
        function_code = format!("%func {ident}\n");
    }
    _  => { return Err(String::from("functions must have a function identifier"));}
    }


    match tokens[*index] {
    Token::LeftParen => { *index += 1; }
    _ => { return Err(String::from("expected '('"));}
    }

    match tokens[*index] {
    Token::RightParen => { *index += 1; }
    _ => { return Err(String::from("expected ')'"));}
    }

    match tokens[*index] {
    Token::LeftCurly => { *index += 1; }
    _ => { return Err(String::from("expected '{'"));}
    }

    while !matches!(tokens[*index], Token::RightCurly) {
        match parse_statement(tokens, index) {
        Ok(statement_code) => {
            function_code += &statement_code;
        }
        Err(e) => {return Err(e);}
        }
    }


    match tokens[*index] {
    Token::RightCurly => { *index += 1; }
    _ => { return Err(String::from("expected '}'"));}
    }

    function_code += "%endfunc\n";
    return Ok(function_code);
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    match tokens[*index] {
    Token::Int => parse_declaration_statement(tokens, index),
    Token::Ident(_) => parse_assignment_statement(tokens, index),
    Token::Return => parse_return_statement(tokens, index),
    Token::Print => parse_print_statement(tokens, index),
    Token::Read => parse_read_statement(tokens, index),
    _ => Err(String::from("invalid statement"))
    }
}

fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

    let statement: String;

    match tokens[*index] {
    Token::Int => {*index += 1;}
    _ => {return Err(String::from("Declaration statements must being with 'int' keyword"));}
    }

    match &tokens[*index] {
    Token::Ident(ident) => {
        *index += 1;
        statement = format!("%int {ident}\n");
    }
    _ => {return Err(String::from("Declarations must have an identifier"));}
    }

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statements must end with a semicolon"));}
    }

    return Ok(statement);
}

fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    let mut statement: String;

    let dest: String;
    match &tokens[*index] {
    Token::Ident(ident) => {
        *index += 1;
        dest = ident.clone(); // copy ident into variable.
    }
    _ => {return Err(String::from("Assignment statements must being with an identifier"));}
    }

    match tokens[*index] {
    Token::Assign => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the '=' operator"));}
    }

    match parse_expression(tokens, index) {
    Ok(expression) => {
        let src = expression.name;
        statement = expression.code;
        statement += &format!("%mov {dest}, {src}\n");
    },
    Err(e) => {return Err(e);}
    }

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    return Ok(statement);
}

fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    match tokens[*index] {
    Token::Return => {*index += 1;}
    _ => {return Err(String::from("Return statements must being with a return keyword"));}
    }

    match parse_expression(tokens, index) {
    Ok(_) => {},
    Err(e) => {return Err(e);}
    }

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    todo!()
}

fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    let expression: Expression;
    match tokens[*index] {
    Token::Print=> {*index += 1;}
    _ => {return Err(String::from("Print statements must being with a return keyword"));}
    }

    match parse_expression(tokens, index) {
    Ok(expr) => {
        expression = expr;
    },
    Err(e) => {return Err(e);}
    }

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    let mut statement = expression.code;
    statement += &format!("%out {}\n", expression.name);
    return Ok(statement);
}

fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    match tokens[*index] {
    Token::Read => {*index += 1;}
    _ => {return Err(String::from("Read statements must being with a 'read' keyword"));}
    }

    match parse_expression(tokens, index) {
    Ok(_) => {},
    Err(e) => {return Err(e);}
    }
    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    todo!()
}

// parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut expression: Expression;
    match parse_multiply_expression(tokens, index) {
    Ok(expr) => {
        expression = expr;
    },
    Err(e) => {return Err(e);}
    }
    loop {
       match tokens[*index] {

       Token::Plus => {
           *index += 1;
           match parse_multiply_expression(tokens, index) {
           Ok(expr2) => {
               let src1 = expression.name;
               let src2 = expr2.name;
               let dest = create_temp();
               expression.code += &expr2.code;
               expression.code += &format!("%int {dest}\n");
               expression.code += &format!("%add {dest}, {src1}, {src2}\n");
               expression.name = dest;
           },
           Err(e) => {return Err(e);}

           }
       }

       Token::Subtract => {
           *index += 1;
           match parse_multiply_expression(tokens, index) {
           Ok(expr2) => {
               let src1 = expression.name;
               let src2 = expr2.name;
               let dest = create_temp();
               expression.code += &expr2.code;
               expression.code += &format!("%int {dest}\n");
               expression.code += &format!("%add {dest}, {src1}, {src2}\n");
               expression.name = dest;
           },
           Err(e) => {return Err(e);}

           }
       }

       _ => { 
           break;
       }

       };
    }

    return Ok(expression);
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut expression: Expression;
    match parse_term(tokens, index) {
    Ok(expr) => {
        expression = expr;
    },
    Err(e) => {return Err(e);}
    }
    loop {
       match tokens[*index] {
       Token::Multiply => {
          *index += 1;
          match parse_term(tokens, index) {
          Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%mult {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
          Err(e) => {return Err(e);}
          }
       }

       Token::Divide => {
          *index += 1;
          match parse_term(tokens, index) {
          Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%div {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
          Err(e) => {return Err(e);}
          }
       }

       Token::Modulus => {
          *index += 1;
          match parse_term(tokens, index) {
          Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%mod {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
          Err(e) => {return Err(e);}
          }
       }
  
       _ => {
           break;
       }

       };

    }

    return Ok(expression);
}

// a term is either a Number or an Identifier.
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    match &tokens[*index] {

    Token::Ident(identifier) => {
        *index += 1;
        let expression = Expression {
            code : String::from(""),
            name : identifier.clone()
        };
        return Ok(expression);
    }

    Token::Num(number) => {
        *index += 1;
        let expression = Expression {
            code : String::from(""),
            name : number.to_string()
        };
        return Ok(expression);
    }

    Token::LeftParen => {
        *index += 1;
        let expression: Expression;
        match parse_expression(tokens, index) {
        Ok(e) => {expression = e;},
        Err(e) => {return Err(e);}
        }

        match tokens[*index] {
        Token::RightParen => {*index += 1;}
        _ => { return Err(String::from("missing right parenthesis ')'")); }
        }
        return Ok(expression);
    }
    
    _ => {
        return Err(String::from("missing expression term."));
    }

    }
}

