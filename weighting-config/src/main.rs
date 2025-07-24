use {
    serde_derive::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct WeightingConfig {
    /// 0 = Static, 1 = Dynamic
    pub weighting_mode: u8,
    pub tc_ms: u64, // IIR time-constant (ms)
}

fn main() -> std::io::Result<()> {
    let cfg = WeightingConfig {
        weighting_mode: 1,
        tc_ms: 30000,
    };
    let encoded = bincode::serialize(&cfg).unwrap();
    std::fs::write("weighting_config.bin", &encoded)?;
    println!("Wrote {} bytes", encoded.len());

    // Read it back to verify
    let data = std::fs::read("weighting_config.bin")?;
    let decoded: WeightingConfig = bincode::deserialize(&data).unwrap();
    println!("Read back: weighting_mode={}, tc_ms={}", decoded.weighting_mode, decoded.tc_ms);
    
    Ok(())
}