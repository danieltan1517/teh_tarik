// The Rust Programming Language: A Crash Course and Building Our First Lexer
// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.
// Creating a Lexer By Hand.

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;

// used for error handling.
use std::error::Error;

fn main() {

    // 1. Let's start from the Basics of the Programming Language:
    // 
    // Let's start out with a basic "Hello Rust!"
    println!("Hello Rust!");

    // 2. Variable Declarations.

    // Variables are declared by putting 'let' in front of the variable
    // In the C programming language, variables are mutable by default.
    // In Rust, variables are read-only constants by default.
    // to make a variable mutable, put 'mut' in front of the variable.
    {
        // declares a 'variable' that is read-only. 
        // Type Inference tells us 'variable' is an integer
        let variable = 0;

        // here's a set of different ways to print 'variable' to the screen
        // all of them do the same thing.
        println!("variable = {}", variable);
        println!("variable = {variable}");

        // if you try to mutate 'variable', it will result in a compile error.
        // uncomment the following line to get a compile error:
        // variable += 100;
    }

    {
        // declare a mutable variable
        let mut var = 0;
        while var < 3 {
            // mutate the 'var'
            println!("var = {}", var);
            var += 1;
        }

        // create a block of code that evaluates to an expression.
        // https://doc.rust-lang.org/reference/expressions/block-expr.html
        let v = {
            let mut num = 0;
            while var < 5 {
                num += var;
                var += 1;
            }
            num 
        };

        println!("v = {}", v);
    }

    // 3. Strings
    // You can find documentation on string at: https://doc.rust-lang.org/rust-by-example/std/str.html
    // There are two strings in Rust:
    //    - String
    //    - &str
    // 'String' is a heap allocated Vec<u8> that is not null terminated. 'String' can be modified.
    // '&str' is a read-only string that is only assigned once and cannot be modified after the first assignment
    {
       // to convert from &str => String, use String::from(...)
       let s: String = String::from("Cat in the hat");

       // to convert from String => &str, use &string_variable
       let reference: &str = &s;
       println!("s = {s}");
       println!("reference = {reference}");
    }

    // 4. References
    // Just like C++, Rust uses references. You can use references to pass data by reference.
    {
       let mut num: i32 = 4;
       function(&num);
       function_with_ref(&mut num);
       println!("num: i32 = {}", num);

       fn function(num: &i32) {
           println!("num: &i32 = {}", num);
       }

       fn function_with_ref(num: &mut i32) {
           // add 200 to num.
           *num += 200;
           println!("num: &i32 = {}", num);
       }
    }

    // 5. Pattern Matching
    // match behaves just like a C switch statement. unlike C switch, match does not have fallthrough.
    // match is more powerful given the pattern matching
    // _ => {} can be used to catch all the rest which cannot be matched.
    {
       let animal = "cat";
       match animal {
       "cow" => {
           println!("cow says: \"Moo!\"");
       }
       "cat" => {
           println!("cat says: \"Meow!\"");
       }
       "dog" => {
           println!("dog says: \"Wuff!\"");
       }
       _ => {
           println!("default case = {}", animal);
       }

       }

       let num = 3;
       match num {
       1 => println!("January is the first month of the year."),
       2 => println!("Febuary is the second month of the year."),
       3 => println!("March is the third month of the year."),
       _ => println!("...Etc."),
       }
    }


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

    Ok(data) => data,
    
    };


    // print out the lexer tokens parsed.

    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("Expression:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
      println!("{:?}", t);
    }

}

// Creating an Enum within Rust.
// Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums are a way of saying a value is one of a possible set of values.
// Unlike C, Rust enums can have values associated with that particular enum value.
// for example, a Num has a 'i32' value associated with it, 
// but Plus, Subtract, Multiply, etc. have no values associated with it.
#[derive(Debug, Clone)]
enum Token {
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign,
  Num(i32),
}

// In Rust, you can model the function behavior using the type system.
// https://doc.rust-lang.org/std/result/
// Result < Vec<Token>, Box<dyn Error>>
// means that this function can either return:
// - A list of tokens as a Vec<Token>
// - Or an error message
// If there is an error, it will return an error
// If successful, it will return Vec<Token>
// A Result is an enum like this:
// enum Result {
//     Ok(the_result),
//     Err(the_error),
// }


// This is a lexer that parses numbers and math operations
// try to add identifier parsing to this lexer.
fn lex(code: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = vec![];
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
                StateMachine::Init
            }
        }

        };

        token_end += 1;

        // actions of state machine.
        match state_machine {

        StateMachine::Init => {
             match character {
             '+' => tokens.push(Token::Plus),
             '-' => tokens.push(Token::Subtract),
             '*' => tokens.push(Token::Multiply),
             '/' => tokens.push(Token::Divide),
             '%' => tokens.push(Token::Modulus),
             '=' => tokens.push(Token::Assign),
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
    }

    return Ok(tokens);

    fn create_number(start: usize, end: usize, code: &str) -> i32 {
        // this code should correctly parse because the lexer verified that this is correct.
        // quit.
        let token = &code[start..end];
        match token.parse::<i32>().unwrap()
        Err(_) => panic!("Error. Logic Error: Lexer failed to lex number \"{token}\" correctly"),
        Ok(num) => num,
        }
    }

    enum StateMachine {
        Init,
        Number,
    }

}

// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;

    #[test]
    fn lexer_test() {
        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));
    }

}
