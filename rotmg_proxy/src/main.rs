mod data;
mod proxy;
mod servers;

use crate::proxy::Proxy;
use crate::servers::ServerList;
use anyhow::ensure;
use anyhow::Context;
use env_logger::Env;
use log::{debug, info};
use rotmg_packets::Parameters;
use std::net::IpAddr;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::try_join;

#[derive(StructOpt)]
pub struct Opts {
    /// Use the given port for listening instead of using the one extracted from
    /// the ROTMG client.
    #[structopt(short, long)]
    port: Option<u16>,

    /// Listen on the given IP address.
    #[structopt(short, long, default_value = "127.0.0.1")]
    ip: IpAddr,

    /// The default server to connect to.
    #[structopt(short = "s", long, default_value = "USEast")]
    default_server: String,

    /// Respond to flash policy file requests with the given policy file instead
    /// of the default "allow-all" policy.
    #[structopt(long)]
    policy_file: Option<PathBuf>,
}

fn init_logging() {
    env_logger::init_from_env(
        Env::new().default_filter_or(concat!(env!("CARGO_PKG_NAME"), "=INFO")),
    )
}

/// Create the data directory and return the path
async fn init_data_dir() -> anyhow::Result<PathBuf> {
    let dir = dirs::data_dir()
        .map(|p| p.join(env!("CARGO_PKG_NAME")))
        .context("getting system data dir")?;

    tokio::fs::create_dir_all(&dir)
        .await
        .context("creating data dir")?;

    Ok(dir)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();
    let opts: Opts = StructOpt::from_args();
    let dir = init_data_dir().await?;

    Proxy::init(dir, opts).await?.start().await?;

    Ok(())
}
