use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(11, "Pest Control", "may")?;

    Ok(())
}
