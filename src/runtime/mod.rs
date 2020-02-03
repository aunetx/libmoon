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

                // `set` instruction
                Instruction::Set { var, value } => match self.memory.get_mut(var) {
                    // If variable exists in memory
                    Some(cll) => match cll {
                        // If variable is an `int`
                        Cll::Int(cll_val) => match value {
                            // If `val` represents an hardcoded value
                            Val::Value(str_val) => {
                                *cll_val = get_int_value((*str_val).clone())?;
                            }
                            // If `val` represents a memory-stored variable
                            Val::Var(name) => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    // If variable does not exists in memory
                    None => return Err(Error::VariableDoesNotExists((*value).clone(), self.lnb)),
                },

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
    VariableDoesNotExists(Val, usize),
    VariablesDifferInType(usize),
    CouldNotParseIntValue(String),
}
