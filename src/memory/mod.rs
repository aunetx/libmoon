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
            Cll::Int(Some(val)) => write!(f, "{:<5} _int", val),
            Cll::Flt(Some(val)) => write!(f, "{:<5} _flt", val),
            Cll::Chr(Some(val)) => write!(f, "{:<5} _chr", val),
            Cll::Int(None) => write!(f, "unitialized _int"),
            Cll::Flt(None) => write!(f, "unitialized _flt"),
            Cll::Chr(None) => write!(f, "unitialized _chr"),
        }
    }
}

pub type Mem = HashMap<String, Cll>;

/// Performs the given operation on two variables, and set the result into the first one
/// We do not merge it with the very similar `get_and_set_carry` macro because we want to open only one time `op1`
// e.g. : get_and_change!(self, var, value, |a, b| { a + b }); to add var and value
#[macro_export]
macro_rules! get_and_change {
    ($prog:expr, $var_op:expr, $val_op:expr, $op:expr) => {
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
                            Err(_) => return Err(Error::CouldNotParseFltValue((*str_val).clone())),
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

/// Similar to `get_and_change`, but set result into `carry` variable : `-`
#[macro_export]
macro_rules! get_and_set_carry {
    ($prog:expr, $op1:expr, $op2:expr, $op:expr) => {
        // Match `op1` as a variable or a value
        match $op1 {
            // op1 is a variable
            Val::Var(var_1) => match $prog.memory.get(var_1) {
                // op1 exists in memory
                Some(var_1_cll) => match var_1_cll {
                    // op1 is uninitialized : error
                    Cll::Int(None) | Cll::Flt(None) => {
                        return Err(Error::VariableIsUninitialized(var_1.to_string(), $prog.lnb))
                    }
                    // op1 is an `int`
                    Cll::Int(Some(var_1_value)) => match $op2 {
                        // op2 is a variable
                        Val::Var(var_2) => match $prog.memory.get(var_2) {
                            // op2 exists in memory
                            Some(var_2_cll) => match var_2_cll {
                                // op2 is uninitialized : error
                                Cll::Int(None) => {
                                    return Err(Error::VariableIsUninitialized(
                                        var_2.to_string(),
                                        $prog.lnb,
                                    ))
                                }
                                // op2 is an `int` too
                                Cll::Int(Some(var_2_value)) => {
                                    $prog.memory.insert(
                                        "-".to_owned(),
                                        Cll::Int(Some($op(var_1_value, var_2_value))),
                                    );
                                }
                                // op2 is not an `int` : error
                                _ => return Err(Error::VariablesDifferInType($prog.lnb)),
                            },
                            // op2 does not exists in memory : error
                            None => {
                                return Err(Error::VariableDoesNotExists(
                                    var_1.to_string(),
                                    $prog.lnb,
                                ))
                            }
                        },
                        // op2 is a value
                        Val::Value(val_2) => match val_2.parse::<i32>() {
                            // if op2 could be parsed as `int`
                            Ok(val_2_value) => {
                                $prog.memory.insert(
                                    "-".to_owned(),
                                    Cll::Int(Some($op(var_1_value, val_2_value))),
                                );
                            }
                            // if op2 could not be parsed : error
                            Err(_) => return Err(Error::CannotDetermineReturnType($prog.lnb)),
                        },
                    },
                    // op1 is a `flt`
                    Cll::Flt(Some(var_1_value)) => match $op2 {
                        // op2 is a variable
                        Val::Var(var_2) => match $prog.memory.get(var_2) {
                            // op2 exists in memory
                            Some(var_2_cll) => match var_2_cll {
                                // op2 is uninitialized : error
                                Cll::Flt(None) => {
                                    return Err(Error::VariableIsUninitialized(
                                        var_2.to_string(),
                                        $prog.lnb,
                                    ))
                                }
                                // op2 is a `flt` too
                                Cll::Flt(Some(var_2_value)) => {
                                    $prog.memory.insert(
                                        "-".to_owned(),
                                        Cll::Flt(Some($op(var_1_value, var_2_value))),
                                    );
                                }
                                // op2 is not a `flt` : error
                                _ => return Err(Error::VariablesDifferInType($prog.lnb)),
                            },
                            // op2 does not exists in memory : error
                            None => {
                                return Err(Error::VariableDoesNotExists(
                                    var_1.to_string(),
                                    $prog.lnb,
                                ))
                            }
                        },
                        // op2 is a value
                        Val::Value(val_2) => match val_2.parse::<f64>() {
                            // if op2 could be parsed as `flt`
                            Ok(val_2_value) => {
                                $prog.memory.insert(
                                    "-".to_owned(),
                                    Cll::Flt(Some($op(var_1_value, val_2_value))),
                                );
                            }
                            // if op2 could not be parsed : error
                            Err(_) => return Err(Error::CannotDetermineReturnType($prog.lnb)),
                        },
                    },
                    // op1 is a `chr` : error
                    Cll::Chr(_) => return Err(Error::CannotApplyOperationsOnChar($prog.lnb)),
                },
                // op1 does not exist in memory : error
                None => return Err(Error::VariableDoesNotExists(var_1.to_string(), $prog.lnb)),
            },
            // op1 is a value
            Val::Value(val_1) => match $op2 {
                // op2 is a value : error
                Val::Value(_) => return Err(Error::CannotDetermineReturnType($prog.lnb)),
                // op2 is a variable
                Val::Var(val_2) => match $prog.memory.get(val_2) {
                    // op2 exists in memory
                    Some(val_2_cll) => match val_2_cll {
                        // op2 is uninitialized
                        Cll::Int(None) | Cll::Flt(None) => {
                            return Err(Error::VariableIsUninitialized(
                                val_2.to_string(),
                                $prog.lnb,
                            ))
                        }
                        // op2 is an `int`
                        Cll::Int(Some(val_2_value)) => match val_1.parse::<i32>() {
                            // if op1 could be parsed as `int`
                            Ok(val_1_value) => {
                                $prog.memory.insert(
                                    "-".to_owned(),
                                    Cll::Int(Some($op(val_1_value, val_2_value))),
                                );
                            }
                            // if op1 could not be parsed : error
                            Err(_) => return Err(Error::CannotDetermineReturnType($prog.lnb)),
                        },
                        // op2 is a `flt`
                        Cll::Flt(Some(val_2_value)) => match val_1.parse::<f64>() {
                            // if op1 could be parsed as `flt`
                            Ok(val_1_value) => {
                                $prog.memory.insert(
                                    "-".to_owned(),
                                    Cll::Flt(Some($op(val_1_value, val_2_value))),
                                );
                            }
                            // if op1 could not be parsed : error
                            Err(_) => return Err(Error::CannotDetermineReturnType($prog.lnb)),
                        },
                        // op2 is a `chr` : error
                        Cll::Chr(_) => return Err(Error::CannotApplyOperationsOnChar($prog.lnb)),
                    },
                    // op2 does not exist in memory : error
                    _ => unimplemented!(),
                },
            },
        }
    };
}

#[macro_export]
macro_rules! compare_and_get_flag {
    ($prog:expr, $val:expr, $flag:expr, $op:expr) => {
        if $op($val) {
            match $prog.file.flags.get($flag) {
                Some(line) => *line,
                None => return Err(Error::CouldNotFindFlag($flag.to_string(), $prog.lnb)),
            }
        } else {
            $prog.lnb + 1
        }
    };
}
