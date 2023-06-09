pub fn compile_and_run(code: &str) {
    println!("Generated code:");
    println!("{code}");
    let tokens = lex_ir(code);
    match parse_ir(&tokens, &mut 0) {
    Some(_) => {
        println!("Good.");
    }
    None => {
        println!("Error.");
    }
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
    for t in &tokens {
        println!("{:?}", t);
    }

    return tokens;
}

fn parse_ir(tokens: &Vec<IRTok>, idx: &mut usize) -> Option<()> {
    if !matches!(next(tokens, idx)?, IRTok::Func) {
        return None;
    }

    let _func_ident = match next(tokens, idx)? {
    IRTok::Var(func_ident) => func_ident,
    _ => return None,
    };

    if !matches!(next(tokens, idx)?, IRTok::EndInstr) {
        return None;
    }

    loop {

        match parse_instruction(tokens, idx) {
        Some(_) => {
        }
        None => break,
        }
    }

    if !matches!(next(tokens, idx)?, IRTok::EndFunc) {
        return None;
    }

    return Some(());
}

fn parse_instruction(tokens: &Vec<IRTok>, idx: &mut usize) -> Option<()> {
    let opcode = peek(tokens, *idx)?;
    match opcode {

    // declarations.
    IRTok::Int => {
        *idx += 1;
        let ident = match next(tokens, idx)? {
        IRTok::Var(ident) => ident,
        _ => return None,
        };
    }

    IRTok::IntArray => {
        *idx += 1;
        let ident = match next(tokens, idx)? {
        IRTok::Var(ident) => ident,
        _ => return None,
        };

        if !matches!(next(tokens, idx)?, IRTok::Comma) {
            return None;
        }

        let num = match next(tokens, idx)? {
        IRTok::Num(num) => num,
        _ => return None,
        };
    }

    // function calling routines.
    IRTok::Call => {
        *idx += 1;
        todo!()
    }

    IRTok::Return => {
        *idx += 1;
        match next(tokens, idx)? {
        IRTok::Num(_) => {}
        IRTok::Var(_) => {}
        _ => return None,
        };
    }

    // input/output routines.
    IRTok::Out => {
        *idx += 1;
        match next(tokens, idx)? {
        IRTok::Var(_) => {}
        _ => return None,
        };
    }

    IRTok::In => {
        *idx += 1;
        match next(tokens, idx)? {
        IRTok::Var(_) => {}
        _ => return None,
        };
    }

    // mathematical operators.
    IRTok::Mov => {
        *idx += 1;
        match next(tokens, idx)? {
        IRTok::Var(_) => {}
        _ => return None,
        }
 
        if !matches!(next(tokens, idx)?, IRTok::Comma) {
            return None;
        }
        
        match next(tokens, idx)? {
        IRTok::Var(_) => {}
        IRTok::Num(_) => {}
        _ => return None,
        }
    }

    IRTok::Add => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::Sub => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::Mult => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::Div => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::Mod => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    // comparison operators.
    IRTok::LessThan => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::LessEqual => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::NotEqual => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::Equal => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::GreaterEqual => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    IRTok::GreaterThan => {
        *idx += 1;
        addr_code3(tokens, idx)?;
    }

    // labels/branching
    //Label,
    IRTok::Jump => {
        *idx += 1;
    }

    IRTok::BranchIf => {
        *idx += 1;
    }

    IRTok::BranchIfNot => {
        *idx += 1;
    }

    _ => {
        return None;
    }

    }

    if !matches!(next(tokens, idx)?, IRTok::EndInstr) {
        return None;
    }

    return Some(());
}

fn addr_code3(tokens: &Vec<IRTok>, idx: &mut usize) -> Option<()> {
    match next(tokens, idx)? {
    IRTok::Var(_) => {}
    _ => return None,
    }

    if !matches!(next(tokens, idx)?, IRTok::Comma) {
        return None;
    }
    
    match next(tokens, idx)? {
    IRTok::Var(_) => {}
    IRTok::Num(_) => {}
    _ => return None,
    }

    if !matches!(next(tokens, idx)?, IRTok::Comma) {
        return None;
    }

    match next(tokens, idx)? {
    IRTok::Var(_) => {}
    IRTok::Num(_) => {}
    _ => return None,
    }

    return Some(());
}

fn peek<'a>(tokens: &'a Vec<IRTok>, index: usize) -> Option<&'a IRTok> {
    if index < tokens.len() {
        return Some(&tokens[index])
    } else {
        return None
    }
}


fn next<'a>(tokens: &'a Vec<IRTok>, index: &mut usize) -> Option<&'a IRTok> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Some(&tokens[ret])
    } else {
        return None
    }
}

