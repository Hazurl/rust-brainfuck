#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Move(i32),
    Add(i32),
    Write,
    Read,
    JumpZero(usize),
    JumpNonZero(usize),
    Clear,
    Copy(i32), // offset
}
