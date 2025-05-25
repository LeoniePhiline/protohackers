use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(5, "Mob in the Middle", "io_uring")?;

    Ok(())
}
