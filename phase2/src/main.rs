use std::env;
use std::fs;
use std::error::Error;

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

    let (tokens, locations) = match lex(&code) {
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
    match parse_expression(&tokens, &mut index) {
    Ok(num) => {
        println!("Expression = {code}");
        println!("Answer = {num}");
    }

    Err(e) => {

        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else if index >= tokens.len() {
            index = tokens.len() - 1;
            let loc = &locations[index];
            println!("Error at line {}:{}.", loc.line_num, loc.col_num);
            println!("{e}");
        } else {
            index -= 1;
            let loc = &locations[index];
            println!("Error at line {}:{}.", loc.line_num, loc.col_num);
            println!("{e}");
        }
    }

    }


}

#[derive(Debug, Clone)]
enum Token {
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  LeftParen,
  RightParen,
  Num(i32),
  EndOfFile,
}

// line of code
struct Loc {
    line_num: i32,
    col_num: i32,
}

fn lex(code: &str) -> Result<(Vec<Token>, Vec<Loc>), Box<dyn Error>> {
    let mut tokens: Vec<Token> = vec![];
    let mut loc = vec![];
    let mut token_start: usize = 0;
    let mut token_end:   usize = 0;
    let mut line_num:    i32   = 1;
    let mut col_num:     i32   = 1;
    let mut state_machine = StateMachine::Init;

    for character in code.chars() {

        // state machine transitions.
        state_machine = match state_machine {

        StateMachine::Init => {
            token_start = token_end;
            if character >= '0' && character <= '9' {
                StateMachine::Number
            } else {
                StateMachine::Init
            }
        }

        StateMachine::Number => {
            if character >= '0' && character <= '9' {
                StateMachine::Number
            } else {
                let number = create_number(token_start, token_end, code);
                tokens.push(Token::Num(number));
                loc.push(Loc{line_num:line_num, col_num:col_num});
                StateMachine::Init
            }
        }

        };

        token_end += 1;

        // actions of state machine.
        match state_machine {

        StateMachine::Init => {
             match character {
             '+' => {
                 tokens.push(Token::Plus);
                 loc.push(Loc{line_num:line_num, col_num:col_num});
             }
             '-' => {
                 tokens.push(Token::Subtract);
                 loc.push(Loc{line_num:line_num, col_num:col_num});
             }
             '*' => {
                 tokens.push(Token::Multiply);
                 loc.push(Loc{line_num:line_num, col_num:col_num});
             }
             '/' => {
                 tokens.push(Token::Divide);
                 loc.push(Loc{line_num:line_num, col_num:col_num});
             }
             '%' => {
                 tokens.push(Token::Modulus);
                 loc.push(Loc{line_num:line_num, col_num:col_num});
             }
             '(' => {
                 tokens.push(Token::LeftParen);
                 loc.push(Loc{line_num:line_num, col_num:col_num});
             }
             ')' => {
                 tokens.push(Token::RightParen);
                 loc.push(Loc{line_num:line_num, col_num:col_num});
             }
              _  => {
                 if !character.is_whitespace() {
                     let ident = &code[token_start..token_end];
                     let message = format!("Error at line {}:{}. Unidentified symbol '{}'", line_num, col_num, ident);
                     return Err(Box::from(message));
                 }
             }

             }
        }

        StateMachine::Number => {}

        };

        if character == '\n' {
            col_num = 1;
            line_num += 1;
        } else {
            col_num += 1;
        }
    }

    if matches!(state_machine, StateMachine::Number) {
        let number = create_number(token_start, token_end, code);
        tokens.push(Token::Num(number));
        loc.push(Loc{line_num:line_num, col_num:col_num});
    }

    tokens.push(Token::EndOfFile);
    loc.push(Loc{line_num:line_num, col_num:col_num});
    return Ok((tokens, loc));

    fn create_number(start: usize, end: usize, code: &str) -> i32 {
        // this code should correctly parse because the lexer verified that this is correct.
        let token = &code[start..end];
        token.parse::<i32>().unwrap()
    }

    enum StateMachine {
        Init,
        Number,
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

fn peek_result<'a>(tokens: &'a Vec<Token>, index: usize) -> Result<&'a Token, Box<dyn Error>> {
    if index < tokens.len() {
        return Ok(&tokens[index])
    } else {
        return Err(Box::from("expected a token, but got nothing"))
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

fn next_result<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, Box<dyn Error>> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Ok(&tokens[ret])
    } else {
        return Err(Box::from("expected a token, but got nothing"))
    }
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<i32, Box<dyn Error>> {
    let mut _ans = parse_multiply_expression(tokens, index)?;
    loop {
       match peek_result(tokens, *index)? {

       Token::Plus => {
           *index += 1;
           let _answer = parse_multiply_expression(tokens, index)?;
       }

       Token::Subtract => {
           *index += 1;
           let _answer = parse_multiply_expression(tokens, index)?;
       }

       Token::RightParen | Token::EndOfFile => {
           break;
       }

       _ => { 
           return Err(Box::from("invalid expression."));
       }

       };
    }

    return Ok(_ans);
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<i32, Box<dyn Error>> {
    let mut _ans = parse_term(tokens, index)?;
    loop {
       match peek_result(tokens, *index)? {
       Token::Multiply => {
          *index += 1;
          let _answer = parse_term(tokens, index)?;
       }

       Token::Divide => {
          *index += 1;
          let _answer = parse_term(tokens, index)?;
       }

       Token::Modulus => {
          *index += 1;
          let _answer = parse_term(tokens, index)?;
       }
  
       Token::EndOfFile => {
           break;
       }

       _ => {
           break;
       }

       };

    }

    return Ok(0);
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<i32, Box<dyn Error>> {
    match next_result(tokens, index)? {

    Token::Num(num) => {
        return Ok(*num);
    }

    Token::LeftParen => {
        let answer = parse_expression(tokens, index)?;
        if !matches!(next_result(tokens, index)?, Token::RightParen) {
            return Err(Box::from("expected ')' parenthesis"));
        }
        return Ok(answer);
    }

    Token::EndOfFile => {
        return Ok(0);
    }

    _ => {
        return Err(Box::from("invalid expression"));
    }

    }
}

// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::lex;
    use crate::parse_expression;

    #[test]
    fn parser_test() {
        // test that parser works on correct cases
        assert!(parse_expression_string("1") == 1);
        assert!(parse_expression_string("1 + 2") == 3);
        assert!(parse_expression_string("(7 * 6)") == 42);
        assert!(parse_expression_string("(7 * 6) + 42") == 84);
        assert!(parse_expression_string("42 + (7 * 3) * 2") == 84);
    }

    fn parse_expression_string(expression: &str) -> i32 {
        let (toks, _) = lex(expression).unwrap();
        parse_expression(&toks, &mut 0).unwrap()
    }
}



