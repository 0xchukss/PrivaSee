use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_anchor::traits::{QueueCompAccs, CallbackCompAccs, InitCompDefAccs};
use arcium_client::idl::arcium::{
    accounts::MXEAccount,
    types::{CallbackAccount, CallbackInstruction, Parameter, Output},
};

declare_id!("9y179rrXA3yLFRi5qB9cCaWDk87shC3MB4ocpTfUCW8h");

#[program]
pub mod privasee {
    use super::*;

    /// Initializes a new computation definition for PSI.
    pub fn init_compute_psi_comp_def(ctx: Context<InitComputePsiCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)
    }

    /// Requests a Private Set Intersection (PSI) computation between two sets.
    pub fn request_psi(
        ctx: Context<RequestPsi>,
        computation_offset: u64,
    ) -> Result<()> {
        let set_a = ctx.accounts.requester_registry.encrypted_contacts.clone();
        let set_b = ctx.accounts.target_registry.encrypted_contacts.clone();

        // Construct arguments for the MPC circuit
        let mut builder = ArgBuilder::new();
        builder = builder.add_shared_encrypted_struct(set_a);
        builder = builder.add_shared_encrypted_struct(set_b);
        let args = builder.build();

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![], 
            1,      
            0,      
        )?;

        let psi_request = &mut ctx.accounts.psi_request;
        psi_request.requester = ctx.accounts.payer.key();
        psi_request.computation_offset = computation_offset;
        psi_request.status = 1; 
        psi_request.bump = ctx.bumps.psi_request;

        Ok(())
    }

    /// Registers the user's encrypted contact set.
    pub fn register_contacts(
        ctx: Context<RegisterContacts>,
        encrypted_set: SharedEncryptedStruct<MAX_CONTACTS>,
    ) -> Result<()> {
        let user_registry = &mut ctx.accounts.user_registry;
        user_registry.owner = ctx.accounts.payer.key();
        user_registry.encrypted_contacts = encrypted_set;
        user_registry.bump = ctx.bumps.user_registry;
        Ok(())
    }

    pub fn compute_psi_callback(
        ctx: Context<ComputePsiCallback>,
        output_bytes: Vec<u8>,
    ) -> Result<()> {
        let output = SignedComputationOutputs::<ComputePsiOutput>::try_from_slice(&output_bytes)?;

        let result = match output {
            SignedComputationOutputs::Success(o_bytes, _sig) => {
                ComputePsiOutput::try_from_slice(&o_bytes)?
            },
            _ => {
                msg!("PSI computation failed or aborted");
                ctx.accounts.psi_request.status = 2;
                return Err(ErrorCode::AbortedComputation.into());
            }
        };

        let psi_request = &mut ctx.accounts.psi_request;
        psi_request.status = 3;
        
        emit!(PsiCompletedEvent {
            requester: psi_request.requester,
            computation_offset: psi_request.computation_offset,
            match_count: result.field_0.match_count,
            matches: result.field_0.matches,
        });

        Ok(())
    }
}

pub const MAX_CONTACTS: usize = 20;
pub const COMP_DEF_OFFSET_COMPUTE_PSI: u32 = 1;

#[account]
pub struct UserRegistry {
    pub owner: Pubkey,
    pub encrypted_contacts: SharedEncryptedStruct<MAX_CONTACTS>,
    pub bump: u8,
}

#[account]
pub struct PsiRequest {
    pub requester: Pubkey,
    pub computation_offset: u64,
    pub status: u8, 
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub struct ContactHash {
    pub hash: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PsiResult {
    pub matches: [bool; MAX_CONTACTS],
    pub matched_hashes: [ContactHash; MAX_CONTACTS],
    pub match_count: u8,
}

impl Default for PsiResult {
    fn default() -> Self {
        Self {
            matches: [false; MAX_CONTACTS],
            matched_hashes: [ContactHash { hash: [0u8; 32] }; MAX_CONTACTS],
            match_count: 0,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ComputePsiOutput {
    pub field_0: PsiResult,
}

impl HasSize for ComputePsiOutput {
    const SIZE: usize = 20 + (20 * 32) + 1;
}

#[event]
pub struct PsiCompletedEvent {
    pub requester: Pubkey,
    pub computation_offset: u64,
    pub match_count: u8,
    pub matches: [bool; MAX_CONTACTS],
}

#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct InitComputePsiCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 1, 
        seeds = [b"ArciumSignerAccount"],
        bump
    )]
    pub signer_pda_account: Box<Account<'info, ArciumSignerAccount>>,
    /// CHECK: mxe_account
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: UncheckedAccount<'info>,
    /// CHECK: comp_def_account
    #[account(
        init,
        payer = payer,
        space = 1000, 
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_COMPUTE_PSI)
    )]
    pub comp_def_account: UncheckedAccount<'info>,
    /// CHECK: address_lookup_table
    #[account(mut)]
    pub address_lookup_table: UncheckedAccount<'info>,
    /// CHECK: lut_program
    pub lut_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct RequestPsi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8 + 1 + 1,
        seeds = [b"psi_request", payer.key().as_ref(), computation_offset.to_le_bytes().as_ref()],
        bump
    )]
    pub psi_request: Box<Account<'info, PsiRequest>>,
    #[account(
        seeds = [b"user_registry", payer.key().as_ref()],
        bump = requester_registry.bump
    )]
    pub requester_registry: Box<Account<'info, UserRegistry>>,
    #[account(
        seeds = [b"user_registry", target_registry.owner.as_ref()],
        bump = target_registry.bump
    )]
    pub target_registry: Box<Account<'info, UserRegistry>>,
    #[account(seeds = [b"ArciumSignerAccount"], bump)]
    pub signer_pda_account: Box<Account<'info, ArciumSignerAccount>>,
    /// CHECK: mxe_account
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: UncheckedAccount<'info>,
    /// CHECK: mempool_account
    #[account(mut)]
    pub mempool_account: UncheckedAccount<'info>,
    /// CHECK: executing_pool
    #[account(mut)]
    pub executing_pool: UncheckedAccount<'info>,
    /// CHECK: computation_account
    #[account(mut)]
    pub computation_account: UncheckedAccount<'info>,
    /// CHECK: comp_def_account
    pub comp_def_account: UncheckedAccount<'info>,
    /// CHECK: cluster_account
    #[account(mut)]
    pub cluster_account: UncheckedAccount<'info>,
    /// CHECK: pool_account
    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: UncheckedAccount<'info>,
    /// CHECK: clock_account
    #[account(mut, address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[derive(Accounts)]
