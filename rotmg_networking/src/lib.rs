mod codec;
mod policy;
mod rc4;

pub use codec::Codec;
pub use policy::PolicyFile;
use std::io;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_util::codec::{Decoder, Framed};

/// A framed ROTMG network connection
pub type Connection = Framed<TcpStream, Codec>;

/// Open a new ROTMG network connection using the given RC4 keys, as a client.
///
/// The raw binary keys should be used, decoded from hexadecimal.
pub async fn connect(address: impl ToSocketAddrs, keys: &[u8]) -> io::Result<Connection> {
    Ok(Codec::new_as_client(keys).framed(TcpStream::connect(address).await?))
}

/// Accept an incoming client connection, framing it as a ROTMG connection.
///
/// The raw binary keys should be used, decoded from hexadecimal.
///
/// This method will automatically detect and handle policy file requests using
/// the given policy file. A timeout should be used to avoid malicious
/// connections that intentionally never close. When a policy file request is
/// handled, `None` will be returned instead of a framed connection.
pub async fn accept(
    connection: TcpStream,
    keys: &[u8],
    policy: &PolicyFile,
) -> io::Result<Option<Connection>> {
    match policy.handle_connection(connection).await? {
        None => Ok(None),
        Some(conn) => Ok(Some(Codec::new_as_server(keys).framed(conn))),
    }
}
