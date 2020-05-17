use log::debug;
use std::borrow::Cow;
use std::io;
use std::net::Shutdown;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{delay_for, Duration};

/// A utility for responding to policy file requests from flash.
pub struct PolicyFile(Cow<'static, [u8]>);

/// The binary message denoting a policy file request
const POLICY_REQUEST: &[u8] = b"<policy-file-request/>\0";

/// A default permissive policy file, allowing unrestricted access to any
/// port from any domain.
//language=XML
const PERMISSIVE_POLICY_FILE: &[u8] = br#"
<?xml version="1.0"?>
<!DOCTYPE cross-domain-policy SYSTEM "/xml/dtds/cross-domain-policy.dtd">
<cross-domain-policy>
    <site-control permitted-cross-domain-policies="all"/>
    <allow-access-from domain="*" to-ports="*"/>
</cross-domain-policy>
"#;

impl PolicyFile {
    /// Create a new policy file with the given response.
    pub fn new(policy_file: impl Into<Cow<'static, [u8]>>) -> Self {
        Self(policy_file.into())
    }

    /// Create a new policy file that allows access to all ports from all
    /// domains. This can potentially be a security risk, and should be used
    /// with care.
    pub fn new_allowing_all() -> Self {
        Self::new(PERMISSIVE_POLICY_FILE)
    }

    /// Handle a given new TCP connection, determining whether it's a policy
    /// file request and handling it appropriately.
    ///
    /// When a policy file request is detected, the response will be sent,
    /// connection closed, and `None` returned. Otherwise, the original TCP
    /// stream will be returned, with no bytes removed from its buffer.
    pub async fn handle_connection(&self, mut conn: TcpStream) -> io::Result<Option<TcpStream>> {
        let mut next_delay_ms = 1;
        let mut buffer = [0u8; POLICY_REQUEST.len()];

        loop {
            let n = conn.peek(&mut buffer).await?;
            if POLICY_REQUEST[..n] == buffer[..n] {
                if n == POLICY_REQUEST.len() {
                    // definitely a policy file request
                    // send response, then close connection
                    debug!("Sending policy file to from {}", conn.peer_addr()?);
                    conn.write_all(&self.0).await?;
                    conn.shutdown(Shutdown::Both)?;
                    return Ok(None);
                } else {
                    // could be a policy file request
                    // check again after a delay
                    if next_delay_ms >= 10_000 {
                        // assume connection is dropped
                        return Err(io::Error::new(
                            io::ErrorKind::TimedOut,
                            "timeout exceeded handling policy file request",
                        ));
                    }
                    delay_for(Duration::from_millis(next_delay_ms)).await;
                    next_delay_ms *= 2;
                }
            } else {
                // definitely not a policy file request
                return Ok(Some(conn));
            }
        }
    }
}
