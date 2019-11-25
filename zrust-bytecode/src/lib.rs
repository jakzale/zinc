mod decode;
pub mod instructions;
pub mod vlq;

pub use decode::*;
pub use instructions::*;

use std::fmt::Debug;

pub trait InstructionInfo: PartialEq + Debug + Sized {
    fn to_assembly(&self) -> String;
    fn code() -> InstructionCode;
    fn encode(&self) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError>;
    fn inputs_count(&self) -> usize;
    fn outputs_count(&self) -> usize;
}

#[derive(Debug,PartialEq)]
pub enum DecodingError {
    UnexpectedEOF,
    UnknownInstructionCode(u8),
    ConstantTooLong,
}

#[derive(Debug)]
pub enum InstructionCode {
    NoOperation,

    // Stack
    Push,
    Pop,
    Copy,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,

    // Boolean
    Not,
    And,
    Or,
    Xor,

    // Comparison
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,

    Cast,

    // Flow control
    ConditionalSelect,
    FrameBegin,
    FrameEnd,
    LoopBegin,
    LoopEnd,
    Call,
    Return,

    // Condition utils
    Assert,
    PushCondition,
    PopCondition,

    Exit,
}

#[derive(Debug,PartialEq)]
pub enum Instruction {
    NoOperation(NoOperation),

    // Stack
    Push(Push),
    Pop(Pop),
    Copy(Copy),

    // Arithmetic
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Rem(Rem),
    Neg(Neg),

    // Boolean
    Not(Not),
    And(And),
    Or(Or),
    Xor(Xor),

    // Comparison
    Lt(Lt),
    Le(Le),
    Eq(Eq),
    Ne(Ne),
    Ge(Ge),
    Gt(Gt),

    Cast(Cast),

    // Flow control
    ConditionalSelect(ConditionalSelect),
    FrameBegin(FrameBegin),
    FrameEnd(FrameEnd),
    LoopBegin(LoopBegin),
    LoopEnd(LoopEnd),
    Call(Call),
    Return(Return),

    // Condition utils
    Assert(Assert),
    PushCondition(PushCondition),
    PopCondition(PopCondition),

    Exit(Exit),
}

/// Useful macro to avoid duplicating `match` constructions.
///
/// ```
/// # use zrust_bytecode::Instruction;
/// # use zrust_bytecode::instructions::Add;
/// let i = Instruction::Add(Add);
/// let opcode = dispatch_instruction!(i => i.assemly());
/// assert_eq!(opcode, "add");
/// ```
#[macro_export]
macro_rules! dispatch_instruction {
    ($pattern:ident => $expression:expr) => {
        match $pattern {
            Instruction::NoOperation($pattern) => $expression,

            Instruction::Push($pattern) => $expression,
            Instruction::Pop($pattern) => $expression,
            Instruction::Copy($pattern) => $expression,

            Instruction::Add($pattern) => $expression,
            Instruction::Sub($pattern) => $expression,
            Instruction::Mul($pattern) => $expression,
            Instruction::Div($pattern) => $expression,
            Instruction::Rem($pattern) => $expression,
            Instruction::Neg($pattern) => $expression,

            Instruction::Not($pattern) => $expression,
            Instruction::And($pattern) => $expression,
            Instruction::Or($pattern) => $expression,
            Instruction::Xor($pattern) => $expression,

            Instruction::Lt($pattern) => $expression,
            Instruction::Le($pattern) => $expression,
            Instruction::Eq($pattern) => $expression,
            Instruction::Ne($pattern) => $expression,
            Instruction::Ge($pattern) => $expression,
            Instruction::Gt($pattern) => $expression,

            Instruction::Cast($pattern) => $expression,

            Instruction::ConditionalSelect($pattern) => $expression,
            Instruction::FrameBegin($pattern) => $expression,
            Instruction::FrameEnd($pattern) => $expression,
            Instruction::LoopBegin($pattern) => $expression,
            Instruction::LoopEnd($pattern) => $expression,
            Instruction::Call($pattern) => $expression,
            Instruction::Return($pattern) => $expression,

            Instruction::Assert($pattern) => $expression,
            Instruction::PushCondition($pattern) => $expression,
            Instruction::PopCondition($pattern) => $expression,

            Instruction::Exit($pattern) => $expression,
        }
    };
}
