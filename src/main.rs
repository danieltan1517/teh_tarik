use std::env;
use std::fs;
use std::error::Error;

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

    let (tokens, location) = match lex(&code) {
    Err(error_message) => {
        println!("**Error**");
        println!("----------------------");
        println!("{}", error_message);
        println!("----------------------");
        return;
    }

    Ok(data) => data,
    
    };
    
    let mut index: usize = 0;
    match parse_program(&tokens, &mut index) {

    Ok(generated_code) => {
        println!("{}", generated_code);
    }

    Err(message) => {
        println!("**Error**");
        println!("----------------------");
        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else if index >= tokens.len() {
            index = tokens.len() - 1;
            let loc = &location[index];
            let tok = &tokens[index];
            print_error(loc.line, loc.col, tok, &message);
        } else {
            let loc = &location[index];
            let tok = &tokens[index];
            print_error(loc.line, loc.col, tok, &message);
        }

    }

    }

    fn print_error<T: std::fmt::Display>(line: i32, col: i32, tok: &Token, message: T) {
        match tok {
        Token::Func => {
            println!("Error at line {}:{}. {}", line, col, message);
        }
        Token::Return => {
            println!("Error at line {}:{}. unexpected 'return' keyword. {}", line, col, message);
        }
        Token::Int => {
            println!("Error at line {}:{}. unexpected 'int' keyword. {}", line, col, message);
        }
        Token::Print => {
            println!("Error at line {}:{}. unexpected 'print' keyword. {}", line, col, message);
        }
        Token::Read => {
            println!("Error at line {}:{}. unexpected 'read' keyword. {}", line, col, message);
        }

        Token::LeftParen => {
            println!("Error at line {}:{}. unexpected '('. {}", line, col, message);
        }
        Token::RightParen => {
            println!("Error at line {}:{}. unexpected ')'. {}", line, col, message);
        }
        Token::LeftCurly => {
            println!("Error at line {}:{}. unexpected '{{'. {}", line, col, message);
        }
        Token::RightCurly => {
            println!("Error at line {}:{}. unexpected '}}'. {}", line, col, message);
        }
        Token::Comma => {
            println!("Error at line {}:{}. unexpected ','. {}", line, col, message);
        }
        Token::Semicolon => {
            println!("Error at line {}:{}. unexpected ';'. {}", line, col, message);
        }

        Token::Plus => {
            println!("Error at line {}:{}. unexpected '+'. {}", line, col, message);
        }
        Token::Subtract => {
            println!("Error at line {}:{}. unexpected '-'. {}", line, col, message);
        }
        Token::Multiply => {
            println!("Error at line {}:{}. unexpected '*'. {}", line, col, message);
        }
        Token::Divide => {
            println!("Error at line {}:{}. unexpected '/'. {}", line, col, message);
        }
        Token::Modulus => {
            println!("Error at line {}:{}. unexpected '%'. {}", line, col, message);
        }
        Token::Assign => {
            println!("Error at line {}:{}. unexpected '='. {}", line, col, message);
        }

        Token::Ident(ident) => {
            println!("Error at line {}:{}. invalid identifier {}. {}", line, col, ident, message);
        }
        Token::Num(num) => {
            println!("Error at line {}:{}. invalid identifier {}. {}", line, col, num, message);
        }

        }
        println!("----------------------");
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

  // comparison operators
  /*Less,
  LessEqual,
  Equal,
  Greater,
  GreaterEqual,*/

  Ident(String),
  Num(i32),
}

struct Loc {
  line: i32,
  col:  i32,
}

