use {litesvm::LiteSVM, solana_keypair::Keypair, solana_signer::Signer};

// Setup function to initialize LiteSVM and create a payer keypair
pub fn setup_test() -> (LiteSVM, Keypair, Keypair) {
    let program_id = escrowq32026::id();
    let maker = Keypair::new();
    let taker = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/escrowq32026.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&maker.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&taker.pubkey(), 1_000_000_000).unwrap();

    // Return the LiteSVM instance and payer keypair
    (svm, maker, taker)
}
