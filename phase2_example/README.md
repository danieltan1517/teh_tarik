# Example Parser in Rust

This is an example parser created in Rust. 

Please note that this example is an incomplete parser. It does not correctly handle operator precedence, arrays, or while loops or if statements. Use this example and build off it when creating your own compiler.

The best way to test a complex piece of software for robustness is to break it down into smaller components, and verify that those smaller components are functioning correctly. Here is an example of how to test an individual statement. As a reminder, you can test your code by writing tests, and calling "cargo test" to run your test cases.

```
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
```
