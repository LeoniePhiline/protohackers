use std::{
    io::{Read, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
};

use color_eyre::eyre::{self, Result};
use tracing::{debug, info};

fn main() -> Result<()> {
    util::init(0, "Smoke Test", "threaded")?;

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8080))?;

    info!(local_addr = ?listener.local_addr()?, "Listening.");

    std::thread::scope(|scope| {
        for stream in listener.incoming() {
            let stream = stream?;

            info!(peer_addr = ?stream.peer_addr()?, "Connected.");

            scope.spawn(|| {
                handle_connection(stream)?;

                debug!("Client thread shutdown.");

                eyre::Ok(())
            });
        }

        eyre::Ok(())
    })?;

    info!("Shutdown.");

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0u8; 8192];

    loop {
        let num_bytes_read = stream.read(&mut buf)?;
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

        stream.write_all(read)?;
    }

    Ok(())
}
