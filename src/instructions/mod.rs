#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    /// Create an empty variable
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// var: hello, Int
    /// ```
    Var { var: Var, var_type: Type },
    /// Set variable to a value
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// set: hello, 10
    /// ```
    Set { var: Var, value: Val },
    /// Add value to variable
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// add: hello, 3
    /// ```
    Add { var: Var, value: Val },
    /// Substract value from variable
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// sub: hello, 5
    /// ```
    Sub { var: Var, value: Val },
    /// Multiply value with variable
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// mul: hello, 2
    /// ```
    Mul { var: Var, value: Val },
    /// Divides variable by value
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// div: hello, 3
    /// ```
    Div { var: Var, value: Val },
    /// Gives the rest of the division of variable by value
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// mod: hello, 3
    /// ```
    Mod { var: Var, value: Val },
    /// Create a flag to jump to
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// flg: flag
    /// ```
    Flg,
    /// Jump unconditionnaly to flag
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// gto: flag
    /// ```
    Gto { flag: Flag },
    /// Jump to flag if result is == 0
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// jmp: hello, flag
    /// ```
    Jmp { var: Var, flag: Flag },
    /// Jump to flag if result is != 0
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// jne: hello, flag
    /// ```
    Jne { var: Var, flag: Flag },
    /// Do nothing and is ignored
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// nll: empty
    /// ```
    Nll,
    /// Print value to screen
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// prt: hello
    /// ```
    Prt { value: Val },

    // carry operations :
    /// Add value to variable and store the result into `-`
    /// Carry alternative to `add`
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// cadd: hello, 5
    /// ```
    Cadd { op1: Val, op2: Val },
    /// Substract value from variable and store the result into `-`
    /// Carry alternative to `sub`
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// csub: hello, 5
    /// ```
    Csub { op1: Val, op2: Val },
    /// Multiply value with variable and store the result into `-`
    /// Carry alternative to `mul`
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// cmul: hello, 2
    /// ```
    Cmul { op1: Val, op2: Val },
    /// Divides variable by value and store the result into `-`
    /// Carry alternative to `div`
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// cdiv: hello, 3
    /// ```
    Cdiv { op1: Val, op2: Val },
    /// Gives the rest of the division of variable by value and store the result into `-`
    /// Carry alternative to `mod`
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// cmod: hello, 3
    /// ```
    Cmod { op1: Val, op2: Val },
}

/// Represents a variable
type Var = String;
/// Represents a flag
type Flag = String;

/// Gives either a hardcoded value or a variable-stored one
#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    /// Links to a variable
    Var(Var),
    /// Gives directly a value
    Value(String),
}

/// Defines a type : `int`, `flt` or `chr`
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    /// Integer type
    Int,
    /// Floating-number type
    Flt,
    /// Char type
    Chr,
}
