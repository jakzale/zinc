use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BitwiseXor;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::bitwise;
use crate::instructions::IExecutable;
impl<VM: IVirtualMachine> IExecutable<VM> for BitwiseXor {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;
        let cs = vm.constraint_system();
        let result = bitwise::xor::bit_xor(cs.namespace(|| "bit_and"), &left, &right)?;
        vm.push(result.into())
    }
}