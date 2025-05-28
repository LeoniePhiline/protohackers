use std::{
    net::Ipv4Addr,
    pin::Pin,
    task::{Context, Poll, ready},
};

use color_eyre::eyre::{self, Result};
use tokio::{
    io::{AsyncBufRead, AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader, copy_buf},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, error, info};

fn main() -> Result<()> {
    util::init(0, "Smoke Test", "async/tokio")?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;

    runtime.block_on(async {
        let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8080)).await?;

        info!(local_addr = ?listener.local_addr()?, "Listening.");

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            info!(?peer_addr, "Connected.");

            tokio::spawn(async {
                handle_connection(stream)
                    .await
                    .inspect_err(|err| error!(?err))
            });
        }

        #[allow(unreachable_code)]
        eyre::Ok(())
    })?;

    Ok(())
}

// Looks like the sync version.
async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0u8; 8192];

    loop {
        let num_bytes_read = stream.read_buf(&mut buf.as_mut_slice()).await?;

        if num_bytes_read == 0 {
            info!("Client disconnected.");

            break;
        }

        let read = &buf[0..num_bytes_read];

        debug!(
            num_bytes_read,
            read = &buf[0..num_bytes_read],
            as_str = ?String::from_utf8_lossy(read)
        );

        stream.write_all(read).await?;
    }

    info!("Client disconnected.");

    Ok(())
}

// Probably more idiomatic, but very different from the straightforward sync code.
#[allow(dead_code)]
async fn handle_connection_overengineered(stream: TcpStream) -> Result<()> {
    pin_project_lite::pin_project! {
        struct TracingBufReader<R> {
            #[pin]
            inner: BufReader<R>,
        }
    }

    impl<R: AsyncRead + Unpin> TracingBufReader<R> {
        fn new(inner: BufReader<R>) -> Self {
            Self { inner }
        }
    }

    impl<R: AsyncRead + Unpin> AsyncRead for TracingBufReader<R> {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            let me = self.project();
            me.inner.poll_read(cx, buf)
        }
    }

    impl<R: AsyncRead + Unpin> AsyncBufRead for TracingBufReader<R> {
        fn poll_fill_buf(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<&[u8]>> {
            let me = self.project();

            Poll::Ready(
                ready!(me.inner.poll_fill_buf(cx))
                    .inspect(|read| debug!(?read, as_str = ?String::from_utf8_lossy(read))),
            )
        }

        fn consume(self: Pin<&mut Self>, amt: usize) {
            let me = self.project();

            me.inner.consume(amt);
        }
    }

    // Function body

    let (read, mut write) = stream.into_split();

    let mut buf_read = TracingBufReader::new(BufReader::new(read));

    copy_buf(&mut buf_read, &mut write).await?;

    info!("Client disconnected.");

    Ok(())
}
