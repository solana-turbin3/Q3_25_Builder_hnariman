#![cfg(feature = "test-sbf")]

use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    mollusk_svm::{result::Check, Mollusk},
};

#[test]
fn test_initialize() {
    let program_id = vault::id();

    let mollusk = Mollusk::new(&program_id, "vault");

    let instruction = Instruction::new_with_bytes(
        program_id,
        &vault::instruction::Initialize {}.data(),
        vault::accounts::Initialize {}.to_account_metas(None),
    );

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}

// #[test]
// fn create_vault(){
//     let program_id = vault::id();
//
//     let key1 = Pubkey::new_unique();
//
//     let instruction = Instruction::new_with_bytes(
//         program_id,
//         $[],
//         vec![
//             AccountMeta::new(key1,false)
//             AccountMeta::new_readonly(key2,false)
//         ],
//     );
//
//     let accounts = vec![
// (key1, Account::default()),
// (key2, Account::default()),
//     ];
//
//     let mollusk = Mollusk::new(&program_id, "vault");
//
//     let result = mollusk.process_instruction(&instruction, &accounts);
// }
