// src/stratum_monero.rs
// developed by bekmen_my
use crate::{config::MinerConfig, fee::{FeeCounter, dev_destination_for, DevDest}};
use anyhow::Result;
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

pub async fn run(cfg: MinerConfig) -> Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", cfg.pool_host, cfg.pool_port)).await?;
    let fee = FeeCounter::new(cfg.dev_fee_interval);

    let rig = cfg.rig_name.clone().unwrap_or_default();
    let login = serde_json::json!({
        "id": 1, "jsonrpc": "2.0", "method": "login",
        "params": { "login": "DUMMY", "pass": "x", "agent": "bekmen_miner/0.1", "rig_id": rig }
    });
    stream.write_all(login.to_string().as_bytes()).await?;
    stream.write_all(b"\n").await?;

    let mut buf = vec![0u8; 8192];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 { break; }
        for line in String::from_utf8_lossy(&buf[..n]).lines() {
            println!("[monero] {}", line);
            // TODO: parse job, compute, submit
            // After pool responds "accepted", call fee.on_accepted()
            let accepted = false; // replace with actual check
            if accepted && fee.on_accepted() {
                if let DevDest::MoneroAddr(addr) = dev_destination_for("monero") {
                    println!("[fee] diverting share to dev address {}", addr);
                    // Re-login temporarily or submit using dev credentials per pool rules
                }
            }
        }
    }
    Ok(())
}
