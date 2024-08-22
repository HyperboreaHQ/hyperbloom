pub mod ui;

use ui::login_window::LoginWindow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::run(LoginWindow::new()).await?;

    Ok(())
}
