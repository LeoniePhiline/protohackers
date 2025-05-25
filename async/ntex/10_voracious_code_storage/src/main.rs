use color_eyre::eyre::Result;

fn main() -> Result<()> {
    util::init(10, "Voracious Code Storage", "async/ntex")?;

    Ok(())
}
