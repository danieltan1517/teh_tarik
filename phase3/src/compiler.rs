pub fn compile_and_run(code: &str) {
    println!("Generated code:");
    println!("{code}");
    let tokens = lex_ir(code);
    match parse_ir(&tokens, &mut 0) {
    Ok(_) => {
        println!("Good.");
    }

    Err(e) => {
        println!("Error.{e}");
    }

    }

    // todo: pass over everything. assign labels to the function.
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

fn parse_ir(tokens: &Vec<IRTok>, idx: &mut usize) -> Result<(), Box<dyn Error>> {
    let mut function_bytecode = FunctionBytecode {
        name:String::from(""),
        id:0,
        variables: HashMap::new(),
    };

    if !matches!(next_result(tokens, idx)?, IRTok::Func) {
        return Err(Box::from("func IR needs to begin with '%func'"));
    }

    match next_result(tokens, idx)? {
    IRTok::Var(func_ident) => {
        function_bytecode.name = func_ident.clone();
    }

    _ => return Err(Box::from("func IR must have an identifier name such as '%func main'")),

    };

    if !matches!(next_result(tokens, idx)?, IRTok::EndInstr) {
        return Err(Box::from("%func header must end with a newline"));
    }

    loop {
        let bytecode = parse_instruction(&mut function_bytecode, tokens, idx)?;
        println!("{:?}", bytecode);
        if matches!(bytecode, Bytecode::End) {
            break;
        }
    }

    if !matches!(next_result(tokens, idx)?, IRTok::EndFunc) {
        return Err(Box::from("func IR needs to end in '%endfunc'"));
    }

    return Ok(());
}

use std::collections::HashMap;
use std::error::Error;

struct FunctionBytecode {
    name: String,
    id: i32,
    variables: HashMap<String, VariableType>,
}

fn get_id(function: &mut FunctionBytecode) -> i32 {
    let id = function.id;
    function.id += 1;
    id
}

fn lookup_integer_variable_id(function: &FunctionBytecode, ident: &String) -> Result<Op, Box<dyn Error>> {
    if let Some(id) = function.variables.get(ident) {
         match id {
         VariableType::IntVar(id) => Ok(Op::Var(*id)),

         VariableType::ArrayVar(_,_) => {
             let f = format!("invalid operand. {} is an array, not an integer.", ident);
             return Err(Box::from(f));
         }

         }
    } else {
         let f = format!("invalid instruction. identifier '{}' has not been declared.", ident);
         return Err(Box::from(f));
    }
}

fn parse_instruction(function: &mut FunctionBytecode, tokens: &Vec<IRTok>, idx: &mut usize) -> Result<Bytecode, Box<dyn Error>> {
    let bytecode: Bytecode;
    let opcode = peek_result(tokens, *idx)?;
    match opcode {

    // declarations.
    IRTok::Int => {
        *idx += 1;
        let ident = match next_result(tokens, idx)? {
        IRTok::Var(ident) => ident,
        _ => return Err(Box::from("invalid instruction. expected identifier like '%int variable'")),
        };

        if let Some(_) = function.variables.get(ident) {
             let f = format!("invalid instruction. identifier '{}' declared too many times", ident);
             return Err(Box::from(f));
        } else {
             let id = get_id(function);
             function.variables.insert(ident.clone(), VariableType::IntVar(id));
             bytecode = Bytecode::Int(Op::Var(id));
        }
    }

    IRTok::IntArray => {
        *idx += 1;
        let ident = match next_result(tokens, idx)? {
        IRTok::Var(ident) => ident,
        _ => return Err(Box::from("invalid instruction. expected format like '%int[] array, 10'")),
        };

        if !matches!(next_result(tokens, idx)?, IRTok::Comma) {
            return Err(Box::from("invalid instruction. expected format like '%int[] array, 10'"));
        }

        let num = match next_result(tokens, idx)? {
        IRTok::Num(num) => *num,
        _ => return Err(Box::from("invalid instruction. expected format like '%int[] array, 10'")),
        };

        if let Some(_) = function.variables.get(ident) {
             let f = format!("invalid instruction. identifier '{}' declared too many times", ident);
             return Err(Box::from(f));
        } else {
             let id = get_id(function);
             function.variables.insert(ident.clone(), VariableType::ArrayVar(id, num));
             bytecode = Bytecode::IntArray(Op::Var(id), Op::Num(num));
        }
    }

    // function calling routines.
    IRTok::Call => {
        *idx += 1;
        todo!()
    }

    IRTok::Return => {
        *idx += 1;
        let op = match next_result(tokens, idx)? {
        IRTok::Var(ident) => lookup_integer_variable_id(function, ident)?,
        IRTok::Num(num) => Op::Num(*num),
        _ => return Err(Box::from("invalid instruction. expected format like '%ret variable'")),
        };
        bytecode = Bytecode::Return(op);
    }

    // input/output routines.
    IRTok::Out => {
        *idx += 1;
        let src = match next_result(tokens, idx)? {
        IRTok::Var(ident) => lookup_integer_variable_id(function, ident)?,
        IRTok::Num(num) => Op::Num(*num),
        _ => return Err(Box::from("invalid instruction. expected format like '%out variable'")),
        };
        bytecode = Bytecode::Out(src);
    }

    IRTok::In => {
        *idx += 1;
        let src = match next_result(tokens, idx)? {
        IRTok::Var(ident) => lookup_integer_variable_id(function, ident)?,
        _ => return Err(Box::from("invalid instruction. expected format like '%in variable'")),
        };
        bytecode = Bytecode::In(src);
    }

    // mathematical operators.
    IRTok::Mov => {
        *idx += 1;
        let dest = match next_result(tokens, idx)? {
        IRTok::Var(ident) => lookup_integer_variable_id(function, ident)?,
        _ => return Err(Box::from("invalid instruction. expected format like '%mov variable, 10'")),
        };
 
        if !matches!(next_result(tokens, idx)?, IRTok::Comma) {
            return Err(Box::from("invalid instruction. missing comma. expected format like '%mov variable, 10'"));
        }
        
        let src = match next_result(tokens, idx)? {
        IRTok::Var(ident) => {
            if let Some(id) = function.variables.get(ident) {
                 match id {
                 VariableType::IntVar(id) => Op::Var(*id),

                 VariableType::ArrayVar(_,_) => {
                     let f = format!("invalid '%mov' statement. {} is an array, not an integer.", ident);
                     return Err(Box::from(f));
                 }

                 }
            } else {
                 let f = format!("invalid instruction. identifier '{}' declared too many times", ident);
                 return Err(Box::from(f));
            }
        }
        IRTok::Num(num) => Op::Num(*num),
        _ => return Err(Box::from("invalid instruction. expected format like '%mov variable, 10'")),
        };
        bytecode = Bytecode::Mov(dest, src);
    }

    IRTok::Add => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::Add(dest, src1, src2);
    }

    IRTok::Sub => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::Sub(dest, src1, src2);
    }

    IRTok::Mult => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::Mult(dest, src1, src2);
    }

    IRTok::Div => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::Div(dest, src1, src2);
    }

    IRTok::Mod => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::Mod(dest, src1, src2);
    }

    // comparison operators.
    IRTok::LessThan => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::LessThan(dest, src1, src2);
    }

    IRTok::LessEqual => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::LessEqual(dest, src1, src2);
    }

    IRTok::NotEqual => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::NotEqual(dest, src1, src2);
    }

    IRTok::Equal => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::Equal(dest, src1, src2);
    }

    IRTok::GreaterEqual => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::GreaterEqual(dest, src1, src2);
    }

    IRTok::GreaterThan => {
        *idx += 1;
        let (dest, src1, src2) = addr_code3(function, tokens, idx)?;
        bytecode = Bytecode::GreaterThan(dest, src1, src2);
    }

    // labels/branching
    //Label,
    IRTok::Jump => {
        *idx += 1;
        todo!();
    }

    IRTok::BranchIf => {
        *idx += 1;
        todo!();
    }

    IRTok::BranchIfNot => {
        *idx += 1;
        todo!();
    }

    IRTok::EndFunc => {
        bytecode = Bytecode::End;
        return Ok(bytecode);
    }

    _ => {
        return Err(Box::from("invalid instruction."));
    }

    }

    if !matches!(next_result(tokens, idx)?, IRTok::EndInstr) {
        return Err(Box::from("expected newline."));
    }

    return Ok(bytecode);
}

