use log::debug;
use std::borrow::Cow;
use std::io;
use std::net::Shutdown;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{delay_for, Duration};

/// A utility for responding to policy file requests from flash.
///
/// Policy files specify what ports are accessible from a flash player. Before
/// connecting, a flash player may send a policy file request, and then parse
/// the response to determine whether the connection is legal. This type
/// provides a simple way to handle policy file requests.
pub struct PolicyFile(Cow<'static, str>);

impl PolicyFile {
    /// The binary message denoting a policy file request.
    pub const REQUEST_MESSAGE: &'static [u8] = b"<policy-file-request/>\0";

    /// Create a new policy file with the given response.
    pub fn new(policy_file: impl Into<Cow<'static, str>>) -> Self {
        Self(policy_file.into())
    }

    /// A policy file that allows access to all ports from all domains.
    ///
    /// Using this policy file is a minor security risk, as it can allow flash
    /// to access other services unexpectedly.
    pub const ALLOW_ALL: Self = Self(Cow::Borrowed(include_str!("unrestricted_policy_file.xml")));

    /// Get the contents of this policy file.
    pub fn contents(&self) -> &str {
        self.0.as_ref()
    }

    /// Handle a given new TCP connection, determining whether it's a policy
    /// file request and handling it appropriately.
    ///
    /// When a policy file request is detected, the response will be sent,
    /// connection closed, and `None` returned. Otherwise, the original TCP
    /// stream will be returned, with no bytes removed from its buffer.
    ///
    /// Note that this method should be used in conjunction with a timeout to
    /// avoid the possibility of malicious connections that never close.
    pub async fn handle_connection(&self, mut conn: TcpStream) -> io::Result<Option<TcpStream>> {
        const RETRY_DELAY: Duration = Duration::from_millis(50);

        let mut buffer = [0u8; Self::REQUEST_MESSAGE.len()];

        loop {
            let n = conn.peek(&mut buffer).await?;
            if Self::REQUEST_MESSAGE[..n] == buffer[..n] {
                if n == Self::REQUEST_MESSAGE.len() {
                    // definitely a policy file request
                    // send response, then close connection
                    debug!("Sending policy file to from {}", conn.peer_addr()?);
                    conn.write_all(self.0.as_bytes()).await?;
                    conn.shutdown(Shutdown::Both)?;
                    return Ok(None);
                } else {
                    // could be a policy file request
                    // check again after a delay - necessary because peek will
                    // never block
                    delay_for(RETRY_DELAY).await;
                }
            } else {
                // definitely not a policy file request
                return Ok(Some(conn));
            }
        }
    }
}
