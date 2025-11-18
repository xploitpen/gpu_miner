// src/stratum.rs
use crate::config::MinerConfig;
use anyhow::Result;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

pub async fn run(cfg: MinerConfig) -> Result<()> {
    let addr = format!("{}:{}", cfg.pool_host, cfg.pool_port);
    let mut stream = TcpStream::connect(addr).await?;

    let login = serde_json::json!({
        "id": 1, "jsonrpc": "2.0", "method": "login",
        "params": { "login": "DUMMY", "pass": "x", "agent": "gpu-miner/0.1", "rig_id": cfg.rig_name.unwrap_or_default() }
    });

    stream.write_all(login.to_string().as_bytes()).await?;
    stream.write_all(b"\n").await?;

    let mut buf = vec![0u8; 8192];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 { break; }
        let s = String::from_utf8_lossy(&buf[..n]);
        for line in s.lines() {
            println!("[pool] {}", line);
            // TODO: parse job, run kernel, submit shares
        }
    }
    Ok(())
}
