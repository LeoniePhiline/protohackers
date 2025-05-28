use std::net::{Ipv4Addr, TcpListener, TcpStream};

use async_executor::Executor;
use async_io::{Async, block_on};
use color_eyre::eyre::{self, Result};
use futures_lite::{AsyncReadExt, AsyncWriteExt, StreamExt, pin};
use tracing::{debug, error, info};

fn main() -> Result<()> {
    util::init(0, "Smoke Test", "async/smol")?;

    let executor = Executor::new();
    block_on(executor.run(async {
        let listener = Async::<TcpListener>::bind((Ipv4Addr::UNSPECIFIED, 8080))?;

        info!(local_addr = ?listener.as_ref().local_addr()?, "Listening.");

        let incoming = listener.incoming();
        pin!(incoming);

        incoming
            .filter_map(|result| match result {
                Ok(stream) => {
                    info!(peer_addr = ?stream.as_ref().peer_addr(), "Connected.");
                    Some(stream)
                }
                Err(err) => {
                    error!(?err);
                    None
                }
            })
            .for_each(|stream| {
                executor
                    .spawn(async {
                        handle_connection(stream)
                            .await
                            .inspect_err(|err| error!(?err))
                    })
                    .detach();
            })
            .await;

        // Alternative implementation:
        //
        // let mut incoming = incoming.filter_map(...);
        //
        // while let Some(stream) = incoming.next().await {
        //     executor
        //         .spawn(async {
        //             handle_connection(stream)
        //                 .await
        //                 .inspect_err(|err| error!(?err))
        //         })
        //         .detach();
        // }

        eyre::Ok(())
    }))?;

    Ok(())
}

async fn handle_connection(mut stream: Async<TcpStream>) -> Result<()> {
    let mut buf = [0u8; 8192];

    loop {
        let num_bytes_read = stream.read(&mut buf).await?;

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

        stream.write(read).await?;
    }

    info!("Client disconnected.");

    Ok(())
}
