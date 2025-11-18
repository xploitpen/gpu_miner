// src/main.rs
// developed by bekmen_my
mod banner;
mod config;
mod detect;
mod fee;
mod algo;
mod stratum_monero;
mod stratum_blake2b;
mod stratum_fisheye;
mod duinocoin;
mod mining;

use anyhow::Result;
use inquire::{Select, Text, Confirm};
use std::{fs, path::PathBuf};
use config::MinerConfig;

fn cfg_path() -> PathBuf { PathBuf::from("miner_config.json") }

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", banner::RUNTIME_BANNER);

    let cfg: MinerConfig = if cfg_path().exists() {
        serde_json::from_str(&fs::read_to_string(cfg_path())?)?
    } else {
        let algo = Select::new("Select algorithm:", vec!["monero", "blake2b", "fisheye", "duino"]).prompt()?;
        let pool_host = Text::new("Pool host:").prompt()?;
        let pool_port: u16 = Text::new("Pool port:").prompt()?.parse()?;
        let rig_name = if Confirm::new("Add rig name?").with_default(true).prompt()? {
            Some(Text::new("Rig name:").prompt()?)
        } else { None };
        let backend = detect::compiled_backend();
        let cfg = MinerConfig {
            algo: algo.to_string(),
            pool_host,
            pool_port,
            rig_name,
            backend,
            dev_fee_interval: 100,
        };
        fs::write(cfg_path(), serde_json::to_string_pretty(&cfg)?)?;
        cfg
    };

    println!("Algo={} | {}:{} | backend={} | rig={}",
        cfg.algo, cfg.pool_host, cfg.pool_port, cfg.backend, cfg.rig_name.as_deref().unwrap_or("-"));

    match cfg.algo.as_str() {
        "monero" => stratum_monero::run(cfg).await?,
        "blake2b" => stratum_blake2b::run(cfg).await?,
        "fisheye" => stratum_fisheye::run(cfg).await?,
        "duino" => duinocoin::run(cfg).await?,
        _ => unreachable!(),
    }

    Ok(())
}
