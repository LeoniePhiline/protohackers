use std::{fmt::Debug, net::SocketAddr};

use color_eyre::eyre::{/* eyre, */ Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};
use tracing::{debug_span, error, info, instrument, trace, Instrument};

#[tokio::main]
async fn main() -> Result<()> {
    util::init(0, "Smoke Test", "async/tokio")?;

    accept("0.0.0.0:10000").await?;

    Ok(())
}

#[instrument]
async fn accept<A: ToSocketAddrs + Debug>(listen_address: A) -> Result<()> {
    let listener = TcpListener::bind(listen_address).await?;

    loop {
        let (stream, address) = listener.accept().await?;

        info!(%address, "Connection accepted");

        tokio::spawn(async move {
            echo(stream, address)
                .await
                .unwrap_or_else(|err| error!("{err:?}"));
        }.in_current_span());
    }
}

#[instrument(level = "debug", skip(stream))]
async fn echo(mut stream: TcpStream, address: SocketAddr) -> Result<()> {
    let mut buffer = [0u8; 4096];

    loop {
        let span = debug_span!("reading some bytes");
        let _guard = span.enter();

        let count = stream.read(&mut buffer).await?;
        if count == 0 {

            // return Err(eyre!("ooh no!"));
            return Ok(())
        }

        let bytes = &buffer[..count];
        trace!(bytes = %String::from_utf8_lossy(bytes), count, "Bytes read");

        stream.write_all(bytes).await?;
    }
}
