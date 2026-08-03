use crate::app::App;

mod app;
mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    let app_result = App::new().run(&mut terminal).await;

    ratatui::restore();

    app_result
}
