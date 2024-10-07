# Phase 1: Building A Lexer in Rust

### Introduction

A lexer converts text into meaningful tokens belonging to categories defined by a "lexer" program. In a programming language compiler, a lexer partitions text into tokens such as identifiers, operators, grouping symbols, and data types.

For example, if we have a statement:
```
c = a + b;
```

The lexer should partition the statement into the following tokens:
```
Identifier("c")
Assign
Identifier("a")
Plus
Identifier("b")
Semicolon
```

To do another example, if we have a statement:
```
int [8] array;
```

The lexer should partition the statement into the following tokens:
```
Int
LeftBracket
Number(8)
RightBracket
Identifier("array")
Semicolon
```

For your lexer, your program should be written in the following way:
```
fn main() {

   // 1) open and read text contents from a file 
   // 2) lex the text contents of the file
   // 3) print out the tokens that have been lexed
}
```

The lexer takes as input a piece of code represented as a string, and outputs a list of tokens based on the input. If the lexer detects an invalid token, then the 
lexer spits out an error, and the compiler should halt. If there is an error detected, the compiler should inform the user about what is wrong.

The lexer must meet the following functionality:
- The lexer should have correct rules for detecting valid/invalid identifiers based on the language 
specification
- The lexer should detect and remove comments properly (e.g. `#This is a comment` should parse as 
one token and then removed from the list of tokens)
- The lexer should detect and report proper error messages for invalid identifiers and unrecognized 
symbols (e.g. invalid identifier tokens such as `2a` is an error)
The lexer is allowed to halt at the first error.
Please take note that a lexer does not need to check for errors such as misplaced semicolons, balanced parenthesis, or balanced curly brace. Handling those error cases is up to the parser in Phase 2.
Comments should not be part of the output list of tokens.


### Table of Tokens

For your lexer, this is the complete list of tokens you need to identify for Phase 1.

|Symbol                | Token Name   |
|----------------------|--------------|
|func                  | Func         |
|return                | Return       |
|int                   | Int          |
|print                 | Print        |
|read                  | Read         |
|while                 | While        |
|if                    | If           |
|else                  | Else         |
|break                 | Break        |
|continue              | Continue     |
|(                     | LeftParen    |
|)                     | RightParen   |
|{                     | LeftCurly    |
|}                     | RightCurly   |
|[                     | LeftBracket  |
|]                     | RightBracket |
|,                     | Comma        |
|;                     | Semicolon    |
|+                     | Plus         |
|-                     | Subtract     |
|*                     | Multiply     |
|/                     | Divide       |
|%                     | Modulus      |
|=                     | Assign       |
|<                     | Less         |
|<=                    | LessEqual    |
|>                     | Greater      |
|>=                    | GreaterEqual |
|==                    | Equality     |
|!=                    | NotEqual     |
|variable_name         | Ident        |
|10311517              | Num          |

#### Variable Identifier Names

Variables begin with an upper or lower case letters A-Z followed by a sequence of underscores or numbers. Examples include:
```
int variable_name;
int var1;
int october_31_1517;
```

#### Comments

Comments can be single line comments starting with `#`. For example:

```
int x; #This is a variable declaration.
```


### Opening and read the entire file

Code to open and read the entire file. This is used to get the all the high level programming language code
from the file.

```
use std::fs;

fn main() {
    let filename = "file.txt";
    let code = match fs::read_to_string(filename) {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
        code
    } 

    };

    println!("Code:");
    println!("{}", code);
}
```

### Building a simple lexer

Let's build a simple lexer that identifies numbers with multiple digits and 
the basic operation `+`. The other operations such as `-`, `*`, `/` can be easily figured out by
modifying the currrent example. We start with an index value of zero, and traverse through the entire string, adding tokens to the list of tokens.
To randomly access a letter of a string, use `code.as_bytes()` to get the string array as a byte array. A number token is parsed by finding the beginning index of the number 0-9,
finding the ending index of the number 0-9, and getting the substring token `code[start..end]`. We append an `End` Token at the end of lexing, which simplifies the parsing implementation in Phase 2.

```
// This is a lexer that parses numbers and math operations
fn lex(mut code: &str) -> Result<Vec<Token>, String> {
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

    '+' => {
      tokens.push(Token::Plus);
      i += 1;
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
```

An identifier could be identified in a way similar to how numbers are identified, just set a start index for the beginning
of the identifier and the end index for the end of the identifier. `if`, `while`, `read` keywords conflicts with identifiers. When creating a identifier, check to see that the string is not a keyword. If it is in the list of keywords, return the appropriate keyword. Else, create a string
using `String::from(token)` and create an identifier token. Below is the code for such a procedure:

```
fn create_identifier(code: &str) -> Token {
  match code {
  "func" => Token::Func,
  "return" => Token::Return,
  "int" => Token::Int,

  // todo: implement all keywords...
  // ... all keywords...

  "read" => Token::Read,
  "while" => Token::While,
  "if" => Token::If,
  _ => Token::Ident(String::from(code)),
  }
}
```

### Example Output

#### add.tt

