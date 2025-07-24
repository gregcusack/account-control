use {
    serde::{Deserialize, Serialize},
    solana_account_info::{next_account_info, AccountInfo},
    solana_program::{
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        program_error::ProgramError,
    }
};

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct WeightingConfig {
    /// 0 = Static, 1 = Dynamic
    pub weighting_mode: u8,
    /// IIR time-constant (ms)
    pub tc_ms: u64,
}

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8], // JSON-encoded WeightingConfig
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let config_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    if !authority.is_signer {
        msg!("Missing required signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if config_account.owner != program_id {
        msg!("Config account not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize JSON instruction data
    let cfg: WeightingConfig = bincode::deserialize(input)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let encoded = bincode::serialize(&cfg).map_err(|_| ProgramError::InvalidInstructionData)?;

    let dst = &mut config_account.data.borrow_mut()[..];
    if encoded.len() > dst.len() {
        msg!("Serialized config too large");
        return Err(ProgramError::AccountDataTooSmall);
    }

    dst[..encoded.len()].copy_from_slice(&encoded);
    msg!("Updated weighting config: {:?}", cfg);

    Ok(())
}
