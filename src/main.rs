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
  While,
  If,
  Else,
  Break,
  Continue,

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
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  Equality,
  NotEqual,

  Ident(String),
  Num(i32),
}

enum CodeNode {
  Code(String),
  Epsilon,
}

struct Expression {
  code: String,
  name: String,
}

static mut VAR_NUM: i64 = 0;
static mut LOOP_NUM: i64 = 0;
static mut LOOP_STACK: Vec<i64> = vec![];
static mut IFS_ID: i64 = 0;

fn create_ifs_id() -> i64 {
    unsafe {
        IFS_ID += 1;
        IFS_ID
    }
}

fn peek_label_stack() -> i64 {
    unsafe {
       match LOOP_STACK.last() {
       None => panic!("Invalid. Attempt to pop empty stack."),
       Some(x) => *x
       }
    }
}

fn push_label_id(id: i64) {
    unsafe {
        LOOP_STACK.push(id);
    }
}

fn pop_label_id() {
    unsafe {
        LOOP_STACK.pop();
    }
}

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}

fn create_label_id() -> i64 {
    unsafe {
        LOOP_NUM += 1;
        LOOP_NUM
    }
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
            } else if character == '<' || character == '>' || character == '=' || character == '!' {
                StateMachine::Sign
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

        StateMachine::Sign => {
            if character == '<' || character == '>' || character == '=' || character == '!' {
                StateMachine::Sign
            } else if character.is_alphabetic() {
                let symbol = create_sign(token_start, token_end, code)?;
                add(&mut tokens, &mut locations, symbol, line_num, col_num);
                token_start = token_end;
                StateMachine::Ident
            } else if character >= '0' && character <= '9' {
                let symbol = create_sign(token_start, token_end, code)?;
                add(&mut tokens, &mut locations, symbol, line_num, col_num);
                token_start = token_end;
                StateMachine::Number
            } else if character == '#' {
                let symbol = create_sign(token_start, token_end, code)?;
                add(&mut tokens, &mut locations, symbol, line_num, col_num);
                StateMachine::Comment
            } else {
                let symbol = create_sign(token_start, token_end, code)?;
                add(&mut tokens, &mut locations, symbol, line_num, col_num);
                StateMachine::Init
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
             //'=' => add(&mut tokens, &mut locations, Token::Assign, line_num, col_num),
             //'<' => add(&mut tokens, &mut locations, Token::Less, line_num, col_num),
             //'>' => add(&mut tokens, &mut locations, Token::Greater, line_num, col_num),
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
        StateMachine::Comment => {}
        StateMachine::Ident => {}

        StateMachine::ErrorNum => {
            if character == ' ' {
                let ident = &code[token_start..token_end];
                let message = format!("Error at line {}:{}. Invalid Number '{}'", line_num, col_num, ident);
                return Err(Box::from(message));
            }
        }

        StateMachine::Sign => {}

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
        "while" => Token::While,
        "if" => Token::If,
        "else" => Token::Else,
        "break" => Token::Break,
        "continue" => Token::Continue,
        _ => Token::Ident(String::from(token)),
        }
    }

    fn create_sign(start: usize, end: usize, code: &str) -> Result<Token, Box<dyn Error>> {
    let token = &code[start..end];
    match token {
    "<" => Ok(Token::Less),
    "<=" => Ok(Token::LessEqual),
    ">" => Ok(Token::Greater),
    ">=" => Ok(Token::GreaterEqual),
    "==" => Ok(Token::Equality),
    "=" => Ok(Token::Assign),
    "!=" => Ok(Token::NotEqual),
    _ => return Err(Box::from(format!("invalid symbol {}", token))),
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
        Sign,
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
        match parse_function(tokens, index)? {
        CodeNode::Epsilon => {
            break;
        }
        CodeNode::Code(func_code) => {
            generated_code += &func_code;
        }
        }
    }

    return Ok(generated_code);
}

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<CodeNode, Box<dyn Error>> {
    
    match next(tokens, index) {
    None => {
        return Ok(CodeNode::Epsilon);
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

    let mut code = format!("func {}\n", func_ident);
    let mut params: Vec<String> = vec![];
    loop {
       match next_error(tokens, index)? {

       Token::RightParen => {
           break;
       }

       Token::Int => {
           match next_error(tokens, index)? {
           Token::Ident(param) => {
               params.push(param.clone());
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

    for (index, param) in params.iter().enumerate() {
        let decl = format!(". {}\n", param);
        let assign = format!("= {}, ${}\n", param, index);
        code += &decl;
        code += &assign;
    }

    loop {
        match parse_statement(tokens, index)? {
        CodeNode::Epsilon => {
            break;
        }
        CodeNode::Code(statement) => {
            code += &statement;
        }
        }
    }

    code += "endfunc\n\n";

    if !matches!(next_error(tokens, index)?, Token::RightCurly) {
      return Err(Box::from("expected '}'"));
    }

    return Ok(CodeNode::Code(code));
}

fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<CodeNode, Box<dyn Error>> {
    match peek(tokens, *index) {
    None => {
        return Ok(CodeNode::Epsilon);
    }

    Some(token) => {
        let codenode: CodeNode;
        match token {

        Token::RightCurly => {
            return Ok(CodeNode::Epsilon);
        }

        Token::While => {
            *index += 1;
            let expression = parse_boolean(tokens, index)?;

            if !matches!(next_error(tokens, index)?, Token::LeftCurly) {
                return Err(Box::from("expected '{'"));
            }

            let id = create_label_id();
            push_label_id(id);
            let mut code = format!(": loopbegin{id}\n");
            code = format!("{}{}?:= loopbody{id}, {}\n", code, expression.code, expression.name);
            code = format!("{}:= endloop{id}\n", code);
            code = format!("{}: loopbody{id}\n", code);
            loop {
                let statement = match parse_statement(tokens, index)? {
                CodeNode::Epsilon => {
                    break;
                }

                CodeNode::Code(statement) => statement,
                };

                code += &statement;

            }

            if !matches!(next_error(tokens, index)?, Token::RightCurly) {
                return Err(Box::from("expected '}'"));
            }

            pop_label_id();
            code = format!("{}:= loopbegin{id}\n", code);
            code = format!("{}: endloop{id}\n", code);

            codenode = CodeNode::Code(code);
            return Ok(codenode);
        }

        Token::If => {
            *index += 1;
            let id = create_ifs_id();
            let expression = parse_boolean(tokens, index)?;
            if !matches!(next_error(tokens, index)?, Token::LeftCurly) {
                return Err(Box::from("expected '{'"));
            }
            // let mut code = format!("{}?:= if_true, {}\n", expression.code, expression.name);
            let mut ifbody = String::from("");
            loop {
                let statement = match parse_statement(tokens, index)? {
                CodeNode::Epsilon => {
                    break;
                }

                CodeNode::Code(statement) => statement,

                };

                ifbody += &statement;
            }

            if !matches!(next_error(tokens, index)?, Token::RightCurly) {
                return Err(Box::from("expected '}'"));
            }

            if matches!(peek_error(tokens, *index)?, Token::Else) {
                *index += 1;

                if !matches!(next_error(tokens, index)?, Token::LeftCurly) {
                    return Err(Box::from("expected '{'"));
                }
                let mut elsebody = String::from("");
                loop {
                    let statement = match parse_statement(tokens, index)? {
                    CodeNode::Epsilon => {
                        break;
                    }
                 
                    CodeNode::Code(statement) => statement,
                 
                    };
                    elsebody += &statement;
                }
                if !matches!(next_error(tokens, index)?, Token::RightCurly) {
                    return Err(Box::from("expected '}'"));
                }
                let mut code = format!("{}", expression.code);
                code = format!("{code}?:= iftrue{id}, {}\n", expression.name);
                code = format!("{code}:= else{id}\n");
                code = format!("{code}: iftrue{id}\n");
                code = format!("{code}{ifbody}");
                code = format!("{code}:= endif{id}\n");
                code = format!("{code}: else{id}\n");
                code = format!("{code}{elsebody}");
                code = format!("{code}: endif{id}\n");
                return Ok(CodeNode::Code(code));

            } else {
                let mut code = format!("{}", expression.code);
                code = format!("{code}?:= iftrue{id}, {}\n", expression.name);
                code = format!("{code}:= endif{id}\n");
                code = format!("{code}: iftrue{id}\n");
                code = format!("{code}{ifbody}");
                code = format!("{code}: endif{id}\n");
                return Ok(CodeNode::Code(code));
            }
            
        }

        Token::Int => {
            *index += 1;
            match next_error(tokens, index)? {
            Token::Ident(ident) => {
                if matches!(peek_error(tokens, *index)?, Token::Assign) {
                    *index += 1;
                    let expr = parse_expression(tokens, index)?;
                    let code = format!("{}. {}\n= {}, {}\n", expr.code, ident, ident, expr.name);
                    codenode = CodeNode::Code(code);
                } else {
                    let statement = format!(". {}\n", ident);
                    codenode = CodeNode::Code(statement);
                }
            }

            _ => {
                return Err(Box::from("expected identifier"));
            }

            }
        }

        Token::Ident(ident) => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::Assign) {
                return Err(Box::from("expected '=' assignment operator"));
            }
            let expr = parse_expression(tokens, index)?;
            let code = format!("{}= {}, {}\n", expr.code, ident, expr.name);
            codenode = CodeNode::Code(code);
        }

        Token::Return => {
            *index += 1;
            let expr = parse_expression(tokens, index)?;
            let code = format!("{}ret {}\n", expr.code, expr.name);
            codenode = CodeNode::Code(code);
        }

        Token::Print => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::LeftParen) {
                return Err(Box::from("expect '(' closing statement"));
            }

            let expr = parse_expression(tokens, index)?;
            let code = format!("{}.> {}\n", expr.code, expr.name);
            if !matches!(next_error(tokens, index)?, Token::RightParen) {
                return Err(Box::from("expect ')' closing statement"));
            }
            codenode = CodeNode::Code(code);
        }

        Token::Read => {
            *index += 1;
            if !matches!(next_error(tokens, index)?, Token::LeftParen) {
                return Err(Box::from("expect '(' closing statement"));
            }

            let expr = parse_expression(tokens, index)?;
            let code = format!("{}.< {}\n", expr.code, expr.name);

            if !matches!(next_error(tokens, index)?, Token::RightParen) {
                return Err(Box::from("expect ')' closing statement"));
            }
            codenode = CodeNode::Code(code);
        }

        Token::Break => {
            *index += 1;
            let id = peek_label_stack();
            let stmt = format!(":= endloop{id}\n");
            codenode = CodeNode::Code(stmt);
        }

        Token::Continue => {
            *index += 1;
            let id = peek_label_stack();
            let stmt = format!(":= loopbegin{id}\n");
            codenode = CodeNode::Code(stmt);
        }

        _ => {
             return Err(Box::from("invalid statement."));
        }

        }

        if !matches!(next_error(tokens, index)?, Token::Semicolon) {
            return Err(Box::from("expect ';' closing statement"));
        }

        return Ok(codenode);
    }

    }
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, Box<dyn Error>> {
    let mut expr = parse_multiply_expression(tokens, index)?;
    loop {
       let opcode = match peek_error(tokens, *index)? {
       Token::Plus => "+",
       Token::Subtract => "-",
       _ => { break; }
       };

       *index += 1;
       let m_expr = parse_multiply_expression(tokens, index)?;
       let t = create_temp();
       let instr = format!(". {}\n{opcode} {}, {}, {}\n", t, t, expr.name, m_expr.name);
       expr.code += &m_expr.code;
       expr.code += &instr;
       expr.name = t;
    }

    return Ok(expr);
}


fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, Box<dyn Error>> {
    let mut expression = parse_term(tokens, index)?;
    loop {
       let opcode = match peek_error(tokens, *index)? {
       Token::Multiply => "*",
       Token::Divide => "/",
       Token::Modulus => "%",
       _ => { break; }
       };

       *index += 1;
       let node = parse_term(tokens, index)?;
       expression.code += &node.code;
       let t = create_temp();
       let instr = format!(". {}\n{opcode} {}, {}, {}\n", t, t, expression.name, node.name);
       expression.code += &instr;
       expression.name = t;
    }

    return Ok(expression);
}

fn parse_boolean(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, Box<dyn Error>> {
    let node1 = parse_term(tokens, index)?;

    let opcode = match next_error(tokens, index)? {
    Token::Less => "<",
    Token::LessEqual => "<=",
    Token::Greater => ">",
    Token::GreaterEqual => ">=",
    Token::Equality => "==",
    Token::NotEqual => "!=",
    _ => {return Err(Box::from("invalid boolean expression. expected comparison operator."));}
    };
    let node2 = parse_term(tokens, index)?;

    let ret = create_temp();
    let mut code = format!("{}{}. {}\n", node1.code, node2.code, ret);
    code = format!("{code}{opcode} {ret}, {}, {}\n", node1.name, node2.name);
    let expr = Expression {
        code : code,
        name : ret,
    };
    return Ok(expr);
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, Box<dyn Error>> {
    match next_error(tokens, index)? {

    Token::Ident(ident) => {
        match peek_error(tokens, *index)? {
        Token::LeftParen => {
            *index += 1;
            let mut code: String = String::from("");
            let mut params: Vec<String> = vec![];
            loop {
               match peek_error(tokens, *index)? {
  
               Token::RightParen => {
                   *index += 1;
                   break;
               }

               _ => {
                   let param = parse_expression(tokens, index)?;
                   code += &param.code;
                   params.push(param.name);

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
            for param in &params {
                let param_instr = format!("param {}\n", param);
                code += &param_instr;
            }
            let t = create_temp();
            let inst = format!(". {}\ncall {}, {}\n", t, ident, t);
            code += &inst;
            let expr = Expression {
                code : code,
                name : t,
            };
            return Ok(expr);
        }

        _ => {
            let expr = Expression {
                code : String::from(""),
                name : ident.clone(),
            };
            return Ok(expr);
        }

        }
    }

    Token::Num(num) => {
        let expr = Expression {
            code : String::from(""),
            name : format!("{}", num),
        };
        return Ok(expr);
    }

    Token::LeftParen => {
        let expression = parse_expression(tokens, index)?;
        if !matches!(next_error(tokens, index)?, Token::RightParen) {
            return Err(Box::from("expected ')' parenthesis"));
        }

        return Ok(expression);
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

        // valid numbers
        tokens = lex("1 > 2 + < 100");
        match tokens {
        Err(_)=> {assert!(false);}
        Ok((tok,_)) => {
            assert!(tok.len() == 6);
            assert!(matches!(tok[0], Token::Num(1)));
            assert!(matches!(tok[1], Token::Greater));
            assert!(matches!(tok[2], Token::Num(2)));
            assert!(matches!(tok[3], Token::Plus));
            assert!(matches!(tok[4], Token::Less));
            assert!(matches!(tok[5], Token::Num(100)));
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



