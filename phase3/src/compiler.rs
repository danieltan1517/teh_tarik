pub fn compile_and_run(code: &str) {
    println!("Generated code:");
    println!("{code}");
}

#[cfg(test)]
mod tests {

}

enum IR {

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
    GreaterThen,

    // labels/branching
    Label,
    Jump,
    BranchIf,
    BranchIfNot,
}
