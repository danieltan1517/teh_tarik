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

    Ok(()) => {
        println!("Program Parsed Successfully.");
    }

    Err(message) => {
        println!("**Error**");
        println!("----------------------");
        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else if index >= tokens.len() {
            index = tokens.len() - 1;
            let loc = &location[index];
            println!("Error at line {}:{}. {}", loc.line, loc.col, message);
            println!("----------------------");
        } else {
            index -= 1;
            let loc = &location[index];
            println!("Error at line {}:{}. {}", loc.line, loc.col, message);
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

struct Loc {
  line: i32,
  col:  i32,
}

fn lex(code: &str) -> Result<(Vec<Token>, Vec<Loc>), String> {
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
            } else if character == '#' {
                StateMachine::Comment
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

        StateMachine::Comment => {
            if character == '\n' {
                StateMachine::Init
            } else {
                StateMachine::Comment
            }
        }

        };

        token_end += 1;

        // actions of state machine.
        match state_machine {

        StateMachine::Init => {
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
                     return Err(format!("Error at line {}:{}. Unidentified symbol '{}'", line_num, col_num, ident));
                 }
             }

             }
        }

        StateMachine::Number => {}
        StateMachine::Comment => {}
        StateMachine::Ident => {}

        StateMachine::ErrorNum => {
            if character == ' ' {
                let ident = &code[token_start..token_end];
                return Err(format!("Error at line {}:{}. Invalid Number '{}'", line_num, col_num, ident));
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
        _ => Token::Ident(String::from(token)),
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
        Comment,
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
    parse_term(tokens, index)?;
    match peek_result(tokens, *index)? {
    Token::Plus => {},
    Token::Subtract => {},
    Token::Multiply => {},
    Token::Divide => {},
    Token::Modulus => {},

    _ => { 
        return Ok(()); 
    }

    };

    *index += 1;
    parse_term(tokens, index)?;

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
        let (tokens,_) = lex("a = 1 + 2;").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        let (tokens,_) = lex("b = 1 / 2;").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();


        // test errors. missing semicolon
        let (tokens,_) = lex("b = 1 / 2").unwrap();
        assert!(matches!(parse_statement(&tokens, &mut 0), Err(_)));

    }

}

