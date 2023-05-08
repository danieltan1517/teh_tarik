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

    let tokens = match lex(&code) {
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
            let tok = &tokens[index];
            print_error(tok, &message);
        } else {
            let tok = &tokens[index];
            print_error(tok, &message);
        }

    }

    }

    fn print_error<T: std::fmt::Display>(tok: &Token, message: T) {
        match tok {
        Token::Func(line, col) => {
            println!("Error at line {}:{}. {}", line, col, message);
        }
        Token::Return(line, col) => {
            println!("Error at line {}:{}. unexpected 'return' keyword. {}", line, col, message);
        }
        Token::Int(line, col) => {
            println!("Error at line {}:{}. unexpected 'int' keyword. {}", line, col, message);
        }
        Token::Print(line, col) => {
            println!("Error at line {}:{}. unexpected 'print' keyword. {}", line, col, message);
        }
        Token::Read(line, col) => {
            println!("Error at line {}:{}. unexpected 'read' keyword. {}", line, col, message);
        }

        Token::LeftParen(line, col) => {
            println!("Error at line {}:{}. unexpected '('. {}", line, col, message);
        }
        Token::RightParen(line, col) => {
            println!("Error at line {}:{}. unexpected ')'. {}", line, col, message);
        }
        Token::LeftCurly(line, col) => {
            println!("Error at line {}:{}. unexpected '{{'. {}", line, col, message);
        }
        Token::RightCurly(line, col) => {
            println!("Error at line {}:{}. unexpected '}}'. {}", line, col, message);
        }
        Token::Comma(line, col) => {
            println!("Error at line {}:{}. unexpected ','. {}", line, col, message);
        }
        Token::Semicolon(line, col) => {
            println!("Error at line {}:{}. unexpected ';'. {}", line, col, message);
        }

        Token::Plus(line, col) => {
            println!("Error at line {}:{}. unexpected '+'. {}", line, col, message);
        }
        Token::Subtract(line, col) => {
            println!("Error at line {}:{}. unexpected '-'. {}", line, col, message);
        }
        Token::Multiply(line, col) => {
            println!("Error at line {}:{}. unexpected '*'. {}", line, col, message);
        }
        Token::Divide(line, col) => {
            println!("Error at line {}:{}. unexpected '/'. {}", line, col, message);
        }
        Token::Modulus(line, col) => {
            println!("Error at line {}:{}. unexpected '%'. {}", line, col, message);
        }
        Token::Assign(line, col) => {
            println!("Error at line {}:{}. unexpected '='. {}", line, col, message);
        }

        Token::Ident(line, col, ident) => {
            println!("Error at line {}:{}. invalid identifier {}. {}", line, col, ident, message);
        }
        Token::Num(line, col, num) => {
            println!("Error at line {}:{}. invalid identifier {}. {}", line, col, num, message);
        }

        }
        println!("----------------------");
    }
}

#[derive(Debug)]
enum Token {
  // keywords:
  Func(i32, i32),
  Return(i32, i32),
  Int(i32, i32),
  Print(i32, i32),
  Read(i32, i32),

  LeftParen(i32, i32),
  RightParen(i32, i32),
  LeftCurly(i32, i32),
  RightCurly(i32, i32),
  Comma(i32, i32),
  Semicolon(i32, i32),

  // mathematical operators.
  Plus(i32, i32),
  Subtract(i32, i32),
  Multiply(i32, i32),
  Divide(i32, i32),
  Modulus(i32, i32),
  Assign(i32, i32), // =

  // comparison operators
  /*Less,
  LessEqual,
  Equal,
  Greater,
  GreaterEqual,*/

  Ident(i32, i32, String),
  Num(i32, i32, i32),
}

