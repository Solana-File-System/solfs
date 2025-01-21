use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::DataStoreError,
    instruction::DataStoreInstruction,
    state::{
        DataStoreAccountMetadata, DataStoreState, DataStoreTypeOption, SerializationStatusOption,
        DATA_STORE_VERSION, METADATA_SIZE, PDA_SEED,
    },
};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = DataStoreInstruction::try_from_slice(instruction_data)
            .map_err(|_| DataStoreError::InvalidInstructionData)?;

        match instruction {
            DataStoreInstruction::InitializeDataStore(args) => {
                Self::initialize_data_store(program_id, accounts, args)
            }
            DataStoreInstruction::UpdateDataStore(args) => {
                Self::update_data_store(program_id, accounts, args)
            }
            DataStoreInstruction::UpdateDataStoreAuthority(args) => {
                Self::update_data_store_authority(program_id, accounts, args)
            }
            DataStoreInstruction::FinalizeDataStore(args) => {
                Self::finalize_data_store(program_id, accounts, args)
            }
            DataStoreInstruction::CloseDataStore(args) => {
                Self::close_data_store(program_id, accounts, args)
            }
        }
    }

    fn initialize_data_store(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: InitializeDataStoreArgs,
    ) -> ProgramResult {
        if args.debug {
            msg!("InitializeDataStore");
        }

        let accounts_iter = &mut accounts.iter();
        let feepayer = next_account_info(accounts_iter)?;
        let data_account = next_account_info(accounts_iter)?;
        let metadata_account = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // Create a data_account of given space if not done so already
        if !args.is_created {
            let space = args.space as usize;
            let rent_exemption_amount = Rent::get()?.minimum_balance(space);

            let create_account_ix = system_instruction::create_account(
                &feepayer.key,
                &data_account.key,
                rent_exemption_amount,
                space as u64,
                &program_id,
            );
            invoke(
                &create_account_ix,
                &[
                    feepayer.clone(),
                    data_account.clone(),
                    system_program.clone(),
                ],
            )?;

            if args.debug {
                msg!("account of space: {} created", space);
            }
        }
        // Else set data program as the owner of the data_account
        else {
            let assign_ix = system_instruction::assign(&data_account.key, &program_id);
            invoke(&assign_ix, &[data_account.clone(), system_program.clone()])?;

            if args.debug {
                msg!("account owner updated");
            }
        }
        data_account.data.borrow_mut().fill(0);

        // Create data_account PDA to store metadata
        let (pda, bump_seed) = Pubkey::find_program_address(
            &[PDA_SEED, data_account.key.as_ref()],
            program_id,
        );
        // Ensure the PDA is valid
        if pda != *metadata_account.key {
            return Err(DataStoreError::InvalidPDA.into());
        }
        // Create PDA account
        let rent_exemption_amount = Rent::get()?.minimum_balance(METADATA_SIZE);
        let create_pda_ix = system_instruction::create_account(
            &feepayer.key,
            &metadata_account.key,
            rent_exemption_amount,
            METADATA_SIZE as u64,
            &program_id,
        );
        invoke_signed(
            &create_pda_ix,
            &[
                feepayer.clone(),
                data_account.clone(),
                metadata_account.clone(),
                system_program.clone(),
            ],
            &[&[PDA_SEED, data_account.key.as_ref(), &[bump_seed]]],
        )?;

        if args.debug {
            msg!("metadata pda created");
        }

        // Create initial state for data_account metadata and write to it
        let account_metadata = DataStoreAccountMetadata::new(
            args.authority,
            args.data_type,
            bump_seed,
        );
        account_metadata.serialize(&mut &mut metadata_account.data.borrow_mut()[..])?;

        Ok(())
    }

    fn update_data_store(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: UpdateDataStoreArgs,
    ) -> ProgramResult {
        if args.debug {
            msg!("UpdateDataStore");
        }

        let accounts_iter = &mut accounts.iter();
        let authority = next_account_info(accounts_iter)?;
        let data_account = next_account_info(accounts_iter)?;
        let metadata_account = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // Ensure authority is signer
        if !authority.is_signer {
            return Err(DataStoreError::NotSigner.into());
        }

        // Ensure authority, data_account, and metadata_account are writable
        if !authority.is_writable
            || !data_account.is_writable
            || !metadata_account.is_writable
        {
            return Err(DataStoreError::NotWriteable.into());
        }

        // Ensure length is not 0
        if metadata_account.data_is_empty() {
            return Err(DataStoreError::NoAccountLength.into());
        }

        let mut account_metadata =
            DataStoreAccountMetadata::try_from_slice(&metadata_account.try_borrow_data()?)?;

        // Ensure data_account is initialized and not finalized
        match *account_metadata.data_status() {
            DataStoreState::Uninitialized => {
                return Err(DataStoreError::NotInitialized.into());
            }
            DataStoreState::Finalized => {
                return Err(DataStoreError::AlreadyFinalized.into());
            }
            _ => (),
        }

        // Ensure data_account is being written to by valid authority
        if account_metadata.authority() != authority.key {
            return Err(DataStoreError::InvalidAuthority.into());
        }

        // Ensure the metadata_account corresponds to the data_account
        let pda = Pubkey::create_program_address(
            &[
                PDA_SEED,
                data_account.key.as_ref(),
                &[account_metadata.bump_seed()],
            ],
            program_id,
        )?;
        if pda != *metadata_account.key {
            return Err(DataStoreError::InvalidPDA.into());
        }

        let old_len = data_account.data_len();
        let end_len = args.offset as usize + args.data.len();

        // Ensure static data_account has sufficient space
        if !account_metadata.is_dynamic() && old_len < end_len {
            return Err(DataStoreError::InsufficientSpace.into());
        }

        if args.debug {
            msg!("account checks passed");
        }

        let new_len = if !account_metadata.is_dynamic() {
            old_len
        } else if args.realloc_down {
            end_len
        } else {
            old_len.max(end_len)
        };

        // Update the metadata_account
        account_metadata.set_data_type(args.data_type);
        account_metadata.serialize(&mut &mut metadata_account.data.borrow_mut()[..])?;

        // Ensure data_account has enough space by reallocing if needed
        if old_len != new_len {
            let new_space = new_len;
            let new_minimum_balance = Rent::get()?.minimum_balance(new_space);
            let lamports_diff = if old_len < new_len {
                new_minimum_balance.saturating_sub(data_account.lamports())
            } else {
                data_account.lamports().saturating_sub(new_minimum_balance)
            };

            if old_len < new_len {
                let transfer_ix = system_instruction::transfer(
                    authority.key,
                    data_account.key,
                    lamports_diff,
                );
                invoke(
                    &transfer_ix,
                    &[
                        authority.clone(),
                        data_account.clone(),
                        system_program.clone(),
                    ],
                )?;
            } else {
                let authority_lamports = authority.lamports();
                **authority.lamports.borrow_mut() = authority_lamports
                    .checked_add(lamports_diff)
                    .ok_or(DataStoreError::Overflow)?;
                **data_account.lamports.borrow_mut() = new_minimum_balance;
            }

            data_account.realloc(new_space, false)?;

            if args.debug {
                msg!("realloc-ed {}", new_space);
            }
        }

        // Update the data_account
        if args.debug {
            msg!(
                "replaced {:?} with {:?}",
                &args.data,
                &data_account.data.borrow()[args.offset as usize..end_len]
            );
        }

        data_account.data.borrow_mut()[args.offset as usize..end_len]
            .copy_from_slice(&args.data);

        Ok(())
    }

    fn update_data_store_authority(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: UpdateDataStoreAuthorityArgs,
    ) -> ProgramResult {
        if args.debug {
            msg!("UpdateDataStoreAuthority");
        }

        let accounts_iter = &mut accounts.iter();
        let authority = next_account_info(accounts_iter)?;
        let data_account = next_account_info(accounts_iter)?;
        let metadata_account = next_account_info(accounts_iter)?;
        let new_authority = next_account_info(accounts_iter)?;

        // Ensure authority and new_authority are signer
        if !authority.is_signer || !new_authority.is_signer {
            return Err(DataStoreError::NotSigner.into());
        }

        // Ensure metadata_account is writable
        if !metadata_account.is_writable {
            return Err(DataStoreError::NotWriteable.into());
        }

        // Ensure length is not 0
        if metadata_account.data_is_empty() {
            return Err(DataStoreError::NoAccountLength.into());
        }

        let mut account_metadata =
            DataStoreAccountMetadata::try_from_slice(&metadata_account.try_borrow_data()?)?;

        // Ensure data_account is initialized
        if *account_metadata.data_status() == DataStoreState::Uninitialized {
            return Err(DataStoreError::NotInitialized.into());
        }

        // Ensure data_account is being written to by valid authority
        if account_metadata.authority() != authority.key {
            return Err(DataStoreError::InvalidAuthority.into());
        }

        // Ensure the metadata_account corresponds to the data_account
        let pda = Pubkey::create_program_address(
            &[
                PDA_SEED,
                data_account.key.as_ref(),
                &[account_metadata.bump_seed()],
            ],
            program_id,
        )?;
        if pda != *metadata_account.key {
            return Err(DataStoreError::InvalidPDA.into());
        }

        if args.debug {
            msg!("account checks passed")
        }

        // Update the authority
        account_metadata.set_authority(*new_authority.key);
        account_metadata.serialize(&mut &mut metadata_account.data.borrow_mut()[..])?;

        if args.debug {
            msg!("updated authority");
        }

        Ok(())
    }

    fn finalize_data_store(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: FinalizeDataStoreArgs,
    ) -> ProgramResult {
        if args.debug {
            msg!("FinalizeDataStore");
        }

        let accounts_iter = &mut accounts.iter();
        let authority = next_account_info(accounts_iter)?;
        let data_account = next_account_info(accounts_iter)?;
        let metadata_account = next_account_info(accounts_iter)?;

        // Ensure authority is signer
        if !authority.is_signer {
            return Err(DataStoreError::NotSigner.into());
        }

        // Ensure metadata_account is writable
        if !metadata_account.is_writable {
            return Err(DataStoreError::NotWriteable.into());
        }

        // Ensure length is not 0
        if metadata_account.data_is_empty() {
            return Err(DataStoreError::NoAccountLength.into());
        }

        let mut account_metadata =
            DataStoreAccountMetadata::try_from_slice(&metadata_account.try_borrow_data()?)?;

        // Ensure data_account is initialized and not finalized
        match *account_metadata.data_status() {
            DataStoreState::Uninitialized => {
                return Err(DataStoreError::NotInitialized.into());
            }
            DataStoreState::Finalized => {
                return Err(DataStoreError::AlreadyFinalized.into());
            }
            _ => (),
        }

        // Ensure metadata_account is being written to by valid authority
        if account_metadata.authority() != authority.key {
            return Err(DataStoreError::InvalidAuthority.into());
        }

        // Ensure the metadata_account corresponds to the data_account
        let pda = Pubkey::create_program_address(
            &[
                PDA_SEED,
                data_account.key.as_ref(),
                &[account_metadata.bump_seed()],
            ],
            program_id,
        )?;
        if pda != *metadata_account.key {
            return Err(DataStoreError::InvalidPDA.into());
        }

        if args.debug {
            msg!("account checks passed");
        }

        // Update the data_account
        account_metadata.set_data_status(DataStoreState::Finalized);
        account_metadata.serialize(&mut &mut metadata_account.data.borrow_mut()[..])?;

        if args.debug {
            msg!("updated finalize flag");
        }

        Ok(())
    }

    fn close_data_store(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: CloseDataStoreArgs,
    ) -> ProgramResult {
        if args.debug {
            msg!("CloseDataStore");
        }

        let accounts_iter = &mut accounts.iter();
        let authority = next_account_info(accounts_iter)?;
        let data_account = next_account_info(accounts_iter)?;
        let metadata_account = next_account_info(accounts_iter)?;

        // Ensure authority is signer
        if !authority.is_signer {
            return Err(DataStoreError::NotSigner.into());
        }

        // Ensure authority, data_account, and metadata_account are writable
        if !authority.is_writable
            || !data_account.is_writable
            || !metadata_account.is_writable
        {
            return Err(DataStoreError::NotWriteable.into());
        }

        // Ensure length is not 0
        if metadata_account.data_is_empty() {
            return Err(DataStoreError::NoAccountLength.into());
        }

        let account_metadata =
            DataStoreAccountMetadata::try_from_slice(&metadata_account.try_borrow_data()?)?;

        // Ensure data_account is initialized
        if *account_metadata.data_status() == DataStoreState::Uninitialized {
            return Err(DataStoreError::NotInitialized.into());
        }

        // Ensure data_account is being closed by valid authority
        if account_metadata.authority() != authority.key {
            return Err(DataStoreError::InvalidAuthority.into());
        }

        // Ensure the metadata_account corresponds to the data_account
        let pda = Pubkey::create_program_address(
            &[
                PDA_SEED,
                data_account.key.as_ref(),
                &[account_metadata.bump_seed()],
            ],
            program_id,
        )?;
        if pda != *metadata_account.key {
            return Err(DataStoreError::InvalidPDA.into());
        }

        // Transfer metadata_account lamports back to authority and reset metadata_account
        let curr_lamports = authority.lamports();
        **authority.lamports.borrow_mut() = curr_lamports
            .checked_add(metadata_account.lamports())
            .ok_or(DataStoreError::Overflow)?;
        **metadata_account.lamports.borrow_mut() = 0;
        metadata_account.data.borrow_mut().fill(0);

        if args.debug {
            msg!("{} transfered to authority for metadata pda", curr_lamports);
        }

        // Transfer data_account lamports back to authority and reset data_account
        let curr_lamports = authority.lamports();
        **authority.lamports.borrow_mut() = curr_lamports
            .checked_add(data_account.lamports())
            .ok_or(DataStoreError::Overflow)?;
        **data_account.lamports.borrow_mut() = 0;
        data_account.data.borrow_mut().fill(0);

        if args.debug {
            msg!("{} transfered to authority for data account", curr_lamports);
        }

        Ok(())
    }
}