Given the following `add.tt` program:
```
func main() {
  int a;
  int b;
  int c;
  a = 100;
  b = 50;
  c = a + b;
  print(c);
}
```
The following output is the correct output:
```
Func
Ident("main")
LeftParen
RightParen
LeftCurly
Int
Ident("a")
Semicolon
Int
Ident("b")
Semicolon
Int
Ident("c")
Semicolon
Ident("a")
Assign
Num(100)
Semicolon
Ident("b")
Assign
Num(50)
Semicolon
Ident("c")
Assign
Ident("a")
Plus
Ident("b")
Semicolon
Print
LeftParen
Ident("c")
RightParen
Semicolon
RightCurly
End
```

#### math.tt

Given the follow `math.tt` program:
```
# A simple program which shows mathematical operations.

func main() {
  int a;
  int b;
  int c;

  a = 100;
  b = 50;

  # This should output '150'
  c = a + b;
  print(c);

  # This should output '50'
  c = a - b;
  print(c);

  # This should output '5000'
  c = a * b;
  print(c);

  # This should output '2'
  c = a / b;
  print(c);

  # This should output '0'
  c = a % b;
  print(c);

  # Complex Expression. (4 + 2) * 7
  a = 4;
  b = 7;
  c = (a + 2) * b;
  print(c);
}
```

The following output is the correct output:
```
Func
Ident("main")
LeftParen
RightParen
LeftCurly
Int
Ident("a")
Semicolon
Int
Ident("b")
Semicolon
Int
Ident("c")
Semicolon
Ident("a")
Assign
Num(100)
Semicolon
Ident("b")
Assign
Num(50)
Semicolon
Ident("c")
Assign
Ident("a")
Plus
Ident("b")
Semicolon
Print
LeftParen
Ident("c")
RightParen
Semicolon
Ident("c")
Assign
Ident("a")
Subtract
Ident("b")
Semicolon
Print
LeftParen
Ident("c")
RightParen
Semicolon
Ident("c")
Assign
Ident("a")
Multiply
Ident("b")
Semicolon
Print
LeftParen
Ident("c")
RightParen
Semicolon
Ident("c")
Assign
Ident("a")
Divide
Ident("b")
Semicolon
Print
LeftParen
Ident("c")
RightParen
Semicolon
Ident("c")
Assign
Ident("a")
Modulus
Ident("b")
Semicolon
Print
LeftParen
Ident("c")
RightParen
Semicolon
Ident("a")
Assign
Num(4)
Semicolon
Ident("b")
Assign
Num(7)
Semicolon
Ident("c")
Assign
LeftParen
Ident("a")
Plus
Num(2)
RightParen
Multiply
Ident("b")
Semicolon
Print
LeftParen
Ident("c")
RightParen
Semicolon
RightCurly
End
```

#### array.tt

Given the following `array.tt` example:
```
func main() {
    int [4] array;

    # Should print out '2'
    array[0] = 2;
    print(array[0]);

    # Should print out '4'
    array[1] = array[0] + array[0];
    print(array[1]);

    # Should print out '8'
    array[2] = array[1] + 2 * 2;
    print(array[2]);

}
```
The following output is the correct output:
```
Func
Ident("main")
LeftParen
RightParen
LeftCurly
Int
LeftBracket
Num(4)
RightBracket
Ident("array")
Semicolon
Ident("array")
LeftBracket
Num(0)
RightBracket
Assign
Num(2)
Semicolon
Print
LeftParen
Ident("array")
LeftBracket
Num(0)
RightBracket
RightParen
Semicolon
Ident("array")
LeftBracket
Num(1)
RightBracket
Assign
Ident("array")
LeftBracket
Num(0)
RightBracket
Plus
Ident("array")
LeftBracket
Num(0)
RightBracket
Semicolon
Print
LeftParen
Ident("array")
LeftBracket
Num(1)
RightBracket
RightParen
Semicolon
Ident("array")
LeftBracket
Num(2)
RightBracket
Assign
Ident("array")
LeftBracket
Num(1)
RightBracket
Plus
Num(2)
Multiply
Num(2)
Semicolon
Print
LeftParen
Ident("array")
LeftBracket
Num(2)
RightBracket
RightParen
Semicolon
RightCurly
End
```

#### function.tt

Given the following `function.tt`:
```
func add(int a, int b) {
    return a + b;
}

func mul(int a, int b) {
     return a * b;
}

func main() {
    int a;
    int b;
    int c;
    a = 10;
    b = 2;
    c = add(a, b);
    print(c);
    c = mul(c, a + b);
    print(c);
}
```
The following output is the correct output:
```
Func
Ident("add")
LeftParen
Int
Ident("a")
Comma
Int
Ident("b")
RightParen
LeftCurly
Return
Ident("a")
Plus
Ident("b")
Semicolon
RightCurly
Func
Ident("mul")
LeftParen
Int
Ident("a")
Comma
Int
Ident("b")
RightParen
LeftCurly
Return
Ident("a")
Multiply
Ident("b")
Semicolon
RightCurly
Func
Ident("main")
LeftParen
RightParen
LeftCurly
Int
Ident("a")
Semicolon
Int
Ident("b")
Semicolon
Ident("a")
Assign
Ident("add")
LeftParen
Num(10)
Comma
Num(2)
RightParen
Semicolon
Print
LeftParen
Ident("a")
RightParen
Semicolon
Ident("b")
Assign
Ident("mul")
LeftParen
Ident("a")
Comma
Ident("a")
Plus
Ident("b")
RightParen
Semicolon
Print
LeftParen
Ident("b")
RightParen
Semicolon
RightCurly
End
```

