// use std::error::Error;

pub fn compile_and_run(mut code: &str) {
    println!("Generated code:");
    println!("{code}");
    loop {
        let (val, rest) = lex_ir(code);
        if matches!(val, None) {
            break;
        }

    }
}

fn lex_ir(mut code: &str) -> (Option<IRTok>, &str) {
   
    #[derive(Debug)]
    enum StateMachine {
        Initial,
        Lit,
        Ident,
        Comments,
    }

    fn tokenize(s: &str) -> Option<IRTok> {
        use IRTok::*;
        match s {
        "%int" => Some(Int),
        "%int[]" => Some(IntArray),
        "%call" => Some(Call),
        "%ret" => Some(Return),
        "%out" => Some(Out),
        "%in" => Some(In),
        "%mov" => Some(Mov),
        "%add" => Some(Add),
        "%sub" => Some(Sub),
        "%mult" => Some(Mult),
        "%div" => Some(Div),
        "%mod" => Some(Mod),
        "%lt" => Some(LessThan),
        "%le" => Some(LessEqual),
        "%neq" => Some(NotEqual),
        "%eq" => Some(Equal),
        "%gt" => Some(GreaterThan),
        "%ge" => Some(GreaterEqual),
        "%jmp" => Some(Jump),
        "%brif" => Some(BranchIf),
        "%brifn" => Some(BranchIfNot),
        _ => None,

        }
    }
 
    let mut state = StateMachine::Initial;

    for (i, c) in code.chars().enumerate() {
        state = match state {

        StateMachine::Initial => {
            if c.is_whitespace() {
                //code = &code[i..];
                continue;
            }
            match c {
            ',' => return (Some(IRTok::Comma), &code[i + 1..]),
            '%' => StateMachine::Lit,
            '[' => return (Some(IRTok::LBrace), &code[i + 1..]),
            ']' => return (Some(IRTok::RBrace), &code[i + 1..]),
            ';' => StateMachine::Comments,
            _   => StateMachine::Ident,
            }
        }

        StateMachine::Lit => {
            if c.is_whitespace() || c == ',' {
                let tok = tokenize(&code[..i]);
                println!("{:?}", tok);
                return (tok, &code[i+1..]);
            }
            StateMachine::Lit
        }

        StateMachine::Comments => {
            if c == '\n' {
                //code = &code[i+1..];
                StateMachine::Initial
            } else {
                StateMachine::Comments
            }
        }

        _ => todo!()
        
        };
    }

    match state {

    StateMachine::Lit => {
        let tok = tokenize(&code);
        println!("{:?} {}", state, code);
        return (tok, "");
    }

    _ => {
        println!("{:?} {}", state, code);
        todo!()
    }

    }

    todo!()
}

#[cfg(test)]
mod ir_tests {
    use crate::compiler::*;

    #[test]
    fn ir_lexer() {
        assert!(matches!(lex_ir("%int"), (Some(IRTok::Int), _)));
        assert!(matches!(lex_ir("%int[]"), (Some(IRTok::IntArray), _)));
        assert!(matches!(lex_ir("%call"), (Some(IRTok::Call), _)));
        assert!(matches!(lex_ir("%ret"), (Some(IRTok::Return), _)));
        assert!(matches!(lex_ir("%out"), (Some(IRTok::Out), _)));
        assert!(matches!(lex_ir("%in"), (Some(IRTok::In), _)));
        assert!(matches!(lex_ir("%mov"), (Some(IRTok::Mov), _)));
        assert!(matches!(lex_ir("%add"), (Some(IRTok::Add), _)));
        assert!(matches!(lex_ir("%sub"), (Some(IRTok::Sub), _)));
        assert!(matches!(lex_ir("%mult"), (Some(IRTok::Mult), _)));
        assert!(matches!(lex_ir("%div"), (Some(IRTok::Div), _)));
        assert!(matches!(lex_ir("%mod"), (Some(IRTok::Mod), _)));
        assert!(matches!(lex_ir("%lt"), (Some(IRTok::LessThan), _)));
        assert!(matches!(lex_ir("%le"), (Some(IRTok::LessEqual), _)));
        assert!(matches!(lex_ir("%neq"), (Some(IRTok::NotEqual), _)));
        assert!(matches!(lex_ir("%eq"), (Some(IRTok::Equal), _)));
        assert!(matches!(lex_ir("%gt"), (Some(IRTok::GreaterThan), _)));
        assert!(matches!(lex_ir("%ge"), (Some(IRTok::GreaterEqual), _)));
        assert!(matches!(lex_ir("%jmp"), (Some(IRTok::Jump), _)));
        assert!(matches!(lex_ir("%brif"), (Some(IRTok::BranchIf), _)));
        assert!(matches!(lex_ir("%brifn"), (Some(IRTok::BranchIfNot), _)));
        assert!(matches!(lex_ir("[are"), (Some(IRTok::LBrace), "are")));
        assert!(matches!(lex_ir("]are"), (Some(IRTok::RBrace), "are")));
        assert!(matches!(lex_ir(",are"), (Some(IRTok::Comma), "are")));

        let code = "; This is a comment\n
                   %mov";
        assert!(matches!(lex_ir(code), (Some(IRTok::Mov), _)));
    }
}


#[derive(Debug)]
enum IRTok {
    // OpCode

    // declarations.
    Int,
    IntArray,

    // function calling routines.
    Call,
    Return,

    // input/output routines.
    Out,
    In,

    // mathematical operators.
    Mov,
    Add,
    Sub,
    Mult,
    Div,
    Mod,

    // comparison operators.
    LessThan,
    LessEqual,
    NotEqual,
    Equal,
    GreaterEqual,
    GreaterThan,

    // labels/branching
    Label,
    Jump,
    BranchIf,
    BranchIfNot,

    Comma,
    LBrace,
    RBrace,

    Num(i32),
    Var(String),
}

