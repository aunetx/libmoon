use super::runtime::Error;

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
    /// Set variable to last result
    ///
    /// Utilisation :
    ///
    /// ```Moon
    /// res: hello
    /// ```
    Ret { var: Var },
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

/// Returns the `i32` value of the given `String`
pub fn get_int_value(val: String) -> Result<Option<i32>, Error> {
    match val.trim().parse() {
        Ok(l) => Ok(Some(l)),
        Err(_) => Err(Error::CouldNotParseIntValue(val)),
    }
}
/// Returns the `f64` value of the given `String`
pub fn get_flt_value(val: String) -> Result<Option<f64>, Error> {
    match val.trim().parse() {
        Ok(l) => Ok(Some(l)),
        Err(_) => Err(Error::CouldNotParseFltValue(val)),
    }
}
/// Returns the `char` value of the given `String`
pub fn get_chr_value(val: String) -> Result<Option<char>, Error> {
    match val.trim().parse() {
        Ok(l) => Ok(Some(l)),
        Err(_) => Err(Error::CouldNotParseChrValue(val)),
    }
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
