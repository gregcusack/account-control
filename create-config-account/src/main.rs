use {
    solana_client::rpc_client::RpcClient,
    solana_signer::Signer,
    solana_keypair::read_keypair_file,
    solana_transaction::Transaction,
    solana_pubkey::Pubkey,
    solana_system_interface::instruction as system_instruction,
    std::str::FromStr,
};

fn main() {
    let rpc = RpcClient::new("http://localhost:8899".to_string());

    let payer = read_keypair_file("config-authority.json").unwrap();
    // let config_keypair = read_keypair_file("gossip-weighting-config-account.json").unwrap();
    let config_keypair = read_keypair_file("GrEGgZ5gBXyfyomLPruuvC6a5KXViq445xuhnHWFoTAN.json").unwrap();

    let program_id = Pubkey::from_str("2dGCYowSix7WWkDUgcxAxyazNkCBZAfrCUxZUGAsTyXh").unwrap();

    let lamports = 10_000_000; // enough for rent
    let space = 256;

    let create_ix = system_instruction::create_account(
        &payer.pubkey(),
        &config_keypair.pubkey(),
        lamports,
        space,
        &program_id,
    );

    let blockhash = rpc.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&payer, &config_keypair],
        blockhash,
    );

    let sig = rpc.send_and_confirm_transaction(&tx).unwrap();
    println!("✅ Created config account: {}", sig);
}