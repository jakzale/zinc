use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Serialize, Deserialize};

/// Stores several values from evaluation stack in data stack.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StoreSequence {
    pub address: usize,
    pub len: usize,
}

impl StoreSequence {
    pub fn new(address: usize, len: usize) -> Self {
        Self { address, len }
    }
}

impl InstructionInfo for StoreSequence {
    fn to_assembly(&self) -> String {
        format!("store_array {} {}", self.address, self.len)
    }

    fn code() -> InstructionCode {
        InstructionCode::StoreSequence
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(Self::code(), &[self.address, self.len])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 2)?;

        Ok((Self::new(args[0], args[1]), len))
    }

    fn inputs_count(&self) -> usize {
        self.len
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::StoreSequence((*self).clone())
    }
}
