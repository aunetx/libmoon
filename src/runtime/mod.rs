use super::instructions::*;
use super::memory::*;
use super::ProgramFile;

pub struct Program {
    file: ProgramFile,
    pub lnb: usize,
    pub memory: Mem,
}

impl Program {
    /// Constructs a new program from the given `ProgramFile`
    pub fn from(file: ProgramFile) -> Self {
        Self {
            file,
            lnb: 0,
            memory: Mem::default(),
        }
    }

    /// Runs the program
    #[allow(clippy::cognitive_complexity)]
    pub fn run(&mut self) -> Result<usize, Error> {
        for (line_number, line) in self.file.lines.iter().enumerate() {
            self.lnb = line_number;
            // Instruction matcher
            match line {
                // ! ------- `VAR` -------------
                // `var` instruction
                Instruction::Var { var, var_type } => match var_type {
                    Type::Int => {
                        self.memory.insert(var.clone(), Cll::Int(None));
                    }
                    Type::Flt => {
                        self.memory.insert(var.clone(), Cll::Flt(None));
                    }
                    Type::Chr => {
                        self.memory.insert(var.clone(), Cll::Chr(None));
                    }
                },

                // ! ------- `SET` -------------
                // `set` instruction
                // TODO do NOT use `clone` on memory, find a way to do `self.memory.get(...)` while memory borrowed by `self.memory.get_mut(...)`
                Instruction::Set { var, value } => {
                    let old_mem = self.memory.clone();
                    match self.memory.get_mut(var) {
                        // If variable exists in memory
                        Some(cll) => match cll {
                            // If variable is an `int`
                            Cll::Int(cll_val) => match value {
                                // If `val` represents an hardcoded value
                                Val::Value(str_val) => {
                                    *cll_val = match str_val.parse() {
                                        Ok(l) => Some(l),
                                        Err(_) => {
                                            return Err(Error::CouldNotParseIntValue(
                                                str_val.to_string(),
                                            ))
                                        }
                                    };
                                }
                                // If `val` represents a memory-stored variable
                                Val::Var(name) => {
                                    let mem = old_mem.get(name);
                                    match mem {
                                        // If `val` exists and is an `int`
                                        Some(Cll::Int(val)) => *cll_val = *val,
                                        // If `val` exists but is not an `int`
                                        Some(_) => {
                                            return Err(Error::VariablesDifferInType(self.lnb))
                                        }
                                        // If `val` is not defined
                                        None => {
                                            return Err(Error::VariableIsUninitialized(
                                                name.to_string(),
                                                self.lnb,
                                            ))
                                        }
                                    }
                                }
                            },
                            // If variable is a `flt`
                            Cll::Flt(cll_val) => match value {
                                // If `val` represents an hardcoded value
                                Val::Value(str_val) => {
                                    *cll_val = match str_val.parse() {
                                        Ok(l) => Some(l),
                                        Err(_) => {
                                            return Err(Error::CouldNotParseFltValue(
                                                str_val.to_string(),
                                            ))
                                        }
                                    };
                                }
                                // If `val` represents a memory-stored variable
                                Val::Var(name) => {
                                    let mem = old_mem.get(name);
                                    match mem {
                                        // If `val` exists and is an `int`
                                        Some(Cll::Flt(val)) => *cll_val = *val,
                                        // If `val` exists but is not an `int`
                                        Some(_) => {
                                            return Err(Error::VariablesDifferInType(self.lnb))
                                        }
                                        // If `val` is not defined
                                        None => {
                                            return Err(Error::VariableIsUninitialized(
                                                name.to_string(),
                                                self.lnb,
                                            ))
                                        }
                                    }
                                }
                            }, // If variable is a `chr`
                            Cll::Chr(cll_val) => match value {
                                // If `val` represents an hardcoded value
                                Val::Value(str_val) => {
                                    *cll_val = match str_val.parse() {
                                        Ok(l) => Some(l),
                                        Err(_) => {
                                            return Err(Error::CouldNotParseChrValue(
                                                str_val.to_string(),
                                            ))
                                        }
                                    };
                                }
                                // If `val` represents a memory-stored variable
                                Val::Var(name) => {
                                    let mem = old_mem.get(name);
                                    match mem {
                                        // If `val` exists and is an `int`
                                        Some(Cll::Chr(val)) => *cll_val = *val,
                                        // If `val` exists but is not an `int`
                                        Some(_) => {
                                            return Err(Error::VariablesDifferInType(self.lnb))
                                        }
                                        // If `val` is not defined
                                        None => {
                                            return Err(Error::VariableIsUninitialized(
                                                name.to_string(),
                                                self.lnb,
                                            ))
                                        }
                                    }
                                }
                            },
                        },
                        // If variable does not exists in memory
                        None => return Err(Error::VariableDoesNotExists((*var).clone(), self.lnb)),
                    }
                }

                // ! ------- `ADD` -------------
                // `add` instruction
                Instruction::Add { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a + b });
                }

                // ! ------- `SUB` -------------
                // `sub` instruction
                Instruction::Sub { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a - b });
                }
                // ! ------- `MUL` -------------
                // `mul` instruction
                Instruction::Mul { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a * b });
                }

                // ! ------- `DIV` -------------
                // `div` instruction
                Instruction::Div { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a / b });
                }

                // ! ------- `MOD` -------------
                // `mod` instruction
                Instruction::Mod { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a % b });
                }

                // ! ------- `PRT` -------------
                // `prt` instruction
                Instruction::Prt { value } => match value {
                    Val::Value(val) => println!("l{} -> {} (value)", self.lnb, val),
                    Val::Var(name) => match self.memory.get(name) {
                        Some(val) => println!("l{} -> {}", self.lnb, val),
                        None => {
                            return Err(Error::VariableDoesNotExists(name.to_string(), self.lnb))
                        }
                    },
                },

                // ! ------- `NLL` -------------
                // `nll` instruction
                Instruction::Nll => (),

                // ! ------- `ERR` -------------
                // Instruction is not implemented yet
                e => return Err(Error::UnimplementedInstruction((*e).clone(), self.lnb)),
            }
        }
        Ok(0)
    }
}

#[derive(Debug)]
pub enum Error {
    UnimplementedInstruction(Instruction, usize),
    VariableDoesNotExists(String, usize),
    VariablesDifferInType(usize),
    VariableIsUninitialized(String, usize),
    CouldNotParseIntValue(String),
    CouldNotParseFltValue(String),
    CouldNotParseChrValue(String),
    CannotApplyOperationsOnChar(usize),
}
