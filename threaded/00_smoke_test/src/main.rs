use std::{
    io::{Read as Foobar, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use color_eyre::eyre::Result;
use tracing::{error, info, trace};

fn main() -> Result<()> {
    util::init(0, "Smoke Test", "threaded")?;

    // - Accept thread
    //   - Create a socket
    // per incoming connection:
    // - Connection thread

    accept()?;

    Ok(())
}

fn accept() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:10000")?;

    loop {
        let (stream, address) = listener.accept()?;

        info!(%address, "Connection accepted");

        let _ = thread::spawn(|| {
            let _ = echo(stream).inspect_err(|err| error!("{err:?}"));
        });
    }
}

fn echo(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0u8; 4096];

    loop {
        let count = stream.read(&mut buffer)?;
        if count == 0 {
            return Ok(());
        }

        let bytes = &buffer[..count];
        trace!(bytes = %String::from_utf8_lossy(bytes), count, "Bytes read");

        stream.write_all(bytes)?;
    }
}
