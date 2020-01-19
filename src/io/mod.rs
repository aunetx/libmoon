use std::fs::File;
use std::io::Read;

/// A struct that contains the program, both under its text form and parsed form.
#[derive(Default)]
pub struct ProgramFile {
    text: String,
    lines: Vec<[String; 3]>,
}

impl ProgramFile {
    /// Return a new ProgramFile object
    pub fn new() -> Self {
        Self {
            text: String::new(),
            lines: Vec::new(),
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
            self.lines.push(self.parse_line(line, line_number)?);
        }
        Ok(())
    }

    fn parse_line(&self, line: &str, line_number: usize) -> Result<[String; 3], ErrorType> {
        // Remove chars that we don't need and replace unused lines with 'nll:nll'
        let line = line.replace(' ', "");
        // Split instruction / operands
        let splitted: Vec<&str> = line.split(':').collect();
        match splitted.len() {
            0 => Ok(["nll".to_owned(), "nll".to_owned(), "".to_owned()]),

            1 => Err(ErrorType::NotEnoughOperands(line_number)),

            2 => {
                let instruction = splitted[0].to_owned();
                let operands: Vec<&str> = splitted[1].split(',').collect();

                if instruction.is_empty() {
                    Err(ErrorType::NullInstruction(line_number))
                } else if operands[0].is_empty() {
                    Err(ErrorType::NullOperand(line_number, 1))
                } else if operands[1].is_empty() {
                    Err(ErrorType::NullOperand(line_number, 2))
                } else {
                    Ok([
                        splitted[0].to_owned(),
                        splitted[1].to_owned(),
                        "".to_owned(),
                    ])
                }
            }

            _ => Err(ErrorType::TooMuchInstructionSeparator(line_number)),
        }
    }
}

/// Contains types of IO errors
#[derive(Debug)]
pub enum ErrorType {
    CannotReadFile(&'static str),
    ErrorParsingLine(usize),
    NotEnoughOperands(usize),
    TooMuchInstructionSeparator(usize),
    NullInstruction(usize),
    NullOperand(usize, usize),
}
