use {
    log::info,
    serde::Serialize,
    solana_client::rpc_client::RpcClient,
    solana_signer::Signer,
    solana_keypair::read_keypair_file,
    solana_transaction::Transaction,
    solana_pubkey::Pubkey,
    solana_instruction::{AccountMeta, Instruction},
    std::str::FromStr,
};


#[derive(Serialize)]
#[repr(C)]
pub struct WeightingConfig {
    /// 0 = Static, 1 = Dynamic
    pub weighting_mode: u8,
    pub tc_ms: u64, // IIR time-constant (ms)
}

fn main() {
    // === Inputs ===
    let rpc_url = "http://localhost:8899";
    let program_id = Pubkey::from_str("2dGCYowSix7WWkDUgcxAxyazNkCBZAfrCUxZUGAsTyXh").unwrap();
    let config_account = read_keypair_file("gossip-weighting-config-account.json")
        .expect("Failed to load config account keypair")
        .pubkey();
    let authority = read_keypair_file("config-authority.json").expect("Failed to load authority");
    
    
    let config = WeightingConfig {
        weighting_mode: 0,
        tc_ms: 30000,
    };
    let data = serde_json::to_vec(&config).expect("Failed to serialize config");
    // let data = read("weighting_config.bin").expect("Failed to read weighting_config.bin");

    // === Build instruction ===
    let ix = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(config_account, false),
            AccountMeta::new_readonly(authority.pubkey(), true),
        ],
    );

    // === Send transaction ===
    let client = RpcClient::new(rpc_url.to_string());
    let recent_blockhash = client.get_latest_blockhash().unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    info!("✅ Config updated on-chain: {}", sig);
}