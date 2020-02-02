use super::instructions::*;
use std::fs::File;
use std::io::Read;

/// A struct that contains the program, both under its text form and parsed form.
#[derive(Default)]
pub struct ProgramFile {
    text: String,
    pub lines: Vec<Instruction>,
    line_number: usize,
}

impl ProgramFile {
    /// Return a new ProgramFile object
    pub fn new() -> Self {
        Self {
            text: String::new(),
            lines: Vec::new(),
            line_number: 0,
        }
    }

    /// Reads a given program file.
    pub fn open(&mut self, file_name: &'static str) -> Result<(), ErrorType> {
        match File::open(file_name) {
            Ok(mut f) => {
                f.read_to_string(&mut self.text).unwrap();
                Ok(())
            }
            Err(_) => Err(ErrorType::CannotReadFile(file_name)),
        }
    }

    /// Parse the program.
    pub fn parse(&mut self) -> Result<(), ErrorType> {
        let program: Vec<&str> = self.text.lines().collect();
        for (line_number, line) in program.iter().enumerate() {
            self.line_number = line_number;
            // TODO remove `dbg!`
            self.lines.push(dbg!(self.parse_line(line)?));
        }
        Ok(())
    }

    fn parse_line(&self, line: &str) -> Result<Instruction, ErrorType> {
        // Remove whitespaces
        let line: String = line.split_whitespace().collect();
        // Split instruction / operands
        let splitted: Vec<&str> = line.split(':').collect();
        match splitted.len() {
            0 => Ok(Instruction::Nll),
            1 => Err(ErrorType::NotEnoughInstructionSeparator(self.line_number)),
            2 => {
                let instruction = splitted[0];
                let operands: Vec<&str> = splitted[1].split(',').collect();

                if instruction.is_empty() {
                    Err(ErrorType::EmptyInstruction(self.line_number))
                } else if operands.is_empty() {
                    Err(ErrorType::NotEnoughOperands(self.line_number))
                } else if operands.len() > 2 {
                    Err(ErrorType::TooMuchOperands(self.line_number))
                } else if operands[0].is_empty() {
                    Err(ErrorType::EmptyOperand(self.line_number, 1))
                } else if operands.len() > 1 && operands[1].is_empty() {
                    Err(ErrorType::EmptyOperand(self.line_number, 2))
                } else {
                    match self.match_instruction(instruction, operands) {
                        Ok(i) => Ok(i),
                        Err(e) => Err(e),
                    }
                }
            }
            _ => Err(ErrorType::TooMuchInstructionSeparator(self.line_number)),
        }
    }

    fn match_instruction(
        &self,
        text_instruction: &str,
        operands: Vec<&str>,
    ) -> Result<Instruction, ErrorType> {
        let op0 = operands[0].to_owned();
        match text_instruction {
            "var" | "set" | "add" | "sub" | "mul" | "div" | "rst" | "jmp" | "jne" => {
                if operands.len() < 2 {
                    return Err(ErrorType::NotEnoughOperands(self.line_number));
                }
            }
            "ret" | "gto" | "flg" | "prt" => {
                if operands.len() > 1 {
                    return Err(ErrorType::TooMuchOperands(self.line_number));
                }
            }
            _ => (),
        };
        match text_instruction {
            "var" => Ok(Instruction::Var {
                var: op0,
                var_type: self.match_type(operands[1])?,
            }),
            "set" => Ok(Instruction::Set {
                var: op0,
                value: self.match_var_or_value(operands[1])?,
            }),
            "add" => Ok(Instruction::Add {
                var: op0,
                value: self.match_var_or_value(operands[1])?,
            }),
            "sub" => Ok(Instruction::Sub {
                var: op0,
                value: self.match_var_or_value(operands[1])?,
            }),
            "mul" => Ok(Instruction::Mul {
                var: op0,
                value: self.match_var_or_value(operands[1])?,
            }),
            "div" => Ok(Instruction::Div {
                var: op0,
                value: self.match_var_or_value(operands[1])?,
            }),
            "mod" => Ok(Instruction::Mod {
                var: op0,
                value: self.match_var_or_value(operands[1])?,
            }),
            "ret" => Ok(Instruction::Ret { var: op0 }),
            "flg" => Ok(Instruction::Flg),
            "gto" => Ok(Instruction::Gto { flag: op0 }),
            "jmp" => Ok(Instruction::Jmp {
                var: op0,
                flag: operands[1].to_owned(),
            }),
            "jne" => Ok(Instruction::Jne {
                var: op0,
                flag: operands[1].to_owned(),
            }),
            "nll" => Ok(Instruction::Nll),
            "prt" => Ok(Instruction::Prt {
                value: self.match_var_or_value(&op0)?,
            }),
            _ => Err(ErrorType::UnknownInstruction(
                text_instruction.to_owned(),
                self.line_number,
            )),
        }
    }

    fn match_type(&self, input: &str) -> Result<Type, ErrorType> {
        match input {
            "int" => Ok(Type::Int),
            "flt" => Ok(Type::Flt),
            "chr" => Ok(Type::Chr),
            e => Err(ErrorType::UnknownType(e.to_owned(), self.line_number)),
        }
    }

    fn match_var_or_value(&self, input: &str) -> Result<Val, ErrorType> {
        match input.get(0..1) {
            Some("&") => Ok(Val::Var(input.to_owned())),
            Some(_) => Ok(Val::Value(input.to_owned())),
            None => Err(ErrorType::EmptyValue(self.line_number)),
        }
    }
}

/// Contains types of IO errors
#[derive(Debug)]
pub enum ErrorType {
    CannotReadFile(&'static str),
    ErrorParsingLine(usize),
    NotEnoughOperands(usize),
    TooMuchOperands(usize),
    NotEnoughInstructionSeparator(usize),
    TooMuchInstructionSeparator(usize),
    EmptyInstruction(usize),
    EmptyOperand(usize, usize),
    UnknownInstruction(String, usize),
    UnknownType(String, usize),
    EmptyValue(usize),
}
