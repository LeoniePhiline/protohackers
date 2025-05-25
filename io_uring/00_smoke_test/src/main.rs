use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(0, "Smoke Test", "io_uring")?;

    Ok(())
}
