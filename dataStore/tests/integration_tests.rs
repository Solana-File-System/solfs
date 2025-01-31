use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    instruction::{Instruction, AccountMeta}, // Import Instruction and AccountMeta
};
use dataaccount::instruction::DataStoreInstruction;
use dataaccount::state::{InitializeDataStoreArgs, DataStoreTypeOption, PDA_SEED}; // Import DataStoreTypeOption and PDA_SEED

#[tokio::test]
async fn test_initialize_data_store() {
    // Set up the test environment
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "dataaccount", // Name of your program
        program_id,
        processor!(dataaccount::processor::Processor::process_instruction), // Replace with your processor
    )
    .start()
    .await;

    // Create a new account for the data store
    let data_account = Keypair::new();

    // Derive the PDA
    let (data_store_pda, bump_seed) = Pubkey::find_program_address(
        &[
            PDA_SEED,
            data_account.pubkey().as_ref(),
        ],
        &program_id,
    );

    // Create the InitializeDataStoreArgs
    let args = InitializeDataStoreArgs {
        debug: false,
        data_type: DataStoreTypeOption::File,
        bump_seed,  // Use the derived bump seed
        is_created: false,
        space: 1024,
        authority: payer.pubkey(),
        is_dynamic: false,
    };

    // Create the instruction
    let instruction = DataStoreInstruction::InitializeDataStore(args);

    // Convert DataStoreInstruction to Solana Instruction
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),         // authority (signer)
        AccountMeta::new(data_account.pubkey(), true),  // data_account (signer)
        AccountMeta::new(data_store_pda, false),        // data_store_pda
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false), // system_program
    ];

    let instruction = Instruction::new_with_borsh(
        program_id,
        &instruction,
        accounts,
    );

    // Create and sign transaction
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &data_account], recent_blockhash);

    // Process the transaction
    banks_client.process_transaction(transaction).await.unwrap();
}