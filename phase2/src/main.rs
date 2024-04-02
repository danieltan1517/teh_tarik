use std::env;
use std::fs;

fn main() {
    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lexer.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file contents, storing them inside 'code' as a string.
    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
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
    match parse_math(&tokens, &mut index) {
    Ok(num) => {
        println!("Expression = {code}");
        println!("Answer = {num}");
    }

    Err(e) => {
        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else {
            println!("Error {e}");
        }
    }

    }


}

#[derive(Debug, Clone)]
enum Token {
  NotToken,
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  LeftParen,
  RightParen,
  Num(i32),
  Ident(String),
  If,
  While,
  Read,
  Write,
  Return,
  Func,
  Assign,
  Int,
  Semicolon,
}

fn create_identifier(code: &str) -> Token {
  match code {
  "func" => Token::Func,
  "return" => Token::Return,
  "int" => Token::Int,
  "print" => Token::Write,
  "read" => Token::Read,
  "while" => Token::While,
  "if" => Token::If,
  _ => Token::Ident(String::from(code)),
  }
}

// the <'a> is the "lifetimes" type annotations in Rust.
//
// this 'dead_code' macro is just to supress Rust's dead_code warning. This macro can be removed.
#[allow(dead_code)]
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

// this 'dead_code' macro is just to supress Rust's dead_code warning. This macro can be removed.
#[allow(dead_code)]
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

fn parse_math(tokens: &Vec<Token>, index: &mut usize) -> Result<i32, String> {
  let answer = parse_expression(tokens, index)?;
  if matches!(next_result(tokens, index)?, Token::Semicolon) {
    return Ok(answer);
  } else {
    return Err(String::from("missing semicolon ';'"));
  }
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<i32, String> {
    let mut answer = parse_multiply_expression(tokens, index)?;
    loop {
       match peek_result(tokens, *index)? {

       Token::Plus => {
           *index += 1;
           answer += parse_multiply_expression(tokens, index)?;
       }

       Token::Subtract => {
           *index += 1;
           answer -= parse_multiply_expression(tokens, index)?;
       }

       _ => { 
           break;
       }

       };
    }

    return Ok(answer);
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<i32, String> {
    let mut answer = parse_term(tokens, index)?;
    loop {
       match peek_result(tokens, *index)? {
       Token::Multiply => {
          *index += 1;
          answer *= parse_term(tokens, index)?;
       }

       Token::Divide => {
          *index += 1;
          answer /= parse_term(tokens, index)?;
       }

       Token::Modulus => {
          *index += 1;
          answer %= parse_term(tokens, index)?;
       }
  
       _ => {
           break;
       }

       };

    }

    return Ok(answer);
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<i32, String> {
    match next_result(tokens, index)? {

    Token::Num(num) => {
        return Ok(*num);
    }

    Token::LeftParen => {
        println!("Expression term\n");
        let answer = parse_expression(tokens, index)?;
        if !matches!(next_result(tokens, index)?, Token::RightParen) {
            return Err(String::from("expected ')' parenthesis"));
        }
        return Ok(answer);
    }

    _ => {
        println!("{:?}", tokens[*index]);
        return Err(String::from("invalid expression"));
    }

    }
}

// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::lex;
    use crate::parse_math;

    #[test]
    fn parser_test() {
        // test that parser works on correct cases
        assert!(parse_math_string("1;") == 1);
        assert!(parse_math_string("1 + 2;") == 3);
        assert!(parse_math_string("(7 * 6);") == 42);
        assert!(parse_math_string("(7 * 6) + 42;") == 84);
        assert!(parse_math_string("42 + (7 * 3) * 2;") == 84);
        assert!(parse_math_string("2 + (7 * 3) + 2;") == 25);
        assert!(parse_math_string("2 + (7 * 3) + 2;") == 25);
        assert!(parse_math_string("2 + (7 * (3 + 1)) + 2;") == 32);
        assert!(parse_math_string("(2);") == 2);

        // test parser errors
        assert!(matches!(parse_error("(2;"), Err(_)));
        assert!(matches!(parse_error("(2"), Err(_)));
        assert!(matches!(parse_error("2);"), Err(_)));
        assert!(matches!(parse_error("2))"), Err(_)));
        assert!(matches!(parse_error("2 2;"), Err(_)));
        assert!(matches!(parse_error("5 200;"), Err(_)));
        assert!(matches!(parse_error("5 +- 200;"), Err(_)));
    }

    fn parse_error(expression: &str) -> Result<i32, String> {
        let toks = lex(expression).unwrap();
        parse_math(&toks, &mut 0)
    }

    fn parse_math_string(expression: &str) -> i32 {
        let toks = lex(expression).unwrap();
        parse_math(&toks, &mut 0).unwrap()
    }
}



