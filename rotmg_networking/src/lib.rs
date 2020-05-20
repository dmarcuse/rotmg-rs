pub mod codec;
mod policy;
mod rc4;

use crate::codec::{Decoder, Encoder};
use crate::rc4::Rc4;
pub use policy::PolicyFile;
use std::io;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpStream, ToSocketAddrs};

/// Create cipher instances from the ROTMG RC4 keys
fn create_ciphers(keys: &[u8]) -> (Rc4, Rc4) {
    let (key0, key1) = keys.split_at(keys.len() / 2);
    (Rc4::new(key0), Rc4::new(key1))
}

/// Open a new ROTMG network connection using the given RC4 keys, as a client.
///
/// The raw binary keys should be used, decoded from hexadecimal.
pub async fn connect(
    address: impl ToSocketAddrs,
    keys: &[u8],
) -> io::Result<(Decoder<OwnedReadHalf>, Encoder<OwnedWriteHalf>)> {
    let (tx_rc4, rx_rc4) = create_ciphers(keys);
    let (rx, tx) = TcpStream::connect(address).await?.into_split();
    Ok((Decoder::new(rx, rx_rc4), Encoder::new(tx, tx_rc4)))
}

/// Accept an incoming ROTMG network connection using the given RC4 keys, as a
/// server.
///
/// The raw binary keys should be used, decoded from hexadecimal.
pub fn accept(conn: TcpStream, keys: &[u8]) -> (Decoder<OwnedReadHalf>, Encoder<OwnedWriteHalf>) {
    let (rx_rc4, tx_rc4) = create_ciphers(keys);
    let (rx, tx) = conn.into_split();
    (Decoder::new(rx, rx_rc4), Encoder::new(tx, tx_rc4))
}

/// Accept an incoming ROTMG network connection as a server, handling policy
/// file requests.
///
/// The raw binary keys should be used, decoded from hexadecimal.
///
/// This method will automatically detect and handle policy file requests using
/// the given policy file. A timeout should be used to avoid malicious
/// connections that intentionally never close. When a policy file request is
/// handled, `None` will be returned.
pub async fn accept_with_policy(
    conn: TcpStream,
    keys: &[u8],
    policy: &PolicyFile,
) -> io::Result<Option<(Decoder<OwnedReadHalf>, Encoder<OwnedWriteHalf>)>> {
    match policy.handle_connection(conn).await? {
        None => Ok(None),
        Some(conn) => Ok(Some(accept(conn, keys))),
    }
}
