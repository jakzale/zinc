//!
//! The generator expression place operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::Zero;

use zinc_bytecode::Instruction;
use zinc_bytecode::ScalarType;

use crate::generator::bytecode::Bytecode;
use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::place::element::Element as SemanticPlaceElement;
use crate::semantic::element::place::memory_type::MemoryType;
use crate::semantic::element::place::Place as SemanticPlace;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct Place {
    pub identifier: Identifier,
    pub element_size: usize,
    pub total_size: usize,
    pub elements: Vec<SemanticPlaceElement>,
    pub memory_type: MemoryType,
}

impl Place {
    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        if !self.elements.is_empty() {
            IntegerConstant::new(BigInt::zero(), false, crate::BITLENGTH_FIELD)
                .write_all_to_bytecode(bytecode.clone());
        }

        for element in self.elements.into_iter() {
            match element {
                SemanticPlaceElement::IndexConstant { constant, access } => {
                    IntegerConstant::from_semantic(&constant)
                        .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
                        Some(self.identifier.location),
                    );
                    IntegerConstant::new(
                        BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Mul(zinc_bytecode::Mul),
                        Some(self.identifier.location),
                    );
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexExpression { expression, access } => {
                    expression.write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
                        Some(self.identifier.location),
                    );
                    IntegerConstant::new(
                        BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Mul(zinc_bytecode::Mul),
                        Some(self.identifier.location),
                    );
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexRange { start, access, .. } => {
                    IntegerConstant::new(
                        start * BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexRangeInclusive { start, access, .. } => {
                    IntegerConstant::new(
                        start * BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::StackField { access } => {
                    IntegerConstant::new(
                        BigInt::from(access.offset),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::ContractField { .. } => {}
            }
        }
    }
}

impl From<SemanticPlace> for Place {
    fn from(place: SemanticPlace) -> Self {
        Self {
            identifier: place.identifier,
            element_size: place.r#type.size(),
            total_size: place.total_size,
            elements: place.elements,
            memory_type: place.memory_type,
        }
    }
}