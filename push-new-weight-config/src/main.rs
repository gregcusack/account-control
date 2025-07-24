use {
    clap::Parser,
    log::info,
    serde::Serialize,
    solana_commitment_config::CommitmentConfig,
    solana_client::rpc_config::RpcSendTransactionConfig,
    solana_client::rpc_client::RpcClient,
    solana_signer::Signer,
    solana_keypair::read_keypair_file,
    solana_transaction::Transaction,
    solana_instruction::{AccountMeta, Instruction},
    solana_system_interface::instruction as system_instruction,
};

#[derive(Debug, Copy, Clone, Serialize)]
#[repr(C)]
pub struct WeightingConfig {
    pub weighting_mode: u8,
    pub tc_ms: u64,
}

mod program_id {
    solana_program::declare_id!("2dGCYowSix7WWkDUgcxAxyazNkCBZAfrCUxZUGAsTyXh");
}

#[derive(Parser)]
struct Commandline {
    #[arg(long, default_value = "1")]
    /// Weighting mode: 0 = Static, 1 = Dynamic
    weighting_mode: u8,

    #[arg(long, default_value = "30000")]
    /// IIR time constant in milliseconds
    tc_ms: u64,

    #[arg(long, default_value = "http://127.0.0.1:8899")]
    rpc_url: String,

    #[arg(long, default_value = "config-authority.json")]
    payer_keypair: String,

    #[arg(long, default_value = "GrEGgZ5gBXyfyomLPruuvC6a5KXViq445xuhnHWFoTAN.json")]
    storage_holder_kp: String,
}

#[tokio::main]
async fn main() {
    let cli = Commandline::parse();
    let client = RpcClient::new_with_commitment(cli.rpc_url.clone(), CommitmentConfig::confirmed());

    let payer_kp = read_keypair_file(&cli.payer_keypair).expect("Failed to load config account keypair");
    let storage_kp = read_keypair_file(&cli.storage_holder_kp).expect("Failed to load storage account keypair");

    // === Create config account if needed ===
    let account_size = std::mem::size_of::<WeightingConfig>();
    let lamports = client.get_minimum_balance_for_rent_exemption(account_size).unwrap();

    let create_ix = system_instruction::create_account(
        &payer_kp.pubkey(),
        &storage_kp.pubkey(),
        lamports,
        account_size as u64,
        &program_id::ID,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer_kp.pubkey()),
        &[&payer_kp, &storage_kp],
        client.get_latest_blockhash().unwrap(),
    );

    let _ = client.send_and_confirm_transaction_with_spinner_and_config(
        &tx,
        CommitmentConfig::confirmed(),
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        },
    );

    // === Write config data ===
    let cfg = WeightingConfig {
        weighting_mode: cli.weighting_mode,
        tc_ms: cli.tc_ms,
    };
    let payload = bincode::serialize(&cfg).expect("Failed to serialize config");

    let write_ix = Instruction::new_with_bytes(
        program_id::ID,
        &payload,
        vec![
            AccountMeta::new(storage_kp.pubkey(), false),
            AccountMeta::new_readonly(payer_kp.pubkey(), true),
        ],
    );

    let tx = Transaction::new_signed_with_payer(
        &[write_ix],
        Some(&payer_kp.pubkey()),
        &[&payer_kp],
        client.get_latest_blockhash().unwrap(),
    );
    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    info!("✅ Config updated. Signature: {}", sig);
}
