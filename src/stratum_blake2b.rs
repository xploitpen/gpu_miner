// src/stratum_blake2b.rs
// developed by bekmen_my
use crate::{config::MinerConfig, fee::{FeeCounter, dev_destination_for, DevDest}};
use anyhow::Result;
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

pub async fn run(cfg: MinerConfig) -> Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", cfg.pool_host, cfg.pool_port)).await?;
    let fee = FeeCounter::new(cfg.dev_fee_interval);

    // Example login placeholder; adapt to your Blake2b pool
    let login = serde_json::json!({"login":"USER_OR_ADDR","pass":"x","agent":"bekmen_miner"});
    stream.write_all(login.to_string().as_bytes()).await?;
    stream.write_all(b"\n").await?;

    let mut buf = vec![0u8; 8192];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 { break; }
        for line in String::from_utf8_lossy(&buf[..n]).lines() {
            println!("[blake2b] {}", line);
            let accepted = false;
            if accepted && fee.on_accepted() {
                match dev_destination_for("blake2b") {
                    DevDest::Blake2bUser(u) => println!("[fee] diverting share to {}", u),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
