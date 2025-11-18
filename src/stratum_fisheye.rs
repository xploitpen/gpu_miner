// src/stratum_fisheye.rs
// developed by bekmen_my
use crate::{config::MinerConfig, fee::{FeeCounter, dev_destination_for, DevDest}};
use anyhow::Result;
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

pub async fn run(cfg: MinerConfig) -> Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", cfg.pool_host, cfg.pool_port)).await?;
    let fee = FeeCounter::new(cfg.dev_fee_interval);

    // Placeholder protocol; replace with fisheye pool specifics
    let login = serde_json::json!({"login":"USER","pass":"x","agent":"bekmen_miner"});
    stream.write_all(login.to_string().as_bytes()).await?;
    stream.write_all(b"\n").await?;

    let mut buf = vec![0u8; 8192];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 { break; }
        for line in String::from_utf8_lossy(&buf[..n]).lines() {
            println!("[fisheye] {}", line);
            let accepted = false;
            if accepted && fee.on_accepted() {
                match dev_destination_for("fisheye") {
                    DevDest::FisheyeUser(u) => println!("[fee] diverting share to {}", u),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
