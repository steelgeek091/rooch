// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

pub mod natives;

use move_binary_format::errors::PartialVMResult;
use move_core_types::gas_algebra::InternalGas;
use move_vm_types::natives::function::NativeResult;
use moveos_stdlib::natives::moveos_stdlib::wasm::E_INCORRECT_LENGTH_OF_ARGS;
pub use rooch_types::addresses::*;

pub fn args_count_error(base_gas: InternalGas) -> PartialVMResult<NativeResult> {
    Ok(NativeResult::err(base_gas, E_INCORRECT_LENGTH_OF_ARGS))
}
