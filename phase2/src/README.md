# Building A Parser in Rust

### Introduction

Now that the lexer has been built, the parser takes the sequence of tokens, and identifies the grammar of the programming language based off of the sequence of tokens. If the lexer is the component of the compiler that identifies the “words” and “punctuation” of a programming language, the parser is the the component of the compiler that identifiers the “sentences” and “paragraphs” of the programming language.

The job of the parser is to identify which tokens represent while loops, if statements, function headers, the function body, variable declarations, and constant variables. If the parser finds a misplaced sequence of tokens, the parser should notify the programming language user of the misplaced tokens.

In Phase 2, we will be creating a parser. A parser takes a sequence of tokens, and determines what the sequence of tokens represents. For example, a sequence of tokens in order: identifier, equals sign, number (e.g. “variable = 0”) will be recognized as a statement by the parser. The output of the parser will be a print out of the production rules of the parser.

### Building a Parser

We will be building a simple top down recursive descent parser without backtracking. Let's start with a simple declaration statement `int a;`. We can parse a simple declaration statement with the following pseudocode:

```
parse_declaration_statement(tokens: Array<Token>) -> Result(value,Error) {
    t := next_token(tokens)?
    if t != INT KEYWORD,
        return Err("expected integer keyword")

    t := next_token(tokens)?
    if t != IDENTIFIER,
        return Err("expected identifier")
    
    t := next_token(tokens)?
    if t != SEMICOLON,
        return Err("expected semicolon ';'")

    return Success
}
```

A simple declaration statement is a sequence of an `integer keyword`, followed by an `identifier`, followed by a `semicolon`. This simple pseudo code checks that the `tokens` has the specificed sequence.

### Branching Parser Behavior

Programming Language Grammars can have branching behavior that allows for expressive power. For example, there are multiple possibilities `int a;` , `int a = 0;`, or `int a = b;`. Here is some pseudocode to parse that grammar:

```
parse_declaration_statement(tokens: Array<Token>) -> Result(value,Error) {
    t := next_token(tokens)?
    if t != INT KEYWORD,
        return Err("expected integer keyword")

    t := next_token(tokens)?
    if t != IDENTIFIER,
        return Err("expected identifier")
    
    t := next_token(tokens)?
    if t == SEMICOLON,
        return Success

    if t == EQUAL {
       t := next_token(tokens)?
       if t == NUMBER
            t := next_token(tokens)?
            if t == SEMICOLON,
                return Success
            else
                return Err("expected semicolon ';'")
            
       if t == IDENTIFIER
            t := next_token(tokens)?
            if t == SEMICOLON,
                return Success
            else
                return Err("expected semicolon ';'")

       return Err("expected number or identifier")
    }

    return Err("expected semicolon ';' or '=' assignment operator")
}
```





