#[derive(Debug)]
pub enum Instruction {
    Var { var: Var, var_type: Type },
    Set { var: Var, value: Val },
    Add { var: Var, value: Val },
    Sub { var: Var, value: Val },
    Mul { var: Var, value: Val },
    Div { var: Var, value: Val },
    Mod { var: Var, value: Val },
    Ret { var: Var },
    Flg,
    Gto { flag: Flag },
    Jmp { var: Var, flag: Flag },
    Jne { var: Var, flag: Flag },
    Nll,
    Prt { value: Val },
}

type Var = String;

/// Gives either a hardcoded value or a variable-stored one
#[derive(Debug)]
pub enum Val {
    Var(Var),
    Value(String),
}

type Flag = String;

#[derive(Debug)]
pub enum Type {
    Int,
    Flt,
    Chr,
}