pub struct RegisterContacts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 32 + (32 + 16 + (MAX_CONTACTS * 32)) + 1,
        seeds = [b"user_registry", payer.key().as_ref()],
        bump
    )]
    pub user_registry: Box<Account<'info, UserRegistry>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ComputePsiCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    /// CHECK: comp_def_account
    pub comp_def_account: UncheckedAccount<'info>,
    /// CHECK: mxe_account
    pub mxe_account: UncheckedAccount<'info>,
    /// CHECK: computation_account
    pub computation_account: UncheckedAccount<'info>,
    /// CHECK: cluster_account
    pub cluster_account: UncheckedAccount<'info>,
    /// CHECK: instructions_sysvar
    pub instructions_sysvar: UncheckedAccount<'info>,
    #[account(mut)]
    pub psi_request: Box<Account<'info, PsiRequest>>,
}

#[account]
pub struct ArciumSignerAccount {
    pub bump: u8,
}

impl<'info> InitCompDefAccs<'info> for InitComputePsiCompDef<'info> {
    fn arcium_program(&self) -> AccountInfo<'info> { self.arcium_program.to_account_info() }
    fn mxe_program(&self) -> Pubkey { crate::ID }
    fn signer(&self) -> AccountInfo<'info> { self.payer.to_account_info() }
    fn mxe_acc(&self) -> AccountInfo<'info> { self.mxe_account.to_account_info() }
    fn comp_def_acc(&self) -> AccountInfo<'info> { self.comp_def_account.to_account_info() }
    fn address_lookup_table(&self) -> AccountInfo<'info> { self.address_lookup_table.to_account_info() }
    fn lut_program(&self) -> AccountInfo<'info> { self.lut_program.to_account_info() }
    fn system_program(&self) -> AccountInfo<'info> { self.system_program.to_account_info() }
    fn params(&self) -> Vec<Parameter> { vec![Parameter::Ciphertext, Parameter::Ciphertext] }
    fn outputs(&self) -> Vec<Output> { vec![Output::Ciphertext] }
    fn comp_def_offset(&self) -> u32 { COMP_DEF_OFFSET_COMPUTE_PSI }
    fn compiled_circuit_len(&self) -> u32 { 1000 } 
    fn weight(&self) -> u64 { 100000 }
}

impl<'info> QueueCompAccs<'info> for RequestPsi<'info> {
    fn comp_def_offset(&self) -> u32 { COMP_DEF_OFFSET_COMPUTE_PSI }
    fn queue_comp_accs(&self) -> arcium_client::idl::arcium::cpi::accounts::QueueComputation<'info> {
        arcium_client::idl::arcium::cpi::accounts::QueueComputation {
            signer: self.payer.to_account_info(),
            sign_seed: self.signer_pda_account.to_account_info(),
            cluster: self.cluster_account.to_account_info(),
            mxe: self.mxe_account.to_account_info(),
            mempool: self.mempool_account.to_account_info(),
            executing_pool: self.executing_pool.to_account_info(),
            comp_def_acc: self.comp_def_account.to_account_info(),
            pool_account: self.pool_account.to_account_info(),
            system_program: self.system_program.to_account_info(),
            clock: self.clock_account.to_account_info(),
            comp: self.computation_account.to_account_info(),
        }
    }
    fn arcium_program(&self) -> AccountInfo<'info> { self.arcium_program.to_account_info() }
    fn mxe_program(&self) -> Pubkey { crate::ID }
    fn signer_pda_bump(&self) -> u8 { self.signer_pda_account.bump }
}

impl<'info> CallbackCompAccs for ComputePsiCallback<'info> {
    fn callback_ix(
        computation_offset: u64,
        mxe_account: &MXEAccount,
        extra_accs: &[CallbackAccount],
    ) -> Result<CallbackInstruction> {
        let mut accounts = vec![
            CallbackAccount { pubkey: crate::ARCIUM_PROG_ID, is_writable: false },
            CallbackAccount { pubkey: derive_comp_def_pda!(COMP_DEF_OFFSET_COMPUTE_PSI), is_writable: false },
            CallbackAccount { pubkey: derive_mxe_pda!(), is_writable: false },
            CallbackAccount { pubkey: derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet), is_writable: false },
            CallbackAccount { pubkey: derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet), is_writable: false },
            CallbackAccount { pubkey: ::anchor_lang::solana_program::sysvar::instructions::ID, is_writable: false },
        ];
        accounts.extend_from_slice(extra_accs);

        Ok(CallbackInstruction {
            program_id: crate::ID,
            discriminator: crate::instruction::ComputePsiCallback::DISCRIMINATOR.to_vec(),
            accounts,
        })
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("MPC cluster was not set in the MXE account")]
    ClusterNotSet,
    #[msg("MPC computation was aborted or failed")]
    AbortedComputation,
}
