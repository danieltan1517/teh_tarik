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
    let high_level_code = match result {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
      code
    } 

    };

    lexer(&high_level_code);
}

enum TokenType<'a> {
  FunctionKeyword(&'a str),
  EOF
}


fn lexer(code: &str) -> (TokenType, &str) {
    println!("Here is the high level code: ");
    println!("{}", code);

    for chr in code.chars() {
      println!("{}", chr);
    }

    // split_at()
    return (TokenType::EOF, code);
}