#### loop.tt

Given the simple loop `loop.tt`:
```
func main() {
    int i;
    i = 0;
    while i < 10 {
        print(i);
        i = i + 1;
    }
}
```
The following output is the correct output:
```
Func
Ident("main")
LeftParen
RightParen
LeftCurly
Int
Ident("i")
Semicolon
Ident("i")
Assign
Num(0)
Semicolon
While
Ident("i")
Less
Num(10)
LeftCurly
Print
LeftParen
Ident("i")
RightParen
Semicolon
Ident("i")
Assign
Ident("i")
Plus
Num(1)
Semicolon
RightCurly
RightCurly
End
```

#### if.tt
Given the following `if.tt`:
```
func main() {
    int a;
    int b;
    int c;

    
    a = 100;
    b = 50;
    if a < b {
        c = 0;
    } else {
        c = 1;
    }

    # Should print out '1'.
    print(c);



    a = 100;
    b = 50;
    if a >= b {
        c = 0;
    } else {
        c = 1;
    }

    # Should print out '0'
    print(c);
}
```

The following is the correct lexical output:
```
Func
Ident("main")
LeftParen
RightParen
LeftCurly
Int
Ident("a")
Semicolon
Int
Ident("b")
Semicolon
Int
Ident("c")
Semicolon
Ident("a")
Assign
Num(100)
Semicolon
Ident("b")
Assign
Num(50)
Semicolon
If
Ident("a")
Less
Ident("b")
LeftCurly
Ident("c")
Assign
Num(0)
Semicolon
RightCurly
Else
LeftCurly
Ident("c")
Assign
Num(1)
Semicolon
RightCurly
Print
LeftParen
Ident("c")
RightParen
Semicolon
Ident("a")
Assign
Num(100)
Semicolon
Ident("b")
Assign
Num(50)
Semicolon
If
Ident("a")
GreaterEqual
Ident("b")
LeftCurly
Ident("c")
Assign
Num(0)
Semicolon
RightCurly
Else
LeftCurly
Ident("c")
Assign
Num(1)
Semicolon
RightCurly
Print
LeftParen
Ident("c")
RightParen
Semicolon
RightCurly
End
```

#### nested_loop.tt

Given the following `nested_loop.tt`:
```
func main() {
    int i;
    int j;
    i = 0;
    while i < 2 {
        j = 0;
        while j < 3 {
            print(j);
            j = j + 1;
        }
        i = i + 1;
    }
}
```

The following is the correct lexical output:
```
Func
Ident("main")
LeftParen
RightParen
LeftCurly
Int
Ident("i")
Semicolon
Int
Ident("j")
Semicolon
Ident("i")
Assign
Num(0)
Semicolon
While
Ident("i")
Less
Num(2)
LeftCurly
Ident("j")
Assign
Num(0)
Semicolon
While
Ident("j")
Less
Num(3)
LeftCurly
Print
LeftParen
Ident("j")
RightParen
Semicolon
Ident("j")
Assign
Ident("j")
Plus
Num(1)
Semicolon
RightCurly
Ident("i")
Assign
Ident("i")
Plus
Num(1)
Semicolon
RightCurly
RightCurly
End
```

#### break.tt

Given the following `break.tt`:
```
func main() {
    int i;
    i = 0;
    while i < 10 {
        if i >= 4 {
            break;
        }
        print(i);
        i = i + 1;
    }
}
```
The following is the correct lexical output:
```
Func
Ident("main")
LeftParen
RightParen
LeftCurly
Int
Ident("i")
Semicolon
Ident("i")
Assign
Num(0)
Semicolon
While
Ident("i")
Less
Num(10)
LeftCurly
If
Ident("i")
GreaterEqual
Num(4)
LeftCurly
Break
Semicolon
RightCurly
Print
LeftParen
Ident("i")
RightParen
Semicolon
Ident("i")
Assign
Ident("i")
Plus
Num(1)
Semicolon
RightCurly
RightCurly
End
```

### Submission
A correct and complete lexer should be able to lex all the example programs correctly, transforming 
the string into a list of tokens. At the end of lexing, print out the tokens using a for loop. An 
example of this can be found in “phase1/src/main.rs”.

### Rubric

Total Points: 100 points total

Demo/Group Participation 10 points

Proper Output for Example Test Cases 80 points (10 points each test case):

* add.tt
* array.tt
* break.tt
* function.tt
* if.tt
* loop.tt
* math.tt
* nested_loop.tt
  
Proper Output for Lexical Errors 10 points

All projects can be turned in up to 1 week late. Each day the project is late, 3% will be deducted per
day for up to 21%. After a week, projects will not be accepted.

