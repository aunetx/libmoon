use super::instructions::Instruction;
use super::ProgramFile;

pub struct Program {
    file: ProgramFile,
    pub lnb: usize,
}

impl Program {
    /// Constructs a new program from the given `ProgramFile`
    pub fn from(file: ProgramFile) -> Self {
        Self { file, lnb: 0 }
    }

    /// Runs the program
    pub fn run(&mut self) -> Result<usize, Error> {
        for (line_number, line) in self.file.lines.iter().enumerate() {
            self.lnb = line_number;
            // TODO remove `dbg!`
            match self.interpret(dbg!(line)) {
                Ok(l) => self.lnb = l,
                Err(e) => panic!("Error running line {} : {:?}", self.lnb, e),
            }
        }
        Ok(0)
    }
}

#[derive(Debug)]
pub enum Error {
    UnimplementedInstruction(Instruction, usize),
}
