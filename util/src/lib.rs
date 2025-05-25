use color_eyre::eyre::{Result, WrapErr};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init(problem_number: u8, problem_title: &str, variant: &str) -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().pretty().with_filter(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::DEBUG.into())
                    .from_env()?,
            ),
        )
        .try_init()
        .wrap_err("failed to initialize tracing-subscriber")?;

    info!(
        "{:=^60}",
        format!(
            " Protohackers {problem_number}: {} ({variant}) ",
            problem_title
        )
    );

    Ok(())
}
