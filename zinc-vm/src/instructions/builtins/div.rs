extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::gadgets::{ScalarType, ScalarTypeExpectation};
use crate::Engine;
use zinc_bytecode::instructions::Div;

impl<E, CS> VMInstruction<E, CS> for Div
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let condition = vm.condition_top()?;

        let (unchecked_div, _rem) =
            vm.operations()
                .div_rem_conditional(left.clone(), right.clone(), condition)?;

        let condition = vm.condition_top()?;
        let div = vm.operations().assert_type(
            condition,
            unchecked_div,
            ScalarType::expect_same(left.get_type(), right.get_type())?,
        )?;

        vm.push(Cell::Value(div))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::scalar::IntegerType;
    use zinc_bytecode::*;

    #[test]
    fn test_div() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new((9).into(), IntegerType::I8.into()))
            .add(PushConst::new((4).into(), IntegerType::I8.into()))
            .add(Div)
            .add(PushConst::new((9).into(), IntegerType::I8.into()))
            .add(PushConst::new((-4).into(), IntegerType::I8.into()))
            .add(Div)
            .add(PushConst::new((-9).into(), IntegerType::I8.into()))
            .add(PushConst::new((4).into(), IntegerType::I8.into()))
            .add(Div)
            .add(PushConst::new((-9).into(), IntegerType::I8.into()))
            .add(PushConst::new((-4).into(), IntegerType::I8.into()))
            .add(Div)
            .test(&[3, -3, -2, 2])
    }
}
