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
    // TODO understand the clippy lint `block_in_if_condition_stmt` used for `compare_and_get_flag` macro
    #[allow(clippy::block_in_if_condition_stmt)]
    // TODO get around lint 'cannot borrow self.memory as mutable because it is also borrowed as immutable'
    #[allow(mutable_borrow_reservation_conflict)]
    #[allow(clippy::cognitive_complexity)]
    #[allow(unreachable_patterns)]
    pub fn run(&mut self) -> Result<usize, Error> {
        let mut line: &Instruction;
        while self.lnb < self.file.lines.len() {
            line = &self.file.lines[self.lnb];
            // Instruction matcher
            self.lnb = match line {
                // ! ------- `VAR` -------------
                // `var` instruction
                Instruction::Var { var, var_type } => {
                    match var_type {
                        Type::Int => {
                            self.memory.insert(var.clone(), Cll::Int(None));
                        }
                        Type::Flt => {
                            self.memory.insert(var.clone(), Cll::Flt(None));
                        }
                        Type::Chr => {
                            self.memory.insert(var.clone(), Cll::Chr(None));
                        }
                    };
                    self.lnb + 1
                }

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
                    };
                    self.lnb + 1
                }

                // ! ------- `ADD` -------------
                // `add` instruction
                Instruction::Add { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a + b });
                    self.lnb + 1
                }

                // ! ------- `SUB` -------------
                // `sub` instruction
                Instruction::Sub { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a - b });
                    self.lnb + 1
                }
                // ! ------- `MUL` -------------
                // `mul` instruction
                Instruction::Mul { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a * b });
                    self.lnb + 1
                }

                // ! ------- `DIV` -------------
                // `div` instruction
                Instruction::Div { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a / b });
                    self.lnb + 1
                }

                // ! ------- `MOD` -------------
                // `mod` instruction
                Instruction::Mod { var, value } => {
                    crate::get_and_change!(self, var, value, |a, b| { a % b });
                    self.lnb + 1
                }

                // ! ------- `CADD` -------------
                // `cadd` instruction
                Instruction::Cadd { op1, op2 } => {
                    crate::get_and_set_carry!(self, op1, op2, |a, b| { a + b });
                    self.lnb + 1
                }

                // ! ------- `CSUB` -------------
                // `csub` instruction
                Instruction::Csub { op1, op2 } => {
                    crate::get_and_set_carry!(self, op1, op2, |a, b| { a - b });
                    self.lnb + 1
                }

                // ! ------- `CMUL` -------------
                // `cmul` instruction
                Instruction::Cmul { op1, op2 } => {
                    crate::get_and_set_carry!(self, op1, op2, |a, b| { a * b });
                    self.lnb + 1
                }

                // ! ------- `CDIV` -------------
                // `cdiv` instruction
                Instruction::Cdiv { op1, op2 } => {
                    crate::get_and_set_carry!(self, op1, op2, |a, b| { a / b });
                    self.lnb + 1
                }

                // ! ------- `CMOD` -------------
                // `cmod` instruction
                Instruction::Cmod { op1, op2 } => {
                    crate::get_and_set_carry!(self, op1, op2, |a, b| { a % b });
                    self.lnb + 1
                }

                // ! ------- `PRT` -------------
                // `prt` instruction
                Instruction::Prt { value } => {
                    match value {
                        Val::Value(val) => println!("l°{:2} : value =   {}", self.lnb, val),
                        Val::Var(name) => match self.memory.get(name) {
                            Some(val) => println!("l°{:2} : {:5} =   {}", self.lnb, name, val),
                            None => {
                                return Err(Error::VariableDoesNotExists(
                                    name.to_string(),
                                    self.lnb,
                                ))
                            }
                        },
                    }
                    self.lnb + 1
                }

                // ! ------- `GTO` -------------
                // `gto` instruction
                Instruction::Gto { flag } => match self.file.flags.get(flag) {
                    Some(line) => *line,
                    None => return Err(Error::CouldNotFindFlag(flag.to_string(), self.lnb)),
                },

                // ! ------- `JMP` -------------
                // `jmp` instruction
                Instruction::Jmp { var, flag } => match self.memory.get(var) {
                    Some(cll) => match cll {
                        Cll::Int(Some(val)) => {
                            crate::compare_and_get_flag!(self, val, flag, |&a| { a == 0 })
                        }
                        Cll::Flt(Some(val)) => {
                            crate::compare_and_get_flag!(self, val, flag, |&a| { a == 0. })
                        }
                        Cll::Chr(_) => return Err(Error::CannotApplyComparisonsOnChar(self.lnb)),
                        _ => return Err(Error::VariableIsUninitialized(var.to_string(), self.lnb)),
                    },
                    None => return Err(Error::VariableDoesNotExists(var.to_string(), self.lnb)),
                },

                // ! ------- `JNE` -------------
                // `jne` instruction
                Instruction::Jne { var, flag } => match self.memory.get(var) {
                    Some(cll) => match cll {
                        Cll::Int(Some(val)) => {
                            crate::compare_and_get_flag!(self, val, flag, |&a| { a != 0 })
                        }
                        Cll::Flt(Some(val)) => {
                            crate::compare_and_get_flag!(self, val, flag, |&a| { a != 0. })
                        }
                        Cll::Chr(_) => return Err(Error::CannotApplyComparisonsOnChar(self.lnb)),
                        _ => return Err(Error::VariableIsUninitialized(var.to_string(), self.lnb)),
                    },
                    None => return Err(Error::VariableDoesNotExists(var.to_string(), self.lnb)),
                },

                // ! ------- `FLG` -------------
                // `flg` instruction
                Instruction::Flg => self.lnb + 1,

                // ! ------- `NLL` -------------
                // `nll` instruction
                Instruction::Nll => self.lnb + 1,

                // ! ------- `ERR` -------------
                // Instruction is not implemented yet
                e => return Err(Error::UnimplementedInstruction((*e).clone(), self.lnb)),
            }
        }
        Ok(self.lnb)
    }
}

/// Contains runtime type of errors
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
    CannotApplyComparisonsOnChar(usize),
    CouldNotFindFlag(String, usize),
    CannotDetermineReturnType(usize),
}
