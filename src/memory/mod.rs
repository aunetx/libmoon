use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Cll {
    Int(Option<i32>),
    Flt(Option<f64>),
    Chr(Option<char>),
}

impl fmt::Display for Cll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cll::Int(None) => write!(f, "unitialized (int)"),
            Cll::Flt(None) => write!(f, "unitialized (flt)"),
            Cll::Chr(None) => write!(f, "unitialized (chr)"),
            Cll::Int(Some(val)) => write!(f, "{} (int)", val),
            Cll::Flt(Some(val)) => write!(f, "{} (flt)", val),
            Cll::Chr(Some(val)) => write!(f, "{} (chr)", val),
        }
    }
}

pub type Mem = HashMap<String, Cll>;

// e.g. : get_and_change!(self, var, value, |a, b| { a + b }); to add var and value
#[macro_export]
macro_rules! get_and_change {
    ($prog:expr, $var_op:expr, $val_op:expr, $op:expr) => {
        use crate::instructions::*;
        use crate::memory::*;
        use crate::runtime::Error;

        // TODO do NOT use `clone` on memory, find a way to do `self.memory.get(...)` while memory borrowed by `self.memory.get_mut(...)`
        let old_mem = $prog.memory.clone();
        match $prog.memory.get_mut($var_op) {
            // ! If variable exists in memory
            Some(cll) => match cll {
                // ! If variable is an `int`
                Cll::Int(cll_val) => match $val_op {
                    // If `val` represents an hardcoded value
                    Val::Value(str_val) => {
                        let b: i32 = match str_val.parse() {
                            // If `val` is successfully parsed, add `cll_val`'s value and `val`
                            Ok(l) => l,
                            // If `val` is not successfully parsed
                            Err(_) => return Err(Error::CouldNotParseIntValue((*str_val).clone())),
                        };
                        let a: i32 = match cll_val {
                            Some(a) => *a,
                            None => {
                                return Err(Error::VariableIsUninitialized($var_op.to_string(), 0))
                            }
                        };
                        // Set result into `cll_val`'s value
                        *cll_val = Some($op(a, b))
                    }
                    // If `val` represents a memory-stored variable
                    Val::Var(name) => {
                        match old_mem.get(name) {
                            // If `val` exists and is an `int`
                            Some(Cll::Int(Some(b))) => {
                                let a: i32 = match cll_val {
                                    Some(a) => *a,
                                    None => {
                                        return Err(Error::VariableIsUninitialized(
                                            $var_op.to_string(),
                                            0,
                                        ))
                                    }
                                };
                                *cll_val = Some($op(a, b))
                            }
                            // If `val` exists but is uninitialized
                            Some(Cll::Int(None)) => {
                                return Err(Error::VariableIsUninitialized(
                                    name.to_string(),
                                    $prog.lnb,
                                ))
                            }
                            // If `val` exists but is not an `int`
                            Some(_) => return Err(Error::VariablesDifferInType($prog.lnb)),
                            // If `val` does not exists
                            None => {
                                return Err(Error::VariableDoesNotExists(
                                    name.to_string(),
                                    $prog.lnb,
                                ))
                            }
                        }
                    }
                },
                // ! If variable is a `flt`
                Cll::Flt(cll_val) => match $val_op {
                    // If `val` represents an hardcoded value
                    Val::Value(str_val) => {
                        let b: f64 = match str_val.parse() {
                            // If `val` is successfully parsed, add `cll_val`'s value and `val`
                            Ok(l) => l,
                            // If `val` is not successfully parsed
                            Err(_) => return Err(Error::CouldNotParseIntValue((*str_val).clone())),
                        };
                        let a: f64 = match cll_val {
                            Some(a) => *a,
                            None => {
                                return Err(Error::VariableIsUninitialized($var_op.to_string(), 0))
                            }
                        };
                        // Set result into `cll_val`'s value
                        *cll_val = Some($op(a, b))
                    }
                    // If `val` represents a memory-stored variable
                    Val::Var(name) => {
                        match old_mem.get(name) {
                            // If `val` exists and is a `flt`
                            Some(Cll::Flt(Some(b))) => {
                                let a: f64 = match cll_val {
                                    Some(a) => *a,
                                    None => {
                                        return Err(Error::VariableIsUninitialized(
                                            $var_op.to_string(),
                                            0,
                                        ))
                                    }
                                };
                                *cll_val = Some($op(a, b))
                            }
                            // If `val` exists but is uninitialized
                            Some(Cll::Flt(None)) => {
                                return Err(Error::VariableIsUninitialized(
                                    name.to_string(),
                                    $prog.lnb,
                                ))
                            }
                            // If `val` exists but is not a `flt`
                            Some(_) => return Err(Error::VariablesDifferInType($prog.lnb)),
                            // If `val` does not exists
                            None => {
                                return Err(Error::VariableDoesNotExists(
                                    name.to_string(),
                                    $prog.lnb,
                                ))
                            }
                        }
                    }
                },
                // ! If variable is a `chr`
                Cll::Chr(_) => return Err(Error::CannotApplyOperationsOnChar($prog.lnb)),
            },
            // ! If variable does not exists in memory
            None => return Err(Error::VariableDoesNotExists((*$var_op).clone(), $prog.lnb)),
        }
    };
}
