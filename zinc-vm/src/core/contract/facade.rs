//!
//! The virtual machine contract facade.
//!

use std::marker::PhantomData;

use num_bigint::BigInt;

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_bytecode::Contract as BytecodeContract;
use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;

use crate::constraint_systems::debug::DebugCS;
use crate::core::contract::storage::dummy::Storage as DummyStorage;
use crate::core::contract::synthesizer::Synthesizer as ContractSynthesizer;
use crate::core::contract::Contract;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::error::TypeSizeError;
use crate::facade::IFacade;
use crate::gadgets::contract::merkle_tree::hasher::sha256::Hasher as Sha256Hasher;
use crate::gadgets::contract::storage::StorageGadget;
use crate::IEngine;

impl IFacade for BytecodeContract {
    fn debug<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        let mut cs = TestConstraintSystem::<Bn256>::new();

        let inputs_flat = input.to_flat_values();
        let output = self.output.to_owned();

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();

        let storage = DummyStorage::new(storage_fields);
        let storage_gadget =
            StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
        let mut contract = Contract::new(cs, storage_gadget, true);

        let mut num_constraints = 0;
        let result = contract.run(
            &BytecodeProgram::Contract(self),
            Some(&inputs_flat),
            |cs| {
                let num = cs.num_constraints() - num_constraints;
                num_constraints += num;
                log::debug!("Constraints: {}", num);
            },
            |cs| {
                if !cs.is_satisfied() {
                    return Err(RuntimeError::UnsatisfiedConstraint);
                }

                Ok(())
            },
        )?;

        let cs = contract.constraint_system();

        log::trace!("{}", cs.pretty_print());

        if !cs.is_satisfied() {
            log::error!("Unsatisfied: {}", cs.which_is_unsatisfied().unwrap());
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let unconstrained = cs.find_unconstrained();
        if !unconstrained.is_empty() {
            log::error!("Unconstrained: {}", unconstrained);
            return Err(RuntimeError::InternalError(
                "Generated unconstrained variables".into(),
            ));
        }

        let output_flat = result
            .into_iter()
            .map(|v| v.expect("`run` always computes witness"))
            .collect::<Vec<_>>();

        let value = TemplateValue::from_flat_values(&output, &output_flat).ok_or_else(|| {
            TypeSizeError::Output {
                expected: 0,
                actual: 0,
            }
        })?;

        Ok(value)
    }

    fn run<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        let mut cs = DebugCS::<Bn256>::default();

        let inputs_flat = input.to_flat_values();
        let output = self.output.to_owned();

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = DummyStorage::new(storage_fields);
        let storage_gadget =
            StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
        let mut contract = Contract::new(cs, storage_gadget, true);

        let mut num_constraints = 0;
        let result = contract.run(
            &BytecodeProgram::Contract(self),
            Some(&inputs_flat),
            |cs| {
                let num = cs.num_constraints() - num_constraints;
                num_constraints += num;
                log::debug!("Constraints: {}", num);
            },
            |cs| {
                if !cs.is_satisfied() {
                    return Err(RuntimeError::UnsatisfiedConstraint);
                }

                Ok(())
            },
        )?;

        let cs = contract.constraint_system();
        if !cs.is_satisfied() {
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let output_flat = result
            .into_iter()
            .map(|v| v.expect("`run` always computes witness"))
            .collect::<Vec<_>>();

        let value = TemplateValue::from_flat_values(&output, &output_flat).ok_or_else(|| {
            TypeSizeError::Output {
                expected: 0,
                actual: 0,
            }
        })?;

        Ok(value)
    }

    fn setup<E: IEngine>(self) -> Result<Parameters<E>, RuntimeError> {
        let rng = &mut rand::thread_rng();
        let mut result = None;

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = DummyStorage::new(storage_fields);

        let synthesizable = ContractSynthesizer {
            inputs: None,
            output: &mut result,
            bytecode: BytecodeProgram::Contract(self),
            storage,

            _pd: PhantomData,
        };

        let params = groth16::generate_random_parameters::<E, _, _>(synthesizable, rng)?;

        match result.expect("vm should return either output or error") {
            Ok(_) => Ok(params),
            Err(error) => Err(error),
        }
    }

    fn prove<E: IEngine>(
        self,
        params: Parameters<E>,
        witness: TemplateValue,
    ) -> Result<(TemplateValue, Proof<E>), RuntimeError> {
        let mut result = None;
        let rng = &mut rand::thread_rng();

        let witness_flat = witness.to_flat_values();
        let output = self.output.to_owned();

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = DummyStorage::new(storage_fields);

        let synthesizable = ContractSynthesizer {
            inputs: Some(witness_flat),
            output: &mut result,
            bytecode: BytecodeProgram::Contract(self),
            storage,

            _pd: PhantomData,
        };

        let proof = groth16::create_random_proof(synthesizable, &params, rng)
            .map_err(RuntimeError::SynthesisError)?;

        match result {
            None => Err(RuntimeError::InternalError(
                "circuit hasn't generate outputs".into(),
            )),
            Some(res) => match res {
                Ok(values) => {
                    let output_flat: Vec<BigInt> = values
                        .into_iter()
                        .map(|v| v.expect("`prove` always computes witness"))
                        .collect();

                    let value = TemplateValue::from_flat_values(&output, &output_flat).ok_or_else(
                        || TypeSizeError::Output {
                            expected: 0,
                            actual: 0,
                        },
                    )?;

                    Ok((value, proof))
                }
                Err(err) => Err(err),
            },
        }
    }
}