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
                // TODO do NOT use `clone` on memory, find a way to do `self.memory.get(...)` while `self.memory.get_mut(...)`
                Instruction::Set { var, value } => {
                    let old_mem = self.memory.clone();
                    match self.memory.get_mut(var) {
                        // If variable exists in memory
                        Some(cll) => match cll {
                            // If variable is an `int`
                            Cll::Int(cll_val) => match value {
                                // If `val` represents an hardcoded value
                                Val::Value(str_val) => {
                                    *cll_val = get_int_value((*str_val).clone())?;
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
                                            return Err(Error::VariableIsUndefined(
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
                                    *cll_val = get_flt_value((*str_val).clone())?;
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
                                            return Err(Error::VariableIsUndefined(
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
                                    *cll_val = get_chr_value((*str_val).clone())?;
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
                                            return Err(Error::VariableIsUndefined(
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
    VariableIsUndefined(String, usize),
    CouldNotParseIntValue(String),
    CouldNotParseFltValue(String),
    CouldNotParseChrValue(String),
}
