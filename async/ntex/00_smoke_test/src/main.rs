use color_eyre::eyre::{self, Result};

fn main() -> Result<()> {
    util::init(0, "Smoke Test", "async/ntex")?;

    let runtime = ntex_neon::Runtime::new()?;

    runtime.block_on(async {
        //

        eyre::Ok(())
    })?;

    Ok(())
}
