use crate::core::location::CodeLocation;
use crate::core::{VMInstruction, VirtualMachine};
<<<<<<< HEAD
use crate::error::RuntimeError;
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::ColumnMarker;
use zinc_bytecode::FileMarker;
use zinc_bytecode::FunctionMarker;
use zinc_bytecode::LineMarker;
=======
use crate::RuntimeError;

use zinc_bytecode::instructions::*;
>>>>>>> am/storage

impl<VM: VirtualMachine> VMInstruction<VM> for FileMarker {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.set_location(CodeLocation {
            file: Some(self.file.clone()),
            function: None,
            line: None,
            column: None,
        });

        Ok(())
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for FunctionMarker {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let mut location = vm.get_location();
        location.function = Some(self.function.clone());
        vm.set_location(location);
        Ok(())
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for LineMarker {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let mut location = vm.get_location();
        location.line = Some(self.line);
        vm.set_location(location);
        Ok(())
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for ColumnMarker {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let mut location = vm.get_location();
        location.column = Some(self.column);
        vm.set_location(location);
        Ok(())
    }
}