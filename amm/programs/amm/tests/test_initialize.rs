#[allow(unused)]
use litesvm;
use litesvm_testing;

#[test]
pub fn test_init() {
    let (mut svm, payer) = litesvm_testing::setup_svm_and_fee_payer();
    // let ID: Pubkey = Pubkey::new_unique();
    // svm.add_program(&amm::ID, include_bytes!("../../../target/build/amm.so"));
    dbg!(amm::ID);
}
