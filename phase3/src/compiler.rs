pub fn compile_and_run(code: &str) {
    println!("Generated code:");
    println!("{code}");
    let tokens = lex_ir(code);
    for t in &tokens {
        println!("{:?}", t);
    }
}

fn lex_ir(mut code: &str) -> Vec<IRTok> {
    let mut tokens: Vec<IRTok> = vec![];
    while code.len() > 0 {
        let (tok, rest) = lex_ir_token(code);
        match tok {
        None => break,
        Some(value) => tokens.push(value),
        }
        code = rest;
    }

    return tokens;
}

fn lex_ir_token(mut code: &str) -> (Option<IRTok>, &str) {
   
    #[derive(Debug)]
    enum StateMachine {
        Initial,
        Lit,
        Ident,
        Comments,
    }

    fn opcode(s: &str) -> Option<IRTok> {
        use IRTok::*;
        match s {
        "%func" => Some(Func),
        "%endfunc" => Some(EndFunc),
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
        "%branch_if" => Some(BranchIf),
        "%branch_if_not" => Some(BranchIfNot),
        _ => {
            match s[1..].parse::<i32>() {
            Ok(val) => Some(Num(val)),
            Err(_) => None,
            }
        }

        }
    }

    // skip left whitespace.
    for (i, c) in code.chars().enumerate() {
        if c.is_whitespace() && c != '\n' {
            continue;
        }
        code = &code[i..];
        break;
    }
 
    let mut state = StateMachine::Initial;

    for (i, c) in code.chars().enumerate() {
        state = match state {

        StateMachine::Initial => {
            if c == '\n' {
                return (Some(IRTok::EndInstr), &code[i + 1..]);
            }
            if c.is_whitespace() {
                continue;
            }
            match c {
            '%' => StateMachine::Lit,
            ',' => return (Some(IRTok::Comma), &code[i + 1..]),
            '[' => return (Some(IRTok::LBrace), &code[i + 1..]),
            ']' => return (Some(IRTok::RBrace), &code[i + 1..]),
            ';' => StateMachine::Comments,
            _ => StateMachine::Ident,
            }
        }

        StateMachine::Lit => {
            if c == ',' || c == '\n' {
                let tok = opcode(&code[..i]);
                return (tok, &code[i..]);
            }
            if c.is_whitespace() {
                let tok = opcode(&code[..i]);
                return (tok, &code[i+1..]);
            }

            StateMachine::Lit
        }

        StateMachine::Comments => {
            if c == '\n' {
                return (Some(IRTok::EndInstr), &code[i + 1..]);
            } else {
                StateMachine::Comments
            }
        }

        StateMachine::Ident => {
            if c == ',' || c == ';' || c == '\n' {
                let tok = IRTok::Var(String::from(&code[..i]));
                return (Some(tok), &code[i..]);
            }

            if c.is_whitespace() {
                let tok = IRTok::Var(String::from(&code[..i]));
                return (Some(tok), &code[i+1..]);
            }

            StateMachine::Ident
        }
        
        };
    }

    match state {

    StateMachine::Lit => {
        return (opcode(code), "");
    }

    StateMachine::Ident => {
        let tok = IRTok::Var(String::from(code));
        return (Some(tok), "");
    }


    _ => {
        println!("{:?} {}", state, code);
        todo!()
    }

    }
}

#[cfg(test)]
mod ir_tests {
    use crate::compiler::*;

    #[test]
    fn ir_lexer() {
        assert!(matches!(lex_ir("  %int"), (Some(IRTok::Int), _)));
        assert!(matches!(lex_ir(" %int[]"), (Some(IRTok::IntArray), _)));
        assert!(matches!(lex_ir("%call"), (Some(IRTok::Call), _)));
        assert!(matches!(lex_ir("%ret"), (Some(IRTok::Return), _)));
        assert!(matches!(lex_ir("%out"), (Some(IRTok::Out), _)));
        assert!(matches!(lex_ir("   %in"), (Some(IRTok::In), _)));
        assert!(matches!(lex_ir("%mov  "), (Some(IRTok::Mov), _)));
        assert!(matches!(lex_ir("%add"), (Some(IRTok::Add), _)));
        assert!(matches!(lex_ir("%sub"), (Some(IRTok::Sub), _)));
        assert!(matches!(lex_ir("%mult"), (Some(IRTok::Mult), _)));
        assert!(matches!(lex_ir("  %div"), (Some(IRTok::Div), _)));
        assert!(matches!(lex_ir("%mod"), (Some(IRTok::Mod), _)));
        assert!(matches!(lex_ir("%lt"), (Some(IRTok::LessThan), _)));
        assert!(matches!(lex_ir("%le"), (Some(IRTok::LessEqual), _)));
        assert!(matches!(lex_ir("%neq"), (Some(IRTok::NotEqual), _)));
        assert!(matches!(lex_ir("%eq"), (Some(IRTok::Equal), _)));
        assert!(matches!(lex_ir("%gt"), (Some(IRTok::GreaterThan), _)));
        assert!(matches!(lex_ir("%ge"), (Some(IRTok::GreaterEqual), _)));
        assert!(matches!(lex_ir("%jmp"), (Some(IRTok::Jump), _)));
        assert!(matches!(lex_ir("%branch_if"), (Some(IRTok::BranchIf), _)));
        assert!(matches!(lex_ir("%branch_if_not"), (Some(IRTok::BranchIfNot), _)));
        assert!(matches!(lex_ir("[are"), (Some(IRTok::LBrace), "are")));
        assert!(matches!(lex_ir("]are"), (Some(IRTok::RBrace), "are")));
        assert!(matches!(lex_ir(",are"), (Some(IRTok::Comma), "are")));
        assert!(matches!(lex_ir("%bad"), (None, _)));

        let code = "; This is a comment\n%mov";
        assert!(matches!(lex_ir(code), (Some(IRTok::EndInstr), "%mov")));
    }
}


#[derive(Debug)]
enum IRTok {
    // func
    Func,
    EndFunc,

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

    EndInstr,

    Num(i32),
    Var(String),
}

