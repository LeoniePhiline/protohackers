use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(2, "Means to an End", "io_uring")?;

    Ok(())
}
