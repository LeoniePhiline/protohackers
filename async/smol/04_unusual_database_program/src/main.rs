use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(4, "Unusual Database Program", "async/smol")?;

    Ok(())
}
