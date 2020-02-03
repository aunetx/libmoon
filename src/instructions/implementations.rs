use super::super::runtime::*;
use super::Instruction;

/// Implement instructions behavior for `Program` runtime
impl super::super::runtime::Program {
    /// Matches and inteprets given instruction
    ///
    /// Returns `Result<usize, Error>` representing the next line to interpret
    pub fn interpret(&self, line: &Instruction) -> Result<usize, Error> {
        match line {
            Instruction::Var { var, var_type } => Ok(0),
            e => Err(Error::UnimplementedInstruction((*e).clone(), self.lnb)),
        }
    }
}
