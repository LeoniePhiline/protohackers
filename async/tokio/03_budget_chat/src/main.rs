use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(3, "Budget Chat", "async/tokio")?;

    Ok(())
}
