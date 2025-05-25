use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(1, "Prime Time", "may")?;

    Ok(())
}