fn lex_ir_token(mut code: &str) -> (Option<IRTok>, &str) {
   
    #[derive(Debug)]
    enum StateMachine {
        Initial,
        Lit,
        Ident,
        Num,
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
        _ => None,

        }
    }

    fn num_literal(s: &str) -> Option<IRTok> {
        match s.parse::<i32>() {
        Ok(val) => Some(IRTok::Num(val)),
        Err(_) => None,
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
            '0'..='9' => StateMachine::Num,
            ';' => StateMachine::Comments,
            _ => StateMachine::Ident,
            }
        }

        StateMachine::Lit => {
            if c == ',' || c == '\n' || c == ';' {
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
            if c == ',' || c == '\n' || c == '[' || c == ']' || c == ';' {
                let tok = IRTok::Var(String::from(&code[..i]));
                return (Some(tok), &code[i..]);
            }

            if c.is_whitespace() {
                let tok = IRTok::Var(String::from(&code[..i]));
                return (Some(tok), &code[i+1..]);
            }

            StateMachine::Ident
        }

        StateMachine::Num => {
            if c >= '0' && c <= '9' {
                StateMachine::Num
            } else {
                let tok = num_literal(&code[..i]);
                return (tok, &code[i..]);
            }

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
    fn ir_token() {
        assert!(matches!(lex_ir_token("  %int"), (Some(IRTok::Int), _)));
        assert!(matches!(lex_ir_token(" %int[]"), (Some(IRTok::IntArray), _)));
        assert!(matches!(lex_ir_token("%call"), (Some(IRTok::Call), _)));
        assert!(matches!(lex_ir_token("%ret"), (Some(IRTok::Return), _)));
        assert!(matches!(lex_ir_token("%out"), (Some(IRTok::Out), _)));
        assert!(matches!(lex_ir_token("   %in"), (Some(IRTok::In), _)));
        assert!(matches!(lex_ir_token("%mov  "), (Some(IRTok::Mov), _)));
        assert!(matches!(lex_ir_token("%add"), (Some(IRTok::Add), _)));
        assert!(matches!(lex_ir_token("%sub"), (Some(IRTok::Sub), _)));
        assert!(matches!(lex_ir_token("%mult"), (Some(IRTok::Mult), _)));
        assert!(matches!(lex_ir_token("  %div"), (Some(IRTok::Div), _)));
        assert!(matches!(lex_ir_token("%mod"), (Some(IRTok::Mod), _)));
        assert!(matches!(lex_ir_token("%lt"), (Some(IRTok::LessThan), _)));
        assert!(matches!(lex_ir_token("%le"), (Some(IRTok::LessEqual), _)));
        assert!(matches!(lex_ir_token("%neq"), (Some(IRTok::NotEqual), _)));
        assert!(matches!(lex_ir_token("%eq"), (Some(IRTok::Equal), _)));
        assert!(matches!(lex_ir_token("%gt"), (Some(IRTok::GreaterThan), _)));
        assert!(matches!(lex_ir_token("%ge"), (Some(IRTok::GreaterEqual), _)));
        assert!(matches!(lex_ir_token("%jmp"), (Some(IRTok::Jump), _)));
        assert!(matches!(lex_ir_token("%branch_if"), (Some(IRTok::BranchIf), _)));
        assert!(matches!(lex_ir_token("%branch_if_not"), (Some(IRTok::BranchIfNot), _)));
        assert!(matches!(lex_ir_token("[are"), (Some(IRTok::LBrace), "are")));
        assert!(matches!(lex_ir_token("]are"), (Some(IRTok::RBrace), "are")));
        assert!(matches!(lex_ir_token(",are"), (Some(IRTok::Comma), "are")));
        assert!(matches!(lex_ir_token("%bad"), (None, _)));

        let code = "; This is a comment\n%mov";
        assert!(matches!(lex_ir_token(code), (Some(IRTok::EndInstr), "%mov")));
    }

    #[test]
    fn ir_lex() {
        {
            let toks = lex_ir("%add a, b, c\n");
            assert!(toks.len() == 7);
            assert!(matches!(toks[0], IRTok::Add));
            assert!(matches!(toks[1], IRTok::Var(_)));
            assert!(matches!(toks[2], IRTok::Comma));
            assert!(matches!(toks[3], IRTok::Var(_)));
            assert!(matches!(toks[4], IRTok::Comma));
            assert!(matches!(toks[5], IRTok::Var(_)));
            assert!(matches!(toks[6], IRTok::EndInstr));
        }

        {
            let toks = lex_ir("%func main,,,\n");
            assert!(toks.len() == 6);
            assert!(matches!(toks[0], IRTok::Func));
            assert!(matches!(toks[1], IRTok::Var(_)));
            assert!(matches!(toks[2], IRTok::Comma));
            assert!(matches!(toks[3], IRTok::Comma));
            assert!(matches!(toks[4], IRTok::Comma));
            assert!(matches!(toks[5], IRTok::EndInstr));
        }

        {
            let toks = lex_ir("%func,main,,,\n");
            assert!(toks.len() == 7);
            assert!(matches!(toks[0], IRTok::Func));
            assert!(matches!(toks[1], IRTok::Comma));
            assert!(matches!(toks[2], IRTok::Var(_)));
            assert!(matches!(toks[3], IRTok::Comma));
            assert!(matches!(toks[4], IRTok::Comma));
            assert!(matches!(toks[5], IRTok::Comma));
            assert!(matches!(toks[6], IRTok::EndInstr));
        }

        {
            let toks = lex_ir("%mov [arr, 0], 100\n");
            assert!(toks.len() == 9);
            assert!(matches!(toks[0], IRTok::Mov));
            assert!(matches!(toks[1], IRTok::LBrace));
            assert!(matches!(toks[2], IRTok::Var(_)));
            assert!(matches!(toks[3], IRTok::Comma));
            assert!(matches!(toks[4], IRTok::Num(0)));
            assert!(matches!(toks[5], IRTok::RBrace));
            assert!(matches!(toks[6], IRTok::Comma));
            assert!(matches!(toks[7], IRTok::Num(100)));
            assert!(matches!(toks[8], IRTok::EndInstr));
        }
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
    //Label,
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

