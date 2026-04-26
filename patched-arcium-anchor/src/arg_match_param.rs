enum ComputationMatchingError {
    AccountLenNotMultipleOf32(usize),
    AccountLenTooBig(usize),
    ArgumentMismatch(usize, Parameter),
    NotEnoughParams(usize),
    NotEnoughArguments,
}

impl ComputationMatchingError {
    #[allow(dead_code)]
    fn emit_solana_msg(&self, arguments: &[ArgumentRef]) {
        match self {
            ComputationMatchingError::AccountLenNotMultipleOf32(arg_id) => {
                msg!(
                    "Invalid argument : account {:?} len is not a multiple of 32",
                    &arguments[*arg_id],
                );
            }
            ComputationMatchingError::AccountLenTooBig(arg_id) => {
                msg!(
                    "Invalid argument : account {:?} is bigger than the circuit size",
                    &arguments[*arg_id],
                );
            }
            ComputationMatchingError::ArgumentMismatch(arg_id, param) => {
                msg!(
                    "Invalid argument {:?} for parameter {:?}",
                    &arguments[*arg_id],
                    param
                );
            }
            ComputationMatchingError::NotEnoughParams(arg_id) => {
                msg!(
                    "Invalid argument : no parameter matching for {:?}",
                    &arguments[*arg_id]
                );
            }
            ComputationMatchingError::NotEnoughArguments => {
                msg!("Invalid arguments : not enough arguments");
            }
        }
    }

    #[allow(dead_code)]
    const fn const_panic(&self) {
        match self {
            ComputationMatchingError::AccountLenNotMultipleOf32(_) => {
                panic!("Invalid argument : account len is not a multiple of 32");
            }
            ComputationMatchingError::AccountLenTooBig(_) => {
                panic!("Invalid argument : account is bigger than the circuit size");
            }
            ComputationMatchingError::ArgumentMismatch(_, _) => {
                panic!("Invalid argument, mismatch with parameter");
            }
            ComputationMatchingError::NotEnoughParams(_) => {
                panic!("Invalid argument : not enough params");
            }
            ComputationMatchingError::NotEnoughArguments => {
                panic!("Invalid arguments : not enough arguments");
            }
        }
    }
}

const fn arg_match_param(arg: &ArgumentRef, param: &Parameter) -> bool {
    match arg {
        ArgumentRef::PlaintextBool(_) => matches!(param, Parameter::PlaintextBool),
        ArgumentRef::PlaintextU8(_) => matches!(param, Parameter::PlaintextU8),
        ArgumentRef::PlaintextU16(_) => matches!(param, Parameter::PlaintextU16),
        ArgumentRef::PlaintextU32(_) => matches!(param, Parameter::PlaintextU32),
        ArgumentRef::PlaintextU64(_) => matches!(param, Parameter::PlaintextU64),
        ArgumentRef::PlaintextU128(_) => matches!(param, Parameter::PlaintextU128),
        ArgumentRef::PlaintextI8(_) => matches!(param, Parameter::PlaintextI8),
        ArgumentRef::PlaintextI16(_) => matches!(param, Parameter::PlaintextI16),
        ArgumentRef::PlaintextI32(_) => matches!(param, Parameter::PlaintextI32),
        ArgumentRef::PlaintextI64(_) => matches!(param, Parameter::PlaintextI64),
        ArgumentRef::PlaintextI128(_) => matches!(param, Parameter::PlaintextI128),
        ArgumentRef::PlaintextFloat(_) => matches!(param, Parameter::PlaintextFloat),
        ArgumentRef::EncryptedBool(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedU8(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedU16(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedU32(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedU64(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedU128(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedI8(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedI16(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedI32(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedI64(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedI128(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::EncryptedFloat(_) => matches!(param, Parameter::Ciphertext),
        ArgumentRef::PlaintextPoint(_) => matches!(param, Parameter::PlaintextPoint),
        ArgumentRef::X25519Pubkey(_) => matches!(param, Parameter::ArcisX25519Pubkey),
        ArgumentRef::ArcisEd25519Signature(_) => {
            panic!("ArcisEd25519SiSignature are not supposed to reach this function.")
        }
        ArgumentRef::Account(_) => {
            panic!("Accounts are not supposed to reach this function.")
        }
    }
}

const fn args_match_params(
    arguments: &[ArgumentRef],
    accounts: &[AccountArgument],
    parameters: &[Parameter],
) -> core::result::Result<(), ComputationMatchingError> {
    // Validate the arguments match the parameters
    let mut param_idx = 0;
    let mut arg_idx = 0;
    while arg_idx < arguments.len() {
        let arg = &arguments[arg_idx];
        if let ArgumentRef::Account(idx) = arg {
            let acc = &accounts[*idx as usize];
            if acc.length.rem_euclid(32) != 0 {
                return Err(ComputationMatchingError::AccountLenNotMultipleOf32(arg_idx));
            }
            param_idx += (acc.length as usize) / 32;
            // >= is ok, the account finishes right at the end
            if param_idx > parameters.len() {
                return Err(ComputationMatchingError::AccountLenTooBig(arg_idx));
            }
        } else if let ArgumentRef::ArcisEd25519Signature(..) = arg {
            let mut sig_to_64 = 0u8;
            while sig_to_64 < 64 {
                sig_to_64 += 1;
                if param_idx >= parameters.len() {
                    return Err(ComputationMatchingError::NotEnoughParams(arg_idx));
                }
                let param = &parameters[param_idx];
                if !matches!(param, Parameter::PlaintextU8) {
                    return Err(ComputationMatchingError::ArgumentMismatch(arg_idx, *param));
                }
                param_idx += 1;
            }
        } else {
            if param_idx >= parameters.len() {
                return Err(ComputationMatchingError::NotEnoughParams(arg_idx));
            }
            let param = &parameters[param_idx];
            if !arg_match_param(arg, param) {
                return Err(ComputationMatchingError::ArgumentMismatch(arg_idx, *param));
            }
            param_idx += 1;
        }
        arg_idx += 1;
    }
    if param_idx < parameters.len() {
        return Err(ComputationMatchingError::NotEnoughArguments);
    }

    Ok(())
}
