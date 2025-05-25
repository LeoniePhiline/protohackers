use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(7, "Line Reversal", "async/tokio")?;

    Ok(())
}
