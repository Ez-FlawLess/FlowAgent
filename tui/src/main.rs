use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::app::App;

mod app;
mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let file_appender = tracing_appender::rolling::daily("./logs", "tui.log");

    let (non_blocking, _gaurd) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")))
        .init();

    color_eyre::install()?;

    let mut terminal = ratatui::init();

    info!("app starting...");
    let app_result = App::new().run(&mut terminal).await;
    info!("app returning...");

    ratatui::restore();

    app_result
}
