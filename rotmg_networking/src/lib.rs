pub mod codec;
pub mod policy;
pub mod rc4;

use codec::Codec;
use futures::TryStreamExt;
use policy::PolicyFile;
use std::io;

use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::stream::Stream;
use tokio_util::codec::{Decoder, Framed};

/// A framed ROTMG network connection
pub type Connection = Framed<TcpStream, Codec>;

/// Open a new ROTMG network connection using the given RC4 keys, as a client.
///
/// The raw binary keys should be used, decoded from hexadecimal.
pub async fn connect(address: impl ToSocketAddrs, keys: &[u8]) -> io::Result<Connection> {
    Ok(Codec::new_as_client(keys).framed(TcpStream::connect(address).await?))
}

/// Listen for incoming ROTMG network connections using the given RC4 keys, as a
/// server.
///
/// The raw binary keys should be used, decoded from hexadecimal.
///
/// A policy file may also be provided, which will be used to handle and filter
/// out any flash policy file requests.
#[allow(clippy::needless_lifetimes)] // false positive on lint here
pub async fn listen<'a>(
    address: impl ToSocketAddrs,
    keys: &[u8],
    policy: Option<&'a PolicyFile>,
) -> io::Result<impl Stream<Item = io::Result<Connection>> + 'a> {
    let codec = Codec::new_as_server(keys);

    Ok(TcpListener::bind(address)
        .await?
        .try_filter_map(move |c| async move {
            match policy {
                None => Ok(Some(c)),
                Some(p) => p.handle_connection(c).await,
            }
        })
        .map_ok(move |c| codec.clone().framed(c)))
}
