use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(6, "Speed Daemon", "async/ntex")?;

    Ok(())
}