fn addr_code3(function: &FunctionBytecode, tokens: &Vec<IRTok>, idx: &mut usize) -> Result<(Op, Op, Op), Box<dyn Error>> {
    let dest = match next_result(tokens, idx)? {
    IRTok::Var(ident) => lookup_integer_variable_id(function, ident)?,
    _ => return Err(Box::from("invalid dest.")),
    };

    if !matches!(next_result(tokens, idx)?, IRTok::Comma) {
        return Err(Box::from("invalid 3-address code. expected comma."));
    }
    
    let src1 = match next_result(tokens, idx)? {
    IRTok::Var(ident) => lookup_integer_variable_id(function, ident)?,
    IRTok::Num(num) => Op::Num(*num),
    _ => return Err(Box::from("invalid src1.")),
    };

    if !matches!(next_result(tokens, idx)?, IRTok::Comma) {
        return Err(Box::from("invalid 3-address code. expected comma."));
    }

    let src2 = match next_result(tokens, idx)? {
    IRTok::Var(ident) => lookup_integer_variable_id(function, ident)?,
    IRTok::Num(num) => Op::Num(*num),
    _ => return Err(Box::from("invalid src2.")),
    };

    return Ok((dest, src1, src2));
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

fn peek_result<'a>(tokens: &'a Vec<IRTok>, index: usize) -> Result<&'a IRTok, Box<dyn Error>> {
    if index < tokens.len() {
        return Ok(&tokens[index])
    } else {
        return Err(Box::from("unexpected end."))
    }
}

fn next_result<'a>(tokens: &'a Vec<IRTok>, index: &mut usize) -> Result<&'a IRTok, Box<dyn Error>> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Ok(&tokens[ret])
    } else {
        return Err(Box::from("unexpected end."))
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

#[derive(Debug)]
enum Op {
    Num(i32),
    Var(i32),
}

#[derive(Debug)]
enum VariableType {
    IntVar(i32),
    ArrayVar(i32, i32),
}

#[derive(Debug)]
enum Bytecode {

    // EndFunc
    End,

    // declarations.
    Int(Op),
    IntArray(Op, Op),

    // input/output routines.
    Out(Op),
    In(Op),

    // mathematical operators.
    Mov(Op, Op),
    Add(Op, Op, Op),
    Sub(Op, Op, Op),
    Mult(Op, Op, Op),
    Div(Op, Op, Op),
    Mod(Op, Op, Op),

    // comparison operators.
    LessThan(Op, Op, Op),
    LessEqual(Op, Op, Op),
    NotEqual(Op, Op, Op),
    Equal(Op, Op, Op),
    GreaterEqual(Op, Op, Op),
    GreaterThan(Op, Op, Op),
    Return(Op),
}


