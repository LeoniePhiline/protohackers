use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(8, "Insecure Sockets Layer", "threaded")?;

    Ok(())
}
