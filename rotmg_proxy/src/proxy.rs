use crate::data::get_params;
use crate::servers::ServerList;
use crate::Opts;
use anyhow::Context;
use log::{debug, error, info};
use rotmg_networking::{accept_with_policy, connect, PolicyFile};
use rotmg_packets::raw::RawPacket;
use rotmg_packets::Parameters;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::read_to_string;
use tokio::net::{TcpListener, TcpStream};
use tokio::{select, try_join};

pub struct Proxy {
    data_dir: PathBuf,
    opts: Opts,
    params: Parameters,
    servers: ServerList,
    policy_file: PolicyFile,
    keys: Vec<u8>,
}

impl Proxy {
    async fn load_policy_file(opts: &Opts) -> anyhow::Result<PolicyFile> {
        match &opts.policy_file {
            None => Ok(PolicyFile::ALLOW_ALL),
            Some(path) => Ok(PolicyFile::new(read_to_string(path).await?)),
        }
    }

    /// Perform proxy initialization tasks like loading data
    pub async fn init(data_dir: PathBuf, opts: Opts) -> anyhow::Result<Self> {
        let (params, servers, policy_file) = try_join!(
            get_params(&data_dir),
            ServerList::load(&opts.default_server),
            Self::load_policy_file(&opts)
        )?;

        let keys = hex::decode(&params.rc4).context("parsing RC4 keys")?;

        Ok(Self {
            data_dir,
            opts,
            params,
            servers,
            policy_file,
            keys,
        })
    }

    /// Run the proxy server
    pub async fn start(self) -> anyhow::Result<()> {
        let addr = (
            self.opts.ip,
            self.opts.port.unwrap_or(self.params.basic.port),
        );

        let mut listener = TcpListener::bind(addr).await.context("binding address")?;
        info!("Proxy server started on {}", listener.local_addr().unwrap());

        let proxy = Arc::new(self);
        loop {
            let (conn, addr) = listener.accept().await.context("accepting client")?;
            debug!("Spawning task to handle connection from {}", addr);

            let proxy = proxy.clone();
            tokio::spawn(async move {
                match proxy.handle_connection(conn, addr).await {
                    Ok(()) => (),
                    Err(e) => error!("Unexpected client error: {}", e),
                };
            });
        }
    }

    async fn handle_connection(
        self: Arc<Self>,
        conn: TcpStream,
        addr: SocketAddr,
    ) -> anyhow::Result<()> {
        let (mut client_rx, mut client_tx) =
            match accept_with_policy(conn, &self.keys, &self.policy_file).await? {
                Some(c) => c,
                None => {
                    info!("Responded to policy file request from {}", addr);
                    return Ok(());
                }
            };

        info!("Accepted game connection from {}", addr);

        let (mut server_rx, mut server_tx) = connect(
            (self.servers.get_default(), self.params.basic.port),
            &self.keys,
        )
        .await
        .context("connecting to real server")?;

        loop {
            select! {
                Some(r) = client_rx.maybe_recv() => {
                    let raw_packet: &mut RawPacket = r?;
                    server_tx.send(raw_packet).await?;
                },
                Some(r) = server_rx.maybe_recv() => {
                    let raw_packet = r?;
                    client_tx.send(raw_packet).await?;
                },
                else => break,
            }
        }

        info!("Connection from {} closed gracefully", addr);

        Ok(())
    }
}
