use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(9, "Job Centre", "async/tokio")?;

    Ok(())
}