fn lex(code: &str) -> Result<(Vec<Token>, Vec<Loc>), Box<dyn Error>> {
    let mut tokens: Vec<Token> = vec![];
    let mut locations: Vec<Loc> = vec![];
    let mut token_start: usize = 0;
    let mut token_end:   usize = 0;
    let mut line_num:    i32   = 1;
    let mut col_num:     i32   = 1;
    let mut state_machine = StateMachine::Init;

    for character in code.chars() {

        // transitions.
        state_machine = match state_machine {

        StateMachine::Init => {
            token_start = token_end;
            if character.is_alphabetic() {
                StateMachine::Ident
            } else if character >= '0' && character <= '9' {
                StateMachine::Number
            } else {
                StateMachine::Init
            }
        }

        StateMachine::Number => {
            if character >= '0' && character <= '9' {
                StateMachine::Number
            } else if character.is_alphabetic() || character == '_' {
                StateMachine::ErrorNum
            } else {
                let number = create_number(token_start, token_end, code);
                add(&mut tokens, &mut locations, Token::Num(number), line_num, col_num);
                StateMachine::Init
            }
        }

        StateMachine::Ident => {
            if character.is_alphabetic() || (character >= '0' && character <= '9') || character == '_' {
                StateMachine::Ident
            } else {
                let ident = create_identifier(token_start, token_end, code);
                add(&mut tokens, &mut locations, ident, line_num, col_num);
                token_start = token_end;
                StateMachine::Init
            }
        }

        StateMachine::ErrorNum => StateMachine::ErrorNum,

        };

        token_end += 1;

        // actions of state machine.
        match state_machine {

        StateMachine::Init => {
             // token_start = token_end;
             match character {
             '+' => add(&mut tokens, &mut locations, Token::Plus, line_num, col_num),
             '-' => add(&mut tokens, &mut locations, Token::Subtract, line_num, col_num),
             '*' => add(&mut tokens, &mut locations, Token::Multiply, line_num,col_num),
             '/' => add(&mut tokens, &mut locations, Token::Divide, line_num, col_num),
             '%' => add(&mut tokens, &mut locations, Token::Modulus, line_num, col_num),
             ',' => add(&mut tokens, &mut locations, Token::Comma, line_num, col_num),
             '{' => add(&mut tokens, &mut locations, Token::LeftCurly, line_num, col_num),
             '}' => add(&mut tokens, &mut locations, Token::RightCurly, line_num, col_num),
             '(' => add(&mut tokens, &mut locations, Token::LeftParen, line_num, col_num),
             ')' => add(&mut tokens, &mut locations, Token::RightParen, line_num, col_num),
             ';' => add(&mut tokens, &mut locations, Token::Semicolon, line_num, col_num),
             '=' => add(&mut tokens, &mut locations, Token::Assign, line_num, col_num),
              _  => {
                 if !character.is_whitespace() {
                     let ident = &code[token_start..token_end];
                     let message = format!("Error at line {}:{}. Unidentified symbol '{}'", line_num, col_num, ident);
                     return Err(Box::from(message));
                 }
             }

             }
        }

        StateMachine::Number => {

        }

        StateMachine::Ident => {

        }

        StateMachine::ErrorNum => {
            if character == ' ' {
                let ident = &code[token_start..token_end];
                let message = format!("Error at line {}:{}. Invalid Number '{}'", line_num, col_num, ident);
                return Err(Box::from(message));
            }
        }

        };

        if character == '\n' {
            col_num = 1;
            line_num += 1;
        } else {
            col_num += 1;
        }
    }

    match state_machine {
    StateMachine::Number => {
        let number = create_number(token_start, token_end, code);
        add(&mut tokens, &mut locations, Token::Num(number), line_num, col_num);
    }
    StateMachine::Ident => {
        let ident = create_identifier(token_start, token_end, code);
        add(&mut tokens, &mut locations, ident, line_num, col_num);
    }
    _ => {}
    }

    return Ok((tokens, locations));

    // helper functions
    fn create_identifier(token_start: usize, token_end: usize, code: &str) -> Token {
        let token = &code[token_start..token_end];
        match token {
        "func" => Token::Func,
        "return" => Token::Return,
        "int" => Token::Int,
        "print" => Token::Print,
        "read" => Token::Read,
        _ => Token::Ident(String::from(token))
        }
    }

    fn add(tokens: &mut Vec<Token>, locations: &mut Vec<Loc>, tok: Token, line: i32, col: i32) {
        tokens.push(tok);
        locations.push(Loc{line, col});
    }

    fn create_number(token_start: usize, token_end: usize, code: &str) -> i32 {
        // this code should correctly parse because the lexer verified that this is correct.
        // quit.
        let token = &code[token_start..token_end];
        match token.parse::<i32>() {
        Err(_) => panic!("Error. Logic Error: Lexer failed to lex number \"{token}\" correctly"),
        Ok(num) => num,
        }
    }

    enum StateMachine {
        Init,
        Number,
        Ident,
        ErrorNum,
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

fn peek_error<'a>(tokens: &'a Vec<Token>, index: usize) -> Result<&'a Token, Box<dyn Error>> {
    if index < tokens.len() {
        return Ok(&tokens[index])
    } else {
        return Err(Box::from("expected a token, but got nothing"))
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

fn next_error<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, Box<dyn Error>> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Ok(&tokens[ret])
    } else {
        return Err(Box::from("expected a token, but got nothing"))
    }
}

fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, Box<dyn Error>> {
    let mut generated_code = String::from("");
    loop {
        let code = parse_function(tokens, index)?;
        if code.eq("") {
            break;
        }

        generated_code += &code;
    }

    return Ok(generated_code);
}

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<String, Box<dyn Error>> {
    
    match next(tokens, index) {
    None => {
        return epsilon();
    }
    Some(token) => {
        if !matches!(token, Token::Func) {
            return Err(Box::from("functions must begin with func"));
        }
    }

    }

    let func_ident = match next_error(tokens, index)? {
    Token::Ident(func_ident) => func_ident,
    _  => {return Err(Box::from("functions must have a function identifier"));}
    };

    if !matches!( next_error(tokens, index)?, Token::LeftParen) {
        return Err(Box::from("expected '('"));
    }

    loop {
       match next_error(tokens, index)? {

       Token::RightParen => {
           break;
       }

       Token::Int => {
           match next_error(tokens, index)? {
           Token::Ident(param) => {
               println!("parameter {}", param);
               match peek_error(tokens, *index)? {
               Token::Comma => {
                   *index += 1;
               }
               Token::RightParen => {}
               _ => {
                   return Err(Box::from("expected ',' or ')'"));
               }

               }
           }
           _ => {
                return Err(Box::from("expected ident function parameter"));
           }

           }
       }

       _ => {
           return Err(Box::from("expected 'int' keyword or ')' token"));
       }

       }
    }

    if !matches!(next_error(tokens, index)?, Token::LeftCurly) {
        return Err(Box::from("expected '{'"));
    }

    loop {
        let code = parse_statement(tokens, index)?;
        println!("{}", code);
        if code.eq("") {
            break;
        }

    }

    if !matches!(next_error(tokens, index)?, Token::RightCurly) {
      return Err(Box::from("expected '}'"));
    }

    return Ok(format!("function {}\n", func_ident));
}

fn epsilon() -> Result<String, Box<dyn Error>> {
    return Ok(String::from(""));
}

fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, Box<dyn Error>> {
    match peek(tokens, *index) {
    None => {
        return epsilon();
    }

    Some(token) => {
        match token {

        Token::RightCurly => {
            return epsilon();
        }

        Token::Int => {
            *index += 1;
            match next_error(tokens, index)? {
            Token::Ident(ident) => {
                println!("declaration {}", ident);
                if matches!(peek_error(tokens, *index)?, Token::Assign) {
                    *index += 1;
                    _ = parse_expression(tokens, index);
                }
            }

            _ => {
                return Err(Box::from("expected identifier"));
            }

            }
        }

        Token::Ident(_) => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::Assign) {
                return Err(Box::from("expected '=' assignment operator"));
            }
            _ = parse_expression(tokens, index)?;
        }

        Token::Return => {
            *index += 1;
            _ = parse_expression(tokens, index)?;
        }

        Token::Print => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::LeftParen) {
                return Err(Box::from("expect '(' closing statement"));
            }

            _ = parse_expression(tokens, index)?;

            if !matches!(next_error(tokens, index)?, Token::RightParen) {
                return Err(Box::from("expect ')' closing statement"));
            }
        }

        Token::Read => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::LeftParen) {
                return Err(Box::from("expect '(' closing statement"));
            }

            _ = parse_expression(tokens, index)?;

            if !matches!(next_error(tokens, index)?, Token::RightParen) {
                return Err(Box::from("expect ')' closing statement"));
            }
        }

        _ => {
             return Err(Box::from("invalid statement."));
        }

        }

        if !matches!(next_error(tokens, index)?, Token::Semicolon) {
            return Err(Box::from("expect ';' closing statement"));
        }

        return Ok(String::from("statement"));
    }

    }
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<String, Box<dyn Error>> {
    let mut code = parse_multiply_expression(tokens, index)?;
    loop {
       match peek_error(tokens, *index)? {
       Token::Plus => {
           *index += 1;
           _ = parse_multiply_expression(tokens, index)?;
           code += "c = (add c b)\n";
       }

       Token::Subtract => {
           *index += 1;
           _ = parse_multiply_expression(tokens, index)?;
           code += "c = (sub c b)\n";
       }

       _ => {
           break;
       }

       }
    }

    return Ok(code);
}


fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<String, Box<dyn Error>> {
    let mut code = parse_term(tokens, index)?;
    loop {
       match peek_error(tokens, *index)? {
       Token::Multiply => {
           *index += 1;
           _ = parse_term(tokens, index)?;
           code += "c = (mult c b)\n";
       }

       Token::Divide => {
           *index += 1;
           _ = parse_term(tokens, index)?;
           code += "c = (divide c b)\n";
       }

       _ => {
           break;
       }

       }
    }

    return Ok(code);
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<String, Box<dyn Error>> {
    match next_error(tokens, index) ? {

    Token::Ident(ident) => {
        match peek_error(tokens, *index)? {
        Token::LeftParen => {
            *index += 1;
            loop {
               match peek_error(tokens, *index)? {
  
               Token::RightParen => {
                   *index += 1;
                   break;
               }

               _ => {
                   let _code = parse_expression(tokens, index)?;
                   match peek_error(tokens, *index)? {
                   Token::Comma => {
                       *index += 1;
                   }
                   Token::RightParen => {}
                   _ => {
                       return Err(Box::from("expected ',' or ')'"));
                   }

                   }
               }
  
               }
            }

            return Ok(String::from("a"));
        }

        _ => {
            return Ok(ident.clone());
        }

        }
    }

    Token::Num(num) => {
        return Ok(format!("{}", num));
    }

    Token::LeftParen => {
        let _code = parse_expression(tokens, index)?;
        if !matches!(next_error(tokens, index)?, Token::RightParen) {
            return Err(Box::from("expected ')' parenthesis"));
        }

        return Ok(String::from("term"));
    }

    _ => {
        return Err(Box::from("invalid expression"));
    }

    }
}


#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;

    #[test]
    fn lexer_test() {
        // valid numbers
        let mut tokens = lex("1 + 2 + 3 * 44");
        match tokens {
        Err(_)=> {assert!(false);}
        Ok((tok,_)) => {
            assert!(tok.len() == 7);
            assert!(matches!(tok[0], Token::Num(1)));
            assert!(matches!(tok[1], Token::Plus));
            assert!(matches!(tok[2], Token::Num(2)));
            assert!(matches!(tok[3], Token::Plus));
            assert!(matches!(tok[4], Token::Num(3)));
            assert!(matches!(tok[5], Token::Multiply));
            assert!(matches!(tok[6], Token::Num(44)));
        }

        }

        // valid numbers
        tokens = lex("1 / 2 - 3");
        match tokens {
        Err(_)=> {assert!(false);}
        Ok((tok,_)) => {
            assert!(tok.len() == 5);
            assert!(matches!(tok[0], Token::Num(1)));
            assert!(matches!(tok[1], Token::Divide));
            assert!(matches!(tok[2], Token::Num(2)));
            assert!(matches!(tok[3], Token::Subtract));
            assert!(matches!(tok[4], Token::Num(3)));
        }

        }

        // valid identifiers 
        tokens = lex("box333 c3a3r dog cat");
        match tokens {
        Err(_)=> {assert!(false);}
        Ok((tok,_)) => {
            assert!(tok.len() == 4);
            assert!(is_name(&tok[0], "box333"));
            assert!(is_name(&tok[1], "c3a3r"));
            assert!(is_name(&tok[2], "dog"));
            assert!(is_name(&tok[3], "cat"));
            fn is_name(t: &Token, name: &str) -> bool {
                match t {
                Token::Ident(id) => {return id.eq(name);}
                _ => {return false;}
                }
            }
        }
        }

        // '31st' is an invalid number
        tokens = lex("October 31st, 1517");
        match tokens {

        Err(_) => {}

        Ok(_) => {assert!(false);}

        }

    }
}