fn lex(code: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = vec![];
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
                tokens.push(Token::Num(line_num, (token_start + 1) as i32, number));
                StateMachine::Init
            }
        }

        StateMachine::Ident => {
            if character.is_alphabetic() || (character >= '0' && character <= '9') || character == '_' {
                StateMachine::Ident
            } else {
                let ident = create_identifier(line_num, (token_start + 1) as i32, token_start, token_end, code);
                tokens.push(ident);
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
             '+' => tokens.push(Token::Plus(line_num,col_num)),
             '-' => tokens.push(Token::Subtract(line_num,col_num)),
             '*' => tokens.push(Token::Multiply(line_num,col_num)),
             '/' => tokens.push(Token::Divide(line_num,col_num)),
             '%' => tokens.push(Token::Modulus(line_num,col_num)),
             ',' => tokens.push(Token::Comma(line_num,col_num)),
             '{' => tokens.push(Token::LeftCurly(line_num,col_num)),
             '}' => tokens.push(Token::RightCurly(line_num,col_num)),
             '(' => tokens.push(Token::LeftParen(line_num,col_num)),
             ')' => tokens.push(Token::RightParen(line_num,col_num)),
             ';' => tokens.push(Token::Semicolon(line_num,col_num)),
             '=' => tokens.push(Token::Assign(line_num,col_num)),
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
        tokens.push(Token::Num(line_num, (token_start + 1) as i32, number));
    }
    StateMachine::Ident => {
        let ident = create_identifier(line_num, (token_start + 1) as i32, token_start, token_end, code);
        tokens.push(ident);
    }
    _ => {}
    }

    return Ok(tokens);

    // helper functions
    fn create_identifier(line: i32, col: i32, token_start: usize, token_end: usize, code: &str) -> Token {
        let token = &code[token_start..token_end];
        match token {
        "func" => Token::Func(line, col),
        "return" => Token::Return(line, col),
        "int" => Token::Int(line, col),
        "print" => Token::Print(line, col),
        "read" => Token::Read(line, col),
        _ => Token::Ident(line, col, String::from(token))
        }
    }

    fn create_number(token_start: usize, token_end: usize, code: &str) -> i32 {
        let token = &code[token_start..token_end];
        match token.parse::<i32>() {
        // this code should correctly parse because the lexer verified that this is correct.
        // quit.
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
        if !matches!(token, Token::Func(_,_)) {
            return Err(Box::from("functions must begin with func"));
        }
    }

    }

    let func_ident = match next_error(tokens, index)? {
    Token::Ident(_,_,func_ident) => func_ident,
    _  => {return Err(Box::from("functions must have a function identifier"));}
    };

    if !matches!( next_error(tokens, index)?, Token::LeftParen(_,_) ) {
        return Err(Box::from("expected '('"));
    }

    loop {
       match next_error(tokens, index)? {

       Token::RightParen(_,_) => {
           break;
       }

       Token::Int(_,_) => {
           match next_error(tokens, index)? {
           Token::Ident(_,_,param) => {
               println!("parameter {}", param);
               match peek_error(tokens, *index)? {
               Token::Comma(_,_) => {
                   *index += 1;
               }
               Token::RightParen(_,_) => {}
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

    if !matches!(next_error(tokens, index)?, Token::LeftCurly(_,_)) {
        return Err(Box::from("expected '{'"));
    }

    loop {
        let code = parse_statement(tokens, index)?;
        println!("{}", code);
        if code.eq("") {
            break;
        }

    }

    if !matches!(next_error(tokens, index)?, Token::RightCurly(_,_)) {
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

        Token::RightCurly(_,_) => {
            return epsilon();
        }

        Token::Int(_,_) => {
            *index += 1;
            match next_error(tokens, index)? {
            Token::Ident(_,_,ident) => {
                println!("declaration {}", ident);
                if matches!(peek_error(tokens, *index)?, Token::Assign(_,_)) {
                    *index += 1;
                    _ = parse_expression(tokens, index);
                }
            }

            _ => {
                return Err(Box::from("expected identifier"));
            }

            }
        }

        Token::Ident(_,_,_) => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::Assign(_,_)) {
                return Err(Box::from("expected '=' assignment operator"));
            }
            _ = parse_expression(tokens, index)?;
        }

        Token::Return(_,_) => {
            *index += 1;
            _ = parse_expression(tokens, index)?;
        }

        Token::Print(_,_) => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::LeftParen(_,_)) {
                return Err(Box::from("expect '(' closing statement"));
            }

            _ = parse_expression(tokens, index)?;

            if !matches!(next_error(tokens, index)?, Token::RightParen(_,_)) {
                return Err(Box::from("expect ')' closing statement"));
            }
        }

        Token::Read(_,_) => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::LeftParen(_,_)) {
                return Err(Box::from("expect '(' closing statement"));
            }

            _ = parse_expression(tokens, index)?;

            if !matches!(next_error(tokens, index)?, Token::RightParen(_,_)) {
                return Err(Box::from("expect ')' closing statement"));
            }
        }

        _ => {
             return Err(Box::from("invalid statement."));
        }

        }

        if !matches!(next_error(tokens, index)?, Token::Semicolon(_,_)) {
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
       Token::Plus(_,_) => {
           *index += 1;
           _ = parse_multiply_expression(tokens, index)?;
           code += "c = (add c b)\n";
       }

       Token::Subtract(_,_) => {
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
       Token::Multiply(_,_) => {
           *index += 1;
           _ = parse_term(tokens, index)?;
           code += "c = (mult c b)\n";
       }

       Token::Divide(_,_) => {
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

    Token::Ident(_,_,ident) => {
        match peek_error(tokens, *index)? {
        Token::LeftParen(_,_) => {
            *index += 1;
            loop {
               match peek_error(tokens, *index)? {
  
               Token::RightParen(_,_) => {
                   *index += 1;
                   break;
               }

               _ => {
                   let _code = parse_expression(tokens, index)?;
                   match peek_error(tokens, *index)? {
                   Token::Comma(_,_) => {
                       *index += 1;
                   }
                   Token::RightParen(_,_) => {}
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

    Token::Num(_,_,num) => {
        return Ok(format!("{}", num));
    }

    Token::LeftParen(_,_) => {
        let _code = parse_expression(tokens, index)?;
        if !matches!(next_error(tokens, index)?, Token::RightParen(_,_)) {
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
        Ok(tok) => {
            assert!(tok.len() == 7);
            assert!(matches!(tok[0], Token::Num(_,_,1)));
            assert!(matches!(tok[1], Token::Plus(_,_)));
            assert!(matches!(tok[2], Token::Num(_,_,2)));
            assert!(matches!(tok[3], Token::Plus(_,_)));
            assert!(matches!(tok[4], Token::Num(_,_,3)));
            assert!(matches!(tok[5], Token::Multiply(_,_)));
            assert!(matches!(tok[6], Token::Num(_,_,44)));
        }

        }

        // valid numbers
        tokens = lex("1 / 2 - 3");
        match tokens {
        Err(_)=> {assert!(false);}
        Ok(tok) => {
            assert!(tok.len() == 5);
            assert!(matches!(tok[0], Token::Num(_,_,1)));
            assert!(matches!(tok[1], Token::Divide(_,_)));
            assert!(matches!(tok[2], Token::Num(_,_,2)));
            assert!(matches!(tok[3], Token::Subtract(_,_)));
            assert!(matches!(tok[4], Token::Num(_,_,3)));
        }

        }

        // valid identifiers 
        tokens = lex("box333 c3a3r dog cat");
        match tokens {
        Err(_)=> {assert!(false);}
        Ok(tok) => {
            assert!(tok.len() == 4);
            assert!(is_name(&tok[0], "box333"));
            assert!(is_name(&tok[1], "c3a3r"));
            assert!(is_name(&tok[2], "dog"));
            assert!(is_name(&tok[3], "cat"));
            fn is_name(t: &Token, name: &str) -> bool {
                match t {
                Token::Ident(_,_,id) => {return id.eq(name);}
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



