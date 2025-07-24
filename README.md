# Gossip Weighting Config: On-Chain Setup Guide

This guide walks you through setting up the on-chain program and config account to control gossip push active set behavior in a Solana validator.

---

## 🔧 Prerequisites

```bash
./multinode-demo/setup.sh
./multinode-demo/faucet.sh
```

---

## 🔑 Generate the Config Authority Keypair

```bash
solana-keygen new -o config-authority.json
solana airdrop 100 --keypair config-authority.json --ul
```

---

## 🚀 Run Bootstrap Validator

```bash
./multinode-demo/bootstrap-validator.sh --log ~/mnode-demo.log
```

---

## 🛠 Build and Deploy the Program

```bash
cd update-config
cargo build-sbf
cd ../target/deploy/
solana program deploy ./update_config.so -ul --keypair ~/accounts-control/config-authority.json
```

> `config-authority.json` is the funded keypair used to deploy the program.

Sample output:
```
Program Id: 2dGCYowSix7WWkDUgcxAxyazNkCBZAfrCUxZUGAsTyXh
Signature: uWpodzBXFE135ynGSrj2NQ18cSpQwnKm2xxnzjeCZefP...
```

---

## 🧱 Create the Config Account Keypair

```bash
solana-keygen new -o gossip-weighting-config-account.json
```

Get the config account pubkey:
```bash
solana address -k gossip-weighting-config-account.json
```
Sample output:
```
9aFDtRxwS1EjG8JnUnzpwRRvN47xNApF1hPY3GAu8kVp
```

Update your validator code:
```rust
mod weighting_config_control_pubkey {
    solana_pubkey::declare_id!("9aFDtRxwS1EjG8JnUnzpwRRvN47xNApF1hPY3GAu8kVp");
}
```
You'll have to restart your validators.

---

## 🧬 Deploy the Config Account

```bash
cargo run -p create-config-account
```

---

## 🏁 Start Additional Validators

```bash
./multinode-demo/validator-x.sh
./multinode-demo/validator-x.sh
```

---

## ⚙️ Push the Initial Config

Edit `push-new-weight-config` to set the appropriate `WeightingConfig`, then run:

```bash
cargo run -p push-new-weight-config
```

You can pass in keypair files, weighting-mode, and tc_ms as well. Run:
```bash
cargo run -p push-new-weight-config -- --help
```
---

## ✅ Observe Validator Logs

Every 7.5 seconds (at each `PushActiveSet` refresh), the validator should load the config account and log:

```text
greg: apply_cfg: WeightingConfig { weighting_mode: 1, tc_ms: 30000 }
```

